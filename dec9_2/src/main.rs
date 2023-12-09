use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("WE ARE STARTING THIS SHITSHOW");
    
    let input_array = read_file("dec9_input.txt".to_string());
    let answer = solve_task(input_array);
    
    println!("FINAL ANSWER {}", answer);
}

fn solve_task(input: Vec<Vec<i32>>) -> i32 {
	
	let mut final_answer: i32 = 0;
	
	for answer_array in input.iter().map(|x| extrapolate(x.clone())) {
		let len = answer_array.len();
		final_answer += answer_array[0];
	}
	
	final_answer	
}

fn extrapolate(mut input: Vec<i32>) -> Vec<i32> {
	let mut temp_lines: Vec<i32> = vec![];
	let len = input.len();
	let mut add: i32 = 0;
	let mut return_line: Vec<i32> = vec![];
	
	
	for i in 0..(len -1) {
		temp_lines.push(input[i + 1] - input[i]);	
	}

	if check_zero(&temp_lines) {
		return_line.push(input[0].clone());
		return_line.append(&mut input.clone());
	}
	else {
		let mut smth = extrapolate(temp_lines);
		add = smth[0];

		return_line.push(input[0] - add);
		return_line.append(&mut input.clone());
	}
	
	return_line
}

fn check_zero (input: &Vec<i32>) -> bool {
	
	for num in input.iter() {
		if *num != 0 {return false}
	}
	return true
}

fn read_file(filename: String) -> Vec<Vec<i32>> {
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);
	let mut vectorized_lines: Vec<Vec<i32>> = vec![];
	
	for line in reader.lines() {
		let line_vec = line.expect("FUUUUUCK").split(" ").map(|x| x.parse().unwrap()).collect();
		vectorized_lines.push(line_vec);
	}
	
	vectorized_lines
}
