// The database module is a faux database, for demonstration purposes.
use crate::product;

// The Database holds a single variable, a list of tuples which
// contain the Product, and the quantity of the product.
pub struct Database {
    products: Vec<(product::Product, u32)>,
}

pub fn new_database() -> Database {
    let products: Vec<(product::Product, u32)> = Vec::new();
    Database { products }
}

impl Database {
    pub fn product(&self, index: u32) -> Option<&(product::Product, u32)> {
        match self.products.get(index as usize) {
            None => None,
            Some(p) => Some(p),
        }
    }

    pub fn add_product(
        &mut self,
        name: String,
        price: f32,
        short_desc: String,
        long_desc: String,
        quantity: u32,
    ) {
        self.products.push((
            product::new_product(name, price, short_desc, long_desc),
            quantity,
        ));
    }
}
