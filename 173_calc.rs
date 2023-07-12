







// command line calculator.
// cargo run  "1" "+" "1"
// cargo run  "1" "-" "1"
// cargo run  "1" "*" "1"
// cargo run  "1" "//" "1"
// cargo run  "1" "%" "1"    // modulus

use std::env::{args, Args};

fn main() {
  let mut args: Args = args();

  let first: String = args.nth(1).unwrap();
  let operator: char = args.nth(0).unwrap().chars().next().unwrap();
  let second: String = args.nth(0).unwrap();

  let first_number = first.parse::<f32>().unwrap();
  let second_number = second.parse::<f32>().unwrap();
  let result = operate(operator, first_number, second_number);

  println!("{}", output(first_number, operator, second_number, result));
}

fn output(first_number: f32, operator: char, second_number: f32, result: f32) -> String {
  format!(
    "{} {} {} = {}",
    first_number, operator, second_number, result
  )
}

fn operate(operator: char, first_number: f32, second_number: f32) -> f32 {
  match operator {
    '+' => first_number + second_number,
    '-' => first_number - second_number,
    '/' => first_number / second_number,
    '%' => first_number % second_number,
    '*' | 'X' | 'x' => first_number * second_number,
    _ => panic!("Invalid operator used."),
  }
}





// // Basic Authentication
// // reqwest-badge cat-net-badge
// // Uses reqwest::RequestBuilder::basic_auth to perform a basic HTTP authentication.

// use reqwest::blocking::Client;
// use reqwest::Error;

// fn main() -> Result<(), Error> {
//     let client = Client::new();

//     let user_name = "testuser".to_string();
//     let password: Option<String> = None;

//     let response = client
//         .get("https://httpbin.org/")
//         .basic_auth(user_name, password)
//         .send();

//     println!("{:?}", response);

//     Ok(())
// }


// // Download a file to a temporary directory
// // reqwest-badge tempdir-badge cat-net-badge cat-filesystem-badge

// // Creates a temporary directory with tempfile::Builder and downloads a file over HTTP using reqwest::get asynchronously.

// // Creates a target File with name obtained from Response::url within tempdir() and copies downloaded data into it with io::copy. The temporary directory is automatically removed on program exit.


// use error_chain::error_chain;
// use std::io::copy;
// use std::fs::File;
// use tempfile::Builder;

// error_chain! {
//      foreign_links {
//          Io(std::io::Error);
//          HttpRequest(reqwest::Error);
//      }
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     let tmp_dir = Builder::new().prefix("example").tempdir()?;
//     let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
//     let response = reqwest::get(target).await?;

//     let mut dest = {
//         let fname = response
//             .url()
//             .path_segments()
//             .and_then(|segments| segments.last())
//             .and_then(|name| if name.is_empty() { None } else { Some(name) })
//             .unwrap_or("tmp.bin");

//         println!("file to download: '{}'", fname);
//         let fname = tmp_dir.path().join(fname);
//         println!("will be located under: '{:?}'", fname);
//         File::create(fname)?
//     };
//     let content =  response.text().await?;
//     copy(&mut content.as_bytes(), &mut dest)?;
//     Ok(())
// }


// // POST a file to paste-rs
// // reqwest-badge cat-net-badge

// // reqwest::Client establishes a connection to https://paste.rs following the reqwest::RequestBuilder pattern. Calling Client::post with a URL establishes the destination, RequestBuilder::body sets the content to send by reading the file, and RequestBuilder::send blocks until the file uploads and the response returns. read_to_string returns the response and displays in the console.


// use error_chain::error_chain;
// use std::fs::File;
// use std::io::Read;

//  error_chain! {
//      foreign_links {
//          HttpRequest(reqwest::Error);
//          IoError(::std::io::Error);
//      }
//  }
//  #[tokio::main]

// async fn main() -> Result<()> {
//     let paste_api = "https://paste.rs";
//     let mut file = File::open("message")?;

//     let mut contents = String::new();
//     file.read_to_string(&mut contents)?;

//     let client = reqwest::Client::new();
//     let res = client.post(paste_api)
//         .body(contents)
//         .send()
//         .await?;
//     let response_text = res.text().await?;
//     println!("Your paste is located at: {}",response_text );
//     Ok(())
// }




// // Make a partial download with HTTP range headers
// // reqwest-badge cat-net-badge

// // Uses reqwest::blocking::Client::head to get the Content-Length of the response.

// // The code then uses reqwest::blocking::Client::get to download the content in chunks of 10240 bytes, while printing progress messages. This exmple uses the synchronous reqwest module. The Range header specifies the chunk size and position.

// // The Range header is defined in RFC7233.


// use error_chain::error_chain;
// use reqwest::header::{HeaderValue, CONTENT_LENGTH, RANGE};
// use reqwest::StatusCode;
// use std::fs::File;
// use std::str::FromStr;

// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         Reqwest(reqwest::Error);
//         Header(reqwest::header::ToStrError);
//     }
// }

// struct PartialRangeIter {
//   start: u64,
//   end: u64,
//   buffer_size: u32,
// }

