use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct Card {
    id: u64,
    winning_num: HashSet<u64>,
    my_num: HashSet<u64>
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;

    let card_list = parse_cards(&content);
	let nb_card = card_list.len();
	let mut nb_card_inst: Vec<u64> = vec![1; nb_card];
	
	for (i, card) in card_list.iter().enumerate() {
        let nb_win = card.winning_num.intersection(&card.my_num).count();

		for j in (i+1)..(i+nb_win+1) {
			nb_card_inst[j] += nb_card_inst[i];
		}
    }

	let total: u64 = nb_card_inst.iter().sum();

    println!("{:?}", total);

    Ok(())
}

fn parse_cards(content: &String) -> Vec<Card> {
    let mut result: Vec<Card> = Vec::new();

    for line in content.lines() {
        let mut card = Card { id: 0, winning_num: HashSet::new(), my_num: HashSet::new() };

        let mut splitted_card = line.split([':', '|']);

        card.id  = splitted_card.next().unwrap().replace("Card", "").trim().parse::<u64>().unwrap(); // Not empy line at the end.
        card.winning_num = splitted_card.next().unwrap().split_ascii_whitespace().map(|num_str| num_str.parse::<u64>().unwrap() ).collect();
        card.my_num = splitted_card.next().unwrap().split_ascii_whitespace().map(|num_str| num_str.parse::<u64>().unwrap() ).collect();

        result.push(card);
    }

    result
}