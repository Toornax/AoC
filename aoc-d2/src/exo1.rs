use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct Game {
	game_id: u64,
	grab: Vec<Grab>
}

#[derive(Debug, Clone)]
struct Grab {
	red: u8,
	green: u8,
	blue: u8
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

	let max_red = 12;
	let max_green = 13;
	let max_blue = 14;

	let mut total: u64 = 0;
	for line in contents.lines() {
		let game = parse_game(line.to_string());	
		let mut result = true;
		println!("{:?}", game);

		for grab in game.grab {
			result = result && (grab.red <= max_red && grab.green <= max_green && grab.blue <= max_blue);
		}

		if result {
			total += game.game_id;
		}
	}

	println!("Total: {}", total);

    Ok(())
}

fn parse_game(mut game_str: String) -> Game {	
	let mut game = Game {
		game_id: 0,
		grab: Vec::new()
	};


	let split_game_str: Vec<&str> = game_str.split(|c| c == ':' || c == ';').collect();

	let mut game_id_str = split_game_str[0].to_string();
	game_id_str.retain(|c| c.is_ascii_digit());

	game.game_id = game_id_str.parse::<u64>().unwrap();

	for (i, grab_str) in split_game_str.iter().skip(1).enumerate() {
		let split_grab: Vec<&str> = grab_str.split(',').collect();
		let mut grab = Grab { red: 0, green: 0, blue: 0 };

		for color_nb_str in split_grab {			
			if color_nb_str.contains("red") {
				let mut color_nb_owned = color_nb_str.to_string();
				color_nb_owned.retain(|c| c.is_ascii_digit());
				grab.red = color_nb_owned.parse::<u8>().unwrap();
			}
			
			if color_nb_str.contains("green") {
				let mut color_nb_owned = color_nb_str.to_string();
				color_nb_owned.retain(|c| c.is_ascii_digit());
				grab.green = color_nb_owned.parse::<u8>().unwrap();
			}

			if color_nb_str.contains("blue") {
				let mut color_nb_owned = color_nb_str.to_string();
				color_nb_owned.retain(|c| c.is_ascii_digit());
				grab.blue = color_nb_owned.parse::<u8>().unwrap();
			} 
		}

		game.grab.push(grab);
	}	

	game
}