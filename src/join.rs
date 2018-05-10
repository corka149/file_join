use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io;

///
/// Reads a list of files and stores their content
///
struct FileJoinerReader<'a> {
    source: &'a [&'a str],
    target: &'a str,
    complete_content: String
}

impl<'a> FileJoinerReader<'a> {

    pub fn new(source: &'a [&'a str], target: &'a str) -> FileJoinerReader<'a> {
        FileJoinerReader {
            source,
            target,
            complete_content: String::new()
        }
    }

    pub fn read_all_files(mut self) -> io::Result<FileJoinerWriter<'a>> {
        for f_name in self.source {
            let f = File::open(f_name)?;
            let mut reader = BufReader::new(f);
            reader.read_to_string(&mut self.complete_content)?;
        }
        Ok(FileJoinerWriter {
            complete_content: self.complete_content,
            target: self.target
        })
    }
}

///
/// Write content to the given output file
///
struct FileJoinerWriter<'a> {
    target: &'a str,
    complete_content: String
}

impl<'a> FileJoinerWriter<'a> {

    pub fn new(complete_content: String, target: &'a str) -> FileJoinerWriter<'a> {
        FileJoinerWriter {
            target,
            complete_content
        }
    }

    pub fn write_output_file(self) -> io::Result<()> {
        let mut output_file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&self.target)?;
        let _ = output_file.write_all(self.complete_content.as_bytes());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::remove_file;
    use super::FileJoinerReader;

    #[test]
    fn test_read_all_files() {
        let files = vec!["test/test1.txt",
                         "test/test2.txt",
                         "test/test3.txt"];
        let file_joiner = FileJoinerReader::new(&files, "new_file.txt");
        file_joiner.read_all_files().unwrap();
        assert!(true);
    }

    #[test]
    fn test_write_output_file() {
        let files = vec!["test/test1.txt",
                         "test/test2.txt",
                         "test/test3.txt"];
        remove_file("test/new_file.txt");
        let file_joiner = FileJoinerReader::new(&files, "test/new_file.txt");
        let writer = file_joiner.read_all_files().unwrap();
        writer.write_output_file().unwrap();
        assert!(true)
    }
}