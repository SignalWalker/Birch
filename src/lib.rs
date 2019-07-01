use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Deref;
use std::ops::DerefMut;

mod edge;
mod tree;
mod vert;
pub use edge::*;
pub use tree::*;
pub use vert::*;

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

    /// Adds a vertex to the graph and returns its index, which is either self.verts.len()
    /// or the old index of the most recently removed vert.
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

    /// Removes a vertex, preserving indices.
    /// # Returns
    /// The removed vertex and all edges to/from it.
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
        for (i, (vert, pos)) in [(start, f_pos), (end, t_pos)].iter().enumerate() {
            if start == end && i != 0 {
                break;
            }
            let vert = self.vert_mut(*vert);
            let at = match pos {
                Some(p) => std::cmp::min(*p, vert.edges.len()),
                None => vert.edges.len(),
            };
            vert.edges.insert(at, index);
        }
        index
    }

    pub fn add_edge(&mut self, start: usize, weight: E, end: usize) -> usize {
        self.insert_edge(start, weight, end, None, None)
    }

    pub fn set_edge(&mut self, start: usize, weight: E, end: usize) -> (usize, Option<E>) {
        self.replace_edge(start, weight, end, |e| {
            e.verts == (start, end) || (!F::DIR && e.verts == (end, start))
        })
    }

    pub fn replace_edge(
        &mut self,
        start: usize,
        mut weight: E,
        end: usize,
        pred: impl FnMut(&&Edge<E>) -> bool,
    ) -> (usize, Option<E>) {
        match self.vert(start).edges(&self).find(pred).map(|e| e.index) {
            Some(index) => {
                let edge = self.edge_mut(index);
                std::mem::swap(&mut edge.weight, &mut weight);
                (edge.index, Some(weight))
            }
            None => (self.add_edge(start, weight, end), None),
        }
    }

    /// Remove an edge, preserving indices.
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

    /// Moves verts & edges into empty cells until there are no empty cells.
    /// Does not preserve indices.
    pub fn compress(&mut self) {
        unimplemented!()
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

    /// Creates a new graph by removing a region from self.
    pub fn isolate(&mut self, _index: usize) -> Self {
        unimplemented!()
    }
}
