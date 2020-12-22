use std::collections::{HashSet, VecDeque};
use util::res::Result;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Decks {
    player1: VecDeque<u64>,
    player2: VecDeque<u64>,
}

impl Decks {
    fn from_input(contents: &String) -> Result<Self> {
        let mut out = Decks{ player1: VecDeque::new(), player2: VecDeque::new() };
        let mut parsing_player_1 = true;
        for line in contents.lines().filter(|l| !l.starts_with("Player")) {
            if line == "" { parsing_player_1 = false; continue; }
            (if parsing_player_1 { &mut out.player1 } else { &mut out.player2 }).push_back(line.parse::<u64>()?);
        }

        Ok(out)
    }

    fn get_next_cards(&mut self) -> (u64, u64) {
        (self.player1.pop_front().unwrap(), self.player2.pop_front().unwrap())
    }

    fn move_cards_based_on_winner(&mut self, player1_card: u64, player2_card: u64, winner: Option<bool>) {
        match winner {
            Some(true) => { // There was a winner, and it was player 1
                self.player1.push_back(player1_card);
                self.player1.push_back(player2_card);
            },
            Some(false) => { // There was a winner, and it wasn't player 1
                self.player2.push_back(player2_card);
                self.player2.push_back(player1_card);
            },
            None => { // There was no winner
                self.player1.push_back(player1_card);
                self.player2.push_back(player2_card);
            },
        }
    }

    fn determine_winner_standard(player1_card: u64, player2_card: u64) -> Option<bool> {
        match player1_card.cmp(&player2_card) {
            std::cmp::Ordering::Less => Some(false),
            std::cmp::Ordering::Greater => Some(true),
            std::cmp::Ordering::Equal => None,
        }
    }

    fn standard_round(&mut self) {
        let (player1_card, player2_card) = self.get_next_cards();
        self.move_cards_based_on_winner(
            player1_card, player2_card, 
            Self::determine_winner_standard(player1_card, player2_card)
        );
    }

    fn play_game_standard(&self) -> Self {
        let mut out = self.clone();
        while out.player1.len() != 0 && out.player2.len() != 0 { out.standard_round(); }
        out
    }

    // Returns the resulting decks, and true if player 1 won or false if player 2 won
    fn play_game_recursive(&self) -> (Self, bool) {
        let mut deck_memory: HashSet<Decks> = HashSet::new();
        let mut out = self.clone();

        // Play the actual game
        while out.player1.len() != 0 && out.player2.len() != 0 {
            if deck_memory.contains(&out) { return (out, true) } // No repeat state
            deck_memory.insert(out.clone());

            let (player1_card, player2_card) = out.get_next_cards();
            let should_recurse = player1_card as usize <= out.player1.len() && player2_card as usize <= out.player2.len();
            out.move_cards_based_on_winner(player1_card, player2_card, if should_recurse {
                Some(Decks{
                    player1: out.player1.iter().take(player1_card as usize).cloned().collect::<VecDeque<u64>>(),
                    player2:out.player2.iter().take(player2_card as usize).cloned().collect::<VecDeque<u64>>(),
                }.play_game_recursive().1)
            } else {
                Self::determine_winner_standard(player1_card, player2_card)
            });
        }

        let player1_won = out.player1.len() != 0;
        (out, player1_won)
    }

    fn calculate_winning_score(&self) -> u64 {
        let winning = if self.player1.len() != 0 { &self.player1 } else { &self.player2 };
        winning.iter().rev().enumerate().fold(0u64, |acc, (idx, val)| acc + (idx as u64 + 1) * val)
    }
}

fn part1(decks: &Decks) {
    let played = decks.play_game_standard();
    println!("[Part 1] Winning score: {}", played.calculate_winning_score());
}

fn part2(decks: &Decks) {
    let (played, _) = decks.play_game_recursive();
    println!("[Part 2] Winning score: {}", played.calculate_winning_score());
}

fn main() -> Result<()> {
    let file_path = util::file::get_input_file_path();
    let contents = util::file::read_to_string(file_path)?;
    let decks = Decks::from_input(&contents)?;

    part1(&decks);
    part2(&decks);
    Ok(())
}