fn main() {
    // Generate a random number for the bill
    let bill_amount = rand::random::<f32>() * 100.0;
    println!("Total amount: ${:.2}", bill_amount);
    
    // Create a vector of silly items
    let items = vec!["Invisible ink", "Air guitar", "Silence", "Digital high five"];
    
    println!("\nItemized list:");
    for (i, item) in items.iter().enumerate() {
        let price = rand::random::<f32>() * 25.0;
        println!("  {}. {} - ${:.2}", i+1, item, price);
    }
    
    println!("\nThank you for shopping with us!");
    println!("Please come back never!");
}
