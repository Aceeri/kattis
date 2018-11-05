use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};
use std::ops::RangeInclusive;
use std::sync::mpsc::channel;
use std::thread;

#[derive(Copy, Clone, Debug)]
pub struct BitSet([u64; 16]);
impl BitSet {
	fn new() -> Self {
		BitSet([0; 16])
	}

	#[inline]
	pub fn set(&mut self, index: usize) {
		self.0[index / 64] |= 1 << index;
	}

	pub fn contains(&self, index: usize) -> bool {
		(self.0[index / 64] & 1 << index) > 0
	}

	pub fn any_similar(&self, bitset: &BitSet) -> bool {
		for index in 0..16 {
			if self.0[index] & bitset.0[index] > 0 {
				return true;
			}
		}

		false
	}
}

pub struct Hopper {
	array: Vec<i32>,
	jump: usize,
	diff: i32,
	cache: Box<[Vec<Cache>]>,
}

pub enum Cache {
	Unsearched(u16), // A found potential jump, but unsearched.
	Searched(BitSet, u16), // A found and searched jump.
}

impl Hopper {
	pub fn new(array: Vec<i32>, jump: usize, diff: i32) -> Self {
		Hopper {
			array: array,
			jump: jump,
			diff: diff,
			//cache: vec![([BitSet::new(); 1000], [0u16; 1000]); 1000].into_boxed_slice(),
			cache: vec![Vec::new(); 1000].into_boxed_slice(),
		}
	}

	pub fn in_cache(
		&self,
		current_path: &BitSet,
		current_length: u16,
		possibility: usize,
	) -> Option<u16> {
		let mut longest = None;
		for (path, length) in &self.cache[possibility] {
			if current_length + length > self.array.len() as u16 {
				continue;
			}

			if !current_path.any_similar(&path) {
				match longest {
					None => longest = Some(*length),
					Some(longest_length) if longest_length < *length => longest = Some(*length),
					_ => (),
				}
			}
		}

		longest
	}

	pub fn cache(&mut self, index: usize, sub: usize) {
		let new_cache = self.cache[sub]
			.iter()
			.map(|cache_point| {
				
				let mut new_path = path.clone();
				new_path.set(index);
				(sub, new_path, length + 1)
			})
			.collect::<Vec<_>>();

		self.cache[index].extend(new_cache);
	}

	//pub fn cache_self(&mut self, index: usize) {
		//let mut bitset = BitSet::new();
		//bitset.set(index);
		//self.cache[index].push((bitset, 1))
	//}

	pub fn longest_branch(&mut self, mut current_path: BitSet, index: usize, length: u16) -> u16 {
		current_path.set(index);

		let mut max = 0;
		for possibility in self.immediate_bounds(index) {
			if current_path.contains(possibility) || !self.in_range(index, possibility) {
				continue;
			}

			for (jump, path, length) in &self.cache[possibility] {
				
			}

			let possible_length = match self.in_cache(&current_path, length, possibility) {
				Some(cached_length) => cached_length,
				None => {
					let result = self.longest_branch(current_path.clone(), possibility, length + 1);

					//build our cache from the sub-branch.
					self.cache(index, possibility);
					result
				}
			};

			if max < possible_length {
				max = possible_length;
			}

			//if max < result {
			//max = result;
			//if index == 0 {
			//println!("new max: {:?}", max);
			//}
			//}
		}

		max + 1
	}

	#[inline]
	pub fn in_range(&self, current: usize, next: usize) -> bool {
		(self.array[current] - self.array[next]).abs() <= self.diff
	}

	// Get the range of indices around the index that are within the array.
	pub fn immediate_bounds(&self, index: usize) -> RangeInclusive<usize> {
		let start_index = index.saturating_sub(self.jump);
		let mut end_index = index + self.jump;

		if end_index >= self.array.len() {
			end_index = self.array.len() - 1;
		}

		start_index..=end_index
	}
}

fn main() -> Result<(), Box<Error>> {
	let (mut hopper, length) = {
		let stdin = stdin();
		let lines = stdin
			.lock()
			.lines()
			.map(|line| line.unwrap())
			.collect::<Vec<String>>();
		let words = lines[0].split_whitespace().collect::<Vec<&str>>();
		let array = lines[1]
			.split_whitespace()
			.map(|n| n.parse::<i32>())
			.collect::<Result<Vec<i32>, _>>()?;

		let maximum_jump = words[1].parse::<usize>()?;
		let maximum_diff = words[2].parse::<i32>()?;
		let length = array.len();
		if length > 1000 {
			panic!("Unequipped to handle lengths of {}", length);
		}

		let mut hopper = Hopper::new(array, maximum_jump, maximum_diff);

		(hopper, length)
	};

	let mut max = 0;
	for n in 0..length {
		println!("attempting {}", n);
		let result = hopper.longest_branch(BitSet::new(), n, 1);
		if max < result {
			max = result;
		}

		if result == length as u16 {
			break;
		}
	}

	stdout().write((max.to_string() + "\n").as_bytes())?;
	Ok(())
}
