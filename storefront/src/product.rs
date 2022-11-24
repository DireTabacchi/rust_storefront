// The product module contains data about products.
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

    // Getter for the product name.
    pub fn name(&self) -> &str {
        &self.name
    }

    // Getter for the product price.
    pub fn price(&self) -> &f32 {
        &self.price
    }

    // Getter for the product short description.
    pub fn short_desc(&self) -> &str {
        &self.short_desc
    }

    // Getter for the product long description.
    pub fn long_desc(&self) -> &str {
        &self.long_desc
    }
}
