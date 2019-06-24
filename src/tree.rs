use super::*;
#[derive(Debug, Default)]
pub struct Tree<V>(pub Graph<V, (), Directed>);

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

    /// Removes a node and gives ownership of its children to its parent, inserting the edges
    pub fn rem_child(&mut self, _parent: usize, _child: usize) -> Vertex<V> {
        unimplemented!()
    }

    pub fn add_tree(&mut self, parent: usize, child: Self) -> usize {
        let child = self.0.merge(child.0)[0];
        self.0.add_edge(parent, (), child);
        child
    }
}
