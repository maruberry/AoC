use std::fs::File;
use std::io::{BufRead, BufReader};
use colored::Colorize;

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
enum Dir {
	Vert,
	Horiz,
	UpRight,
	UpLeft,
	LeftDown,
	RightDown,
	Ground,
	Start,
}

#[derive(PartialEq)] 
enum IsIn{
	Yes,
	No,
	NoUp,	
	NoDown,
	YesUp,
	YesDown,
}

fn main() {
    println!("WE ARE STARTING THIS SHITSHOW");
    
    let (map, start) = read_in_map("day10_input.txt");
    let answer = calc_answer(map, start);
    
    println!("The final answer is {}", answer);
}

fn calc_answer(map: Vec<Vec<char>>, start: (usize, usize)) -> u32 {
	let (mut prev_i, mut prev_j) = start;
	let (mut current_i, mut current_j) = start_next(start, map.clone());
	let mut answer_vec: Vec<char> = vec![];
	
	let change_i = current_i as i32 - prev_i as i32;
	let change_j = current_j as i32 - prev_j as i32;
	//println!("START {:?}", start);
	//println!("{}", map[current_i] [current_j]);
	while assign_dir(map[current_i][current_j]) != Dir::Start {
		answer_vec.push(map[current_i][current_j].clone());
		let temp = what_next((current_i, current_j, assign_dir(map[current_i][current_j].clone())), (prev_i, prev_j));
		
		(prev_i, prev_j) = (current_i, current_j);
		(current_i, current_j) = temp;
		//println!("{}", map[current_i] [current_j]);
	}
	
	let new_map = make_new_map(answer_vec, (change_i, change_j), start, map.len(), map[0].len());
	
	//println!("The answer is {}", answer);
	
	return new_map
}

fn make_new_map(input: Vec<char>, change: (i32, i32), start: (usize, usize), i: usize, j: usize) ->  u32{
	let mut answer_vec: Vec<Vec<char>> = vec![vec![' '; j]; i];
	
	//println!("array size {} {}", answer_vec.len(), answer_vec[0].len());
	//let mut temp_vec: Vec<Vec<char>> = vec![vec![' '; i]; j];
	let (mut next_i, mut next_j) = start;
	let (change_i, change_j) = change;
	
	//println!("INPUT: {:?}", input);
	
	answer_vec[next_i][next_j] = 'S';
	let (mut prev_i, mut prev_j) = (next_i, next_j);
	next_i = (next_i as i32 + change_i) as usize;
	next_j = (next_j as i32 + change_j) as usize;
	
	for elm in input.clone().iter() {
		//println!("{}, next {} {}", elm, next_i, next_j);
		answer_vec[next_i][next_j] = elm.clone();
		//println!("FUCK");
		let temp = what_next((next_i, next_j, assign_dir(*elm)), (prev_i, prev_j));
		(prev_i, prev_j) = (next_i, next_j);
		(next_i, next_j) = temp;
	}
	
	let mut final_answer: u32 = 0;
	final_answer += calc_answer2(answer_vec);
	
	return final_answer
}

fn calc_answer2(input: Vec<Vec<char>>) -> u32 {
	
	let breaks: Vec<Vec<usize>> = vec![];
	
	let mut state: IsIn = IsIn::No;
	let mut count: u32 = 0;
	
	for (i, line) in input.iter().enumerate() {
		state = IsIn::No;
		print!("{:3} - ", i);
		for x in line.iter() {
			
			match x {
				'L' | 'J' => {
					match state {
						IsIn::Yes => state = IsIn::YesUp,
						IsIn::No => state = IsIn::NoUp,
						IsIn::NoUp | IsIn::YesDown => state = IsIn::No	,
						IsIn::NoDown  | IsIn::YesUp => state = IsIn::Yes,
						
					}
					print!("{}", x);				
				}
				'F' | '7' => {
					match state {
						IsIn::Yes => state = IsIn::YesDown,
						IsIn::No => state = IsIn::NoDown,
						IsIn::NoUp | IsIn::YesDown => state = IsIn::Yes,
						IsIn::NoDown  | IsIn::YesUp => state = IsIn::No,
						
					}
					print!("{}", x);	
				}	
				'|' | 'S' => {
					match state {
						IsIn::Yes => state = IsIn::No,
						IsIn::No => state = IsIn::Yes,
						IsIn::NoUp | IsIn::YesDown => print!("X"),
						IsIn::NoDown  | IsIn::YesUp => print!("X"),					
					}
					print!("{}", x);	
				}
				'-' => print!("-"),
				' ' => 	{
					match state{
						IsIn::Yes | IsIn::YesUp | IsIn::YesDown => {count += 1; print!("{}", "1".green());}
						IsIn::No | IsIn::NoUp | IsIn::NoDown => {count += 0; print!("{}", "0".red());}
						
					}
										
				}
				_ => print!("X"),
			}
				
		}
		println!("");
	}
	
	count
	
}

