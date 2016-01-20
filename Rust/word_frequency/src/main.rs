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
use std::io::BufReader;
use std::io::stdin;

fn main(){
    get_input_file_name(stdin());
}


fn get_input_file_name<R: Read>(reader: R){
    let mut lines = BufReader::new(reader).lines();

    while let Some(Ok(line)) = lines.next() {
        if line == "EOF" {break}
        read_and_count_and_print(line);
    }
}

fn read_and_count_and_print(file_name:String)-> std::io::Result<()>{
    let f = try!(File::open(file_name));
    let mut reader = BufReader::new(f).lines();
    let mut count_table = CountTable::new();

    while let Some(Ok(line)) = reader.next(){
        word_freq(&mut count_table, line, |word: char| !word.is_alphabetic());
    }
    let format_string = sort_by_freq(count_table);
    print!("{}", format_string);
    Ok(())
}

type CountTable = HashMap<String, usize>;

#[allow(dead_code)]
fn increment_word(map: &mut CountTable, word: String) {
    *map.entry(word).or_insert(0) += 1;
}

fn word_freq<F>(count_table: &mut CountTable, s: String, space: F)
    where F : Fn(char) -> bool {
        s.split(space)
            .filter( |s| !s.is_empty() )
            .map(|s| { s.chars().flat_map(char::to_lowercase).collect::<String>() })
            .fold((),|m,i| {
                increment_word(count_table,i);
            })
    }

fn sort_by_freq(count_table: CountTable) -> String {
    match count_table.len(){
        0 => String::from("\n"),
        _ => count_table.iter()
                .sort_by(|a, b| Ord::cmp(&b.1, &a.1))
                .iter()
                .map(|&(key, value)| format!("{} {}", key, value))
                .fold("".to_string(), |i, v| format!("{}{}\n", i, v))
    }
}

















// #[cfg(test)]
// mod increment_word_tests {
//     use super::{increment_word, CountTable};

//     #[test]
//     fn test_case_initial(){
//         assert_eq!(1, 1);
//     }

//     #[test]
//     fn inserts_if_empty() {
//         let mut h = CountTable::new();
//         increment_word(&mut h, "one".to_owned());

//         assert_eq!(Some(&1), h.get("one"));
//         assert_eq!(1, h.len());
//     }

//     #[test]
//     fn increments_if_present() {
//         let mut under_test = fixture();
//         let mut expected   = fixture();

//         increment_word(&mut under_test, "three".to_owned());
//         expected.insert("three".to_owned(), 4);

//         assert_eq!(expected, under_test);
//     }

//     #[test]
//     fn insert_if_absent() {
//         let mut under_test = fixture();
//         let mut expected   = fixture();

//         increment_word(&mut under_test, "one".to_owned());
//         expected.insert("one".to_owned(), 1);

//         assert_eq!(expected, under_test);
//     }

//     fn fixture() -> CountTable {
//         let mut h = CountTable::new();
//         h.insert("two".to_owned(), 2);
//         h.insert("three".to_owned(), 3);

//         assert_eq!(None, h.get("one"));
//         assert_eq!(Some(&2), h.get("two"));
//         assert_eq!(Some(&3), h.get("three"));
//         assert_eq!(2, h.len());
//         h
//     }
// }

#[cfg(test)]
mod sort_count_table_tests {
    use super::{sort_by_freq, increment_word, CountTable};

    #[test]
    fn change_line_if_empty() {
        let mut h = CountTable::new();
        let sort_table_string = sort_by_freq(h);

        assert_eq!("\n", sort_table_string);
    }

    #[test]
    fn sort_if_valid(){
        let mut h = CountTable::new();
        h.insert("one".to_owned(),1);
        h.insert("tow".to_owned(),2);

        let sort_table_string = sort_by_freq(h);
        assert_eq!("tow 2\none 1\n", sort_table_string);
    }

    fn table_initial() -> CountTable {
        let mut h = CountTable::new();
        h.insert("two".to_owned(), 2);
        h.insert("three".to_owned(), 3);

        assert_eq!(Some(&2), h.get("two"));
        assert_eq!(Some(&3), h.get("three"));
        assert_eq!(2, h.len());
        h
    }
}