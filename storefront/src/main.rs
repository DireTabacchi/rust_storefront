// The database module is a faux database, for demonstration purposes.
mod database {
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

        pub fn product_len(&self) -> u32 {
            self.products.len().try_into().unwrap()
        }

        pub fn add_product(&mut self, product: product::Product, quantity: u32) {
            self.products.push((product, quantity));
        }
    }
}

// The product module contains data about products.
pub mod product {
    #[derive(Debug)]
    pub struct Product {
        name: String,
        price: f32,
        short_desc: String,
        long_desc: String,
    }

    pub fn new_product(name: String, price: f32, short_desc: String, long_desc: String) -> Product {
        Product {
            name,
            price,
            short_desc,
            long_desc,
        }
    }
    impl Product {
        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn price(&self) -> f32 {
            self.price
        }

        pub fn short_desc(&self) -> &str {
            &self.short_desc
        }

        pub fn long_desc(&self) -> &str {
            &self.long_desc
        }
    }
}

fn main() {
    let mut db: database::Database = database::new_database();
    db.add_product(
        product::new_product(
            String::from("Test Product"),
            123.45,
            String::from("This is a Test Product."),
            String::from("This is a Test Product for a test."),
        ),
        42,
    );

    db.add_product(
        product::new_product(
            String::from("Test Product 2.0"),
            123.45,
            String::from("This is the Test Product 2.0!"),
            String::from("This is a Test Product 2.0 for another test."),
        ),
        42,
    );

    match db.product(0) {
        None => println!("That isn't an item!"),
        Some(p) => println!("{:?}", p),
    }

    match db.product(1) {
        None => println!("That isn't an item!"),
        Some(p) => println!("{:?}", p),
    }

    match db.product(2) {
        None => println!("That isn't an item!"),
        Some(p) => println!("{:?}", p),
    }
}
