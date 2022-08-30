//mod elements;
use crate::elements::{Card, CardType, Deck, Suit};

#[derive(Copy, Clone)]
enum TeamSide {
    T1,
    T2,
}

#[derive(Copy, Clone)]
enum TeamPlayer {
    P1,
    P2,
}

struct Prize {
    stack: Vec<Card>,
    scopa_count: usize,
}

struct Team {
    p1_hand: Vec<Card>,
    p2_hand: Vec<Card>,
    prize: Prize,
    points: usize,
}

fn card_sum_check(sum_card: Card, partial_card: Card) -> bool {
    // Returns true if "partial_value" can be used in a sum to get to "sum_value"
    let sum_value = sum_card.value();
    let partial_value = partial_card.value();
    partial_value as f32 != sum_value as f32 / 2.0 && partial_value < sum_value - 1
}
struct Game {
    board: Vec<Card>,
    team_1: Team,
    team_2: Team,
}

impl Game {
    pub fn team<'a>(&self, team_side: TeamSide) -> &Team {
        match team_side {
            TeamSide::T1 => &self.team_1,
            TeamSide::T2 => &self.team_2,
        }
    }

    fn team_mut<'a>(&'a mut self, team_side: TeamSide) -> &'a mut Team {
        match team_side {
            TeamSide::T1 => &mut self.team_1,
            TeamSide::T2 => &mut self.team_2,
        }
    }

    pub fn hand(&mut self, team_side: TeamSide, player: TeamPlayer) -> &Vec<Card> {
        let team = self.team(team_side);
        match player {
            TeamPlayer::P1 => &team.p1_hand,
            TeamPlayer::P2 => &team.p2_hand,
        }
    }

    fn hand_mut(&mut self, team_side: TeamSide, player: TeamPlayer) -> &mut Vec<Card> {
        let team = self.team_mut(team_side);
        match player {
            TeamPlayer::P1 => &mut team.p1_hand,
            TeamPlayer::P2 => &mut team.p2_hand,
        }
    }

    fn board_to_prize(&mut self, card_idx: usize, team_side: TeamSide) {
        let prize_card = self.board.remove(card_idx);
        let len = self.board.len();
        let prize: &mut Prize = &mut self.team_mut(team_side).prize;
        prize.stack.push(prize_card);
        if len == 0 {
            // If this was the last card on board, one scopa point is scored
            prize.scopa_count += 1;
        }
    }

    pub fn play(&mut self, team_side: TeamSide, player: TeamPlayer, index: usize) {
        let hand = self.hand_mut(team_side, player);
        let played_card = hand.remove(index);
        let mut picked_up = false;

        let len = self.board.len();
        match played_card.card_type {
            CardType::Ace => {
                // If the played card is an Ace, then all the cards on board are collected
                let mut old_board: Vec<Card> = self.board.clone();
                self.board.clear();
                self.team_mut(team_side).prize.stack.append(&mut old_board);
                //self.team_mut(team_side).prize.stack.push(played_card);
                picked_up = true;
            }
            _ => {
                // Searches for a matching card
                for i in 0..len {
                    let board_card = &self.board[i];
                    if played_card.value() == board_card.value() {
                        self.board_to_prize(i, team_side);
                        /*self.team_mut(team_side).prize.stack.push(played_card);
                        return;*/
                        picked_up = true;
                        break;
                    }
                }
                // Otherwise searches for a matching sum
                // Should be done properly
                if !picked_up {
                    for i in 0..len {
                        let board_card = self.board[i];
                        // Cards with half the value of the playted card or cards which value isn't less than at least two cannot be used for a sum
                        if card_sum_check(played_card, board_card) {
                            let val_1 = board_card.value();
                            for j in i + 1..len {
                                let board_card_2 = self.board[j];
                                if card_sum_check(played_card, board_card_2) {
                                    let val_2 = val_1 + board_card_2.value();
                                    if val_2 == played_card.value() {
                                        self.board_to_prize(j, team_side);
                                        picked_up = true;
                                        break;
                                    } else if val_2 < played_card.value() {
                                        for k in j + 1..len {
                                            let board_card_3 = self.board[j];
                                            if card_sum_check(played_card, board_card_3) {
                                                let val_3 = val_2 + board_card_3.value();
                                                if val_3 == played_card.value() {
                                                    self.board_to_prize(k, team_side);
                                                    picked_up = true;
                                                    break;
                                                }
                                            }
                                        }
                                        if picked_up {
                                            self.board_to_prize(j, team_side);
                                            break;
                                        }
                                    }
                                }
                            }
                            if picked_up {
                                self.board_to_prize(i, team_side);
                                break;
                            }
                        }
                    }
                }
                if picked_up {
                    self.team_mut(team_side).prize.stack.push(played_card);
                } else {
                    // If no cards were matched, then the played card remains on the board
                    self.board.push(played_card);
                }
            }
        }
    }
}
