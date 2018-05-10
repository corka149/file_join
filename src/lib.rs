extern crate clap;
extern crate regex;

pub mod filter;
pub mod join;
pub mod dir_reader;

pub mod bootstrap {
    use clap::{Arg, App};

    pub fn run() {
        let matches = App:: new("A programm for merging files with ease\n")
            .version("0.1")
            .author("corka149 <corka149@mailbox.org")
            .arg(Arg::with_name("dir")
                .short("D")
                .long("dir")
                .value_name("DIR")
                .help("Current folder of wanted files")
                .required(true)
                .takes_value(true))
            .arg(Arg::with_name("filter")
                .short("F")
                .long("filter")
                .help("Pre-filter for joining with RegEx. Multiple filter are possible. \
                    They are applied in an AND-conjunction.")
                .required(false)
                .multiple(true)
                .takes_value(true))
            .get_matches();

        if let Some(v) = matches.value_of("dir") {
            println!("Your folder: {}", v);
        }
    }

}
