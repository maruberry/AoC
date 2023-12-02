use std::fs::File;
use std::io::{BufRead, BufReader};


//We are gonna be saving the largest value for each color here
struct Colors {
	red: i64,
	blue: i64,
	green: i64,
}

struct Values {
	value: i64,
	color: Options,
}

enum Options {
	RedOption,
	BlueOption,
	GreenOption,
}


fn main() {
    
    let filename = "dec2_input.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut final_answer: i64 = 0;
    println!("WE ARE STARTING THIS SHITSHOW");
    
    for line in reader.lines(){
		match line {
			Ok(power) => final_answer += calculate_line(power),
			Err(_f) => println!("No more lines left"),
		}
		
	}
	println!("The final result is {}", final_answer);
}


fn calculate_line(line: String) -> i64 {
	
	let mut color_values = Colors { //Holding the biggest values per color, per line here
		red: 0,
		blue: 0,
		green: 0,	
	};
	let mut good_line: &str = "";
	let line_power: i64;
	
	let discard = line.split(':'); //removing the first part of the line we don't need
	for (i, things) in discard.enumerate() {
		if i == 1 {
			good_line = things;
		}
	}
		
	let splitter_reveals = good_line.split(';');	// splitting up based on reveal
	for parts in splitter_reveals {
		let compare_values = split_color(parts); 
		color_values = which_is_larger(color_values, compare_values);
	}
	
	//println!("This biggest values in this line are: {} {} {}", color_values.green, color_values.blue, color_values.red);
	line_power = color_values.green * color_values.blue * color_values.red;
	
	return line_power;
}

//
fn split_color(reveal: &str) -> Colors {
	
	let mut fuck_i_dunno: Values;
	
	let mut biggest_colors =  Colors { //Holding the color values we have gotten from the "reveal"
		red: 0,
		blue: 0,
		green: 0,
	
	};
	
	let splitter_colors = reveal.split(','); //splitting up based on color
	for colors in splitter_colors {
		fuck_i_dunno = split_amount(&colors[1..]); //Splitting the color and the amount and putting the result in a struct
		biggest_colors = assign_values(biggest_colors, fuck_i_dunno); //Assigning the values we've gotten
	}
	
	return biggest_colors;
	
}


fn split_amount(colors: &str) -> Values {
	
	//println!("THIS: {}", colors);
	let mut saving_shit_here = Values{
		value: 0,
		color: Options::RedOption,
		
	};
	
	let splitter_amount = colors.split(' '); //splitting up the amount and the color
	for amount in splitter_amount {					
		let check = amount.parse::<i64>(); //Checking if what we got is a number
		match check{
			Ok(f) => {
				//println!("WE HAVE A NUMBER {} ALSO CHECK {}", amount, f);
				saving_shit_here.value = f;
			}
			Err(_s) => {
				//println!("We should have a color {} also check {}", amount, _s);
				saving_shit_here.color = which_color(amount);
			}
		}
	}
	

	
	return saving_shit_here;
}

fn which_is_larger(biggy: Colors, compare: Colors) -> Colors {
	
	let mut biggest = biggy;
	
	if biggest.red < compare.red {
		biggest.red = compare.red;
	}
	
	if biggest.blue < compare.blue {
		biggest.blue = compare.blue;
	}
	
	if biggest.green < compare.green {
		biggest.green = compare.green;
	}
	
	return biggest;
}

fn assign_values(biggy: Colors, compare: Values) -> Colors {
	
	let mut biggest = biggy;
	
	match compare.color {
		
		Options::BlueOption => {
			if compare.value > biggest.blue {
				biggest.blue = compare.value;
			}
		}
		Options::GreenOption => {
			if compare.value > biggest.green {
				biggest.green = compare.value;
			}
		}
		Options::RedOption => {
			if compare.value > biggest.red {
				biggest.red = compare.value;
			}
		}
	}
	
	return biggest;
}

fn which_color(text: &str) -> Options {
	
	//println!("Checking for this color {}", text);
	match text {	
		"red" => return Options::RedOption,
		"green" => return Options::GreenOption,
		"blue" => return Options::BlueOption,	
		&_ => panic!("Unknown color!!"),
	}
}
