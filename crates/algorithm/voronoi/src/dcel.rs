// Based on the book "Computational Geometry" from Mark Berg , Otfried Cheong , Marc Kreveld , Mark Overmars. [DOI](https://doi.org/10.1007/978-3-662-04245-8)

use core::fmt::Display;

use common::math::A;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
// The vertex record of a vertex v stores the coordinates of v in a field called
// Coordinates(v). It also stores a pointer IncidentEdge(v) to an arbitrary
// half-edge that has v as its origin.
pub struct Vertex<T: A> {
    pub x: T,
    pub y: T,
    pub incident_edge: usize,
    pub origin_sites: Vec<usize>,
}

impl<T: A> Vertex<T> {
    pub fn new(
        x: T,
        y: T,
        incident_edge: usize,
        origin_sites: impl IntoIterator<Item = usize>,
    ) -> Self {
        Self {
            x,
            y,
            incident_edge,
            origin_sites: origin_sites.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct HalfEdge {
    /// Pointer to origin Vertex
    pub origin: Option<usize>,
    pub origin_site: usize,
    /// Pointer next edge
    pub next: Option<usize>,
    /// Pointer to twin
    pub twin: usize,
}

/// Implementation of a Doubly-Connected Edge List as described in Section 2.2 of ISBN 978-3-540-77973-5
// "The doubly-connected edge list consists of three collections
/// of records: one for the vertices, one for the faces, and one for the half-edges.
/// These records store the following geometric and topological information:"
#[derive(Debug, Clone)]
pub struct Dcel<T: A> {
    /// These are all our vertices
    pub vertices: Vec<Vertex<T>>,

    /// All our Half Edges
    pub half_edges: Vec<HalfEdge>,

    /// Each index is the id of an site. The value corresponds to a Edge
    pub faces: Vec<Option<usize>>,
}

impl<T: A> Dcel<T> {
    #[must_use]
    pub fn new(size: usize) -> Self {
        Self {
            vertices: Vec::new(),
            half_edges: Vec::new(),
            faces: vec![None; size],
        }
    }

    pub fn set_face(&mut self, face: usize, half_edge: usize) {
        if self.faces[face].is_none() {
            self.faces[face] = Some(half_edge);
        }
    }

    #[must_use]
    pub fn vertex(&self, idx: usize) -> &Vertex<T> {
        &self.vertices[idx]
    }

    #[must_use]
    pub fn get_twin(&self, idx: usize) -> usize {
        self.half_edges[idx].twin
    }

    #[must_use]
    pub fn get_optional_edge(&self, idx: usize) -> Option<(usize, usize)> {
        let l = self.half_edges[idx];
        let r = self.half_edges[l.twin];
        if let (Some(v1), Some(v2)) = (l.origin, r.origin) {
            Some((v1, v2))
        } else {
            None
        }
    }

    pub fn new_twins(&mut self) -> (usize, usize) {
        let index = self.half_edges.len();
        self.half_edges.push(HalfEdge::default());

        let twin = self.half_edges.len();
        self.half_edges.push(HalfEdge::default());

        self.half_edges[index].twin = twin;
        self.half_edges[twin].twin = index;

        (index, twin)
    }

    pub fn push_vertex(&mut self, vertex: Vertex<T>) -> usize {
        let index = self.vertices.len();
        self.vertices.push(vertex);
        index
    }

    pub fn set_origin(&mut self, key: usize, origin: usize) {
        self.half_edges[key].origin = Some(origin);
    }

    pub fn set_origin_face(&mut self, key: usize, origin: usize) {
        self.half_edges[key].origin_site = origin;
    }
    pub fn set_next(&mut self, key: usize, next: impl Into<Option<usize>>) {
        self.half_edges[key].next = next.into();
    }

    pub fn pair(&self) -> impl Iterator<Item = [HalfEdge; 2]> {
        self.half_edges.as_chunks::<2>().0.iter().copied()
    }

    pub fn half_infinite_pair(&self) -> impl Iterator<Item = [HalfEdge; 2]> {
        self.pair()
            .filter(|[l, r]| matches!((l.origin, r.origin), (Some(_), None) | (None, Some(_))))
    }

    pub fn full_infinite_pair(&self) -> impl Iterator<Item = [HalfEdge; 2]> {
        self.pair()
            .filter(|[l, r]| matches!((l.origin, r.origin), (None, None)))
    }

    pub fn get_all_vertices(&self) -> impl Iterator<Item = [Vertex<T>; 2]> {
        self.half_edges
            .as_chunks::<2>()
            .0
            .iter()
            .filter_map(|[l, r]| {
                let l = self.vertices[l.origin?].clone();
                let r = self.vertices[r.origin?].clone();
                Some([l, r])
            })
    }
}

impl<T: A> Display for Dcel<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Vertices:")?;
        for (i, v) in self.vertices.iter().enumerate() {
            writeln!(f, "{i:.2}:{v:?}")?;
        }
        writeln!(f, "Half Edges:")?;
        for (i, v) in self.half_edges.iter().enumerate() {
            writeln!(f, "{i:.2}:{v:?}")?;
        }
        writeln!(f, "Faces: {:?}", self.faces)
    }
}
