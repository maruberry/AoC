use std::fs;

fn main() {
    
    let (directions, map, end_index, start_index) = read_file("day8_input.txt".to_string());
	let answer = navigate(directions, map, end_index, start_index);
	println!("The final answer is {}", answer);
}

fn navigate (directions: String, map: Vec<Vec<String>>, end_index: usize, start_index: usize) -> u32 {
	let mut current_pos = start_index;
	let mut move_count = 0;
	println!("end at {}", end_index);
	println!("index 4 -> {:?}", map[4]);
	while current_pos != end_index{
		let direct_clone = directions.clone();
		
		for dir in direct_clone.chars() {
			move_count += 1;
			current_pos = map[current_pos][dir as usize - 0x30].parse().unwrap();
			println!("{}", move_count);
			if current_pos == end_index{break;}
		}	
	}
	
	return move_count
}

fn read_file(filename: String) -> (String, Vec<Vec<String>>, usize, usize) {
	
	let full_string = fs::read_to_string(filename).unwrap();
	let mut end_index: usize = 0;
	let mut start_index: usize = 0;
	let mut iter = full_string.split("\n\n");
	let left_right = iter.next().unwrap().to_string().replace("L", "0").replace("R", "1");
	let mut map = iter.next().unwrap().to_string();
	(map, end_index, start_index) = replace_map(map);
	//println!("{}", map);
	let map_vec = vectorize_map(map);
	//println!("{:#?}", map_vec);
	
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

fn replace_map (mut input: String) -> (String, usize, usize) {
	
	let copy = input.clone();
	let mut end_index = 0;
	let mut start_index = 0;
	
	for (i, line) in copy.lines().enumerate() {
		let (first, second) = line.split_once(" = ").unwrap();
		println!("replacing {} w {}", first, i.to_string());
		if "ZZZ".eq(first) {end_index = i;}	
		if "AAA".eq(first) {start_index = i;}	

		input = input.replace(first, &i.to_string());
	}	
	
	//println!("{}", input);
	
	return (input, end_index, start_index);
}
