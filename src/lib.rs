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

    const SOURCE_DIR: &'static str = "source_dir";
    const OUTPUT_FILE: &'static str = "output_file";
    const FILTER: &'static str = "filter";
    const RECURSIVE: &'static str = "recursive";
    const OVERWRITE: &'static str = "overwrite";

    struct ExtractedArgs {
        source_dir: String,
        output_file: String,
        patterns: Vec<String>,
        recursive: bool,
        overwrite: bool
    }

    pub fn run() {
        setup_panic!();

        let matches = App:: new("A programm for merging files with ease\n")
            .version("0.2")
            .author("corka149 <corka149@mailbox.org")
            .arg(Arg::with_name(SOURCE_DIR)
                .help("Folder of wanted files")
                .required(true)
                .index(1))
            .arg(Arg::with_name(OUTPUT_FILE)
                .help("Specifies the output file")
                .required(true)
                .index(2))
            .arg(Arg::with_name(FILTER)
                .short("F")
                .long(FILTER)
                .help("Pre-filter for joining with RegEx. Multiple filter are possible. \
                    They are applied in an AND-conjunction. By default nothing will be filtered.")
                .multiple(true)
                .takes_value(true))
            .arg(Arg::with_name(RECURSIVE)
                .short("R")
                .long(RECURSIVE)
                .takes_value(false)
                .help("Gather files recursively"))
            .arg(Arg::with_name(OVERWRITE)
                .short("O")
                .long(OVERWRITE)
                .takes_value(false)
                .help(""))
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
        match file_join_writer.write_output_file(extracted_args.overwrite) {
            Ok(()) => println!("Finished joining!"),
            Err(e) => panic!(e)
        }
    }

    fn extract_args_from_matches(arg_matches: ArgMatches) -> ExtractedArgs {
        let source_dir = arg_matches.value_of(SOURCE_DIR).expect("No source dir provided!");
        let output_file = arg_matches.value_of(OUTPUT_FILE).unwrap_or("new_file");
        let patterns = arg_matches.values_of_lossy(FILTER).unwrap_or(vec![String::from("")]);
        let recursive = arg_matches.is_present(RECURSIVE);
        let overwrite = arg_matches.is_present(OVERWRITE);

        ExtractedArgs{
            source_dir: String::from(source_dir),
            output_file: String::from(output_file),
            patterns,
            overwrite,
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
