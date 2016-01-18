
// use std::io::{BufRead,BufReader,Read,stdin};
extern crate itertools;


use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use itertools::Itertools;




pub enum Sorted {
    No,
    Alpha,
    Freq
}

fn main(){
    foo();
    read_file("test.txt");
}

fn read_file(file_name:&str) -> std::io::Result<()>{
	let mut f = try!(File::open(file_name));
	let mut s = String::new();
	try!(f.read_to_string(&mut s));
	println!("Sting in the file is : {}", s);
	print!("{}", get_freqs(s, true, Sorted::Freq));
	Ok(())
}

fn foo() -> std::io::Result<()> {
	let mut f = try!(File::create("foo.txt"));
	try!(f.write_all(b"Hello, world!"));

	let mut f = try!(File::open("foo.txt"));
	let mut s = String::new();
	try!(f.read_to_string(&mut s));
	Ok(())
}



// The frequency counters
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

fn word_freq_nums(s: String) -> HashMap<String, u32> {
    word_freq(s, |c: char| !c.is_alphanumeric())
}

fn word_freq_no_nums(s: String) -> HashMap<String, u32> {
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

fn sort_by_alpha(c: HashMap<String, u32>) -> String {
    let mut arr: Vec<String> = c.iter().map(|(k, v)| format!("{} {}", k, v)).collect();
    arr.sort();
    arr.iter().fold("".to_string(), |i, v| i + v + "\n")
}

fn no_sort(c: HashMap<String, u32>) -> String {
    c.iter().map(|(k, v)| format!("{} {}", k, v)).fold("".to_string(), |i, v| format!("{}{}\n", i, v))
}

// The main dispatch function
pub fn get_freqs(s: String, nums: bool, sort: Sorted) -> String {
    println!("{}, {}", s,nums);
    let count = match nums {
        true  => word_freq_nums(s),
        false => word_freq_no_nums(s),
    };
    match sort {
        Sorted::No    => no_sort(count),
        Sorted::Alpha => sort_by_alpha(count),
        Sorted::Freq  => sort_by_freq(count),
    }
}



// fn read_measurements<R: Read>(reader: R) -> Vec<f64> {
//     let mut measurements: Vec<f64> = vec![]; // Vec::new()
//     let mut lines = BufReader::new(reader).lines();

//     while let Some(Ok(line)) = lines.next() {
//         if line == "999" {break}

//         if let Ok(f) = line.parse() {
//             if f >= 0.0 {
//                 measurements.push(f);
//             }
//         }
//     }

//     for line in &measurements{
//     	println!("{}", line);
//     }

//     return measurements;
// }