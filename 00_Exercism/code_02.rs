/*
* Initially, reversed_chars is an empty vector.

For each character ch in the original string, it is inserted at the 0th index of reversed_chars.

    For the first character, it's inserted at index 0 (the beginning).
    For the second character, it's inserted at index 0, pushing the first character to index 1.
    For the third character, it's inserted at index 0, pushing both previous characters to higher indices.
    This process continues for all characters.

As a result, the characters are effectively inserted in reverse order at the beginning of the vector.


* reversed_chars.into_iter().collect(); in detail:

    reversed_chars.into_iter():
        reversed_chars is a Vec<char> vector that contains the characters of the reversed string.
        into_iter() is called on reversed_chars, which converts the vector into an iterator.

    .collect():
        The collect() method is used to consume the iterator and collect its elements into a new collection.
        In this case, it's collecting the characters from the iterator into a new String.

    let reversed_string: String = ...:
        The result of collect() is assigned to a new variable reversed_string.
        The type annotation : String indicates that reversed_string is expected to be of type String.

        
*/

pub fn reverse(data: &str) -> String {

    let mut rev_chars = Vec::new(); // Creating mutable vector to
    for i in data.chars() {
        rev_chars.insert(0, i); // this is inserting each character at 0th index, which will make our first character pushes to last.
    }

    let rev_string = rev_chars.into_iter().collect();   // This line converts the reversed_chars vector into an iterator, and then collects the characters into a new String. The result is the reversed string.
    rev_string
}

fn main() {
    let data: &str = "sahilwep";
    let rev_string = reverse(data);

    println!("original string: {}", data);
    println!("reversed string: {}", rev_string);
}