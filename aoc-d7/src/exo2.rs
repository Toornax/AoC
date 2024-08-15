use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
	CardA = 14,
	CardK = 13,
	CardQ = 12,
	CardT = 10,
	Card9 = 9,
	Card8 = 8,
	Card7 = 7,
	Card6 = 6,
	Card5 = 5,
	Card4 = 4,
	Card3 = 3,
	Card2 = 2,
	CardJ = 1
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
	FiveOfAKind = 7,
	FourOfAKind = 6,
	FullHouse = 5,
	ThreeOfAKind = 4,
	TwoPair = 3,
	OnePair = 2,
	HighCard = 1
}

#[derive(Debug, Clone, Copy, Eq)]
struct Hand {
	hand_type: HandType,
	cards: [Card; 5],
	bid: u64
}


fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader.read_to_string(&mut content)?;

	let mut hands = parse_hands(&content);
	hands.sort();

	let total: u64 = hands.iter().enumerate().map(|(i, h)| h.bid * (1 + i as u64)).sum();

	// println!("The hands order is: {:?}\n\n with a total of {}", hands, total);

	Ok(())
}

fn parse_hands(content: &String) -> Vec<Hand> {
	let mut result: Vec<Hand> = Vec::new();

	for line in content.lines() {
		let mut cards = [Card::Card2; 5];

		let mut hand_iter = line.split_ascii_whitespace();
		for (i, card) in hand_iter.next().unwrap().chars().enumerate() {
			cards[i] = Card::from(card);
		}

		let bid = hand_iter.next().unwrap().parse::<u64>().unwrap();

		result.push(Hand::new(cards, bid));
	}

	result
}

impl Hand {
	fn new(cards: [Card; 5], bid: u64) -> Self {	

		let mut hand_type: HandType = HandType::HighCard;

		'outer: for card in &cards {
			if cards.iter().filter(|&n| card == n).count() == 5 {
				hand_type = HandType::FiveOfAKind;
				println!("Cards: {:?} -> HandType: {:?}", cards, hand_type);	

				break
			} else if cards.iter().filter(|&n| card == n).count() == 4 {
				if cards.iter().filter(|&n| card != n && n == &Card::CardJ).count() == 1 {
					hand_type = HandType::FiveOfAKind;

				} else {
					hand_type = HandType::FourOfAKind;

				}
				break
			} else if cards.iter().filter(|&n| card == n).count() == 3 {
				let mut others = cards.iter().filter(|&n| card != n);
				let nb_joker = others.clone().filter(|&c| c == &Card::CardJ).count();	

				if nb_joker == 2 {
					hand_type = HandType::FiveOfAKind;
					
				} else if nb_joker == 1 {
					hand_type = HandType::FourOfAKind;

				} else {
					if others.next() == others.next() {
						hand_type = HandType::FullHouse;
					} else {
						hand_type = HandType::ThreeOfAKind;
					}
				}				
				break
			} else if cards.iter().filter(|&n| card == n).count() == 2 {
				let others = cards.iter().filter(|&n| card != n);
				let nb_joker = others.clone().filter(|&c| c == &Card::CardJ).count();	

				if nb_joker == 3 {
					hand_type = HandType::FiveOfAKind;

					break;
				} else if nb_joker == 2 {
					hand_type = HandType::FourOfAKind;

					break;					
				} else if nb_joker == 1 {
					let mut others_wo_joker = others.filter(|&n| n != &Card::CardJ);
					if others_wo_joker.next() == others_wo_joker.next() {
						hand_type = HandType::FullHouse;
					} else {
						hand_type = HandType::ThreeOfAKind;
					}

					break;						
				} else {
					for other_card in others.clone() {
						if others.clone().filter(|&n| other_card == n).count() == 2 {
							hand_type = HandType::TwoPair;
							break 'outer;
						}
					}
				}

				hand_type = HandType::OnePair;
			}
		}

		Self { cards: cards, bid: bid, hand_type: hand_type }
	}
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {		
        if self.hand_type == other.hand_type {
			self.cards.cmp(&other.cards)
		} else {
			self.hand_type.cmp(&other.hand_type)
		}
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type && self.cards == other.cards
    }
}

impl From<char> for Card {
	fn from(text: char) -> Self {
		match text {
			'A' => Card::CardA,
			'K' => Card::CardK,
			'Q' => Card::CardQ,
			'J' => Card::CardJ,
			'T' => Card::CardT,
			'9' => Card::Card9,
			'8' => Card::Card8,
			'7' => Card::Card7,
			'6' => Card::Card6,
			'5' => Card::Card5,
			'4' => Card::Card4,
			'3' => Card::Card3,
			'2' => Card::Card2,
			_ => Card::Card2
		}
	}
}