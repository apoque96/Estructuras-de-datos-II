use serde::{Deserialize, Serialize};
use std::fmt;

use crate::huffman::huffman_tree::HuffmanTree;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Article {
    pub isbn: Option<String>,
    pub name: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub price: Option<String>,
    pub quantity: Option<String>,
}

impl Article {
    fn to_string_excluding_none(&self) -> String {
        let mut parts = vec![];

        if let Some(ref isbn) = self.isbn {
            parts.push(format!("\"isbn\":\"{}\"", isbn));
        }
        if let Some(ref name) = self.name {
            parts.push(format!("\"name\":\"{}\"", name));

            //I'm way to lazy to refactor the code just so it doesn't panic when no name
            let tree = HuffmanTree::build(name.to_string()).expect("Error, no name found");
            tree.encode(name.to_string());
        }
        if let Some(ref author) = self.author {
            parts.push(format!("\"author\":\"{}\"", author));
        }
        if let Some(ref category) = self.category {
            parts.push(format!("\"category\":\"{}\"", category));
        }
        if let Some(ref price) = self.price {
            parts.push(format!("\"price\":\"{}\"", price));
        }
        if let Some(ref quantity) = self.quantity {
            parts.push(format!("\"quantity\":\"{}\"", quantity));
        }

        if let Some(ref name) = self.name {
            //I'm way to lazy to refactor the code just so it doesn't panic when no name
            let tree = HuffmanTree::build(name.to_string()).expect("Error, no name found");
            // tree.print_tree();
            // tree.print_symbols();
            parts.push(format!("\"namesize\":\"{}\"", name.len() * 2));
            parts.push(format!(
                "\"namesizehuffman\":\"{}\"",
                tree.encode(name.to_string()).len()
            ));
        }

        parts.join(",")
    }
}

impl fmt::Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}}}\r", self.to_string_excluding_none())
    }
}
