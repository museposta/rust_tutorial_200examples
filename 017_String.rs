
// Common Methods - String Object
// Sr.No.	Method	Signature	Description
// 1	new()	pub const fn new() → String	Creates a new empty String.
// 2	to_string()	fn to_string(&self) → String	Converts the given value to a String.
// 3	replace()	pub fn replace<'a, P>(&'a self, from: P, to: &str) → String	Replaces all matches of a pattern with another string.
// 4	as_str()	pub fn as_str(&self) → &str	Extracts a string slice containing the entire string.
// 5	push()	pub fn push(&mut self, ch: char)	Appends the given char to the end of this String.
// 6	push_str()	pub fn push_str(&mut self, string: &str)	Appends a given string slice onto the end of this String.
// 7	len()	pub fn len(&self) → usize	Returns the length of this String, in bytes.
// 8	trim()	pub fn trim(&self) → &str	Returns a string slice with leading and trailing whitespace removed.
// 9	split_whitespace()	pub fn split_whitespace(&self) → SplitWhitespace	Splits a string slice by whitespace and returns an iterator.
// 10	split()	pub fn split<'a, P>(&'a self, pat: P) → Split<'a, P> , where P is pattern can be &str, char, or a closure that determines the split.	Returns an iterator over substrings of this string slice, separated by characters matched by a pattern.
// 11	chars()	pub fn chars(&self) → Chars	Returns an iterator over the chars of a string slice.

fn print_str(data: &str){
    println!("{:?}", data);
}

fn main() {
    print_str("this is a string slice");
    let owned_string = "this is a own string!".to_owned();
    let some_string = String::from("this is some string");

    print_str(&owned_string);
    print_str(&some_string);


    let s = String::from("hello, 世界");
    let slice1 = &s[0..1]; //tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(slice1, "h");

    let slice2 = &s[7..10]; // Tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(slice2, "世");

    // Iterate through all chars in s
    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世')
        }
    }

    let mut s = String::new();
    s.push_str("hello");

    // Some bytes, in a vector
    let v: Vec<u8> = vec![104, 101, 108, 108, 111];

    // Turn a byte's vector into a String
    let s1: String = String::from_utf8(v).unwrap();

    assert_eq!(s, s1);

    println!("Success!");


}


// String : new()
// An empty string object is created using the new() method and its value is set to hello.

// fn main(){
//    let mut z = String::new();
//    z.push_str("hello");
//    println!("{}",z);
// }
// Output
// The above program generates the following output −

// hello
// String : to_string()
// To access all methods of String object, convert a string literal to object type using the to_string() function.

// fn main(){
//    let name1 = "Hello MustaphaBadici , 
//    Hello!".to_string();
//    println!("{}",name1);
// }
// Output
// The above program generates the following output −

// Hello MustaphaBadici , Hello!
// String : replace()
// The replace() function takes two parameters − the first parameter is a string pattern to search for and the second parameter is the new value to be replaced. In the above example, Hello appears two times in the name1 string.

// The replace function replaces all occurrences of the string Hello with Howdy.

// fn main(){
//    let name1 = "Hello MustaphaBadici , 
//    Hello!".to_string();         //String object
//    let name2 = name1.replace("Hello","Howdy");    //find and replace
//    println!("{}",name2);
// }
// Output
// The above program generates the following output −

// Howdy MustaphaBadici , Howdy!
// String : as_str()
// The as_str() function extracts a string slice containing the entire string.

// fn main() {
//    let example_string = String::from("example_string");
//    print_literal(example_string.as_str());
// }
// fn print_literal(data:&str ){
//    println!("displaying string literal {}",data);
// }
// Output
// The above program generates the following output −

// displaying string literal example_string
// String : push()
// The push() function appends the given char to the end of this String.

// fn main(){
//    let mut company = "Mustafa".to_string();
//    company.push('s');
//    println!("{}",company);
// }
// Output
// The above program generates the following output −

// Mustapha
// String : push_str()
// The push_str() function appends a given string slice onto the end of a String.

// fn main(){
//    let mut company = "Mustapha".to_string();
//    company.push_str(" Badici");
//    println!("{}",company);
// }
// Output
// The above program generates the following output −

// Mustapha Badici
// String : len()
// The len() function returns the total number of characters in a string (including spaces).

// fn main() {
//    let fullname = " Mustapha Badici";
//    println!("length is {}",fullname.len());
// }
// Output
// The above program generates the following output −

// length is 20
// String : trim()
// The trim() function removes leading and trailing spaces in a string. NOTE that this function will not remove the inline spaces.

// fn main() {
//    let fullname = " Mustapha Badici \r\n";
//    println!("Before trim ");
//    println!("length is {}",fullname.len());
//    println!();
//    println!("After trim ");
//    println!("length is {}",fullname.trim().len());
// }
// Output
// The above program generates the following output −

// Before trim
// length is 24

// After trim
// length is 15
// String :split_whitespace()
// The split_whitespace() splits the input string into different strings. It returns an iterator so we are iterating through the tokens as shown below −

// fn main(){
//    let msg = "Mustapha Badici has good t
//    utorials".to_string();
//    let mut i = 1;
   
//    for token in msg.split_whitespace(){
//       println!("token {} {}",i,token);
//       i+=1;
//    }
// }
// Output
// token 1 Mustapha
// token 2 Badici
// token 3 has
// token 4 good
// token 5 Mustapha
// String : split() string
// The split() string method returns an iterator over substrings of a string slice, separated by characters matched by a pattern. The limitation of the split() method is that the result cannot be stored for later use. The collect method can be used to store the result returned by split() as a vector.

// fn main() {
//    let fullname = "Kannan,Sudhakaran,MustaphaBadici";

//    for token in fullname.split(","){
//       println!("token is {}",token);
//    }

//    //store in a Vector
//    println!("\n");
//    let tokens:Vec<&str>= fullname.split(",").collect();
//    println!("firstName is {}",tokens[0]);
//    println!("lastname is {}",tokens[1]);
//    println!("company is {}",tokens[2]);
// }
// The above example splits the string fullname, whenever it encounters a comma (,).

// Output
// token is Kannan
// token is Sudhakaran
// token is MustaphaBadici

// firstName is Kannan
// lastname is Sudhakaran
// company is MustaphaBadici
// String : chars()
// Individual characters in a string can be accessed using the chars method. Let us consider an example to understand this.

// fn main(){
//    let n1 = "Mustapha".to_string();

//    for n in n1.chars(){
//       println!("{}",n);
//    }
// }