// impl PartialRangeIter {
//   pub fn new(start: u64, end: u64, buffer_size: u32) -> Result<Self> {
//     if buffer_size == 0 {
//       Err("invalid buffer_size, give a value greater than zero.")?;
//     }
//     Ok(PartialRangeIter {
//       start,
//       end,
//       buffer_size,
//     })
//   }
// }

// impl Iterator for PartialRangeIter {
//   type Item = HeaderValue;
//   fn next(&mut self) -> Option<Self::Item> {
//     if self.start > self.end {
//       None
//     } else {
//       let prev_start = self.start;
//       self.start += std::cmp::min(self.buffer_size as u64, self.end - self.start + 1);
//       Some(HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1)).expect("string provided by format!"))
//     }
//   }
// }

// fn main() -> Result<()> {
//   let url = "https://httpbin.org/range/102400?duration=2";
//   const CHUNK_SIZE: u32 = 10240;
    
//   let client = reqwest::blocking::Client::new();
//   let response = client.head(url).send()?;
//   let length = response
//     .headers()
//     .get(CONTENT_LENGTH)
//     .ok_or("response doesn't include the content length")?;
//   let length = u64::from_str(length.to_str()?).map_err(|_| "invalid Content-Length header")?;
    
//   let mut output_file = File::create("download.bin")?;
    
//   println!("starting download...");
//   for range in PartialRangeIter::new(0, length - 1, CHUNK_SIZE)? {
//     println!("range {:?}", range);
//     let mut response = client.get(url).header(RANGE, range).send()?;
    
//     let status = response.status();
//     if !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
//       error_chain::bail!("Unexpected server response: {}", status)
//     }
//     std::io::copy(&mut response, &mut output_file)?;
//   }
    
//   let content = response.text()?;
//   std::io::copy(&mut content.as_bytes(), &mut output_file)?;

//   println!("Finished with success!");
//   Ok(())
// }




// // Create and delete Gist with GitHub API
// // reqwest-badge serde-badge cat-net-badge cat-encoding-badge

// // Creates a gist with POST request to GitHub gists API v3 using Client::post and removes it with DELETE request using Client::delete.

// // The reqwest::Client is responsible for details of both requests including URL, body and authentication. The POST body from serde_json::json! macro provides arbitrary JSON body. Call to RequestBuilder::json sets the request body. RequestBuilder::basic_auth handles authentication. The call to RequestBuilder::send synchronously executes the requests.


// use error_chain::error_chain;
// use serde::Deserialize;
// use serde_json::json;
// use std::env;
// use reqwest::Client;

// error_chain! {
//     foreign_links {
//         EnvVar(env::VarError);
//         HttpRequest(reqwest::Error);
//     }
// }

// #[derive(Deserialize, Debug)]
// struct Gist {
//     id: String,
//     html_url: String,
// }

// #[tokio::main]
// async fn main() ->  Result<()> {
//     let gh_user = env::var("GH_USER")?;
//     let gh_pass = env::var("GH_PASS")?;

//     let gist_body = json!({
//         "description": "the description for this gist",
//         "public": true,
//         "files": {
//              "main.rs": {
//              "content": r#"fn main() { println!("hello world!");}"#
//             }
//         }});

//     let request_url = "https://api.github.com/gists";
//     let response = Client::new()
//         .post(request_url)
//         .basic_auth(gh_user.clone(), Some(gh_pass.clone()))
//         .json(&gist_body)
//         .send().await?;

//     let gist: Gist = response.json().await?;
//     println!("Created {:?}", gist);

//     let request_url = format!("{}/{}",request_url, gist.id);
//     let response = Client::new()
//         .delete(&request_url)
//         .basic_auth(gh_user, Some(gh_pass))
//         .send().await?;

//     println!("Gist {} deleted! Status code: {}",gist.id, response.status());
//     Ok(())
// }


// // The example uses HTTP Basic Auth in order to authorize access to GitHub API. Typical use case would employ one of the much more complex OAuth authorization flows.

// // Consume a paginated RESTful API
// // reqwest-badge serde-badge cat-net-badge cat-encoding-badge

// // Wraps a paginated web API in a convenient Rust iterator. The iterator lazily fetches the next page of results from the remote server as it arrives at the end of each page.


// use reqwest::Result;
// use serde::Deserialize;

// #[derive(Deserialize)]
// struct ApiResponse {
//     dependencies: Vec<Dependency>,
//     meta: Meta,
// }

// #[derive(Deserialize)]
// struct Dependency {
//     crate_id: String,
// }

// #[derive(Deserialize)]
// struct Meta {
//     total: u32,
// }

// struct ReverseDependencies {
//     crate_id: String,
//     dependencies: <Vec<Dependency> as IntoIterator>::IntoIter,
//     client: reqwest::blocking::Client,
//     page: u32,
//     per_page: u32,
//     total: u32,
// }

// impl ReverseDependencies {
//     fn of(crate_id: &str) -> Result<Self> {
//         Ok(ReverseDependencies {
//                crate_id: crate_id.to_owned(),
//                dependencies: vec![].into_iter(),
//                client: reqwest::blocking::Client::new(),
//                page: 0,
//                per_page: 100,
//                total: 0,
//            })
//     }

//     fn try_next(&mut self) -> Result<Option<Dependency>> {
//         if let Some(dep) = self.dependencies.next() {
//             return Ok(Some(dep));
//         }

