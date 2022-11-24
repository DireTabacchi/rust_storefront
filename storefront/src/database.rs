// The database module is a faux database, for demonstration purposes.
use crate::product::Product;

// The Database holds a single variable, a list of tuples which
// contain the Product, and the stock of the product.
pub struct Database {
    products: Vec<(Product, u32)>,
}

impl Database {
    pub fn new() -> Self {
        let products: Vec<(Product, u32)> = Vec::new();
        Self { products }
    }

    // Return an Option enum. If there is a product at the index,
    // then the Option will contain the database entry. Otherwise,
    // return the None variant, to signal that the index chosen is
    // invalid.
    pub fn product(&self, index: u32) -> Option<&(Product, u32)> {
        match self.products.get(index as usize) {
            None => None,
            Some(p) => Some(p),
        }
    }

    // Return all the products in the database as a Vector of product
    // entries.
    pub fn products(&self) -> &Vec<(Product, u32)> {
        &self.products
    }

    // Add a product to the database.
    pub fn add_product(
        &mut self,
        name: String,
        price: f32,
        short_desc: String,
        long_desc: String,
        stock: u32,
    ) {
        self.products
            .push((Product::new(name, price, short_desc, long_desc), stock));
    }
}
