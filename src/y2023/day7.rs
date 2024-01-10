
use crate::{prelude::*, utils::NomFail};

day!(7);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    strength: u8,
    cards: [u8; 5],
}

impl Hand {
    pub fn from_cards(mut cards: [u8; 5], use_jokers: bool) -> Self {
        // Sort cards
        let mut sorted = cards;
        sorted.sort_unstable();

        // Count duplicates
        let mut state: Option<(u8, usize)> = None;
        let mut dups = [0u8; 5];
        let mut jokers = 0u8;
        for card in sorted {
            let nstate = match state {
                None => (card, 0),
                Some(s @ (c, _)) if c == card => s,
                Some((_, i)) => (card, i+1),
            };

            if use_jokers && card == 11 {
                jokers += 1;
            } else {
                dups[nstate.1] += 1;
            }

            state = Some(nstate);
        }

        // Sort duplicates
        dups.sort_unstable_by(|a, b| a.cmp(b).reverse());

        // Add jokers
        if use_jokers {
            dups[0] += jokers;
        }
        
        // Match strength signatures 
        let strength = match dups {
            [5, 0, 0, 0, 0] => 6,
            [4, 1, 0, 0, 0] => 5,
            [3, 2, 0, 0, 0] => 4,
            [3, 1, 1, 0, 0] => 3,
            [2, 2, 1, 0, 0] => 2,
            [2, 1, 1, 1, 0] => 1,
            [1, 1, 1, 1, 1] => 0,
            _ => panic!("Invalid dups state: {dups:?}"),
        };
        
        // Lower value of jokers
        if use_jokers {
            for card in &mut cards {
                if *card == 11 {
                    *card = 1;
                }
            }
        }

        Self {
            strength,
            cards,
        }
    }
}

pub fn run() -> Result<(), Whatever> {
    let input = whatever!(load_input_bytes(DAY), "Failed to load input");

    let mut hands = whatever!(parser::parse_hands(&input, false).map_err(NomFail::from), "Failed to parse input");
    hands.sort_by_key(|x| x.1);
    let part1: u64 = hands.iter().zip(1u64..).map(|((b, _), i)| i * b).sum();
    println!("Part 1: {part1}");

    // Recalculate using jokers
    for (_, hand) in &mut hands {
        *hand = Hand::from_cards(hand.cards, true);
    }

    // Re-sort
    hands.sort_by_key(|x| x.1);
    let part2: u64 = hands.iter().zip(1u64..).map(|((b, _), i)| i * b).sum();
    println!("Part 2: {part2}");

    Ok(())
}

mod parser {
    use nom::{IResult, combinator::{all_consuming, map, map_opt, cut}, multi::many0, sequence::separated_pair, character::complete::{space1, u64 as take_u64}, number::complete::u8 as take_byte, bytes::complete::take, Finish, error::Error};

    use crate::utils::parser::line;

    use super::Hand;

    fn card_val(card: u8) -> Option<u8> {
        Some(match card {
            b'2'..=b'9' => card - b'0',
            b'T' => 10,
            b'J' => 11,
            b'Q' => 12,
            b'K' => 13,
            b'A' => 14,
            _ => return None,
        })
    }

    fn parse_cards(input: &[u8]) -> IResult<&[u8], [u8; 5]> {
        let (input, mut scards) = take(5u8)(input)?;
        let mut cards = [0u8; 5];
        let mut pcard = cut(map_opt(take_byte, card_val));
        for card in &mut cards {
            (scards, *card) = pcard(scards)?;
        }
        Ok((input, cards))
    }

    pub fn parse_hands(input: &[u8], use_jokers: bool) -> Result<Vec<(u64, Hand)>, Error<&[u8]>> {
        all_consuming(many0(map(line(separated_pair(parse_cards, space1, take_u64)), |(c, b)| (b, Hand::from_cards(c, use_jokers)))))(input).finish().map(|(_, v)| v)
    }
}