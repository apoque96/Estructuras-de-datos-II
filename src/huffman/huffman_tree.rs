use std::{cell::RefCell, collections::HashMap, io, rc::Rc};

use super::huffman_node::HuffmanNode;
use bit_vec::BitVec;

pub struct HuffmanTree {
    nodes: Vec<HuffmanNode>,
    fruequencies: HashMap<char, usize>,
    pub root: Option<Rc<RefCell<HuffmanNode>>>,
}

impl HuffmanTree {
    pub fn build(s: String) -> Result<HuffmanTree, io::Error> {
        let mut tree = HuffmanTree {
            nodes: Vec::new(),
            root: None,
            fruequencies: HashMap::new(),
        };

        tree.calculate_fruequencies(s.clone());

        for c in tree.fruequencies.keys() {
            if let Some(k) = tree.fruequencies.get(c) {
                tree.nodes.push(HuffmanNode::new(k.clone(), c.clone()))
            }
        }

        while tree.nodes.len() >= 2 {
            tree.nodes.sort();

            let left = tree.nodes[0].clone();
            let right = tree.nodes[1].clone();

            // '|' == non-leaf Node
            let parent =
                HuffmanNode::new_with_children(left.frequency + right.frequency, '|', left, right);

            tree.nodes.remove(1);
            tree.nodes.remove(0);

            tree.nodes.push(parent);
        }

        tree.root = Some(Rc::new(RefCell::new(tree.nodes.pop().unwrap())));

        Ok(tree)
    }

    pub fn encode(&self, s: String) -> BitVec<u8> {
        let mut encoded_source = Vec::new();

        for symbol in s.chars() {
            if let Some(mut encoded_symbol) =
                HuffmanNode::traverse(self.root.clone(), symbol, Vec::new())
            {
                encoded_source.append(&mut encoded_symbol);
            }
        }

        BitVec::from_iter(encoded_source)
    }

    pub fn print_tree(&self) {
        if let Some(root) = &self.root {
            root.borrow().print_tree(0);
        }
    }

    pub fn print_symbols(&self) {
        for item in self.fruequencies.clone() {
            if let Some(root) = &self.root {
                let a = HuffmanNode::traverse(Some(root.clone()), item.0, Vec::new()).unwrap();
                let b: Vec<char> = a
                    .iter()
                    .map(|e| {
                        if *e == true {
                            return '1';
                        } else {
                            return '0';
                        }
                    })
                    .collect();
                println!("Symbol: {} - Code: {:?}", item.0, b);
            }
        }
    }

    fn calculate_fruequencies(&mut self, s: String) {
        for c in s.chars() {
            self.fruequencies
                .entry(c)
                .and_modify(|f| *f += 1)
                .or_insert(1);
        }
    }
}
