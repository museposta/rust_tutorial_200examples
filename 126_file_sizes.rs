


// // file sizes
use walkdir::WalkDir;

fn main() {
    let total_size = WalkDir::new("C:/Users/musep/OneDrive/Belgeler")
        .min_depth(1)
        .max_depth(99)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.metadata().ok())
        .filter(|metadata| metadata.is_file())
        .fold(0, |acc, m| acc + m.len());

    println!("Total size: {} mb.", total_size/1000000);
}