use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {
	
	let numbers = vec!["one", "two", "three", "four", "five",
		"six", "seven", "eight", "nine"];
	
    let filename = "input1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut answer: i32 = 0;
    
    for (i, line) in reader.lines().enumerate() {
		
        let line = line.unwrap(); 
        let mut first: char = 'h'; 
        let mut last: char = 'p';
        let mut first_appear: usize = usize::MAX;
        let mut last_appear: usize = usize::MAX;
        
        for number in numbers.clone().into_iter() {
			
			if line.contains(number)
			{
				let location: Vec<_> = line.match_indices(number).collect();
				
				for (index, text) in location.iter() {
					
					if first_appear == usize::MAX
					{
						first_appear = *index;
						last_appear = *index;
						
						match *text {
							"one" => first = '1',
							"two" => first = '2',
							"three" => first = '3',
							"four" => first = '4',
							"five" => first = '5',
							"six" => first = '6',
							"seven" => first = '7',
							"eight" => first = '8',
							"nine" => first = '9',
							 &_ => first = '?',
						}
						
						last = first;
					}				
					else if index > &last_appear
					{
						last_appear = *index;
						
						match *text {
							"one" => last = '1',
							"two" => last = '2',
							"three" => last = '3',
							"four" => last = '4',
							"five" => last = '5',
							"six" => last = '6',
							"seven" => last = '7',
							"eight" => last = '8',
							"nine" => last = '9',
							 &_ => first = '?',
						}
					}
					else if index < &first_appear
					{
						first_appear = *index;
						
						match *text {
							"one" => first = '1',
							"two" => first = '2',
							"three" => first = '3',
							"four" => first = '4',
							"five" => first = '5',
							"six" => first = '6',
							"seven" => first = '7',
							"eight" => first = '8',
							"nine" => first = '9',
							 &_ => first = '?',
						}
					}
				}
			}
		}
        
        for (index, c) in line.chars().enumerate() { 
			
			if c.is_digit(10) == true
			{
				if index < first_appear
				{
					if last_appear == usize::MAX
					{
						last_appear = index;
						last = c;
					}
					
					first = c;
					first_appear = index;
				}
				else if index > last_appear
				{
					last = c;
					last_appear = index;			
				}
			}
		}	
        
        if last.is_digit(10) == true
        {
			let first_num = (first as i32 - 0x30) * 10;			
			let last_num = last as i32 - 0x30;
			let together = first_num + last_num;
			answer += together;
		}
		
		else if first.is_digit(10) == true
		{
			let first_num = first as i32 - 0x30;
			answer += (first_num * 10) + first_num;
		}
    }
    println!("Final total is {}", answer);
}
