#[doc="
__author__ = 'Xiangyu'

Word Frequency

Counts the frequencies of words read from the file, file name got through standard input, and print
a sorted frequency table.

INPUT:

The input format is the content inside the file by Linux input way of <, line by line read.
Each time give an input, the program count and print the word frequency inside the input file.


OUTPUT:

The program counts the word frequency inside the input file. It prints the results in this format, one word per line, following the frequency:

    world: 2
    bye: 1
    test: 1
    hello: 1
    wor: 1
    ld: 1

Assumptions:

  - Char which is not alphabetic will be parse and deleted.

  - If there is nothing in the file, program would start a new line to wait for the new coming input.

  - words will be treated lowercase to merge the same word case

"]

extern crate itertools;


use std::io::prelude::*;
use std::collections::HashMap;
use itertools::Itertools;
use std::io::BufReader;
use std::io::stdin;

fn main(){
    get_input_file_name(stdin());
}


fn get_input_file_name<R: Read>(reader: R){
    let mut lines = BufReader::new(reader).lines();
    let mut count_table = CountTable::new();

    while let Some(Ok(line)) = lines.next() {
        let input_line = line.to_owned();
        let words: Vec<&str> = input_line.split_whitespace().collect();
        parse_alpha(&mut count_table, words);
    }
    let format_string = sort_by_freq(count_table);
    print!("{}", format_string);
}


fn parse_alpha(count_table:&mut CountTable, input: Vec<&str>){
    for word in input{
        let to_low = word.to_lowercase();
        let mut count_string = String::from("");
        for al in to_low.chars(){
            if al.is_alphabetic(){
                count_string.push(al);
            }
        }
        increment_word(count_table,count_string);
    }
}

type CountTable = HashMap<String, usize>;

#[allow(dead_code)]
fn increment_word(map: &mut CountTable, word: String) {
    *map.entry(word).or_insert(0) += 1;
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