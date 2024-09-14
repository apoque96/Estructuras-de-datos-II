use std::{cell::RefCell, cmp::Ordering, rc::Rc};
#[derive(Debug, Clone)]
pub struct HuffmanNode {
    pub frequency: usize,
    pub symbol: char,
    pub left: Option<Rc<RefCell<HuffmanNode>>>,
    pub right: Option<Rc<RefCell<HuffmanNode>>>,
}

impl HuffmanNode {
    pub fn new(frequency: usize, symbol: char) -> HuffmanNode {
        HuffmanNode {
            frequency,
            symbol,
            left: None,
            right: None,
        }
    }

    pub fn new_with_children(
        frequency: usize,
        symbol: char,
        left: HuffmanNode,
        right: HuffmanNode,
    ) -> HuffmanNode {
        HuffmanNode {
            frequency,
            symbol,
            left: Some(Rc::new(RefCell::new(left))),
            right: Some(Rc::new(RefCell::new(right))),
        }
    }

    pub fn print_tree(&self, level: usize) {
        for _ in 0..(level * 4) {
            print!(" ");
        }
        println!(" {}({}) ", self.symbol, self.frequency);

        if let Some(left) = &self.left {
            left.borrow().print_tree(level + 1);
        }

        if let Some(right) = &self.right {
            right.borrow().print_tree(level + 1);
        }
    }

    pub fn traverse(
        root: Option<Rc<RefCell<HuffmanNode>>>,
        symbol: char,
        data: Vec<bool>,
    ) -> Option<Vec<bool>> {
        match root {
            Some(node_ref) => {
                let left = node_ref.borrow().left.to_owned();
                let right = node_ref.borrow().right.to_owned();

                if left.is_none() && right.is_none() {
                    if symbol == node_ref.borrow().symbol {
                        return Some(data);
                    } else {
                        return None;
                    }
                } else {
                    let mut left_vec: Option<Vec<bool>> = None;
                    let mut right_vec: Option<Vec<bool>> = None;

                    if left.is_some() {
                        let mut left_path = Vec::new();

                        let mut temp = data.clone();
                        left_path.append(&mut temp);
                        left_path.push(false);

                        left_vec = HuffmanNode::traverse(left, symbol, left_path);
                    }

                    if right.is_some() {
                        let mut right_path = Vec::new();

                        let mut temp = data.clone();
                        right_path.append(&mut temp);
                        right_path.push(true);

                        right_vec = HuffmanNode::traverse(right, symbol, right_path);
                    }

                    if left_vec.is_some() {
                        left_vec
                    } else {
                        right_vec
                    }
                }
            }
            None => return None,
        }
    }
}

impl PartialEq for HuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency && self.symbol == other.symbol
    }
}

impl Eq for HuffmanNode {}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // First compare by frequency
        match self.frequency.cmp(&other.frequency) {
            Ordering::Equal => {
                // If frequencies are equal, compare by symbol
                self.symbol.cmp(&other.symbol)
            }
            other => other, // Otherwise return the frequency comparison
        }
    }
}
