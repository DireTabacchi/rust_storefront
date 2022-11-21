// The product module contains data about products.
#[derive(Debug)]
pub struct Product {
    name: String,
    price: f32,
    short_desc: String,
    long_desc: String,
}

impl Product {
    pub fn new(name: String, price: f32, short_desc: String, long_desc: String) -> Self {
        Self {
            name,
            price,
            short_desc,
            long_desc,
        }
    }
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn price(&self) -> &f32 {
        &self.price
    }

    pub fn short_desc(&self) -> &str {
        &self.short_desc
    }

    pub fn long_desc(&self) -> &str {
        &self.long_desc
    }
}
