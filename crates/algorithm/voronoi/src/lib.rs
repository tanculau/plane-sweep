// Based on the book "Computational Geometry" from Mark Berg , Otfried Cheong , Marc Kreveld , Mark Overmars. [DOI](https://doi.org/10.1007/978-3-662-04245-8)

use core::fmt::Display;
use std::collections::HashMap;

use common::{
    AlgoSteps,
    math::{A, B, Float, cartesian::CartesianCoord, find_circle, homogeneous::HomogeneousLine},
    segment::Segment,
};

use crate::{
    beachline::{BeachLine, SQKey},
    dcel::{Dcel, Vertex},
    event_queue::{Event, EventQueue},
    ui::steps::{Step, StepType},
};

mod beachline;
pub mod dcel;
mod event_queue;
pub mod ui;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeIdx;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Site<T: A> {
    pub x: T,
    pub y: T,
    pub id: usize,
    pub old_id: usize,
}

impl<T: A> Display for Site<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Site {}: ({:.2}, {:.2})", self.id, self.x, self.y)
    }
}

impl<T: A> Site<T> {
    pub fn new(x: impl Into<T>, y: impl Into<T>, id: usize, old_id: usize) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            id,
            old_id,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Circle<T: A> {
    pub x: T,
    pub y: T,
    pub r: T,
}

impl<T: A> Circle<T> {
    pub fn new(x: impl Into<T>, y: impl Into<T>, r: impl Into<T>) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            r: r.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct SitePair(usize, usize);

#[derive(Debug, Clone)]
pub struct Voronoi<T: B> {
    sites: Vec<Site<T>>,
    event_queue: EventQueue<T>,
    beach: BeachLine<T>,
    dcel: Dcel<T>,
    steps: AlgoSteps<Step<T>>,
    half_edges: HashMap<SitePair, usize>,
}

impl<T: B + From<f64>> Voronoi<T> {
    pub fn build(sites: &common::site::Sites, bound: u32) -> Self {
        let sites = sites
            .iter()
            //.chain([common::site::Site { x: -20, y: 20, id: usize::MAX - 1 }, common::site::Site { x: 20, y: 20, id: usize::MAX - 2 }].iter())
            .enumerate()
            .map(|(i, v)| Site::new(v.x, v.y, i, v.id))
            .collect();
        let mut voronoi = Self::new(sites);
        voronoi.report(StepType::Init, None);
        voronoi.report(StepType::InitQ, None);
        voronoi.run();
        voronoi.bound(BoundingBox::from_width(bound));

        voronoi.report(StepType::Stopped, None);

        voronoi
    }

    pub fn report(&mut self, step: StepType<T>, event: impl Into<Option<Event<T>>>) {
        self.steps.push(Step {
            typ: step,
            step: self.steps.len(),
            event_queue: self.event_queue.clone(),
            beachline: self.beach.clone(),
            dcel: self.dcel.clone(),
            event: event.into(),
        });
    }

    #[must_use]
    pub fn new(sites: Vec<Site<T>>) -> Self {
        let steps = AlgoSteps::new();
        let mut event_queue = EventQueue::init(sites.iter().copied());
        let mut beach = BeachLine::new();
        if let Some(Event::Site(site)) = event_queue.pop() {
            beach.init(site);
        }

        Self {
            dcel: Dcel::new(sites.len()),
            sites,
            event_queue,
            beach,
            half_edges: HashMap::new(),
            steps,
        }
    }

