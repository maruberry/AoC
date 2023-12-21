use std::io::Read;
use std::fs::File;

fn main() {
    println!("WE ARE STARTING THIS SHITSHOW");
    let map = read_shit("day15_input.txt");
    let answer = calc_shit(map);
    
    println!("The final answer is {}", answer);
}

fn calc_shit(map: Vec<Vec<u8>>) -> u32 {
	let mut answer: u32 = 0;
	let mut cur_value: u32  = 0;
	
	for step in map.iter() {
		cur_value = 0;
		for c in step.iter() {
			if *c == "\n".as_bytes()[0] { continue; }
			
			println!("c = {}", c);
			cur_value += *c as u32;
			cur_value *= 17;
			cur_value %= 256;
		}
		println!("cur_value {}", cur_value);
		answer += cur_value;
	}
	
	answer
	
}

fn read_shit(filename: &str) -> Vec<Vec<u8>> {	
	let mut file = File::open(filename).unwrap();
	let mut buffer = String::new();
	let _ = file.read_to_string(&mut buffer);
	let mut map: Vec<Vec<u8>> = vec![];
	
	for input in buffer.split(",") {
		
		let bytes: Vec<u8> = input.as_bytes().to_vec();
		
		
		
		map.push(bytes);
	}
	
	map
}
