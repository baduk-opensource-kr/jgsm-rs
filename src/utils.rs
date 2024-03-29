use crate::models::{Lineup, MatchResult, Player, PlayerRelativity, Team, TiebreakerRelativity};
use crossterm::{
    execute,
    terminal::{Clear, ClearType},
    cursor::{MoveTo, SavePosition},
    style::Print,
};
use chrono::{Datelike, Timelike, NaiveDate};
use fantoccini::{Client, Locator};
use fantoccini::wd::TimeoutConfiguration;
use itertools::Itertools;
use reqwest;
use scraper::{Html, Selector};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::f64::consts::E;
use std::io;
use std::io::stdout;
use std::time::Duration;
use xlsxwriter::format::FormatColor;
use xlsxwriter::prelude::Workbook;
use xlsxwriter::Format;

pub fn calculate_win_probability(player1_elo: f64, player2_elo: f64) -> f64 {
    let elo_diff = player2_elo - player1_elo;
    let probability = 1.0 / (1.0 + 10.0_f64.powf(elo_diff / 400.0));
    probability
}

pub fn calculate_win_probability_with_relative_record(player1_elo: f64, player2_elo: f64, player1_wins: u32, player2_wins: u32) -> f64 {
    let base_probability = calculate_win_probability(player1_elo, player2_elo);
    let total_games = player1_wins + player2_wins;
    let win_rate_difference = if total_games > 0 {
        player1_wins as f64 / total_games as f64
    } else {
        0.5
    };

    interpolate(win_rate_difference, ((standard_error(base_probability, total_games as f64) * 2.0) + 1.0) / 2.0, base_probability)
}

fn standard_error(base_probability: f64, total_games: f64) -> f64 {
    if total_games <= 0.0 {
        return 0.5;
    }
    (base_probability * (1.0 - base_probability) / total_games).sqrt()
}

fn interpolate(relative_probability: f64, standard_error: f64, base_probability: f64) -> f64 {
    // weight가 0보다 작으면 left를 반환
    if standard_error <= 0.0 {
        return relative_probability;
    // weight가 1보다 크면 right를 반환
    } else if standard_error >= 1.0 {
        return base_probability;
    // weight가 0에서 1 사이에 있을 때는 left와 right 사이를 선형 보간하여 반환
    } else {
        return relative_probability * (1.0 - standard_error) + base_probability * standard_error;
    }
}

pub fn fetch_player_ratings_on_baeteil() -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let mut ratings = HashMap::new();
    let client = reqwest::blocking::Client::new();
    let res = client.post("http://baduk.or.kr/record/rankingPlayer_in.asp")
        .form(&[("pageNo", "1"), ("keyColumn", &chrono::Utc::now().year().to_string()), ("keyWord", &chrono::Utc::now().month().to_string()), ("etcKey1", "1")])
        .send()?;
    let body = res.text()?;
    let document = Html::parse_document(&body);
    let player_selector = Selector::parse("table.tbstyle03 > tbody > tr").unwrap();
    let name_selector = Selector::parse("td:nth-child(2) a").unwrap();
    let rating_selector = Selector::parse("td:nth-child(3)").unwrap();

    for player in document.select(&player_selector) {
        if let Some(name_element) = player.select(&name_selector).next() {
            if let Some(rating_element) = player.select(&rating_selector).next() {
                let name = name_element.inner_html();
                let rating_str = rating_element.inner_html().chars().filter(|c| c.is_digit(10)).collect::<String>();
                if let Ok(rating) = rating_str.parse::<f64>() {
                    ratings.insert(name, rating * 0.75);
                }
            }
        }
    }

    Ok(ratings)
}

pub fn fetch_player_ratings_on_goratings() -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    let mut ratings = HashMap::new();
    let body = reqwest::blocking::get("https://www.goratings.org/en/")?.text()?;
    let document = Html::parse_document(&body);
    let player_selector = Selector::parse("tr").unwrap();
    let name_selector = Selector::parse("td:nth-child(2) a").unwrap();
    let rating_selector = Selector::parse("td:nth-child(5)").unwrap();

    for player in document.select(&player_selector) {
        if let Some(name_element) = player.select(&name_selector).next() {
            if let Some(rating_element) = player.select(&rating_selector).next() {
                let name = name_element.inner_html();
                let rating_str = rating_element.inner_html();
                if let Ok(rating) = rating_str.parse::<f64>() {
                    ratings.insert(name, rating);
                }
            }
        }
    }

    Ok(ratings)
}

pub fn fetch_head_to_head_record(gisa1: &str, gisa2: &str) -> Result<HashMap<String, u32>, Box<dyn Error>> {
    let current_year = chrono::Utc::now().year();
    let mut records = HashMap::new();

    let mut gisa1_wins = 0;
    let mut gisa2_wins = 0;

    for year in (current_year - 3)..=current_year {
        let url = format!("https://cyberoro.com/cooperate/giwon/gibo_M_in.oro?ydate={}&gisa1={}&gisa2={}&listCnt=20&P_KEY=0", year, gisa1, gisa2);
        let body = reqwest::blocking::get(&url)?.text()?;
        let document = Html::parse_document(&body);
        let result_selector = Selector::parse("span[style='color:#1c97fe']").unwrap();

        for result in document.select(&result_selector) {
            let result_text = result.inner_html();
            if result_text.contains(gisa1) {
                gisa1_wins += 1;
            } else if result_text.contains(gisa2) {
                gisa2_wins += 1;
            }
        }
    }
    records.insert(gisa1.to_string(), gisa1_wins);
    records.insert(gisa2.to_string(), gisa2_wins);

    Ok(records)
}

pub fn update_team_elo_ratings(selected_teams: &mut Vec<Team>) -> Result<(), Box<dyn Error>> {
    let player_ratings_on_baeteil = fetch_player_ratings_on_baeteil()?;
    let player_ratings_on_goratings = fetch_player_ratings_on_goratings()?;

    let park_junghwan_baeteil_rating = player_ratings_on_baeteil.get("박정환").cloned().unwrap_or(0.0);
    let byun_sangil_baeteil_rating = player_ratings_on_baeteil.get("변상일").cloned().unwrap_or(0.0);
    let baeteil_standard = (park_junghwan_baeteil_rating + byun_sangil_baeteil_rating) / 2.0;

    let park_junghwan_goratings_rating = player_ratings_on_goratings.get("Park Junghwan").cloned().unwrap_or(0.0);
    let byun_sangil_goratings_rating = player_ratings_on_goratings.get("Byun Sangil").cloned().unwrap_or(0.0);
    let goratings_standard = (park_junghwan_goratings_rating + byun_sangil_goratings_rating) / 2.0;

    let goratings_to_baeteil = baeteil_standard - goratings_standard;

    for team in selected_teams.iter_mut() {
        for player in team.players_mut().iter_mut() {
            if let Some(&rating) = player_ratings_on_baeteil.get(player.korean_name()) {
                match get_recent_record(player.korean_name(), rating, &player_ratings_on_baeteil) {
                    Ok(current_rating) => {
                        player.set_elo_rating(current_rating);
                    },
                    Err(_) => {
                        player.set_elo_rating(rating);
                    }
                }
                player.set_blitz_weight(speed_aging_curve(player.get_days_since_birth()));
                player.set_bullet_weight(speed_aging_curve(player.get_days_since_birth()) * 2.0);
            } else if let Some(&rating) = player_ratings_on_goratings.get(player.english_name()) {
                match get_recent_record(player.korean_name(), rating + goratings_to_baeteil, &player_ratings_on_baeteil) {
                    Ok(current_rating) => {
                        player.set_elo_rating(current_rating);
                    },
                    Err(_) => {
                        player.set_elo_rating(rating + goratings_to_baeteil);
                    }
                }
                player.set_blitz_weight(speed_aging_curve(player.get_days_since_birth()));
                player.set_bullet_weight(speed_aging_curve(player.get_days_since_birth()) * 2.0);
            }
        }
    }

    Ok(())
}

