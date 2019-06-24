use super::*;
use std::fmt::Debug;

pub struct Edge<E> {
    pub index: usize,
    pub weight: E,
    /// 0 -> 1, if this is a directed edge
    pub verts: (usize, usize),
}

impl<E: Debug> Debug for Edge<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Edge")
            .field("index", &self.index)
            .field("weight", &self.weight)
            .field("verts", &self.verts)
            .finish()
    }
}

impl<E> Deref for Edge<E> {
    type Target = E;
    fn deref(&self) -> &Self::Target {
        &self.weight
    }
}

impl<E> DerefMut for Edge<E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.weight
    }
}

impl<E> Edge<E> {
    pub fn new(index: usize, weight: E, from: usize, to: usize) -> Self {
        Self {
            index,
            weight,
            verts: (from, to),
        }
    }

    pub fn start<'v, V>(&self, graph: &'v Graph<V, E>) -> &'v Vertex<V> {
        graph.vert(self.verts.0)
    }

    pub fn end<'v, V>(&self, graph: &'v Graph<V, E>) -> &'v Vertex<V> {
        graph.vert(self.verts.1)
    }

    pub fn verts<'v, V>(&self, graph: &'v Graph<V, E>) -> (&'v Vertex<V>, &'v Vertex<V>) {
        (self.start(graph), self.end(graph))
    }

    pub fn start_mut<'v, V>(&self, graph: &'v mut Graph<V, E>) -> &'v mut Vertex<V> {
        graph.vert_mut(self.verts.0)
    }

    pub fn end_mut<'v, V>(&self, graph: &'v mut Graph<V, E>) -> &'v mut Vertex<V> {
        graph.vert_mut(self.verts.1)
    }
}
