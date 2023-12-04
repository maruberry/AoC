use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    println!("WE ARE STARTING THIS SHITSHOW");
	let filename = "dec3_input.txt";
	let schema = make_array(filename);
	
	let gear_positions = find_gears(schema.clone());	
	
	//println!("The gear positions are {:?}", gear_positions);
	let final_answer = calculate_total(schema.clone(), gear_positions);
	
	println!("THe final answer is {}", final_answer);
}

fn find_gears(schematic: Vec<Vec<char>>) -> Vec<(usize, usize)> {
	
	let mut gears = Vec::new();
	
	for (i, line) in schematic.iter().enumerate() {
		for (j, element) in line.iter().enumerate() {
			if  *element == '*' {
				gears.push((i, j));
			}
		}
	}
	
	return gears
}

fn calculate_total(schematic: Vec<Vec<char>>, positions: Vec<(usize, usize)>) -> i64 {
	
	let mut final_answer: i64 = 0;
	
	for ratio in positions.iter().map(|x| calculate_ratio(schematic.clone(), *x)) {
		final_answer += ratio;
	}
	
	return final_answer	
}

fn calculate_ratio(schematic: Vec<Vec<char>>, position: (usize, usize)) -> i64 {
	
	let (width, height) = get_schema_size("dec3_input.txt");
	let (i, j) = position;
	
	let _num_count: i32 = 0;
	
	let column = values_column(i, width);
	let row = values_column(j, height);
	
	//println!("ROW {} - {} COLUMN {} - {}", start_row, end_row, start_column, end_column); Vec<Option<i32>>
	
	let total = fuck_if_i_know(schematic.clone(), column, row);
	if total.len() == 2 {
		return total[0] * total[1]
	}
	
	return 0;
	
}

fn fuck_if_i_know(schematic: Vec<Vec<char>>, column: (usize, usize), row: (usize, usize)) -> Vec<i64>{
	
	
	let (start_column, end_column) = column;
	let (start_row, end_row) = row;
	
	let mut total: Vec<_> = vec![];
	for (i, lines) in schematic.iter().enumerate(){	
		
		if i >= start_column && i <= end_column {
			let digits: Vec<_>  = lines[start_row..=end_row]
							.iter()
							.enumerate()
							.filter(|(_count, x)| x.is_digit(10)	)
							.map(|(count, _x)| read_full_number(schematic.clone(), (i, count + start_row)))
							.collect();
							
			total.extend(digits);			
		}
	}
	
	total.dedup();
	return total
}

fn read_full_number(schematic: Vec<Vec<char>>, location: (usize, usize)) -> i64 {
	
	let (i ,mut j) = location;
	let mut number: i32;
	
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
	//println!("NUM {}", number);
	return number as i64
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

fn values_column(i: usize, width: usize) -> (usize, usize) {
	let start_column: usize;
	let end_column: usize;
	
	if i != 0 { start_column = i - 1 }
	else { start_column = i}
	
	if i == width { end_column = i}
	else { end_column = i + 1}
	
	return (start_column, end_column)
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
