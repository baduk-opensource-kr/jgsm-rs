#[derive(Clone, PartialEq)]
pub struct Player {
    korean_name: String,
    english_name: String,
    elo_rating: f64,
    condition_weight: f64,
    rapid_weight: f64,
    blitz_weight: f64,
    bullet_weight: f64,
}

impl Player {
    pub fn new(
        korean_name: String,
        english_name: String,
        elo_rating: f64,
        condition_weight: f64,
        rapid_weight: f64,
        blitz_weight: f64,
        bullet_weight: f64,
    ) -> Player {
        Player {
            korean_name,
            english_name,
            elo_rating,
            condition_weight,
            rapid_weight,
            blitz_weight,
            bullet_weight,
        }
    }

    pub fn korean_name(&self) -> &String {
        &self.korean_name
    }

    pub fn english_name(&self) -> &String {
        &self.english_name
    }

    pub fn elo_rating(&self) -> f64 {
        self.elo_rating
    }

    pub fn condition_weight(&self) -> f64 {
        self.condition_weight
    }

    pub fn rapid_weight(&self) -> f64 {
        self.rapid_weight
    }

    pub fn blitz_weight(&self) -> f64 {
        self.blitz_weight
    }

    pub fn bullet_weight(&self) -> f64 {
        self.bullet_weight
    }

    pub fn set_elo_rating(&mut self, elo_rating: f64) {
        self.elo_rating = elo_rating;
    }

    pub fn set_condition_weight(&mut self, condition_weight: f64) {
        self.condition_weight = condition_weight;
    }

    pub fn set_rapid_weight(&mut self, rapid_weight: f64) {
        self.rapid_weight = rapid_weight;
    }

    pub fn set_blitz_weight(&mut self, blitz_weight: f64) {
        self.blitz_weight = blitz_weight;
    }

    pub fn set_bullet_weight(&mut self, bullet_weight: f64) {
        self.bullet_weight = bullet_weight;
    }
}

pub struct Team {
    team_name: String,
    players: Vec<Player>,
}

impl Team {
    pub fn new(team_name: String, players: Vec<Player>) -> Team {
        Team { team_name, players }
    }

    pub fn team_name(&self) -> &String {
        &self.team_name
    }

    pub fn players(&self) -> &Vec<Player> {
        &self.players
    }

    pub fn remove_player(&mut self, index: usize) -> Player {
        self.players.remove(index)
    }

    pub fn players_mut(&mut self) -> &mut Vec<Player> {
        &mut self.players
    }
}

#[derive(Clone)]
pub struct PlayerRelativity {
    player1: Player,
    player2: Player,
    player1_wins: u32,
    player2_wins: u32,
    elo_win_probability: f64,
    condition_win_probability: f64,
    rapid_win_probability: f64,
    blitz_win_probability: f64,
    bullet_win_probability: f64,
}

impl PlayerRelativity {
    pub fn new(
        player1: Player,
        player2: Player,
        player1_wins: u32,
        player2_wins: u32,
        elo_win_probability: f64,
        condition_win_probability: f64,
        rapid_win_probability: f64,
        blitz_win_probability: f64,
        bullet_win_probability: f64,
    ) -> Self {
        Self {
            player1,
            player2,
            player1_wins,
            player2_wins,
            elo_win_probability,
            condition_win_probability,
            rapid_win_probability,
            blitz_win_probability,
            bullet_win_probability,
        }
    }

    pub fn player1(&self) -> &Player {
        &self.player1
    }

    pub fn player2(&self) -> &Player {
        &self.player2
    }

    pub fn player1_wins(&self) -> u32 {
        self.player1_wins
    }

    pub fn player2_wins(&self) -> u32 {
        self.player2_wins
    }

    pub fn elo_win_probability(&self) -> f64 {
        self.elo_win_probability
    }

    pub fn condition_win_probability(&self) -> f64 {
        self.condition_win_probability
    }

    pub fn rapid_win_probability(&self) -> f64 {
        self.rapid_win_probability
    }

    pub fn blitz_win_probability(&self) -> f64 {
        self.blitz_win_probability
    }

    pub fn bullet_win_probability(&self) -> f64 {
        self.bullet_win_probability
    }
}

#[derive(Clone)]
pub struct Lineup {
    first_rapid: Player,
    second_blitz: Player,
    third_blitz: Player,
    forth_blitz: Player,
}

impl Lineup {
    pub fn new(
        first_rapid: Player,
        second_blitz: Player,
        third_blitz: Player,
        forth_blitz: Player,
    ) -> Self {
        Self {
            first_rapid,
            second_blitz,
            third_blitz,
            forth_blitz,
        }
    }

    pub fn first_rapid(&self) -> &Player {
        &self.first_rapid
    }

    pub fn second_blitz(&self) -> &Player {
        &self.second_blitz
    }