//         if self.page > 0 && self.page * self.per_page >= self.total {
//             return Ok(None);
//         }

//         self.page += 1;
//         let url = format!("https://crates.io/api/v1/crates/{}/reverse_dependencies?page={}&per_page={}",
//                           self.crate_id,
//                           self.page,
//                           self.per_page);

//         let response = self.client.get(&url).send()?.json::<ApiResponse>()?;
//         self.dependencies = response.dependencies.into_iter();
//         self.total = response.meta.total;
//         Ok(self.dependencies.next())
//     }
// }

// impl Iterator for ReverseDependencies {
//     type Item = Result<Dependency>;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.try_next() {
//             Ok(Some(dep)) => Some(Ok(dep)),
//             Ok(None) => None,
//             Err(err) => Some(Err(err)),
//         }
//     }
// }

// fn main() -> Result<()> {
//     for dep in ReverseDependencies::of("serde")? {
//         println!("reverse dependency: {}", dep?.crate_id);
//     }
//     Ok(())
// }



// // Check if an API resource exists
// // reqwest-badge cat-net-badge

// // Query the GitHub Users Endpoint using a HEAD request (Client::head) and then inspect the response code to determine success. This is a quick way to query a rest resource without needing to receive a body. reqwest::Client configured with ClientBuilder::timeout ensures a request will not last longer than a timeout.

// // Due to both ClientBuilder::build and [ReqwestBuilder::send] returning reqwest::Error types, the shortcut reqwest::Result is used for the main function return type.


// use reqwest::Result;
// use std::time::Duration;
// use reqwest::ClientBuilder;

// #[tokio::main]
// async fn main() -> Result<()> {
//     let user = "ferris-the-crab";
//     let request_url = format!("https://api.github.com/users/{}", user);
//     println!("{}", request_url);

//     let timeout = Duration::new(5, 0);
//     let client = ClientBuilder::new().timeout(timeout).build()?;
//     let response = client.head(&request_url).send().await?;

//     if response.status().is_success() {
//         println!("{} is a user!", user);
//     } else {
//         println!("{} is not a user!", user);
//     }

//     Ok(())
// }



// // call web api
// use serde::Deserialize;
// use reqwest::Error;

// #[derive(Deserialize, Debug)]
// struct User {
//     login: String,
//     id: u32,
// }

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
//                               owner = "rust-lang-nursery",
//                               repo = "rust-cookbook");
//     println!("{}", request_url);
//     let response = reqwest::get(&request_url).await?;

//     let users: Vec<User> = response.json().await?;
//     println!("{:?}", users);
//     Ok(())
// }


// // async
// use error_chain::error_chain;

// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//     }
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//     let res = reqwest::get("http://httpbin.org/get").await?;
//     println!("Status: {}", res.status());
//     println!("Headers:\n{:#?}", res.headers());

//     let body = res.text().await?;
//     println!("Body:\n{}", body);
//     Ok(())
// }



// // http get request
// use error_chain::error_chain;
// use std::io::Read;

// error_chain! {
//     foreign_links {
//         Io(std::io::Error);
//         HttpRequest(reqwest::Error);
//     }
// }
// fn main() -> Result<()> {
//     let mut res = reqwest::blocking::get("http://httpbin.org/get")?;
//     let mut body = String::new();
//     res.read_to_string(&mut body)?;
//     println!("Status: {}", res.status());
//     println!("Headers:\n{:#?}", res.headers());
//     println!("Body:\n{}", body);
//     Ok(())
// }



// // parse mime type
// use error_chain::error_chain;
// use mime::Mime;
// use std::str::FromStr;
// use reqwest::header::CONTENT_TYPE;

//  error_chain! {
//     foreign_links {
//         Reqwest(reqwest::Error);
//         Header(reqwest::header::ToStrError);
//         Mime(mime::FromStrError);
//     }
//  }

// #[tokio::main]
// async fn main() -> Result<()> {
//     let response = reqwest::get("https://www.rust-lang.org/logos/rust-logo-32x32.png").await?;
//     let headers = response.headers();

//     match headers.get(CONTENT_TYPE) {
//         None => {
//             println!("The response does not contain a Content-Type header.");
//         }
//         Some(content_type) => {
//             let content_type = Mime::from_str(content_type.to_str()?)?;
//             let media_type = match (content_type.type_(), content_type.subtype()) {
//                 (mime::TEXT, mime::HTML) => "a HTML document",
//                 (mime::TEXT, _) => "a text document",
//                 (mime::IMAGE, mime::PNG) => "a PNG image",
//                 (mime::IMAGE, _) => "an image",
//                 _ => "neither text nor image",
//             };

//             println!("The reponse contains {}.", media_type);
//         }
//     };

//     Ok(())
// }



// // Create a base URL by removing path segments
// use url::Url;

// fn main() -> Result<()> {
//     let full = "https://github.com/rust-lang/cargo?asdf";

//     let url = Url::parse(full)?;
//     let base = base_url(url)?;

//     assert_eq!(base.as_str(), "https://github.com/");
//     println!("The base of the URL is: {}", base);

//     Ok(())
// }

