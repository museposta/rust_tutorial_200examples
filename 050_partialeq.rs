


// fn similarity(string1: &str, string2: &str) -> i32 {
//     let mut common_bytes = 0;
//     let mut total_bytes = 0;
//     for (byte1, byte2) in string1.bytes().zip(string2.bytes()) {
//       if byte1 == byte2 {
//         common_bytes += 1;
//       }
//       total_bytes += 1;
//     }

//     100 * common_bytes / total_bytes
// }

enum BookFormat {
    Paperback,
    Hardback,
    Ebook,
}

struct Book {
    isbn: i32,
    format: BookFormat,
}

impl PartialEq for Book {
    fn eq(&self, other: &Self) -> bool {
        self.isbn == other.isbn
    }
}




fn similarity(string1: &str, string2: &str) -> i32 {
    let mut common_words = 0;
    let mut total_words = 0;
    for word1 in string1.split_whitespace() {
      for word2 in string2.split_whitespace() {
        if word1 == word2 {
          common_words += 1;
          break;
        }
      }
      total_words += 1;
    }
        println!("{} {}", common_words,total_words );  
    100 * common_words / total_words
  }

  
  fn main() {
    let string1 = "world! Hello,that will take some time to find an answer ";
    let string2 = "Goodbye, world! one more drink";
    let similarity_percentage = similarity(string1, string2);
    println!(
      "The similarity percentage between the two strings is {}%.",
      similarity_percentage
    );


    let b1 = Book { isbn: 3, format: BookFormat::Paperback };
    let b2 = Book { isbn: 3, format: BookFormat::Ebook };
    let b3 = Book { isbn: 10, format: BookFormat::Paperback };
    
    assert!(b1 == b2);
    assert!(b1 != b3);
    


  }