#[doc="
Counts the frequencies of words read from the standard input, and print
a sorted frequency table.
Assumptions:
"]

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

type CountTable = HashMap<String, usize>;

#[allow(dead_code)]
fn increment_word(map: &mut CountTable, word: String) {
    *map.entry(word).or_insert(0) += 1;
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

#[cfg(test)]
mod increment_word_tests {
    use super::{increment_word, CountTable};

    #[test]
    fn test_case_initial(){
        assert_eq!(1, 2);
    }

    #[test]
    fn inserts_if_empty() {
        let mut h = CountTable::new();
        increment_word(&mut h, "one".to_owned());

        assert_eq!(Some(&1), h.get("one"));
        assert_eq!(1, h.len());
    }

    #[test]
    fn increments_if_present() {
        let mut under_test = fixture();
        let mut expected   = fixture();

        increment_word(&mut under_test, "three".to_owned());
        expected.insert("three".to_owned(), 4);

        assert_eq!(expected, under_test);
    }

    #[test]
    fn insert_if_absent() {
        let mut under_test = fixture();
        let mut expected   = fixture();

        increment_word(&mut under_test, "one".to_owned());
        expected.insert("one".to_owned(), 1);

        assert_eq!(expected, under_test);
    }

    fn fixture() -> CountTable {
        let mut h = CountTable::new();
        h.insert("two".to_owned(), 2);
        h.insert("three".to_owned(), 3);

        assert_eq!(None, h.get("one"));
        assert_eq!(Some(&2), h.get("two"));
        assert_eq!(Some(&3), h.get("three"));
        assert_eq!(2, h.len());

        h
    }
}