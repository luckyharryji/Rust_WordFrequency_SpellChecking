#[doc="
	__author__ = 'Xiangyu'

	Spell Checking
	Correct the typo for the english word. The program first train a language model to define the 
	frequency of the corrent of the word inside a formal corpus, as the groung truth. Then the pro-
	gram deal with differnt input string to correct it to the write word with the largest possi-
	bility. The right word is assumed to be one/two edit different form the origin input.

	Reference:
	https://www.reddit.com/r/rust
	https://github.com/jameswhang/Rust_Programs/blob/master/freq/src/main.rs

	INPUT:
	The first input is the name of the corpus file.
	The second input is the lines of the words waiting to be corrected, one word per line.

	OUTPUT:
	The program would correct each input as the most likely right word. One word per line.
		- If the word is correct, print the origin word, i.e:
			hello
		- If the word can be corrected, print the origin word and the corrected word in the format:
			hell, hello
		- if the right word can not be got from corpus, print the origin word in the format:
			w, -

	Assumptions:
	  - The input file name is correct.
	  - The correct word is one/two edit distance from the origin word.
	  - If there is nothing in the file, program would start a new line to wait for the new coming input.
	  - The txt contain the origin words is transported to the program in Linux way of < .
"]




extern crate regex;

use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::io::stdin;
use std::io::BufReader;


fn train(words:&Vec<Vec<u8>>)->HashMap<Vec<u8>,i64>{
	let mut model = HashMap::new();
	for word in words{
		let count = model.entry(word.to_owned()).or_insert(1);
		*count += 1;
	}
	model
}


static ALPHABET : &'static str = "abcdefghijklmnopqrstuvwxyz";

fn edit1(word:&Vec<u8>)-> Vec<Vec<u8>>{
	let mut result = Vec::<Vec<u8>>::new();

	count_delete(word, &mut result);
	count_replace(word, &mut result);
	count_inserts(word, &mut result);
	count_transpose(word,&mut result);

	remove_duplicate(&mut result);

	result
}


fn count_delete(word:&Vec<u8>,result:&mut Vec<Vec<u8>>){
	for i in 0..word.len(){
		let mut delete_one = Vec::<u8>::new();
		for (index,alpha_num) in word.iter().enumerate(){
			if index != i{
				let ch = alpha_num.to_owned();
				delete_one.push(ch);
			}
		}
		result.push(delete_one);
	}
}


fn count_replace(word: &Vec<u8>, result: &mut Vec<Vec<u8>>){
	for i in 0..word.len(){
		for alpha in ALPHABET.chars(){
			let mut replace_word = Vec::<u8>::new();
			for (index, alpha_num) in word.iter().enumerate(){
				if index!= i{
					replace_word.push(alpha_num.to_owned());
				}
				else{
					replace_word.push(alpha as u8);
				}
			}
			result.push(replace_word);
		}
	}
}


fn count_inserts(word:&Vec<u8>,result:&mut Vec<Vec<u8>>){
	for i in 0..word.len(){
		for alpha in ALPHABET.chars(){
			let mut ins = Vec::<u8>::new();
			for (index, alpha_num) in word.iter().enumerate(){
				if index == i{
					ins.push(alpha as u8);
				}
				ins.push(alpha_num.to_owned());
			}
			result.push(ins);
		}
	}

	for alpha in ALPHABET.chars(){
		let mut ins = Vec::<u8>::new();
		for alpha_num in word.iter(){
			ins.push(alpha_num.to_owned());
		}
		ins.push(alpha as u8);
		result.push(ins);
	}
}


fn count_transpose(word:&Vec<u8>, result:&mut Vec<Vec<u8>>){
	for i in 0..(word.len()-1){
		let mut trans_word = word.to_owned();
		trans_word.swap(i,i+1);
		result.push(trans_word);
	}
}


fn remove_duplicate(words: &mut Vec<Vec<u8>>){
	words.sort();
	words.dedup();
}


fn parse_candidate(words:&mut Vec<Vec<u8>>, model:&HashMap<Vec<u8>,i64>){
	words.retain(|word| model.contains_key(&word[..]));
}


fn select_candidate(candidates: &Vec<Vec<u8>>, model:&HashMap<Vec<u8>,i64>)->Vec<u8>{
	let mut count = 0;
	let mut result = Vec::<u8>::new();
	for candidate in candidates{
		let temp_count = model.get(candidate).unwrap().to_owned();
		if temp_count>count{
			count = temp_count;
			result = candidate.to_owned();
		}
	}
	result
}


fn edit2(word:&Vec<u8>)->Vec<Vec<u8>>{
	let mut result = Vec::<Vec<u8>>::new();
	let _one_edit = edit1(word);
	for word_vector in _one_edit.iter(){
		let _two_edit = edit1(&word_vector);
		for two_word_vector in _two_edit.iter(){
			result.push(two_word_vector.to_owned());
		}
	}
	remove_duplicate(&mut result);
	return result
}


