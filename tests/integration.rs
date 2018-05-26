extern crate assert_cli;

#[cfg(test)]
mod integration {
  use assert_cli::Assert;
  use std::fs::remove_file;

  const NO_ARGS_PROVIDED: &str =
    "The following required arguments were not provided";
  const FILE_EXISTS: &str = "Problem occurred: entity already exists";

  #[test]
  fn calling_file_join_without_args() {
    Assert::main_binary()
      .fails_with(1)
      .and()
      .stderr()
      .contains(NO_ARGS_PROVIDED)
      .unwrap();
  }

  #[test]
  fn calling_file_join_with_args() {
    remove_file("test/new_file.txt");
    Assert::main_binary()
      .with_args(&["test", "test/new_file.txt"])
      .succeeds()
      .unwrap();
  }

  #[test]
  fn calling_file_join_without_overwrite_when_file_exists() {
    Assert::main_binary()
      .with_args(&["test", "new_file.txt"])
      .fails_with(1)
      .and()
      .stderr()
      .contains(FILE_EXISTS)
      .unwrap();
  }
}
