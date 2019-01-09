#![feature(pin)]

// pub extern crate petgraph;

use std::collections::VecDeque;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::ops::Add;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug)]
pub struct Leaf<V> {
    pub value: V,
    branch: Option<usize>,
    leaves: Vec<usize>,
}

impl<V> Leaf<V> {
    pub fn with_val(value: V) -> Leaf<V> {
        Leaf {
            value,
            branch: None,
            leaves: Vec::new(),
        }
    }

    /// Returns a clone of self.branch
    pub fn branch(&self) -> Option<usize> {
        self.branch
    }

    pub fn leaves(&self) -> &[usize] {
        &self.leaves[..]
    }
}

impl<V> Index<usize> for Leaf<V> {
    type Output = usize;
    fn index(&self, index: usize) -> &usize {
        &self.leaves[index]
    }
}

/// Indices are stable across removal/insertion.
pub struct Tree<L> {
    clean: bool,
    nodes: Vec<Option<Leaf<L>>>,
}

impl<L> Into<Vec<Option<Leaf<L>>>> for Tree<L> {
    fn into(self) -> Vec<Option<Leaf<L>>> {
        self.nodes
    }
}

impl<L> From<Vec<Option<Leaf<L>>>> for Tree<L> {
    fn from(vec: Vec<Option<Leaf<L>>>) -> Tree<L> {
        Tree {
            clean: false,
            nodes: vec,
        }
    }
}

impl<L> Tree<L> {
    pub fn with_root(root: L) -> Tree<L> {
        Tree {
            clean: true,
            nodes: vec![Some(Leaf {
                value: root,
                branch: None,
                leaves: Vec::new(),
            })],
        }
    }

    pub fn nodes(&self) -> &[Option<Leaf<L>>] {
        &self.nodes[..]
    }

    pub fn insert(&mut self, branch: usize, index: usize, leaf: L) {
        let nindex = self.size();
        self.nodes.push(Some(Leaf::with_val(leaf)));
        self[branch].leaves.insert(index, nindex);
        let node = &mut self[nindex];
        node.branch = Some(branch);
    }

    pub fn insert_tree(&mut self, branch: usize, index: usize, mut tree: Tree<L>) {
        if !tree.clean {
            tree.clean();
        }
        let start = self.size();
        self.nodes.append(&mut tree.nodes);
        self[branch].leaves.insert(index, start);

        // Since the nodes of the added tree are all going to be in one block on the end,
        // we can just shift their internal references
        for n in &mut self.nodes[start..] {
            let mut n = n.as_mut().unwrap();
            if let Some(ref mut b) = &mut n.branch {
                *b += start
            }
            for leaf in n.leaves.iter_mut() {
                *leaf += start
            }
        }

        self[start].branch = Some(branch);
    }

    /// Add a leaf to the beginning of a node
    pub fn queue(&mut self, branch: usize, leaf: L) {
        self.insert(branch, 0, leaf);
    }

    /// Add a leaf to the end of a node
    pub fn push(&mut self, branch: usize, leaf: L) {
        let len = self[branch].leaves.len();
        self.insert(branch, len, leaf)
    }

    /// Add a tree to the beginning of a node
    pub fn queue_tree(&mut self, branch: usize, tree: Tree<L>) {
        self.insert_tree(branch, 0, tree);
    }

    /// Add a tree to the end of a node
    pub fn push_tree(&mut self, branch: usize, tree: Tree<L>) {
        let len = self[branch].leaves.len();
        self.insert_tree(branch, len, tree)
    }

    pub fn remove(&mut self, node: usize) -> Tree<L> {
        // Alright so basically the reason this works is that we don't actually remove anything
        // from the node list, so indices are stable
        fn recurse<L>(from: &mut Tree<L>, to: &mut Tree<L>, new_p: usize, old_c: &[usize]) {
            // for each leaf in old_c, take the leaf from the original tree and give it to me
            // as n
            for n in old_c
                .iter()
                .map(|i| from.nodes[*i].take().unwrap())
                .collect::<Vec<Leaf<L>>>()
            {
                let (val, leaves) = (n.value, n.leaves);
                to.queue(new_p, val);
                let p = to.size() - 1;
                recurse(from, to, p, &leaves[..])
            }
            // Because of the way this recurses, the tree created is "column-major"
        }
        {
            let branch = self[node]
                .branch
                .expect("Tried to remove root || Tried to remove leaf with no branch");
            let p_node = &mut self[branch];
            let nindex = p_node.leaves.iter().position(|i| *i == node).unwrap();
            p_node.leaves.remove(nindex);
        }
        let r_node = self.nodes[node].take().unwrap();
        let (r_val, rc) = (r_node.value, r_node.leaves);
        let mut res = Tree::with_root(r_val);
        recurse(self, &mut res, 0, &rc[..]);
        self.clean = false;
        res
    }

