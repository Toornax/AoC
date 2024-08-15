use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;
use std::ops::{Range};

#[derive(Debug)]
struct Almanac {
	seeds: HashSet<u64>,
	maps: Vec<HashSet<AlmanacRange>>,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct AlmanacRange {
	src: u64,
	dest: u64,
	length: u64,
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;

	let almanac = parse_almanac(&content);

	let mut results: Vec<u64> = Vec::new();
	for seed in almanac.seeds {
		let mut val = seed;
		for map in &almanac.maps {
			for range in map {
				if (range.src..range.src+range.length).contains(&val) {
					val = range.dest + (val - range.src);
					break;
				}
			}
		}

		println!("\n\nFinal pos: {}", val);
		results.push(val);
	}

	println!("The lowest location is: {}", results.iter().min().unwrap());

	Ok(())
}

fn parse_almanac(content: &String) -> Almanac {
	let mut almanac = Almanac { 
		seeds: HashSet::new(), 
		maps: Vec::new()
	};

	let mut data: HashSet<AlmanacRange> = HashSet::new();
	let mut lines = content.lines();
	while let Some(line) = lines.next() {
		if line.contains("seeds:") {
			almanac.seeds = line.replace("seeds:", "").trim().split_ascii_whitespace().map(|val| val.parse::<u64>().unwrap()).collect();
		} else if line.contains("map:") {
			continue;
		} else if line.len() == 0 {
			if !data.is_empty() {
				almanac.maps.push(data);
				data = HashSet::new();
			}
		} else {
			let vals: Vec<u64> = line.trim().split_ascii_whitespace().map(|val| val.parse::<u64>().unwrap()).collect();

			data.insert(AlmanacRange {
				src: vals[1],
				dest: vals[0],
				length: vals[2]
			});
		}
	}

	almanac
}