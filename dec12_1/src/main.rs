use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() {
    println!("we are starting this shitshow");
    let (dot_hash, num_map) = read_input("day12_input.txt");
    //println!("{:?}", num_map);
    let answer = calc_answer(dot_hash, num_map);
    println!("Final answer is {}", answer);
}

fn calc_answer(dot_hash: Vec<String>, num_map: Vec<Vec<u32>>) -> u32 {
	let mut possible: u32 = 0;
	
	for (i, line) in num_map.iter().enumerate() {
		possible += all_possible(line.clone(), dot_hash[i].clone());					
	}
	
	possible
}

fn all_possible(num_map: Vec<u32>, dot_hash: String) -> u32{
	
	let groups: usize = num_map.len();
	let len: usize = dot_hash.len();
	let sum: usize = num_map.clone().iter().map(|x| *x as usize).sum::<usize>();
	let spaces: usize = len - sum;
	let mut answer: u32 = 0;
	
	//println!("{}", spaces);
	
	let it = (0..=spaces).combinations(groups);
	
	for el in it {
		let new_string = make_string(el, num_map.clone());
		if suitable(new_string, dot_hash.clone()) {
			answer += 1;
		}
	}
	//println!("{}", answer);
	
	//~ options = all_options(s, dot_hash.clone(), space);
	answer
}

fn suitable (input_init: String, dot_hash_init: String) -> bool {
	let mut input: Vec<char> = input_init.chars().collect();
	let dot_hash: Vec<char> = dot_hash_init.chars().collect();

	let in_len = input.len();
	let guide_len = dot_hash.len();
	
	if in_len < guide_len {
		for _i in 0..(guide_len - in_len) {
			input.push('.');
			
		}	
	}
	
	for i in 0..guide_len {
		if dot_hash[i] == '?' {

		}
		else {
			if input[i] == dot_hash[i] {
				
			}
			else {
				return false;
			}
		}		
	}
	
	return true;
}

fn make_string (guide: Vec<usize>, map: Vec<u32>) -> String{
	let mut pos: usize = 0;
	let mut prev: usize = 0;
	let mut s: Vec<char> = vec![];
	
	for (i, el) in guide.iter().enumerate() {
		if i == 0 {
			pos = *el;	
			prev = *el;
		}
		else {
			pos = *el - prev;
			prev = *el;
		}
		
		
		for _i in 0..pos{
			s.push('.')
		}
		
		for _i in 0..map[i] {
			s.push('#');
		}
	}
	let s_string: String = s.iter().collect();
	
	s_string
	
}


fn read_input(filename: &str) -> (Vec<String>, Vec<Vec<u32>>) {
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);
	let mut dot_hash: Vec<String> = vec![];
	let mut num_map: Vec<Vec<u32>> = vec![];
	
	for line in reader.lines() {
		let bind = line.expect("FUCKED");
		let mut iter = bind.split(" ");
		dot_hash.push(iter.next().expect("TRIPLE FUCKED").to_string().replace("....",".").replace("...",".").replace("..","."));
		num_map.push(iter.next().expect("DOUBLE FUCKED").split(",").map(|x| x.parse().unwrap()).collect());
	}
	
	return (dot_hash, num_map)
}
