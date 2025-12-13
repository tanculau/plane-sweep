// Based on the book "Computational Geometry" from Mark Berg , Otfried Cheong , Marc Kreveld , Mark Overmars. [DOI](https://doi.org/10.1007/978-3-662-04245-8)
// Binary search tree https://www.geeksforgeeks.org/dsa/deletion-in-binary-search-tree/

use core::ops::{Index, IndexMut};

use common::math::{A, B, breakpoint_between};
use slotmap::{SlotMap, new_key_type};

use crate::{Site, event_queue::Event};

new_key_type! {pub struct SQKey;}

type Storage<T> = SlotMap<SQKey, Node<T>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Node<T: A> {
    pub parent: Option<SQKey>,
    pub left: Option<SQKey>,
    pub right: Option<SQKey>,
    value: Site<T>,
    event: Option<Event<T>>,
}

impl<T: A> Node<T> {
    pub fn new(value: Site<T>, parent: impl Into<Option<SQKey>>) -> Self {
        Self {
            parent: parent.into(),
            left: None,
            right: None,
            value,
            event: None,
        }
    }

    pub const fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    pub const fn site(&self) -> &Site<T> {
        &self.value
    }

    pub const fn take_event(&mut self) -> Option<Event<T>> {
        self.event.take()
    }

    pub fn set_event(&mut self, event: impl Into<Option<Event<T>>>) {
        self.event = event.into();
    }
}

#[derive(Debug, Clone)]
pub struct BeachLine<T: B> {
    storage: Storage<T>,
    pub head: Option<SQKey>,
}

impl<T: B> Index<SQKey> for BeachLine<T> {
    type Output = Node<T>;

    fn index(&self, index: SQKey) -> &Self::Output {
        &self.storage[index]
    }
}

impl<T: B> IndexMut<SQKey> for BeachLine<T> {
    fn index_mut(&mut self, index: SQKey) -> &mut Self::Output {
        &mut self.storage[index]
    }
}

impl<T: B> BeachLine<T> {
    pub fn new() -> Self {
        Self {
            storage: Storage::default(),
            head: None,
        }
    }

    pub fn next(&self, key: SQKey) -> Option<SQKey> {
        self[key].right.map(|right| self.leftest(right))
    }

    pub fn successor(&self, key: SQKey) -> Option<SQKey> {
        if self[key].right.is_some() {
            return self.next(key);
        }

        let mut parent = self[key].parent?;
        let mut child = key;

        while self[parent].right == Some(child) {
            child = parent;
            parent = self[parent].parent?;
        }

        Some(parent)
    }

    pub fn predecessor(&self, key: SQKey) -> Option<SQKey> {
        if self[key].left.is_some() {
            return self.prev(key);
        }

        let mut parent = self[key].parent?;
        let mut child = key;

        while self[parent].left == Some(child) {
            child = parent;
            parent = self[parent].parent?;
        }

        Some(parent)
    }

    pub fn prev(&self, key: SQKey) -> Option<SQKey> {
        self[key].left.map(|left| self.rightest(left))
    }

    pub fn leftest(&self, mut key: SQKey) -> SQKey {
        while let Some(left) = self[key].left {
            key = left;
        }

        key
    }

    pub fn rightest(&self, mut key: SQKey) -> SQKey {
        while let Some(right) = self[key].right {
            key = right;
        }

        key
    }

    pub fn replace_leaf(&mut self, key: SQKey, site: Site<T>) -> (SQKey, SQKey, SQKey, Site<T>) {
        assert!(self[key].left.is_none() && self[key].right.is_none());

        let old = self[key]
            .parent
            .map_or_else(|| self[key].value, |parent| self[parent].value);

        let middle = Node::new(site, self[key].parent);
        let mp = self.storage.insert(middle);

        if let Some(parent) = self[key].parent {
            if let Some(left) = self[parent].left
                && left == key
            {
                self[parent].left = Some(mp);
            }

            if let Some(right) = self[parent].right
                && right == key
            {
                self[parent].right = Some(mp);
            }
        } else {
            self.head = Some(mp);
        }

        let left = Node::new(old, mp);
        let lp = self.storage.insert(left);

        self[mp].left = Some(lp);

        let right = Node::new(old, mp);
        let rp = self.storage.insert(right);
        self[mp].right = Some(rp);

        (lp, mp, rp, old)
    }

    pub fn insert_after(&mut self, key: SQKey, site: Site<T>) -> SQKey {
        if let Some(right) = self[key].right {
            self.insert_before(right, site)
        } else {
            let node = Node::new(site, key);
            let node_p = self.storage.insert(node);
            self[key].right = Some(node_p);
            self[node_p].parent = Some(key);
            node_p
        }
    }

