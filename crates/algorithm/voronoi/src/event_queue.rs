// Based on the book "Computational Geometry" from Mark Berg , Otfried Cheong , Marc Kreveld , Mark Overmars. [DOI](https://doi.org/10.1007/978-3-662-04245-8)

use core::fmt::Display;

use common::math::A;
use priority_queue::PriorityQueue;

use crate::{Circle, Site, beachline::SQKey};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Event<T: A> {
    Site(Site<T>),
    Circle(SQKey, Circle<T>),
}

impl<T: A> Display for Event<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Site(site) => {
                write!(
                    f,
                    "New Site {}: ({:.2}, {:.2})",
                    site.old_id, site.x, site.y
                )
            }
            Self::Circle(_, circle) => {
                write!(
                    f,
                    "Circle: ({:.2},{:.2},{:.2})",
                    circle.x, circle.y, circle.r
                )
            }
        }
    }
}

impl<T: A> Event<T> {
    pub const fn x(&self) -> &T {
        match self {
            Self::Site(site) => &site.x,
            Self::Circle(_, circle) => &circle.x,
        }
    }

    pub const fn y(&self) -> &T {
        match self {
            Self::Site(site) => &site.y,
            Self::Circle(_, circle) => &circle.y,
        }
    }

    pub fn corrected_y(&self) -> T {
        match self {
            Self::Site(site) => site.y.clone(),
            Self::Circle(_, circle) => circle.y.clone() - &circle.r,
        }
    }

    pub const fn radius(&self) -> Option<&T> {
        match self {
            Self::Site(_) => None,
            Self::Circle(_, circle) => Some(&circle.r),
        }
    }

    /// Returns `true` if the event is [`Circle`].
    ///
    /// [`Circle`]: Event::Circle
    #[must_use]
    pub const fn is_circle(&self) -> bool {
        matches!(self, Self::Circle(..))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Priority<T: A> {
    y: T,
    x: T,
}

impl<T: A + PartialOrd> PartialOrd for Priority<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: A + Ord> Ord for Priority<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.y.cmp(&self.y).then(self.x.cmp(&other.x)).reverse()
    }
}

#[derive(Debug, Clone)]
pub struct EventQueue<T: A> {
    inner: priority_queue::PriorityQueue<Event<T>, Priority<T>>,
}

impl<T: A> EventQueue<T> {
    /// Initialize the event queue Q with all site events
    pub fn init(sites: impl Iterator<Item = Site<T>>) -> Self {
        let mut ret = Self::with_capacity(sites.size_hint().0);

        for site in sites {
            ret.insert(Event::Site(site));
        }

        ret
    }

    pub fn new() -> Self {
        Self {
            inner: PriorityQueue::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            inner: PriorityQueue::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, event: impl Into<Event<T>>) -> Event<T> {
        let event = event.into();
        let x = event.x().clone();
        let y = event.y().clone() - event.radius().unwrap_or(&T::default());
        let priority = Priority { y, x };

        self.inner.push(event.clone(), priority);
        event
    }

    pub fn delete(&mut self, event: &Event<T>) -> bool {
        self.inner.remove(event).is_some()
    }

    pub fn pop(&mut self) -> Option<Event<T>> {
        self.inner.pop().map(|v| v.0)
    }
}

#[cfg(test)]
mod tests {
    use common::math::Float;

    use crate::event_queue::Event;

    use super::*;

    #[test]
    fn test_prio() {
        let sites = [
            Event::<Float>::Site(Site::new(0.0, 0.0, 0, 0)),
            Event::Site(Site::new(-1.0, 0.0, 1, 1)),
            Event::Site(Site::new(1.0, 0.0, 2, 2)),
            Event::Site(Site::new(0.0, 0.0, 3, 3)),
            Event::Site(Site::new(-1.0, -1.0, 4, 4)),
            Event::Site(Site::new(1.0, 1.0, 5, 5)),
        ];

        let mut queue = EventQueue::new();

        for site in sites {
            queue.insert(site);
        }

        while let Some(event) = queue.pop() {
            println!("{event:?}");
        }
    }
}
