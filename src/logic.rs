use std::{cell::RefCell, collections::BTreeMap, error::Error, fs::File, io::Write, rc::Rc};

use serde_json::Value;

use crate::structs::{article::Article, data::Data};
pub struct Logic {
    ds_isb: BTreeMap<String, Rc<RefCell<Article>>>,
    ds_name: BTreeMap<String, Rc<RefCell<Article>>>,
}

impl Logic {
    pub fn new() -> Logic {
        Logic {
            ds_isb: BTreeMap::new(),
            ds_name: BTreeMap::new(),
        }
    }

    pub fn execute_methods(&mut self, data: Vec<Data>) -> Result<(), Box<dyn Error>> {
        // println!("{:?}", data);
        for element in data {
            match element.method.as_str() {
                "INSERT" => {
                    let article: Article = serde_json::from_str(&element.data.as_str())?;
                    let ptr = Rc::new(RefCell::new(article.clone()));
                    if let Some(isbn) = article.isbn {
                        self.ds_isb.entry(isbn).or_insert(Rc::clone(&ptr));
                    }
                    if let Some(name) = article.name {
                        self.ds_name.entry(name).or_insert(Rc::clone(&ptr));
                    }
                }
                "PATCH" => {
                    let key: Value = serde_json::from_str(&element.data.as_str())?;
                    if key["isbn"].is_null() {
                        continue;
                    }
                    if let Some(ptr) = self.ds_isb.get(key["isbn"].as_str().unwrap()) {
                        let mut article = ptr.borrow_mut();
                        if !key["name"].is_null() {
                            article.name = Some(String::from(key["name"].as_str().unwrap()));
                        }
                        if !key["author"].is_null() {
                            article.author = Some(String::from(key["author"].as_str().unwrap()));
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
                    } else {
                        continue;
                    }
                }
                "DELETE" => {
                    let key: Value = serde_json::from_str(&element.data.as_str())?;
                    if key["isbn"].is_null() {
                        continue;
                    }
                    if let Some(ptr) = self.ds_isb.remove(key["isbn"].as_str().unwrap()) {
                        let a = ptr.as_ref().borrow();
                        if let Some(name) = &a.name {
                            self.ds_name.remove(name);
                        }
                    }
                }
                _ => {
                    return Err("Incorrect method(valid methods: INSERT, PATCH, DELETE)".into());
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
            if let Some(ptr) = self.ds_name.get(key["name"].as_str().unwrap()) {
                let article = ptr.as_ref().borrow();
                ans.push(article);
            }
        }

        let path = "output.txt";
        let mut file = File::create(path)?;
        print!("\x1B[2J\x1B[1;1H");
        for article in ans {
            // let json = serde_json::to_string(&*article).unwrap();
            println!("{}", article);
            writeln!(file, "{}", article)?;
        }

        Ok(())
    }
}