    #[allow(clippy::missing_panics_doc)]
    pub fn run(&mut self) {
        while let Some(event) = self.event_queue.pop() {
            match event {
                event_queue::Event::Site(site) => {
                    // 2.
                    self.report(StepType::PopQ, event);

                    let above = self.beach.find_arc(&site);
                    self.report(StepType::SearchT(above), event);

                    let to_remove = self.beach[above].take_event();
                    if let Some(event) = to_remove {
                        self.event_queue.delete(&event);
                    }
                    self.report(StepType::DeleteVertexEvent(to_remove), event);

                    let rightest = self.beach.rightest(self.beach.head.unwrap());
                    let leftest = self.beach.leftest(self.beach.head.unwrap());
                    if above == leftest && site.x < self.beach[leftest].site().x {
                        // Special case new site is left from all known sites
                        // TODO REPORT
                        let middle = above;
                        let left = self.beach.insert_before(middle, site);
                        self.report(StepType::ReplaceLeft, event);
                        self.create_half_edges(middle, left, event);

                        self.create_vertex_event(middle, event, false);
                    } else if above == rightest && site.x > self.beach[rightest].site().x {
                        let middle = above;
                        let right = self.beach.insert_after(middle, site);
                        self.report(StepType::ReplaceRight, event);

                        self.create_half_edges(middle, right, event);
                        self.create_vertex_event(middle, event, false);

                        // Special case right arc
                    } else {
                        // Normal arc
                        let left = above;
                        let old = *self.beach[above].site();
                        let middle = self.beach.insert_after(above, site);
                        let right = self.beach.insert_after(middle, old);
                        self.report(StepType::ReplaceT, event);
                        self.create_half_edges(middle, left, event);
                        self.create_vertex_event(left, event, false);
                        self.create_vertex_event(right, event, false);
                    }
                }
                event_queue::Event::Circle(key, Circle { x, y, r: _ }) => {
                    self.report(StepType::PopQ, event);

                    let left = self.beach.predecessor(key).unwrap();
                    let right = self.beach.successor(key).unwrap();

                    let middle = *self.beach[key].site();
                    self.report(StepType::Delete, event);
                    self.beach.delete(key);
                    let take_event = self.beach[left].take_event();
                    if let Some(event) = take_event {
                        self.event_queue.delete(&event);
                    }
                    self.report(StepType::DeleteVertexEvent2(left, take_event), event);
                    let take_event = self.beach[right].take_event();
                    if let Some(event) = take_event {
                        self.event_queue.delete(&event);
                    }
                    self.report(StepType::DeleteVertexEvent2(left, take_event), event);

                    // 2.
                    let lm = self.get_halfedge(self.beach[left].site().id, middle.id);
                    let mr = self.get_halfedge(middle.id, self.beach[right].site().id);

                    let lm_twin = self.dcel.get_twin(lm);
                    let mr_twin = self.dcel.get_twin(mr);

                    let vertex = self.dcel.push_vertex(Vertex::new(
                        x,
                        y,
                        middle.id,
                        [
                            self.beach[left].site().id,
                            middle.id,
                            self.beach[right].site().id,
                        ],
                    ));
                    self.report(StepType::AddVertexToEdges(vertex), event);

                    let (rl, rl_twin) = self.create_half_edges(right, left, event);

                    self.dcel.set_origin(lm_twin, vertex);
                    self.dcel.set_origin(mr_twin, vertex);
                    self.dcel.set_origin(rl_twin, vertex);

                    self.dcel.set_next(lm, rl_twin);
                    self.dcel.set_next(mr, lm_twin);
                    self.dcel.set_next(rl, mr_twin);

                    // 3.
                    self.create_vertex_event(left, event, true);
                    self.create_vertex_event(right, event, true);
                }
            }
        }
    }

    fn get_halfedge(&self, left: usize, right: usize) -> usize {
        *self.half_edges.get(&SitePair(left, right)).unwrap()
    }