// fn base_url(mut url: Url) -> Result<Url> {
//     match url.path_segments_mut() {
//         Ok(mut path) => {
//             path.clear();
//         }
//         Err(_) => {
//             return Err(Error::from_kind(ErrorKind::CannotBeABase));
//         }
//     }

//     url.set_query(None);

//     Ok(url)
// }







// // Extract all unique links from a MediaWiki markup
// use lazy_static::lazy_static;
// use regex::Regex;
// use std::borrow::Cow;
// use std::collections::HashSet;
// use std::error::Error;

// fn extract_links(content: &str) -> HashSet<Cow<str>> {
//   lazy_static! {
//     static ref WIKI_REGEX: Regex = Regex::new(
//       r"(?x)
//                 \[\[(?P<internal>[^\[\]|]*)[^\[\]]*\]\]    # internal links
//                 |
//                 (url=|URL\||\[)(?P<external>http.*?)[ \|}] # external links
//             "
//     )
//     .unwrap();
//   }

//   let links: HashSet<_> = WIKI_REGEX
//     .captures_iter(content)
//     .map(|c| match (c.name("internal"), c.name("external")) {
//       (Some(val), None) => Cow::from(val.as_str().to_lowercase()),
//       (None, Some(val)) => Cow::from(val.as_str()),
//       _ => unreachable!(),
//     })
//     .collect();

//   links
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//   let content = reqwest::get(
//     "https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&action=raw",
//   )
//   .await?
//   .text()
//   .await?;

//   println!("{:#?}", extract_links(content.as_str()));

//   Ok(())
// }




// // Check a webpage for broken links
// use error_chain::error_chain;
// use reqwest::StatusCode;
// use select::document::Document;
// use select::predicate::Name;
// use std::collections::HashSet;
// use url::{Position, Url};

// error_chain! {
//   foreign_links {
//       ReqError(reqwest::Error);
//       IoError(std::io::Error);
//       UrlParseError(url::ParseError);
//       JoinError(tokio::task::JoinError);
//   }
// }

// async fn get_base_url(url: &Url, doc: &Document) -> Result<Url> {
//   let base_tag_href = doc.find(Name("base")).filter_map(|n| n.attr("href")).nth(0);
//   let base_url =
//     base_tag_href.map_or_else(|| Url::parse(&url[..Position::BeforePath]), Url::parse)?;
//   Ok(base_url)
// }

// async fn check_link(url: &Url) -> Result<bool> {
//   let res = reqwest::get(url.as_ref()).await?;
//   Ok(res.status() != StatusCode::NOT_FOUND)
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//   let url = Url::parse("https://www.rust-lang.org/en-US/")?;
//   let res = reqwest::get(url.as_ref()).await?.text().await?;
//   let document = Document::from(res.as_str());
//   let base_url = get_base_url(&url, &document).await?;
//   let base_parser = Url::options().base_url(Some(&base_url));
//   let links: HashSet<Url> = document
//     .find(Name("a"))
//     .filter_map(|n| n.attr("href"))
//     .filter_map(|link| base_parser.parse(link).ok())
//     .collect();
//     let mut tasks = vec![];

//     for link in links {
//         tasks.push(tokio::spawn(async move {
//             if check_link(&link).await.unwrap() {
//                 println!("{} is OK", link);
//             } else {
//                 println!("{} is Broken", link);
//             }
//         }));
//     }

//     for task in tasks {
//         task.await?
//     }

//   Ok(())
// }





// // extract links
// use error_chain::error_chain;
// use select::document::Document;
// use select::predicate::Name;

// error_chain! {
//       foreign_links {
//           ReqError(reqwest::Error);
//           IoError(std::io::Error);
//       }
// }

// #[tokio::main]
// async fn main() -> Result<()> {
//   let res = reqwest::get("https://www.rust-lang.org/en-US/")
//     .await?
//     .text()
//     .await?;

//   Document::from(res.as_str())
//     .find(Name("a"))
//     .filter_map(|n| n.attr("href"))
//     .for_each(|x| println!("{}", x));

//   Ok(())
// }


// // filter a log file
// use std::fs::File;
// use std::io::{BufReader, BufRead};
// use regex::RegexSetBuilder;

// fn main() -> Result<()> {
//     let log_path = "application.log";
//     let buffered = BufReader::new(File::open(log_path)?);

//     let set = RegexSetBuilder::new(&[
//         r#"version "\d\.\d\.\d""#,
//         r#"\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}:443"#,
//         r#"warning.*timeout expired"#,
//     ]).case_insensitive(true)
//         .build()?;

//     buffered
//         .lines()
//         .filter_map(|line| line.ok())
//         .filter(|line| set.is_match(line.as_str()))
//         .for_each(|x| println!("{}", x));

//     Ok(())
// }


// // extract telephone numbers ERROR
// use regex::Regex;
// use std::fmt;

// struct PhoneNumber<'a> {
//     area: &'a str,
//     exchange: &'a str,
//     subscriber: &'a str,
// }

// impl<'a> fmt::Display for PhoneNumber<'a> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "1 ({}) {}-{}", self.area, self.exchange, self.subscriber)
//     }
// }

// fn main() -> Result<()> {
//     let phone_text = "
//     +1 505 881 9292 (v) +1 505 778 2212 (c) +1 505 881 9297 (f)
//     (202) 991 9534
//     Alex 5553920011
//     1 (800) 233-2010
//     1.299.339.1020";

