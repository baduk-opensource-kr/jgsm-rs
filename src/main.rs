use chrono::Datelike;
use itertools::Itertools;
use reqwest;
use scraper::{Html, Selector};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, Write};
use xlsxwriter::format::FormatColor;
use xlsxwriter::prelude::Workbook;
use xlsxwriter::Format;

#[derive(Clone, PartialEq)]
struct Player {
    korean_name: String,
    english_name: String,
    elo_rating: f64,
    condition_weight: f64,
    rapid_weight: f64,
    blitz_weight: f64,
    bullet_weight: f64,
}

struct Team {
    team_name: String,
    players: Vec<Player>,
}

#[derive(Clone)]
struct PlayerRelativity {
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

#[derive(Clone)]
struct Lineup {
    first_rapid: Player,
    second_blitz: Player,
    third_blitz: Player,
    forth_blitz: Player,
}

#[derive(Clone)]
struct MatchResult {
    first_rapid: PlayerRelativity,
    second_blitz: PlayerRelativity,
    third_blitz: PlayerRelativity,
    forth_blitz: PlayerRelativity,
    first_rapid_win_probability: f64,
    second_blitz_win_probability: f64,
    third_blitz_win_probability: f64,
    forth_blitz_win_probability: f64,
    perfect_win_probability: f64,
    win_probability: f64,
    tie_probability: f64,
    lose_probability: f64,
    perfect_defeat_probability:  f64,
    tiebreaker_win_probability: f64,
    total_win_probability: f64,
}

fn main() {
    let mut teams: Vec<Team> = Vec::new();

    teams.push(Team {
        team_name: "한국물가정보".to_string(),
        players: vec![
            Player { korean_name: "강동윤".to_string(), english_name: "Kang Dongyun".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "한승주".to_string(), english_name: "Han Seungjoo".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "박민규".to_string(), english_name: "Park Minkyu".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "최재영".to_string(), english_name: "Choi Jaeyoung".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "이춘규".to_string(), english_name: "Lee Chungyu".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "당이페이".to_string(), english_name: "Dang Yifei".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
        ],
    });
    teams.push(Team {
        team_name: "수려한합천".to_string(),
        players: vec![
            Player { korean_name: "원성진".to_string(), english_name: "Weon Seongjin".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "한우진".to_string(), english_name: "Han Woojin".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "송지훈".to_string(), english_name: "Song Jihoon".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "한태희".to_string(), english_name: "Han Taehee".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "윤성식".to_string(), english_name: "Yun Seongsik".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "김승진".to_string(), english_name: "Kim Seungjin".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
        ],
    });
    teams.push(Team {
        team_name: "마한의 심장 영암".to_string(),
        players: vec![
            Player { korean_name: "안성준".to_string(), english_name: "An Sungjoon".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "설현준".to_string(), english_name: "Seol Hyunjun".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "최철한".to_string(), english_name: "Choi Cheolhan".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "박종훈".to_string(), english_name: "Park Jonghoon".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "엄동건".to_string(), english_name: "Eom Donggeon".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "쉬하오훙".to_string(), english_name: "Xu Haohong".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
        ],
    });
    teams.push(Team {
        team_name: "정관장천녹".to_string(),
        players: vec![
            Player { korean_name: "변상일".to_string(), english_name: "Byun Sangil".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "홍성지".to_string(), english_name: "Hong Seongji".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "김정현(大)".to_string(), english_name: "Kim Junghyun".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "한상훈".to_string(), english_name: "Han Sanghoon".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "김승구".to_string(), english_name: "Kim Seunggu".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "박상진".to_string(), english_name: "Park Sangjin".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
        ],
    });
    teams.push(Team {
        team_name: "울산 고려아연".to_string(),
        players: vec![
            Player { korean_name: "신민준".to_string(), english_name: "Shin Minjun".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "이창석".to_string(), english_name: "Lee Changseok".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "문민종".to_string(), english_name: "Moon Minjong".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "한상조".to_string(), english_name: "Han Sangcho".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "김채영".to_string(), english_name: "Kim Chaeyoung".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "랴오위안허".to_string(), english_name: "Liao Yuanhe".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
        ],
    });
    teams.push(Team {
        team_name: "바둑메카 의정부".to_string(),
        players: vec![
            Player { korean_name: "김명훈".to_string(), english_name: "Kim Myounghoon".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "박건호".to_string(), english_name: "Park Geunho".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "이원영".to_string(), english_name: "Lee Wonyoung".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "허영호".to_string(), english_name: "Heo Yongho".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "박재근".to_string(), english_name: "Park Jaekeun".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "양카이원".to_string(), english_name: "Yang Kaiwen".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
        ],
    });
    teams.push(Team {
        team_name: "Kixx".to_string(),
        players: vec![
            Player { korean_name: "신진서".to_string(), english_name: "Shin Jinseo".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "박진솔".to_string(), english_name: "Park Jinsol".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "김승재".to_string(), english_name: "Kim Seungjae".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "백현우".to_string(), english_name: "Baek Hyeonwoo".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "김창훈".to_string(), english_name: "Kim Changhoon".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
        ],
    });
    teams.push(Team {
        team_name: "원익".to_string(),
        players: vec![
            Player { korean_name: "박정환".to_string(), english_name: "Park Junghwan".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "이지현(남)".to_string(), english_name: "Lee Jihyun (m)".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "박영훈".to_string(), english_name: "Park Yeonghun".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "김진휘".to_string(), english_name: "Kim Jinhwi".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "금지우".to_string(), english_name: "Geum Jiwoo".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
            Player { korean_name: "구쯔하오".to_string(), english_name: "Gu Zihao".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 },
        ],
    });

    let mut selected_teams: Vec<Team> = Vec::new();
    for _ in 0..2 {
        loop {
            println!("팀{}을 선택하세요: ", selected_teams.len() + 1);

            for (index, team) in teams.iter().enumerate() {
                println!("{}. {}", index + 1, team.team_name);
            }

            io::stdout().flush().unwrap(); // 표준 출력 버퍼를 강제로 비워줍니다.
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_err() {
                println!("입력을 읽는 데 실패했습니다. 다시 시도해주세요.");
                continue;
            }
            let selected_index: usize = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("유효한 숫자를 입력해주세요.");
                    continue;
                }
            };

            if selected_index > 0 && selected_index <= teams.len() {
                selected_teams.push(teams.remove(selected_index - 1));
                break;
            } else {
                println!("유효한 팀 번호를 입력해주세요.");
            }
        }
    }

    println!("ELO 레이팅을 업데이트 중...");
    if let Err(e) = update_team_elo_ratings(&mut selected_teams) {
        println!("ELO 레이팅을 업데이트하는 동안 오류가 발생했습니다: {}", e);
        return;
    } else {
        println!("ELO 레이팅이 성공적으로 업데이트되었습니다.");
    }

    for selected_team in &mut selected_teams {
        loop {
            println!("\n{} 팀의 스쿼드:", selected_team.team_name);
            for (index, player) in selected_team.players.iter().enumerate() {
                println!("{}. {} (elo: {:.2})", index + 1, player.korean_name, player.elo_rating);
            }
            let mut input = String::new();
            print!("\n제외할 기사를 선택하세요 (완료시 엔터): ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
            let input = input.trim();
            if input.is_empty() {
                break;
            }

            let selected_index: usize = match input.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("유효한 숫자를 입력해주세요.");
                    continue;
                }
            };

            if selected_index > 0 && selected_index <= selected_team.players.len() {
                let removed_player = selected_team.players.remove(selected_index - 1);
                println!("{} 기사가 목록에서 제외되었습니다.", removed_player.korean_name);
            } else {
                println!("유효한 기사 번호를 입력해주세요.");
            }
        }

        println!("\n{} 팀의 기사에 대한 컨디션 가중치를 입력하세요.", selected_team.team_name);
        loop {
            for (index, player) in selected_team.players.iter().enumerate() {
                println!("{}. {} (elo: {:.2})\n    컨디션 가중치: {:.2}", index + 1, player.korean_name, player.elo_rating, player.condition_weight);
            }
            println!("컨디션 가중치를 입력할 기사를 선택하세요 (완료시 엔터): ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
            if input.trim().is_empty() {
                break;
            }
            let selected_index: usize = input.trim().parse().expect("정수를 입력해주세요.");

            if selected_index > 0 && selected_index <= selected_team.players.len() {
                let player = &mut selected_team.players[selected_index - 1];

                input.clear();
                println!("\n{} 기사의 컨디션 가중치를 입력하세요 (변경하지 않으려면 엔터): ", player.korean_name);
                io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
                if !input.trim().is_empty() {
                    player.condition_weight = input.trim().parse().expect("정수를 입력해주세요.");
                }
            } else {
                println!("유효한 기사 번호를 입력해주세요.");
            }
        }

        println!("\n{} 팀의 기사에 대한 게임속도 가중치를 입력하세요.", selected_team.team_name);
        loop {
            for (index, player) in selected_team.players.iter().enumerate() {
                println!("{}. {} (elo: {:.2})\n    장고(Rapid) 가중치: {:.2}\n    속기(Blitz) 가중치: {:.2}\n    초속기(Bullet) 가중치: {:.2}", index + 1, player.korean_name, player.elo_rating, player.rapid_weight, player.blitz_weight, player.bullet_weight);
            }
            println!("게임속도 가중치를 입력할 기사를 선택하세요 (완료시 엔터): ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
            if input.trim().is_empty() {
                break;
            }
            let selected_index: usize = input.trim().parse().expect("정수를 입력해주세요.");

            if selected_index > 0 && selected_index <= selected_team.players.len() {
                let player = &mut selected_team.players[selected_index - 1];

                input.clear();
                println!("\n{} 기사의 장고(Rapid) 가중치를 입력하세요 (변경하지 않으려면 엔터): ", player.korean_name);
                io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
                if !input.trim().is_empty() {
                    player.rapid_weight = input.trim().parse().expect("정수를 입력해주세요.");
                }
                input.clear();

                println!("{} 기사의 속기(Blitz) 가중치를 입력하세요 (변경하지 않으려면 엔터): ", player.korean_name);
                io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
                if !input.trim().is_empty() {
                    player.blitz_weight = input.trim().parse().expect("정수를 입력해주세요.");
                }
                input.clear();

                println!("{} 기사의 초속기(Bullet) 가중치를 입력하세요 (변경하지 않으려면 엔터): ", player.korean_name);
                io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
                if !input.trim().is_empty() {
                    player.bullet_weight = input.trim().parse().expect("정수를 입력해주세요.");
                }
            } else {
                println!("유효한 기사 번호를 입력해주세요.");
            }
        }
    }

    println!("\n업데이트된 스쿼드:");
    for team in selected_teams.iter() {
        println!("\n{}:", team.team_name);
        for player in &team.players {
            println!("{} (elo: {:.2}, 컨디션: {:.2}, 장고: {:.2}, 속기: {:.2}, 초속기: {:.2})", player.korean_name, player.elo_rating, player.elo_rating + player.condition_weight, player.elo_rating + player.rapid_weight, player.elo_rating + player.blitz_weight, player.elo_rating + player.bullet_weight);
        }
    }

    let mut team1_all_lineups: Vec<Lineup> = Vec::new();
    let mut team2_all_lineups: Vec<Lineup> = Vec::new();

    for team_index in 0..2 {
        let team_players = &selected_teams[team_index].players;
        let all_lineups = if team_index == 0 { &mut team1_all_lineups } else { &mut team2_all_lineups };

        for first_rapid in team_players {
            for second_blitz in team_players {
                if second_blitz == first_rapid { continue; }
                for third_blitz in team_players {
                    if third_blitz == first_rapid || third_blitz == second_blitz { continue; }
                    for forth_blitz in team_players {
                        if forth_blitz == first_rapid || forth_blitz == second_blitz || forth_blitz == third_blitz { continue; }
                        all_lineups.push(Lineup {
                            first_rapid: first_rapid.clone(),
                            second_blitz: second_blitz.clone(),
                            third_blitz: third_blitz.clone(),
                            forth_blitz: forth_blitz.clone(),
                        });
                    }
                }
            }
        }
    }

    println!("\n상대전적을 업데이트 중...");
    match generate_player_relativities(&selected_teams) {
        Ok(player_relativities) => {
            println!("\n라인업 메트릭스 생성 중...");
            let mut match_results_matrix: Vec<Vec<MatchResult>> = Vec::new();
            let mut team1_lineups_with_avg: Vec<(Lineup, f64)> = team1_all_lineups.iter().map(|lineup| {
                let avg_total_win_probability: f64 = team2_all_lineups.iter().map(|opponent_lineup| {
                    let match_result = calculate_match_result(lineup.clone(), opponent_lineup.clone(), player_relativities.clone());
                    match_result.total_win_probability
                }).sum::<f64>() / team2_all_lineups.len() as f64;
                (lineup.clone(), avg_total_win_probability)
            }).collect();

            let mut team2_lineups_with_avg: Vec<(Lineup, f64)> = team2_all_lineups.iter().map(|lineup| {
                let avg_total_win_probability: f64 = team1_all_lineups.iter().map(|opponent_lineup| {
                    let match_result = calculate_match_result(opponent_lineup.clone(), lineup.clone(), player_relativities.clone());
                    match_result.total_win_probability
                }).sum::<f64>() / team1_all_lineups.len() as f64;
                (lineup.clone(), avg_total_win_probability)
            }).collect();

            team1_lineups_with_avg.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            team2_lineups_with_avg.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            for (team1_lineup, _) in team1_lineups_with_avg {
                let mut row: Vec<MatchResult> = Vec::new();
                for (team2_lineup, _) in &team2_lineups_with_avg {
                    let match_result = calculate_match_result(team1_lineup.clone(), team2_lineup.clone(), player_relativities.clone());
                    row.push(match_result);
                }
                match_results_matrix.push(row);
            }

            loop {
                println!("\n선택할 옵션:");
                println!("1. {}의 스쿼드", selected_teams[0].team_name);
                println!("2. {}의 스쿼드", selected_teams[1].team_name);
                println!("3. 양팀의 라인업 메트릭스를 Excel로 출력\n");
                println!("4. 지정 라인업 승리확률\n");
                println!("5. {} 최고 평균승률 라인업", selected_teams[0].team_name);
                println!("6. {} 베스트24 라인업에 대한 {} 최고 평균승률 라인업", selected_teams[1].team_name, selected_teams[0].team_name);
                println!("7. {} 카운터픽 면역 라인업", selected_teams[0].team_name);
                println!("8. {} 예상라인업에 대한 {} 카운터픽\n", selected_teams[1].team_name, selected_teams[0].team_name);
                println!("9. 에이스 결정전(추가예정)");

                println!("exit. 종료");

                let mut option = String::new();
                io::stdin().read_line(&mut option).expect("입력을 읽는 데 실패했습니다.");
                let option = option.trim();

                match option {
                    "1" => {
                        println!("\n{} 팀의 스쿼드:", selected_teams[0].team_name);
                        for player in &selected_teams[0].players {
                            println!("{} (elo: {:.2}, 컨디션: {:.2}, 장고: {:.2}, 속기: {:.2}, 초속기: {:.2})", player.korean_name, player.elo_rating, player.elo_rating + player.condition_weight, player.elo_rating + player.rapid_weight, player.elo_rating + player.blitz_weight, player.elo_rating + player.bullet_weight);
                        }
                    },
                    "2" => {
                        println!("\n{} 팀의 스쿼드:", selected_teams[1].team_name);
                        for player in &selected_teams[1].players {
                            println!("{} (elo: {:.2}, 컨디션: {:.2}, 장고: {:.2}, 속기: {:.2}, 초속기: {:.2})", player.korean_name, player.elo_rating, player.elo_rating + player.condition_weight, player.elo_rating + player.rapid_weight, player.elo_rating + player.blitz_weight, player.elo_rating + player.bullet_weight);
                        }
                    },
                    "3" => {
                        match create_excel_from_relativities(player_relativities.clone(), match_results_matrix.clone()) {
                            Ok(_) => println!("Excel 파일이 성공적으로 생성되었습니다."),
                            Err(e) => println!("Excel 파일 생성 중 오류가 발생했습니다: {}", e),
                        }
                    },
                    "4" => {
                        let mut team1_combination: Vec<&Player> = Vec::new();
                        println!("\n{} 팀의 스쿼드:", selected_teams[0].team_name);
                        for (index, player) in selected_teams[0].players.iter().enumerate() {
                            println!("{}. {} (elo: {:.2}, 컨디션: {:.2}, 장고: {:.2}, 속기: {:.2}, 초속기: {:.2})", index + 1, player.korean_name, player.elo_rating, player.elo_rating + player.condition_weight, player.elo_rating + player.rapid_weight, player.elo_rating + player.blitz_weight, player.elo_rating + player.bullet_weight);
                        }
                        for i in 0..4 {
                            loop {
                                let mut input = String::new();
                                match i {
                                    0 => println!("\n{} 팀의 1국 장고(rapid) 기사 번호를 입력하세요:", selected_teams[0].team_name),
                                    1 => println!("\n{} 팀의 2국 속기(blitz) 기사 번호를 입력하세요:", selected_teams[0].team_name),
                                    2 => println!("\n{} 팀의 3국 속기(blitz) 기사 번호를 입력하세요:", selected_teams[0].team_name),
                                    3 => println!("\n{} 팀의 4국 속기(blitz) 기사 번호를 입력하세요:", selected_teams[0].team_name),
                                    _ => {}
                                }
                                io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
                                match input.trim().parse::<usize>() {
                                    Ok(num) if num > 0 && num <= selected_teams[0].players.len() => {
                                        team1_combination.push(&selected_teams[0].players[num - 1]);
                                        break;
                                    },
                                    _ => println!("잘못된 입력입니다. 다시 입력해주세요."),
                                }
                            }
                        }

                        let mut team2_combination: Vec<&Player> = Vec::new();
                        println!("\n{} 팀의 스쿼드:", selected_teams[1].team_name);
                        for (index, player) in selected_teams[1].players.iter().enumerate() {
                            println!("{}. {} (elo: {:.2}, 컨디션: {:.2}, 장고: {:.2}, 속기: {:.2}, 초속기: {:.2})", index + 1, player.korean_name, player.elo_rating, player.elo_rating + player.condition_weight, player.elo_rating + player.rapid_weight, player.elo_rating + player.blitz_weight, player.elo_rating + player.bullet_weight);
                        }
                        for i in 0..4 {
                            loop {
                                let mut input = String::new();
                                match i {
                                    0 => println!("\n{} 팀의 1국 장고(rapid) 기사 번호를 입력하세요:", selected_teams[1].team_name),
                                    1 => println!("\n{} 팀의 2국 속기(blitz) 기사 번호를 입력하세요:", selected_teams[1].team_name),
                                    2 => println!("\n{} 팀의 3국 속기(blitz) 기사 번호를 입력하세요:", selected_teams[1].team_name),
                                    3 => println!("\n{} 팀의 4국 속기(blitz) 기사 번호를 입력하세요:", selected_teams[1].team_name),
                                    _ => {}
                                }
                                io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
                                match input.trim().parse::<usize>() {
                                    Ok(num) if num > 0 && num <= selected_teams[1].players.len() => {
                                        team2_combination.push(&selected_teams[1].players[num - 1]);
                                        break;
                                    },
                                    _ => println!("잘못된 입력입니다. 다시 입력해주세요."),
                                }
                            }
                        }

                        let match_result = match_results_matrix.iter().flatten().find(|&result| {
                            result.first_rapid.player1.korean_name == team1_combination[0].korean_name && 
                            result.second_blitz.player1.korean_name == team1_combination[1].korean_name && 
                            result.third_blitz.player1.korean_name == team1_combination[2].korean_name && 
                            result.forth_blitz.player1.korean_name == team1_combination[3].korean_name && 
                            result.first_rapid.player2.korean_name == team2_combination[0].korean_name && 
                            result.second_blitz.player2.korean_name == team2_combination[1].korean_name && 
                            result.third_blitz.player2.korean_name == team2_combination[2].korean_name && 
                            result.forth_blitz.player2.korean_name == team2_combination[3].korean_name
                        }).expect("매치 결과를 찾을 수 없습니다.");

                        println!("========================");
                        println!("1국 장고(rapid): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", team1_combination[0].korean_name, team2_combination[0].korean_name, chrono::Utc::now().year() - 1, match_result.first_rapid.player1_wins, match_result.first_rapid.player2_wins, match_result.first_rapid_win_probability);
                        println!("2국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", team1_combination[1].korean_name, team2_combination[1].korean_name, chrono::Utc::now().year() - 1, match_result.second_blitz.player1_wins, match_result.second_blitz.player2_wins, match_result.second_blitz_win_probability);
                        println!("3국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", team1_combination[2].korean_name, team2_combination[2].korean_name, chrono::Utc::now().year() - 1, match_result.third_blitz.player1_wins, match_result.third_blitz.player2_wins, match_result.third_blitz_win_probability);
                        println!("4국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", team1_combination[3].korean_name, team2_combination[3].korean_name, chrono::Utc::now().year() - 1, match_result.forth_blitz.player1_wins, match_result.forth_blitz.player2_wins, match_result.forth_blitz_win_probability);
                        println!("\n4-0: {:.2}%", match_result.perfect_win_probability);
                        println!("3-1: {:.2}%", match_result.win_probability);
                        println!("2-2: {:.2}%", match_result.tie_probability);
                        println!("1-3: {:.2}%", match_result.lose_probability);
                        println!("0-4: {:.2}%", match_result.perfect_defeat_probability);
                        println!("\n총 승리확률: {:.2}%", match_result.total_win_probability);
                        println!("에이스결정전 예상 승리확률: {:.2}%", match_result.tiebreaker_win_probability);
                        println!("========================");

                        println!("\n계속하려면 엔터를 누르세요.");
                        let mut pause = String::new();
                        io::stdin().read_line(&mut pause).expect("입력을 읽는 데 실패했습니다.");
                    },
                    "5" => {
                        let mut avg_probabilities: Vec<(Lineup, f64, f64, f64, f64, f64, f64, f64, f64)> = Vec::new();
                        for lineup in &team1_all_lineups {
                            let mut total_win_prob = 0.0;
                            let mut win_prob = 0.0;
                            let mut perfect_win_prob = 0.0;
                            let mut tie_prob = 0.0;
                            let mut first_rapid_win_prob = 0.0;
                            let mut second_blitz_win_prob = 0.0;
                            let mut third_blitz_win_prob = 0.0;
                            let mut forth_blitz_win_prob = 0.0;
                            let mut count = 0.0;

                            for match_result in &match_results_matrix {
                                for result in match_result {
                                    if result.first_rapid.player1.korean_name == lineup.first_rapid.korean_name &&
                                       result.second_blitz.player1.korean_name == lineup.second_blitz.korean_name &&
                                       result.third_blitz.player1.korean_name == lineup.third_blitz.korean_name &&
                                       result.forth_blitz.player1.korean_name == lineup.forth_blitz.korean_name {
                                        total_win_prob += result.total_win_probability;
                                        win_prob += result.perfect_win_probability + result.win_probability;
                                        perfect_win_prob += result.perfect_win_probability;
                                        tie_prob += result.tie_probability;
                                        first_rapid_win_prob += result.first_rapid_win_probability;
                                        second_blitz_win_prob += result.second_blitz_win_probability;
                                        third_blitz_win_prob += result.third_blitz_win_probability;
                                        forth_blitz_win_prob += result.forth_blitz_win_probability;
                                        count += 1.0;
                                    }
                                }
                            }

                            if count > 0.0 {
                                avg_probabilities.push((
                                    lineup.clone(),
                                    total_win_prob / count,
                                    win_prob / count,
                                    perfect_win_prob / count,
                                    tie_prob / count,
                                    first_rapid_win_prob / count,
                                    second_blitz_win_prob / count,
                                    third_blitz_win_prob / count,
                                    forth_blitz_win_prob / count
                                ));
                            }
                        }


                        println!("========================");
                        avg_probabilities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_perfect_win_prob, avg_tie_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 총 승리확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid.korean_name, avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz.korean_name, avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz.korean_name, avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz.korean_name, avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_perfect_win_prob);
                            println!("평균 동점 확률: {:.2}%", avg_tie_prob);
                        } else {
                            println!("적합한 구성을 찾을 수 없습니다.");
                        }
                        println!("========================");
                        avg_probabilities.sort_by(|a, b| {
                            if (b.2 - a.2).abs() < 0.001 {
                                if (b.1 - a.1).abs() < 0.001 {
                                    std::cmp::Ordering::Equal
                                } else {
                                    b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
                                }
                            } else {
                                b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal)
                            }
                        });
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_perfect_win_prob, avg_tie_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 동점 없는 승리확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid.korean_name, avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz.korean_name, avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz.korean_name, avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz.korean_name, avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_perfect_win_prob);
                            println!("평균 동점 확률: {:.2}%", avg_tie_prob);
                        } else {
                            println!("적합한 구성을 찾을 수 없습니다.");
                        }
                        println!("========================");
                        avg_probabilities.sort_by(|a, b| {
                            if (b.3 - a.3).abs() < 0.001 {
                                if (b.1 - a.1).abs() < 0.001 {
                                    std::cmp::Ordering::Equal
                                } else {
                                    b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
                                }
                            } else {
                                b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal)
                            }
                        });
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_perfect_win_prob, avg_tie_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 완봉승확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid.korean_name, avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz.korean_name, avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz.korean_name, avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz.korean_name, avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_perfect_win_prob);
                            println!("평균 동점 확률: {:.2}%", avg_tie_prob);
                        } else {
                            println!("적합한 구성을 찾을 수 없습니다.");
                        }
                        println!("========================");
                        avg_probabilities.sort_by(|a, b| {
                            if (b.4 - a.4).abs() < 0.001 {
                                if (b.1 - a.1).abs() < 0.001 {
                                    std::cmp::Ordering::Equal
                                } else {
                                    b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
                                }
                            } else {
                                b.4.partial_cmp(&a.4).unwrap_or(std::cmp::Ordering::Equal)
                            }
                        });
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_perfect_win_prob, avg_tie_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 동점 확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid.korean_name, avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz.korean_name, avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz.korean_name, avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz.korean_name, avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_perfect_win_prob);
                            println!("평균 동점 확률: {:.2}%", avg_tie_prob);
                        } else {
                            println!("적합한 구성을 찾을 수 없습니다.");
                        }
                        println!("========================");

                        println!("\n계속하려면 엔터를 누르세요.");
                        let mut pause = String::new();
                        io::stdin().read_line(&mut pause).expect("입력을 읽는 데 실패했습니다.");
                    },
                    "6" => {
                        let mut avg_probabilities: Vec<(Lineup, f64, f64, f64, f64, f64, f64, f64, f64)> = Vec::new();
                        for lineup in &team1_all_lineups {
                            let mut total_win_prob = 0.0;
                            let mut win_prob = 0.0;
                            let mut perfect_win_prob = 0.0;
                            let mut tie_prob = 0.0;
                            let mut first_rapid_win_prob = 0.0;
                            let mut second_blitz_win_prob = 0.0;
                            let mut third_blitz_win_prob = 0.0;
                            let mut forth_blitz_win_prob = 0.0;
                            let mut count = 0.0;

                            if let Some(match_result) = match_results_matrix.iter().find(|match_result| 
                                match_result.get(0).map_or(false, |result| 
                                    result.first_rapid.player1.korean_name == lineup.first_rapid.korean_name &&
                                    result.second_blitz.player1.korean_name == lineup.second_blitz.korean_name &&
                                    result.third_blitz.player1.korean_name == lineup.third_blitz.korean_name &&
                                    result.forth_blitz.player1.korean_name == lineup.forth_blitz.korean_name
                                )
                            ) {
                                for result in match_result.iter().take(24) {
                                    total_win_prob += result.total_win_probability;
                                    win_prob += result.perfect_win_probability + result.win_probability;
                                    perfect_win_prob += result.perfect_win_probability;
                                    tie_prob += result.tie_probability;
                                    first_rapid_win_prob += result.first_rapid_win_probability;
                                    second_blitz_win_prob += result.second_blitz_win_probability;
                                    third_blitz_win_prob += result.third_blitz_win_probability;
                                    forth_blitz_win_prob += result.forth_blitz_win_probability;
                                    count += 1.0;
                                }
                            }

                            if count > 0.0 {
                                avg_probabilities.push((
                                    lineup.clone(),
                                    total_win_prob / count,
                                    win_prob / count,
                                    perfect_win_prob / count,
                                    tie_prob / count,
                                    first_rapid_win_prob / count,
                                    second_blitz_win_prob / count,
                                    third_blitz_win_prob / count,
                                    forth_blitz_win_prob / count
                                ));
                            }
                        }


                        println!("========================");
                        avg_probabilities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_perfect_win_prob, avg_tie_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 총 승리확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid.korean_name, avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz.korean_name, avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz.korean_name, avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz.korean_name, avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_perfect_win_prob);
                            println!("평균 동점 확률: {:.2}%", avg_tie_prob);
                        } else {
                            println!("적합한 구성을 찾을 수 없습니다.");
                        }
                        println!("========================");
                        avg_probabilities.sort_by(|a, b| {
                            if (b.2 - a.2).abs() < 0.001 {
                                if (b.1 - a.1).abs() < 0.001 {
                                    std::cmp::Ordering::Equal
                                } else {
                                    b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
                                }
                            } else {
                                b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal)
                            }
                        });
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_perfect_win_prob, avg_tie_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 동점 없는 승리확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid.korean_name, avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz.korean_name, avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz.korean_name, avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz.korean_name, avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_perfect_win_prob);
                            println!("평균 동점 확률: {:.2}%", avg_tie_prob);
                        } else {
                            println!("적합한 구성을 찾을 수 없습니다.");
                        }
                        println!("========================");
                        avg_probabilities.sort_by(|a, b| {
                            if (b.3 - a.3).abs() < 0.001 {
                                if (b.1 - a.1).abs() < 0.001 {
                                    std::cmp::Ordering::Equal
                                } else {
                                    b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
                                }
                            } else {
                                b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal)
                            }
                        });
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_perfect_win_prob, avg_tie_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 완봉승확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid.korean_name, avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz.korean_name, avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz.korean_name, avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz.korean_name, avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_perfect_win_prob);
                            println!("평균 동점 확률: {:.2}%", avg_tie_prob);
                        } else {
                            println!("적합한 구성을 찾을 수 없습니다.");
                        }
                        println!("========================");
                        avg_probabilities.sort_by(|a, b| {
                            if (b.4 - a.4).abs() < 0.001 {
                                if (b.1 - a.1).abs() < 0.001 {
                                    std::cmp::Ordering::Equal
                                } else {
                                    b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
                                }
                            } else {
                                b.4.partial_cmp(&a.4).unwrap_or(std::cmp::Ordering::Equal)
                            }
                        });
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_perfect_win_prob, avg_tie_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 동점 확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid.korean_name, avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz.korean_name, avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz.korean_name, avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz.korean_name, avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_perfect_win_prob);
                            println!("평균 동점 확률: {:.2}%", avg_tie_prob);
                        } else {
                            println!("적합한 구성을 찾을 수 없습니다.");
                        }
                        println!("========================");

                        println!("\n계속하려면 엔터를 누르세요.");
                        let mut pause = String::new();
                        io::stdin().read_line(&mut pause).expect("입력을 읽는 데 실패했습니다.");
                    },
                    "7" => {
                        let mut best_match_result: Option<&MatchResult> = None;
                        let mut highest_min_total_win_prob = 0.0;

                        for lineup in &team1_all_lineups {
                            let mut min_total_win_prob = std::f64::MAX;

                            for match_result in &match_results_matrix {
                                for result in match_result.iter() {
                                    if result.first_rapid.player1.korean_name == lineup.first_rapid.korean_name &&
                                       result.second_blitz.player1.korean_name == lineup.second_blitz.korean_name &&
                                       result.third_blitz.player1.korean_name == lineup.third_blitz.korean_name &&
                                       result.forth_blitz.player1.korean_name == lineup.forth_blitz.korean_name {
                                        if result.total_win_probability < min_total_win_prob {
                                            min_total_win_prob = result.total_win_probability;
                                        }
                                    }
                                }
                            }

                            if min_total_win_prob != std::f64::MAX && min_total_win_prob > highest_min_total_win_prob {
                                highest_min_total_win_prob = min_total_win_prob;
                                best_match_result = match_results_matrix.iter().flatten().find(|&r| 
                                    r.first_rapid.player1.korean_name == lineup.first_rapid.korean_name &&
                                    r.second_blitz.player1.korean_name == lineup.second_blitz.korean_name &&
                                    r.third_blitz.player1.korean_name == lineup.third_blitz.korean_name &&
                                    r.forth_blitz.player1.korean_name == lineup.forth_blitz.korean_name &&
                                    (r.total_win_probability - highest_min_total_win_prob).abs() < std::f64::EPSILON
                                );
                            }
                        }

                        println!("========================");
                        if let Some(best_result) = best_match_result {
                            println!("1국 장고(rapid): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", best_result.first_rapid.player1.korean_name, best_result.first_rapid.player2.korean_name, chrono::Utc::now().year() - 1, best_result.first_rapid.player1_wins, best_result.first_rapid.player2_wins, best_result.first_rapid_win_probability);
                            println!("2국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", best_result.second_blitz.player1.korean_name, best_result.second_blitz.player2.korean_name, chrono::Utc::now().year() - 1, best_result.second_blitz.player1_wins, best_result.second_blitz.player2_wins, best_result.second_blitz_win_probability);
                            println!("3국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", best_result.third_blitz.player1.korean_name, best_result.third_blitz.player2.korean_name, chrono::Utc::now().year() - 1, best_result.third_blitz.player1_wins, best_result.third_blitz.player2_wins, best_result.third_blitz_win_probability);
                            println!("4국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", best_result.forth_blitz.player1.korean_name, best_result.forth_blitz.player2.korean_name, chrono::Utc::now().year() - 1, best_result.forth_blitz.player1_wins, best_result.forth_blitz.player2_wins, best_result.forth_blitz_win_probability);
                            println!("\n4-0: {:.2}%", best_result.perfect_win_probability);
                            println!("3-1: {:.2}%", best_result.win_probability);
                            println!("2-2: {:.2}%", best_result.tie_probability);
                            println!("1-3: {:.2}%", best_result.lose_probability);
                            println!("0-4: {:.2}%", best_result.perfect_defeat_probability);
                            println!("\n총 승리확률: {:.2}%", best_result.total_win_probability);
                            println!("에이스결정전 예상 승리확률: {:.2}%", best_result.tiebreaker_win_probability);
                        } else {
                            println!("적합한 매치 결과를 찾을 수 없습니다.");
                        }
                        println!("========================");


                        println!("\n계속하려면 엔터를 누르세요.");
                        let mut pause = String::new();
                        io::stdin().read_line(&mut pause).expect("입력을 읽는 데 실패했습니다.");
                    },
                    "8" => {
                        let mut team2_combination: Vec<&Player> = Vec::new();
                        let unknown_player = Player { korean_name: "알 수 없음".to_string(), english_name: "unknown".to_string(), elo_rating: 0.0, condition_weight: 0.0, rapid_weight: 0.0, blitz_weight: 0.0, bullet_weight: 0.0 };
                        println!("\n{} 팀의 스쿼드:", selected_teams[1].team_name);
                        let mut last_index = 0;
                        for (index, player) in selected_teams[1].players.iter().enumerate() {
                            println!("{}. {} (elo: {:.2}, 컨디션: {:.2}, 장고: {:.2}, 속기: {:.2}, 초속기: {:.2})", index + 1, player.korean_name, player.elo_rating, player.elo_rating + player.condition_weight, player.elo_rating + player.rapid_weight, player.elo_rating + player.blitz_weight, player.elo_rating + player.bullet_weight);
                            last_index = index;
                        }
                        println!("{}. 알 수 없음", last_index + 2);
                        for i in 0..4 {
                            loop {
                                let mut input = String::new();
                                println!("\n{} 팀의 {}국 {} 기사 번호를 입력하세요:", selected_teams[1].team_name, i + 1, if i == 0 { "장고(rapid)" } else { "속기(blitz)" });
                                io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
                                match input.trim().parse::<usize>() {
                                    Ok(num) if num > 0 && num <= selected_teams[1].players.len() => {
                                        team2_combination.push(&selected_teams[1].players[num - 1]);
                                        break;
                                    },
                                    Ok(num) if num == selected_teams[1].players.len() + 1 => {
                                        team2_combination.push(&unknown_player);
                                        break;
                                    },
                                    _ => println!("잘못된 입력입니다. 다시 입력해주세요."),
                                }
                            }
                        }

                        let unknown_player_count = team2_combination.iter().filter(|&player| player.english_name == "unknown").count();
                        let combinations = [36, 9, 4, 2, 1][unknown_player_count.min(4)];

                        let mut avg_probabilities: Vec<(Lineup, f64, f64, f64, f64, f64, f64, f64, f64)> = Vec::new();
                        for lineup in &team1_all_lineups {
                            let mut total_win_prob = 0.0;
                            let mut win_prob = 0.0;
                            let mut perfect_win_prob = 0.0;
                            let mut tie_prob = 0.0;
                            let mut first_rapid_win_prob = 0.0;
                            let mut second_blitz_win_prob = 0.0;
                            let mut third_blitz_win_prob = 0.0;
                            let mut forth_blitz_win_prob = 0.0;
                            let mut count = 0.0;

                            for match_result in &match_results_matrix {
                                let filtered_results = match_result.iter().filter(|result| 
                                    team2_combination.iter().enumerate().all(|(index, player)| {
                                        match index {
                                            0 => player.english_name == "unknown" || player.korean_name == result.first_rapid.player2.korean_name,
                                            1 => player.english_name == "unknown" || player.korean_name == result.second_blitz.player2.korean_name,
                                            2 => player.english_name == "unknown" || player.korean_name == result.third_blitz.player2.korean_name,
                                            3 => player.english_name == "unknown" || player.korean_name == result.forth_blitz.player2.korean_name,
                                            _ => false,
                                        }
                                    })
                                );

                                for result in filtered_results.take(combinations) {
                                    if result.first_rapid.player1.korean_name == lineup.first_rapid.korean_name &&
                                    result.second_blitz.player1.korean_name == lineup.second_blitz.korean_name &&
                                    result.third_blitz.player1.korean_name == lineup.third_blitz.korean_name &&
                                    result.forth_blitz.player1.korean_name == lineup.forth_blitz.korean_name {
                                        total_win_prob += result.total_win_probability;
                                        win_prob += result.perfect_win_probability + result.win_probability;
                                        perfect_win_prob += result.perfect_win_probability;
                                        tie_prob += result.tie_probability;
                                        first_rapid_win_prob += result.first_rapid_win_probability;
                                        second_blitz_win_prob += result.second_blitz_win_probability;
                                        third_blitz_win_prob += result.third_blitz_win_probability;
                                        forth_blitz_win_prob += result.forth_blitz_win_probability;
                                        count += 1.0;
                                    }
                                }
                            }

                            if count > 0.0 {
                                avg_probabilities.push((
                                    lineup.clone(),
                                    total_win_prob / count,
                                    win_prob / count,
                                    perfect_win_prob / count,
                                    tie_prob / count,
                                    first_rapid_win_prob / count,
                                    second_blitz_win_prob / count,
                                    third_blitz_win_prob / count,
                                    forth_blitz_win_prob / count
                                ));
                            }
                        }


                        println!("========================");
                        avg_probabilities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_perfect_win_prob, avg_tie_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 총 승리확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid.korean_name, team2_combination[0].korean_name, avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz.korean_name, team2_combination[1].korean_name, avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz.korean_name, team2_combination[2].korean_name, avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz.korean_name, team2_combination[3].korean_name, avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_perfect_win_prob);
                            println!("평균 동점 확률: {:.2}%", avg_tie_prob);
                        } else {
                            println!("적합한 구성을 찾을 수 없습니다.");
                        }
                        println!("========================");
                        avg_probabilities.sort_by(|a, b| {
                            if (b.2 - a.2).abs() < 0.001 {
                                if (b.1 - a.1).abs() < 0.001 {
                                    std::cmp::Ordering::Equal
                                } else {
                                    b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
                                }
                            } else {
                                b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal)
                            }
                        });
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_perfect_win_prob, avg_tie_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 동점 없는 승리확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid.korean_name, team2_combination[0].korean_name, avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz.korean_name, team2_combination[1].korean_name, avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz.korean_name, team2_combination[2].korean_name, avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz.korean_name, team2_combination[3].korean_name, avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_perfect_win_prob);
                            println!("평균 동점 확률: {:.2}%", avg_tie_prob);
                        } else {
                            println!("적합한 구성을 찾을 수 없습니다.");
                        }
                        println!("========================");
                        avg_probabilities.sort_by(|a, b| {
                            if (b.3 - a.3).abs() < 0.001 {
                                if (b.1 - a.1).abs() < 0.001 {
                                    std::cmp::Ordering::Equal
                                } else {
                                    b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
                                }
                            } else {
                                b.3.partial_cmp(&a.3).unwrap_or(std::cmp::Ordering::Equal)
                            }
                        });
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_perfect_win_prob, avg_tie_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 완봉승확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid.korean_name, team2_combination[0].korean_name, avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz.korean_name, team2_combination[1].korean_name, avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz.korean_name, team2_combination[2].korean_name, avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz.korean_name, team2_combination[3].korean_name, avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_perfect_win_prob);
                            println!("평균 동점 확률: {:.2}%", avg_tie_prob);
                        } else {
                            println!("적합한 구성을 찾을 수 없습니다.");
                        }
                        println!("========================");
                        avg_probabilities.sort_by(|a, b| {
                            if (b.4 - a.4).abs() < 0.001 {
                                if (b.1 - a.1).abs() < 0.001 {
                                    std::cmp::Ordering::Equal
                                } else {
                                    b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
                                }
                            } else {
                                b.4.partial_cmp(&a.4).unwrap_or(std::cmp::Ordering::Equal)
                            }
                        });
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_perfect_win_prob, avg_tie_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 동점 확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid.korean_name, team2_combination[0].korean_name, avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz.korean_name, team2_combination[1].korean_name, avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz.korean_name, team2_combination[2].korean_name, avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz.korean_name, team2_combination[3].korean_name, avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_perfect_win_prob);
                            println!("평균 동점 확률: {:.2}%", avg_tie_prob);
                        } else {
                            println!("적합한 구성을 찾을 수 없습니다.");
                        }
                        println!("========================");

                        // let match_result = match_results_matrix.iter().flatten().find(|&result| {
                        //     result.first_rapid.player1.korean_name == team1_combination[0].korean_name && 
                        //     result.second_blitz.player1.korean_name == team1_combination[1].korean_name && 
                        //     result.third_blitz.player1.korean_name == team1_combination[2].korean_name && 
                        //     result.forth_blitz.player1.korean_name == team1_combination[3].korean_name && 
                        //     result.first_rapid.player2.korean_name == team2_combination[0].korean_name && 
                        //     result.second_blitz.player2.korean_name == team2_combination[1].korean_name && 
                        //     result.third_blitz.player2.korean_name == team2_combination[2].korean_name && 
                        //     result.forth_blitz.player2.korean_name == team2_combination[3].korean_name
                        // }).expect("매치 결과를 찾을 수 없습니다.");

                        // println!("========================");
                        // println!("1국 장고(rapid): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", team1_combination[0].korean_name, team2_combination[0].korean_name, chrono::Utc::now().year() - 1, match_result.first_rapid.player1_wins, match_result.first_rapid.player2_wins, match_result.first_rapid_win_probability);
                        // println!("2국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", team1_combination[1].korean_name, team2_combination[1].korean_name, chrono::Utc::now().year() - 1, match_result.second_blitz.player1_wins, match_result.second_blitz.player2_wins, match_result.second_blitz_win_probability);
                        // println!("3국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", team1_combination[2].korean_name, team2_combination[2].korean_name, chrono::Utc::now().year() - 1, match_result.third_blitz.player1_wins, match_result.third_blitz.player2_wins, match_result.third_blitz_win_probability);
                        // println!("4국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", team1_combination[3].korean_name, team2_combination[3].korean_name, chrono::Utc::now().year() - 1, match_result.forth_blitz.player1_wins, match_result.forth_blitz.player2_wins, match_result.forth_blitz_win_probability);
                        // println!("\n4-0: {:.2}%", match_result.perfect_win_probability);
                        // println!("3-1: {:.2}%", match_result.win_probability);
                        // println!("2-2: {:.2}%", match_result.tie_probability);
                        // println!("1-3: {:.2}%", match_result.lose_probability);
                        // println!("0-4: {:.2}%", match_result.perfect_defeat_probability);
                        // println!("\n총 승리확률: {:.2}%", match_result.total_win_probability);
                        // println!("에이스결정전 예상 승리확률: {:.2}%", match_result.tiebreaker_win_probability);
                        // println!("========================");

                        println!("\n계속하려면 엔터를 누르세요.");
                        let mut pause = String::new();
                        io::stdin().read_line(&mut pause).expect("입력을 읽는 데 실패했습니다.");
                    },
                    "exit" => break,
                    _ => println!("잘못된 입력입니다. 다시 입력해주세요."),
                }
            }
        },
        Err(e) => println!("상대전적을 생성하는 동안 오류가 발생했습니다: {}", e),
    }
}

