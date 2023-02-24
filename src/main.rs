use std::cmp;
use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::env;

fn search(word: &str) -> usize {
    let mut token_file = File::open("token.txt").unwrap();

    let mut token_data = String::new();
    token_file.read_to_string(&mut token_data).unwrap();
    

    todo!()
}

fn lazy_hash(string: &str) -> usize {
    let lowercase = string.to_lowercase();
    let mut chars: Vec<u32> = lowercase.chars().map(|c| c as u32).collect();

    // Convert utf-8 encodings into letter index in the alphabet
    for i in 0..chars.len() {
        if chars[i] < 128 { // a-z
            chars[i] -= 'a' as u32;
        } else if chars[i] == 'å' as u32 {
            chars[i] = 26;
        } else if chars[i] == 'ä' as u32 {
            chars[i] = 27;
        } else if chars[i] == 'ö' as u32 {
            chars[i] = 28;
        }
    }

    // Convert into base 29
    let mut hash = 0;
    for i in 0..cmp::min(string.len(), 3) {
        hash += chars[i] * u32::pow(29, i as u32);
    }
    hash as usize
}

fn gen_index_file() {
    // Open token file
    let mut token_file = File::open("token.txt").unwrap();
    let mut token_data = String::new();
    token_file.read_to_string(&mut token_data).unwrap();

    // Create index file
    let mut index_file = File::create("index.txt").unwrap();

    let mut current_word = "";
    for line in token_data.lines() {
        let mut split_line = line.split(' ');
        let line_word = split_line.next().unwrap();
        let line_byte = split_line.next().unwrap();

        if line_word != current_word {
            current_word = line_word;
            writeln!(index_file).unwrap(); // Newline
            write!(index_file, "{}", current_word).unwrap();
        }
        write!(index_file, " {}", line_byte).unwrap();
    }
}

fn main() {
    if let Some(argument) = env::args().nth(1) {
        if argument == "index" {
            println!("Generating index file...");
            gen_index_file();
        } else if argument == "hash" {

        } else { // Testing
            println!("Hash of 'a': {}", lazy_hash("häsh"));
        }
        println!("Done!");
        return;
    }   

    println!("Welcome to Korpus Inc.!");
    print!("Enter a word to search for: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let word = input.trim().to_ascii_lowercase();

    let matches = search(word.as_str());
    println!("{} is written {} times", word, matches);
}