//     let re = Regex::new(
//         r#"(?x)
//           (?:\+?1)?                       # Country Code Optional
//           [\s\.]?
//           (([2-9]\d{2})|\(([2-9]\d{2})\)) # Area Code
//           [\s\.\-]?
//           ([2-9]\d{2})                    # Exchange Code
//           [\s\.\-]?
//           (\d{4})                         # Subscriber Number"#,
//     )?;

//     let phone_numbers = re.captures_iter(phone_text).filter_map(|cap| {
//         let groups = (cap.get(2).or(cap.get(3)), cap.get(4), cap.get(5));
//         match groups {
//             (Some(area), Some(ext), Some(sub)) => Some(PhoneNumber {
//                 area: area.as_str(),
//                 exchange: ext.as_str(),
//                 subscriber: sub.as_str(),
//             }),
//             _ => None,
//         }
//     });

//     assert_eq!(
//         phone_numbers.map(|m| m.to_string()).collect::<Vec<_>>(),
//         vec![
//             "1 (505) 881-9292",
//             "1 (505) 778-2212",
//             "1 (505) 881-9297",
//             "1 (202) 991-9534",
//             "1 (555) 392-0011",
//             "1 (800) 233-2010",
//             "1 (299) 339-1020",
//         ]
//     );

//     Ok(())
// }




// // matrix serialize ERROR

// extern crate nalgebra;
// extern crate serde_json;

// use nalgebra::DMatrix;

// fn main() -> Result<(), std::io::Error> {
//     let row_slice: Vec<i32> = (1..5001).collect();
//     let matrix = DMatrix::from_row_slice(50, 100, &row_slice);

//     // serialize matrix
//     let serialized_matrix = serde_json::to_string(&matrix)?;

//     // deserialize matrix
//     let deserialized_matrix: DMatrix<i32> = serde_json::from_str(&serialized_matrix)?;

//     // verify that `deserialized_matrix` is equal to `matrix`
//     assert!(deserialized_matrix == matrix);

//     Ok(())
// }






// // ERROR
// use approx::assert_abs_diff_eq;
// use ndarray::Array;

// fn main() {
//   let a = Array::from(vec![1., 2., 3., 4., 5.]);
//   let b = Array::from(vec![5., 4., 3., 2., 1.]);
//   let mut c = Array::from(vec![1., 2., 3., 4., 5.]);
//   let mut d = Array::from(vec![5., 4., 3., 2., 1.]);

//   let z = a + b;
//   let w =  &c + &d;

//   assert_abs_diff_eq!(z, Array::from(vec![6., 6., 6., 6., 6.]));

//   println!("c = {}", c);
//   c[0] = 10.;
//   d[1] = 10.;

//   assert_abs_diff_eq!(w, Array::from(vec![6., 6., 6., 6., 6.]));

// }


// // external command ERROR
// use std::process::Command;
// use regex::Regex;

// #[derive(PartialEq, Default, Clone, Debug)]
// struct Commit {
//     hash: String,
//     message: String,
// }

// fn main() -> Result<()> {
//     let output = Command::new("git").arg("log").arg("--oneline").output()?;

//     if !output.status.success() {
//         error_chain::bail!("Command executed with failing error code");
//     }

//     let pattern = Regex::new(r"(?x)
//                                ([0-9a-fA-F]+) # commit hash
//                                (.*)           # The commit message")?;

//     String::from_utf8(output.stdout)?
//         .lines()
//         .filter_map(|line| pattern.captures(line))
//         .map(|cap| {
//                  Commit {
//                      hash: cap[1].to_string(),
//                      message: cap[2].trim().to_string(),
//                  }
//              })
//         .take(5)
//         .for_each(|x| println!("{:?}", x));

//     Ok(())
// }




// // find json files ERROR
// use walkdir::WalkDir;
// //fn main() -> Result<(),std::io::Error> {
// fn main() {
//     let path = "c:/temp";
//     for entry in WalkDir::new(path)
//             .follow_links(true)
//             .into_iter()
//             .filter_map(|e| e.ok()) {
//         let f_name = entry.file_name().to_string_lossy();
//         let sec = entry.metadata()?.modified()?;
//         if f_name.ends_with(".json") && sec.elapsed()?.as_secs() < 86400 {
//             println!("{}", f_name);
//         }
//     }

//     // Ok(())
// }


// // serde json
// use serde_json::json;
// use serde_json::{Value, Error};

// fn main() -> Result<(), Error> {
//     let j = r#"{
//                  "userid": 103609,
//                  "verified": true,
//                  "access_privileges": [
//                    "user",
//                    "admin"
//                  ]
//                }"#;

//     let parsed: Value = serde_json::from_str(j)?;

//     let expected = json!({
//         "userid": 103609,
//         "verified": true,
//         "access_privileges": [
//             "user",
//             "admin"
//         ]
//     });

//     println!(" parsed json : {}", parsed);

//     assert_eq!(parsed, expected);

//     Ok(())
// }

// // transform csv  ERROR
// use csv::{Reader, Writer};
// use serde::{de, Deserialize, Deserializer};
// use std::str::FromStr;

