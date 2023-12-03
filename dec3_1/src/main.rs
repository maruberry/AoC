use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
	println!("WE ARE STARTING THIS SHITSHOW");
	let filename = "dec3_input.txt";
	let schema = make_array(filename);
	//println!("{:?}", schema);
   // println!("We hath arrived to start");
    let final_answer: i32 = calculate_answer(schema);
	println!("The final answer is {}", final_answer)
}

fn calculate_answer(schematic: Vec<Vec<char>>) -> i32 {
	//println!("We hath arrived");
	let mut final_answer: i32 = 0;
	let mut result = 0;
	let mut begin = 0;
	let mut end = 0;
	
	let mut iter_row = schematic.iter().enumerate();				
	while let Some(x) = iter_row.next() {
		let (i, line) = x;
		let mut iter_column = line.iter().enumerate();
		print!("LINE {}: ", i);
		while let Some(y) = iter_column.next() {
			let (j, elements) = y;
			if schematic[i][j].is_digit(10) {
				
				(result, begin, end) = read_full_number(schematic.clone(), (i, j));
				
				if check_for_symbols(schematic.clone(), i, begin, end) {
					
					print!("{} ", result);
					final_answer += result;
					
					//print!("SKIP {} ", end - j);
					iter_column.nth(end - j);
					
				}			
			}
		}
		println!("");
	}
	return final_answer;
}

fn check_for_symbols(schematic: Vec<Vec<char>>, row: usize, mut begin: usize, mut end: usize) -> bool {
	
	//println!("We hath arrived");
	
	if begin != 0 { begin -= 1; }
	if end != 139 { end += 1; }
	
	for i in begin..=end {
		if row != 0  && row != 139{
			for j in row-1..=row+1 {
				if check_if_symbol(schematic[j][i]) { return true }
			}
		}
		else if row == 0{
			for j in row..=row+1 {
				if check_if_symbol(schematic[j][i]) { return true }
			}
		}
		else {
			for j in row-1..=row {
				if check_if_symbol(schematic[j][i]) { return true }
			}
		}		
	}
	
	return false;
}

fn check_if_symbol(check: char) -> bool {
	
	if  !check.is_digit(10)  && (check as i32) != 0x2E {
		return true;
	}
	else {
		return false;
	}
}


fn read_full_number(schematic: Vec<Vec<char>>, location: (usize, usize)) -> (i32, usize, usize) {
	let ( i,mut j) = location;
	let mut number = 0;
	
	let (begin, end) = find_begin_end(schematic.clone(), location);

	if begin == end { 
		number = schematic[i][j] as i32 - 0x30;
	}
	else {
		j = begin;
		number = schematic[i][j] as i32 - 0x30;
		while (j) < end {
			j += 1;
			number = number * 10 + (schematic[i][j] as i32 - 0x30);			
		}
	}
	
	return (number, begin, end)
}

fn find_begin_end(schematic: Vec<Vec<char>>, location: (usize, usize)) -> (usize, usize) {
	let (i, mut j) = location;
	let mut begin = 0;
	let mut end = 0;
	let mut k = j;
	while schematic[i][j].is_digit(10) {
		end = j;
		j += 1;
		if j >= 140 { break; }
	}

	while schematic[i][k].is_digit(10) {
		
		begin = k;
		if k == 0 {
			break;
		}
		k -= 1;
	}

	return (begin, end)
}

fn make_array(filename: &str) -> Vec<Vec<char>> {
	
	let (width, height) = get_schema_size(filename);
	let mut schema_temp = vec![vec!['l'; width]; height];
	
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);
	
	for (i, line)in reader.lines().enumerate(){
		match line {
			Ok(content) => {
				for (j, c) in content.chars().enumerate() {			
					schema_temp[i][j] = c;				
				}		
			},
			Err(_f) => println!("Shit happened ig"),
		}
	}	
	return schema_temp
}

fn get_schema_size(filename: &str) -> (usize, usize) {

	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);
	let mut width: usize = 0;
	let mut height: usize = 0;

	for (i, line)in reader.lines().enumerate(){
		height = i;
		match line {
			Ok(content) => width = content.len(),
			Err(_f) => println!("fucked ig"),
		}
	}
	
	return (width, height + 1);
}
