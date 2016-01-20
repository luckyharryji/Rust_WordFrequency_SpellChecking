extern crate itertools;


use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use itertools::Itertools;

fn main(){
    read_file_and_count("test.txt");
}

fn read_file_and_count(file_name:&str) -> std::io::Result<()>{
	let mut f = try!(File::open(file_name));
	let mut s = String::new();
	try!(f.read_to_string(&mut s));
	println!("String in the file is : {}", s);
	print!("{}", get_freqs(s));
	Ok(())
}

fn word_freq<F>(s: String, sf: F) -> HashMap<String, u32>
    where F : Fn(char) -> bool {

        s.split(sf)
            .filter( |s| !s.is_empty() )
            .map(|s| { s.chars().flat_map(char::to_lowercase).collect::<String>() })
            .fold(HashMap::new(), |mut m, i| {
                *m.entry(i).or_insert(0u32) += 1;
                m
            })
    }

fn word_freq_count(s: String) -> HashMap<String, u32> {
    word_freq(s, |c: char| !c.is_alphabetic())
}

// Preparing for output
fn sort_by_freq(c: HashMap<String, u32>) -> String {
    c.iter()
        .sort_by(|a, b| Ord::cmp(&b.1, &a.1))
        .iter()
        .map(|&(k, v)| format!("{} {}", k, v))
        .fold("".to_string(), |i, v| format!("{}{}\n", i, v))
} 

pub fn get_freqs(s: String) -> String {
    let count = word_freq_count(s);
    sort_by_freq(count)
}