fn start_next (start: (usize, usize), map: Vec<Vec<char>>) -> (usize, usize) {
	let (i, j) = start;
	
	
	
	if i != 0 {
		let l = assign_dir(map[i - 1][j]);
		
		if l == Dir::Vert 
			|| l == Dir::LeftDown 
			|| l == Dir::RightDown 
		{
			return (i - 1, j)			
		}
	}
	
	if i != map.len() - 1 {
		let l = assign_dir(map[i + 1][j]);
		if l == Dir::Vert
			|| l == Dir::UpLeft
			|| l == Dir::UpRight
		{
			return (i + 1, j)
		} 
	}
	
	if j != 0 {
		let l = assign_dir(map[i][j - 1]);
		if l == Dir::Horiz
			|| l == Dir::RightDown
			|| l == Dir::UpRight
		{
			return (i, j - 1)
		}
	}
	
	if j != map[0].len() - 1 {
		let l = assign_dir(map[i][j + 1]);
		if l == Dir::Horiz
			|| l == Dir::LeftDown
			|| l == Dir::UpLeft
		{
			return (i, j + 1)
		}
	}
	
	println!("WE ARE SO FUCKED 3");
	std::process::abort()
}

fn what_next(start: (usize, usize, Dir), prev: (usize, usize) ) -> (usize, usize) {
	let (i, j, direct) = start;
	let mut next_i: usize = 0;
	let mut next_j: usize = 0;
	let (prev_i, prev_j) = prev;
	
	//println!("FINDIN NEXT FROM {:?}, index {} {}", direct, i, j);
	
	match direct {
		Dir::Vert => { 
			if prev_i < i {
				next_i = i + 1;
			}
			else {
				next_i = i - 1;				
			}
			next_j = j;		
		}
		
		Dir::Horiz => {
			
			if prev_j < j {
				next_j = j + 1;
			}
			else {
				next_j = j - 1;
			}
			next_i = i;
		}
		Dir::LeftDown => {
			
			if prev_j < j {
				next_i = i + 1;
				next_j = j;
			}
			else {
				next_j = j - 1;
				next_i = i;
			}
		}
		Dir::RightDown => {
			if prev_j > j {
				next_i = i + 1;
				next_j = j;
		}
			else {
				next_j = j + 1;
				next_i = i;
			}
		}
		Dir::UpLeft => {
			if prev_j < j {
				next_i = i - 1;
				next_j = j;
			}
			else {
				next_j = j - 1;
				next_i = i;
			}
		}
		Dir::UpRight => {
			if prev_j > j {
				next_i = i - 1;
				next_j = j;
			}
			else {
				next_j = j + 1;
				next_i = i;
			}
		}
		
		_ => { println!("FUCKED PART 2"); 
			  std::process::abort() }
	}
	//println!("We are out");
	(next_i, next_j)
}

fn read_in_map (filename: &str) -> (Vec<Vec<char>>, (usize, usize)) {
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);
	let mut vec_lines: Vec<Vec<char>> = vec![];
	let mut start_i: usize = 0;
	let mut start_j: usize = 0;
	
	for (i, line) in reader.lines().enumerate() {
		let mut line_chars: Vec<char> = vec![];
		
		for (j, character) in line.expect("HATE YA").chars().enumerate() {
			if character == 'S' { 
				start_i = i;
				start_j = j;
			}
			line_chars.push(character);			
		}
		vec_lines.push(line_chars);
	}
	
	//println!("{:?}", vec_lines);
	(vec_lines, (start_i, start_j))
}

fn assign_dir (input: char) -> Dir {
	
	let return_dir: Dir;
	
	match input {
		'|' => return_dir = Dir::Vert,
		'-' => return_dir = Dir::Horiz,
		'L' => return_dir = Dir::UpRight,
		'J' => return_dir = Dir::UpLeft,
		'7' => return_dir = Dir::LeftDown,
		'F' => return_dir = Dir::RightDown,
		'.' => return_dir = Dir::Ground,
		'S' => return_dir = Dir::Start,
		  _ => { println!("FUCKED"); 
			  std::process::abort() }
	}
	
	return_dir
}
