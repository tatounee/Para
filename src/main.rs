
#[macro_use]
extern crate clap;

use clap::{App, Arg};
use dotenv::dotenv;
use paranagram::{Paranagram, Word};

use std::path::Path;
use std::io::Write;
use std::fs::File;
use std::process::exit;
use std::env;
fn main() {
    dotenv().ok();

    let matches =  App::new("Para")
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .after_help("If you haven't already done yet, set the path of input and output file in the .env file.\nExemple of use: \n\t$ paranagram -d -j --output=\"myoutputfile.txt\" \"Rust for ever\"\nIt will generate all words (-j) which can be created from \"Rust for ever\" and output that in the file \"myoutputfile.txt\" (--output). You will see the steps of the creation of anagrams (-d) and it's based from the input file set in the \".env\" file.")
        .bin_name("para")
        .arg(Arg::with_name("SENTENCE")
            .help("The sentence from where you want to find anagrams")
            .required(true))
        .arg(Arg::with_name("INPUT")
            .help("Set the input file which be use as a dictionary")
            .short("i")
            .long("input")
            .env("INPUT_PATH")
            .default_value("words.txt")
            .takes_value(true))
        .arg(Arg::with_name("OUTPUT")
            .help("Set the output file")
            .short("o")
            .long("output")
            .env("OUTPUT_PATH")
            .takes_value(true))
        .arg(Arg::with_name("JUSTWORDS")
            .help("Use if you only want the words which can be created from your sentence")
            .short("j")
            .long("justwords"))
        .arg(Arg::with_name("DEBUG")
            .help("Prints advancement of the generation of anagrams")
            .short("d")
            .long("debug")
            .possible_values(&["true", "false"]))
        .get_matches();

    let output = match matches.value_of("OUTPUT") {
        Some(o) => o.to_owned(),
        None => matches.value_of("SENTENCE").unwrap().to_owned() + ".txt",
    };

    let output_path: &Path = Path::new(output.as_str());

    let debug = matches.is_present("DEBUG");
    let just_words = matches.is_present("JUSTWORDS");

    let goal = if just_words { 2 } else { 4 };

    let paranagram = {
        let path = Path::new(matches.value_of("INPUT").unwrap());
        if !path.exists() {
            println!("Error: The file {} doesn't existe", path.display());
            exit(0)
        } else if path.extension().unwrap_or_default() != "txt" {
            println!("Error : The file {} must be a .txt file", path.display());
            exit(0)
        }
        if debug {
            println!("Loading {} ...", path.display())
        }
        match Paranagram::new(path) {
            Ok(p) => {
                if debug {
                    println!("[1/{}] The file {} has been successfully loaded", goal, path.display())
                }
                p
            },
            Err(e) => {
                println!("Error : {}", e);
                exit(0);
            }
        }
    };

    let buffer: String;
    let sentence = matches.value_of("SENTENCE").unwrap();
    let word = Word::new(sentence);
    if just_words {
        let words = paranagram.existing_anagrams(&word);
        buffer = words.into_iter().map(|w| {
            let mut line = format!("{} ", w);
            line.push('\n');
            line
        }).collect::<String>();
        if debug {
            println!("[2/{}] Possible anagrams found", goal);
        }
    } else {
        let anagrams = if debug {
            paranagram.generate_anagrams_debug(sentence, 1, goal)
        } else {
            paranagram.generate_anagrams(sentence)
        };
        buffer = anagrams.into_iter().map(|v| {
            let mut line = v.into_iter().map(|w| format!("{} ", w)).collect::<String>();
            line.push('\n');
            line
        }).collect::<String>();
    }
    let mut file = match File::create(output_path) {
        Ok(f) => f,
        Err(e) => {
            println!("Error : {}", e);
            exit(0);
        }
    };
    match file.write_all(&buffer.as_bytes()) {
        Err(e) => println!("Error : {}", e),
        Ok(_) => println!("Anagrams found ! Look at {}", output_path.display())
    }
}