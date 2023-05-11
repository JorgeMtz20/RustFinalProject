// Import necessary libraries
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;

// Set the number of threads to use for processing the files
const NUM_THREADS: usize = 3;

// Function to read text files and count occurrences of the word "the"
fn count_the_occurrences(file_path: &str) -> (String, HashMap<String, usize>) {
    // Open file and create buffer reader
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    // Create a hash map to store word counts
    let mut word_counts: HashMap<String, usize> = HashMap::new();

    // Loop over each line in the file
    for line in reader.lines() {
        // Unwrap the line
        let line = line.unwrap();

        // Split the line into words
        let words = line.split_whitespace();

        // Loop over each word in the line
        for word in words {
            // Convert the word to lowercase and remove any non-alphabetic characters
            let cleaned_word = word.to_lowercase().chars().filter(|c| c.is_alphabetic()).collect::<String>();

            // Check if the word is "the"
            if cleaned_word == "the" {
                // If the word is "the", increment the count in the hash map
                let count = word_counts.entry(cleaned_word).or_insert(0);
                *count += 1;
            }
        }
    }

    // Return the file name and the hash map of word counts
    (file_path.to_string(), word_counts)
}

// Function to count the occurrences of "the" in multiple files sequentially
fn sequential_processing(file_paths: &[String]) -> HashMap<String, usize> {
    // Create a hash map to store word counts
    let mut word_counts: HashMap<String, usize> = HashMap::new();

    // Loop over each file path
    for file_path in file_paths {
        // Count the occurrences of "the" in the file
        let (_, file_word_counts) = count_the_occurrences(file_path);

        // Add the file word counts to the overall word counts
        for (word, count) in file_word_counts.iter() {
            let overall_count = word_counts.entry(word.clone()).or_insert(0);
            *overall_count += count;
        }
    }

    // Return the hash map of word counts
    word_counts
}

// Function to count the occurrences of "the" in multiple files using task parallelism
fn task_parallelism(file_paths: &[String]) -> Vec<(String, HashMap<String, usize>)> {
    // Create a vector to store the handles to the threads
    let mut handles = vec![];

    // Create a channel for communicating the word counts between threads
    let (tx, rx) = mpsc::channel();

    // Loop over each file path
    for file_path in file_paths {
        // Clone the transmitter for the channel
        let tx_clone = tx.clone();

        // Spawn a new thread to count the occurrences of "the" in the file
        let file_path_clone = file_path.clone();
        let handle = thread::spawn(move || {
            let (file_name, word_counts) = count_the_occurrences(&file_path_clone);
            tx_clone.send((file_name, word_counts)).unwrap();
        });

        // Store the handle to the thread in the vector
        handles.push(handle);
    }

    // Create a vector to store the file name and word counts tuples
    let mut file_word_counts = vec![];

    // Collect the word counts from the channel
    for _ in 0..handles.len() {
        let (file_name, word_counts) = rx.recv().unwrap();
        file_word_counts.push((file_name.clone(), word_counts));
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Return the vector of file name and word counts tuples
    file_word_counts
}



fn print_word_counts(file_word_counts: &Vec<HashMap<String, usize>>) {
    // Loop over each file word counts hash map
    for (i, word_counts) in file_word_counts.iter().enumerate() {
        println!("File {}: ", i + 1);

        // Loop over each word and count in the word counts hash map
        for (word, count) in word_counts {
            println!("{}: {}", word, count);
        }

        println!(); // add empty line between files
    }
}



fn main() {
    // Define the file paths to read
    let file_paths = vec![
        String::from("C:\\Users\\swegs\\Desktop\\Rust Final Project\\github.txt"),
        String::from("C:\\Users\\swegs\\Desktop\\Rust Final Project\\js.txt"),
        String::from("C:\\Users\\swegs\\Desktop\\Rust Final Project\\vscode.txt"),
    ];

    // Count the occurrences of "the" in the files using task parallelism
    let file_word_counts = task_parallelism(&file_paths);

    // Print the word counts to the console
    print_word_counts(&file_word_counts.iter().map(|(_, word_counts)| word_counts.clone()).collect());
}


//Jorge Martinez - ID#20508988 - CSCI - 3334