    /// Removes Nones from self.nodes, fixes pointers in Somes, returns locations of removed objects
    /// from largest index to smallest
    pub fn clean(&mut self) -> VecDeque<usize> {
        self.clean = true;
        let mut res = VecDeque::new();
        let mut i = 0;
        let mut shift = 0;
        while i < self.size() {
            if self.nodes[i].is_none() {
                res.push_front(i + shift);
                self.nodes.remove(i);
                i -= 1;
                shift += 1;
            }
            i += 1;
        }
        let len = res.len();
        for node in &mut self.nodes {
            let mut node = node.as_mut().unwrap();
            if let Some(ref mut b) = node.branch {
                if *b >= res[len - 1] {
                    for del in &res {
                        if *b > *del {
                            *b -= 1;
                        }
                    }
                }
            }
            for leaf in &mut node.leaves {
                if *leaf < res[len - 1] {
                    continue;
                }
                for del in &res {
                    if *leaf > *del {
                        *leaf -= 1;
                    }
                }
            }
        }
        res
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn first_nearest<C, N: Fn(&L, &C) -> bool>(&self, c: &C, near: N) -> usize
    where
        L: PartialEq<C>,
    {
        fn recurse<C, L: PartialEq<C>, N: Fn(&L, &C) -> bool>(
            tree: &Tree<L>,
            branch: usize,
            c: &C,
            near: N,
        ) -> usize {
            let leaves = {
                let node = &tree[branch];
                if node.value == *c {
                    return branch;
                }
                node.leaves()
            };
            for leaf in leaves {
                if near(&tree[*leaf].value, c) {
                    return recurse(tree, *leaf, c, near);
                }
            }
            branch
        }
        recurse(&self, 0, c, near)
    }

    pub fn find_or_insert<N: Fn(&L, &L) -> bool>(&mut self, l: L, near: N) -> usize
    where
        L: PartialEq,
    {
        fn recurse<L: PartialEq, N: Fn(&L, &L) -> bool>(
            tree: &mut Tree<L>,
            branch: usize,
            l: L,
            near: N,
        ) -> usize {
            let leaves: Vec<usize> = {
                let node = &tree[branch];
                if node.value == l {
                    return branch;
                }
                node.leaves().to_vec()
            };
            for leaf in leaves {
                if near(&tree[leaf].value, &l) {
                    return recurse(tree, leaf, l, near);
                }
            }
            tree.push(branch, l);
            tree.size() - 1
        }
        recurse(self, 0, l, near)
    }

    pub fn all_nearest<C, N: Copy + Fn(&L, &C) -> bool>(&self, c: &C, near: N) -> Vec<usize>
    where
        L: PartialEq<C>,
    {
        fn recurse<C, L: PartialEq<C>, N: Copy + Fn(&L, &C) -> bool>(
            tree: &Tree<L>,
            branch: usize,
            c: &C,
            near: N,
        ) -> Vec<usize> {
            let mut res = vec![branch];
            let leaves = {
                let node = &tree[branch];
                if node.value == *c {
                    // anything past this is more specific and doesn't match
                    return res;
                }
                node.leaves()
            };
            for leaf in leaves {
                if near(&tree[*leaf].value, c) {
                    res.append(&mut recurse(tree, *leaf, c, near))
                }
            }
            res
        }
        recurse(&self, 0, c, near)
    }

    pub fn replace(&mut self, branch: usize, mut new_leaves: Tree<L>, new_branch: L) {
        self.nodes[branch] = Some(Leaf::with_val(new_branch));
        for leaf in new_leaves[0].leaves().to_vec() {
            self.push_tree(branch, new_leaves.remove(leaf))
        }
    }

    pub fn ancestors(&self, leaf: usize) -> Vec<usize> {
        let mut res = Vec::new();
        if let Some(b) = self[leaf].branch {
            res.append(&mut self.ancestors(b));
            res.push(b);
        } else {
            res.push(leaf);
        }
        res
    }

    pub fn descendants(&self, leaf: usize) -> Vec<usize> {
        let mut res = Vec::new();
        for leaf in self[leaf].leaves() {
            res.push(*leaf);
            res.append(&mut self.descendants(*leaf));
        }
        res
    }
}

impl<L: Debug> Debug for Tree<L> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        use std::fmt::Error;
        fn tab(depth: usize) -> String {
            let mut res = String::new();
            for _i in 0..depth {
                res = res.add("|  ")
            }
            res
        }
        fn recurse<L: Debug>(
            tree: &Tree<L>,
            f: &mut Formatter,
            branch: usize,
            depth: usize,
        ) -> Result<(), Error> {
            writeln!(f, "{}{} :: {:?}", tab(depth), branch, tree[branch].value);
            for leaf in tree[branch].leaves() {
                recurse(tree, f, *leaf, depth + 1)?
            }
            Ok(())
        }
        recurse(self, f, 0, 0)
    }
}

impl<L> Index<usize> for Tree<L> {
    type Output = Leaf<L>;
    fn index(&self, index: usize) -> &Leaf<L> {
        self.nodes[index].as_ref().unwrap()
    }
}

impl<L> IndexMut<usize> for Tree<L> {
    fn index_mut(&mut self, index: usize) -> &mut Leaf<L> {
        self.nodes[index].as_mut().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test() -> Tree<usize> {
        let mut res = Tree::with_root(0);
        res.push(0, 1);
        res.push(1, 2);
        res.push(1, 3);
        res.push(0, 4);
        res.push(4, 5);
        res.push(4, 6);
        res.push(1, 7);
        res.push(1, 8);
        res
    }

    #[test]
    fn with_root() {
        println!("{:?}", Tree::with_root(0))
    }

    #[test]
    fn print() {
        println!("{:?}", make_test())
    }

    #[test]
    fn remove() {
        let mut tree = make_test();
        println!("Base:\n{:?}", tree);
        let rem = tree.remove(1);
        println!("Removed:\n{:?}", rem);
        println!("Result:\n{:?}", tree);
        tree.clean();
        println!("Cleaned:\n{:?}", tree);
    }
}