pub fn generate_player_relativities(selected_teams: &Vec<Team>) -> Result<Vec<PlayerRelativity>, String> {
    let mut all_relative_records: Vec<PlayerRelativity> = Vec::new();

    let team1 = &selected_teams[0];
    let team2 = &selected_teams[1];
    for player1 in team1.players() {
        for player2 in team2.players() {
            let record = fetch_head_to_head_record(&player1.korean_name(), &player2.korean_name())
                               .map_err(|e| format!("상대전적을 가져오는 중 오류가 발생했습니다: {}", e))?;
            let player1_wins = *record.get(player1.korean_name()).unwrap_or(&0);
            let player2_wins = *record.get(player2.korean_name()).unwrap_or(&0);
            let elo_win_probability = calculate_win_probability_with_relative_record(player1.elo_rating() as f64, player2.elo_rating() as f64, player1_wins as u32, player2_wins as u32);
            let condition_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating() + player1.condition_weight()) as f64, (player2.elo_rating() + player2.condition_weight()) as f64, player1_wins, player2_wins);
            let rapid_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating() + player1.condition_weight() + player1.rapid_weight()) as f64, (player2.elo_rating() + player2.condition_weight() + player2.rapid_weight()) as f64, player1_wins, player2_wins);
            let blitz_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating() + player1.condition_weight() + player1.blitz_weight()) as f64, (player2.elo_rating() + player2.condition_weight() + player2.blitz_weight()) as f64, player1_wins, player2_wins);
            let bullet_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating() + player1.condition_weight() + player1.bullet_weight()) as f64, (player2.elo_rating() + player2.condition_weight() + player2.bullet_weight()) as f64, player1_wins, player2_wins);
            all_relative_records.push(PlayerRelativity::new(
                player1.clone(),
                player2.clone(),
                player1_wins,
                player2_wins,
                elo_win_probability * 100.0,
                condition_win_probability * 100.0,
                rapid_win_probability * 100.0,
                blitz_win_probability * 100.0,
                bullet_win_probability * 100.0,
            ));
        }
    }

    Ok(all_relative_records)
}

pub fn calculate_match_result(team1_lineup: Lineup, team2_lineup: Lineup, player_relativities: Vec<PlayerRelativity>) -> MatchResult {
    let team1_players = vec![team1_lineup.first_rapid(), team1_lineup.second_blitz(), team1_lineup.third_blitz(), team1_lineup.forth_blitz()];
    let team2_players = vec![team2_lineup.first_rapid(), team2_lineup.second_blitz(), team2_lineup.third_blitz(), team2_lineup.forth_blitz()];

    let mut win_probabilities = vec![0.0; team1_players.len()];
    let mut bullet_win_probabilities = vec![0.0; team1_players.len()];

    for (i, player1) in team1_players.iter().enumerate() {
        if let Some(player2) = team2_players.get(i) {
            if let Some(relativity) = player_relativities.iter().find(|r| r.player1().korean_name() == player1.korean_name() && r.player2().korean_name() == player2.korean_name()) {
                win_probabilities[i] = match i {
                    0 => relativity.rapid_win_probability(),
                    _ => relativity.blitz_win_probability(),
                };
                bullet_win_probabilities[i] = relativity.bullet_win_probability();
            }
        }
    }

    let mapped_tiebreaker_win_probability: Vec<TiebreakerRelativity> = player_relativities.iter()
        .map(|relativity| {
            let player1_position = team1_players.iter().position(|p| p.korean_name() == relativity.player1().korean_name());
            let player2_position = team2_players.iter().position(|p| p.korean_name() == relativity.player2().korean_name());
            let player1_penalty = if let Some(pos) = player1_position {
                match pos {
                    0 => (1.0 / 1.04) * (1.0 / (1.0 + (0.04 * (1.0 - relativity.rapid_win_probability() / 100.0)))),
                    1 => (1.0 / 1.02) * (1.0 / (1.0 + (0.02 * (1.0 - relativity.blitz_win_probability() / 100.0)))),
                    2 => (1.0 / 1.08) * (1.0 / (1.0 + (0.08 * (1.0 - relativity.blitz_win_probability() / 100.0)))),
                    3 => (1.0 / 1.08) * (1.0 / (1.0 + (0.08 * (1.0 - relativity.blitz_win_probability() / 100.0)))),
                    _ => 1.0,
                }
            } else {
                1.0
            };
            let player2_penalty = if let Some(pos) = player2_position {
                match pos {
                    0 => 1.04 * (1.0 + (0.04 * (1.0 - relativity.rapid_win_probability() / 100.0))),
                    1 => 1.02 * (1.0 + (0.02 * (1.0 - relativity.blitz_win_probability() / 100.0))),
                    2 => 1.08 * (1.0 + (0.08 * (1.0 - relativity.blitz_win_probability() / 100.0))),
                    3 => 1.08 * (1.0 + (0.08 * (1.0 - relativity.blitz_win_probability() / 100.0))),
                    _ => 1.0,
                }
            } else {
                1.0
            };
            TiebreakerRelativity::new(
                relativity.player1().clone(), 
                relativity.player2().clone(), 
                relativity.bullet_win_probability() * player1_penalty * player2_penalty
            )
        })
        .collect();

    let team1_tiebreaker_details = mapped_tiebreaker_win_probability.iter()
        .fold(HashMap::<String, Vec<&TiebreakerRelativity>>::new(), |mut acc, relativity| {
            let player1_name = relativity.player1().korean_name();
            acc.entry(player1_name.to_string()).or_insert_with(Vec::new).push(relativity);
            acc
        })
        .values()
        .map(|relativities| {
            relativities.iter().min_by(|a, b| a.win_probability().partial_cmp(&b.win_probability()).unwrap()).unwrap()
        })
        .max_by(|a, b| a.win_probability().partial_cmp(&b.win_probability()).unwrap())
        .cloned();

    let team2_tiebreaker_details = mapped_tiebreaker_win_probability.iter()
        .fold(HashMap::<String, Vec<&TiebreakerRelativity>>::new(), |mut acc, relativity| {
            let player2_name = relativity.player2().korean_name();
            acc.entry(player2_name.to_string()).or_insert_with(Vec::new).push(relativity);
            acc
        })
        .values()
        .map(|relativities| {
            relativities.iter().max_by(|a, b| a.win_probability().partial_cmp(&b.win_probability()).unwrap()).unwrap()
        })
        .min_by(|a, b| a.win_probability().partial_cmp(&b.win_probability()).unwrap())
        .cloned();

    let tiebreaker_win_probability = (team1_tiebreaker_details.map_or(50.0, |details| details.win_probability()) + team2_tiebreaker_details.map_or(50.0, |details| details.win_probability())) / 2.0;

    let all_win_probability = win_probabilities.iter().map(|p| p / 100.0).product::<f64>();

    let three_win_one_lose_probability = win_probabilities.iter().enumerate().map(|(i, &win_prob)| {
        let lose_prob = 1.0 - (win_prob / 100.0);
        win_probabilities.iter().enumerate().filter(|&(j, _)| i != j).map(|(_, &other_win_prob)| other_win_prob / 100.0).product::<f64>() * lose_prob
    }).sum::<f64>();

    let two_win_two_lose_probability = win_probabilities.iter().enumerate().combinations(2).map(|win_indices| {
        let win_prob_product = win_indices.iter().map(|&(i, _)| win_probabilities[i] / 100.0).product::<f64>();
        let lose_indices = (0..win_probabilities.len()).filter(|i| !win_indices.iter().any(|&(wi, _)| wi == *i)).collect::<Vec<_>>();
        let lose_prob_product = lose_indices.iter().map(|&i| 1.0 - (win_probabilities[i] / 100.0)).product::<f64>();
        win_prob_product * lose_prob_product
    }).sum::<f64>();

    let one_win_three_lose_probability = win_probabilities.iter().enumerate().map(|(i, &win_prob)| {
        let win_prob = win_prob / 100.0;
        win_probabilities.iter().enumerate().filter(|&(j, _)| i != j).map(|(_, &other_lose_prob)| 1.0 - (other_lose_prob / 100.0)).product::<f64>() * win_prob
    }).sum::<f64>();

    let all_lose_probability = win_probabilities.iter().map(|&win_prob| 1.0 - (win_prob / 100.0)).product::<f64>();

    let tie_win_probability = two_win_two_lose_probability * (tiebreaker_win_probability / 100.0);

    let total_win_probability = tie_win_probability + three_win_one_lose_probability + all_win_probability;

    MatchResult::new(
        player_relativities.iter().find(|relativity| relativity.player1().korean_name() == team1_players[0].korean_name() && relativity.player2().korean_name() == team2_players[0].korean_name()).unwrap().clone(),
        player_relativities.iter().find(|relativity| relativity.player1().korean_name() == team1_players[1].korean_name() && relativity.player2().korean_name() == team2_players[1].korean_name()).unwrap().clone(),
        player_relativities.iter().find(|relativity| relativity.player1().korean_name() == team1_players[2].korean_name() && relativity.player2().korean_name() == team2_players[2].korean_name()).unwrap().clone(),
        player_relativities.iter().find(|relativity| relativity.player1().korean_name() == team1_players[3].korean_name() && relativity.player2().korean_name() == team2_players[3].korean_name()).unwrap().clone(),
        win_probabilities[0],
        win_probabilities[1],
        win_probabilities[2],
        win_probabilities[3],
        all_win_probability * 100.0,
        three_win_one_lose_probability * 100.0,
        two_win_two_lose_probability * 100.0,
        one_win_three_lose_probability * 100.0,
        all_lose_probability * 100.0,
        total_win_probability * 100.0,
        vec![team1_tiebreaker_details.cloned(), team2_tiebreaker_details.cloned()],
        tiebreaker_win_probability,
    )
}