    #[allow(clippy::missing_panics_doc)]
    #[allow(clippy::tuple_array_conversions)]
    #[allow(clippy::too_many_lines, clippy::many_single_char_names)]
    pub fn bound(&mut self, bound: BoundingBox<T>) {
        let up = Segment::<T>::new((bound.x_min, bound.y_max), (bound.x_max, bound.y_max));
        let down = Segment::<T>::new((bound.x_min, bound.y_min), (bound.x_max, bound.y_min));
        let left = Segment::<T>::new((bound.x_min, bound.y_min), (bound.x_min, bound.y_max));
        let right = Segment::<T>::new((bound.x_max, bound.y_min), (bound.x_max, bound.y_max));
        self.report(StepType::Bound, None);
        // Bound infinite

        // We also have to bound edges, that intersect two times with the bounding box!!!

        // Idea

        // We calculate the bisector

        // We intersect this bisector with all bounding sites, and take all unique points. the number of points must be two
        let twins = self.dcel.full_infinite_pair().collect::<Vec<_>>();

        for [l, r] in twins {
            let site_a = l.origin_site;
            let site_b = r.origin_site;

            let s_a = self.sites[site_a];
            let s_b = self.sites[site_b];

            let line = Segment::<T>::new((s_a.x, s_a.y), (s_b.x, s_b.y)).pedendicular_bisector();

            let mut vertices = [
                Self::check_bound(up, line),
                Self::check_bound(down, line),
                Self::check_bound(left, line),
                Self::check_bound(right, line),
            ]
            .into_iter()
            .flatten();

            let Some(mut one) = vertices.next() else {
                continue;
            };
            let Some(two) = vertices.next() else {
                continue;
            };

            if one == two {
                let Some(new) = vertices.next() else {
                    continue;
                };
                one = new;
            }

            // Add Vertice

            let ver_one = self.dcel.push_vertex(Vertex::new(
                one.x,
                one.y,
                r.origin_site,
                [l.origin_site, r.origin_site],
            ));
            let ver_two = self.dcel.push_vertex(Vertex::new(
                two.x,
                two.y,
                l.origin_site,
                [l.origin_site, r.origin_site],
            ));
            self.report(
                StepType::FullInfinite(r.twin, l.twin, s_a, s_b, ver_one, ver_two),
                None,
            );

            // Update edge
            self.dcel.set_origin(r.twin, ver_one);
            self.dcel.set_origin(l.twin, ver_two);
        }

        // Idea

        // We inspect each half_edge pair, where only one vertex is set.

        // we calculate the line between the two sites

        // we calulate the direction, by choosing the one, which goes further away from the third site

        // we calculate the collision with the bounding box

        // we update the line with the new calulated vertex

        //Those are all edges we have to update
        let twins = self.dcel.half_infinite_pair().collect::<Vec<_>>();

        for [l, r] in twins {
            let vertex_id = l.origin.unwrap_or_else(|| r.origin.unwrap());

            let Vertex {
                x,
                y,
                incident_edge: _,
                origin_sites,
            } = self.dcel.vertex(vertex_id);

            // The three sites spanning the origin Vertex
            let site_a = l.origin_site;
            let site_b = r.origin_site;
            let site_foreign = origin_sites
                .iter()
                .find(|&&v| v != site_a && v != site_b)
                .unwrap();

            let f = self.sites[*site_foreign];
            // Construct bisector between site_a and site_b

            let s_a = self.sites[site_a];
            let s_b = self.sites[site_b];
            let s = Segment::<T>::new((s_a.x, s_a.y), (s_b.x, s_b.y));
            let line = s.pedendicular_bisector();
            let m = s.midpoint();
            let mdx = m.x - x;
            let mdy = m.y - y;

            let mut direction = match (mdx >= T::from(0), mdy >= T::from(0)) {
                (true, true) => Direction::UpRight,
                (true, false) => Direction::DownRight,
                (false, true) => Direction::UpLeft,
                (false, false) => Direction::DownLeft,
            };

            let l_o = l.origin_site;
            let r_o = r.origin_site;
            let f_o = f.id;

            let lf = self.half_edges.get(&SitePair(l_o, f_o)).unwrap();
            let rf = self.half_edges.get(&SitePair(r_o, f_o)).unwrap();

            let s = Segment::<T>::new((s_a.x, s_a.y), (s_b.x, s_b.y));
            let mut should_inverse = false;

            if let Some((v1, v2)) = self.dcel.get_optional_edge(*lf) {
                let v1 = self.dcel.vertex(v1);
                let v2 = self.dcel.vertex(v2);
                let s1 = Segment::<T>::new((v1.x, v1.y), (v2.x, v2.y));
                should_inverse |= s1.intersect2(s);
            }

            if let Some((v1, v2)) = self.dcel.get_optional_edge(*rf) {
                let v1 = self.dcel.vertex(v1);
                let v2 = self.dcel.vertex(v2);
                let s1 = Segment::<T>::new((v1.x, v1.y), (v2.x, v2.y));
                should_inverse |= s1.intersect2(s);
            }

            if should_inverse {
                direction = direction.inverse();
            }

            let [bound1, bound2] = match direction {
                Direction::UpLeft => [up, left],
                Direction::UpRight => [up, right],
                Direction::DownLeft => [down, left],
                Direction::DownRight => [down, right],
            };

            // Calculate intersection between bounds and line
            let vertex = match (
                Self::check_bound(bound1, line),
                Self::check_bound(bound2, line),
            ) {
                (None, None) => continue,
                (None, Some(v)) | (Some(v), None | _) => v,
            };

            let (t_empty, t_exists) = if l.origin.is_none() { (l, r) } else { (r, l) };

            // Create vertex
            let ver = self.dcel.push_vertex(Vertex::new(
                vertex.x,
                vertex.y,
                t_empty.origin_site,
                origin_sites.iter().copied(),
            ));

            self.report(
                StepType::FullInfinite(r.twin, l.twin, s_a, s_b, vertex_id, ver),
                None,
            );

            // Update edge
            self.dcel.set_origin(t_exists.twin, ver);
        }
    }

