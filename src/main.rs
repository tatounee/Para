
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

    let matches =  App::new("Paranagram")
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .bin_name("paranagram")
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
            .help("Use if you only want the word witch can be create from your sentence")
            .short("j")
            .long("justwords"))
        .arg(Arg::with_name("DEBUG")
            .help("Prints advencement of anagrams's generation")
            .short("d")
            .long("debug")
            .possible_values(&["true", "false"]))
        .get_matches();
