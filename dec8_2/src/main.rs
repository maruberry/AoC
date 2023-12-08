use std::fs;
use num::integer::lcm;

fn main() {
    
    let (directions, map, end_index, start_index) = read_file("day8_input.txt".to_string());
	let answer = navigate(directions, map, end_index, start_index);
	println!("The final answer is {}", answer);
}

fn navigate (directions: String, map: Vec<Vec<String>>, end_index: Vec<usize>, start_index: Vec<usize>) -> usize {
	let mut found: bool = false;
	let mut answer_vec = Vec::new();

	for start in start_index {
		
		let mut current_pos = start;
		let mut move_count = 0;
		
		'outer: while !found {
			let direct_clone = directions.clone();
			
			for dir in direct_clone.chars() {
				move_count += 1;
				current_pos = map[current_pos][dir as usize - 0x30].parse().unwrap();
				if end_index.contains(&current_pos){break 'outer;}
			}	
		}
		answer_vec.push(move_count);
	}
	
	let answer = lcm_vec(&answer_vec);
	
	return answer
}

fn read_file(filename: String) -> (String, Vec<Vec<String>>, Vec<usize>, Vec<usize>) {
	
	let full_string = fs::read_to_string(filename).unwrap();
	let mut iter = full_string.split("\n\n");
	let left_right = iter.next().unwrap().to_string().replace("L", "0").replace("R", "1");
	let  map = iter.next().unwrap().to_string();
	let (map, end_index, start_index) = replace_map(map);
	let map_vec = vectorize_map(map);
	
	return (left_right, map_vec, end_index, start_index)
}

fn vectorize_map (input: String) -> Vec<Vec<String>> {
	 
	let mut input_vec: Vec<Vec<String>> = Vec::new();
	
	for line in input.lines(){
		let mut inside_vec: Vec<String> = Vec::new();
		let (first, second) = line.split_once(" = (").unwrap();
		
		let (left, right1) = second.split_once(", ").unwrap();
		inside_vec.push(left.to_string());
		let right = &right1[0..right1.len()-1];
		inside_vec.push(right.to_string());
		input_vec.push(inside_vec);
	}
	return input_vec;
}

fn replace_map (mut input: String) -> (String, Vec<usize>, Vec<usize>) {
	
	let copy = input.clone();
	let mut end_index = Vec::new();
	let mut start_index = Vec::new();
	
	for (i, line) in copy.lines().enumerate() {
		let (first, second) = line.split_once(" = ").unwrap();

		if "Z".eq(&first[2..3]) {end_index.push(i);}	
		if "A".eq(&first[2..3]) {start_index.push(i);}	

		input = input.replace(first, &i.to_string());
	}	
	
	
	return (input, end_index, start_index);
}

// I got this function from https://github.com/lysender/advent-of-code-2023/blob/master/src/day8.rs
fn lcm_vec(input: &Vec<usize>) -> usize {
    if input.len() == 2 {
        return lcm(input[0], input[1]);
    } else {
        let first_num = input[0];
        let the_rest: Vec<usize> = input.iter().skip(1).map(|x| *x).collect();
        let second_num = lcm_vec(&the_rest);
        return lcm(first_num, second_num);
    }
}
