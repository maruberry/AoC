use std::fs::File;
use std::io::{BufRead, BufReader};

enum Colors {
	Red,
	Blue,
	Green,
	Unknown,
}

fn main() {
    
    let filename = "dec2_input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut final_answer: usize = 0;
    println!("WE ARE STARTING THIS SHITSHOW");
    
    for (i, line) in reader.lines().enumerate(){
		
		match line {
			
			Ok(h) => {
				let f = h.clone();
				let line_possible = separate_per_reveal(h);
				match line_possible {
					Ok(_v) => {
						final_answer += i + 1;
						//println!("GOT: {}", f);
						//println!("Current answer is {}", final_answer);
					}
					Err(_e) => println!("GOT: {}\n", f),
				}
			}
			Err(s) => println!("{}", s),
		}
	}
	println!("The final result is {}", final_answer);
}


fn find_index(line: &str, x: char, y: char, z: char) -> usize {
	
	for (i, item) in line.chars().enumerate(){
        if item == x || item == y || item == z{
            return i;
        }
    }
    
    line.len()
}

fn separate_per_reveal(line: String) ->  Result<i32, i32> {
	
	let len = line.len();
	let mut start_index = find_index(&line[0..len], ':', ',', ';') + 2;
	let mut	end_index = find_index(&line[start_index..len], ':',',', ';') - 1 + start_index;
	let mut num_final: i32;	
	
	while end_index < len - 1 {
		let num = &line[start_index..start_index + 1].chars().next().unwrap();
		if (&line[start_index + 1..start_index + 2].chars().next().unwrap()).is_digit(10) == true
		{
			let num_as_digit = (*num as i32 - 0x30 )* 10;
			num_final = num_as_digit + (line[start_index + 1..start_index + 2].chars().next().unwrap() as i32 - 0x30);
		}
		else 
		{
			num_final = *num as i32 - 0x30;
		}
		//println!("Hopefully the color {}", color);
		let current_color: Colors = which_color(&line[(start_index + 2)..end_index + 1]);
		let possible = is_reveal_possible(current_color, num_final);
		
		match possible {
			
			Ok(_h) => print!(""),
			Err(_s) => {
				//println!("Shit went sideways");
				return Err(0)
			}
		}
		start_index = end_index + 3;
	    end_index = find_index(&line[start_index..len], ':',',', ';') - 1 + start_index;	
	    //println!("LEN IS {} END IS {}", len, end_index);
	}
	
	//println!("We are gonna return now");
	Ok(1)
	
}

fn which_color (text: &str) -> Colors {
	if text.contains("red") {
		return Colors::Red
	}
	else if text.contains("blue") {
		return Colors::Blue
	}
	else if text.contains("green") {
		return Colors::Green
	}
	else {
		return Colors::Unknown
	}
}

fn is_reveal_possible(col: Colors, count: i32) -> Result<i32, i32> {
	let count_allowed: i32;
	
	match col {
        Colors::Red => count_allowed = 12,
        Colors::Green => count_allowed = 13,
        Colors::Blue => count_allowed = 14,
        Colors::Unknown => return Err(1),
    }

	//println!("COUNT {} VS ALLOWED {}", count, count_allowed);
	if count > count_allowed {
		println!("This is not allowed");
	    println!("COUNT {} VS ALLOWED {}", count, count_allowed);
		Err(1)
	}
	else {
		Ok(0)
	}

}
