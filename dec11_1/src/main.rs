use std::fs::File;
use std::io::{BufRead, BufReader};
use colored::Colorize;


fn main() {
    println!("WE ARE STARTING THIS SHITSHOW");
    
    let map = read_map_add_rows("day11_input.txt");
	//check_map(map.clone());
	let answer = calc_shit(map);
	println!("Final answer is {}", answer);
}

fn calc_shit (input: Vec<Vec<char>>) -> usize {
	
	let line_str = input[0].iter().map(|x| x.to_string()).collect::<Vec<_>>().join("");
	let mut hash_loc = line_str.match_indices("#");
	let mut loc_vec: Vec<usize> = vec![];
	let mut answer: usize = 0;
	let len = loc_vec.len();
	
	if len != 1 {
		for (loc, _s) in hash_loc {
			loc_vec.push(loc);
			for (i, line) in input[1..].iter().enumerate() {
				for (j, c) in line.iter().enumerate() {
					if *c == '#' {
						
						if loc >= j {
							answer += i + loc - j + 1;
						}
						else {
							answer += i + j - loc + 1;
						}
						
					}
				}
			}	
		}
	}
	
	for (i, ay) in loc_vec[0..].to_vec().iter().enumerate() {
		for oh in loc_vec[(i+1)..].to_vec().iter() {
			answer += oh - ay;
		}
	}
	
	if input.len() != 1 {
		answer += calc_shit(input[1..].to_vec());
	}
	//println!("Current answer {}", answer);
	return answer;
}

fn read_map_add_rows(filename: &str) -> Vec<Vec<char>> {
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);
	let mut vec_lines: Vec<Vec<char>> = vec![];
	
	for lines in reader.lines() {
		let line = lines.expect("LINE16");
		if check_empty(line.clone()) {
			vec_lines.push(line.clone().chars().collect());		
		}	
		vec_lines.push(line.clone().chars().collect())		
	}
	let mut return_vec = add_column(vec_lines);	
	
	//println!("{:#?}", return_vec);
	
	return return_vec	
}

fn check_empty(input: String) -> bool {
	
	let len = input.len();
	let count = input.matches(".").collect::<Vec<_>>().len();
	
	if len == count { return true}
	else { return false }
	
}

fn add_column(mut input: Vec<Vec<char>>) -> Vec<Vec<char>> {
	println!("fuck");
	let mut return_vec: Vec<Vec<char>> = vec![];
	let mut added: usize = 0;
	let mut smth = input.clone();
	
	let len = input[0].clone().len();
	for i in 0..len {
		//println!("{:?}", smth);
		let mut column: Vec<char> = vec![];
		for line in input.clone().iter() {
			column.push(line[i]);
		}
		if !column.contains(&'#') {		
			for line in smth.clone().iter() {
				let mut new_line: Vec<char> = vec![];
				//println!("{:?}",line);
				for (j, tht) in line.iter().enumerate() {
					//print!("S {}", tht);
					new_line.push(*tht);
					if j == i + added {
						new_line.push(*tht);
					}
				}	
				//println!("");
				//println!("{:?}", new_line);	
				return_vec.push(new_line);				
			}
			added += 1;		
			smth = return_vec.clone();	
			return_vec.clear();
		}	
	}
	
	smth 
}

fn check_map(map: Vec<Vec<char>>){
	println!("?");
	for line in map.iter(){
		for m in line.iter(){
			print!("{}", m);
		}
		println!("");
	}
}