    pub fn third_blitz(&self) -> &Player {
        &self.third_blitz
    }

    pub fn forth_blitz(&self) -> &Player {
        &self.forth_blitz
    }
}

#[derive(Clone)]
pub struct MatchResult {
    first_rapid: PlayerRelativity,
    second_blitz: PlayerRelativity,
    third_blitz: PlayerRelativity,
    forth_blitz: PlayerRelativity,
    first_rapid_win_probability: f64,
    second_blitz_win_probability: f64,
    third_blitz_win_probability: f64,
    forth_blitz_win_probability: f64,
    four_zero_probability: f64,
    three_one_probability: f64,
    two_two_probability: f64,
    one_three_probability: f64,
    zero_four_probability: f64,
    tiebreaker_win_probability: f64,
    total_win_probability: f64,
}

impl MatchResult {
    pub fn new(
        first_rapid: PlayerRelativity,
        second_blitz: PlayerRelativity,
        third_blitz: PlayerRelativity,
        forth_blitz: PlayerRelativity,
        first_rapid_win_probability: f64,
        second_blitz_win_probability: f64,
        third_blitz_win_probability: f64,
        forth_blitz_win_probability: f64,
        four_zero_probability: f64,
        three_one_probability: f64,
        two_two_probability: f64,
        one_three_probability: f64,
        zero_four_probability: f64,
        tiebreaker_win_probability: f64,
        total_win_probability: f64,
    ) -> Self {
        Self {
            first_rapid,
            second_blitz,
            third_blitz,
            forth_blitz,
            first_rapid_win_probability,
            second_blitz_win_probability,
            third_blitz_win_probability,
            forth_blitz_win_probability,
            four_zero_probability,
            three_one_probability,
            two_two_probability,
            one_three_probability,
            zero_four_probability,
            tiebreaker_win_probability,
            total_win_probability,
        }
    }

    pub fn first_rapid(&self) -> &PlayerRelativity {
        &self.first_rapid
    }

    pub fn second_blitz(&self) -> &PlayerRelativity {
        &self.second_blitz
    }

    pub fn third_blitz(&self) -> &PlayerRelativity {
        &self.third_blitz
    }

    pub fn forth_blitz(&self) -> &PlayerRelativity {
        &self.forth_blitz
    }

    pub fn first_rapid_win_probability(&self) -> f64 {
        self.first_rapid_win_probability
    }

    pub fn second_blitz_win_probability(&self) -> f64 {
        self.second_blitz_win_probability
    }

    pub fn third_blitz_win_probability(&self) -> f64 {
        self.third_blitz_win_probability
    }

    pub fn forth_blitz_win_probability(&self) -> f64 {
        self.forth_blitz_win_probability
    }

    pub fn four_zero_probability(&self) -> f64 {
        self.four_zero_probability
    }

    pub fn three_one_probability(&self) -> f64 {
        self.three_one_probability
    }

    pub fn two_two_probability(&self) -> f64 {
        self.two_two_probability
    }

    pub fn one_three_probability(&self) -> f64 {
        self.one_three_probability
    }

    pub fn zero_four_probability(&self) -> f64 {
        self.zero_four_probability
    }

    pub fn tiebreaker_win_probability(&self) -> f64 {
        self.tiebreaker_win_probability
    }

    pub fn total_win_probability(&self) -> f64 {
        self.total_win_probability
    }
    pub fn set_first_rapid_win_probability(&mut self, value: f64) {
        self.first_rapid_win_probability = value;
    }

    pub fn set_second_blitz_win_probability(&mut self, value: f64) {
        self.second_blitz_win_probability = value;
    }

    pub fn set_third_blitz_win_probability(&mut self, value: f64) {
        self.third_blitz_win_probability = value;
    }

    pub fn set_forth_blitz_win_probability(&mut self, value: f64) {
        self.forth_blitz_win_probability = value;
    }

    pub fn set_four_zero_probability(&mut self, value: f64) {
        self.four_zero_probability = value;
    }

    pub fn set_three_one_probability(&mut self, value: f64) {
        self.three_one_probability = value;
    }

    pub fn set_two_two_probability(&mut self, value: f64) {
        self.two_two_probability = value;
    }

    pub fn set_one_three_probability(&mut self, value: f64) {
        self.one_three_probability = value;
    }

    pub fn set_zero_four_probability(&mut self, value: f64) {
        self.zero_four_probability = value;
    }

    pub fn set_total_win_probability(&mut self, value: f64) {
        self.total_win_probability = value;
    }
}

