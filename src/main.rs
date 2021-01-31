
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
