use std::fs::File;
use std::io::Read;


fn main() {
    println!("I skipped the previous day..");
    let (parts, workflow) = read_input("day19_input.txt");
    let answer = calc_shit(parts, workflow);
    println!("the final answer is {}", answer);
}

fn calc_shit(parts: Vec<Vec<u32>>, workflow: Vec<Vec<String>>) -> u32 {
	let accept: usize = workflow.len() + 1;
	let deny: usize = workflow.len();
	let mut loc: usize = 0;
	
	println!("ACCEPT {}, DENY {}", accept, deny);
	
	let mut accept_vec: Vec<Vec<u32>> = vec![];
	
	for el in parts.iter() {
		loc = 0;
		while loc != accept && loc != deny {
			//println!("{}", loc);
			loc = where_next(el.clone(), workflow[loc].clone());
		}	
		if loc == accept {
			accept_vec.push(el.clone());
		}
		if loc == deny {
			continue
		}	
	}
	
	add_shit(accept_vec)
}

fn add_shit (good_part: Vec<Vec<u32>>) -> u32 {
	let mut answer: u32 = 0;
	
	for part in good_part.iter() {
		for el in part.iter() {
			answer += el;
		}
	}
	answer
}

fn where_next(part: Vec<u32>, flow: Vec<String>) -> usize {
	
	let len = flow.len();
	let mut spare: usize = 0;
	//println!("{:?}", flow);
	for (i, el) in flow.iter().enumerate() {
		//println!("{}", el);
		//println!("{}", len);
		if i == len -1 {
			//println!("{:?}", el);
			spare = el.replace(" ", "").parse::<usize>().unwrap();
			return spare;
		}
		let (rest, dest) = el.split_once(":").unwrap();
		let cvec: Vec<char> = rest.chars().collect();
		let comp: u32 = cvec[2..].iter().collect::<String>().parse().unwrap();
		match cvec[1] {
			'<' => {
				//println!("less than");
				if part[cvec[0].to_digit(10).unwrap() as usize] < comp {
					return dest.parse().unwrap()
				}
			}
			'>' => {
				if part[cvec[0].to_digit(10).unwrap() as usize] > comp {
					return dest.parse().unwrap()
				}
			}
			_ => println!("This hsould not be here"),
		}
	}
	
	spare
}

fn read_input(filename: &str) -> (Vec<Vec<u32>>, Vec<Vec<String>>){
	let mut file = File::open(filename).unwrap();
	let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    
    let (mut workflow, parts) = contents.split_once("\n\n").unwrap();
    let binding = num_workflow(workflow.to_string());
    workflow = &binding;
	//println!("{}", workflow);
	let workflow_return = process_workflow(workflow.to_string());
	let parts_return = process_parts(parts.to_string());
	
	(parts_return, workflow_return)
}

fn process_parts (input: String) -> Vec<Vec<u32>> {
	let parts = input.replace("{", "").replace("}", "");
	let mut temp: Vec<u32> = vec![];
	let mut parts_vec: Vec<Vec<u32>> = vec![];
	
	for p in parts.lines() {
		temp = p.split(",").map(|x| x[2..].parse().unwrap()).collect();
		//println!("{:?}", temp);
		parts_vec.push(temp);
	}
	
	
	parts_vec
	
}

fn process_workflow (input: String) -> Vec<Vec<String>>{
	
	let work_vec: Vec<String> = input.lines().map(|s| s.to_string()).collect();
	let line_cnt = work_vec.len();
	let mut work_better: Vec<Vec<String>> = vec![vec![]; line_cnt + 1];
	
	for line in work_vec.clone().iter() {
		let (snum, rules) = line.split_once(" ").unwrap();
		//println!("{:?}",snum);
		let num: usize = snum.to_string().parse().unwrap();
		let temp: Vec<String> = rules.to_string().split(",").map(|s| s.to_string()).collect();
		
		work_better[num] = temp;
	}
	
	work_better
}

fn num_workflow(mut input: String) -> String {
	let mut input_num = input.clone();
	input_num = input_num.replace("in", "0 ")
						 .replace("{", " ; ")
						 .replace("}", " ")
						 .replace(",", " , ")
						 .replace(":", ": ")
						 .replace("\n","\n ");

	let mut cnt: usize = 1;
	//println!("{}", input_num);
	
	input = input.replace("{", " ; ")
				 .replace("}", " ")
				 .replace(",", " , ")
				 .replace(":", ": ")
				 .replace("\n","\n ");
	for line in input.lines(){
		//println!("line")
		let (name, _the_rest) = line.split_once("; ").unwrap();
		if name.eq("in") {
			continue;
		}
		let mut num = cnt.to_string();
		num.push(' ');
		
		//println!("REPLACE {} W {}", name, num);
		input_num = input_num.replace(&name, &num);
		cnt += 1;
	}
	let mut num = cnt.to_string();
	num.push(' ');
	cnt += 1;
	input_num = input_num.replace("R", &num);
	
	num = cnt.to_string();
	num.push(' ');
	
	input_num = input_num.replace("A", &num)
						 .replace("x", "0")
						 .replace("m", "1")
						 .replace("a", "2")
						 .replace("s", "3")
						 .replace(" 0 ", "0")
						 .replace(" , ", ",")
						 .replace(": ", ":")
						 .replace(" ,", ",")
						 .replace(" ,", ",")
						 .replace("; ", "");
	
	input_num
}
