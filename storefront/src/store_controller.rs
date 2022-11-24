use crate::listing_view::ListingView;
use crate::product_listing::ProductListing;
use crate::product_view::ProductView;
use crate::store_model::StoreModel;
use std::io;
use std::io::Write;

// Enum to determine what viewtype is being used.
pub enum ViewType {
    ListingView,
    ProductView,
}

// The struct for the store controller.
pub struct StoreController {
    store_model: StoreModel,
    listing_view: ListingView,
    product_view: ProductView,
}

impl StoreController {
    pub fn new(
        store_model: StoreModel,
        listing_view: ListingView,
        product_view: ProductView,
    ) -> Self {
        Self {
            store_model,
            listing_view,
            product_view,
        }
    }

    // Add a product to the database. Currently only used in demonstration.
    pub fn add_product(
        &mut self,
        name: String,
        price: f32,
        short_desc: String,
        long_desc: String,
        stock: u32,
    ) {
        self.store_model
            .add_product(name, price, short_desc, long_desc, stock);
    }

    // Method to request the listing view to update. Note that the product
    // listings much be created to pass to the listing view.
    pub fn update_listing_view(&self) {
        let mut listings: Vec<ProductListing> = Vec::new();
        let products = self.store_model.products();

        for product in products {
            listings.push(ProductListing::new(
                String::from(product.0.name()),
                *product.0.price(),
                String::from(product.0.short_desc()),
                product.1,
            ));
            println!("{}", products.get(0).unwrap().0.name());
        }

        self.listing_view.update(listings);
    }

    pub fn update_product_view(&self, user_choice: u32) -> (bool, ViewType) {
        let product = self.store_model.product(user_choice - 1);
        match product {
            // If there was a product, request the product
            // view to update with this product.
            Some(p) => {
                self.product_view.update(
                    String::from(p.0.name()),
                    *p.0.price(),
                    String::from(p.0.long_desc()),
                    String::from(p.0.short_desc()),
                    p.1,
                );
                (true, ViewType::ProductView)
            }
            // If there was no product, or the user entered
            // an invalid command, inform the user.
            None => {
                self.product_view.invalid_choice();
                (true, ViewType::ListingView)
            }
        }
    }

    // Handle input from the user.
    pub fn input(&self) -> String {
        // Print a prompt.
        print!("\n--,^^,*> ");
        io::stdout().flush().unwrap();

        // Get the user's choice.
        let mut choice = String::new();
        match io::stdin().read_line(&mut choice) {
            Err(..) => String::from("0"),
            Ok(..) => choice,
        }
    }

    // Returns false when the user wants to quit.
    // Returns True and the view being navigated to if the user entered
    // a valid command.
    pub fn user_choice(&self, view_type: &ViewType) -> (bool, ViewType) {
        let choice: String;

        match view_type {
            // If the current view is the listing view
            ViewType::ListingView => {
                choice = self.input();
                let choice = String::from(choice.trim().to_lowercase());

                // User wants to quit
                if choice.eq(&String::from("quit")) {
                    (false, ViewType::ListingView)
                // User wants to view a product
                } else {
                    let choice: u32 = match choice.parse::<u32>() {
                        Ok(x) => x,
                        Err(..) => 0,
                    };

                    // User entered an invalid command.
                    if choice == 0 {
                        self.listing_view.invalid_choice();
                        return (true, ViewType::ListingView);
                    }

                    // Get the product from the store model.
                    self.update_product_view(choice)
                }
            }
            // If the current view is the product view
            ViewType::ProductView => {
                choice = self.input();
                let choice = String::from(choice.trim().to_lowercase());

                // Product view has two options: go back to the listings,
                // or quit.

                // User wants to go back to the listings.
                if choice.eq(&String::from("back")) {
                    self.update_listing_view();
                    (true, ViewType::ListingView)
                // User wants to quit.
                } else if choice.eq(&String::from("quit")) {
                    (false, ViewType::ProductView)
                } else {
                    self.product_view.invalid_choice();
                    (true, ViewType::ProductView)
                }
            }
        }
    }
}
