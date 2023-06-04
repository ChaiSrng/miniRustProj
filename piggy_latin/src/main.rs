///Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added,
///  so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead
///  (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
use std::io;
fn main() {
    println!("Enter the word to be converted to pig latin ;)");
    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failed to readline");

    let mut chars = word.trim().chars();
    let letter = chars.next();
    if letter == Some('a')
        || letter == Some('e')
        || letter == Some('i')
        || letter == Some('o')
        || letter == Some('u')
        || letter == Some('A')
        || letter == Some('E')
        || letter == Some('I')
        || letter == Some('O')
        || letter == Some('U')
    {
        println!("Your pig-latin word is :");
        println!("{}-hay",word.trim());
    } else {
        let result = format!("{}-fay", chars.as_str());
        println!("Your pig-latin word is :");
        println!("{:?}", result);
    }
}
