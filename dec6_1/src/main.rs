use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("WE ARE STARTING THIS SHITSHOW");
    
    let (times, record) = read_in_data("dec6_input.txt".to_string());
    let final_answer = calc_answer(times, record);
    
    println!("Final answer is {}", final_answer);
}

fn calc_answer(times: Vec<i32>, record: Vec<i32>) -> i32 {
	
	let mut final_answer: i32 = 1;
	
	for (i, time) in times.iter().enumerate() {
		println!("Going through race {}", i + 1 );
		final_answer *= race_calc(*time, record[i]);
	}
	
	final_answer
}

fn race_calc (time: i32, record: i32) -> i32{
	
	let mut margin: i32 = 0;
	
	for i in 0..=time {
		if i * (time - i) > record {
			margin += 1;		
		}
	}
	
	margin
}

fn read_in_data(filename: String) -> (Vec<i32>, Vec<i32>) {	
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);
	let mut iter = reader.lines();
	
	let times: Vec<i32> = process_input(iter
						.next()
						.unwrap()
						.expect("FUCKED"));
						
	let record: Vec<i32> = process_input(iter
						.next()
						.unwrap()
						.expect("FUCKED"));
						
	
	
	return (times, record)
}

fn process_input(line: String) -> Vec<i32> {
	
	line.split(":")
		.nth(1)
		.expect("DOUBLE FUCKED")
		.trim()
		.split(' ')
		.filter(|s| !s.is_empty())
		.collect::<Vec<_>>()
		.join(" ")
		.split(" ")
		.map(|x| x.parse::<i32>().unwrap())
		.collect()
}
