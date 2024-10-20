use serde::{Deserialize, Serialize};
use std::fmt;

use crate::{arithmetic::ArithmeticCoding, huffman::huffman_tree::HuffmanTree};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Article {
    pub isbn: Option<String>,
    pub name: Option<String>,
    pub author: Option<String>,
    pub category: Option<String>,
    pub price: Option<String>,
    pub quantity: Option<String>,
    pub namesize: Option<usize>,
    pub namesizehuffman: Option<usize>,
    pub namesizearithmetic: Option<usize>,
}

impl Article {
    pub fn compress(&mut self) {
        if let Some(ref name) = self.name {
            let namesize = name.len() * 16;
            self.namesize = Some(namesize);

            let namesizehuffman = HuffmanTree::build(name.to_string())
                .expect("Error, no name found")
                .encode(name.to_string())
                .len();
            self.namesizehuffman = Some(namesizehuffman);

            let mut namesizearithmetic = ArithmeticCoding::new(name.to_string())
                .compress(name.to_string())
                .len();

            if namesizearithmetic % 8 != 0 {
                namesizearithmetic -= namesizearithmetic % 8;
                namesizearithmetic += 8;
            }

            self.namesizearithmetic = Some(namesizearithmetic);
        }
    }

    fn to_string_excluding_none(&self) -> String {
        let mut parts = vec![];

        if let Some(ref isbn) = self.isbn {
            parts.push(format!("\"isbn\":\"{}\"", isbn));
        }
        if let Some(ref name) = self.name {
            parts.push(format!("\"name\":\"{}\"", name));
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

        // data compression
        // parts.push(format!("\"namesize\":\"{}\"", self.namesize.unwrap() / 8));
        // parts.push(format!(
        //     "\"namesizehuffman\":\"{}\"",
        //     self.namesizehuffman.unwrap()
        // ));
        // parts.push(format!(
        //     "\"namesizearithmetic\":\"{}\"",
        //     self.namesizearithmetic.unwrap() / 8
        // ));

        parts.join(",")
    }
}

impl fmt::Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}}}\r", self.to_string_excluding_none())
    }
}
