# Market Simulation Project

This project is a basic market simulation written in Rust. It includes two main structures: `Customer` and `Product`. The project simulates a scenario where customers can purchase products based on their balance and the available stock of products.

## 📁 Project Structure

The project consists of the following main components:

- **Customer**: Represents a customer with a `name`, `surname`, and `balance`. Customers can buy products using the `buy_product` function.
- **Product**: Represents a product with a `name`, `price`, and `stock_quantity`.

## 📊 Functionality

### `Customer` Structure

The `Customer` structure includes the following attributes:
- `name`: The name of the customer.
- `surname`: The surname of the customer.
- `balance`: The balance of the customer, which represents how much money they have.

### `Product` Structure

The `Product` structure includes the following attributes:
- `name`: The name of the product.
- `price`: The price of the product.
- `stock_quantity`: The current stock of the product available in the market.

### 🛒 Market Functions

The main functionality of the market simulation is implemented in the `buy_product` method under the `Customer` structure. This function allows customers to purchase a product based on the following conditions:

- The product must have sufficient stock to fulfill the purchase.
- The customer must have a balance greater than or equal to the total cost of the products.

If these conditions are met, the product's stock is reduced, and the customer's balance is updated accordingly.

## 🛠️ How to Run

1. **Install Rust**: If Rust is not installed, download and install it from [rust-lang.org](https://www.rust-lang.org/).
   
2. **Clone the Repository**:
   ```bash
   git clone <repository-url>
