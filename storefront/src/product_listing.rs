pub struct ProductListing {
    title: String,
    price: f32,
    short_desc: String,
    quantity: u32,
}

impl ProductListing {
    pub fn new(title: String, price: f32, short_desc: String, quantity: u32) -> Self {
        Self {
            title,
            price,
            short_desc,
            quantity,
        }
    }
    pub fn title(&self) -> &String {
        &self.title
    }
    pub fn price(&self) -> &f32 {
        &self.price
    }
    pub fn short_desc(&self) -> &String {
        &self.short_desc
    }
    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }
}
