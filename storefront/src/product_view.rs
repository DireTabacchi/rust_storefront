pub struct ProductView {}

impl ProductView {
    pub fn new() -> Self {
        Self {}
    }

    // Update the view to show the product.
    pub fn update(
        &self,
        name: String,
        price: f32,
        short_desc: String,
        long_desc: String,
        stock: u32,
    ) {
        // Clear the screen for this view.
        print!("{}[2J{}[H", 27 as char, 27 as char);
        println!("/====================================================================\\");

        // Print the name and price, on separate lines.
        println!("\n{}\n${}", name, price);

        // Check if there is stock.
        if stock < 1 {
            println!("Out of stock");
        } else {
            println!("Stock: {}", stock);
        }

        // Print the short description of the product.
        println!("{}", short_desc);
        println!("\n----------------------------------------------------------------------\n");

        // Print the long description of the product.
        println!("{}", long_desc);

        println!("\n\\====================================================================/\n");

        // Print some instructions for the user to navigate the store.
        println!("To return to the product listings page, type 'back', then hit the\nEnter key.\nTo exit, type 'quit', then hit the Enter key.");
    }

    // Called if the user entered an invalid command.
    pub fn invalid_choice(&self) {
        println!("Please enter a valid choice.");
    }
}
