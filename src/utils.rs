use chrono::Datelike;
use itertools::Itertools;
use reqwest;
use scraper::{Html, Selector};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use xlsxwriter::format::FormatColor;
use xlsxwriter::Format;
use xlsxwriter::prelude::Workbook;
use crate::models::{Team, PlayerRelativity, Lineup, MatchResult};

pub fn calculate_win_probability(player1_elo: f64, player2_elo: f64) -> f64 {
    let elo_diff = player2_elo - player1_elo;
    let probability = 1.0 / (1.0 + 10.0_f64.powf(elo_diff / 400.0));
    probability
}

pub fn calculate_win_probability_with_relative_record(player1_elo: f64, player2_elo: f64, player1_wins: u32, player2_wins: u32) -> f64 {
    let base_probability = calculate_win_probability(player1_elo, player2_elo);
    let total_games = player1_wins + player2_wins;
    let win_rate_difference = if total_games > 0 {
        (player1_wins as f64 / total_games as f64) - (player2_wins as f64 / total_games as f64)
    } else {
        0.0
    };
        let adjusted_probability = base_probability + (win_rate_difference * (0.01 * total_games.min(5) as f64));
    if adjusted_probability > 1.0 {
        1.0
    } else if adjusted_probability < 0.0 {
        0.0
    } else {
        adjusted_probability
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
                    ratings.insert(name, rating / 1.3);
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

    for year in (current_year - 1)..=current_year {
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
                player.set_elo_rating(rating);
            } else if let Some(&rating) = player_ratings_on_goratings.get(player.english_name()) {
                player.set_elo_rating(rating + goratings_to_baeteil);
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

    let tiebreaker_win_probability = player_relativities.iter()
        .map(|relativity| {
            let player1_position = team1_players.iter().position(|p| p.korean_name() == relativity.player1().korean_name());
            let player2_position = team2_players.iter().position(|p| p.korean_name() == relativity.player2().korean_name());
            let player1_penalty = if let Some(pos) = player1_position {
                match pos {
                    0 => 0.95,
                    1 => 0.98,
                    _ => 0.90,
                }
            } else {
                1.0 // 순서에 없는 선수는 패널티 없음
            };
            let player2_penalty = if let Some(pos) = player2_position {
                match pos {
                    0 => 1.0 / 0.95,
                    1 => 1.0 / 0.98,
                    _ => 1.0 / 0.90,
                }
            } else {
                1.0 // 순서에 없는 선수는 패널티 없음
            };
            (relativity.player1().korean_name().clone(), relativity.bullet_win_probability() * player1_penalty * player2_penalty)
        })
        .fold(HashMap::new(), |mut acc: HashMap<String, Vec<f64>>, (name, prob)| {
            acc.entry(name).or_insert_with(Vec::new).push(prob);
            acc
        })
        .values()
        .map(|probs| probs.iter().cloned().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0))
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap_or(0.0);

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
        tiebreaker_win_probability,
        total_win_probability * 100.0,
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
    let mut worksheet_win = workbook.add_worksheet(Some("팀-에결없이 승리"))?;
    let mut worksheet_perfect_win = workbook.add_worksheet(Some("팀-완봉승"))?;
    let mut worksheet_tiebreak = workbook.add_worksheet(Some("팀-에결진출"))?;

    for (row_index, row) in match_results_matrix.iter().enumerate() {
        if row_index == 0 { // 첫 번째 행에 상대 팀 라인업 이름 쓰기
            for (col_index, match_result) in row.iter().enumerate().take(24) {
                let lineup_names = format!("1국:{}, 2국:{}, 3국:{}, 4국:{}", 
                    match_result.second_blitz().player2().korean_name(), 
                    match_result.third_blitz().player2().korean_name(), 
                    match_result.forth_blitz().player2().korean_name(), 
                    match_result.first_rapid().player2().korean_name());
                worksheet_total_win.write_string(0, col_index as u16 + 1, &lineup_names, None)?;
                worksheet_win.write_string(0, col_index as u16 + 1, &lineup_names, None)?;
                worksheet_perfect_win.write_string(0, col_index as u16 + 1, &lineup_names, None)?;
                worksheet_tiebreak.write_string(0, col_index as u16 + 1, &lineup_names, None)?;
            }
        }
        // 첫 번째 열에 자신의 팀 라인업 이름 쓰기
        let lineup_names = format!("1국:{}, 2국:{}, 3국:{}, 4국:{}", 
            row[0].second_blitz().player1().korean_name(), 
            row[0].third_blitz().player1().korean_name(), 
            row[0].forth_blitz().player1().korean_name(), 
            row[0].first_rapid().player1().korean_name());
        worksheet_total_win.write_string(row_index as u32 + 1, 0, &lineup_names, None)?;
        worksheet_win.write_string(row_index as u32 + 1, 0, &lineup_names, None)?;
        worksheet_perfect_win.write_string(row_index as u32 + 1, 0, &lineup_names, None)?;
        worksheet_tiebreak.write_string(row_index as u32 + 1, 0, &lineup_names, None)?;

        for (col_index, match_result) in row.iter().enumerate().take(36) {
            let total_win_format = create_custom_format(match_result.total_win_probability(), 25.0)?;
            let win_format = create_custom_format(match_result.perfect_win_probability() + match_result.win_probability(), 25.0)?;
            let perfect_win_format = create_custom_format(match_result.perfect_win_probability(), 25.0)?;
            let tiebreaker_format = create_custom_format(match_result.tie_probability(), 25.0)?;

            worksheet_total_win.write_number(row_index as u32 + 1, col_index as u16 + 1, match_result.total_win_probability() / 100.0, Some(&total_win_format))?;
            worksheet_win.write_number(row_index as u32 + 1, col_index as u16 + 1, (match_result.perfect_win_probability() + match_result.win_probability()) / 100.0, Some(&win_format))?;
            worksheet_perfect_win.write_number(row_index as u32 + 1, col_index as u16 + 1, match_result.perfect_win_probability() / 100.0, Some(&perfect_win_format))?;
            worksheet_tiebreak.write_number(row_index as u32 + 1, col_index as u16 + 1, match_result.tie_probability() / 100.0, Some(&tiebreaker_format))?;
        }
    }

    workbook.close()?;

    Ok(())
}

pub fn create_custom_format(win_probability: f64, maximum: f64) -> Result<Format, Box<dyn std::error::Error>> {
    let mut format = Format::new(); // 새로운 포맷 인스턴스 생성

    // 승리확률에 따라 색상을 점진적으로 변경합니다.
    let custom_color = if win_probability >= (100.0 - maximum) {
        FormatColor::Blue
    } else if win_probability <= maximum {
        FormatColor::Red
    } else {
        // 50%에 가까울수록 하얀색, 85% 또는 15%에 가까울수록 각각 파란색 또는 빨간색으로 점진적으로 변화
        let (red, blue) = if win_probability > 50.0 {
            // 50%에서 85% 사이일 때 파란색으로 점진적으로 변화
            let gradient = (win_probability - 50.0) / (50.0 - maximum);
            (255.0 * (1.0 - gradient), 255.0)
        } else {
            // 15%에서 50% 사이일 때 빨간색으로 점진적으로 변화
            let gradient = (50.0 - win_probability) / (50.0 - maximum);
            (255.0, 255.0 * (1.0 - gradient))
        };

        // 50%에 가까울수록 하얀색이 되도록 초록색 성분도 조절
        let green = if win_probability > 50.0 {
            255.0 * (1.0 - (win_probability - 50.0) / (50.0 - maximum))
        } else {
            255.0 * (1.0 - (50.0 - win_probability) / (50.0 - maximum))
        };

        FormatColor::Custom((red as u32) << 16 | (green as u32) << 8 | blue as u32)
    };

    format.set_num_format("0.00%").set_bg_color(custom_color); // 포맷에 숫자 형식과 배경색 설정

    Ok(format) // 설정된 포맷 반환
}
