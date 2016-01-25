extern crate regex;

use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use std::collections::HashMap;
use std::env;
use std::io::stdin;
use std::io::BufReader;
// use std::io::{BufRead,BufReader,Read,stdin};


fn train(words:&Vec<Vec<u8>>)->HashMap<Vec<u8>,i64>{
	let mut model = HashMap::<Vec<u8>,i64>::new();
	for word in words.iter(){
		let count = model.entry(word.to_owned()).or_insert(1);
		*count += 1;
	}
	model
}


fn edit1(word:&Vec<u8>)-> Vec<Vec<u8>>{
	let alphabet = "abcdefghijklmnopqrstuvwxyz";
	let mut result = Vec::<Vec<u8>>::new();

	delete(&word, &mut result);
	replace(&word, &mut result, &alphabet);
	inserts(&word, &mut result, &alphabet);
	transpose(&word,&mut result);

	remove_duplicate(&mut result);

	result
}


fn delete(word:&Vec<u8>,result:&mut Vec<Vec<u8>>){
	for i in 0..word.len(){
		let mut delete_one = Vec::<u8>::new();
		for (itr,c) in word.iter().enumerate(){
			if itr != i{
				let ch = c.to_owned();
				delete_one.push(ch);
			}
		}
		result.push(delete_one);
	}
}


fn replace(word: &Vec<u8>, result: &mut Vec<Vec<u8>>, alphabet:&str){
	for i in 0..word.len(){
		for alpha in alphabet.chars(){
			let mut replace_word = Vec::<u8>::new();
			for (itr, c) in word.iter().enumerate(){
				if itr!= i{
					replace_word.push(c.to_owned());
				}
				else{
					replace_word.push(alpha as u8);
				}
			}
			result.push(replace_word);
		}
	}
}


fn inserts(word:&Vec<u8>,result:&mut Vec<Vec<u8>>, alphabet:&str){
	for i in 0..word.len(){
		for alpha in alphabet.chars(){
			let mut ins = Vec::<u8>::new();
			for (iter, c) in word.iter().enumerate(){
				if iter == i{
					ins.push(alpha as u8);
				}
				ins.push(c.to_owned());
			}
		}
	}

	for alpha in alphabet.chars(){
		let mut ins = Vec::<u8>::new();
		for c in word.iter(){
			ins.push(c.to_owned());
		}
		ins.push(alpha as u8);
		result.push(ins);
	}
}


fn transpose(word:&Vec<u8>, result:&mut Vec<Vec<u8>>){
	for i in 0..(word.len()-1){
		let mut trans_word = word.to_owned();
		let temp = trans_word[i];
		trans_word[i] = trans_word[i+1];
		trans_word[i+1] = temp;
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
	for candidate in candidates.iter(){
		let temp_count = model.get(candidate).unwrap().to_owned();
		if temp_count>count{
			count = temp_count;
			result = candidate.to_owned();
		}
	}
	result
}


fn edit2(word:&Vec<u8>,model:&HashMap<Vec<u8>,i64>)->Vec<Vec<u8>>{
	let mut result = Vec::<Vec<u8>>::new();
	let _one_edit = edit1(&word);
	for word_vector in _one_edit.iter(){
		let _two_edit = edit1(&word_vector);
		for two_word_vector in _two_edit.iter(){
			if model.contains_key(&two_word_vector[..]){
				result.push(two_word_vector.to_owned());
			}
		}
	}
	remove_duplicate(&mut result);
	result
}

fn correct(word:&str, model:&HashMap<Vec<u8>, i64>) -> Vec<u8>{
	let vector_word:Vec<u8> = word.as_bytes().to_owned();

	if model.contains_key(&vector_word[..]){
		return vector_word
	}

	let mut _one_edit = edit1(&vector_word);
	parse_candidate(&mut _one_edit,&model);
	if _one_edit.len()>0{
		select_candidate(&_one_edit,model);
	}
	let _two_edit = edit2(&vector_word,&model);
	if _two_edit.len()>0{
		select_candidate(&_two_edit,model);
	}
	vector_word
}


fn print_vector_word(vector_word:&Vec<u8>){
	let mut print_str = String::from("");
	for alpha in vector_word.iter(){
		// println!("{} ", alpha.to_owned() as char);
		// print!(" ");
		// print_str += alpha.to_owned() as char;
		print_str.push(alpha.to_owned() as char);
	}
	println!("{}",print_str);
}


fn main(){
	let mut file = File::open("test.txt").unwrap();
	let mut content = String::new();
	file.read_to_string(&mut content).unwrap();

	let re = Regex::new(r"([a-z]+)").unwrap();
	let mut words = Vec::<Vec<u8>>::new();
	for match_word in re.captures_iter(&content){
		let word = match_word.at(1).unwrap();
		// println!("{}", word);
		// println!("{}", match_word.at().unwrap());
		// println!("{}", word.as_bytes().to_owned());
		words.push(word.as_bytes().to_owned());
	}

	let model = train(&words);

	read_line_of_input(stdin(),&model);
	// for (iter,arg) in env::args().enumerate(){
	// 	let correction = correct(&arg,&model);
	// 	print_vector_word(&correction);

	// }
	// for (word,count) in &model{
	// 	println!("{}: \"{}\"", word,count);
	// }
}


fn read_line_of_input<R:Read>(reader:R, model:&HashMap<Vec<u8>,i64>){
	let mut lines = BufReader::new(reader).lines();
	while let Some(Ok(line)) = lines.next(){
		let correction = correct(&line,&model);
		print_vector_word(&correction);
	}

}