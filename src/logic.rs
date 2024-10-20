use btreemultimap::BTreeMultiMap;
use std::{
    cell::RefCell,
    error::Error,
    fs::{self, File},
    io::Write,
    rc::Rc,
};

use serde_json::Value;

use crate::structs::{article::Article, data::Data, des::DES};
pub struct Logic {
    ds_isb: BTreeMultiMap<String, Rc<RefCell<Article>>>,
    ds_name: BTreeMultiMap<String, Rc<RefCell<Article>>>,
}

impl Logic {
    pub fn new() -> Logic {
        Logic {
            ds_isb: BTreeMultiMap::new(),
            ds_name: BTreeMultiMap::new(),
        }
    }

    pub fn execute_methods(&mut self, data: Vec<Data>) -> Result<(), Box<dyn Error>> {
        for element in data {
            match element.method.as_str() {
                "INSERT" => {
                    let article: Article = serde_json::from_str(&element.data.as_str())?;
                    let ptr = Rc::new(RefCell::new(article.clone()));
                    if let Some(isbn) = article.isbn {
                        self.ds_isb.insert(isbn, Rc::clone(&ptr));
                    }
                    if let Some(name) = article.name {
                        self.ds_name.insert(name, Rc::clone(&ptr));
                    }
                }
                "PATCH" => {
                    let key: Value = serde_json::from_str(&element.data.as_str())?;
                    if key["isbn"].is_null() {
                        continue;
                    }
                    if let Some(vec) = self.ds_isb.get_vec(key["isbn"].as_str().unwrap()) {
                        for ptr in vec {
                            let mut article = ptr.borrow_mut();
                            if !key["name"].is_null() {
                                let name = String::from(key["name"].as_str().unwrap());
                                if let Some(original_name) = article.name.clone() {
                                    if let Some(v) = self.ds_name.remove(&original_name) {
                                        self.ds_name.insert_many(name.clone(), v);
                                    }
                                }
                                article.name = Some(name);
                            }
                            if !key["author"].is_null() {
                                article.author =
                                    Some(String::from(key["author"].as_str().unwrap()));
                            }
                            if !key["category"].is_null() {
                                article.category =
                                    Some(String::from(key["category"].as_str().unwrap()));
                            }
                            if !key["price"].is_null() {
                                article.price = Some(String::from(key["price"].as_str().unwrap()));
                            }
                            if !key["quantity"].is_null() {
                                article.quantity =
                                    Some(String::from(key["quantity"].as_str().unwrap()));
                            }
                        }
                    } else {
                        continue;
                    }
                }
                "DELETE" => {
                    let key: Value = serde_json::from_str(&element.data.as_str())?;
                    if key["isbn"].is_null() {
                        continue;
                    }

                    let isbn = key["isbn"].as_str().unwrap();

                    if let Some(vec) = self.ds_isb.get_vec_mut(isbn) {
                        if let Some(ptr) = vec.pop() {
                            let a = ptr.as_ref().borrow();
                            if let Some(name) = &a.name {
                                if self.ds_name.contains_key(name) {
                                    self.ds_name.remove(name);
                                }
                            }
                        }
                    }
                }
                _ => {
                    continue;
                }
            }
        }
        Ok(())
    }

    pub fn search(&mut self, data: Vec<Data>) -> Result<(), Box<dyn Error>> {
        let mut ans = Vec::new();
        for element in data {
            let key: Value = serde_json::from_str(&element.data.as_str())?;
            if key["name"].is_null() {
                continue;
            }
            if element.method != "SEARCH" {
                return Err("Invalid method: must be SEARCH".into());
            }
            if let Some(ptr) = self.ds_name.get_vec(key["name"].as_str().unwrap()) {
                let article = ptr[0].as_ref().borrow_mut();
                ans.push(article);
            }
        }

        // data compression
        // let mut equal = 0;
        // let mut decompress = 0;
        // let mut huffman = 0;
        // let mut arithmetic = 0;
        // let mut either = 0;

        let path = "output.txt";
        let mut file = File::create(path)?;
        for article in ans {
            // data compression
            // article.compress();

            // let namesize = article.namesize.unwrap();
            // let namesizehuffman = article.namesizehuffman.unwrap();
            // let namesizearithmetic = article.namesizearithmetic.unwrap();

            // if namesize == namesizehuffman && namesize == namesizearithmetic {
            //     equal += 1;
            // } else if namesize < namesizehuffman && namesize < namesizearithmetic {
            //     decompress += 1;
            // } else if namesizehuffman < namesize && namesizehuffman < namesizearithmetic {
            //     huffman += 1;
            // } else if namesizearithmetic < namesize && namesizearithmetic < namesizehuffman {
            //     arithmetic += 1;
            // } else {
            //     either += 1;
            // }

            println!("{}", article);
            writeln!(file, "{}", article)?;
        }

        // data compression
        // writeln!(file, "Equal: {}\r", equal)?;
        // writeln!(file, "Decompress: {}\r", decompress)?;
        // writeln!(file, "Huffman: {}\r", huffman)?;
        // writeln!(file, "Arithmetic: {}\r", arithmetic)?;
        // writeln!(file, "Either: {}\r", either)?;

        let encrypter = DES::new();
        let key = b"ok:uo1IN";
        let mut encrypted_file = File::create("encrypted")?;
        encrypted_file.write(&encrypter.encrypt(fs::read(path)?, key, true)?)?;

        Ok(())
    }
}