fn correct(word:&str, model:&HashMap<Vec<u8>, i64>){
	let vector_word:Vec<u8> = word.as_bytes().to_owned();

	if model.contains_key(&vector_word[..]){
		print_vector_word(&vector_word); // format input when find origin word
		print!("\n");
		return
	}

	let mut _one_edit = edit1(&vector_word);
	parse_candidate(&mut _one_edit,&model);
	if _one_edit.len()>0{
		print_vector_word(&vector_word); // format input when one edit different
		print!(", ");
		print_vector_word(&select_candidate(&_one_edit,model));
		print!("\n");
		return 
	}
	
	let mut _two_edit = edit2(&vector_word);
	parse_candidate(&mut _two_edit,&model);
	if _two_edit.len()>0{
		print_vector_word(&vector_word); // format input when two edit different
		print!(", ");
		print_vector_word(&select_candidate(&_two_edit,model));
		print!("\n");
		return 
	}

	print_vector_word(&vector_word);
	print!(", -\n");
}


fn print_vector_word(vector_word:&Vec<u8>){
	let mut print_str = String::from("");
	for alpha in vector_word.iter(){
		print_str.push(alpha.to_owned() as char);
	}
	print!("{}",print_str);
}


fn find_english_word(content:&str)->Vec<Vec<u8>>{
	let re = Regex::new(r"([a-z]+)").unwrap();
	let mut words = Vec::<Vec<u8>>::new();
	for match_word in re.captures_iter(&content){
		let word = match_word.at(1).unwrap();
		words.push(word.to_lowercase().as_bytes().to_owned());
	}
	words
}


fn read_line_of_input<R:Read>(reader:R, model:&HashMap<Vec<u8>,i64>){
	let mut lines = BufReader::new(reader).lines();
	while let Some(Ok(line)) = lines.next(){
		correct(&line, &model);
	}

}


fn main(){
	let args: Vec<_> = env::args().collect();
	if args.len()!= 2{
		panic!("Error with the name of the training file.");
	}
	let mut content = String::new();
	let mut file = File::open(&args[1]).expect("Error");
	
	file.read_to_string(&mut content).unwrap();

	let words = find_english_word(&content);
	let model = train(&words);

	println!("Trainging end");

	read_line_of_input(stdin(),&model);
}



#[cfg(test)]
mod test_for_spell_checking{
	use super::{find_english_word,edit1,edit2};

	#[test]
	fn regex_match_test(){
		let test_tring = "hello".as_bytes().to_owned();
		let test_tring2 = "world".as_bytes().to_owned();
		assert!(find_english_word("hello+").contains(&test_tring));
		assert!(find_english_word("world!!!").contains(&test_tring2));
		assert!(find_english_word("hello world!!! ").contains(&test_tring2));
		assert!(find_english_word("hello world!!! ").contains(&test_tring));
	}


	#[test]
	fn edit1_test(){
		let set_of_edit_one = edit1(&"hello".as_bytes().to_owned());
		assert!(set_of_edit_one.contains(&"hell".as_bytes().to_owned()));
		assert!(set_of_edit_one.contains(&"helloo".as_bytes().to_owned()));
		assert!(set_of_edit_one.contains(&"helo".as_bytes().to_owned()));
		assert!(set_of_edit_one.contains(&"helle".as_bytes().to_owned()));
		assert!(set_of_edit_one.contains(&"heslo".as_bytes().to_owned()));
		assert!(set_of_edit_one.contains(&"helol".as_bytes().to_owned()));

		assert!(!set_of_edit_one.contains(&"hellooo".as_bytes().to_owned()));
		assert!(!set_of_edit_one.contains(&"heo".as_bytes().to_owned()));
		assert!(!set_of_edit_one.contains(&"hablo".as_bytes().to_owned()));
		assert!(!set_of_edit_one.contains(&"hleol;".as_bytes().to_owned()));
	}


	#[test]
	fn edit2_test(){
		let set_of_edit_two = edit2(&"hello".as_bytes().to_owned());
		assert!(set_of_edit_two.contains(&"hellooo".as_bytes().to_owned()));
		assert!(set_of_edit_two.contains(&"heo".as_bytes().to_owned()));
		assert!(set_of_edit_two.contains(&"hablo".as_bytes().to_owned()));
	}
}

#[cfg(test)]
mod test_for_module_function{
	use super::{count_delete, count_replace, count_inserts, count_transpose};

	#[test]
	fn test_delete_char(){
		let mut set = Vec::<Vec<u8>>::new();
		assert!(!set.contains(&"hell".as_bytes().to_owned()));
		count_delete(&"hello".as_bytes().to_owned(),&mut set);
		assert!(set.contains(&"hell".as_bytes().to_owned()));
	}

	#[test]
	fn test_insert_char(){
		let mut set = Vec::<Vec<u8>>::new();
		assert!(!set.contains(&"helloa".as_bytes().to_owned()));
		count_inserts(&"hello".as_bytes().to_owned(),&mut set);
		assert!(set.contains(&"helloa".as_bytes().to_owned()));
	}

	#[test]
	fn test_replace_char(){
		let mut set = Vec::<Vec<u8>>::new();
		assert!(!set.contains(&"hella".as_bytes().to_owned()));
		count_replace(&"hello".as_bytes().to_owned(),&mut set);
		assert!(set.contains(&"hella".as_bytes().to_owned()));
	}

	#[test]
	fn test_transpose_char(){
		let mut set = Vec::<Vec<u8>>::new();
		assert!(!set.contains(&"helol".as_bytes().to_owned()));
		count_transpose(&"hello".as_bytes().to_owned(),&mut set);
		assert!(set.contains(&"helol".as_bytes().to_owned()));
	}

}