fn calculate_win_probability(player1_elo: f64, player2_elo: f64) -> f64 {
    let elo_diff = player2_elo - player1_elo;
    let probability = 1.0 / (1.0 + 10.0_f64.powf(elo_diff / 400.0));
    probability
}

fn calculate_win_probability_with_relative_record(player1_elo: f64, player2_elo: f64, player1_wins: u32, player2_wins: u32) -> f64 {
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

fn fetch_player_ratings_on_baeteil() -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
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

fn fetch_player_ratings_on_goratings() -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
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

fn fetch_head_to_head_record(gisa1: &str, gisa2: &str) -> Result<HashMap<String, u32>, Box<dyn Error>> {
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

fn update_team_elo_ratings(selected_teams: &mut Vec<Team>) -> Result<(), Box<dyn Error>> {
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
        for player in team.players.iter_mut() {
            if let Some(&rating) = player_ratings_on_baeteil.get(&player.korean_name) {
                player.elo_rating = rating;
            } else if let Some(&rating) = player_ratings_on_goratings.get(&player.english_name) {
                player.elo_rating = rating + goratings_to_baeteil;
            }
        }
    }

    Ok(())
}

fn generate_player_relativities(selected_teams: &Vec<Team>) -> Result<Vec<PlayerRelativity>, String> {
    let mut all_relative_records: Vec<PlayerRelativity> = Vec::new();

    let team1 = &selected_teams[0];
    let team2 = &selected_teams[1];
    for player1 in &team1.players {
        for player2 in &team2.players {
            let record = fetch_head_to_head_record(&player1.korean_name, &player2.korean_name)
                               .map_err(|e| format!("상대전적을 가져오는 중 오류가 발생했습니다: {}", e))?;
            let player1_wins = *record.get(&player1.korean_name).unwrap_or(&0);
            let player2_wins = *record.get(&player2.korean_name).unwrap_or(&0);
            let elo_win_probability = calculate_win_probability_with_relative_record(player1.elo_rating as f64, player2.elo_rating as f64, player1_wins as u32, player2_wins as u32);
            let condition_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating + player1.condition_weight) as f64, (player2.elo_rating + player2.condition_weight) as f64, player1_wins, player2_wins);
            let rapid_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating + player1.condition_weight + player1.rapid_weight) as f64, (player2.elo_rating + player2.condition_weight + player2.rapid_weight) as f64, player1_wins, player2_wins);
            let blitz_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating + player1.condition_weight + player1.blitz_weight) as f64, (player2.elo_rating + player2.condition_weight + player2.blitz_weight) as f64, player1_wins, player2_wins);
            let bullet_win_probability = calculate_win_probability_with_relative_record((player1.elo_rating + player1.condition_weight + player1.bullet_weight) as f64, (player2.elo_rating + player2.condition_weight + player2.bullet_weight) as f64, player1_wins, player2_wins);
            all_relative_records.push(PlayerRelativity {
                player1: player1.clone(),
                player2: player2.clone(),
                player1_wins,
                player2_wins,
                elo_win_probability: elo_win_probability * 100.0,
                condition_win_probability: condition_win_probability * 100.0,
                rapid_win_probability: rapid_win_probability * 100.0,
                blitz_win_probability: blitz_win_probability * 100.0,
                bullet_win_probability: bullet_win_probability * 100.0,
            });
        }
    }

    Ok(all_relative_records)
}

