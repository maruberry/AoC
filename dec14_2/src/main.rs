use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
    println!("Hello, world!");
    
    let map = read_in_map("day14_input.txt");
    let moved_map = scramble(map);
    let answer = calc_answer(transpose(moved_map));
    
    println!("The final answer is {}", answer);
    
    
}

fn scramble (map: Vec<Vec<char>> ) -> Vec<Vec<char>> {
	
	let mut same: bool = false;
	let mut le_ri: bool = true;
	let mut scram_map = map.clone();
	let mut cnt: usize = 0;
	let mut change_cnt: u32 = 0;
	let mut check_loop = map.clone();
	let mut loop_cnt: u32 = 0;
	let mut loop_len: u32 = 0;
	let mut loop_found: u32 = 0;
	let mut needed = 1000000000;
	
	let mut found: bool = false;
	
	while change_cnt != needed {
		
		change_cnt += 1;
		if cnt == 2 {
			le_ri = !le_ri	;	
			cnt = 0;
		}
		
		scram_map = transpose(scram_map.clone());
		
		if le_ri {
			scram_map = push_left(scram_map.clone());
		}
		else {
			scram_map = push_right(scram_map.clone());
		}
	
		cnt += 1;
	
		if calc_answer(scram_map.clone()) == 64 && change_cnt % 4 == 0{
			//println!("ANSWER AT {}", change_cnt);
			debug(scram_map.clone());
		}
		
		if change_cnt == 1000 {
			check_loop = scram_map.clone();
		}
		
		if change_cnt > 1000 && change_cnt % 4 == 0{
			loop_cnt += 4;
			
			if comp_map(check_loop.clone(), scram_map.clone()) && loop_len == 0 {
				loop_len = loop_cnt;
				loop_found = change_cnt;
				debug(scram_map.clone());
				let to_go = (1000000000 - (loop_found / 4) ) % loop_len;
				needed = change_cnt + to_go * 4;
			}
		}
	}
	
	scram_map	
}

fn debug (og_map: Vec<Vec<char>>) {
	for l in og_map.iter() {
		for c in l.iter(){
			print!("{}", c);
		}
		println!("");
	}
	
}

fn comp_map(og_map: Vec<Vec<char>>, new_map: Vec<Vec<char>>) -> bool {
	
	
	for (i, a) in og_map.iter().enumerate() {
	
		for (j, c) in a.iter().enumerate() {
			if new_map[i][j] != *c {
				return false
			}
		}
	}
	
	return true
}

fn calc_answer (map: Vec<Vec<char>>) -> usize {
	
	let mut answer: usize = 0;
	let add: usize = map[0].len();

	for line in map.iter(){
		for (i, c) in line.iter().enumerate() {
			if *c == 'O' {
				answer += add - i;
			}
		}

	}
	
	return answer;
}

fn push_left (map: Vec<Vec<char>>) -> Vec<Vec<char>> {
	
	let wide = map[0].len();
	let len = map.len();
	let mut push_where: usize = 0;

	let mut new_map: Vec<Vec<char>> = vec![vec!['.'; len]; wide];
	
	for (i, line) in map.iter().enumerate() {
		
		push_where = 0;
		
		
		for (j, c) in line.iter().enumerate() {
			if *c == 'O' {
				new_map[i][push_where] = 'O';
				
				push_where += 1;

			}
			else if *c == '#' {
				new_map[i][j] = '#';

				push_where = j + 1;
			}		
		}	
	}
	
	new_map
}

fn push_right (map: Vec<Vec<char>>) -> Vec<Vec<char>> {
	
	let wide = map[0].len() - 1;
	let len = map.len();
	let mut push_where: usize = 0;

	let mut new_map: Vec<Vec<char>> = vec![vec!['.'; len]; wide + 1];
	
	for (i, line) in map.iter().enumerate() {
		
		push_where = wide;
		
		
		for (j, c) in line.iter().rev().enumerate() {
			//println!("PUSH TO {}", push_where);
			if *c == 'O' {
				new_map[i][push_where] = 'O';
				
				if push_where != 0 {
					push_where -= 1;
				}

			}
			else if *c == '#' {
				//println!("{} {} {}",wide , j, i);
				new_map[i][wide - j] = '#';
				
				if (wide - j) != 0 {
					push_where = wide - j - 1;
				}
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
