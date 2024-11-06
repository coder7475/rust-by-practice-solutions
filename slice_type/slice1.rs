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
 *          or the length of the string if no space is found
 */
fn first_word(s: &String) -> usize {
    let byte = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn main(){}