fn calculate_match_result(team1_lineup: Lineup, team2_lineup: Lineup, player_relativities: Vec<PlayerRelativity>) -> MatchResult {
    let team1_players = vec![team1_lineup.first_rapid, team1_lineup.second_blitz, team1_lineup.third_blitz, team1_lineup.forth_blitz];
    let team2_players = vec![team2_lineup.first_rapid, team2_lineup.second_blitz, team2_lineup.third_blitz, team2_lineup.forth_blitz];

    let mut win_probabilities = vec![0.0; team1_players.len()];
    let mut bullet_win_probabilities = vec![0.0; team1_players.len()];

    for (i, player1) in team1_players.iter().enumerate() {
        if let Some(player2) = team2_players.get(i) {
            if let Some(relativity) = player_relativities.iter().find(|r| r.player1.korean_name == player1.korean_name && r.player2.korean_name == player2.korean_name) {
                win_probabilities[i] = match i {
                    0 => relativity.rapid_win_probability,
                    _ => relativity.blitz_win_probability,
                };
                bullet_win_probabilities[i] = relativity.bullet_win_probability;
            }
        }
    }

    let tiebreaker_win_probability = player_relativities.iter()
        .map(|relativity| {
            let player1_position = team1_players.iter().position(|p| p.korean_name == relativity.player1.korean_name);
            let player2_position = team2_players.iter().position(|p| p.korean_name == relativity.player2.korean_name);
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
            (relativity.player1.korean_name.clone(), relativity.bullet_win_probability * player1_penalty * player2_penalty)
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

    MatchResult {
        first_rapid: player_relativities.iter().find(|relativity| relativity.player1.korean_name == team1_players[0].korean_name && relativity.player2.korean_name == team2_players[0].korean_name).unwrap().clone(),
        second_blitz: player_relativities.iter().find(|relativity| relativity.player1.korean_name == team1_players[1].korean_name && relativity.player2.korean_name == team2_players[1].korean_name).unwrap().clone(),
        third_blitz: player_relativities.iter().find(|relativity| relativity.player1.korean_name == team1_players[2].korean_name && relativity.player2.korean_name == team2_players[2].korean_name).unwrap().clone(),
        forth_blitz: player_relativities.iter().find(|relativity| relativity.player1.korean_name == team1_players[3].korean_name && relativity.player2.korean_name == team2_players[3].korean_name).unwrap().clone(),
        first_rapid_win_probability: win_probabilities[0],
        second_blitz_win_probability: win_probabilities[1],
        third_blitz_win_probability: win_probabilities[2],
        forth_blitz_win_probability: win_probabilities[3],
        perfect_win_probability: all_win_probability * 100.0,
        win_probability: three_win_one_lose_probability * 100.0,
        tie_probability: two_win_two_lose_probability * 100.0,
        lose_probability: one_win_three_lose_probability * 100.0,
        perfect_defeat_probability: all_lose_probability * 100.0,
        tiebreaker_win_probability: tiebreaker_win_probability,
        total_win_probability: total_win_probability * 100.0,
    }
}

fn create_excel_from_relativities(player_relativities: Vec<PlayerRelativity>, match_results_matrix: Vec<Vec<MatchResult>>) -> Result<(), Box<dyn std::error::Error>> {
    let workbook = Workbook::new("player_relativities.xlsx")?;
    let mut worksheet_elo = workbook.add_worksheet(Some("개인-기본ELO 기반"))?;
    let mut worksheet_condition = workbook.add_worksheet(Some("개인-컨디션 기반"))?;
    let mut worksheet_rapid = workbook.add_worksheet(Some("개인-장고 기반"))?;
    let mut worksheet_blitz = workbook.add_worksheet(Some("개인-속기 기반"))?;
    let mut worksheet_bullet = workbook.add_worksheet(Some("개인-초속기 기반"))?;

    let mut player1_set = HashSet::new();
    let mut player2_set = HashSet::new();

    for relativity in &player_relativities {
        player1_set.insert(relativity.player1.korean_name.clone());
        player2_set.insert(relativity.player2.korean_name.clone());
    }

    // let player1s: Vec<_> = player1_set.into_iter().sorted_by_key(|player| -(player_relativities.iter().find(|relativity| &relativity.player1.korean_name == player).map_or(0, |relativity| relativity.player1.elo_rating + relativity.player1.condition_weight))).collect();
    // let player2s: Vec<_> = player2_set.into_iter().sorted_by_key(|player| -(player_relativities.iter().find(|relativity| &relativity.player2.korean_name == player).map_or(0, |relativity| relativity.player2.elo_rating + relativity.player2.condition_weight))).collect();
    let player1s: Vec<_> = player1_set.into_iter().sorted_by(|a, b| {
        let a_score = player_relativities.iter().find(|relativity| &relativity.player1.korean_name == a)
            .map_or(0.0, |relativity| relativity.player1.elo_rating + relativity.player1.condition_weight);
        let b_score = player_relativities.iter().find(|relativity| &relativity.player1.korean_name == b)
            .map_or(0.0, |relativity| relativity.player1.elo_rating + relativity.player1.condition_weight);
        b_score.partial_cmp(&a_score).unwrap_or(std::cmp::Ordering::Equal)
    }).collect();
    let player2s: Vec<_> = player2_set.into_iter().sorted_by(|a, b| {
        let a_score = player_relativities.iter().find(|relativity| &relativity.player2.korean_name == a)
            .map_or(0.0, |relativity| relativity.player2.elo_rating + relativity.player2.condition_weight);
        let b_score = player_relativities.iter().find(|relativity| &relativity.player2.korean_name == b)
            .map_or(0.0, |relativity| relativity.player2.elo_rating + relativity.player2.condition_weight);
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
        let row = player1_index[&relativity.player1.korean_name];
        let col = player2_index[&relativity.player2.korean_name];

        let elo_format = create_custom_format(relativity.elo_win_probability, 15.0)?;
        let condition_format = create_custom_format(relativity.condition_win_probability, 15.0)?;
        let rapid_format = create_custom_format(relativity.rapid_win_probability, 15.0)?;
        let blitz_format = create_custom_format(relativity.blitz_win_probability, 15.0)?;
        let bullet_format = create_custom_format(relativity.bullet_win_probability, 15.0)?;

        worksheet_elo.write_number(row.try_into().unwrap(), col.try_into().unwrap(), relativity.elo_win_probability / 100.0, Some(&elo_format))?;
        worksheet_condition.write_number(row.try_into().unwrap(), col.try_into().unwrap(), relativity.condition_win_probability / 100.0, Some(&condition_format))?;
        worksheet_rapid.write_number(row.try_into().unwrap(), col.try_into().unwrap(), relativity.rapid_win_probability / 100.0, Some(&rapid_format))?;
        worksheet_blitz.write_number(row.try_into().unwrap(), col.try_into().unwrap(), relativity.blitz_win_probability / 100.0, Some(&blitz_format))?;
        worksheet_bullet.write_number(row.try_into().unwrap(), col.try_into().unwrap(), relativity.bullet_win_probability / 100.0, Some(&bullet_format))?;
    }

    let mut worksheet_total_win = workbook.add_worksheet(Some("팀-최종승리"))?;
    let mut worksheet_win = workbook.add_worksheet(Some("팀-에결없이 승리"))?;
    let mut worksheet_perfect_win = workbook.add_worksheet(Some("팀-완봉승"))?;
    let mut worksheet_tiebreak = workbook.add_worksheet(Some("팀-에결진출"))?;

    for (row_index, row) in match_results_matrix.iter().enumerate() {
        if row_index == 0 { // 첫 번째 행에 상대 팀 라인업 이름 쓰기
            for (col_index, match_result) in row.iter().enumerate().take(24) {
                let lineup_names = format!("1국:{}, 2국:{}, 3국:{}, 4국:{}", 
                    match_result.second_blitz.player2.korean_name, 
                    match_result.third_blitz.player2.korean_name, 
                    match_result.forth_blitz.player2.korean_name, 
                    match_result.first_rapid.player2.korean_name);
                worksheet_total_win.write_string(0, col_index as u16 + 1, &lineup_names, None)?;
                worksheet_win.write_string(0, col_index as u16 + 1, &lineup_names, None)?;
                worksheet_perfect_win.write_string(0, col_index as u16 + 1, &lineup_names, None)?;
                worksheet_tiebreak.write_string(0, col_index as u16 + 1, &lineup_names, None)?;
            }
        }
        // 첫 번째 열에 자신의 팀 라인업 이름 쓰기
        let lineup_names = format!("1국:{}, 2국:{}, 3국:{}, 4국:{}", 
            row[0].second_blitz.player1.korean_name, 
            row[0].third_blitz.player1.korean_name, 
            row[0].forth_blitz.player1.korean_name, 
            row[0].first_rapid.player1.korean_name);
        worksheet_total_win.write_string(row_index as u32 + 1, 0, &lineup_names, None)?;
        worksheet_win.write_string(row_index as u32 + 1, 0, &lineup_names, None)?;
        worksheet_perfect_win.write_string(row_index as u32 + 1, 0, &lineup_names, None)?;
        worksheet_tiebreak.write_string(row_index as u32 + 1, 0, &lineup_names, None)?;

        for (col_index, match_result) in row.iter().enumerate().take(36) {
            let total_win_format = create_custom_format(match_result.total_win_probability, 25.0)?;
            let win_format = create_custom_format(match_result.perfect_win_probability + match_result.win_probability, 25.0)?;
            let perfect_win_format = create_custom_format(match_result.perfect_win_probability, 25.0)?;
            let tiebreaker_format = create_custom_format(match_result.tie_probability, 25.0)?;

            worksheet_total_win.write_number(row_index as u32 + 1, col_index as u16 + 1, match_result.total_win_probability / 100.0, Some(&total_win_format))?;
            worksheet_win.write_number(row_index as u32 + 1, col_index as u16 + 1, (match_result.perfect_win_probability + match_result.win_probability) / 100.0, Some(&win_format))?;
            worksheet_perfect_win.write_number(row_index as u32 + 1, col_index as u16 + 1, match_result.perfect_win_probability / 100.0, Some(&perfect_win_format))?;
            worksheet_tiebreak.write_number(row_index as u32 + 1, col_index as u16 + 1, match_result.tie_probability / 100.0, Some(&tiebreaker_format))?;
        }
        // for (col_index, match_result) in row.iter().enumerate() {
        //     let total_win_format = create_custom_format(match_result.total_win_probability, 25.0)?;
        //     worksheet_total_win.write_number(row_index.try_into().unwrap(), col_index.try_into().unwrap(), match_result.total_win_probability / 100.0, Some(&total_win_format))?;
        // }
    }

    workbook.close()?;

    Ok(())
}

fn create_custom_format(win_probability: f64, maximum: f64) -> Result<Format, Box<dyn std::error::Error>> {
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
