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

    /// Removes a node and gives ownership of its children to its parent, inserting edges from
    /// parent to descendants starting at the index of the child.
    pub fn rem_child(&mut self, index: usize) -> (Vertex<V>, Vec<Edge<()>>) {
        // Must not be root.
        assert!(index != 0);
        unimplemented!()
    }

    pub fn add_tree(&mut self, parent: usize, child: Self) -> usize {
        let child = self.0.merge(child.0)[0];
        self.0.add_edge(parent, (), child);
        child
    }

    pub fn rem_tree(&mut self, index: usize) -> Self {
        assert!(index != 0);
        self.0
            .rem_edge(self.0.vert(index).incoming(&self.0).next().unwrap().index);
        Self(self.0.isolate(index))
    }

    pub fn parent(&self, child: usize) -> usize {
        assert!(child != 0);
        self.0.vert(child).incoming(&self.0).next().unwrap().verts.0
    }

    pub fn children(&self, parent: usize) -> Vec<usize> {
        self.0
            .vert(parent)
            .outgoing(&self.0)
            .map(|e| e.verts.1)
            .collect()
    }
}
