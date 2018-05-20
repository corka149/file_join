# file_join

A simple and fast cli-tool to join multiple files.

```bash
corka@my-pc: file_join

error: The following required arguments were not provided:
    --dir <DIR>

USAGE:
    main [OPTIONS] --dir <DIR>

For more information try --help
```

## Todos

- Replace panics as much as possible
  - When listing files (list_files)
  - When filtering files (match file_filter)
  - When reading result (read_result)
  - When writing back to file (write_output_file)
- Run clippy
- More tests
- Multithreading for reading dirs and files