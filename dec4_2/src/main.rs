use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    
    let cards = read_in_cards("dec4_input.txt");
    let answer = calculate_shiz(cards);
    println!("We got the answer {}", answer);
}

fn calculate_shiz(cards: Vec<(usize, String, String)>) -> i32 {

	let mut answer: i32 = 0;
	let len = cards.len();
	let mut cards_adjustable = cards;
	for i in 0..len {	
		let (count, matches) = checking_correct(cards_adjustable[i].clone()) ; 
		println!("{} matches {}", count, matches);
		cards_adjustable = adjusting_shit(cards_adjustable, count, matches, i);
		answer += count as i32;
	}
	
	return answer;
}

fn adjusting_shit(cards: Vec<(usize, String, String)>, amount: usize, how_many: usize, position: usize )-> Vec<(usize, String, String)> {

	let mut will_return = cards;
	
	for i in position..=position + how_many {
		let (card_count, win, own) = will_return[i].clone();
		will_return[i] = (card_count + amount, win, own);
	}
	
	return will_return;
}

fn checking_correct(card: (usize, String, String)) -> (usize, usize) {
	
	let (count, mut win_untrim, mut own_untrim) = card;
	
	let own: Vec<_> = own_untrim.trim().split(" ").collect();
	let win = win_untrim.trim();
	
	let num = win.split(" ")
				 .filter(|x| { own.contains(x)})
				 .count();
				 
	return 	(count, num)	 
}

fn read_in_cards(filename: &str) -> Vec<(usize, String, String)> {
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);
	let mut cards = Vec::new();
	
	
	for line in reader.lines() {
		match line {
			Ok(x) => {
				let adjusted = adjust_cards(x); 
				cards.push(adjusted);
			}
			Err(_) => println!("Shit"),
		}	
	}
	
	return cards;
}

fn adjust_cards (line: String) -> (usize, String, String) {
		
	let mut good_shiz = line.split(": ").nth(1).unwrap().split(" | ");
	let win_untrim = good_shiz.next().unwrap().replace("  ", " ");
	let own_untrim = good_shiz.next().unwrap(). replace("  ", " ");
	
	
	return (1, win_untrim, own_untrim);

}