// #[derive(Clone)]
// pub struct PostMatchResult {
//     first_rapid: PlayerRelativity,
//     second_blitz: PlayerRelativity,
//     third_blitz: PlayerRelativity,
//     forth_blitz: PlayerRelativity,
//     fifth_bullet: PlayerRelativity,
//     first_rapid_win_probability: f64,
//     second_blitz_win_probability: f64,
//     third_blitz_win_probability: f64,
//     forth_blitz_win_probability: f64,
//     fifth_bullet_win_probability: f64,
//     five_zero_probability: f64,
//     four_one_probability: f64,
//     three_two_probability: f64,
//     two_three_probability: f64,
//     one_four_probability: f64,
//     zero_five_probability: f64,
//     tiebreaker_win_probability: f64,
//     total_win_probability: f64,
// }

// impl PostMatchResult {
//     pub fn new(
//         first_rapid: PlayerRelativity,
//         second_blitz: PlayerRelativity,
//         third_blitz: PlayerRelativity,
//         forth_blitz: PlayerRelativity,
//         fifth_bullet: PlayerRelativity,
//         first_rapid_win_probability: f64,
//         second_blitz_win_probability: f64,
//         third_blitz_win_probability: f64,
//         forth_blitz_win_probability: f64,
//         fifth_bullet_win_probability: f64,
//         five_zero_probability: f64,
//         four_one_probability: f64,
//         three_two_probability: f64,
//         two_three_probability: f64,
//         one_four_probability: f64,
//         zero_five_probability: f64,
//         tiebreaker_win_probability: f64,
//         total_win_probability: f64,
//     ) -> Self {
//         Self {
//             first_rapid,
//             second_blitz,
//             third_blitz,
//             forth_blitz,
//             fifth_bullet,
//             first_rapid_win_probability,
//             second_blitz_win_probability,
//             third_blitz_win_probability,
//             forth_blitz_win_probability,
//             fifth_bullet_win_probability,
//             five_zero_probability,
//             four_one_probability,
//             three_two_probability,
//             two_three_probability,
//             one_four_probability,
//             zero_five_probability,
//             tiebreaker_win_probability,
//             total_win_probability,
//         }
//     }

//     pub fn first_rapid(&self) -> &PlayerRelativity {
//         &self.first_rapid
//     }

//     pub fn second_blitz(&self) -> &PlayerRelativity {
//         &self.second_blitz
//     }

//     pub fn third_blitz(&self) -> &PlayerRelativity {
//         &self.third_blitz
//     }

//     pub fn forth_blitz(&self) -> &PlayerRelativity {
//         &self.forth_blitz
//     }

//     pub fn fifth_bullet(&self) -> &PlayerRelativity {
//         &self.fifth_bullet
//     }

//     pub fn first_rapid_win_probability(&self) -> f64 {
//         self.first_rapid_win_probability
//     }

//     pub fn second_blitz_win_probability(&self) -> f64 {
//         self.second_blitz_win_probability
//     }

//     pub fn third_blitz_win_probability(&self) -> f64 {
//         self.third_blitz_win_probability
//     }

//     pub fn forth_blitz_win_probability(&self) -> f64 {
//         self.forth_blitz_win_probability
//     }

//     pub fn fifth_bullet_win_probability(&self) -> f64 {
//         self.fifth_bullet_win_probability
//     }

//     pub fn five_zero_probability(&self) -> f64 {
//         self.five_zero_probability
//     }

//     pub fn four_one_probability(&self) -> f64 {
//         self.four_one_probability
//     }

//     pub fn three_two_probability(&self) -> f64 {
//         self.three_two_probability
//     }

//     pub fn two_three_probability(&self) -> f64 {
//         self.two_three_probability
//     }

//     pub fn one_four_probability(&self) -> f64 {
//         self.one_four_probability
//     }

//     pub fn zero_five_probability(&self) -> f64 {
//         self.zero_five_probability
//     }

//     pub fn tiebreaker_win_probability(&self) -> f64 {
//         self.tiebreaker_win_probability
//     }

//     pub fn total_win_probability(&self) -> f64 {
//         self.total_win_probability
//     }
// }

// #[derive(Clone)]
// pub struct PostLineup {
//     first_rapid: Player,
//     second_blitz: Player,
//     third_blitz: Player,
//     forth_blitz: Player,
//     fifth_bullet: Player,
// }

// impl PostLineup {
//     pub fn new(
//         first_rapid: Player,
//         second_blitz: Player,
//         third_blitz: Player,
//         forth_blitz: Player,
//         fifth_bullet: Player,
//     ) -> Self {
//         Self {
//             first_rapid,
//             second_blitz,
//             third_blitz,
//             forth_blitz,
//             fifth_bullet,
//         }
//     }

//     pub fn first_rapid(&self) -> &Player {
//         &self.first_rapid
//     }

//     pub fn second_blitz(&self) -> &Player {
//         &self.second_blitz
//     }

//     pub fn third_blitz(&self) -> &Player {
//         &self.third_blitz
//     }

//     pub fn forth_blitz(&self) -> &Player {
//         &self.forth_blitz
//     }

//     pub fn fifth_bullet(&self) -> &Player {
//         &self.fifth_bullet
//     }
// }
