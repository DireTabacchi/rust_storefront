// A struct to easily pass relavent info to the listing_view.
pub struct ProductListing {
    title: String,
    price: f32,
    short_desc: String,
    stock: u32,
}

impl ProductListing {
    pub fn new(title: String, price: f32, short_desc: String, stock: u32) -> Self {
        Self {
            title,
            price,
            short_desc,
            stock,
        }
    }

    // Getter for the listing title.
    pub fn title(&self) -> &String {
        &self.title
    }

    // Getter for the listing price.
    pub fn price(&self) -> &f32 {
        &self.price
    }

    // Getter for the listing short description.
    pub fn short_desc(&self) -> &String {
        &self.short_desc
    }

    // Getter for the listing stock.
    pub fn stock(&self) -> &u32 {
        &self.stock
    }
}
