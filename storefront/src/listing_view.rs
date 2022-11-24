use crate::product_listing::ProductListing;

pub struct ListingView {}

impl ListingView {
    pub fn new() -> Self {
        Self {}
    }

    // Update the view to show the listings.
    pub fn update_listing_view(&self, listings: Vec<ProductListing>) {
        // Clear the screen for this view.
        print!("{}[2J{}[H", 27 as char, 27 as char);

        // Counter to show the listing numbers for users to navigate.
        let mut listing_id = 1;

        // Loop through the listings, and print each one.
        for listing in listings {
            println!("{})", listing_id);
            println!("{}    |    ${}", listing.title(), listing.price());
            println!("{}", listing.short_desc());

            if listing.stock() < &1 {
                println!("Out of stock!");
            } else {
                println!("Stock: {}", listing.stock());
            }

            // Visual separator between listings.
            println!("======================================================================");

            listing_id += 1;
        }

        // Print some instructions for the user to navigate the store.
        println!("\nTo view a product's page, type the number of the product, then hit the\nEnter key.\nTo exit, type 'quit', then hit the Enter key.");
    }

    // Called if the user entered an invalid command.
    pub fn invalid_choice(&self) {
        println!("Please enter a valid choice.");
    }
}
