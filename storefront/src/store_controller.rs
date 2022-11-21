use crate::listing_view::ListingView;
use crate::product_listing::ProductListing;
use crate::product_view::ProductView;
use crate::store_model::StoreModel;

pub struct StoreController {
    store_model: StoreModel,
    listing_view: ListingView,
    product_view: ProductView,
}

impl StoreController {
    pub fn new() -> Self {
        Self {
            store_model: StoreModel::new(),
            listing_view: ListingView::new(),
            product_view: ProductView::new(),
        }
    }
    pub fn update_listing_view(&self) {
        let mut listings: Vec<ProductListing> = Vec::new();
        let products = self.store_model.products();

        for product in products {
            listings.push(ProductListing::new(
                String::from(product.0.name()),
                *product.0.price(),
                String::from(product.0.short_desc()),
                product.1,
            ))
        }

        self.listing_view.print_listings(listings);
    }
}
