

struct DirReader<'a> {
    path: &'a str,
    recursive: bool
}

impl<'a> DirReader<'a> {

    pub fn new(path: &'a str, recursive: bool) -> DirReader<'a> {
        DirReader {
            path,
            recursive
        }
    }

    pub fn list_files(&self) -> Vec<&'a str>{
        vec![""]
    }

}


#[cfg(test)]
mod tests {
    use std::fs::remove_file;
    use super::DirReader;

    #[test]
    fn test_list_files() {
        let dir_reader = DirReader::new("", false);
        remove_file("test/new_file.txt");
        let len = dir_reader.list_files().len();
        assert_eq!(3, len);
    }

}