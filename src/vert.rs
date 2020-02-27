use super::*;
use std::fmt::Debug;

pub struct Vertex<V> {
    pub index: usize,
    pub val: V,
    pub edges: Vec<usize>,
}

impl<V> Deref for Vertex<V> {
    type Target = V;
    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<V> DerefMut for Vertex<V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.val
    }
}

impl<V: Debug> Debug for Vertex<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Vertex")
            .field("index", &self.index)
            .field("val", &self.val)
            .field("edges", &self.edges)
            .finish()
    }
}

impl<V> Vertex<V> {
    pub fn edges<'v, 'g, E, F: Flow>(
        &'v self,
        graph: &'g Graph<V, E, F>,
    ) -> EdgeIter<'v, 'g, V, E, F> {
        EdgeIter {
            vert: self,
            graph,
            dir: Direction::None,
            i: 0,
        }
    }

    pub fn outgoing<'v, 'g, E>(
        &'v self,
        graph: &'g Graph<V, E, Directed>,
    ) -> EdgeIter<'v, 'g, V, E, Directed> {
        EdgeIter {
            vert: self,
            graph,
            dir: Direction::Out,
            i: 0,
        }
    }

    pub fn incoming<'v, 'g, E>(
        &'v self,
        graph: &'g Graph<V, E, Directed>,
    ) -> EdgeIter<'v, 'g, V, E, Directed> {
        EdgeIter {
            vert: self,
            graph,
            dir: Direction::In,
            i: 0,
        }
    }

    pub fn swap(&mut self, n: &mut V) {
        std::mem::swap(&mut self.val, n);
    }

    // pub fn edges_mut<'e, E>(
    //     &self,
    //     graph: &'e mut Graph<V, E>,
    // ) -> std::iter::Map<std::iter::Cloned<std::slice::Iter<usize>>, impl FnMut(usize) -> &'e mut E>
    // {
    //     self.edges.iter().cloned().map(|e| &mut graph.edges[e])
    // }
}

pub struct EdgeIter<'v, 'g, V, E, F: Flow> {
    vert: &'v Vertex<V>,
    graph: &'g Graph<V, E, F>,
    dir: Direction,
    i: usize,
}

impl<'v, 'g, V, E, F: Flow> Iterator for EdgeIter<'v, 'g, V, E, F> {
    type Item = &'g Edge<E>;
    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.vert.edges.len() {
            let edge = self.graph.edge(self.vert.edges[self.i]);
            self.i += 1;
            match self.dir {
                Direction::None => return Some(edge),
                Direction::Out => {
                    if self.vert.index == edge.verts.0 {
                        return Some(edge);
                    }
                }
                Direction::In => {
                    if self.vert.index == edge.verts.1 {
                        return Some(edge);
                    }
                }
            }
        }
        None
    }
}
