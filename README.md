# Bank Project in Rust

## Overview

This is a simple banking application implemented in Rust. The application allows you to create bank accounts, deposit and withdraw funds, and view account summaries and total balances.

## Features

- Create new bank accounts
- Deposit and withdraw funds
- View account summaries
- Calculate total balance across all accounts

## Getting Started

### Prerequisites

- Rust installed on your machine. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/tools/install).

### Installation

1. Clone the repository:

   ```bash
   git clone <repository-url>
   cd <repository-directory>
   ```
2. Build the project:

   ```bash
   cargo build
   ```
3. Run the project:

   ```bash
   cargo run
   ```

## Usage

The main function demonstrates how to create accounts, perform transactions, and display account information. You can modify the `main` function to test different scenarios.

### Example

fn main() {

let mut bank = Bank::new();

let mut account = Account::new(1, "John".to_string());

account.deposit(500);

account.withdraw(200);

bank.add_account(account);

println!("Total balance: {}", bank.total_balance());

println!("Total summary: {:?}", bank.total_summary());

}
