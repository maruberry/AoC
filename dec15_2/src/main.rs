use std::io::Read;
use std::fs::File;

#[derive(Clone)]
struct Slot {
	label: String,
	lens_t: u32,
}

fn main() {
    println!("WE ARE STARTING THIS SHITSHOW");
    let map = read_shit("day15_input.txt");
    let box_list = box_make(map);
    let answer = calc_shit(box_list);
    
    println!("The final answer is {}", answer);
}

fn calc_shit(boxes: Vec<Vec<Slot>>) -> u32 {
	let mut answer: u32  = 0;
	
	for (i, slot) in boxes.iter().enumerate() {
		for (j, lens) in slot.iter().enumerate() {
			answer += (i as u32 + 1) * (j as u32 + 1) * lens.lens_t;
			println!("i {}, j {}, lens {}", i, j , lens.lens_t);
		}	
	}
	
	answer
}

fn box_make(map: Vec<String>) -> Vec<Vec<Slot>> {
	
	let mut boxes: Vec<Vec<Slot>> = vec![vec![]; 256];
	
	for step in map.iter(){
		let label_cur: String = step.clone().chars().filter(|a| a.is_ascii_alphabetic()).collect();
		
		let mut lens: Slot = Slot{
			label: label_cur.clone(),
			lens_t: 0,
		};
								
		let box_loc = hash(label_cur.as_bytes().to_vec());						

		//println!("box_loc {}, label {}", box_loc, label_cur);
		if step.contains("-"){
			let mut temp: Vec<Slot> = vec![];
			
			for hah in boxes[box_loc as usize].clone().iter(){
				if !hah.label.eq(&label_cur) {
					temp.push(hah.clone());
				}			
			}
			boxes[box_loc as usize] = temp;
		}
		else {
			let mut find: bool = false;
			let num: Vec<u32> = step.clone().chars().filter(|a| a.is_digit(10)).map(|x| x.to_digit(10).unwrap()).collect();			
			lens.lens_t = num[0];	
			
			for (i, lmao) in boxes[box_loc as usize].clone().iter().enumerate() {
				if lmao.label.eq(&label_cur) {
					boxes[box_loc as usize][i] = lens.clone();
					find = true;
				}				
			}
			if !find {
				boxes[box_loc as usize].push(lens);
			}
		}
	}
	
	boxes	
}

fn hash(map: Vec<u8>) -> u32 {
	let mut answer: u32 = 0;

	for c in map.iter() {
		//println!("c = {}", c);
		answer += *c as u32;
		answer *= 17;
		answer %= 256;
	}
	
	answer
	
}

fn read_shit(filename: &str) -> Vec<String> {	
	let mut file = File::open(filename).unwrap();
	let mut buffer = String::new();
	let _ = file.read_to_string(&mut buffer);
	let mut map: Vec<String> = vec![];
	
	for input in buffer.split(",") {		
		map.push(input.to_string());
	}
	
	map
}
