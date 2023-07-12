

use std::fs::File;

fn main() {
   let f = File::open("main.jpg");   // main.jpg doesn't exist
   match f {
      Ok(f)=> {
         println!("file found {:?}",f);
      },
      Err(e)=> {
         println!("file not found \n{:?}",e);   //handled error
      }
   }
   println!("end of main");
}


// The commonly used methods of the File struct are listed in the table below −

// Sr.No	Module	Method	Signature	Description
// 1	std::fs::File	        open()	pub fn open<P: AsRef>(path: P) -> Result	The open static method can be used to open a file in read-only mode.
// 2	std::fs::File	        create()	pub fn create<P: AsRef>(path: P) -> Result	Static method opens a file in write-only mode. If the file already existed, the old content is destroyed. Otherwise, a new file is created.
// 3	std::fs::remove_file	remove_file()	pub fn remove_file<P: AsRef>(path: P) -> Result<()>	Removes a file from the filesystem. There is no guarantee that the file is immediately deleted.
// 4	std::fs::OpenOptions	append()	pub fn append(&mut self, append: bool) -> &mut OpenOptions	Sets the option for the append mode of file.
// 5	std::io::Writes	        write_all()	fn write_all(&mut self, buf: &[u8]) -> Result<()>	Attempts to write an entire buffer into this write.
// 6	std::io::Read	        read_to_string()	fn read_to_string(&mut self, buf: &mut String) -> Result	Reads all bytes until EOF in this source, appending them to buf.