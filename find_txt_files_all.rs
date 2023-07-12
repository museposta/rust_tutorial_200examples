

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
  // Get the current working directory.
  let current_directory = std::env::current_dir().unwrap();

  // Iterate over all the directories in the current working directory.
  for entry in current_directory.read_dir().unwrap() {
    // Check if the entry is a directory.
    if entry.is_dir() {
      // Recursively search the directory for text files.
      find_text_files(entry.path());
    } else {
      // Check if the entry is a text file.
      if entry.path().extension().ends_with(".txt") {
        // Print the path to the text file.
        println!("{}", entry.path());
      }
    }
  }
}

fn find_text_files(path: &Path) {
  // Iterate over all the files in the directory.
  for entry in path.read_dir().unwrap() {
    // Check if the entry is a file.
    if entry.is_file() {
      // Check if the file is a text file.
      if entry.path().extension().ends_with(".txt") {
        // Print the path to the text file.
        println!("{}", entry.path());
      }
    } else {
      // Recursively search the directory for text files.
      find_text_files(entry.path());
    }
  }
}