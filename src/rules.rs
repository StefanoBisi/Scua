//mod elements;
use crate::elements::{Card, CardType, Deck, Suit};

enum TeamSide {
    T1,
    T2,
}

enum TeamPlayer {
    P1, P2
}

struct Team {
    p1_hand: Vec<Card>,
    p2_hand: Vec<Card>,
    prize: Vec<Card>,
}

struct Game {
    board: Vec<Card>,
    team_1: Team,
    team_2: Team,
}

impl Game {
    fn team(&'static mut self, team_side: TeamSide) -> &'static mut Team {
        match team_side {
            TeamSide::T1 => &mut self.team_1,
            TeamSide::T2 => &mut self.team_2,
        }
    }

    pub fn team_points(&'static mut self, team_side: TeamSide) -> usize {
        let mut value = 0;
        let team = self.team(team_side);
        for card in team.prize.iter() {
            value += card.value();
        }
        value
    }

    pub fn hand(&'static mut self, team_side: TeamSide, player: TeamPlayer) -> &mut Vec<Card> {
        let team = self.team(team_side);
        match player {
            TeamPlayer::P1 => &mut team.p1_hand,
            TeamPlayer::P2 => &mut team.p2_hand,
        }
    }

    pub fn play(&'static mut self, team_side: TeamSide, player: TeamPlayer, index: usize) {
        let hand = self.hand(team_side, player);
        let card = hand.remove(index);
    }
}
