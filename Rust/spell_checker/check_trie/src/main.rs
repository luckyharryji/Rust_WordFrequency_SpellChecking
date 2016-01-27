extern crate regex;

use std::io::prelude::*;
use std::fs::File;
use regex::Regex;
use std::env;
use std::collections::HashMap;


// struct Trie {
//     eow: bool,
//     val: char,
//     chd: Vec<Trie>
// }

// impl Trie {
//     fn new(eow: bool, val: char, chd: Vec<Trie>) -> Trie {
//         Trie{eow: eow, val: val, chd: chd}
//     }

//     fn push_word(&mut self, word: &String) {
//         let mut trie = self;
//         for c in word.chars(){
//             if trie.chd.last().map_or(true, |t| t.val != c) {
//                 trie.chd.push(Trie::new(false, c, vec![]))
//             }

//             let tmp = trie; // *
//             trie = tmp.chd.last_mut().unwrap();
//         }

//         trie.eow = true;
//     }                           
// }



// struct Trie<K, V> where K:Eq+Hash+Clone, V:Clone{
// 	key: Option<V>,
// 	suffix: HashMap<K, Trie<K,V>>, 
// }

// impl <K,V> Trie<K, V> where K:Eq+Hash+Clone, V:Clone{
// 	fn new()->Trie<K,V>{
// 		Trie{
// 			value: None,
// 			suffix:HashMap::new(),
// 		}
// 	}
// }

// struct Trie<K, V> where K: Eq+Hash+Clone, V: Clone {
//     value: Option<V>,
//     children: HashMap<K, Trie<K, V>>,
// }

// impl<K, V> Trie<K,V> where K: Eq+Hash+Clone, V: Clone {
//     fn new() -> Trie<K, V> {
//         Trie {
//             value: None,
//             children: HashMap::new(),
//         }
//     }

//     fn insert(&mut self, path: Vec<K>, v: V) {
//         if path.is_empty() {
//             self.children.entry("$").or_insert(Trie::new());
//             return;
//         }

//         self.children.entry(path[0].clone())
//             .or_insert(Trie::new())
//             .insert(path[1..].to_vec(), v)
//     }

//     fn fetch(&self, path: Vec<K>) -> Option<V> {
//         match path.len() {
//             0 => self.value.clone(),
//             _ => self.children.get(&path[0])
//                     .unwrap()
//                     .fetch(path[1..].to_vec())
//         }
//     }
// }

// END = '$'

// def make_trie(words):
//     trie = {}
//     for word in words:
//         t = trie
//         for c in word:
//             if c not in t: t[c] = {}
//             t = t[c]
//         t[END] = {}
//     return trie


fn train(words:&Vec<Vec<u8>>)->HashMap<Vec<u8>,i64>{
	let mut model = HashMap::<Vec<u8>,i64>::new();
	for word in words.iter(){
		let count = model.entry(word.to_owned()).or_insert(1);
		*count += 1;
	}
	model
}


fn main(){
	let args: Vec<_> = env::args().collect();
	if args.len()!= 2{
		panic!("Error with the name of the training file.");
	}
	let mut content = String::new();
	let mut file = File::open(&args[1]).expect("Error");
	
	file.read_to_string(&mut content).unwrap();

	let re = Regex::new(r"([a-z]+)").unwrap();
	let mut words = Vec::<Vec<u8>>::new();
	for match_word in re.captures_iter(&content){
		let word = match_word.at(1).unwrap();
		words.push(word.as_bytes().to_owned());
	}

	let model = train(&words);
}