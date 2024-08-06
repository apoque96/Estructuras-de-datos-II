#[derive(Debug)]
pub struct Data {
    pub method: String,
    pub data: String,
}

impl Data {
    pub fn new(method: String, data: String) -> Data {
        Data { method, data }
    }
}