    pub fn check_bound(bound: Segment<T>, line: HomogeneousLine<T>) -> Option<CartesianCoord<T>> {
        let coord = bound.line().intersection(line).cartesian().ok()?;
        bound.contains(&coord).then_some(coord)
    }

    pub fn create_half_edges(
        &mut self,
        left: SQKey,
        right: SQKey,
        event: impl Into<Option<Event<T>>>,
    ) -> (usize, usize) {
        let left_id = self.beach[left].site().id;
        let right_id = self.beach[right].site().id;
        self.report(
            StepType::CreateHalfEdges {
                e1: *self.beach[left].site(),
                e2: *self.beach[left].site(),
            },
            event,
        );
        let (edge, twin) = self.dcel.new_twins();
        self.dcel.set_origin_face(edge, self.beach[left].site().id);
        self.dcel.set_origin_face(twin, self.beach[right].site().id);

        let left = self.beach[left];
        let right = self.beach[right];
        self.half_edges.insert(SitePair(left_id, right_id), edge);
        self.half_edges.insert(SitePair(right_id, left_id), twin);

        self.dcel.set_face(left.site().id, edge);
        self.dcel.set_face(right.site().id, twin);
        (edge, twin)
    }

    pub fn create_vertex_event(&mut self, key: SQKey, event_report: Event<T>, from_circle: bool) {
        let Some(left) = self.beach.predecessor(key) else {
            if from_circle {
                self.report(StepType::CreateVertexCircle(key, None), event_report);
            } else {
                self.report(StepType::CreateVertexEvent(key, None), event_report);
            }
            return;
        };
        let Some(right) = self.beach.successor(key) else {
            if from_circle {
                self.report(StepType::CreateVertexCircle(key, None), event_report);
            } else {
                self.report(StepType::CreateVertexEvent(key, None), event_report);
            }
            return;
        };

        let left_side = self.beach[left].site();
        let middle_site = self.beach[key].site();
        let right_side = self.beach[right].site();

        // source https://cp-algorithms.com/geometry/oriented-triangle-area.html
        let det = (middle_site.x - left_side.x) * (right_side.y - left_side.y)
            - (middle_site.y - left_side.y) * (right_side.x - left_side.x);

        if det >= T::from(0) {
            if from_circle {
                self.report(StepType::CreateVertexCircle(key, None), event_report);
            } else {
                self.report(StepType::CreateVertexEvent(key, None), event_report);
            }
            return;
        }
        // end source

        let Some((x, y, r)) = find_circle(
            left_side.x,
            left_side.y,
            middle_site.x,
            middle_site.y,
            right_side.x,
            right_side.y,
        ) else {
            return;
        };

        let event = Event::Circle(key, Circle::new(x, y, r));
        if from_circle {
            self.report(StepType::CreateVertexCircle(key, Some(event)), event_report);
        } else {
            self.report(StepType::CreateVertexEvent(key, Some(event)), event_report);
        }
        self.event_queue.insert(event);
        self.beach[key].set_event(event);
    }

    #[must_use]
    pub const fn dcel(&self) -> &Dcel<T> {
        &self.dcel
    }

