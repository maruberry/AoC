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
		let mut joker_cnt: usize = 0;
			
		for hands in input.iter() {
			joker_cnt = hands[0].matches("1").collect::<Vec<_>>().len();
			
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
			
			//println!("{} - {:?}", hands[0], match_cnt);		
			hand_types = what_hand_push(hand_types, match_cnt.clone(), joker_cnt, hands.clone());
			joker_cnt = 0;
			match_cnt.clear();
		}
		//println!("{:?}", hand_types.full_house);
		hand_types.sort_section();
		//println!("FOUR KIND {:#?}", hand_types.four_kind);
		
		return hand_types;
}

//~ fn joker_type_adjust (num_type: u32, joker_cnt: u32, hand: Vec<String>, mut hand_types: Cards, ) -> Cards {
	
	
	
//~ }

fn what_hand_push (mut hand_types: Cards, match_cnt: Vec<usize>, joker_cnt: usize, hands: Vec<String>) -> Cards {
	
	let len = match_cnt.len();
	let mut hand_type: u32 = 0;
	let mut temp: Cards = hand_types;

	if len == 0 {

		hand_type = 1;
	}	
	else if len == 2 && (match_cnt[0] == 3 || match_cnt[1] == 3 ){
		
		hand_type = 5;			
	}
	else if len == 2 && (match_cnt[0] == 2 && match_cnt[1] == 2 ){
			
		hand_type = 3;					
	}
	else if len == 1 && match_cnt[0] == 2 {
		
		hand_type = 2;		
	}
	else if len == 1 && match_cnt[0] == 3 {
		
		hand_type = 4;			
	}
	else if len == 1 && match_cnt[0] == 4 {	
		hand_type = 6;		
	}
	else if len == 1 && match_cnt[0] == 5 {
		hand_type = 7;		
	}
	
	// 1 = high card
	// 2 = one pair
	// 3 = two pair
	// 4 = three kind
	// 5 = full house
	// 6 = four kind
	// 7 = five kind
	print!("{} {}",hands[0], hand_type);
	if joker_cnt == 0 {
		if hand_type == 1 {temp.high_card.push(hands.clone()); hand_type = 1;}
		else if hand_type == 2 {temp.one_pair.push(hands.clone()); hand_type = 2;}
		else if hand_type == 3 {temp.two_pair.push(hands.clone()); hand_type = 3;}
		else if hand_type == 4 {temp.three_kind.push(hands.clone()); hand_type = 4;}
		else if hand_type == 5 {temp.full_house.push(hands.clone()); hand_type = 5;}
		else if hand_type == 6 {temp.four_kind.push(hands.clone()); hand_type = 6;}
		else if hand_type == 7 {temp.five_kind.push(hands.clone()); hand_type = 7;}
	}
	else if joker_cnt == 1 {
		if hand_type == 1 {temp.one_pair.push(hands.clone()); hand_type = 2;}
		else if hand_type == 2 {temp.three_kind.push(hands.clone()); hand_type = 4;}
		else if hand_type == 3 {temp.full_house.push(hands.clone()); hand_type = 5;}
		else if hand_type == 4 {temp.four_kind.push(hands.clone()); hand_type = 6;}
		else if hand_type == 6 {temp.five_kind.push(hands.clone()); hand_type = 7;}	
	}
	else if joker_cnt == 2 {
		//print!{"->"};
		if hand_type == 1 {temp.three_kind.push(hands.clone()); hand_type = 4;}
		else if hand_type == 2 {temp.four_kind.push(hands.clone()); hand_type = 6;}
		else if hand_type == 4 {temp.five_kind.push(hands.clone()); hand_type = 7;}
	}
	else if joker_cnt == 3 {
		if hand_type == 1 {temp.four_kind.push(hands.clone()); hand_type = 6;}
		else if hand_type == 2 {temp.five_kind.push(hands.clone()); hand_type = 7;}
	}
	else if joker_cnt == 4 {
		if hand_type == 1 {temp.five_kind.push(hands.clone()); hand_type = 7;}
	}
	else if joker_cnt == 5 {
		if hand_type == 1 {temp.five_kind.push(hands.clone()); hand_type = 7;}
	}
	print!(" -> {} joke count {}\n", hand_type, joker_cnt);
	hand_types = temp;
	
	//println!("{:#?}", hand_types);	
	
	return hand_types;
}
