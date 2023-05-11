// Import necessary libraries
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

// Set the number of threads to use for processing the files
const NUM_THREADS: usize = 3;

// Function to read text files and count occurrences of the word "the"
fn count_the_occurrences(file_path: &str) -> HashMap<String, usize> {
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

    // Return the hash map of word counts
    word_counts
}

// Function to count the occurrences of "the" in multiple files sequentially
fn sequential_processing(file_paths: &[String]) -> HashMap<String, usize> {
    // Create a hash map to store word counts
    let mut word_counts: HashMap<String, usize> = HashMap::new();

    // Loop over each file path
    for file_path in file_paths {
        // Count the occurrences of "the" in the file
        let file_word_counts = count_the_occurrences(file_path);

        // Add the file word counts to the overall word counts
        for (word, count) in file_word_counts {
            let overall_count = word_counts.entry(word).or_insert(0);
            *overall_count += count;
        }
    }

    // Return the hash map of word counts
    word_counts
}

// Function to count the occurrences of "the" in multiple files using task parallelism
fn task_parallelism(file_paths: &[String]) -> HashMap<String, usize> {
    // Create a hash map to store word counts
    let word_counts = Arc::new(Mutex::new(HashMap::new()));

    // Create an Arc of a Mutex containing the vector of file paths
    let file_paths = Arc::new(Mutex::new(file_paths.to_vec()));

    // Create a vector to store the handles to the threads
    let mut handles = vec![];

    // Loop over each file path
    for _ in 0..NUM_THREADS {
        // Clone the arc of the word counts hash map
        let word_counts_clone = Arc::clone(&word_counts);

        // Clone the arc of the vector of file paths
        let file_paths_clone = Arc::clone(&file_paths);

        // Spawn a new thread to count the occurrences of "the" in the files
        let handle = thread::spawn(move || {
            loop {
                // Lock the mutex and remove a file path from the vector
                let mut file_paths = file_paths_clone.lock().unwrap();
                let file_path = match file_paths.pop() {
                    Some(file_path) => file_path,
                    None => break,  // No more file paths left to process
                };

                // Count the occurrences of "the" in the file
                let file_word_counts = count_the_occurrences(&file_path);

                // Lock the mutex and update the overall word counts
                let mut word_counts = word_counts_clone.lock().unwrap();
                for (word, count) in file_word_counts {
                    let overall_count = word_counts.entry(word).or_insert(0);
                    *overall_count += count;
                }
            }
        });

        // Store the handle to the thread in the vector
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Extract the word counts from the hash map and return them
    let word_counts_mutex = Arc::try_unwrap(word_counts).unwrap();
    let word_counts = word_counts_mutex.into_inner().unwrap();
    word_counts
}


fn main() {
    // Construct the file paths
    let file_paths = vec![
        String::from("C:\\Users\\swegs\\Desktop\\Rust Final Project\\github.txt"),
        String::from("C:\\Users\\swegs\\Desktop\\Rust Final Project\\js.txt"),
        String::from("C:\\Users\\swegs\\Desktop\\Rust Final Project\\vscode.txt"),
    ];


    // Count the occurrences of "the" in the files using task parallelism
    let word_counts = task_parallelism(&file_paths);

    // Print the results
    for (word, count) in word_counts {
        println!("{}: {}", word, count);
    }
}
