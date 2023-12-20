use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("WE ARE STARTING THIS SHITSHOW");
    
    let maps = read_maps("day13_input.txt");
    let answer = arithmetics_n_shiz(maps);
    println!("final answer is {}", answer);
}

fn arithmetics_n_shiz (maps: Vec<Vec<Vec<u8>>>) -> u32{
	
	let mut answer: u32 = 0;
	//println!("{:?}", maps);
	
	for el in maps.iter() {
		answer += handle_map(el.clone()) * 100;
		let um = transpose(el.clone());
		answer += handle_map(um.clone())
	}
	
	answer
}

fn handle_map (map: Vec<Vec<u8>>) -> u32 {
	
	let len = map.len() - 1;
	let mut answer1: usize = 0;

	let mut state: bool = true;
	let mut prev: Vec<Vec<u8>> = vec![];
	
	for (i, el) in map.clone().iter().enumerate() {

		if i == 0 {
			prev.push(el.clone());
			continue;
		}
		
		if el.eq(&prev[i - 1]) {
			//answer1 += i;
			state = true;
			
			for (j, ah) in prev.iter().rev().enumerate() {
				
				if i + j > len {
					continue;
				}

				//println!("{:?} VS {:?}", ah, map[i + j]);
				if ah.iter().zip(&map[i + j]).filter(|&(a, b)| a != b).count() != 0 {
					state = false;
				}
			}
			
			if state {
				answer1 += i;
			}			
		}
		
		prev.push(el.clone());
	}

	return answer1 as u32
}

fn transpose (map: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
	let wide = map[0].len();
	let len = map.len();
	//println!("in {:?}", map);
	//println!("in WID {}, LEN {}", wide, len);
	let mut new: Vec<Vec<u8>> = vec![vec![0; len]; wide];
	
	for i in 0..wide {
		for j in 0..len {
			new[i][j] = map[j][i]
		}
	}
//	println!("in {:?}", new);
//	println!("out WID {}, LEN {}", new[0].len(), new.len());
	new
}

fn read_maps(filename: &str) -> Vec<Vec<Vec<u8>>> {
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);
	let mut all: Vec<Vec<Vec<u8>>> = vec![];
	let mut single: Vec<Vec<u8>> = vec![];
	
	for el in reader.lines() {
		let huh = el.expect("FFS");
		if huh.len() != 0 {
			single.push(huh.bytes().collect());
		}	
		else {
			//println!("EMPTY");
			all.push(single.clone());
			single.drain(..);
		}	
	}
	all.push(single.clone());
	
	return all
}
