use crate::product_listing::ProductListing;

pub struct ListingView {}

impl ListingView {
    pub fn new() -> Self {
        Self {}
    }
    pub fn print_listings(&self, listings: Vec<ProductListing>) {
        for listing in listings {
            println!("{}\t{}", listing.title(), listing.price());
            println!("{}", listing.short_desc());

            if listing.quantity() < &1 {
                println!("Out of stock!");
            } else {
                println!("Stock: {}", listing.quantity());
            }

            println!("======================================================================");
        }
    }
}
