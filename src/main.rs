// Define the Customer structure
struct Customer {
    name: String,
    surname: String,
    balance: f64,
}

// Define the Product structure
struct Product {
    name: String,
    price: f64,
    stock_quantity: u32,
}

// Implement market functions for the Customer
impl Customer {
    // Function for purchasing a product
    fn buy_product(&mut self, product: &mut Product, quantity: u32) -> bool {
        let total_cost = product.price * quantity as f64;

        // Check if there's enough stock and if the customer can afford the purchase
        if product.stock_quantity >= quantity && self.balance >= total_cost {
            // Reduce product stock and update customer balance
            product.stock_quantity -= quantity;
            self.balance -= total_cost;
            return true; // Purchase successful
        } else {
            return false; // Purchase failed
        }
    }
}

// Main program
fn main() {
    // Instantiate sample customers
    let mut customer1 = Customer {
        name: String::from("Alice"),
        surname: String::from("Doe"),
        balance: 100.0, // $100 balance
    };

    let mut customer2 = Customer {
        name: String::from("Bob"),
        surname: String::from("Smith"),
        balance: 50.0, // $50 balance
    };

    // Instantiate a sample product
    let mut product = Product {
        name: String::from("Laptop"),
        price: 20.0, // $20 per product
        stock_quantity: 10, // 10 items in stock
    };

    // Customers' product purchase operations
    println!("Customer 1 is buying a product...");
    if customer1.buy_product(&mut product, 3) {
        println!("Customer 1 successfully purchased the product.");
    } else {
        println!("Customer 1 couldn't purchase the product.");
    }

    println!("Customer 2 is buying a product...");
    if customer2.buy_product(&mut product, 8) {
        println!("Customer 2 successfully purchased the product.");
    } else {
        println!("Customer 2 couldn't purchase the product.");
    }

    // Display remaining stock and customers' balances
    println!("Remaining stock: {}", product.stock_quantity);
    println!("Customer 1 balance: ${:.2}", customer1.balance);
    println!("Customer 2 balance: ${:.2}", customer2.balance);
}
