use std::io;

// ==========================================
// ENGINEERING CONTROL PANEL (Constants)
// This is where a SWE keeps business rules so they are easy to update.
// ==========================================
const PRICE_POUNDO: i32 = 3200;
const PRICE_FRIED_RICE: i32 = 3000;
const PRICE_AMALA: i32 = 2500;
const PRICE_EBA: i32 = 2000;
const PRICE_WHITE_RICE: i32 = 2500;

const DISCOUNT_THRESHOLD: i32 = 10_000;
const DISCOUNT_RATE: f64 = 0.05; // 5% discount

// ANSI Color Codes for terminal styling (Using String literals!)
const COLOR_CYAN: &str = "\x1b[36m";
const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_YELLOW: &str = "\x1b[33m";
const COLOR_RESET: &str = "\x1b[0m";

fn main() {
    // 1. Display the Styled Menu
    println!("{}=========================================================", COLOR_CYAN);
    println!("              🌟 NTHNL'S BISTRO MENU 🌟               ");
    println!("========================================================={}", COLOR_RESET);
    
    // Using format! to neatly align the menu items
    println!("{}[P]{} Poundo Yam / Edinkaiko Soup .... {}N3,200{}", COLOR_YELLOW, COLOR_RESET, COLOR_GREEN, COLOR_RESET);
    println!("{}[F]{} Fried Rice & Chicken ........... {}N3,000{}", COLOR_YELLOW, COLOR_RESET, COLOR_GREEN, COLOR_RESET);
    println!("{}[A]{} Amala & Ewedu Soup ............. {}N2,500{}", COLOR_YELLOW, COLOR_RESET, COLOR_GREEN, COLOR_RESET);
    println!("{}[E]{} Eba & Egusi Soup ............... {}N2,000{}", COLOR_YELLOW, COLOR_RESET, COLOR_GREEN, COLOR_RESET);
    println!("{}[W]{} White Rice & Stew .............. {}N2,500{}", COLOR_YELLOW, COLOR_RESET, COLOR_GREEN, COLOR_RESET);
    println!("{}========================================================={}", COLOR_CYAN, COLOR_RESET);

    // 2. Robust Input Validation: Food Choice
    let price: i32;
    let item_name: &str;
    
    // We loop infinitely until the user gives us a valid input, then we "break" out of the loop.
    loop {
        println!("\nPlease enter the letter for your food choice (P, F, A, E, W):");
        let mut food_choice = String::new();
        io::stdin().read_line(&mut food_choice).expect("Failed to read input");
        
        // Clean the string: remove spaces and make uppercase
        let cleaned_choice = food_choice.trim().to_uppercase(); 

        if cleaned_choice == "P" {
            price = PRICE_POUNDO;
            item_name = "Poundo Yam / Edinkaiko Soup";
            break;
        } else if cleaned_choice == "F" {
            price = PRICE_FRIED_RICE;
            item_name = "Fried Rice & Chicken";
            break;
        } else if cleaned_choice == "A" {
            price = PRICE_AMALA;
            item_name = "Amala & Ewedu Soup";
            break;
        } else if cleaned_choice == "E" {
            price = PRICE_EBA;
            item_name = "Eba & Egusi Soup";
            break;
        } else if cleaned_choice == "W" {
            price = PRICE_WHITE_RICE;
            item_name = "White Rice & Stew";
            break;
        } else {
            // Error handling for bad letters
            println!("{}❌ Invalid choice. Please enter exactly P, F, A, E, or W.{}", COLOR_RED, COLOR_RESET);
        }
    }

    // 3. Robust Input Validation: Quantity
    let quantity: i32;
    loop {
        println!("How many portions of {} would you like?", item_name);
        let mut quantity_input = String::new();
        io::stdin().read_line(&mut quantity_input).expect("Failed to read input");
        
        // match acts like a safer version of .expect(). If it's Ok, we break. If it's Err, we warn and ask again.
        match quantity_input.trim().parse::<i32>() {
            Ok(num) => {
                if num > 0 {
                    quantity = num;
                    break;
                } else {
                    println!("{}❌ Quantity must be at least 1.{}", COLOR_RED, COLOR_RESET);
                }
            }
            Err(_) => {
                println!("{}❌ Please enter a valid number (e.g., 2).{}", COLOR_RED, COLOR_RESET);
            }
        }
    }

    // 4. Mathematics & Logic
    let total_cost = price * quantity;
    let mut discount = 0.0;

    // Apply 5% discount if strictly greater than N10,000
    if total_cost > DISCOUNT_THRESHOLD {
        discount = (total_cost as f64) * DISCOUNT_RATE;
    }
    
    let final_price = (total_cost as f64) - discount;

    // 5. Generate a beautiful ASCII Receipt
    println!("\n{}=========================================================", COLOR_CYAN);
    println!("                     OFFICIAL RECEIPT                    ");
    println!("========================================================={}", COLOR_RESET);
    println!("Item:         {}", item_name);
    println!("Quantity:     {}", quantity);
    println!("Unit Price:   {}N{}{}", COLOR_GREEN, price, COLOR_RESET);
    println!("---------------------------------------------------------");
    println!("Subtotal:     N{}", total_cost);
    
    if discount > 0.0 {
        println!("Discount:    {} -N{:.2}{}", COLOR_GREEN, discount, COLOR_RESET);
        println!("{}*** 5% Over N10k Discount Applied! ***{}", COLOR_YELLOW, COLOR_RESET);
    } else {
        println!("Discount:     N0.00");
    }
    
    println!("---------------------------------------------------------");
    println!("{}FINAL TOTAL:  N{:.2}{}", COLOR_GREEN, final_price, COLOR_RESET);
    println!("{}========================================================={}\n", COLOR_CYAN, COLOR_RESET);
}