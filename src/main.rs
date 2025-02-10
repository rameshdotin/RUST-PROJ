fn main() {
    // Print a greeting message to the console
    println!("Hello, world!");

    // Variable declarations with different data types
    let x: i8 = 8; // Signed 8-bit integer
    let y: u32 = 1000; // Unsigned 32-bit integer
    let z: f32 = 1000.215; // 32-bit floating point number

    // Print the values of x, y, and z
    print!("x: {}, y: {}, z: {}", x, y, z);

    // Boolean variables
    let is_male = true;
    let is_above_18 = true;

    // Conditional check for gender
    if is_male {
        println!("You are a male.!");
    } else {
        println!("You are not a male.!");
    }

    // Checking both conditions: if the person is male and above 18
    if is_male && is_above_18 {
        print!("You are a legal male.")
    }

    // Immutable string slice (cannot be modified)
    let greet = "Hello Greet";

    // Commented out: Trying to modify a string slice, which is not allowed in Rust
    // for i in 1..10 {
    //     greet = greet + "Hii"; // This will cause a compilation error
    // }

    // Fixing ownership issue in `gretting`
    // `String::from` creates a heap-allocated string, allowing modification
    let gretting = String::from("Hello World!");

    // Commented out: This would work if `gretting` was mutable
    // for _ in 1..10 {
    //     gretting.push_str(" Hii"); // Appending string
    // }

    // Print the `greet` and `gretting` values
    println!("{} {}", greet, gretting);

    // Extracting the character at index 10 from the `gretting` string
    let char1 = gretting.chars().nth(10);

    // Commented out: Directly unwrapping without handling None case could cause a panic
    // println!("{}", char1.unwrap())

    // Using `match` to handle the case where the character might not exist
    match char1 {
        Some(c) => println!("{}", c), // If the character exists, print it
        None => println!("No character at index 10"), // If index 10 is out of bounds
    }

    // Create a new string
    let sentance = String::from("My name is ramesh");

    // Print the first word of the sentence using a helper function
    println!("First word is: {}", get_first_word(sentance));

    // Creating a mutable string
    let mut s = String::from("initial string");

    // Print the initial state of the string
    println!("Before update: {}", s);

    // Print string details: capacity, length, and memory pointer
    println!(
        "Capacity: {}, Length: {}, Pointer: {:p}",
        s.capacity(),
        s.len(),
        s.as_ptr()
    );

    // Append text to the string multiple times and observe changes in capacity
    for _ in 0..100 {
        s.push_str(" and some additional text"); // Appending more text
        println!(
            "Capacity: {}, Length: {}, Pointer: {:p}",
            s.capacity(),
            s.len(),
            s.as_ptr()
        );
    }
}

// Function to extract the first word from a given sentence
fn get_first_word(sentance: String) -> String {
    let mut ans = String::new(); // Initialize an empty string to store the first word

    // Iterate over each character in the sentence
    for char in sentance.chars() {
        ans.push_str(char.to_string().as_str()); // Append the character to the result string
        if char == ' ' {
            // Stop when the first space is encountered
            break;
        }
    }
    return ans; // Return the first word
}
