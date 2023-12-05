use std::fs;

fn main() {
	println!("WE ARE STARTING THIS SHITSHOW");  
	
    let data = fs::read_to_string("dec5_input.txt").unwrap();
    let maps = read_in_maps(data);
    let answer = calc_answer(maps);
    
    println!("The final answer is {}", answer);
}

fn calc_answer(almanac: Vec<String>) -> i64{
	let mut answer: i64 = -1;
	
	for seed in almanac[0]
				.clone()
				.split(" ")
				.map(|x| conversion_therapy(almanac.clone(), x.to_string())) 
	{
		
		//println!("seed {}", seed);
		if answer == -1 { answer = seed }
		else if seed < answer { answer = seed }
	}
	
	return answer
}

fn conversion_therapy(almanac: Vec<String>, seed: String) -> i64{
	
	let seed_num: i64 = seed.parse().unwrap();
	let mut conversion_result: i64 = seed_num;
	
	for conversions in almanac[1..].iter() {
		
		conversion_result = therapy_sesh(conversions, conversion_result);
	}
	
	
	return conversion_result
}

fn therapy_sesh(data: &String, from: i64) -> i64 {
	
	
	for line in data.lines() {
		
		let line_split: Vec<_> = line.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();
		if line_split[1] < from && line_split[1] + line_split[2] > from {
			
			return line_split[0] + (from - line_split[1])
		}		
	}
	
	return from
}

fn read_in_maps(data: String) -> Vec<String> {
	
	let maps: Vec<String> = data
							.split("\n\n")
							.map(str::to_string)
							.collect::<Vec<String>>()
							.iter()
							.map(|x| remove_words(x.clone()))
							.collect();
	

	
	//println!("{}", maps[1]);
	return maps;
}

fn remove_words(data: String) -> String {
	
	let good_shiz = data.split(":\n").nth(1).unwrap();
	
	return good_shiz.to_string();
}
