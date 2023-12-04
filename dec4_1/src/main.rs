use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {    
    let answer = win_vs_own("dec4_input.txt");
    
    println!("The final answer is {}", answer);
    
}

fn win_vs_own(filename: &str) -> i32{
	
	let mut answer: i32 = 0;
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);
	
	for (i, line) in reader.lines(). enumerate(){	
		//print!("{} ",i + 1);			
		match line {
			Ok(x) => {answer += split_shit(x);}
			Err(_) => println!("Shit"),
		}
	}	
	
	return answer
}

fn split_shit(line: String) -> i32 {
	let mut good_shiz = line.split(": ").nth(1).unwrap().split(" | ");
	let win_untrim = good_shiz.next().unwrap().replace("  ", " ");
	let own_untrim = good_shiz.next().unwrap(). replace("  ", " ");
	
	let win = win_untrim.trim();
	let own = own_untrim.trim();
	
	//println!("{}           vs         {}", win, own);
	
	let num = win.split(" ").filter(|x| own.contains(x)).count() as u32;
	//println!("Found {}", num);
	if num != 0 {
		println!("2 to the power of {} is {} ", num - 1, 2_i32.pow(num - 1));
		return 2_i32.pow(num - 1);
	}
	//println!("");
	return 0;
}