    #[must_use]
    pub fn sites(&self) -> &[Site<T>] {
        &self.sites
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox<T> {
    pub x_min: T,
    pub y_min: T,
    pub x_max: T,
    pub y_max: T,
}

impl<T: A + Copy> BoundingBox<T> {
    pub fn new(
        x_min: impl Into<T>,
        y_min: impl Into<T>,
        x_max: impl Into<T>,
        y_max: impl Into<T>,
    ) -> Self {
        Self {
            x_min: x_min.into(),
            y_min: y_min.into(),
            x_max: x_max.into(),
            y_max: y_max.into(),
        }
    }

    #[must_use]
    pub fn from_width(width: u32) -> Self {
        let width = T::from(width);
        let half = width / T::from(2);
        Self::new(-half, -half, half, half)
    }

    pub fn is_within_bounding(&self, vertex: &Vertex<T>) -> bool {
        vertex.x >= self.x_min
            && vertex.x <= self.x_max
            && vertex.y >= self.y_min
            && vertex.y <= self.y_max
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub const fn inverse(self) -> Self {
        match self {
            Self::UpLeft => Self::DownRight,
            Self::UpRight => Self::DownLeft,
            Self::DownLeft => Self::UpRight,
            Self::DownRight => Self::UpLeft,
        }
    }
}

#[cfg(test)]
mod tests {
    use common::{
        math::{Float, breakpoint_between},
        segment::Segment,
    };

    use super::*;

    #[test]
    fn feature55() {
        let sites = common::site::Sites::from_iter([
            common::site::Site::new(1, 0),
            common::site::Site::new(4, -5),
            common::site::Site::new(-6, 0),
            common::site::Site::new(6, 8),
        ]);

        //let voronoi = Voronoi::<Float>::build(&sites, -10.0, -10.0, 10.0, 10.0);

        //println!("{}", voronoi.dcel);
        //for i in voronoi.dcel.get_all_vertices() {
        //    println!("{i:?}");
        //}
    }

    #[test]
    fn test_name() {
        let sites = &[
            (Site::new(1, 0, 0, 0)),
            (Site::new(4, -5, 1, 1)),
            (Site::new(-6, 0, 2, 2)),
            (Site::new(8, -7, 3, 3)),
            (Site::new(10, 10, 4, 4)),
            (Site::new(-10, 10, 5, 5)),
            //    (Site::new(3, 8,3, 2)),

            //                        (Site::new(4, -5, 1, 1)),
            //                        (Site::new(-6, 0, 2, 2)),
            //               //         (Site::new(7, 6, 3, 3)),
            //            (Site::new(1, 5, 4, 4)),
            //(Site::new(0, 0.0, 0, 0)),
            //(Site::new(1, 1, 1, 1)),
            //(Site::new(2, 2, 2, 2)),
        ];

        let mut voronoi = Voronoi::<Float>::new(sites.to_vec());
        voronoi.run();
        println!("{}", voronoi.dcel);
        println!("-------------- Bounding ------------");
        voronoi.bound(BoundingBox::new(-10.0, -10.0, 10.0, 10.0));
        println!("{}", voronoi.dcel);
        for i in voronoi.dcel.get_all_vertices() {
            println!("{i:?}");
        }
    }

    #[test]
    fn test_name2() {
        let sites = &[(Site::new(0.0, 0.0, 0, 0)), (Site::new(0.0, -1.0, 1, 1))];

        let mut voronoi = Voronoi::<Float>::new(sites.to_vec());
        voronoi.run();
        voronoi.bound(BoundingBox::new(-10.0, -10.0, 10.0, 10.0));
        println!("{}", voronoi.dcel);
    }

    #[test]
    fn feature() {
        let brea = breakpoint_between::<Float>(
            (-5.0).into(),
            (22.0).into(),
            (34.0).into(),
            (6.0).into(),
            (40.0).into(),
        );

        println!("{brea}");
    }

    #[test]
    fn feature3() {
        let s1 = Segment::<Float>::new((-1, 0), (1, 0));
        let l1 = s1.pedendicular_bisector();

        println!("{l1:?}");
        println!("{:?}", s1.line());
        println!("{:?}", l1.contains_coord((0, 0)));
        println!("{:?}", l1.contains_coord((0, 1)));
        println!("{:?}", l1.contains_coord((0, 2)));
    }
}

#[test]
fn feature009() {
    let left_side = Site::<Float>::new(-6, 0, 0, 0);
    let middle_site = Site::<Float>::new(3, 4, 0, 0);
    let right_side = Site::<Float>::new(1, 0, 0, 0);

    let det = (middle_site.x - left_side.x) * (right_side.y - left_side.y)
        - (middle_site.y - left_side.y) * (right_side.x - left_side.x);
    println!("{det}");
}
