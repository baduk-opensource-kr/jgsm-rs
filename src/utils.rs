use crate::models::{Lineup, MatchResult, Player, PlayerRelativity, Team, TeamRelativity, TiebreakerRelativity, WPAResult};
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
use regex::Regex;
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
    if standard_error <= 0.0 {
        return relative_probability;
    } else if standard_error >= 1.0 {
        return base_probability;
    } else {
        return relative_probability * (1.0 - standard_error) + base_probability * standard_error;
    }
}

pub fn fetch_player_ratings_on_baeteil(year: &str, month: &str) -> Result<(HashMap<String, f64>, String), Box<dyn std::error::Error>> {
    let mut ratings = HashMap::new();
    let client = reqwest::blocking::Client::new();
    let res = client.post("http://baduk.or.kr/record/rankingPlayer_in.asp")
        .form(&[("pageNo", "1"), ("keyColumn", year), ("keyWord", month), ("etcKey1", "1")])
        .send()?;
    let body = res.text()?;
    let document = Html::parse_document(&body);
    let ranking_month_selector = Selector::parse("button.on").unwrap();
    let ranking_month_element = document.select(&ranking_month_selector).next().unwrap();
    let ranking_month_text = ranking_month_element.text().collect::<String>();
    let ranking_month = ranking_month_text.chars().filter(|c| c.is_digit(10)).collect::<String>();

    let player_selector = Selector::parse("table.tbstyle03 > tbody > tr").unwrap();
    let name_selector = Selector::parse("td:nth-child(2) a").unwrap();
    let rating_selector = Selector::parse("td:nth-child(3)").unwrap();

    for player in document.select(&player_selector) {
        if let Some(name_element) = player.select(&name_selector).next() {
            if let Some(rating_element) = player.select(&rating_selector).next() {
                let name = name_element.inner_html();
                let rating_str = rating_element.inner_html().chars().filter(|c| c.is_digit(10)).collect::<String>();
                if let Ok(rating) = rating_str.parse::<f64>() {
                    ratings.insert(name, rating);
                }
            }
        }
    }

    Ok((ratings, ranking_month))
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
    let syne_day = chrono::Utc::now().date_naive() - chrono::Duration::try_days(1095).unwrap();
    let mut records = HashMap::new();

    let re = Regex::new(r"choice\('[^']+', ?'(\d+)', ?'\d+'\)").unwrap();
    let choice_selector = Selector::parse("li[onclick]").unwrap();

    let url1 = format!("http://baduk.or.kr/common/search_pro.asp?keyword={}&R_name=player1&R_code=player1_code", gisa1);
    let body1 = reqwest::blocking::get(&url1)?.text()?;
    let document1 = Html::parse_document(&body1);
    let script_texts1 = document1.select(&choice_selector).map(|script| script.value().attr("onclick").unwrap_or_default().to_string()).collect::<Vec<_>>();
    let script_text1 = script_texts1.first().unwrap(); // 첫번째만 사용하도록 변경
    let gisa1_code = re.captures(script_text1).and_then(|cap| cap.get(1).map(|match_| match_.as_str().parse::<i32>().ok())).flatten().unwrap_or_default();

    let url2 = format!("http://baduk.or.kr/common/search_pro.asp?keyword={}&R_name=player2&R_code=player2_code", gisa2);
    let body2 = reqwest::blocking::get(&url2)?.text()?;
    let document2 = Html::parse_document(&body2);
    let script_texts2 = document2.select(&choice_selector).map(|script| script.value().attr("onclick").unwrap_or_default().to_string()).collect::<Vec<_>>();
    let script_text2 = script_texts2.first().unwrap();
    let gisa2_code = re.captures(script_text2).and_then(|cap| cap.get(1).map(|match_| match_.as_str().parse::<i32>().ok())).flatten().unwrap_or_default();

    let mut gisa1_wins = 0;
    let mut gisa2_wins = 0;

    let mut page_no = 1;
    loop {
        let url3 = format!("http://baduk.or.kr/record/diary_in.asp?foreignKey=&pageNo={}&keyWord={}&etcKey={}&etc2=1", page_no, gisa1_code, gisa2_code);
        let body3 = reqwest::blocking::get(&url3)?.text()?;
        let document3 = Html::parse_document(&body3);

        let match_selector = Selector::parse("tbody>tr").unwrap();
        let date_selector = Selector::parse("td.no").unwrap();

        let matches: Vec<_> = document3.select(&match_selector).collect();
        if matches.is_empty() {
            break;
        }
        let mut break_loop = false;

        for selected_match in matches.iter() {
            if let Some(date_element) = selected_match.select(&date_selector).next() {
                let date_text = date_element.text().collect::<String>();
                let match_date = NaiveDate::parse_from_str(&date_text, "%Y-%m-%d").ok();

                if let Some(match_date) = match_date {
                    if match_date < syne_day {
                        break_loop = true;
                        break;
                    }

                    let player_names = selected_match.select(&Selector::parse("td").unwrap())
                        .enumerate()
                        .filter_map(|(index, element)| {
                            if index == 2 || index == 3 {
                                Some(element.text().collect::<String>())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();

                    if !player_names.is_empty() {
                        if player_names[0] == gisa1 {
                            gisa1_wins += 1;
                        } else {
                            gisa2_wins += 1;
                        }
                    }
                }
            }
        }

        if break_loop {
            break;
        }

        page_no += 1;
    }

    records.insert(gisa1.to_string(), gisa1_wins);
    records.insert(gisa2.to_string(), gisa2_wins);

    Ok(records)
}

pub fn update_team_elo_ratings(selected_teams: &mut Vec<Team>) -> Result<(), Box<dyn Error>> {
    let (player_ratings_on_baeteil, ranking_month) = fetch_player_ratings_on_baeteil(&chrono::Utc::now().year().to_string(), &chrono::Utc::now().month().to_string())?;
    let player_ratings_on_goratings = fetch_player_ratings_on_goratings()?;

    for team in selected_teams.iter_mut() {
        for player in team.players_mut().iter_mut() {
            if let Some(&rating) = player_ratings_on_baeteil.get(player.korean_name()) {
                match get_recent_record(player.korean_name(), baeteil_to_goratings(rating), &player_ratings_on_baeteil, ranking_month.clone()) {
                    Ok(current_rating) => {
                        player.set_elo_rating(current_rating);
                    },
                    Err(_) => {
                        player.set_elo_rating(baeteil_to_goratings(rating));
                    }
                }
                player.set_blitz_weight(speed_aging_curve(player.get_days_since_birth()) / 2.0);
                player.set_bullet_weight(speed_aging_curve(player.get_days_since_birth()));

                if let Ok((white_weight, black_weight)) = get_color_weight(player.korean_name()) {
                    player.set_white_weight(white_weight);
                    player.set_black_weight(black_weight);
                }
            } else if let Some(&rating) = player_ratings_on_goratings.get(player.english_name()) {
                match get_recent_record(player.korean_name(), rating, &player_ratings_on_baeteil, ranking_month.clone()) {
                    Ok(current_rating) => {
                        player.set_elo_rating(current_rating);
                    },
                    Err(_) => {
                        player.set_elo_rating(rating);
                    }
                }
                player.set_blitz_weight(speed_aging_curve(player.get_days_since_birth()) / 2.0);
                player.set_bullet_weight(speed_aging_curve(player.get_days_since_birth()));
            }
        }
    }

    Ok(())
}

fn baeteil_to_goratings(x: f64) -> f64 {
    if x < 9250.0 {
        x - 6050.0
    } else {
        // Parameters for the logarithmic part
        let a = 740.332659;
        let b = 0.0946919155;
        let c = 8456.81141;

        a * (b * (x - c)).ln()
    }
}

pub fn generate_player_relativities(selected_teams: &Vec<Team>, first_rapid_black: bool, first_rapid_none_color: bool) -> Result<Vec<PlayerRelativity>, String> {
    let mut all_relative_records: Vec<PlayerRelativity> = Vec::new();

    let team1 = &selected_teams[0];
    let team2 = &selected_teams[1];
    for player1 in team1.players() {
        for player2 in team2.players() {
            let record = fetch_head_to_head_record(&player1.korean_name(), &player2.korean_name())
                               .map_err(|e| format!("상대전적을 가져오는 중 오류가 발생했습니다: {}", e))?;
            let player1_wins = *record.get(player1.korean_name()).unwrap_or(&0);
            let player2_wins = *record.get(player2.korean_name()).unwrap_or(&0);

            if first_rapid_none_color {
                if first_rapid_black {
                    let first_rapid_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating() + player1.condition_weight() + player1.rapid_weight() + player1.black_weight()) as f64, (player2.elo_rating() + player2.condition_weight() + player2.rapid_weight() + player2.white_weight()) as f64, player1_wins, player2_wins);
                    let second_blitz_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating() + player1.condition_weight() + player1.blitz_weight() + player1.white_weight()) as f64, (player2.elo_rating() + player2.condition_weight() + player2.blitz_weight() + player2.black_weight()) as f64, player1_wins, player2_wins);
                    let third_blitz_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating() + player1.condition_weight() + player1.blitz_weight() + player1.black_weight()) as f64, (player2.elo_rating() + player2.condition_weight() + player2.blitz_weight() + player2.white_weight()) as f64, player1_wins, player2_wins);
                    let forth_blitz_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating() + player1.condition_weight() + player1.blitz_weight() + player1.white_weight()) as f64, (player2.elo_rating() + player2.condition_weight() + player2.blitz_weight() + player2.black_weight()) as f64, player1_wins, player2_wins);
                    let fifth_bullet_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating() + player1.condition_weight() + player1.bullet_weight() + player1.black_weight()) as f64, (player2.elo_rating() + player2.condition_weight() + player2.bullet_weight() + player2.white_weight()) as f64, player1_wins, player2_wins);

                    all_relative_records.push(PlayerRelativity::new(
                        player1.clone(),
                        player2.clone(),
                        player1_wins,
                        player2_wins,
                        first_rapid_win_probability * 100.0,
                        second_blitz_win_probability * 100.0,
                        third_blitz_win_probability * 100.0,
                        forth_blitz_win_probability * 100.0,
                        fifth_bullet_win_probability * 100.0,
                    ));
                } else {
                    let first_rapid_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating() + player1.condition_weight() + player1.rapid_weight() + player1.white_weight()) as f64, (player2.elo_rating() + player2.condition_weight() + player2.rapid_weight() + player2.black_weight()) as f64, player1_wins, player2_wins);
                    let second_blitz_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating() + player1.condition_weight() + player1.blitz_weight() + player1.black_weight()) as f64, (player2.elo_rating() + player2.condition_weight() + player2.blitz_weight() + player2.white_weight()) as f64, player1_wins, player2_wins);
                    let third_blitz_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating() + player1.condition_weight() + player1.blitz_weight() + player1.white_weight()) as f64, (player2.elo_rating() + player2.condition_weight() + player2.blitz_weight() + player2.black_weight()) as f64, player1_wins, player2_wins);
                    let forth_blitz_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating() + player1.condition_weight() + player1.blitz_weight() + player1.black_weight()) as f64, (player2.elo_rating() + player2.condition_weight() + player2.blitz_weight() + player2.white_weight()) as f64, player1_wins, player2_wins);
                    let fifth_bullet_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating() + player1.condition_weight() + player1.bullet_weight() + player1.white_weight()) as f64, (player2.elo_rating() + player2.condition_weight() + player2.bullet_weight() + player2.black_weight()) as f64, player1_wins, player2_wins);

                    all_relative_records.push(PlayerRelativity::new(
                        player1.clone(),
                        player2.clone(),
                        player1_wins,
                        player2_wins,
                        first_rapid_win_probability * 100.0,
                        second_blitz_win_probability * 100.0,
                        third_blitz_win_probability * 100.0,
                        forth_blitz_win_probability * 100.0,
                        fifth_bullet_win_probability * 100.0,
                    ));
                }
            } else {
                let first_rapid_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating() + player1.condition_weight() + player1.rapid_weight()) as f64, (player2.elo_rating() + player2.condition_weight() + player2.rapid_weight()) as f64, player1_wins, player2_wins);
                let second_blitz_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating() + player1.condition_weight() + player1.blitz_weight()) as f64, (player2.elo_rating() + player2.condition_weight() + player2.blitz_weight()) as f64, player1_wins, player2_wins);
                let third_blitz_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating() + player1.condition_weight() + player1.blitz_weight()) as f64, (player2.elo_rating() + player2.condition_weight() + player2.blitz_weight()) as f64, player1_wins, player2_wins);
                let forth_blitz_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating() + player1.condition_weight() + player1.blitz_weight()) as f64, (player2.elo_rating() + player2.condition_weight() + player2.blitz_weight()) as f64, player1_wins, player2_wins);
                let fifth_bullet_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating() + player1.condition_weight() + player1.bullet_weight()) as f64, (player2.elo_rating() + player2.condition_weight() + player2.bullet_weight()) as f64, player1_wins, player2_wins);

                all_relative_records.push(PlayerRelativity::new(
                    player1.clone(),
                    player2.clone(),
                    player1_wins,
                    player2_wins,
                    first_rapid_win_probability * 100.0,
                    second_blitz_win_probability * 100.0,
                    third_blitz_win_probability * 100.0,
                    forth_blitz_win_probability * 100.0,
                    fifth_bullet_win_probability * 100.0,
                ));
            }
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
                    0 => relativity.first_rapid_win_probability(),
                    1 => relativity.second_blitz_win_probability(),
                    2 => relativity.third_blitz_win_probability(),
                    3 => relativity.forth_blitz_win_probability(),
                    _ => relativity.fifth_bullet_win_probability(),
                };
                bullet_win_probabilities[i] = relativity.fifth_bullet_win_probability();
            }
        }
    }

    let mapped_tiebreaker_win_probability: Vec<TiebreakerRelativity> = player_relativities.iter()
        .map(|relativity| {
            let player1_position = team1_players.iter().position(|p| p.korean_name() == relativity.player1().korean_name());
            let player2_position = team2_players.iter().position(|p| p.korean_name() == relativity.player2().korean_name());
            let player1_penalty = if let Some(pos) = player1_position {
                match pos {
                    0 => (1.0 / 1.04) * (1.0 / (1.0 + (0.04 * (1.0 - relativity.first_rapid_win_probability() / 100.0)))),
                    1 => (1.0 / 1.02) * (1.0 / (1.0 + (0.02 * (1.0 - relativity.second_blitz_win_probability() / 100.0)))),
                    2 => (1.0 / 1.08) * (1.0 / (1.0 + (0.08 * (1.0 - relativity.third_blitz_win_probability() / 100.0)))),
                    3 => (1.0 / 1.08) * (1.0 / (1.0 + (0.08 * (1.0 - relativity.forth_blitz_win_probability() / 100.0)))),
                    _ => 1.0,
                }
            } else {
                1.0
            };
            let player2_penalty = if let Some(pos) = player2_position {
                match pos {
                    0 => 1.04 * (1.0 + (0.04 * (1.0 - relativity.first_rapid_win_probability() / 100.0))),
                    1 => 1.02 * (1.0 + (0.02 * (1.0 - relativity.second_blitz_win_probability() / 100.0))),
                    2 => 1.08 * (1.0 + (0.08 * (1.0 - relativity.third_blitz_win_probability() / 100.0))),
                    3 => 1.08 * (1.0 + (0.08 * (1.0 - relativity.forth_blitz_win_probability() / 100.0))),
                    _ => 1.0,
                }
            } else {
                1.0
            };
            TiebreakerRelativity::new(
                relativity.player1().clone(), 
                relativity.player2().clone(), 
                relativity.fifth_bullet_win_probability() * player1_penalty * player2_penalty
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
    let mut worksheet_first_rapid = workbook.add_worksheet(Some("1국 장고 기반"))?;
    let mut worksheet_second_blitz = workbook.add_worksheet(Some("2국 속기 기반"))?;
    let mut worksheet_third_blitz = workbook.add_worksheet(Some("3국 속기 기반"))?;
    let mut worksheet_forth_blitz = workbook.add_worksheet(Some("4국 속기 기반"))?;
    let mut worksheet_fifth_bullet = workbook.add_worksheet(Some("5국 초속기 기반"))?;

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
        worksheet_first_rapid.write_string((index + 1).try_into().unwrap(), 0, player, None)?;
        worksheet_second_blitz.write_string((index + 1).try_into().unwrap(), 0, player, None)?;
        worksheet_third_blitz.write_string((index + 1).try_into().unwrap(), 0, player, None)?;
        worksheet_forth_blitz.write_string((index + 1).try_into().unwrap(), 0, player, None)?;
        worksheet_fifth_bullet.write_string((index + 1).try_into().unwrap(), 0, player, None)?;
        player1_index.insert(player.clone(), index + 1);
    }

    for (index, player) in player2s.iter().enumerate() {
        worksheet_first_rapid.write_string(0, (index + 1).try_into().unwrap(), player, None)?;
        worksheet_second_blitz.write_string(0, (index + 1).try_into().unwrap(), player, None)?;
        worksheet_third_blitz.write_string(0, (index + 1).try_into().unwrap(), player, None)?;
        worksheet_forth_blitz.write_string(0, (index + 1).try_into().unwrap(), player, None)?;
        worksheet_fifth_bullet.write_string(0, (index + 1).try_into().unwrap(), player, None)?;
        player2_index.insert(player.clone(), index + 1);
    }

    for relativity in &player_relativities {
        let row = player1_index[relativity.player1().korean_name()];
        let col = player2_index[relativity.player2().korean_name()];

        let first_rapid_format = create_custom_format(relativity.first_rapid_win_probability(), 15.0)?;
        let second_blitz_format = create_custom_format(relativity.second_blitz_win_probability(), 15.0)?;
        let third_blitz_format = create_custom_format(relativity.third_blitz_win_probability(), 15.0)?;
        let forth_blitz_format = create_custom_format(relativity.forth_blitz_win_probability(), 15.0)?;
        let fifth_bullet_format = create_custom_format(relativity.fifth_bullet_win_probability(), 15.0)?;

        worksheet_first_rapid.write_number(row.try_into().unwrap(), col.try_into().unwrap(), relativity.first_rapid_win_probability() / 100.0, Some(&first_rapid_format))?;
        worksheet_second_blitz.write_number(row.try_into().unwrap(), col.try_into().unwrap(), relativity.second_blitz_win_probability() / 100.0, Some(&second_blitz_format))?;
        worksheet_third_blitz.write_number(row.try_into().unwrap(), col.try_into().unwrap(), relativity.third_blitz_win_probability() / 100.0, Some(&third_blitz_format))?;
        worksheet_forth_blitz.write_number(row.try_into().unwrap(), col.try_into().unwrap(), relativity.forth_blitz_win_probability() / 100.0, Some(&forth_blitz_format))?;
        worksheet_fifth_bullet.write_number(row.try_into().unwrap(), col.try_into().unwrap(), relativity.fifth_bullet_win_probability() / 100.0, Some(&fifth_bullet_format))?;
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

            let format = create_custom_format(relativity.fifth_bullet_win_probability(), 15.0)?;

            worksheet.write_number(row.try_into().unwrap(), col.try_into().unwrap(), relativity.fifth_bullet_win_probability() / 100.0, Some(&format))?;
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
    let unknown_player = Player::new("알 수 없음".to_string(), "unknown".to_string(), "未知".to_string(), NaiveDate::from_ymd_opt(2000, 1, 1).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

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
        let mut first_rapid_now_sn = 0.0;
        let mut second_blitz_now_sn = 0.0;
        let mut third_blitz_now_sn = 0.0;
        let mut forth_blitz_now_sn = 0.0;
        for match_element in &matches {
            let text = match match_element.text().await {
                Ok(t) => t,
                Err(_) => continue,
            };

            let livedtl_el = match_element.find(Locator::Css("span.livedtl_time")).await.expect("span.livedtl_time 요소를 찾는 중 오류가 발생했습니다.").text().await.expect("텍스트를 가져오는 중 오류가 발생했습니다.");

            let now = chrono::Local::now();
            let livedtl_time = livedtl_el.split(' ').nth(1).unwrap();
            let livedtl_hour = livedtl_time.split(':').next().unwrap().parse::<u32>().expect("시간을 파싱하는 데 실패했습니다.");
            let livedtl_minute = livedtl_time.split(':').nth(1).unwrap().parse::<u32>().expect("분을 파싱하는 데 실패했습니다.");
            let today_20_clock = now.with_hour(20).unwrap().with_minute(0).unwrap().with_second(0).unwrap().with_nanosecond(0).unwrap();
            let livedtl_datetime = now.with_hour(livedtl_hour).unwrap().with_minute(livedtl_minute).unwrap().with_second(0).unwrap().with_nanosecond(0).unwrap();

            if (text.contains("KB") || text.contains("韩国围甲") || text.contains("韩围甲")) && livedtl_datetime < today_20_clock {
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
                c.update_timeouts(TimeoutConfiguration::new(Some(Duration::from_millis(500)), Some(Duration::from_millis(500)), Some(Duration::from_millis(500)))).await.expect("타임아웃 설정 실패");
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
                        first_rapid_now_sn = now_sn;
                        match_result.first_rapid_win_probability()
                    } else if text.contains(match_result.second_blitz().player1().chinese_name()) && text.contains(match_result.second_blitz().player2().chinese_name()) {
                        second_blitz_now_sn = now_sn;
                        match_result.second_blitz_win_probability()
                    } else if text.contains(match_result.third_blitz().player1().chinese_name()) && text.contains(match_result.third_blitz().player2().chinese_name()) {
                        third_blitz_now_sn = now_sn;
                        match_result.third_blitz_win_probability()
                    } else if text.contains(match_result.forth_blitz().player1().chinese_name()) && text.contains(match_result.forth_blitz().player2().chinese_name()) {
                        forth_blitz_now_sn = now_sn;
                        match_result.forth_blitz_win_probability()
                    } else {
                        50.0
                    };

                    live_win_probability = (ai_win * ai_title_font * now_sn * now_sn * now_sn * ((elo1 + elo2) * 0.0000000002 - 0.0000025) * (2.5351 - (0.0315 * current_elo_win_probability) - (0.0315 * ai_win) + (0.00008 * current_elo_win_probability * current_elo_win_probability) + (0.00008 * ai_win * ai_win) + (0.00047 * current_elo_win_probability * ai_win)) + current_elo_win_probability) / (ai_title_font * now_sn * now_sn * now_sn * ((elo1 + elo2) * 0.0000000002 - 0.0000025) * (2.5351 - (0.0315 * current_elo_win_probability) - (0.0315 * ai_win) + (0.00008 * current_elo_win_probability * current_elo_win_probability) + (0.00008 * ai_win * ai_win) + (0.00047 * current_elo_win_probability * ai_win)) + 1.0)
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
                    first_rapid_now_sn = 200.0;
                    live_match_result.set_first_rapid_win_probability(live_win_probability);
                } else if text.contains(match_result.second_blitz().player1().chinese_name()) && text.contains(match_result.second_blitz().player2().chinese_name()) {
                    second_blitz_now_sn = 200.0;
                    live_match_result.set_second_blitz_win_probability(live_win_probability);
                } else if text.contains(match_result.third_blitz().player1().chinese_name()) && text.contains(match_result.third_blitz().player2().chinese_name()) {
                    third_blitz_now_sn = 200.0;
                    live_match_result.set_third_blitz_win_probability(live_win_probability);
                } else if text.contains(match_result.forth_blitz().player1().chinese_name()) && text.contains(match_result.forth_blitz().player2().chinese_name()) {
                    forth_blitz_now_sn = 200.0;
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
                    relativity.fifth_bullet_win_probability() * player1_penalty * player2_penalty
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

            let livedtl_el = match_element.find(Locator::Css("span.livedtl_time")).await.expect("span.livedtl_time 요소를 찾는 중 오류가 발생했습니다.").text().await.expect("텍스트를 가져오는 중 오류가 발생했습니다.");

            let now = chrono::Local::now();
            let livedtl_time = livedtl_el.split(' ').nth(1).unwrap();
            let livedtl_hour = livedtl_time.split(':').next().unwrap().parse::<u32>().expect("시간을 파싱하는 데 실패했습니다.");
            let livedtl_minute = livedtl_time.split(':').nth(1).unwrap().parse::<u32>().expect("분을 파싱하는 데 실패했습니다.");
            let today_20_clock = now.with_hour(20).unwrap().with_minute(0).unwrap().with_second(0).unwrap().with_nanosecond(0).unwrap();
            let livedtl_datetime = now.with_hour(livedtl_hour).unwrap().with_minute(livedtl_minute).unwrap().with_second(0).unwrap().with_nanosecond(0).unwrap();

            if (text.contains("KB") || text.contains("韩国围甲") || text.contains("韩围甲")) && livedtl_datetime < today_20_clock {
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

                    c.update_timeouts(TimeoutConfiguration::new(Some(Duration::from_millis(500)), Some(Duration::from_millis(500)), Some(Duration::from_millis(500)))).await.expect("타임아웃 설정 실패");
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
                        tiebreaker_live_win_probability = (ai_win * ai_title_font * now_sn * now_sn * now_sn * ((elo1 + elo2) * 0.0000000002 - 0.0000025) * (2.5351 - (0.0315 * current_elo_win_probability) - (0.0315 * ai_win) + (0.00008 * current_elo_win_probability * current_elo_win_probability) + (0.00008 * ai_win * ai_win) + (0.00047 * current_elo_win_probability * ai_win)) + current_elo_win_probability) / (ai_title_font * now_sn * now_sn * now_sn * ((elo1 + elo2) * 0.0000000002 - 0.0000025) * (2.5351 - (0.0315 * current_elo_win_probability) - (0.0315 * ai_win) + (0.00008 * current_elo_win_probability * current_elo_win_probability) + (0.00008 * ai_win * ai_win) + (0.00047 * current_elo_win_probability * ai_win)) + 1.0)
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
        let tiebreaker_details = if !tiebreaker_name1.is_empty() {
            format!("5국 초속기(bullet):  {} vs {}\
                \n      예상승리확률: {:6.2}%  : {:6.2}%\n\
            ",
                tiebreaker_name1,
                tiebreaker_name2,
                tiebreaker_live_win_probability,
                100.0 - tiebreaker_live_win_probability
            )
        } else if live_match_result.two_two_probability() > 0.0  {
            format!("5국 초속기(bullet): ({}) vs ({})\
                \n      예상승리확률: {:6.2}%  : {:6.2}%\n\
            ",
                player1_best_tiebreaker_names.iter().cloned().collect::<Vec<_>>().join(", "),
                player2_best_tiebreaker_names.iter().cloned().collect::<Vec<_>>().join(", "),
                live_match_result.tiebreaker_win_probability(),
                100.0 - live_match_result.tiebreaker_win_probability()
            )
        } else {
            String::new()
        };

        let wpa_result = (|| {
            let mut match_result_for_first_rapid_wpa = match_result.clone();
            let mut match_result_for_first_rapid_zero = match_result.clone();
            let mut match_result_for_first_rapid_one = match_result.clone();
            match_result_for_first_rapid_wpa.set_first_rapid_win_probability(live_match_result.first_rapid_win_probability());
            match_result_for_first_rapid_zero.set_first_rapid_win_probability(0.0);
            match_result_for_first_rapid_one.set_first_rapid_win_probability(100.0);
            let mut match_result_for_second_blitz_wpa = match_result.clone();
            let mut match_result_for_second_blitz_zero = match_result.clone();
            let mut match_result_for_second_blitz_one = match_result.clone();
            match_result_for_second_blitz_wpa.set_second_blitz_win_probability(live_match_result.second_blitz_win_probability());
            match_result_for_second_blitz_zero.set_second_blitz_win_probability(0.0);
            match_result_for_second_blitz_one.set_second_blitz_win_probability(100.0);
            let mut match_result_for_third_blitz_wpa = match_result.clone();
            let mut match_result_for_third_blitz_zero = match_result.clone();
            let mut match_result_for_third_blitz_one = match_result.clone();
            match_result_for_third_blitz_wpa.set_third_blitz_win_probability(live_match_result.third_blitz_win_probability());
            match_result_for_third_blitz_zero.set_third_blitz_win_probability(0.0);
            match_result_for_third_blitz_one.set_third_blitz_win_probability(100.0);
            let mut match_result_for_forth_blitz_wpa = match_result.clone();
            let mut match_result_for_forth_blitz_zero = match_result.clone();
            let mut match_result_for_forth_blitz_one = match_result.clone();
            match_result_for_forth_blitz_wpa.set_forth_blitz_win_probability(live_match_result.forth_blitz_win_probability());
            match_result_for_forth_blitz_zero.set_forth_blitz_win_probability(0.0);
            match_result_for_forth_blitz_one.set_forth_blitz_win_probability(100.0);

            let (win_prob, team1_score, team2_score) = get_total_win_probability(match_result.clone(), &player_relativities);
            let (
                first_rapid_player1_wpa,
                first_rapid_player1_score,
                first_rapid_player2_wpa,
                first_rapid_player2_score,
                second_blitz_player1_wpa,
                second_blitz_player1_score,
                second_blitz_player2_wpa,
                second_blitz_player2_score,
                third_blitz_player1_wpa,
                third_blitz_player1_score,
                third_blitz_player2_wpa,
                third_blitz_player2_score,
                forth_blitz_player1_wpa,
                forth_blitz_player1_score,
                forth_blitz_player2_wpa,
                forth_blitz_player2_score
            ) = (|| {
                let (first_rapid_win_prob, first_rapid_team1_score, first_rapid_team2_score) = get_total_win_probability(match_result_for_first_rapid_wpa, &player_relativities);
                let (second_blitz_win_prob, second_blitz_team1_score, second_blitz_team2_score) = get_total_win_probability(match_result_for_second_blitz_wpa, &player_relativities);
                let (third_blitz_win_prob, third_blitz_team1_score, third_blitz_team2_score) = get_total_win_probability(match_result_for_third_blitz_wpa, &player_relativities);
                let (forth_blitz_win_prob, forth_blitz_team1_score, forth_blitz_team2_score) = get_total_win_probability(match_result_for_forth_blitz_wpa, &player_relativities);
                let (_, first_rapid_team1_zero, _) = get_total_win_probability(match_result_for_first_rapid_zero, &player_relativities);
                let (_, second_blitz_team1_zero, _) = get_total_win_probability(match_result_for_second_blitz_zero, &player_relativities);
                let (_, third_blitz_team1_zero, _) = get_total_win_probability(match_result_for_third_blitz_zero, &player_relativities);
                let (_, forth_blitz_team1_zero, _) = get_total_win_probability(match_result_for_forth_blitz_zero, &player_relativities);
                let (_, _, first_rapid_team2_zero) = get_total_win_probability(match_result_for_first_rapid_one, &player_relativities);
                let (_, _, second_blitz_team2_zero) = get_total_win_probability(match_result_for_second_blitz_one, &player_relativities);
                let (_, _, third_blitz_team2_zero) = get_total_win_probability(match_result_for_third_blitz_one, &player_relativities);
                let (_, _, forth_blitz_team2_zero) = get_total_win_probability(match_result_for_forth_blitz_one, &player_relativities);
                let average_team1_score = (first_rapid_team1_score + second_blitz_team1_score + third_blitz_team1_score + forth_blitz_team1_score - team1_score * 4.0) / 4.0;
                let average_team2_score = (first_rapid_team2_score + second_blitz_team2_score + third_blitz_team2_score + forth_blitz_team2_score - team2_score * 4.0) / 4.0;
                let (
                    team1_first_rapid_score,
                    team1_second_blitz_score,
                    team1_third_blitz_score,
                    team1_forth_blitz_score
                ) = redistribute_scores(
                    first_rapid_team1_score - first_rapid_team1_zero + first_rapid_team1_score - team1_score - average_team1_score,
                    second_blitz_team1_score - second_blitz_team1_zero + second_blitz_team1_score - team1_score - average_team1_score,
                    third_blitz_team1_score - third_blitz_team1_zero + third_blitz_team1_score - team1_score - average_team1_score,
                    forth_blitz_team1_score - forth_blitz_team1_zero + forth_blitz_team1_score - team1_score - average_team1_score
                );
                let (
                    team2_first_rapid_score,
                    team2_second_blitz_score,
                    team2_third_blitz_score,
                    team2_forth_blitz_score
                ) = redistribute_scores(
                    first_rapid_team2_score - first_rapid_team2_zero + first_rapid_team2_score - team2_score - average_team2_score,
                    second_blitz_team2_score - second_blitz_team2_zero + second_blitz_team2_score - team2_score - average_team2_score,
                    third_blitz_team2_score - third_blitz_team2_zero + third_blitz_team2_score - team2_score - average_team2_score,
                    forth_blitz_team2_score - forth_blitz_team2_zero + forth_blitz_team2_score - team2_score - average_team2_score
                );
                (
                    first_rapid_win_prob - win_prob,
                    interpolate(0.0, first_rapid_now_sn / 200.0, team1_first_rapid_score),
                    win_prob - first_rapid_win_prob,
                    interpolate(0.0, first_rapid_now_sn / 200.0, team2_first_rapid_score),
                    second_blitz_win_prob - win_prob,
                    interpolate(0.0, second_blitz_now_sn / 200.0, team1_second_blitz_score),
                    win_prob - second_blitz_win_prob,
                    interpolate(0.0, second_blitz_now_sn / 200.0, team2_second_blitz_score),
                    third_blitz_win_prob - win_prob,
                    interpolate(0.0, third_blitz_now_sn / 200.0, team1_third_blitz_score),
                    win_prob - third_blitz_win_prob,
                    interpolate(0.0, third_blitz_now_sn / 200.0, team2_third_blitz_score),
                    forth_blitz_win_prob - win_prob,
                    interpolate(0.0, forth_blitz_now_sn / 200.0, team1_forth_blitz_score),
                    win_prob - forth_blitz_win_prob,
                    interpolate(0.0, forth_blitz_now_sn / 200.0, team2_forth_blitz_score)
                )
            })();

            WPAResult::new(
                first_rapid_player1_wpa,
                first_rapid_player1_score,
                first_rapid_player2_wpa,
                first_rapid_player2_score,
                second_blitz_player1_wpa,
                second_blitz_player1_score,
                second_blitz_player2_wpa,
                second_blitz_player2_score,
                third_blitz_player1_wpa,
                third_blitz_player1_score,
                third_blitz_player2_wpa,
                third_blitz_player2_score,
                forth_blitz_player1_wpa,
                forth_blitz_player1_score,
                forth_blitz_player2_wpa,
                forth_blitz_player2_score,
                0.0,
                0.0,
                0.0,
                0.0
            )
        })();

        let output = format!("\n\
            1국 장고(rapid):  {} vs {} (최근3년 상대전적: {}-{})\
            \n   예상승리확률: {:6.2}%  : {:6.2}%          \
            \n           득점:   {:4.2}   :   {:4.2}     \
            \n            WPA: {}%p : {}%p           \n                              \n\
            2국 속기(blitz):  {} vs {} (최근3년 상대전적: {}-{})\
            \n   예상승리확률: {:6.2}%  : {:6.2}%          \
            \n           득점:   {:4.2}   :   {:4.2}     \
            \n            WPA: {}%p : {}%p           \n                              \n\
            3국 속기(blitz):  {} vs {} (최근3년 상대전적: {}-{})\
            \n   예상승리확률: {:6.2}%  : {:6.2}%          \
            \n           득점:   {:4.2}   :   {:4.2}     \
            \n            WPA: {}%p : {}%p           \n                              \n\
            4국 속기(blitz):  {} vs {} (최근3년 상대전적: {}-{})\
            \n   예상승리확률: {:6.2}%  : {:6.2}%          \
            \n           득점:   {:4.2}   :   {:4.2}     \
            \n            WPA: {}%p : {}%p           \n                              \n\
            {}\
            \n4-0: {:6.2}%                                   \n\
            3-1: {:6.2}%                                   \n\
            3-2: {:6.2}%                                   \n\
            2-2: {:6.2}%                                   \n\
            2-3: {:6.2}%                                   \n\
            1-3: {:6.2}%                                   \n\
            0-4: {:6.2}%                                   \n                                             \
            \n총 승리확률: {:6.2}%                              \n                                             \
            \n현재 스코어: {:.2}-{:.2}                              \
            \n                                             \
            \n                                             \
            \n                                             \
            ",
            live_match_result.first_rapid().player1().korean_name(),
            live_match_result.first_rapid().player2().korean_name(),
            live_match_result.first_rapid().player1_wins(),
            live_match_result.first_rapid().player2_wins(),
            live_match_result.first_rapid_win_probability(),
            100.0 - live_match_result.first_rapid_win_probability(),
            wpa_result.first_rapid_player1_score(),
            wpa_result.first_rapid_player2_score(),
            if wpa_result.first_rapid_player1_wpa() * 100.0 >= 0.0 {
                format!("+{:5.2}", wpa_result.first_rapid_player1_wpa() * 100.0)
            } else {
                format!("-{:5.2}", wpa_result.first_rapid_player1_wpa().abs() * 100.0)
            },
            if wpa_result.first_rapid_player2_wpa() * 100.0 >= 0.0 {
                format!("+{:5.2}", wpa_result.first_rapid_player2_wpa() * 100.0)
            } else {
                format!("-{:5.2}", wpa_result.first_rapid_player2_wpa().abs() * 100.0)
            },
            live_match_result.second_blitz().player1().korean_name(),
            live_match_result.second_blitz().player2().korean_name(),
            live_match_result.second_blitz().player1_wins(),
            live_match_result.second_blitz().player2_wins(),
            live_match_result.second_blitz_win_probability(),
            100.0 - live_match_result.second_blitz_win_probability(),
            wpa_result.second_blitz_player1_score(),
            wpa_result.second_blitz_player2_score(),
            if wpa_result.second_blitz_player1_wpa() * 100.0 >= 0.0 {
                format!("+{:5.2}", wpa_result.second_blitz_player1_wpa() * 100.0)
            } else {
                format!("-{:5.2}", wpa_result.second_blitz_player1_wpa().abs() * 100.0)
            },
            if wpa_result.second_blitz_player2_wpa() * 100.0 >= 0.0 {
                format!("+{:5.2}", wpa_result.second_blitz_player2_wpa() * 100.0)
            } else {
                format!("-{:5.2}", wpa_result.second_blitz_player2_wpa().abs() * 100.0)
            },
            live_match_result.third_blitz().player1().korean_name(),
            live_match_result.third_blitz().player2().korean_name(),
            live_match_result.third_blitz().player1_wins(),
            live_match_result.third_blitz().player2_wins(),
            live_match_result.third_blitz_win_probability(),
            100.0 - live_match_result.third_blitz_win_probability(),
            wpa_result.third_blitz_player1_score(),
            wpa_result.third_blitz_player2_score(),
            if wpa_result.third_blitz_player1_wpa() * 100.0 >= 0.0 {
                format!("+{:5.2}", wpa_result.third_blitz_player1_wpa() * 100.0)
            } else {
                format!("-{:5.2}", wpa_result.third_blitz_player1_wpa().abs() * 100.0)
            },
            if wpa_result.third_blitz_player2_wpa() * 100.0 >= 0.0 {
                format!("+{:5.2}", wpa_result.third_blitz_player2_wpa() * 100.0)
            } else {
                format!("-{:5.2}", wpa_result.third_blitz_player2_wpa().abs() * 100.0)
            },
            live_match_result.forth_blitz().player1().korean_name(),
            live_match_result.forth_blitz().player2().korean_name(),
            live_match_result.forth_blitz().player1_wins(),
            live_match_result.forth_blitz().player2_wins(),
            live_match_result.forth_blitz_win_probability(),
            100.0 - live_match_result.forth_blitz_win_probability(),
            wpa_result.forth_blitz_player1_score(),
            wpa_result.forth_blitz_player2_score(),
            if wpa_result.forth_blitz_player1_wpa() * 100.0 >= 0.0 {
                format!("+{:5.2}", wpa_result.forth_blitz_player1_wpa() * 100.0)
            } else {
                format!("-{:5.2}", wpa_result.forth_blitz_player1_wpa().abs() * 100.0)
            },
            if wpa_result.forth_blitz_player2_wpa() * 100.0 >= 0.0 {
                format!("+{:5.2}", wpa_result.forth_blitz_player2_wpa() * 100.0)
            } else {
                format!("-{:5.2}", wpa_result.forth_blitz_player2_wpa().abs() * 100.0)
            },
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

pub fn get_recent_record(gisa1: &str, mut gisa1_rating: f64, rating_list: &HashMap<String, f64>, ranking_month: String) -> Result<f64, Box<dyn Error>> {
    let current_month = format!("-{:02}-", chrono::Utc::now().month());
    let current_ranking_month = format!("-{:02}-", ranking_month.parse::<u32>().unwrap());

    let re = Regex::new(r"choice\('[^']+', ?'(\d+)', ?'\d+'\)").unwrap();
    let choice_selector = Selector::parse("li[onclick]").unwrap();

    let url1 = format!("http://baduk.or.kr/common/search_pro.asp?keyword={}&R_name=player1&R_code=player1_code", gisa1);
    let body1 = reqwest::blocking::get(&url1)?.text()?;
    let document1 = Html::parse_document(&body1);
    let script_texts1 = document1.select(&choice_selector).map(|script| script.value().attr("onclick").unwrap_or_default().to_string()).collect::<Vec<_>>();
    let script_text1 = script_texts1.first().unwrap(); // 첫번째만 사용하도록 변경
    let gisa_code = re.captures(script_text1).and_then(|cap| cap.get(1).map(|match_| match_.as_str().parse::<i32>().ok())).flatten().unwrap_or_default();

    let mut matches_to_process = Vec::new();

    for page_no in 1..=3 {
        let url2 = format!("http://baduk.or.kr/record/diary_in.asp?foreignKey=&pageNo={}&keyWord={}&etcKey=&etc2=1", page_no, gisa_code);
        let body2 = reqwest::blocking::get(&url2)?.text()?;
        let document2 = Html::parse_document(&body2);

        let match_selector = Selector::parse("tr").unwrap();
        let date_selector = Selector::parse("td.no").unwrap();

        let matches: Vec<_> = document2.select(&match_selector).collect();

        for selected_match in matches.iter() {
            if let Some(date_element) = selected_match.select(&date_selector).next() {
                let date_text = date_element.text().collect::<String>();
                if date_text.contains(&current_month) || date_text.contains(&current_ranking_month) {
                    let player_names = selected_match.select(&Selector::parse("td").unwrap())
                        .enumerate()
                        .filter_map(|(index, element)| {
                            if index == 2 || index == 3 {
                                Some(element.text().collect::<String>())
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<_>>();
                    matches_to_process.push(player_names);
                }
            }
        }
    }

    for players in matches_to_process.iter().rev() {
        let winner_text = players.first().unwrap().clone();
        
        let gisa2 = if !winner_text.contains(gisa1) { &players[0] } else { &players[1] };
        if let Some(gisa2_rating) = rating_list.get(gisa2) {
            let is_win = if winner_text.contains(gisa1) { 1.0 } else { 0.0 };
            let win_probability = calculate_win_probability(gisa1_rating, baeteil_to_goratings(*gisa2_rating));
            gisa1_rating += 1.5 * (is_win - win_probability);
        }
    }

    Ok(gisa1_rating)
}

pub fn create_excel_from_team(team_relativities_matrix: Vec<Vec<TeamRelativity>>) -> Result<(), Box<dyn std::error::Error>> {
    let workbook = Workbook::new("team_relativities.xlsx")?;
    let mut worksheet = workbook.add_worksheet(Some("팀 승률"))?;

    let mut team1_names = HashSet::new();
    let mut team2_names = HashSet::new();

    for team_relativity_row in &team_relativities_matrix {
        for team_relativity in team_relativity_row {
            team1_names.insert(team_relativity.team1().team_name().clone());
            team2_names.insert(team_relativity.team2().team_name().clone());
        }
    }

    let team1_names: Vec<_> = team1_names.into_iter().collect();
    let team2_names: Vec<_> = team2_names.into_iter().collect();

    for (index, team_name) in team1_names.iter().enumerate() {
        worksheet.write_string(0, (index + 1) as u16, team_name, None)?;
    }

    for (index, team_name) in team2_names.iter().enumerate() {
        worksheet.write_string((index + 1) as u32, 0, team_name, None)?;
    }

    for team_relativity_row in &team_relativities_matrix {
        for team_relativity in team_relativity_row {
            let team1_index = team1_names.iter().position(|name| name == team_relativity.team1().team_name()).unwrap();
            let team2_index = team2_names.iter().position(|name| name == team_relativity.team2().team_name()).unwrap();

            worksheet.write_number((team2_index + 1) as u32, (team1_index + 1) as u16, team_relativity.win_probability(), None)?;
        }
    }

    workbook.close()?;

    Ok(())
}

fn get_total_win_probability(match_result: MatchResult, player_relativities: &Vec<PlayerRelativity>) -> (f64, f64, f64) {
    let win_probabilities = [
        match_result.first_rapid_win_probability(),
        match_result.second_blitz_win_probability(),
        match_result.third_blitz_win_probability(),
        match_result.forth_blitz_win_probability(),
    ];

    let mapped_tiebreaker_win_probability: Vec<TiebreakerRelativity> = player_relativities.iter()
        .map(|relativity| {
            let player1_position = [
                match_result.first_rapid().player1().korean_name(),
                match_result.second_blitz().player1().korean_name(),
                match_result.third_blitz().player1().korean_name(),
                match_result.forth_blitz().player1().korean_name(),
            ].iter().position(|&name| name == relativity.player1().korean_name());
            let player2_position = [
                match_result.first_rapid().player2().korean_name(),
                match_result.second_blitz().player2().korean_name(),
                match_result.third_blitz().player2().korean_name(),
                match_result.forth_blitz().player2().korean_name(),
            ].iter().position(|&name| name == relativity.player2().korean_name());

            let player1_penalty = if let Some(pos) = player1_position {
                match pos {
                    0 => (1.0 / 1.04) * (1.0 / (1.0 + (0.04 * (1.0 - match_result.first_rapid_win_probability() / 100.0)))),
                    1 => (1.0 / 1.02) * (1.0 / (1.0 + (0.02 * (1.0 - match_result.second_blitz_win_probability() / 100.0)))),
                    2 => (1.0 / 1.08) * (1.0 / (1.0 + (0.08 * (1.0 - match_result.third_blitz_win_probability() / 100.0)))),
                    3 => (1.0 / 1.08) * (1.0 / (1.0 + (0.08 * (1.0 - match_result.forth_blitz_win_probability() / 100.0)))),
                    _ => 1.0,
                }
            } else {
                1.0
            };
            let player2_penalty = if let Some(pos) = player2_position {
                match pos {
                    0 => 1.04 * (1.0 + (0.04 * (1.0 - match_result.first_rapid_win_probability() / 100.0))),
                    1 => 1.02 * (1.0 + (0.02 * (1.0 - match_result.second_blitz_win_probability() / 100.0))),
                    2 => 1.08 * (1.0 + (0.08 * (1.0 - match_result.third_blitz_win_probability() / 100.0))),
                    3 => 1.08 * (1.0 + (0.08 * (1.0 - match_result.forth_blitz_win_probability() / 100.0))),
                    _ => 1.0,
                }
            } else {
                1.0
            };
            TiebreakerRelativity::new(
                relativity.player1().clone(), 
                relativity.player2().clone(), 
                relativity.fifth_bullet_win_probability() * player1_penalty * player2_penalty
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

    let tiebreaker_win_probability = (team1_tiebreaker_details.map_or(0.0, |details| details.win_probability()) + team2_tiebreaker_details.map_or(0.0, |details| details.win_probability())) / 2.0;

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

    (
        total_win_probability,
        4.0 * all_win_probability + 3.0 * three_win_one_lose_probability + 2.0 * two_win_two_lose_probability + 1.0 * one_win_three_lose_probability,
        1.0 * three_win_one_lose_probability + 2.0 * two_win_two_lose_probability + 3.0 * one_win_three_lose_probability + 4.0 * all_lose_probability
    )
}

fn redistribute_scores(a: f64, b: f64, c: f64, d: f64) -> (f64, f64, f64, f64) {
    let scores = vec![a, b, c, d];
    let total_score: f64 = scores.iter().sum();

    let adjusted_scores: Vec<f64> = scores.iter().map(|&score| if score < 0.0 { 0.0 } else { score }).collect();
    let new_total_score: f64 = adjusted_scores.iter().sum();
    let redistributed_scores: Vec<f64> = adjusted_scores.iter().map(|&score| if score > 0.0 { score / new_total_score * total_score } else { 0.0 }).collect();

    (redistributed_scores[0], redistributed_scores[1], redistributed_scores[2], redistributed_scores[3])
}

pub fn get_color_weight(gisa1: &str) -> Result<(f64, f64), Box<dyn Error>> {
    let current_date = chrono::Utc::now();
    let three_months_ago = current_date - chrono::Duration::try_days(90).unwrap();
    let three_months_ago_year = three_months_ago.year().to_string();
    let three_months_ago_month = three_months_ago.month().to_string();

    let (rating_list, _) = fetch_player_ratings_on_baeteil(&three_months_ago_year, &three_months_ago_month)?;

    let mut white_rating = baeteil_to_goratings(*rating_list.get(gisa1).unwrap_or(&0.0));
    let mut black_rating = baeteil_to_goratings(*rating_list.get(gisa1).unwrap_or(&0.0));

    let re = Regex::new(r"choice\('[^']+', ?'(\d+)', ?'\d+'\)").unwrap();
    let choice_selector = Selector::parse("li[onclick]").unwrap();

    let url1 = format!("http://baduk.or.kr/common/search_pro.asp?keyword={}&R_name=player1&R_code=player1_code", gisa1);
    let body1 = reqwest::blocking::get(&url1)?.text()?;
    let document1 = Html::parse_document(&body1);
    let script_texts1 = document1.select(&choice_selector).map(|script| script.value().attr("onclick").unwrap_or_default().to_string()).collect::<Vec<_>>();
    let script_text1 = script_texts1.first().unwrap(); // 첫번째만 사용하도록 변경
    let gisa_code = re.captures(script_text1).and_then(|cap| cap.get(1).map(|match_| match_.as_str().parse::<i32>().ok())).flatten().unwrap_or_default();

    let mut matches_to_process = Vec::new();

    let mut page_no = 1;
    loop {
        let url3 = format!("http://baduk.or.kr/record/diary_in.asp?foreignKey=&pageNo={}&keyWord={}&etcKey=&etc2=1", page_no, gisa_code);
        let body3 = reqwest::blocking::get(&url3)?.text()?;
        let document3 = Html::parse_document(&body3);

        let match_selector = Selector::parse("tbody>tr").unwrap();
        let date_selector = Selector::parse("td.no").unwrap();

        let matches: Vec<_> = document3.select(&match_selector).collect();
        if matches.is_empty() {
            break;
        }
        let mut break_loop = false;

        for selected_match in matches.iter() {
            if let Some(date_element) = selected_match.select(&date_selector).next() {

                let date_text = date_element.text().collect::<String>();

                if let Ok(date) = NaiveDate::parse_from_str(&date_text, "%Y-%m-%d") {
                    let three_months_ago_date = NaiveDate::from_ymd_opt(three_months_ago.year(), three_months_ago.month(), 1).unwrap();

                    if date > three_months_ago_date {
                        let player_names = selected_match.select(&Selector::parse("td").unwrap())
                            .enumerate()
                            .filter_map(|(index, element)| {
                                if index == 2 || index == 3 || index == 4 {
                                    Some(element.text().collect::<String>())
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<_>>();
                        matches_to_process.push(player_names);
                    } else {
                        break_loop = true;
                        break;
                    }
                }
            }
        }

        if break_loop {
            break;
        }

        page_no += 1;
    }

    for players in matches_to_process.iter().rev() {
        let winner_text = players.first().unwrap().clone();
        
        let gisa2 = if !winner_text.contains(gisa1) { &players[0] } else { &players[1] };
        if let Some(gisa2_rating) = rating_list.get(gisa2) {
            let is_win = if winner_text.contains(gisa1) { 1.0 } else { 0.0 };
            if (players[2].contains("백") && is_win == 1.0) || (players[2].contains("흑") && is_win == 0.0) {
                let win_probability = calculate_win_probability(white_rating, baeteil_to_goratings(*gisa2_rating));
                white_rating += 10.0 * (is_win - win_probability);
            } else if (players[2].contains("흑") && is_win == 1.0) || (players[2].contains("백") && is_win == 0.0) {
                let win_probability = calculate_win_probability(black_rating, baeteil_to_goratings(*gisa2_rating));
                black_rating += 10.0 * (is_win - win_probability);
            }
        }
    }

    let average_rating = (white_rating + black_rating) / 2.0;
    let white_weight = white_rating - average_rating;
    let black_weight = black_rating - average_rating;

    Ok((white_weight, black_weight))
}
