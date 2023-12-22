use pathfinding::prelude::dijkstra;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32, Dir, i32);

#[derive(PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Clone)]
enum Dir {
	Up, 
	Down, 
	Left, 
	Right, 
	None,
}

impl Pos {
	fn successors(&self, map: &Vec<Vec<usize>>) -> Vec<(Pos, usize)> {
		let Pos(x, y, from, count) = self;
		
		let wide: i32 = map[0].len().try_into().unwrap();
		let len: i32 = map.len().try_into().unwrap();
		
		let mut return_map: Vec<(Pos, usize)> = vec![];
		
		let mut nup: bool = true;
		let mut ndown: bool = true;
		let mut nleft: bool = true;
		let mut nright: bool = true;
		let mut cnt: i32 = 0;
		
		if *x == len -1 && *y == wide - 1 && *count > 2{
			return vec![(Pos(len - 1, wide -1, Dir::None, 0), 0)]
		}
		
		if *count == 9{
			match *from {
				Dir::Down => ndown = false,
				Dir::Left => nleft = false,
				Dir::Right => nright = false,
				Dir::Up => nup = false,
				Dir::None => println!("START NODE"),
			}		
		}		
		else if *count < 3 {
			match *from {
				Dir::Down => {
					nleft = false; 
					nright = false;
					},
				Dir::Left => {
					nup = false; 
					ndown = false;
					},
				Dir::Up => {
					nleft = false; 
					nright = false;
					},
				Dir::Right => {
					nup = false; 
					ndown = false;
					},				
				Dir::None => println!("START NODE"),
			}	
		}
		
		match *from {
				Dir::Down => nup = false,
				Dir::Left => nright = false,
				Dir::Right => nleft = false,
				Dir::Up => ndown = false,
				Dir::None => println!("START NODE"),
			}	
		
		
		if *x != 0 && nup{	
			if *from == Dir::Up {cnt = count + 1;}
			else {cnt = 0;}
				
			return_map.push((Pos(*x - 1, *y, Dir::Up, cnt),map[(*x -1) as usize][*y as usize]));
		}
		if *y != 0 && nleft{
			if *from == Dir::Left {cnt = count + 1;}
			else {cnt = 0;}
			return_map.push((Pos(*x,*y - 1, Dir::Left, cnt), map[*x as usize][(*y - 1) as usize]));
		}
		if *y != wide - 1 && nright{
			if *from == Dir::Right {cnt = count + 1;}
			else {cnt = 0;}
			return_map.push((Pos(*x,*y + 1, Dir::Right, cnt), map[*x as usize][(*y + 1) as usize]));
		}
		if *x != len - 1 && ndown{
			if *from == Dir::Down {cnt = count + 1;}
			else {cnt = 0;}
			return_map.push((Pos(*x + 1, *y, Dir::Down, cnt), map[(*x + 1) as usize][*y as usize]));
		}
		
		return_map
	}
}

fn main() {
    println!("HERE WE GO, THIS FEELS LIKE CHEATING BUT I'D NEVER FIGURE THIS SHIT OUT MESELF!!");
    
	let map = read_map("day17_input.txt");
	
	//debug(map.clone());
	let wide: i32 = map[0].len().try_into().unwrap();
	let len: i32 = map.len().try_into().unwrap();
	
	let goal: Pos = Pos(len - 1, wide - 1, Dir::None, 0);
	let result = dijkstra(&Pos(0, 0, Dir::None, 0), |p| p.successors(&map), |p| *p == goal);

    let (a, f) = result.expect("No result?");
    println!("RESULT {:?}", f);
 
	
}

fn debug (og_map: Vec<Vec<usize>>) {
	for l in og_map.iter() {
		for k in l.iter() {
			print!("{}", k);
		}
		println!("");
	}	
	println!("");
}

fn read_map(filename: &str) -> Vec<Vec<usize>> {
	let file = File::open(filename).unwrap();
	let reader = BufReader::new(file);
	let mut map: Vec<Vec<usize>> = vec![];
	
	for lines in reader.lines() {
		map.push(lines.expect("FUCKED").chars().map(|x| x.to_digit(10).expect("NOT a number") as usize).collect());		
	}
	
	
	map
}
