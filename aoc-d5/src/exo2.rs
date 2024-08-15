use std::cmp::min;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashSet;
use std::ops::RangeBounds;
use std::ops::{Range};
use std::cmp;
use std::thread;


#[derive(Debug, Clone)]
struct Almanac {
	seeds: Vec<Range<u64>>,
	maps: Vec<Vec<AlmanacRange>>,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct AlmanacRange {
	src: Range<u64>,
	dest: Range<u64>,
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;

	let almanac = parse_almanac(&content);

	let mut thread_handle = Vec::new();
	for (i, seed_range) in almanac.clone().seeds.iter().enumerate() {
		let almanac_clone = almanac.clone();
		let seed_range_clone = seed_range.clone();
		let result = thread::spawn(move || {
			calculation(seed_range_clone, almanac_clone.maps, format!("{}", i))
		});

		thread_handle.push(result);
	}

	let mut value = u64::MAX;
	for handle in thread_handle {
		let res = handle.join().unwrap();
		if value > res {
			value = res;
		}
	}


	println!("The lowest location is: {:?}", value);

	Ok(())
}

fn calculation(seed_range: Range<u64>, maps: Vec<Vec<AlmanacRange>>, name: String) -> u64 {
	let mut result: u64 = u64::MAX;
	let max_seed = seed_range.end - seed_range.start;
	for seed in seed_range.clone() {			
		let mut val = seed;
		for map in &maps {
			for range in map {
				if range.src.contains(&val) {
					val = range.dest.start + (val - range.src.start);
					break;
				}
			}
		}

		if result > val {
			result = val;
		}

		let seed_count = seed - seed_range.start;
		if seed_count % 100000 == 0 {
			println!("[Thread {}] {} / {} : {}", name, seed_count, max_seed, 100*(seed_count)/max_seed);
		}
	}

	result
}

fn get_ranges(src: Range<u64>, map: Vec<AlmanacRange>) -> Vec<Range<u64>> {


	Vec::new()
}

fn parse_almanac(content: &String) -> Almanac {
	let mut almanac = Almanac { 
		seeds: Vec::new(), 
		maps: Vec::new()
	};

	let mut data: Vec<AlmanacRange> = Vec::new();
	let mut lines = content.lines();
	while let Some(line) = lines.next() {
		if line.contains("seeds:") {
			let vec_seeds: Vec<u64> = line.replace("seeds:", "").trim().split_ascii_whitespace().map(|val| val.parse::<u64>().unwrap()).collect();
			for i in (0..vec_seeds.len()).step_by(2) {	
				almanac.seeds.push(Range { start: vec_seeds[i], end: vec_seeds[i]+vec_seeds[i+1] });
			}
		} else if line.contains("map:") {
			continue;
		} else if line.len() == 0 {
			if !data.is_empty() {
				almanac.maps.push(data);
				data = Vec::new();
			}
		} else {
			let vals: Vec<u64> = line.trim().split_ascii_whitespace().map(|val| val.parse::<u64>().unwrap()).collect();

			data.push(AlmanacRange {
				src: Range { start: vals[1], end: vals[1]+vals[2] },
				dest: Range { start: vals[0], end: vals[0]+vals[2] }
			});
		}
	}

	almanac
}

fn intersect_range(a: Range<u64>, b: Range<u64>) -> Option<Range<u64>> {
	let mut start = 0;
	let mut end = 0;

	if a.start >= b.end || b.start >= a.end { 
		None
	} else {
		Some(Range { start: cmp::max(a.start, b.start), end: cmp::min(a.end, b.end)})
	}
}

fn diff_range(a: Range<u64>, b: Range<u64>) -> Option<Range<u64>>{
	if a.start >= b.end || b.start >= a.end  {
		Some(a)
	} else if a.start < b.start {
		Some(Range { start: a.start, end: b.start })
	} else if a.end > b.end {
		Some(Range { start: b.end, end: a.end })
	} else {
		None
	}
}