// #[derive(Debug)]
// struct HexColor {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// #[derive(Debug, Deserialize)]
// struct Row {
//     color_name: String,
//     color: HexColor,
// }

// impl FromStr for HexColor {
//     type Err = Error;

//     fn from_str(hex_color: &str) -> std::result::Result<Self, Self::Err> {
//         let trimmed = hex_color.trim_matches('#');
//         if trimmed.len() != 6 {
//             Err("Invalid length of hex string".into())
//         } else {
//             Ok(HexColor {
//                 red: u8::from_str_radix(&trimmed[..2], 16)?,
//                 green: u8::from_str_radix(&trimmed[2..4], 16)?,
//                 blue: u8::from_str_radix(&trimmed[4..6], 16)?,
//             })
//         }
//     }
// }

// impl<'de> Deserialize<'de> for HexColor {
//     fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let s = String::deserialize(deserializer)?;
//         FromStr::from_str(&s).map_err(de::Error::custom)
//     }
// }

// fn main() -> Result<(),std::io::Error> {
//     let data = "color_name,color
// red,#ff0000
// green,#00ff00
// blue,#0000FF
// periwinkle,#ccccff
// magenta,#ff00ff"
//         .to_owned();
//     let mut out = Writer::from_writer(vec![]);
//     let mut reader = Reader::from_reader(data.as_bytes());
//     for result in reader.deserialize::<Row>() {
//         let res = result?;
//         out.serialize((
//             res.color_name,
//             res.color.red,
//             res.color.green,
//             res.color.blue,
//         ))?;
//     }
//     let written = String::from_utf8(out.into_inner()?)?;
//     assert_eq!(Some("magenta,255,0,255"), written.lines().last());
//     println!("{}", written);
//     Ok(())
// }

// // serde serialize csv ERROR.
// se serde::Serialize;
// use std::io;

// #[derive(Serialize)]
// struct Record<'a> {
//     name: &'a str,
//     place: &'a str,
//     id: u64,
// }

// fn main() -> Result<(),std::io::Error> {
//     let mut wtr = csv::Writer::from_writer(io::stdout());

//     let rec1 = Record { name: "Mark", place: "Melbourne", id: 56};
//     let rec2 = Record { name: "Ashley", place: "Sydney", id: 64};
//     let rec3 = Record { name: "Akshat", place: "Delhi", id: 98};

//     wtr.serialize(rec1)?;
//     wtr.serialize(rec2)?;
//     wtr.serialize(rec3)?;

//     wtr.flush()?;

//     Ok(())
// }



// // CSV error ERROR
// use csv::Error;
// use serde::Deserialize;

// #[derive(Debug, Deserialize)]
// struct Record {
//     name: String,
//     place: String,
//     #[serde(deserialize_with = "csv::invalid_option")]
//     id: Option<u64>,
// }

// fn main() -> Result<(), Error> {
//     let data = "name,place,id
// mark,sydney,46.5
// ashley,zurich,92
// akshat,delhi,37
// alisha,colombo,xyz";

//     let mut rdr = csv::Reader::from_reader(data.as_bytes());
//     for result in rdr.deserialize() {
//         let record: Record = result?;
//         println!("{:?}", record);
//     }

//     Ok(())
// }

// // csv filter ERROR
// use std::io;

// fn main() -> Result<()> {
//     let query = "CA";
//     let data = "\
// City,State,Population,Latitude,Longitude
// Kenai,AK,7610,60.5544444,-151.2583333
// Oakman,AL,,33.7133333,-87.3886111
// Sandfort,AL,,32.3380556,-85.2233333
// West Hollywood,CA,37031,34.0900000,-118.3608333";

//     let mut rdr = csv::ReaderBuilder::new().from_reader(data.as_bytes());
//     let mut wtr = csv::Writer::from_writer(io::stdout());

//     wtr.write_record(rdr.headers()?)?;

//     for result in rdr.records() {
//         let record = result?;
//         if record.iter().any(|field| field == query) {
//             wtr.write_record(&record)?;
//         }
//     }

//     wtr.flush()?;
//     Ok(())
// }

// // csv with different delimineter ERROR
// use csv::Error;
// use serde::Deserialize;
// #[derive(Debug, Deserialize)]
// struct Record {
//     name: String,
//     place: String,
//     #[serde(deserialize_with = "csv::invalid_option")]
//     id: Option<u64>,
// }

// use csv::ReaderBuilder;

// fn main() -> Result<(), Error> {
//     let data = "name\tplace\tid
//         Mark\tMelbourne\t46
//         Ashley\tZurich\t92";

//     let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(data.as_bytes());
//     for result in reader.deserialize::<Record>() {
//         println!("{:?}", result?);
//     }

//     Ok(())
// }

// // serde. desializze ERROR.
// use serde::Deserialize;
// #[derive(Deserialize)]
// struct Record {
//     year: u16,
//     make: String,
//     model: String,
//     description: String,
// }

// fn main() -> Result<(), csv::Error> {
//     let csv = "year,make,model,description
// 1948,Porsche,356,Luxury sports car
// 1967,Ford,Mustang fastback 1967,American car";

//     let mut reader = csv::Reader::from_reader(csv.as_bytes());

