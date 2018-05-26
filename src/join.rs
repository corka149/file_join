use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*, BufReader};

///
/// Reads a list of files and stores their content
///
pub struct FileJoinerReader<'a> {
  source: &'a [&'a str],
  target: &'a str,
  complete_content: Vec<u8>,
}

impl<'a> FileJoinerReader<'a> {
  pub fn new(source: &'a [&'a str], target: &'a str) -> FileJoinerReader<'a> {
    FileJoinerReader {
      source,
      target,
      complete_content: Vec::new(),
    }
  }

  pub fn read_all_files(mut self) -> io::Result<FileJoinerWriter<'a>> {
    for (i, f_name) in self.source.iter().enumerate() {
      let f = File::open(f_name)?;
      let mut reader = BufReader::new(f);
      reader.read_to_end(&mut self.complete_content)?;
      if i < (self.source.len() - 1) {
        self.complete_content.push(b'\n');
      }
    }
    Ok(FileJoinerWriter {
      complete_content: self.complete_content,
      target: self.target,
    })
  }

}

///
/// Write content to the given output file
///
pub struct FileJoinerWriter<'a> {
  complete_content: Vec<u8>,
  target: &'a str
}

impl<'a> FileJoinerWriter<'a> {
  pub fn new(
    complete_content: Vec<u8>,
    target: &'a str,
  ) -> FileJoinerWriter<'a> {
    FileJoinerWriter {
      complete_content,
      target
    }
  }

  pub fn write_output_file(self, overwrite: bool) -> io::Result<()> {
    let mut output_file = OpenOptions::new()
      .write(true)
      .create_new(!overwrite)
      .truncate(true)
      .open(&self.target)?;
    output_file.write_all(&self.complete_content)?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use std::fs::remove_file;
  use super::FileJoinerReader;

  #[test]
  fn test_read_all_files() {
    let files = vec!["test/test1.txt", "test/test2.txt", "test/test3.txt"];
    let file_joiner = FileJoinerReader::new(&files, "new_file.txt");
    file_joiner.read_all_files().unwrap();
    assert!(true);
  }

  #[test]
  fn test_write_output_file() {
    let files = vec!["test/test1.txt", "test/test2.txt", "test/test3.txt"];
    remove_file("test/new_file.txt");
    let file_joiner = FileJoinerReader::new(&files, "test/new_file.txt");
    let writer = file_joiner.read_all_files().unwrap();
    writer.write_output_file(false).unwrap();
    assert!(true)
  }
}
