use std::io;

fn main() {
  println!("Convert words to pig latin");

  loop {
    println!("Please type a word.");

    let mut word = String::new();

    io::stdin().read_line(&mut word)
        .expect("Failed to read line");

    let word = word.trim();

    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    let first_letter = &word[0..1];
    let latter = &word[1..];
    let ay = "ay";
    let hay = "hay";

    let mut starts_with_vowel = false;
    for i in VOWELS.iter() {
      if word.starts_with(|x| &x == i) {
        println!("{}-{}", word, hay);
        starts_with_vowel = true;
        break;
      }
    };
    if !starts_with_vowel {
      println!("{}-{}{}", latter, first_letter, ay);
    }
  }
}