//     for record in reader.deserialize() {
//         let record: Record = record?;
//         println!(
//             "In {}, {} built the {} model. It is a {}.",
//             record.year,
//             record.make,
//             record.model,
//             record.description
//         );
//     }

//     Ok(())
// }

// // csv
// use csv::Error;

// fn main() -> Result<(), Error> {
//     let csv = "year,make,model,description
//         1948,Porsche,356,Luxury sports car
//         1967,Ford,Mustang fastback 1967,American car";

//     let mut reader = csv::Reader::from_reader(csv.as_bytes());
//     for record in reader.records() {
//         let record = record?;
//         println!(
//             "In {}, {} built the {} model. It is a {}.",
//             &record[0],
//             &record[1],
//             &record[2],
//             &record[3]
//         );
//     }

//     Ok(())
// }



// // log in custom location ERROR
// use log::LevelFilter;
// use log4rs::append::file::FileAppender;
// use log4rs::encode::pattern::PatternEncoder;
// use log4rs::config::{Appender, Config, Root};

// // fn main() -> Result<()> {
// fn main()  {
//     let logfile = FileAppender::builder()
//         .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
//         .build("log/output.log")?;

//     let config = Config::builder()
//         .appender(Appender::builder().build("logfile", Box::new(logfile)))
//         .build(Root::builder()
//                    .appender("logfile")
//                    .build(LevelFilter::Info))?;

//     // log4rs::init_config(config)?;
//     log4rs::init_config(config).unwrap();

//     log::info!("Hello, world!");

//     // Ok(())
// }



// mod foo {
//     mod bar {
//         pub fn run() {
//             log::warn!("[bar] warn");
//             log::info!("[bar] info");
//             log::debug!("[bar] debug");
//         }
//     }

//     pub fn run() {
//         log::warn!("[foo] warn");
//         log::info!("[foo] info");
//         log::debug!("[foo] debug");
//         bar::run();
//     }
// }

// fn main() {
//     env_logger::init();
//     log::warn!("[root] warn");
//     log::info!("[root] info");
//     log::debug!("[root] debug");
//     foo::run();
// }

// // custom logger.

// use log::{Record, Level, Metadata, LevelFilter, SetLoggerError};

// static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

// struct ConsoleLogger;

// impl log::Log for ConsoleLogger {
//   fn enabled(&self, metadata: &Metadata) -> bool {
//      metadata.level() <= Level::Info
//     }

//     fn log(&self, record: &Record) {
//         if self.enabled(record.metadata()) {
//             println!("Rust says: {} - {}", record.level(), record.args());
//         }
//     }

//     fn flush(&self) {}
// }

// fn main() -> Result<(), SetLoggerError> {
//     log::set_logger(&CONSOLE_LOGGER)?;
//     log::set_max_level(LevelFilter::Info);

//     log::info!("hello log");
//     log::warn!("warning");
//     log::error!("oops");
//     Ok(())
// }

// // ERROR
// use env_logger::{Builder, Target};
// use log::*;

// fn main() {
//     Builder::new()
//         .target(Target::Stdout)
//         .init();

//     log::error!("This error has been printed to Stdout");
// }

// // ERROR
// fn execute_query(_query: &str) -> Result<(), &'static str> {
//     Err("I'm afraid I can't do that")
// }

// fn main() {
//     env_logger::init();

//     let response = execute_query("DROP TABLE students");
//     if let Err(err) = response {
//         log::error!("Failed to execute query: {}", err);
//     }
// }

// use log::*; // ERROR RUST_LOG=debug cargo run
// // use std::{log::*}

// // log
// fn execute_query(query: &str) {
//     log::debug!("Executing query: {}", query);
// }

// fn main() {
//     env_logger::init();

//     execute_query("DROP TABLE students");
// }





// // unix timestamp ERROR

// use chrono::{NaiveDate, NaiveDateTime};

// fn main() {
//     let date_time: NaiveDateTime = NaiveDate::from_ymd(2017, 11, 12).and_hms(17, 33, 44);
//     println!(
//         "Number of seconds between 1970-01-01 00:00:00 and {} is {}.",
//         date_time, date_time.timestamp());

//     let date_time_after_a_billion_seconds = NaiveDateTime::from_timestamp_opt(1_000_000_000, 0);
//     println!(
//         "Date after a billion seconds since 1970-01-01 00:00:00 was {}.",
//         date_time_after_a_billion_seconds);
// }



// // local time ERROR
// use chrono::{DateTime, FixedOffset, Local, Utc};

// fn main() {
//     let local_time = Local::now();
//     let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
//     let china_timezone = FixedOffset::east_opt(8 * 3600);
//     let rio_timezone = FixedOffset::west_opt(2 * 3600);
//     println!("Local time now is {}", local_time);
//     println!("UTC time now is {}", utc_time);
//     println!(
//         "Time in Hong Kong now is {}",
//         utc_time.with_timezone(&china_timezone)
//     );
//     println!("Time in Rio de Janeiro now is {}", utc_time.with_timezone(&rio_timezone));
// }



// // duration and calculation
// use std::time::{Duration, Instant};

// fn main() {
//     let start = Instant::now();
//     expensive_function();
//     let duration = start.elapsed();

//     println!("Time elapsed in expensive_function() is: {:?}", duration);
// }