pub fn create_excel_from_relativities(player_relativities: Vec<PlayerRelativity>, match_results_matrix: Vec<Vec<MatchResult>>) -> Result<(), Box<dyn std::error::Error>> {
    let workbook = Workbook::new("player_relativities.xlsx")?;
    let mut worksheet_elo = workbook.add_worksheet(Some("개인-기본ELO 기반"))?;
    let mut worksheet_condition = workbook.add_worksheet(Some("개인-컨디션 기반"))?;
    let mut worksheet_rapid = workbook.add_worksheet(Some("개인-장고 기반"))?;
    let mut worksheet_blitz = workbook.add_worksheet(Some("개인-속기 기반"))?;
    let mut worksheet_bullet = workbook.add_worksheet(Some("개인-초속기 기반"))?;

    let mut player1_set = HashSet::new();
    let mut player2_set = HashSet::new();

    for relativity in &player_relativities {
        player1_set.insert(relativity.player1().korean_name().clone());
        player2_set.insert(relativity.player2().korean_name().clone());
    }

    let player1s: Vec<_> = player1_set.into_iter().sorted_by(|a, b| {
        let a_score = player_relativities.iter().find(|relativity| relativity.player1().korean_name() == a)
            .map_or(0.0, |relativity| relativity.player1().elo_rating() + relativity.player1().condition_weight());
        let b_score = player_relativities.iter().find(|relativity| relativity.player1().korean_name() == b)
            .map_or(0.0, |relativity| relativity.player1().elo_rating() + relativity.player1().condition_weight());
        b_score.partial_cmp(&a_score).unwrap_or(std::cmp::Ordering::Equal)
    }).collect();
    let player2s: Vec<_> = player2_set.into_iter().sorted_by(|a, b| {
        let a_score = player_relativities.iter().find(|relativity| relativity.player2().korean_name() == a)
            .map_or(0.0, |relativity| relativity.player2().elo_rating() + relativity.player2().condition_weight());
        let b_score = player_relativities.iter().find(|relativity| relativity.player2().korean_name() == b)
            .map_or(0.0, |relativity| relativity.player2().elo_rating() + relativity.player2().condition_weight());
        b_score.partial_cmp(&a_score).unwrap_or(std::cmp::Ordering::Equal)
    }).collect();

    let mut player1_index = HashMap::new();
    let mut player2_index = HashMap::new();

    for (index, player) in player1s.iter().enumerate() {
        worksheet_elo.write_string((index + 1).try_into().unwrap(), 0, player, None)?;
        worksheet_condition.write_string((index + 1).try_into().unwrap(), 0, player, None)?;
        worksheet_rapid.write_string((index + 1).try_into().unwrap(), 0, player, None)?;
        worksheet_blitz.write_string((index + 1).try_into().unwrap(), 0, player, None)?;
        worksheet_bullet.write_string((index + 1).try_into().unwrap(), 0, player, None)?;
        player1_index.insert(player.clone(), index + 1);
    }

    for (index, player) in player2s.iter().enumerate() {
        worksheet_elo.write_string(0, (index + 1).try_into().unwrap(), player, None)?;
        worksheet_condition.write_string(0, (index + 1).try_into().unwrap(), player, None)?;
        worksheet_rapid.write_string(0, (index + 1).try_into().unwrap(), player, None)?;
        worksheet_blitz.write_string(0, (index + 1).try_into().unwrap(), player, None)?;
        worksheet_bullet.write_string(0, (index + 1).try_into().unwrap(), player, None)?;
        player2_index.insert(player.clone(), index + 1);
    }

    for relativity in &player_relativities {
        let row = player1_index[relativity.player1().korean_name()];
        let col = player2_index[relativity.player2().korean_name()];

        let elo_format = create_custom_format(relativity.elo_win_probability(), 15.0)?;
        let condition_format = create_custom_format(relativity.condition_win_probability(), 15.0)?;
        let rapid_format = create_custom_format(relativity.rapid_win_probability(), 15.0)?;
        let blitz_format = create_custom_format(relativity.blitz_win_probability(), 15.0)?;
        let bullet_format = create_custom_format(relativity.bullet_win_probability(), 15.0)?;

        worksheet_elo.write_number(row.try_into().unwrap(), col.try_into().unwrap(), relativity.elo_win_probability() / 100.0, Some(&elo_format))?;
        worksheet_condition.write_number(row.try_into().unwrap(), col.try_into().unwrap(), relativity.condition_win_probability() / 100.0, Some(&condition_format))?;
        worksheet_rapid.write_number(row.try_into().unwrap(), col.try_into().unwrap(), relativity.rapid_win_probability() / 100.0, Some(&rapid_format))?;
        worksheet_blitz.write_number(row.try_into().unwrap(), col.try_into().unwrap(), relativity.blitz_win_probability() / 100.0, Some(&blitz_format))?;
        worksheet_bullet.write_number(row.try_into().unwrap(), col.try_into().unwrap(), relativity.bullet_win_probability() / 100.0, Some(&bullet_format))?;
    }

    let mut worksheet_total_win = workbook.add_worksheet(Some("팀-최종승리"))?;
    let mut worksheet_three_one = workbook.add_worksheet(Some("팀-에결없이 승리"))?;
    let mut worksheet_four_zero = workbook.add_worksheet(Some("팀-완봉승"))?;
    let mut worksheet_tiebreak = workbook.add_worksheet(Some("팀-에결진출"))?;

    for (row_index, row) in match_results_matrix.iter().enumerate() {
        if row_index == 0 { 
            for (col_index, match_result) in row.iter().enumerate().take(24) {
                let lineup_names = format!("1국:{}, 2국:{}, 3국:{}, 4국:{}", 
                    match_result.second_blitz().player2().korean_name(), 
                    match_result.third_blitz().player2().korean_name(), 
                    match_result.forth_blitz().player2().korean_name(), 
                    match_result.first_rapid().player2().korean_name());
                worksheet_total_win.write_string(0, col_index as u16 + 1, &lineup_names, None)?;
                worksheet_three_one.write_string(0, col_index as u16 + 1, &lineup_names, None)?;
                worksheet_four_zero.write_string(0, col_index as u16 + 1, &lineup_names, None)?;
                worksheet_tiebreak.write_string(0, col_index as u16 + 1, &lineup_names, None)?;
            }
        }
        let lineup_names = format!("1국:{}, 2국:{}, 3국:{}, 4국:{}", 
            row[0].second_blitz().player1().korean_name(), 
            row[0].third_blitz().player1().korean_name(), 
            row[0].forth_blitz().player1().korean_name(), 
            row[0].first_rapid().player1().korean_name());
        worksheet_total_win.write_string(row_index as u32 + 1, 0, &lineup_names, None)?;
        worksheet_three_one.write_string(row_index as u32 + 1, 0, &lineup_names, None)?;
        worksheet_four_zero.write_string(row_index as u32 + 1, 0, &lineup_names, None)?;
        worksheet_tiebreak.write_string(row_index as u32 + 1, 0, &lineup_names, None)?;

        for (col_index, match_result) in row.iter().enumerate().take(36) {
            let total_win_format = create_custom_format(match_result.total_win_probability(), 25.0)?;
            let win_format = create_custom_format(match_result.four_zero_probability() + match_result.three_one_probability(), 25.0)?;
            let four_zero_format = create_custom_format(match_result.four_zero_probability(), 25.0)?;
            let tiebreaker_format = create_custom_format(match_result.two_two_probability(), 25.0)?;

            worksheet_total_win.write_number(row_index as u32 + 1, col_index as u16 + 1, match_result.total_win_probability() / 100.0, Some(&total_win_format))?;
            worksheet_three_one.write_number(row_index as u32 + 1, col_index as u16 + 1, (match_result.four_zero_probability() + match_result.three_one_probability()) / 100.0, Some(&win_format))?;
            worksheet_four_zero.write_number(row_index as u32 + 1, col_index as u16 + 1, match_result.four_zero_probability() / 100.0, Some(&four_zero_format))?;
            worksheet_tiebreak.write_number(row_index as u32 + 1, col_index as u16 + 1, match_result.two_two_probability() / 100.0, Some(&tiebreaker_format))?;
        }
    }

    workbook.close()?;

    Ok(())
}

