use std::fs;

struct Cards {
	
	five_kind: Vec<Vec<String>>,
	four_kind: Vec<Vec<String>>,
	full_house: Vec<Vec<String>>,
	three_kind: Vec<Vec<String>>,
	two_pair: Vec<Vec<String>>,
	one_pair: Vec<Vec<String>>,
	high_card: Vec<Vec<String>>,
	
}

impl Cards {
	
	fn sort_section(&mut self) {
		
		self.five_kind = (&self.five_kind[1..]).to_vec();
		self.five_kind.sort_by(|a, c| {
						(u32::from_str_radix(&a[0], 16)).expect("AHHHH")
						.cmp(&(u32::from_str_radix(&c[0], 16)).expect("AHHHH"))
					});
		self.four_kind = (&self.four_kind[1..]).to_vec();
		self.four_kind.sort_by(|a, c| {
						(u32::from_str_radix(&a[0], 16)).expect("AHHHH")
						.cmp(&(u32::from_str_radix(&c[0], 16)).expect("AHHHH"))
					});
		self.three_kind = (&self.three_kind[1..]).to_vec();
		self.three_kind.sort_by(|a, c| {
						(u32::from_str_radix(&a[0], 16)).expect("AHHHH")
						.cmp(&(u32::from_str_radix(&c[0], 16)).expect("AHHHH"))
					});
		self.full_house = (&self.full_house[1..]).to_vec();
		self.full_house.sort_by(|a, c| {
						(u32::from_str_radix(&a[0], 16)).expect("AHHHH")
						.cmp(&(u32::from_str_radix(&c[0], 16)).expect("AHHHH"))
					});
		self.one_pair = (&self.one_pair[1..]).to_vec();
		self.one_pair.sort_by(|a, c| {
						(u32::from_str_radix(&a[0], 16)).expect("AHHHH")
						.cmp(&(u32::from_str_radix(&c[0], 16)).expect("AHHHH"))
					});
					
		self.two_pair = (&self.two_pair[1..]).to_vec();
		self.two_pair.sort_by(|a, c| {
						(u32::from_str_radix(&a[0], 16)).expect("AHHHH")
						.cmp(&(u32::from_str_radix(&c[0], 16)).expect("AHHHH"))
					});
		self.high_card = (&self.high_card[1..]).to_vec();
		self.high_card.sort_by(|a, c| {
						(u32::from_str_radix(&a[0], 16)).expect("AHHHH")
						.cmp(&(u32::from_str_radix(&c[0], 16)).expect("AHHHH"))
					});
							
		
	}
	
	fn default() -> Cards {
		Cards {
			five_kind: vec!(vec!("".to_string())),
			four_kind: vec!(vec!("".to_string())),
			full_house: vec!(vec!("".to_string())),
			three_kind: vec!(vec!("".to_string())),
			two_pair: vec!(vec!("".to_string())),
			one_pair: vec!(vec!("".to_string())),
			high_card: vec!(vec!("".to_string())),
		}
	}
}


fn main() {
	println!("WE ARE STARTING THIS SHITSHOW");
    let data = fs::read_to_string("day7_input.txt")
					.unwrap()
					.replace("A", "E")
					.replace("K", "D")
					.replace("Q", "C")
					.replace("J", "1")
					.replace("T", "A");
   
    let hands = input_to_vec(data);
    let answer = assign_points(hands);
    
    println!("Final answer is {}", answer);
    
}

fn assign_points(hands: Cards) -> u32{
	
	let len_vec: Vec<_> = vec![	
		hands.high_card.len(),
		hands.one_pair.len(),
		hands.two_pair.len(),
		hands.three_kind.len(),
		hands.full_house.len(),
		hands.four_kind.len(),
		hands.five_kind.len()
		];
	
	let card_vec = vec![
		hands.high_card,
		hands.one_pair,
		hands.two_pair,
		hands.three_kind,
		hands.full_house,
		hands.four_kind,
		hands.five_kind
	];
	
	let mut multiplier_count = 0;
	let mut answer: u32 = 0;
	
	for (j, parts) in card_vec.iter().enumerate() {
		
	//	if parts.len() != 0 {
			for i in 0..len_vec[j] {
				multiplier_count += 1;
				println!("{} X {}", multiplier_count, parts[i][1]);
				answer += multiplier_count * parts[i][1].parse::<u32>().unwrap()
			}
		
	}
	
	return answer;
}

fn input_to_vec (input: String) -> Cards{
	
	//let ffs: Vec<_> = input.lines().map(str::to_string).collect();	
	let mut lines = Vec::new();
	
	for shit in input.lines(){
		lines.push(shit.to_string().split(" ").map(str::to_string).collect());
	}
	let hands: Cards = what_hand(lines);
	
	return hands;
}

fn what_hand (input: Vec<Vec<String>>) -> Cards {

		let mut match_cnt: Vec<usize>  = Vec::new();
		let mut hand_types: Cards = Cards::default();	
			
		for hands in input.iter() {
			for i in 2..=0xE {
				let mut hex_string = format!("{:x}", i);
				hex_string.make_ascii_uppercase();
				//println!("Looking for {}", hex_string);
				let v = hands[0].matches(&hex_string).collect::<Vec<_>>().len();
				if v > 1 {
					//println!("Found {} same in {}. Matching was {}", v, hands[0], hex_string);
					match_cnt.push(v);
				}
			}
			
			println!("{} - {:?}", hands[0], match_cnt);
			let len = match_cnt.len();
			hand_types = what_hand_push(hand_types, match_cnt.clone(), len, hands.clone());
		
			match_cnt.clear();
		}
		//println!("{:?}", hand_types.full_house);
		hand_types.sort_section();
		println!("FULL HOUSE{:#?}", hand_types.full_house);
		
		return hand_types;
}

fn what_hand_push (mut hand_types: Cards, match_cnt: Vec<usize>, len: usize, hands: Vec<String>) -> Cards {
	
	if len == 0 {
		hand_types.high_card.push(hands.clone());
	}	
	else if len == 2 && (match_cnt[0] == 3 || match_cnt[1] == 3 ){
		hand_types.full_house.push(hands.clone());			
	}
	else if len == 2 && (match_cnt[0] == 2 && match_cnt[1] == 2 ){
		hand_types.two_pair.push(hands.clone());			
	}
	else if len == 1 && match_cnt[0] == 2 {
		hand_types.one_pair.push(hands.clone());			
	}
	else if len == 1 && match_cnt[0] == 3 {
		hand_types.three_kind.push(hands.clone());			
	}
	else if len == 1 && match_cnt[0] == 4 {
		hand_types.four_kind.push(hands.clone());			
	}
	else if len == 1 && match_cnt[0] == 5 {
		hand_types.five_kind.push(hands.clone());			
	}
	
	
	return hand_types;
}
