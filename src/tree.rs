use super::*;
use std::ops::{Index, IndexMut};

pub struct Tree<V>(pub Graph<V, (), Directed>);

impl<V> Index<usize> for Tree<V> {
    type Output = V;
    fn index(&self, k: usize) -> &Self::Output {
        self.0.vert(k)
    }
}

impl<V> IndexMut<usize> for Tree<V> {
    fn index_mut(&mut self, k: usize) -> &mut Self::Output {
        self.0.vert_mut(k)
    }
}

impl<V> Tree<V> {
    pub fn new(root: V) -> Self {
        let mut res = Self(Graph::new());
        res.0.add_vert(root);
        res
    }

    pub fn add_child(&mut self, parent: usize, child: V) -> usize {
        let child = self.0.add_vert(child);
        self.0.add_edge(parent, (), child);
        child
    }

    /// Removes a node and gives ownership of its children to its parent, inserting edges from
    /// parent to descendants starting at the index of the child.
    pub fn rem_child(&mut self, index: usize) -> (Vertex<V>, Vec<Edge<()>>) {
        // Must not be root.
        assert!(index != 0);
        unimplemented!()
    }

    pub fn add_tree(&mut self, parent: usize, child: Self) -> (usize, Converter) {
        let conv = self.0.merge(child.0);
        let child = conv.conv(0).unwrap();
        self.0.add_edge(parent, (), child);
        (child, conv)
    }

    pub fn rem_tree(&mut self, index: usize) -> (Self, Converter) {
        assert!(index != 0);
        self.0
            .rem_edge(self.0.vert(index).incoming(&self.0).next().unwrap().index);
        let (graph, conv) = self.0.isolate(index);
        (Self(graph), conv)
    }

    pub fn parent(&self, child: usize) -> Result<usize, usize> {
        if child == 0 {
            Err(0)
        } else {
            Ok(self.0.vert(child).incoming(&self.0).next().unwrap().verts.0)
        }
    }

    pub fn children(&self, parent: usize) -> Vec<usize> {
        self.0
            .vert(parent)
            .outgoing(&self.0)
            .map(|e| e.verts.1)
            .collect()
    }
}

impl<V: Debug> Debug for Tree<V> {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        unimplemented!()
    }
}