pub fn create_excel_from_tiebreaker_relativities(outcome_map: HashMap<&str, Vec<PlayerRelativity>>) -> Result<(), Box<dyn std::error::Error>> {
    let workbook = Workbook::new("tiebreaker_relativities.xlsx")?;

    for (outcome, tiebreaker_relativities) in outcome_map {
        let worksheet_name = format!("에결-{}", outcome);
        let mut worksheet = workbook.add_worksheet(Some(&worksheet_name))?;

        let mut player1_set = HashSet::new();
        let mut player2_set = HashSet::new();

        for relativity in &tiebreaker_relativities {
            player1_set.insert(relativity.player1().korean_name().clone());
            player2_set.insert(relativity.player2().korean_name().clone());
        }

        let player1s: Vec<_> = player1_set.into_iter().sorted_by(|a, b| {
            let a_score = tiebreaker_relativities.iter().find(|relativity| relativity.player1().korean_name() == a)
                .map_or(0.0, |relativity| relativity.player1().elo_rating() + relativity.player1().condition_weight() + relativity.player1().bullet_weight());
            let b_score = tiebreaker_relativities.iter().find(|relativity| relativity.player1().korean_name() == b)
                .map_or(0.0, |relativity| relativity.player1().elo_rating() + relativity.player1().condition_weight() + relativity.player1().bullet_weight());
            b_score.partial_cmp(&a_score).unwrap_or(std::cmp::Ordering::Equal)
        }).collect();

        let player2s: Vec<_> = player2_set.into_iter().sorted_by(|a, b| {
            let a_score = tiebreaker_relativities.iter().find(|relativity| relativity.player2().korean_name() == a)
                .map_or(0.0, |relativity| relativity.player2().elo_rating() + relativity.player2().condition_weight() + relativity.player2().bullet_weight());
            let b_score = tiebreaker_relativities.iter().find(|relativity| relativity.player2().korean_name() == b)
                .map_or(0.0, |relativity| relativity.player2().elo_rating() + relativity.player2().condition_weight() + relativity.player2().bullet_weight());
            b_score.partial_cmp(&a_score).unwrap_or(std::cmp::Ordering::Equal)
        }).collect();

        let mut player1_index = HashMap::new();
        let mut player2_index = HashMap::new();

        for (index, player) in player1s.iter().enumerate() {
            worksheet.write_string((index + 1).try_into().unwrap(), 0, player, None)?;
            player1_index.insert(player.clone(), index + 1);
        }

        for (index, player) in player2s.iter().enumerate() {
            worksheet.write_string(0, (index + 1).try_into().unwrap(), player, None)?;
            player2_index.insert(player.clone(), index + 1);
        }

        for relativity in &tiebreaker_relativities {
            let row = player1_index[relativity.player1().korean_name()];
            let col = player2_index[relativity.player2().korean_name()];

            let format = create_custom_format(relativity.bullet_win_probability(), 15.0)?;

            worksheet.write_number(row.try_into().unwrap(), col.try_into().unwrap(), relativity.bullet_win_probability() / 100.0, Some(&format))?;
        }
    }

    workbook.close()?;

    Ok(())
}

pub fn create_custom_format(win_probability: f64, maximum: f64) -> Result<Format, Box<dyn std::error::Error>> {
    let mut format = Format::new();

    // 승리확률에 따라 색상을 점진적으로 변경합니다.
    let custom_color = if win_probability >= (100.0 - maximum) {
        FormatColor::Blue
    } else if win_probability <= maximum {
        FormatColor::Red
    } else {
        // 50%에 가까울수록 하얀색
        let (red, blue) = if win_probability > 50.0 {
            let gradient = (win_probability - 50.0) / (50.0 - maximum);
            (255.0 * (1.0 - gradient), 255.0)
        } else {
            let gradient = (50.0 - win_probability) / (50.0 - maximum);
            (255.0, 255.0 * (1.0 - gradient))
        };

        let green = if win_probability > 50.0 {
            255.0 * (1.0 - (win_probability - 50.0) / (50.0 - maximum))
        } else {
            255.0 * (1.0 - (50.0 - win_probability) / (50.0 - maximum))
        };

        FormatColor::Custom((red as u32) << 16 | (green as u32) << 8 | blue as u32)
    };

    format.set_num_format("0.00%").set_bg_color(custom_color);

    Ok(format)
}

pub fn select_team_combination(team: &Team) -> Vec<&Player> {
    let mut team_combination: Vec<&Player> = Vec::new();
    println!("\n{} 팀의 스쿼드:", team.team_name());
    for (index, player) in team.players().iter().enumerate() {
        println!("{}. {} (elo: {:.2}, 컨디션: {:.2}, 장고: {:.2}, 속기: {:.2}, 초속기: {:.2})", index + 1, player.korean_name(), player.elo_rating(), player.elo_rating() + player.condition_weight(), player.elo_rating() + player.rapid_weight(), player.elo_rating() + player.blitz_weight(), player.elo_rating() + player.bullet_weight());
    }
    for i in 0..4 {
        loop {
            let mut input = String::new();
            match i {
                0 => println!("\n{} 팀의 1국 장고(rapid) 기사 번호를 입력하세요:", team.team_name()),
                1 => println!("\n{} 팀의 2국 속기(blitz) 기사 번호를 입력하세요:", team.team_name()),
                2 => println!("\n{} 팀의 3국 속기(blitz) 기사 번호를 입력하세요:", team.team_name()),
                3 => println!("\n{} 팀의 4국 속기(blitz) 기사 번호를 입력하세요:", team.team_name()),
                _ => {}
            }
            io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
            match input.trim().parse::<usize>() {
                Ok(num) if num > 0 && num <= team.players().len() => {
                    team_combination.push(&team.players()[num - 1]);
                    break;
                },
                _ => println!("잘못된 입력입니다. 다시 입력해주세요."),
            }
        }
    }
    team_combination
}

pub fn filter_team1_lineups(selected_teams: &[Team], team1_all_lineups: &[Lineup]) -> Vec<Lineup> {
    let unknown_player = Player::new("알 수 없음".to_string(), "unknown".to_string(), "未知".to_string(), NaiveDate::from_ymd_opt(2000, 1, 1).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0);

    let mut team1_combination: Vec<&Player> = Vec::new();
    println!("\n{} 팀의 스쿼드:", selected_teams[0].team_name());
    println!("특정 기사에게 고정포지션이 있다면 선택해주세요. 없다면 알 수 없음을 선택해주세요.");
    let mut last_index = 0;
    for (index, player) in selected_teams[0].players().iter().enumerate() {
        println!("{}. {} (elo: {:.2}, 컨디션: {:.2}, 장고: {:.2}, 속기: {:.2}, 초속기: {:.2})", index + 1, player.korean_name(), player.elo_rating(), player.condition_weight(), player.rapid_weight(), player.blitz_weight(), player.bullet_weight());
        last_index = index;
    }
    println!("{}. 알 수 없음", last_index + 2);
    for i in 0..4 {
        loop {
            let mut input = String::new();
            println!("\n{} 팀의 {}국 {} 기사 번호를 입력하세요:", selected_teams[0].team_name(), i + 1, if i == 0 { "장고(rapid)" } else { "속기(blitz)" });
            io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
            match input.trim().parse::<usize>() {
                Ok(num) if num > 0 && num <= selected_teams[0].players().len() => {
                    team1_combination.push(&selected_teams[0].players()[num - 1]);
                    break;
                },
                Ok(num) if num == selected_teams[0].players().len() + 1 => {
                    team1_combination.push(&unknown_player);
                    break;
                },
                _ => println!("잘못된 입력입니다. 다시 입력해주세요."),
            }
        }
    }

    team1_all_lineups.iter().filter(|lineup| {
        (team1_combination[0].english_name() == "unknown" || lineup.first_rapid().korean_name() == team1_combination[0].korean_name()) &&
        (team1_combination[1].english_name() == "unknown" || lineup.second_blitz().korean_name() == team1_combination[1].korean_name()) &&
        (team1_combination[2].english_name() == "unknown" || lineup.third_blitz().korean_name() == team1_combination[2].korean_name()) &&
        (team1_combination[3].english_name() == "unknown" || lineup.forth_blitz().korean_name() == team1_combination[3].korean_name())
    }).cloned().collect()
}

