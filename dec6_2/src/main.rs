use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("WE ARE STARTING THIS SHITSHOW");
    
    let (times, record) = read_in_data("day6_input.txt".to_string());
    let final_answer = race_calc(times, record);
    
    println!("Final answer is {}", final_answer);
}

//~ fn calc_answer(times: Vec<i32>, record: Vec<i32>) -> i32 {
	
	//~ let mut final_answer: i32 = 1;
	
	//~ for (i, time) in times.iter().enumerate() {
		//~ println!("Going through race {}", i + 1 );
		//~ final_answer *= race_calc(*time, record[i]);
	//~ }
	
	//~ final_answer
//~ }

fn race_calc (time: i64, record: i64) -> i64{
	
	let mut margin: i64 = 0;
	
	for i in 0..=time {
		if i * (time - i) > record {
			margin += 1;		
		}
	}
	
	margin
}

fn read_in_data(filename: String) -> (i64, i64) {	
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);
	let mut iter = reader.lines();
	
	let times: i64 = process_input(iter
						.next()
						.unwrap()
						.expect("FUCKED"));
						
	let record: i64 = process_input(iter
						.next()
						.unwrap()
						.expect("FUCKED"));
						
	
	
	return (times, record)
}

fn process_input(line: String) -> i64 {
	
	line.split(":")
		.nth(1)
		.expect("DOUBLE FUCKED")
		.trim()
		.split(' ')
		.filter(|s| !s.is_empty())
		.collect::<Vec<_>>()
		.join("")
		.parse::<i64>().unwrap()

}
