use storefront::store_controller::StoreController;
use storefront::store_model::StoreModel;

fn main() {
    let mut store_model: StoreModel = StoreModel::new();
    let store_controller: StoreController = StoreController::new();

    store_model.add_product(
        String::from("Test Product"),
        123.45,
        String::from("This is a Test Product."),
        String::from("This is a Test Product for a test."),
        42,
    );

    store_model.add_product(
        String::from("Wonderfon"),
        77.34,
        String::from("This is the 1998 Wonderfon, brought back in 2022!"),
        String::from("Call like it's the 1990's! The Wonderfon is back in all its glory!"),
        77,
    );

    store_model.add_product(
        String::from("Test Product 2.0"),
        123.45,
        String::from("This is the Test Product 2.0!"),
        String::from("This is a Test Product 2.0 for another test."),
        42,
    );

    match store_model.product(0) {
        None => println!("That isn't an item!"),
        Some(p) => {
            let (product, quantity) = p;
            println!("{}: {}", product.name(), quantity);
        }
    }

    match store_model.product(1) {
        None => println!("That isn't an item!"),
        Some(p) => println!("{:?}", p),
    }

    match store_model.product(2) {
        None => println!("That isn't an item!"),
        Some(p) => println!("{:?}", p),
    }

    match store_model.product(3) {
        None => println!("That isn't an item!"),
        Some(p) => println!("{:?}", p),
    }
}
