use std::io;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

/// Reader of directory to get a list of files.
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

    /// List all files of a given directory and if set to true it will do it for every subfolder
    /// and their subfolders.
    pub fn list_files(&self) -> io::Result<Vec<PathBuf>>{
        let path = Path::new(&self.path);
        let mut file_paths: Vec<PathBuf> = Vec::new();
        self.read_dir(path, &mut file_paths)?;
        Ok(file_paths)
    }

    /// Reads the content of a file and add all paths of found files to a vec.
    /// When the file reader should search recursive, it will do so.
    fn read_dir(&self, path: &Path, file_paths: &mut Vec<PathBuf>) -> io::Result<()> {
        if path.is_dir() {
            for entry in path.read_dir()? {
                let entry = entry?;

                if entry.path().is_file() {
                    file_paths.push(entry.path());
                }
                else if self.recursive && entry.path().is_dir() {
                    self.read_dir(entry.path().as_path(), file_paths);
                }
            }
        }

        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use std::fs::remove_file;
    use super::DirReader;

    #[test]
    fn test_list_files() {
        let dir_reader = DirReader::new("test", false);
        remove_file("test/new_file.txt");
        assert_eq!(3, dir_reader.list_files().unwrap().len());
    }

}