pub async fn live_win_ratings(match_result: MatchResult, player_relativities: Vec<PlayerRelativity>) {
    let c = Client::new("http://127.0.0.1:4444").await.expect("WebDriver에 연결하지 못했습니다.");
    c.goto("https://home.yikeweiqi.com/#/live").await.expect("yikeweiqi에 연결하지 못했습니다.");

    let mut live_match_result = match_result.clone();
    let mut stdout = stdout();
    execute!(stdout, SavePosition, Clear(ClearType::All)).expect("화면을 지우는 데 실패했습니다.");

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
        tx.send(()).expect("메인 스레드에 신호를 보내는 데 실패했습니다.");
    });

    'outer: loop {
        if rx.try_recv().is_ok() {
            break 'outer;
        }
        let matches = c.find_all(Locator::Css("div.live_detail")).await.expect("div.live_detail 요소를 찾는 중 오류가 발생했습니다.");
        for match_element in &matches {
            let text = match match_element.text().await {
                Ok(t) => t,
                Err(_) => continue,
            };

            let livedtl_time = match_element.find(Locator::Css("span.livedtl_time")).await.expect("span.livedtl_time 요소를 찾는 중 오류가 발생했습니다.").text().await.expect("텍스트를 가져오는 중 오류가 발생했습니다.");

            let now = chrono::Local::now();
            let livedtl_date = chrono::NaiveDate::parse_from_str(&format!("{} {}", now.format("%Y"), livedtl_time.split(' ').next().unwrap()), "%Y %m-%d").expect("날짜를 파싱하는 데 실패했습니다.");
            let today_20_clock = now.with_hour(20).unwrap().with_minute(0).unwrap().with_second(0).unwrap().with_nanosecond(0).unwrap();

            if text.contains("KB") || text.contains("韩国围甲") && livedtl_date == now.date_naive() && now < today_20_clock {
                let mut live_win_probability = 50.0;
                let (name1, elo1, elo2) = if text.contains(match_result.first_rapid().player1().chinese_name()) && text.contains(match_result.first_rapid().player2().chinese_name()) {
                    (
                        match_result.first_rapid().player1().chinese_name(),
                        match_result.first_rapid().player1().elo_rating() + match_result.first_rapid().player1().condition_weight() + match_result.first_rapid().player1().rapid_weight(),
                        match_result.first_rapid().player2().elo_rating() + match_result.first_rapid().player2().condition_weight() + match_result.first_rapid().player2().rapid_weight()
                    )
                } else if text.contains(match_result.second_blitz().player1().chinese_name()) && text.contains(match_result.second_blitz().player2().chinese_name()) {
                    (
                        match_result.second_blitz().player1().chinese_name(),
                        match_result.second_blitz().player1().elo_rating() + match_result.second_blitz().player1().condition_weight() + match_result.second_blitz().player1().rapid_weight(),
                        match_result.second_blitz().player2().elo_rating() + match_result.second_blitz().player2().condition_weight() + match_result.second_blitz().player2().rapid_weight()
                    )
                } else if text.contains(match_result.third_blitz().player1().chinese_name()) && text.contains(match_result.third_blitz().player2().chinese_name()) {
                    (
                        match_result.third_blitz().player1().chinese_name(),
                        match_result.third_blitz().player1().elo_rating() + match_result.third_blitz().player1().condition_weight() + match_result.third_blitz().player1().rapid_weight(),
                        match_result.third_blitz().player2().elo_rating() + match_result.third_blitz().player2().condition_weight() + match_result.third_blitz().player2().rapid_weight()
                    )
                } else if text.contains(match_result.forth_blitz().player1().chinese_name()) && text.contains(match_result.forth_blitz().player2().chinese_name()) {
                    (
                        match_result.forth_blitz().player1().chinese_name(),
                        match_result.forth_blitz().player1().elo_rating() + match_result.forth_blitz().player1().condition_weight() + match_result.forth_blitz().player1().rapid_weight(),
                        match_result.forth_blitz().player2().elo_rating() + match_result.forth_blitz().player2().condition_weight() + match_result.forth_blitz().player2().rapid_weight()
                    )
                } else {
                    (match_result.first_rapid().player1().chinese_name(), 0.0, 0.0)
                };
                let b_player = match_element.find(Locator::Css("div.livedtl_first")).await.expect("div.livedtl_first 요소를 찾는 중 오류가 발생했습니다.").text().await.expect("텍스트를 가져오는 중 오류가 발생했습니다.");
                let w_player = match_element.find(Locator::Css("div.livedtl_third")).await.expect("div.livedtl_third 요소를 찾는 중 오류가 발생했습니다.").text().await.expect("텍스트를 가져오는 중 오류가 발생했습니다.");
                c.update_timeouts(TimeoutConfiguration::new(Some(Duration::from_millis(100)), Some(Duration::from_millis(100)), Some(Duration::from_millis(100)))).await.expect("타임아웃 설정 실패");
                if match_element.find(Locator::Css("div.progress_bar_text_box")).await.is_ok() {
                    c.update_timeouts(TimeoutConfiguration::new(Some(Duration::from_secs(10)), Some(Duration::from_secs(10)), Some(Duration::from_secs(10)))).await.expect("타임아웃 설정 실패");
                    let ai_title_font = match match_element.find(Locator::Css("span.overwrap.flex_item.center")).await {
                        Ok(element) => {
                            let ai_title_font_text = element.text().await.expect("텍스트를 가져오는 중 오류가 발생했습니다.");
                            ai_title_font_text.chars().filter(|c| c.is_digit(10) || *c == '.').collect::<String>().parse::<f64>().expect("숫자로 변환하는 데 실패했습니다.")
                        },
                        Err(_) => 0.0,
                    };
                    let now_sn_text = match_element.find(Locator::Css("span.overwrap.flex_item:not(.center):not(.text_right)")).await.expect("span.overwrap.flex_item:not(.center):not(.text_right) 요소를 찾는 중 오류가 발생했습니다.").text().await.expect("텍스트를 가져오는 중 오류가 발생했습니다.");
                    let now_sn = now_sn_text.chars().filter(|c| c.is_digit(10) || *c == '.').collect::<String>().parse::<f64>().expect("숫자로 변환하는 데 실패했습니다.");
                    let ai_bwin_text = match_element.find(Locator::Css("span.progress_bar_text.left")).await.expect("span.progress_bar_text.left 요소를 찾는 중 오류가 발생했습니다.").text().await.expect("텍스트를 가져오는 중 오류가 발생했습니다.");
                    let ai_bwin = ai_bwin_text.chars().filter(|c| c.is_digit(10) || *c == '.').collect::<String>().parse::<f64>().expect("숫자로 변환하는 데 실패했습니다.");
                    let ai_wwin_text = match_element.find(Locator::Css("span.progress_bar_text.right")).await.expect("span.progress_bar_text.right 요소를 찾는 중 오류가 발생했습니다.").text().await.expect("텍스트를 가져오는 중 오류가 발생했습니다.");
                    let ai_wwin = ai_wwin_text.chars().filter(|c| c.is_digit(10) || *c == '.').collect::<String>().parse::<f64>().expect("숫자로 변환하는 데 실패했습니다.");
                    let ai_win = if b_player.contains(name1) {
                        ai_bwin
                    } else if w_player.contains(name1) {
                        ai_wwin
                    } else {
                        50.0
                    };
                    let current_elo_win_probability = if text.contains(match_result.first_rapid().player1().chinese_name()) && text.contains(match_result.first_rapid().player2().chinese_name()) {
                        match_result.first_rapid_win_probability()
                    } else if text.contains(match_result.second_blitz().player1().chinese_name()) && text.contains(match_result.second_blitz().player2().chinese_name()) {
                        match_result.second_blitz_win_probability()
                    } else if text.contains(match_result.third_blitz().player1().chinese_name()) && text.contains(match_result.third_blitz().player2().chinese_name()) {
                        match_result.third_blitz_win_probability()
                    } else if text.contains(match_result.forth_blitz().player1().chinese_name()) && text.contains(match_result.forth_blitz().player2().chinese_name()) {
                        match_result.forth_blitz_win_probability()
                    } else {
                        50.0
                    };

                    live_win_probability = (ai_win * ai_title_font * now_sn * now_sn * now_sn * ((elo1 + elo2) * 0.0000000002 - 0.0000025) * (5.152 + (-0.0883 * current_elo_win_probability) + (-0.0883 * ai_win) + (0.000339 * current_elo_win_probability * current_elo_win_probability) + (0.000339 * ai_win * ai_win) + (0.0010875 * current_elo_win_probability * ai_win)) + current_elo_win_probability) / (ai_title_font * now_sn * now_sn * now_sn * ((elo1 + elo2) * 0.0000000002 - 0.0000025) * (5.152 + (-0.0883 * current_elo_win_probability) + (-0.0883 * ai_win) + (0.000339 * current_elo_win_probability * current_elo_win_probability) + (0.000339 * ai_win * ai_win) + (0.0010875 * current_elo_win_probability * ai_win)) + 1.0)
                } else {
                    c.update_timeouts(TimeoutConfiguration::new(Some(Duration::from_secs(10)), Some(Duration::from_secs(10)), Some(Duration::from_secs(10)))).await.expect("타임아웃 설정 실패");
                    let winner = if let Ok(element) = match_element.find(Locator::Css("span.livedtl_tag_black")).await {
                        element.text().await.expect("텍스트를 가져오는 중 오류가 발생했습니다.")
                    } else {
                        continue;
                    };

                    if winner.contains("黑胜") || winner.contains("黑中盘胜") {
                        if b_player.contains(name1) {
                            live_win_probability = 100.0;
                        } else if w_player.contains(name1) {
                            live_win_probability = 0.0;
                        }
                    } else if winner.contains("白胜") || winner.contains("白中盘胜") {
                        if b_player.contains(name1) {
                            live_win_probability = 0.0;
                        } else if w_player.contains(name1) {
                            live_win_probability = 100.0;
                        }
                    } else {
                        continue;
                    };
                }
                if text.contains(match_result.first_rapid().player1().chinese_name()) && text.contains(match_result.first_rapid().player2().chinese_name()) {
                    live_match_result.set_first_rapid_win_probability(live_win_probability);
                } else if text.contains(match_result.second_blitz().player1().chinese_name()) && text.contains(match_result.second_blitz().player2().chinese_name()) {
                    live_match_result.set_second_blitz_win_probability(live_win_probability);
                } else if text.contains(match_result.third_blitz().player1().chinese_name()) && text.contains(match_result.third_blitz().player2().chinese_name()) {
                    live_match_result.set_third_blitz_win_probability(live_win_probability);
                } else if text.contains(match_result.forth_blitz().player1().chinese_name()) && text.contains(match_result.forth_blitz().player2().chinese_name()) {
                    live_match_result.set_forth_blitz_win_probability(live_win_probability);
                }
            }
        }

        if c.refresh().await.is_err() {
            continue;
        }
        if c.wait().for_element(Locator::Css("div.ivu-col.ivu-col-span-24")).await.is_err() {
            if let Err(e) = c.refresh().await {
                eprintln!("새로고침 하는 중 오류가 발생했습니다: {}", e);
                continue;
            }
        }
        c.find(Locator::Css("div.ivu-col.ivu-col-span-24")).await.expect("div.ivu-col.ivu-col-span-24 요소를 찾는 중 오류가 발생했습니다.").click().await.expect("클릭하는 데 실패했습니다.");

        let mapped_tiebreaker_win_probability: Vec<TiebreakerRelativity> = player_relativities.iter()
            .map(|relativity| {
                let player1_position = [
                    live_match_result.first_rapid().player1().korean_name(),
                    live_match_result.second_blitz().player1().korean_name(),
                    live_match_result.third_blitz().player1().korean_name(),
                    live_match_result.forth_blitz().player1().korean_name(),
                ].iter().position(|&name| name == relativity.player1().korean_name());
                let player2_position = [
                    live_match_result.first_rapid().player2().korean_name(),
                    live_match_result.second_blitz().player2().korean_name(),
                    live_match_result.third_blitz().player2().korean_name(),
                    live_match_result.forth_blitz().player2().korean_name(),
                ].iter().position(|&name| name == relativity.player2().korean_name());

                let player1_penalty = if let Some(pos) = player1_position {
                    match pos {
                        0 => (1.0 / 1.04) * (1.0 / (1.0 + (0.04 * (1.0 - live_match_result.first_rapid_win_probability() / 100.0)))),
                        1 => (1.0 / 1.02) * (1.0 / (1.0 + (0.02 * (1.0 - live_match_result.second_blitz_win_probability() / 100.0)))),
                        2 => (1.0 / 1.08) * (1.0 / (1.0 + (0.08 * (1.0 - live_match_result.third_blitz_win_probability() / 100.0)))),
                        3 => (1.0 / 1.08) * (1.0 / (1.0 + (0.08 * (1.0 - live_match_result.forth_blitz_win_probability() / 100.0)))),
                        _ => 1.0,
                    }
                } else {
                    1.0
                };
                let player2_penalty = if let Some(pos) = player2_position {
                    match pos {
                        0 => 1.04 * (1.0 + (0.04 * (1.0 - live_match_result.first_rapid_win_probability() / 100.0))),
                        1 => 1.02 * (1.0 + (0.02 * (1.0 - live_match_result.second_blitz_win_probability() / 100.0))),
                        2 => 1.08 * (1.0 + (0.08 * (1.0 - live_match_result.third_blitz_win_probability() / 100.0))),
                        3 => 1.08 * (1.0 + (0.08 * (1.0 - live_match_result.forth_blitz_win_probability() / 100.0))),
                        _ => 1.0,
                    }
                } else {
                    1.0
                };
                TiebreakerRelativity::new(
                    relativity.player1().clone(), 
                    relativity.player2().clone(), 
                    relativity.bullet_win_probability() * player1_penalty * player2_penalty
                )
            })
            .collect();

        let mut tiebreaker_name1 = String::new();
        let mut tiebreaker_name2 = String::new();
        let mut tiebreaker_live_win_probability = 50.0;
        for match_element in matches {
            let text = match match_element.text().await {
                Ok(t) => t,
                Err(_) => continue,
            };

            let livedtl_time = match_element.find(Locator::Css("span.livedtl_time")).await.expect("span.livedtl_time 요소를 찾는 중 오류가 발생했습니다.").text().await.expect("텍스트를 가져오는 중 오류가 발생했습니다.");

            let now = chrono::Local::now();
            let livedtl_date = chrono::NaiveDate::parse_from_str(&format!("{} {}", now.format("%Y"), livedtl_time.split(' ').next().unwrap()), "%Y %m-%d").expect("날짜를 파싱하는 데 실패했습니다.");
            let today_20_clock = now.with_hour(20).unwrap().with_minute(0).unwrap().with_second(0).unwrap().with_nanosecond(0).unwrap();

            if text.contains("KB") || text.contains("韩国围甲") && livedtl_date == now.date_naive() && now >= today_20_clock {
                let b_player = match_element.find(Locator::Css("div.livedtl_first")).await.expect("div.livedtl_first 요소를 찾는 중 오류가 발생했습니다.").text().await.expect("텍스트를 가져오는 중 오류가 발생했습니다.");
                let w_player = match_element.find(Locator::Css("div.livedtl_third")).await.expect("div.livedtl_third 요소를 찾는 중 오류가 발생했습니다.").text().await.expect("텍스트를 가져오는 중 오류가 발생했습니다.");

                println!("흑돌 플레이어: {}", b_player);
                println!("백돌 플레이어: {}", w_player);

                let relevant_tiebreaker = mapped_tiebreaker_win_probability.iter().find(|tiebreaker| {
                    (b_player.contains(tiebreaker.player1().chinese_name()) || w_player.contains(tiebreaker.player1().chinese_name())) &&
                    (b_player.contains(tiebreaker.player2().chinese_name()) || w_player.contains(tiebreaker.player2().chinese_name()))
                });
                if let Some(tiebreaker) = relevant_tiebreaker {
                    tiebreaker_name1 = tiebreaker.player1().korean_name().clone();
                    tiebreaker_name2 = tiebreaker.player2().korean_name().clone();

                    let (name1, elo1, elo2) = if text.contains(tiebreaker.player1().chinese_name()) {
                        (
                            tiebreaker.player1().chinese_name(),
                            tiebreaker.player1().elo_rating() + tiebreaker.player1().condition_weight() + tiebreaker.player1().bullet_weight(),
                            tiebreaker.player2().elo_rating() + tiebreaker.player2().condition_weight() + tiebreaker.player2().bullet_weight()
                        )
                    } else {
                        (tiebreaker.player1().chinese_name(), 0.0, 0.0)
                    };

                    c.update_timeouts(TimeoutConfiguration::new(Some(Duration::from_millis(100)), Some(Duration::from_millis(100)), Some(Duration::from_millis(100)))).await.expect("타임아웃 설정 실패");
                    if match_element.find(Locator::Css("div.progress_bar_text_box")).await.is_ok() {
                        c.update_timeouts(TimeoutConfiguration::new(Some(Duration::from_secs(10)), Some(Duration::from_secs(10)), Some(Duration::from_secs(10)))).await.expect("타임아웃 설정 실패");
                        let ai_title_font = match match_element.find(Locator::Css("span.overwrap.flex_item.center")).await {
                            Ok(element) => {
                                let ai_title_font_text = element.text().await.expect("텍스트를 가져오는 중 오류가 발생했습니다.");
                                ai_title_font_text.chars().filter(|c| c.is_digit(10) || *c == '.').collect::<String>().parse::<f64>().expect("숫자로 변환하는 데 실패했습니다.")
                            },
                            Err(_) => 0.0,
                        };
                        let now_sn_text = match_element.find(Locator::Css("span.overwrap.flex_item:not(.center):not(.text_right)")).await.expect("span.overwrap.flex_item:not(.center):not(.text_right) 요소를 찾는 중 오류가 발생했습니다.").text().await.expect("텍스트를 가져오는 중 오류가 발생했습니다.");
                        let now_sn = now_sn_text.chars().filter(|c| c.is_digit(10) || *c == '.').collect::<String>().parse::<f64>().expect("숫자로 변환하는 데 실패했습니다.");
                        let ai_bwin_text = match_element.find(Locator::Css("span.progress_bar_text.left")).await.expect("span.progress_bar_text.left 요소를 찾는 중 오류가 발생했습니다.").text().await.expect("텍스트를 가져오는 중 오류가 발생했습니다.");
                        let ai_bwin = ai_bwin_text.chars().filter(|c| c.is_digit(10) || *c == '.').collect::<String>().parse::<f64>().expect("숫자로 변환하는 데 실패했습니다.");
                        let ai_wwin_text = match_element.find(Locator::Css("span.progress_bar_text.right")).await.expect("span.progress_bar_text.right 요소를 찾는 중 오류가 발생했습니다.").text().await.expect("텍스트를 가져오는 중 오류가 발생했습니다.");
                        let ai_wwin = ai_wwin_text.chars().filter(|c| c.is_digit(10) || *c == '.').collect::<String>().parse::<f64>().expect("숫자로 변환하는 데 실패했습니다.");
                        let ai_win = if b_player.contains(name1) {
                            ai_bwin
                        } else if w_player.contains(name1) {
                            ai_wwin
                        } else {
                            50.0
                        };

                        let current_elo_win_probability = tiebreaker.win_probability();
                        // tiebreaker_live_win_probability = (ai_win * ai_title_font * now_sn * now_sn * now_sn * 0.0000005 + current_elo_win_probability) / (ai_title_font * now_sn * now_sn * now_sn * 0.0000005 + 1.0);
                        tiebreaker_live_win_probability = (ai_win * ai_title_font * now_sn * now_sn * now_sn * ((elo1 + elo2) * 0.0000000002 - 0.0000025) * (5.152 + (-0.0883 * current_elo_win_probability) + (-0.0883 * ai_win) + (0.000339 * current_elo_win_probability * current_elo_win_probability) + (0.000339 * ai_win * ai_win) + (0.0010875 * current_elo_win_probability * ai_win)) + current_elo_win_probability) / (ai_title_font * now_sn * now_sn * now_sn * ((elo1 + elo2) * 0.0000000002 - 0.0000025) * (5.152 + (-0.0883 * current_elo_win_probability) + (-0.0883 * ai_win) + (0.000339 * current_elo_win_probability * current_elo_win_probability) + (0.000339 * ai_win * ai_win) + (0.0010875 * current_elo_win_probability * ai_win)) + 1.0)
                    } else {
                        c.update_timeouts(TimeoutConfiguration::new(Some(Duration::from_secs(10)), Some(Duration::from_secs(10)), Some(Duration::from_secs(10)))).await.expect("타임아웃 설정 실패");
                        let winner = if let Ok(element) = match_element.find(Locator::Css("span.livedtl_tag_black")).await {
                            element.text().await.expect("텍스트를 가져오는 중 오류가 발생했습니다.")
                        } else {
                            continue;
                        };

                        if winner.contains("黑胜") || winner.contains("黑中盘胜") {
                            if b_player.contains(name1) {
                                tiebreaker_live_win_probability = 100.0;
                            } else if w_player.contains(name1) {
                                tiebreaker_live_win_probability = 0.0;
                            }
                        } else if winner.contains("白胜") || winner.contains("白中盘胜") {
                            if b_player.contains(name1) {
                                tiebreaker_live_win_probability = 0.0;
                            } else if w_player.contains(name1) {
                                tiebreaker_live_win_probability = 100.0;
                            }
                        } else {
                            continue;
                        };
                    }
                } else {
                    continue;
                }
            }
        }

        let team1_tiebreaker_details = mapped_tiebreaker_win_probability.iter()
            .fold(HashMap::<String, Vec<&TiebreakerRelativity>>::new(), |mut acc, relativity| {
                let player1_name = relativity.player1().korean_name();
                acc.entry(player1_name.to_string()).or_insert_with(Vec::new).push(relativity);
                acc
            })
            .values()
            .map(|relativities| {
                relativities.iter().min_by(|a, b| a.win_probability().partial_cmp(&b.win_probability()).unwrap()).unwrap()
            })
            .max_by(|a, b| a.win_probability().partial_cmp(&b.win_probability()).unwrap())
            .cloned();

        let team2_tiebreaker_details = mapped_tiebreaker_win_probability.iter()
            .fold(HashMap::<String, Vec<&TiebreakerRelativity>>::new(), |mut acc, relativity| {
                let player2_name = relativity.player2().korean_name();
                acc.entry(player2_name.to_string()).or_insert_with(Vec::new).push(relativity);
                acc
            })
            .values()
            .map(|relativities| {
                relativities.iter().max_by(|a, b| a.win_probability().partial_cmp(&b.win_probability()).unwrap()).unwrap()
            })
            .min_by(|a, b| a.win_probability().partial_cmp(&b.win_probability()).unwrap())
            .cloned();

        let tiebreaker_win_probability = (team1_tiebreaker_details.map_or(0.0, |details| details.win_probability()) + team2_tiebreaker_details.map_or(0.0, |details| details.win_probability())) / 2.0;

        let win_probabilities = [
            live_match_result.first_rapid_win_probability(),
            live_match_result.second_blitz_win_probability(),
            live_match_result.third_blitz_win_probability(),
            live_match_result.forth_blitz_win_probability(),
        ];

        let all_win_probability = win_probabilities.iter().map(|p| p / 100.0).product::<f64>();

        let three_win_one_lose_probability = win_probabilities.iter().enumerate().map(|(i, &win_prob)| {
            let lose_prob = 1.0 - (win_prob / 100.0);
            win_probabilities.iter().enumerate().filter(|&(j, _)| i != j).map(|(_, &other_win_prob)| other_win_prob / 100.0).product::<f64>() * lose_prob
        }).sum::<f64>();

        let two_win_two_lose_probability = win_probabilities.iter().enumerate().combinations(2).map(|win_indices| {
            let win_prob_product = win_indices.iter().map(|&(i, _)| win_probabilities[i] / 100.0).product::<f64>();
            let lose_indices = (0..win_probabilities.len()).filter(|i| !win_indices.iter().any(|&(wi, _)| wi == *i)).collect::<Vec<_>>();
            let lose_prob_product = lose_indices.iter().map(|&i| 1.0 - (win_probabilities[i] / 100.0)).product::<f64>();
            win_prob_product * lose_prob_product
        }).sum::<f64>();

        let one_win_three_lose_probability = win_probabilities.iter().enumerate().map(|(i, &win_prob)| {
            let win_prob = win_prob / 100.0;
            win_probabilities.iter().enumerate().filter(|&(j, _)| i != j).map(|(_, &other_lose_prob)| 1.0 - (other_lose_prob / 100.0)).product::<f64>() * win_prob
        }).sum::<f64>();

        let all_lose_probability = win_probabilities.iter().map(|&win_prob| 1.0 - (win_prob / 100.0)).product::<f64>();

        let tie_win_probability = two_win_two_lose_probability * (tiebreaker_win_probability / 100.0);

        let total_win_probability = tie_win_probability + three_win_one_lose_probability + all_win_probability;

        live_match_result.set_four_zero_probability(all_win_probability * 100.0);
        live_match_result.set_three_one_probability(three_win_one_lose_probability * 100.0);
        live_match_result.set_two_two_probability(two_win_two_lose_probability * 100.0);
        live_match_result.set_one_three_probability(one_win_three_lose_probability * 100.0);
        live_match_result.set_zero_four_probability(all_lose_probability * 100.0);
        live_match_result.set_total_win_probability(total_win_probability * 100.0);
        live_match_result.set_tiebreaker_relativities(vec![team1_tiebreaker_details.cloned(), team2_tiebreaker_details.cloned()]);
        live_match_result.set_tiebreaker_win_probability(tiebreaker_win_probability);
        let four_zero = live_match_result.four_zero_probability() / 100.0;
        let three_one = live_match_result.three_one_probability() / 100.0;
        let two_two = live_match_result.two_two_probability() / 100.0;
        let one_three = live_match_result.one_three_probability() / 100.0;
        let zero_four = live_match_result.zero_four_probability() / 100.0;
        let tiebreaker = live_match_result.tiebreaker_win_probability() / 100.0;
        let team1_score = 4.0 * four_zero + 3.0 * three_one + 2.0 * two_two + 1.0 * one_three + two_two * tiebreaker;
        let team2_score = 1.0 * three_one + 2.0 * two_two + 3.0 * one_three + 4.0 * zero_four + two_two * (1.0 - tiebreaker);
        let player1_best_tiebreaker_names: HashSet<String> = live_match_result.tiebreaker_relativities().iter()
            .filter_map(|detail| detail.as_ref())
            .map(|detail| detail.player1().korean_name().to_string())
            .collect();
        let player2_best_tiebreaker_names: HashSet<String> = live_match_result.tiebreaker_relativities().iter()
            .filter_map(|detail| detail.as_ref())
            .map(|detail| detail.player2().korean_name().to_string())
            .collect();
        // let tiebreaker_details = if live_match_result.two_two_probability() > 0.0 {
        //     format!("5국 초속기(bullet): ({}) vs ({}) (승리확률: {:.2}%)",
        //         player1_best_tiebreaker_names.iter().cloned().collect::<Vec<_>>().join(", "),
        //         player2_best_tiebreaker_names.iter().cloned().collect::<Vec<_>>().join(", "),
        //         live_match_result.tiebreaker_win_probability()
        //     )
        // } else {
        //     String::new()
        // };
        let tiebreaker_details = if !tiebreaker_name1.is_empty() {
            format!("5국 초속기(bullet): {} vs {} (승리확률: {:.2}%)",
                tiebreaker_name1,
                tiebreaker_name2,
                tiebreaker_live_win_probability
            )
        } else if live_match_result.two_two_probability() > 0.0  {
            format!("5국 초속기(bullet): ({}) vs ({}) (승리확률: {:.2}%)",
                player1_best_tiebreaker_names.iter().cloned().collect::<Vec<_>>().join(", "),
                player2_best_tiebreaker_names.iter().cloned().collect::<Vec<_>>().join(", "),
                live_match_result.tiebreaker_win_probability()
            )
        } else {
            String::new()
        };

        let output = format!(
            "========================\n\
            1국 장고(rapid): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)\n\
            2국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)\n\
            3국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)\n\
            4국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)\n\
            {}
            \n4-0: {:.2}%\n\
            3-1: {:.2}%\n\
            3-2: {:.2}%\n\
            2-2: {:.2}%\n\
            2-3: {:.2}%\n\
            1-3: {:.2}%\n\
            0-4: {:.2}%\n\
            \n총 승리확률: {:.2}%\n\
            \n현재 스코어: {:.2}-{:.2}\n\
            ========================",
            live_match_result.first_rapid().player1().korean_name(),
            live_match_result.first_rapid().player2().korean_name(),
            chrono::Utc::now().year() - 3,
            live_match_result.first_rapid().player1_wins(),
            live_match_result.first_rapid().player2_wins(),
            live_match_result.first_rapid_win_probability(),
            live_match_result.second_blitz().player1().korean_name(),
            live_match_result.second_blitz().player2().korean_name(),
            chrono::Utc::now().year() - 3,
            live_match_result.second_blitz().player1_wins(),
            live_match_result.second_blitz().player2_wins(),
            live_match_result.second_blitz_win_probability(),
            live_match_result.third_blitz().player1().korean_name(),
            live_match_result.third_blitz().player2().korean_name(),
            chrono::Utc::now().year() - 3,
            live_match_result.third_blitz().player1_wins(),
            live_match_result.third_blitz().player2_wins(),
            live_match_result.third_blitz_win_probability(),
            live_match_result.forth_blitz().player1().korean_name(),
            live_match_result.forth_blitz().player2().korean_name(),
            chrono::Utc::now().year() - 3,
            live_match_result.forth_blitz().player1_wins(),
            live_match_result.forth_blitz().player2_wins(),
            live_match_result.forth_blitz_win_probability(),
            tiebreaker_details,
            live_match_result.four_zero_probability(),
            live_match_result.three_one_probability(),
            live_match_result.two_two_probability() * (live_match_result.tiebreaker_win_probability() / 100.0),
            live_match_result.two_two_probability(),
            live_match_result.two_two_probability() * (1.0 - (live_match_result.tiebreaker_win_probability() / 100.0)),
            live_match_result.one_three_probability(),
            live_match_result.zero_four_probability(),
            live_match_result.total_win_probability(),
            team1_score, team2_score
        );
        execute!(stdout, MoveTo(0, 0)).expect("커서를 이동하는 데 실패했습니다.");
        execute!(stdout, Print(" ".repeat(2000))).expect("화면을 클리어하는 데 실패했습니다.");
        execute!(stdout, MoveTo(0, 0)).expect("커서를 이동하는 데 실패했습니다.");
        execute!(stdout, Print(output)).expect("텍스트를 출력하는 데 실패했습니다.");
    }

    println!("\nWebDriver를 닫으려면 엔터를 누르세요.");
    let mut pause = String::new();
    io::stdin().read_line(&mut pause).expect("입력을 읽는 데 실패했습니다.");

    c.close().await.expect("WebDriver를 닫는 데 실패했습니다.");
}

