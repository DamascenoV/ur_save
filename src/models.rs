#[derive(Debug)]
pub struct Url {
   pub id: i32,
   pub name: String,
   pub url: String
}

impl Url {
    pub fn new(id: i32, name: String, url: String) -> Url {
        Url { id, name, url }
    }
}
