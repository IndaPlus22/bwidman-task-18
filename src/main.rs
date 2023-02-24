use std::cmp;
use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::env;

fn hash_file_search(line_index: usize, word: &str) -> u64 {
    // Open hash file
    let mut hash_file = File::open("hash.txt").unwrap();
    let mut hash_data = String::new();
    hash_file.read_to_string(&mut hash_data).unwrap();

    let line = hash_data.lines().nth(line_index).unwrap();
    let mut split_line = line.split(' ');
    
    split_line.find(|&s| s == word).unwrap(); // Skip to word

    return split_line.next().unwrap().parse().unwrap();
}

fn search(word: &str) -> usize {
    let hash_line_index = lazy_hash(word);

    let index_file_offset = hash_file_search(hash_line_index, word);

    // Open index file
    let mut index_file = File::open("index.txt").unwrap();

    index_file.seek(io::SeekFrom::Start(index_file_offset)).unwrap();

    let mut index_data = String::new();
    index_file.read_to_string(&mut index_data).unwrap();
    
    // Split newline two times to account for newline at 
    let mut word_line = index_data.splitn(2, '\n').collect::<Vec<&str>>();
    word_line.pop();
    
    return word_line[0].split(' ').count() - 1 // Amount of matches
}

fn lazy_hash(string: &str) -> usize {
    let lowercase = string.to_lowercase();
    let chars: Vec<u32> = lowercase.chars().map(|c| c as u32).collect();
    
    let mut hash = 0;
    for i in 0..cmp::min(chars.len(), 3) {
        // Convert utf-8 encoding into letter index in the alphabet
        let mut alpabet_index = 0;
        if chars[i] < 128 { // a-z
            alpabet_index = chars[i] - 'a' as u32;
        } else if chars[i] == 'å' as u32 {
            alpabet_index = 26;
        } else if chars[i] == 'ä' as u32 {
            alpabet_index = 27;
        } else if chars[i] == 'ö' as u32 {
            alpabet_index = 28;
        }
        
        // Convert letter into base 29
        hash += alpabet_index * u32::pow(29, i as u32);
    }
    hash as usize
}

fn make_index_file() {
    // Open token file
    let mut token_file = File::open("token.txt").unwrap();
    let mut token_data = String::new();
    token_file.read_to_string(&mut token_data).unwrap();

    // Create index file
    let mut index_file = File::create("index.txt").unwrap();

    let mut previous_word = "";
    for line in token_data.lines() {
        let mut split_line = line.split(' ');
        let line_word = split_line.next().unwrap();
        let line_byte = split_line.next().unwrap();

        if line_word != previous_word && previous_word != "" {
            writeln!(index_file).unwrap(); // Newline
            write!(index_file, "{}", line_word).unwrap();
        }
        previous_word = line_word;
        write!(index_file, " {}", line_byte).unwrap();
    }
}

fn make_hash_file() {
    // Open index file
    let mut index_file = File::open("index.txt").unwrap();
    let mut index_data = String::new();
    index_file.read_to_string(&mut index_data).unwrap();
    
    let mut hash_file_lines: Vec<String> = vec!["".to_string(); 29*29*29]; // Start with 29^3 rows
    
    let mut current_offset = 0;
    
    for line in index_data.lines() {
        let line_word = line.split(' ').next().unwrap();
        let word_hash = lazy_hash(line_word);
        
        hash_file_lines[word_hash].push_str(format!("{} {} ", line_word, current_offset).as_str());
        
        current_offset += line.len() + 1;
    }

    // Create hash file
    let mut hash_file = File::create("hash.txt").unwrap();
    
    write!(hash_file, "{}", hash_file_lines.join("\n")).unwrap();
}

fn main() {
    if let Some(argument) = env::args().nth(1) {
        if argument == "index" {
            println!("Generating index file...");
            make_index_file();
        } else if argument == "hash" {
            println!("Generating hash file...");
            make_hash_file();
        } else { // Testing
            println!("Test hash: {}", lazy_hash("ä"));
        }
        println!("Done!");
        return;
    }   

    println!("Welcome to Korpus!");
    print!("Enter a word to search for: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let word = input.trim().to_ascii_lowercase();

    let matches = search(word.as_str());
    println!("{} is written {} times", word, matches);
}