// // sqlite ERROR

// use rusqlite::{Connection, Result};
// use rusqlite::NO_PARAMS;

// fn main() -> Result<()> {
//     let conn = Connection::open("cats.db")?;

//     conn.execute(
//         "create table if not exists cat_colors (
//              id integer primary key,
//              name text not null unique
//          )",
//         NO_PARAMS,
//     )?;
//     conn.execute(
//         "create table if not exists cats (
//              id integer primary key,
//              name text not null,
//              color_id integer not null references cat_colors(id)
//          )",
//         NO_PARAMS,
//     )?;

//     Ok(())
// }

// // Generate jpg thumbnails in parallel ERROR...
// // rayon-badge glob-badge image-badge cat-concurrency-badge cat-filesystem-badge

// // This example generates thumbnails for all .jpg files in the current directory then saves them in a new folder called thumbnails.

// // glob::glob_with finds jpeg files in current directory. rayon resizes images in parallel using par_iter calling DynamicImage::resize.

// use std::path::Path;
// use std::fs::create_dir_all;

// use glob::{glob_with, MatchOptions};
// use image::{FilterType, ImageError};
// use rayon::prelude::*;
// // fn main() -> Result<()> {

//     fn main() -> Result<()> {
//     let options: MatchOptions = Default::default();
//     let files: Vec<_> = glob_with("*.jpg", options)?
//         .filter_map(|x| x.ok())
//         .collect();

//     if files.len() == 0 {
//         error_chain::bail!("No .jpg files found in current directory");
//     }

//     let thumb_dir = "thumbnails";
//     create_dir_all(thumb_dir)?;

//     println!("Saving {} thumbnails into '{}'...", files.len(), thumb_dir);

//     let image_failures: Vec<> = files
//         .par_iter()
//         .map(|path| {
//             make_thumbnail(path, thumb_dir, 300)
//                 .map_err(|e| e.chain_err(|| path.display().to_string()))
//         })
//         .filter_map(|x| x.err())
//         .collect();

//     image_failures.iter().for_each(|x| println!("{}", x.display_chain()));

//     println!("{} thumbnails saved successfully", files.len() - image_failures.len());
//     Ok(())
// }

// fn make_thumbnail<PA, PB>(original: PA, thumb_dir: PB, longest_edge: u32) -> Result<()>
// where
//     PA: AsRef<Path>,
//     PB: AsRef<Path>,
// {
//     let img = image::open(original.as_ref())?;
//     let file_path = thumb_dir.as_ref().join(original);

//     Ok(img.resize(longest_edge, longest_edge, FilterType::Nearest)
//         .save(file_path)?)
// }



// // global state. mutex ERROR

// use lazy_static::lazy_static;
// use std::sync::Mutex;

// lazy_static! {
//     static ref FRUIT: Mutex<Vec<String>> = Mutex::new(Vec::new());
// }

// fn insert(fruit: &str) -> Result<()> {
//     let mut db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;
//     db.push(fruit.to_string());
//     Ok(())
// }

// fn main() -> Result<()> {
//     insert("apple")?;
//     insert("orange")?;
//     insert("peach")?;
//     {
//         let db = FRUIT.lock().map_err(|_| "Failed to acquire MutexGuard")?;

//         db.iter().enumerate().for_each(|(i, item)| println!("{}: {}", i, item));
//     }
//     insert("grape")?;
//     Ok(())
// }























// use error_chain::error_chain;
// use glob::{glob_with, MatchOptions};

// error_chain! {
//     foreign_links {
//         Glob(glob::GlobError);
//         Pattern(glob::PatternError);
//     }
// }

// fn main() -> Result<()> {
//     let options = MatchOptions {
//         case_sensitive: false,
//         ..Default::default()
//     };

//     for entry in glob_with("/media/img_[0-9]*.png", options)? {
//         println!("{}", entry?.display());
//     }

//     Ok(())
// }

// // // find all png files ERROR
// use glob::glob;
// use std::io::Result;

// fn main() -> Result<()> {
//     for entry in glob("**/*.png")? {
//         println!("{}", entry?.display());
//     }

//     Ok(())
// }



// // find json files. error
// use walkdir::WalkDir;

// fn main() -> Result<()> {
//     for entry in WalkDir::new(".")
//             .follow_links(true)
//             .into_iter()
//             .filter_map(|e| e.ok()) {
//         let f_name = entry.file_name().to_string_lossy();
//         let sec = entry.metadata()?.modified()?;

//         if f_name.ends_with(".rs") && sec.elapsed()?.as_secs() < 86400 {
//             println!("{}", f_name);
//         }
//     }

//     Ok(())
// }

// find dublicate files
// use std::collections::HashMap;
// use walkdir::WalkDir;

// fn main() {
//     let mut filenames = HashMap::new();

//     for entry in WalkDir::new(".")
//             .into_iter()
//             .filter_map(Result::ok)
//             .filter(|e| !e.file_type().is_dir()) {
//         let f_name = String::from(entry.file_name().to_string_lossy());
//         let counter = filenames.entry(f_name.clone()).or_insert(0);
//         *counter += 1;

//        // if *counter == 2 {
//             println!("{}", f_name);
//         //}
//     }
// }
