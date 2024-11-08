/* 
 Write a function that takes a string of words separated by spaces and 
 returns the first word it finds in that string. If the function doesn't 
 find a space in the string, the whole string must be one word, so the 
 entire string should be returned.
*/ 

/**
 * ? Finds and returns the first word in a string
 * * @param s - The input string to search through
 * * @returns The index position where the first word ends (at the first space),
 *  
 */ 
 
fn first_word(s: &String) -> &str { // Function that takes a string reference and returns a string slice
    let bytes = s.as_bytes(); // Convert string to bytes for character-by-character processing

    for (i, &item) in bytes.iter().enumerate() { // Iterate through bytes with their indices
        if item == b' ' { // If we find a space character
            return &s[..i];  // Return slice from start up to space
        }
    }
    // If no space found, return slice of entire string
    &s[..]
}

fn main(){ 
    let mut s = String::from("Hello World!"); // Create mutable string to demonstrate slicing
    let _hello = &s[0..5]; // Slice containing "Hello"
    let _world = &s[6..11]; // Slice containing "World"
    let s1 = &s[..5]; // Another way to slice "Hello" (from start to position 5)
    let s2 = &s[6..]; // Slice from position 6 to end
    
    println!("s: {}, slice 1: {}, slice 2: {}", s, s1, s2); // Print full string and two slices
    
    let word = first_word(&s); // Get first word as a slice

    println!("{}", word); // Print the first word

    s.clear(); // Empty the string
    println!("s is empty: {}", s); // Show that string is now empty
}