fn speed_aging_curve(days_since_birth: f64) -> f64 {
    let k1: f64 = 81.04;
    let k2: f64 = -0.00224;
    let k3: f64 = 10775.89;
    let k4: f64 = 0.00314;
    let k5: f64 = -81.51;

    k1 / (1.0 + E.powf(-k2 * (days_since_birth - k3))) + k4 * days_since_birth + k5
}

pub fn get_recent_record(gisa1: &str, mut gisa1_rating: f64, rating_list: &HashMap<String, f64>) -> Result<f64, Box<dyn Error>> {
    let current_year = chrono::Utc::now().year();
    let current_month = format!("{:02}.", chrono::Utc::now().month());

    let url = format!("https://cyberoro.com/cooperate/giwon/gibo_M_in.oro?ydate={}&gisa1={}&listCnt=20&P_KEY=0", current_year, gisa1);
    let body = reqwest::blocking::get(&url)?.text()?;
    let document = Html::parse_document(&body);

    let match_selector = Selector::parse("tr").unwrap();
    let date_selector = Selector::parse("th.tb-date").unwrap();
    let player_selector = Selector::parse("td.text-center>span").unwrap();
    let winner_selector = Selector::parse("span[style='color:#1c97fe']").unwrap();

    let matches: Vec<_> = document.select(&match_selector).collect();
    let mut matches_to_process = Vec::new();

    for (index, selected_match) in matches.iter().enumerate() {
        if let Some(date_cell) = selected_match.select(&date_selector).next() {
            let date_text = date_cell.text().collect::<String>();
            if date_text.contains(&current_month) && index + 1 < matches.len() {
                matches_to_process.push(&matches[index + 1]);
            }
        }
    }

    for selected_match in matches_to_process.iter().rev() {
        let match_text = selected_match.text().collect::<Vec<_>>().join(" ");
        if match_text.contains(gisa1) {
            let players = selected_match.select(&player_selector).collect::<Vec<_>>();
            let winner_text = selected_match.select(&winner_selector).next().unwrap().text().collect::<String>();
            
            let gisa2 = players.iter()
                .find(|p| !p.text().collect::<String>().contains(&winner_text))
                .map_or_else(|| "".to_string(), |p| p.text().collect::<String>());
            if let Some(gisa2_rating) = rating_list.get(&gisa2) {
                let is_win = if winner_text.contains(gisa1) { 1.0 } else { 0.0 };
                let win_probability = calculate_win_probability(gisa1_rating, *gisa2_rating);
                gisa1_rating += is_win - win_probability;
            }
        }
    }

    Ok(gisa1_rating)
}