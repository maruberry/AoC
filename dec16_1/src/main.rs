use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq)]
#[derive(Clone)]
enum Direct {
	Up,
	Down,
	Left,
	Right,
}

fn main() {
    println!("WE ARE STARTING THIS SHITSHOW");
    
    let map = read_map("day16_input.txt");
    let start_loc: Vec<usize> = vec![0,0];
    let final_energy = light_flow(map.clone(), new_map(map), Direct::Right, &start_loc);
    debug(final_energy.clone());
    
    let answer = answer_count(final_energy.clone());
    
    println!("Final answer is {}", answer);
    
}

fn answer_count (en_map: Vec<Vec<u8>>) -> usize {
	
	let mut answer: usize = 0;
	for line in en_map.iter() {
		answer += line.iter().filter(|x| **x == b'#').collect::<Vec<_>>().len();
	}
	answer
}

fn debug (og_map: Vec<Vec<u8>>) {
	for l in og_map.iter() {
		let s = std::str::from_utf8(&l).expect("invalid utf-8 sequence");
		println!("{}", s);
	}	
	println!("");
}

fn new_map (map: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
	let wide = map[0].len();
	let len = map.len();
	
	
	vec![vec![b'.'; wide]; len]
}

fn light_flow(map: Vec<Vec<u8>>, energy_map: Vec<Vec<u8>>, start_dir: Direct, start_loc: &Vec<usize>) -> Vec<Vec<u8>>{
	
	let wide = map[0].len() as i32;
	let len = map.len() as i32;
	
	let mut energized = energy_map.clone();
	let mut location: Vec<usize> = start_loc.clone();
	let mut loc32: Vec<i32> = vec![location[0] as i32, location[1] as i32];
	let snuffed: bool = true;
	let mut dir: Direct = start_dir;
	
	while snuffed {
		
		if map[location[0]][location[1]] == b'.' ||
			map[location[0]][location[1]] == b'/' ||
			map[location[0]][location[1]] == 92	{
			dir = new_dir(dir, map[location[0]][location[1]]);
			
		}
		else if (dir == Direct::Right || dir == Direct::Left)
			&& map[location[0]][location[1]] == b'-' 
		{
			dir = new_dir(dir, map[location[0]][location[1]]);
		}
		else if (dir == Direct::Up || dir == Direct::Down)
			&& map[location[0]][location[1]] == b'|' 
		{
			dir = new_dir(dir, map[location[0]][location[1]]);
		}
		else if (dir == Direct::Left || dir == Direct::Right)
			&& map[location[0]][location[1]] == b'|'  
		{
			if energized[location[0]][location[1]] == b'#' {
				return energized;
			}
			energized[location[0]][location[1]] = b'#';	
			energized = light_flow(map.clone(), energized.clone(), Direct::Up, &location);
			dir = Direct::Down;
		}
		else if (dir == Direct::Up || dir == Direct::Down)
			&& map[location[0]][location[1]] == b'-' 
		{
			if energized[location[0]][location[1]] == b'#' {
				return energized;
			}
			energized[location[0]][location[1]] = b'#';	
			energized = light_flow(map.clone(), energized.clone(), Direct::Right, &location);
			dir = Direct::Left;
		}
		
		energized[location[0]][location[1]] = b'#';	
		loc32 = new_loc(dir.clone(), loc32.clone());
		if loc32[0] < 0 || loc32[1] < 0 || loc32[0] >= len || loc32[1] >= wide{

			return energized
		}
		else {
			location[0] = loc32[0] as usize;
			location[1] = loc32[1] as usize;
		}		
		
	}
	
	return energized;
}

fn new_dir (mut dir: Direct, ch: u8) -> Direct {
	
	match ch {
		46 | b'|' | b'-' => return dir,
		92 => {
			match dir {
				Direct::Up => dir = Direct::Right,
				Direct::Down => dir = Direct::Left,
				Direct::Left => dir = Direct::Down,
				Direct::Right => dir = Direct::Up,
			}			
		}
		b'/' => {
			match dir {
				Direct::Up => dir = Direct::Left,
				Direct::Down => dir = Direct::Right,
				Direct::Left => dir = Direct::Up,
				Direct::Right => dir = Direct::Down,
			}			
		}
		_ => panic!("This should not be on the map"),
		
	}
	
	dir
}

fn new_loc(dir: Direct, old_loc: Vec<i32>) -> Vec<i32> {
	let mut location = old_loc;
	
	match dir {
			Direct::Up => location[0] += 1,
			Direct::Down => location[0] -= 1,
			Direct::Left => location[1] -= 1,
			Direct::Right => location[1] += 1,
	}	
	
	location
}

fn read_map(filename: &str) -> Vec<Vec<u8>>{	
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);
	let mut map: Vec<Vec<u8>> = vec![];
	
	for lines in reader.lines() {
		map.push(lines.expect("FUCKED").bytes().collect());		
	}
	
	map
}
