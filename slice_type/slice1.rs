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
 
fn first_word(s: &String) -> usize { // Function declaration that takes string reference and returns usize
    let bytes = s.as_bytes(); // Convert string to byte array for easier character processing

    for (i, &item) in bytes.iter().enumerate() { // Iterate through bytes with index
        if item == b' ' { // Check if current byte is a space character
            return i;  // Return index if space is found
        }
    }
    s.len() // Return full string length if no space found
}

fn main(){ 
    let mut s = String::from("Hello World!"); // Create mutable string with test content
    let hello = &s[0..5]; // Take slice of first word
    let world = &s[6..11]; // Take slice of second word
    let s1 = &s[..5]; 
    let s2 = &s[6..];
    
    let word = first_word(&s); // Get index of first word
    println!("s: {}, slice 1: {}, slice 2: {}", s, s1, s2); // Print original string and slices
    println!("{}", word); // Print index where first word ends
    s.clear(); // Clear the string content
    println!("s is empty: {}", s); // Print empty string
}