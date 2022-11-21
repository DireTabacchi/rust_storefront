use crate::database::Database;
use crate::product::Product;

pub struct StoreModel {
    db: Database,
}

impl StoreModel {
    pub fn new() -> Self {
        Self {
            db: Database::new(),
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
        self.db
            .add_product(name, price, short_desc, long_desc, quantity);
    }

    pub fn product(&self, index: u32) -> Option<&(Product, u32)> {
        self.db.product(index)
    }

    pub fn products(&self) -> &Vec<(Product, u32)> {
        &self.db.products()
    }
}
