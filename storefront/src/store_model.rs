use crate::database;
use crate::product;

pub struct StoreModel {
    db: database::Database,
}

pub fn new_store_model() -> StoreModel {
    let db: database::Database = database::new_database();
    StoreModel { db }
}

impl StoreModel {
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

    pub fn product(&self, index: u32) -> Option<&(product::Product, u32)> {
        self.db.product(index)
    }
}
