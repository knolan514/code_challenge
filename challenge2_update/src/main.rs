extern crate regex;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::env;
use std::error::Error;
use std::str;
use regex::Regex;

fn main() {
    // gets arguments from command line
    let args: Vec<String> = env::args().collect();
    // makes sure there is one argument
    match args.len() {
        // one argument was passed
        2 => {}
        // no arguments were passed and an error appears
        _ => {
            panic!("ERROR: Must take one argument");
        }
    };
    // creates path with argument that was passed
    let path = Path::new(&args[1]);
    // opens path
    let mut file = match File::open(&path) {
        // describes error if there is one
        Err(why) => panic!("{}", why.description()),
        Ok(file) => file,
    };
    // gets file content and puts it into a string
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("{}", why.description()),
        Ok(_) => {},
    }
    // removes characters other than letters ans whitespace
    let re = Regex::new(r"[^\w\n]").unwrap();
    let s = re.replace_all(&s, "");
    // turns string into a vector and splits each line
    let mut vec: Vec<&str> = s.split('\n').collect();
    let l = vec.len()-1;
    vec.truncate(l);
    for phrase in vec.iter() {
        // changes all letters to lowercase
        let phrase = phrase.to_lowercase();
        // reverse the string in the list
        let mut reverse: String = phrase.chars().rev().collect();
           if phrase == reverse.to_lowercase() {
               print!("AY | ");
           } else {
               print!("NAY | ");
           }
           unsafe {
               let reverse = reverse.as_mut_vec();
               // sorts in reverse alphabetical order
               reverse.sort_by(|a, b| b.cmp(a));
               println!("{}", str::from_utf8(reverse).unwrap());
           }
    }
}