    pub fn insert_before(&mut self, key: SQKey, site: Site<T>) -> SQKey {
        if let Some(left) = self[key].left {
            self.insert_before(left, site)
        } else {
            let node = Node::new(site, key);
            let node_p = self.storage.insert(node);
            self[key].left = Some(node_p);
            self[node_p].parent = Some(key);
            node_p
        }
    }
    pub fn init(&mut self, value: Site<T>) {
        assert!(self.head.is_none());
        let node = Node::new(value, None);
        let head = self.storage.insert(node);
        self.head = Some(head);
    }

    pub fn find_arc(&self, event: &Site<T>) -> SQKey {
        self.print_debug();
        let mut curr = self.head.unwrap();

        loop {
            let site = self[curr].site();

            let l = self.predecessor(curr);
            let lb = l.map(|v| {
                let left = self[v].site();
                breakpoint_between(left.x, left.y, site.x, site.y, event.y)
            });

            if let Some(lb) = lb
                && event.x < lb
            {
                curr = self[curr].left.unwrap();
                continue;
            }

            let r = self.successor(curr);
            let rb = r.map(|v| {
                let right = self[v].site();
                breakpoint_between(site.x, site.y, right.x, right.y, event.y)
            });

            if let Some(rb) = rb
                && event.x > rb
            {
                curr = self[curr].right.unwrap();
                continue;
            }

            break;
        }

        curr
    }

    pub fn delete(&mut self, key: SQKey) {
        let parent = self[key].parent;

        match (self[key].left, self[key].right) {
            (None, None) => {
                if let Some(parent_key) = parent {
                    let parent = &mut self[parent_key];
                    if parent.left == Some(key) {
                        parent.left = None;
                    }
                    if parent.right == Some(key) {
                        parent.right = None;
                    }
                } else {
                    // No children and no parents, we a root
                    self.head = None;
                }
            }
            (None, Some(child)) | (Some(child), None) => {
                self[child].parent = parent;

                if let Some(parent_key) = parent {
                    let parent = &mut self[parent_key];
                    if parent.left == Some(key) {
                        parent.left = Some(child);
                    }
                    if parent.right == Some(key) {
                        parent.right = Some(child);
                    }
                } else {
                    // We are root, child is new root
                    self.head = Some(child);
                }
            }
            (Some(_), Some(_)) => {
                // Find succesor
                let s = self.successor(key).unwrap();

                let ss = self[s].value;
                let se = self[s].event;

                let cs = self[s].value;
                let ce = self[s].event;

                self[key].value = ss;
                self[key].event = se;

                self[s].value = cs;
                self[s].event = ce;

                self.delete(s);
            }
        }
    }

    pub fn print_debug_node(&self, key: SQKey, depth: usize) {
        if let Some(left) = self[key].left {
            self.print_debug_node(left, depth + 1);
        }

        let curr = self[key];
        if self.head == Some(key) {
            println!(
                "HEAD: ({key:?},{}): l{:?},r{:?},p{:?} - {depth}",
                curr.site().id,
                curr.left,
                curr.right,
                curr.parent
            );
        } else {
            println!(
                "({key:?},{}): l{:?},r{:?},p{:?} - {depth}",
                curr.site().id,
                curr.left,
                curr.right,
                curr.parent
            );
        }

        if let Some(right) = self[key].right {
            self.print_debug_node(right, depth + 1);
        }
    }

    pub fn print_debug(&self) {
        let Some(curr) = self.head else {
            println!("Beachline: Empty");
            return;
        };

        println!("-------");
        self.print_debug_node(curr, 0);
        println!("-------");
    }

    pub fn verify_tree(&self) {
        if let Some(head) = self.head {
            self.assert_child(head, None);
        }
    }

    pub fn assert_child(&self, key: SQKey, parent: Option<SQKey>) {
        assert_eq!(self[key].parent, parent);
        if let Some(left) = self[key].left {
            self.assert_child(left, Some(key));
        }
        if let Some(right) = self[key].right {
            self.assert_child(right, Some(key));
        }
    }

    pub fn sites_intern(&self, key: SQKey, sites: &mut Vec<(usize, SQKey)>) {
        if let Some(left) = self[key].left {
            self.sites_intern(left, sites);
        }

        sites.push((self[key].site().id, key));

        if let Some(right) = self[key].right {
            self.sites_intern(right, sites);
        }
    }

    pub fn sites_intern2(&self, key: SQKey, sites: &mut Vec<Site<T>>) {
        if let Some(left) = self[key].left {
            self.sites_intern2(left, sites);
        }

        sites.push(self[key].value);

        if let Some(right) = self[key].right {
            self.sites_intern2(right, sites);
        }
    }

    pub fn sites(&self, sites: &mut Vec<(usize, SQKey)>) {
        if let Some(head) = self.head {
            self.sites_intern(head, sites);
        }
    }

    pub fn sites2(&self, sites: &mut Vec<Site<T>>) {
        if let Some(head) = self.head {
            self.sites_intern2(head, sites);
        }
    }
}
