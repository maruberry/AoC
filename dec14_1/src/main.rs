use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("Hello, world!");
    
    let map = read_in_map("day14_input.txt");
    let moved_map = push_north(map);
    let answer = calc_answer(moved_map);
    
    println!("The final answer is {}", answer);
    
    
}

fn calc_answer (map: Vec<Vec<char>>) -> usize {
	
	let mut answer: usize = 0;
	let mut add: usize = map[0].len();
	
	for line in map.iter(){
		//println!("LINE {:?}", line);
		for (i, c) in line.iter().enumerate() {
			if *c == 'O' {
				//println!("{}", )
				answer += add - i;
			}
		}

	}
	
	return answer;
}

fn push_north (map: Vec<Vec<char>>) -> Vec<Vec<char>> {
	
	let wide = map[0].len();
	let len = map.len();
	
	let mut push_where: usize = 0;
	let mut new_map: Vec<Vec<char>> = vec![vec!['.'; len]; wide];
	
	for (i, line) in map.iter().enumerate() {
		
		let mut push_where: usize = 0;
		
		for (j, c) in line.iter().enumerate() {
			//println!("PUSH TO {}", push_where);
			if *c == 'O' {
				new_map[i][push_where] = 'O';
				push_where += 1;
			}
			else if *c == '#' {
				new_map[i][j] = '#';
				push_where = j + 1;
			}		
		}
		//println!("OLD {:?}", line);
		//println!("NEW {:?}", new_map[i]);		
	}
	
	new_map
}

fn read_in_map (filename: &str) -> Vec<Vec<char>> {
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);
	let mut map: Vec<Vec<char>> = vec![];
	
	for lines in reader.lines() {
		map.push(lines.expect("FUCKED").chars().collect());		
	}
	
	map = transpose(map);
	
	map
	
}

fn transpose (map: Vec<Vec<char>>) -> Vec<Vec<char>> {
	let wide = map[0].len();
	let len = map.len();

	let mut new: Vec<Vec<char>> = vec![vec!['.'; len]; wide];
	
	for i in 0..wide {
		for j in 0..len {
			new[i][j] = map[j][i]
		}
	}

	new
}
