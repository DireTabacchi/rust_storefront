use storefront::listing_view::ListingView;
use storefront::product_view::ProductView;
use storefront::store_controller::{StoreController, ViewType};
use storefront::store_model::StoreModel;

fn main() {
    let store_model: StoreModel = StoreModel::new();
    let listing_view: ListingView = ListingView::new();
    let product_view: ProductView = ProductView::new();
    let mut store_controller: StoreController =
        StoreController::new(store_model, listing_view, product_view);
    let mut no_exit: bool = true;

    // Add some products to the database to have something to show in the views.
    store_controller.add_product(
        String::from("Diga ZamPom"),
        400.95,
        String::from("The Diga Robot you can build and Configure!"),
        String::from("Draw a picture! Protect your things! Turn him into a car! Make him do\nyour homework! Create a device to solve a Rubank's Cuboid!\nThe Diga ZamPom can do all this and more! With the parts and pieces\ncontained in this set, you can let your imagination run wild!"),
        42,
    );

    store_controller.add_product(
        String::from("SuperWatch DIY Clock"),
        249.42,
        String::from("Build a clock, and watch the time tick by!"),
        String::from("Have you ever wondered how a clock works? Well, wonder no more! With\nthis new set from SuperWatch, you'll be able to learn just that.\nYou'll end up with a slick looking clock, too. Let those cogs in your\nhead turn, with Superwatch's new DIY Clock set!\n\nThis set includes all of the necessary parts to build an analog,\nnon-batter-powered clock.\n\nSome assembly required."),
        77,
    );

    store_controller.add_product(
        String::from("The Rust Programming Langauge"),
        39.99,
        String::from("Learn to program in Rust, and become a Rustacean!"),
        String::from("Have you heard of this new-fangled Rust programming language and want\nto learn more about it, and possibly learn to write programs in it?\nThis book, written by Steve Klabnik and Carol Nichols, can help you\nwith that! In this book, you'll learn about the basics of programming\nin Rust. You'll also learn about Rust's solution to memory safety,\nownership and that concept your mom taught you about, borrowing!\n\nThe book is also available for free at https://doc.rust-lang.org/book/."),
        7,
    );
    store_controller.add_product(
        String::from("Andruoni Spaceship Control-Panel Kit"),
        75.87,
        String::from("Start building your own spaceship!"),
        String::from("Have you wanted to build your own spaceship? We have the blueprints,\nyou just have to build it! With this Control Panel, you'll be able to\nread your fuel level, relative velocity, and your RSD (Reality Shift\nDrive) status for super-luminal (faster than light) speed!\nThis kit is a perfect introduction to circuit building and programming\nfor children. It includes:\n- 1 Andruoni\n- 1 breadboard\n- 100 jumper wires (5 lengths)\n- 40 red LEDs\n- 20 blue LEDs\n- 20 yellow LEDs\n- 25 green LEDs\n- 4 SPST switches\n- 4 push-buttons\n- 4 rheostats\n- Instruction book\nTo write the software, visit the downloads page and get the AtMore\nStudio! Instructions and explanations for the program are in the\ninstruction book contained in this kit.\n\nAdult supervision recommended.\nChoking Hazard: Contains small components. Not for ages 6 and under."),
        0,
    );

    // Show the Listings view on startup
    let mut current_view = ViewType::ListingView;
    store_controller.update_listing_view();
    // loop on store_controller getting input from the user
    while no_exit {
        (no_exit, current_view) = store_controller.user_choice(&current_view);
    }
}
