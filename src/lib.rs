extern crate clap;
extern crate regex;
#[macro_use]
extern crate human_panic;

pub mod filter;
pub mod join;
pub mod dir_reader;

pub mod bootstrap {
    use clap::{Arg, App, ArgMatches};
    use filter::FileFilter;
    use join::{FileJoinerReader};
    use dir_reader::DirReader;


    struct ExtractedArgs {
        source_dir: String,
        output_file: String,
        patterns: Vec<String>,
        recursive: bool
    }

    pub fn run() {
        setup_panic!();

        let matches = App:: new("A programm for merging files with ease\n")
            .version("0.2")
            .author("corka149 <corka149@mailbox.org")
            .arg(Arg::with_name("dir")
                .help("Folder of wanted files")
                .required(true)
                .index(1))
            .arg(Arg::with_name("out")
                .help("Specifies the output file")
                .required(true)
                .index(2))
            .arg(Arg::with_name("filter")
                .short("F")
                .long("filter")
                .help("Pre-filter for joining with RegEx. Multiple filter are possible. \
                    They are applied in an AND-conjunction. By default nothing will be filtered.")
                .required(false)
                .multiple(true)
                .takes_value(true))
            .arg(Arg::with_name("recursive")
                .short("R")
                .long("recursive")
                .takes_value(false)
                .required(false)
                .help("Gather files recursively"))
            .get_matches();

        let extracted_args = extract_args_from_matches(matches);

        run_join(extracted_args);
    }

    fn run_join(extracted_args: ExtractedArgs) {
        // Get dir content
        let dir_reader = DirReader::new(&extracted_args.source_dir, extracted_args.recursive);
        let files = match dir_reader.list_files() {
            Ok(f) => f,
            Err(e) => panic!(e)
        };

        // Filter files
        let str_files = to_str_vec(&files);
        let str_patterns = to_str_vec(&extracted_args.patterns);
        let file_filter = FileFilter::new( &str_files, &str_patterns);
        let file_filter = match file_filter {
            Ok(ff) => ff,
            Err(e) => panic!(e)
        };
        let filtered_files = file_filter.apply_patterns();

        // Join file
        let file_join_reader = FileJoinerReader::new(&filtered_files, &extracted_args.output_file);
        let read_result = file_join_reader.read_all_files();
        let file_join_writer = match read_result {
            Ok(fjw) => fjw,
            Err(e) => panic!(e)
        };
        match file_join_writer.write_output_file() {
            Ok(()) => println!("Finished joining!"),
            Err(e) => panic!(e)
        }
    }

    fn extract_args_from_matches(arg_matches: ArgMatches) -> ExtractedArgs {
        let source_dir = arg_matches.value_of("dir").expect("No source dir provided!");
        let output_file = arg_matches.value_of("out").unwrap_or("new_file");
        let patterns = arg_matches.values_of_lossy("filter").unwrap_or(vec![String::from("")]);
        let recursive = arg_matches.is_present("recursive");

        ExtractedArgs{
            source_dir: String::from(source_dir),
            output_file: String::from(output_file),
            patterns,
            recursive
        }
    }

    fn to_str_vec<'a>(string_vec: &'a [String]) -> Vec<&'a str> {
        let mut vec_with_str: Vec<&'a str> = Vec::new();
        for it in string_vec {
            vec_with_str.push(it.as_str());
        }

        vec_with_str
    }

}
