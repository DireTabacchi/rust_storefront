use crate::database::Database;
use crate::product::Product;

// The store model interacts with the database.
pub struct StoreModel {
    db: Database,
}

impl StoreModel {
    pub fn new() -> Self {
        Self {
            db: Database::new(),
        }
    }

    // Add a product to the database.
    pub fn add_product(
        &mut self,
        name: String,
        price: f32,
        short_desc: String,
        long_desc: String,
        quantity: u32,
    ) {
        self.db
            .add_product(name, price, short_desc, long_desc, quantity);
    }

    // Gets a product from the database. Return an Option, as there may
    // not be anything at the given index.
    pub fn product(&self, index: u32) -> Option<&(Product, u32)> {
        self.db.product(index)
    }

    // Gets the vector of products from the database. Returns a reference
    // to the list of products.
    pub fn products(&self) -> &Vec<(Product, u32)> {
        &self.db.products()
    }
}
