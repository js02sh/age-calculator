use std::io;
use chrono::prelude::*;

fn main() {
    println!("Welcome to the Age Checker!");

    // Get today's date
    let today = Local::now().naive_local().date();

    // Get the user's date of birth
    println!("Enter your date of birth (YYYYMMDD): ");
    let mut dob_input = String::new();
    io::stdin()
        .read_line(&mut dob_input)
        .expect("Failed to read line");

    // Parse the date of birth
    let dob = NaiveDate::parse_from_str(&dob_input.trim(), "%Y%m%d")
        .expect("Please enter a valid date");

    // Calculate age
    let age = today.year() as u32 - dob.year() as u32;

    // Check if the birthday has occurred this year
    let has_birthday_occurred = today.month() > dob.month()
        || (today.month() == dob.month() && today.day() >= dob.day());

    // Adjust age based on whether the birthday has occurred this year
    let final_age = if has_birthday_occurred { age } else { age - 1 };

    // Check if the person is 18 or older
    if final_age >= 18 {
        println!("You are {} years old. You are eligible!", final_age);
    } else {
        println!("Sorry, you are not eligible. You must be 18 or older.");
    }
}
