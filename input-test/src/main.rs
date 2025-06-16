use console::Term;
use rand::seq::SliceRandom;
use std::cmp::{min, max};

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
struct  Card{
    rank: i8,
    color: i8,

    points: i32,
}

// return the name as a char of a card
fn rank_to_char(rank: i8) -> char {
    match rank {
        14 => 'A',
        13 => 'K',
        12 => 'Q',
        11 => 'J',
        10 => 'X',
        _ => (rank + 48) as u8 as char,
    }
}

// return provided mult of hand
fn hand_type(mut ranks: Vec<i8>, mut colors: Vec<i8>) -> (i32, i32) {
    let mut flush = true;
    let mut col = colors[0];
    for i in 0..5 {
        if col != colors[i] {
            flush = false;
            break;
        }
    }

    ranks.sort();
    let mut sames = 0;
    let mut ori = ranks[0];
    for i in 0..ranks.len() {
        if ori != ranks[i] {
            sames = i;
            break;
        }
    }

    if sames == 5 {
        if flush {
            return (160, 16);
        }
        return (120, 12);
    }
    if sames == 4 {
        return (60, 7);
    }
    if sames == 3 {
        if ranks[4] == ranks[3]{
            if flush {
                return (140, 14);
            }
            return (40, 4);
        }
        return (30, 3);
    }
    if sames == 2 {
        if ranks[2] == ranks[3] || ranks[3] == ranks[4]{
            return (20, 2);
        }
        return (10, 2);
    }
    
    ranks.reverse();
    let mut grad = 0;
    let mut prev = ranks[0];
    for i in 0..5 {
        if prev + 1 != ranks[i] {
            grad = i;
            break;
        }
    }

    if grad == 5 {
        if flush {
            if ranks[0] == 14{
                return (100, 8);
            }
            return (100, 8);
        }
        return (30, 4);
    }

    if flush {
        return (35, 4);
    }
    
    return (5, 1);
}

fn main() {
    let input = Term::buffered_stdout();
    let mut rng = rand::rng();

    const ESC : char = 27u8 as char;
    const ENT : char = 10u8 as char;

    let mut cursor_pos = 0;
    let mut click = false;
    let mut submit = false;

    let mut hand_size = 8;

    let mut hand: Vec<Card> = Vec::new();
    let mut deck: Vec<Card> = Vec::new();
    let mut play: Vec<Card> = Vec::new();

    let mut selected: Vec<bool> = vec![false; hand_size];

    for i in 0..=3 {
        for j in 1..=14 {
            deck.push(Card { rank: (j), color: (i), points: min(j as i32, 11)});
        }
    }
    deck.shuffle(&mut rng);

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        while hand.len() < hand_size && deck.len() > 0{
            let c: Option<Card> = deck.pop();
            if !c.is_none(){
                hand.push(c.unwrap());
            }
        }
        hand.sort();
        hand.reverse();
        
        if let Ok(ch) = input.read_char() {
            match ch {
                'a' => cursor_pos = max(cursor_pos - 1, 0),
                'd' => cursor_pos += 1,
                ' ' => click = true,
                ENT => submit = true,
                _ => (),
            }
        }

        if cursor_pos < 0 {
            cursor_pos = 0;
        }
        if click {
            click = false;

            if cursor_pos < selected.len(){
                selected[cursor_pos] = !selected[cursor_pos];
            }
        }
        if submit{
            submit = false;
            for i in 0..hand.len() {
                if selected[i]{
                    play.push(hand[i]);
                }
            }
            let mut i = 0;
            let mut j = 0;
            let m = hand.len();
            while j < m {
                if selected[j] {
                    hand.remove(i);
                }else {
                    i += 1;
                }
                j += 1;
            }

            for i in 0..selected.len() {
                selected[i] = false;
            }

            let mut color_vec: Vec<i8> = Vec::new();
            let mut rank_vec: Vec<i8> = Vec::new();
            for i in &play {
                color_vec.push(i.color);
                rank_vec.push(i.rank);
            }
             

            let chips_and_mult = hand_type(rank_vec, color_vec);

            let mut chips = chips_and_mult.0;
            let mut mult = chips_and_mult.1;

            for i in &play {
                chips += i.points;
            }

            play.clear();
            println!("({})({}) --> {} ", chips, mult, chips * mult);
        }


        {
            let mut i = 0;
            for c in &hand {
                if selected[i] == true{
                    print!("{}, ", rank_to_char(c.rank));
                }else {
                    print!("   ");
                }
                i += 1;
            }
            println!();
            i = 0;
            for c in &hand {
                if selected[i] == false{
                    print!("{}, ", rank_to_char(c.rank));
                }else {
                    print!("   ");
                }
                i += 1;
            }
        }
        println!();
        for x in 0..cursor_pos {
            print!("   ");
        }
        println!("^");
    }
}
