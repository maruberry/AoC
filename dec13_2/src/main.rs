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
	
	for (i, el) in maps.iter().enumerate() {
		
		answer += handle_map(el.clone(), i) * 100;
		let um = transpose(el.clone());
		answer += handle_map(um.clone(), i)
	}
	
	answer
}

fn handle_map (map: Vec<Vec<u8>>, num: usize) -> u32 {
	
	let len = map.len() - 1;
	let mut answer1: usize = 0;
	let mut prev: Vec<Vec<u8>> = vec![];

	let mut diff_cnt: u32 = 0;
	prev.clear();
	
	for (i, el) in map.clone().iter().enumerate() {
		
		diff_cnt = 0;
		if i == 0 {
			prev.push(el.clone());
			continue;
		}
		
		let mut prev_len = prev.len();
		for (n, by) in el.clone().iter().enumerate() {
			if *by != prev[prev_len - 1][n] {
				diff_cnt += 1;
			}
			
			if diff_cnt > 1 {
				break;
			}
		}
		if diff_cnt == 1 && i == 1 {
			println!("LINE {} NEW {}", num, i);
			answer1 += i;
			break;
		}
		
		if diff_cnt < 2 {
				
			for (j, ah) in prev.iter().rev().enumerate() {
				
				if j == 0 {
					continue;
				}
				
				if i + j > len {
					continue;
				}

				for _m in 0..ah.iter().zip(&map[i + j]).filter(|&(a, b)| a != b).count()  {
					diff_cnt += 1;
				}
				
				if diff_cnt > 1 {
					break;
				}
			}

			if diff_cnt == 1 {
				println!("LINE {} NEW {}", num, i);
				answer1 += i;
				return answer1 as u32
			}
		}
			
		prev.push(el.clone());
		
		
	}

	return answer1 as u32
}

fn transpose (map: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
	let wide = map[0].len();
	let len = map.len();

	let mut new: Vec<Vec<u8>> = vec![vec![0; len]; wide];
	
	for i in 0..wide {
		for j in 0..len {
			new[i][j] = map[j][i]
		}
	}

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

			all.push(single.clone());
			single.drain(..);
		}	
	}
	all.push(single.clone());
	
	return all
}
