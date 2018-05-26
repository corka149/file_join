use std::io;
use std::path::{Path, PathBuf};

/// Reader of directory to get a list of files.
pub struct DirReader<'a> {
  path: &'a str,
  recursive: bool,
}

impl<'a> DirReader<'a> {
  pub fn new(path: &'a str, recursive: bool) -> DirReader<'a> {
    DirReader { path, recursive }
  }

  /// List all files of a given directory and if set to true it will do it for every subfolder
  /// and their subfolders.
  pub fn list_files(&self) -> io::Result<Vec<String>> {
    let path = Path::new(&self.path);
    let mut file_paths: Vec<String> = Vec::new();
    self.read_dir(path, &mut file_paths)?;
    Ok(file_paths)
  }

  /// Reads the content of a file and add all paths of found files to a vec.
  /// When the file reader should search recursive, it will do so.
  ///
  /// Errors
  /// --
  /// The following errors could be thrown:
  /// - Path does not exists
  /// - Not the permissions for viewing content
  /// - Path is not a directory
  fn read_dir(
    &self,
    path: &Path,
    file_paths: &mut Vec<String>,
  ) -> io::Result<()> {
    for entry in path.read_dir()? {
      let entry = entry?;

      if entry.path().is_file() {
        let entry_path = entry.path();
        DirReader::add_extracted_path_string(entry_path, file_paths);
      } else if self.recursive && entry.path().is_dir() {
        let read_result = self.read_dir(entry.path().as_path(), file_paths);
        if let Err(e) = read_result {
          eprintln!("{}", e);
        };
      }
    }

    Ok(())
  }

  /// Helps to transform PathBuf to string for easier handling and adds it to a given vector
  fn add_extracted_path_string(
    path_buf: PathBuf,
    file_paths: &mut Vec<String>,
  ) {
    let path = path_buf.into_os_string();
    if let Ok(value) = path.into_string() {
      file_paths.push(value);
    }
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
    assert_eq!(4, dir_reader.list_files().unwrap().len());
  }

  #[test]
  fn test_list_recursive_files() {
    let dir_reader = DirReader::new("test", true);
    remove_file("test/new_file.txt");
    assert_eq!(6, dir_reader.list_files().unwrap().len());
  }

  #[test]
  #[should_panic(expected = "No such file or directory")]
  fn test_list_files_of_not_existing_dir() {
    let dir_reader = DirReader::new("does_not_exist", false);
    dir_reader.list_files().unwrap();
  }
}
