#[doc="
author = '__Xiangyu__'

Word Frequency

Counts the frequencies of words read from the file, file name got through standard input, and print
a sorted frequency table.

INPUT:

The input format is the file name, one execution perline

Each time give an input, the program count and print the word frequency inside the input file.

The program terminates with the input of EOF

OUTPUT:

The program counts the word frequency inside the input file, the number is considered invalid and will
be considerd the space between the words. It prints the results in this format:

    world: 2
    bye: 1
    test: 1
    hello: 1
    wor: 1
    ld: 1

Assumptions:

  - Numeriacal numbers are considered as the space between words, is means if meet a string like 'wor3ld',
  the program will consider it as 2 words 'wor' and 'ld'.

  - Input does not need to be perfect, the programs controls through io::File.

  - If there is nothing in the file, program would start a new line to wait for the new coming input.

  - The termination of the program is the Input with the string text 'EOF', not a line/word inside text that 
  is 'EOF'.
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
        print_word_frequency_of_file(line);
    }
}

fn print_word_frequency_of_file(file_name:String)-> std::io::Result<()>{
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
                .map(|&(key, value)| format!("{}: {}", key, value))
                .fold("".to_string(), |i, v| format!("{}{}\n", i, v))
    }
}


#[cfg(test)]
mod sort_count_table_tests {
    use super::{sort_by_freq, CountTable};

    #[test]
    fn change_line_if_empty() {
        let h = CountTable::new();
        let sort_table_string = sort_by_freq(h);

        assert_eq!("\n", sort_table_string);
    }

    #[test]
    fn sort_descending_order_if_valid(){
        let h = table_initial();
        let sort_table_string = sort_by_freq(h);
        
        assert_eq!("three: 3\ntwo: 2\n", sort_table_string);
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