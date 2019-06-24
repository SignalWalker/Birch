use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

mod tree;
pub use tree::*;

pub trait Flow {
    const DIR: bool;
}

macro_rules! flow {
    ($i:ident, $d:literal) => {
        #[derive(Debug, Default, Hash, Eq, PartialEq, Copy, Clone)]
        pub struct $i;
        impl Flow for $i {
            const DIR: bool = $d;
        }
    };
}

flow!(Directed, true);
flow!(Undirected, false);

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    In,
    Out,
    None,
}

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

    // pub fn edges_mut<'e, E>(
    //     &self,
    //     graph: &'e mut Graph<V, E>,
    // ) -> std::iter::Map<std::iter::Cloned<std::slice::Iter<usize>>, impl FnMut(usize) -> &'e mut E>
    // {
    //     self.edges.iter().cloned().map(|e| &mut graph.edges[e])
    // }
}

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

#[derive(Default)]
pub struct Graph<V, E, F: Flow = Undirected> {
    pub verts: Vec<Option<Vertex<V>>>,
    pub edges: Vec<Option<Edge<E>>>,
    empty_v: Vec<usize>,
    empty_e: Vec<usize>,
    flow: PhantomData<F>,
}

impl<V: Debug, E: Debug, F: Flow + Debug> Debug for Graph<V, E, F> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("Graph")
            .field("verts", &self.verts)
            .field("edges", &self.edges)
            .field("empty_v", &self.empty_v)
            .field("empty_e", &self.empty_e)
            .finish()
    }
}

impl<V, E, F: Flow> Graph<V, E, F> {
    pub fn new() -> Self {
        Self {
            verts: Vec::new(),
            edges: Vec::new(),
            empty_v: Vec::new(),
            empty_e: Vec::new(),
            flow: PhantomData,
        }
    }

    pub fn vert(&self, i: usize) -> &Vertex<V> {
        self.verts[i].as_ref().unwrap()
    }

    pub fn edge(&self, i: usize) -> &Edge<E> {
        self.edges[i].as_ref().unwrap()
    }

    pub fn vert_mut(&mut self, i: usize) -> &mut Vertex<V> {
        self.verts[i].as_mut().unwrap()
    }

    pub fn edge_mut(&mut self, i: usize) -> &mut Edge<E> {
        self.edges[i].as_mut().unwrap()
    }

    pub fn add_vert(&mut self, vert: V) -> usize {
        let index = {
            match self.empty_v.pop() {
                Some(i) => i,
                None => self.verts.len(),
            }
        };
        let vert = Vertex {
            index,
            val: vert,
            edges: Vec::new(),
        };
        if index == self.verts.len() {
            self.verts.push(Some(vert));
        } else {
            self.verts[index] = Some(vert);
        }
        index
    }

    pub fn rem_vert(&mut self, index: usize) -> (Vertex<V>, Vec<Edge<E>>) {
        let vert = self.verts[index].take().unwrap();
        let edges = vert
            .edges(&self)
            .map(|e| e.index)
            .collect::<Vec<_>>()
            .drain(0..)
            .map(|e| self.rem_edge(e))
            .collect::<Vec<_>>();
        self.empty_v.push(index);
        (vert, edges)
    }

    pub fn insert_edge(
        &mut self,
        start: usize,
        weight: E,
        end: usize,
        f_pos: Option<usize>,
        t_pos: Option<usize>,
    ) -> usize {
        let index = {
            match self.empty_e.pop() {
                Some(i) => i,
                None => self.edges.len(),
            }
        };
        let edge = Edge {
            index,
            weight,
            verts: (start, end),
        };
        if index == self.edges.len() {
            self.edges.push(Some(edge));
        } else {
            self.edges[index] = Some(edge);
        }
        for vert in &[start, end] {
            let vert = self.vert_mut(*vert);

            vert.edges.push(index);
        }
        index
    }

    pub fn add_edge(&mut self, start: usize, weight: E, end: usize) -> usize {
        self.insert_edge(start, weight, end, None, None)
    }

    pub fn rem_edge(&mut self, index: usize) -> Edge<E> {
        let edge = self.edges[index].take().unwrap();
        self.empty_e.push(index);
        for vert in &[edge.verts.0, edge.verts.1] {
            if let Some(vert) = self.verts[*vert].as_mut() {
                vert.edges
                    .remove(vert.edges.iter().position(|e| *e == edge.index).unwrap());
            }
        }
        edge
    }

    /// Add another graph to this one as its own disconnected region.
    /// # Returns
    /// New indices of added vertices, in the same order in which they were in the old graph.
    pub fn merge(&mut self, _other: Self) -> Vec<usize> {
        unimplemented!()
    }

    /// Split disconnected graph into connected regions.
    pub fn split(self) -> Vec<Self> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
