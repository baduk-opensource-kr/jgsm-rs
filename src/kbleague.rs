use crate::models::{Lineup, MatchResult, Player, PlayerRelativity, Team, TeamRelativity};
use crate::utils;
use chrono::{Datelike, NaiveDate};
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use tokio;

fn init_teams() -> Vec<Team> {
    let mut teams: Vec<Team> = Vec::new();
    teams.push(Team::new(
        "한국물가정보".to_string(),
        vec![
            Player::new("강동윤".to_string(), "Kang Dongyun".to_string(), "姜东润".to_string(), NaiveDate::from_ymd_opt(1989, 1, 23).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("한승주".to_string(), "Han Seungjoo".to_string(), "韩升周".to_string(), NaiveDate::from_ymd_opt(1996, 11, 27).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("박민규".to_string(), "Park Minkyu".to_string(), "朴珉奎".to_string(), NaiveDate::from_ymd_opt(1994, 6, 5).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("최재영".to_string(), "Choi Jaeyoung".to_string(), "崔宰荣".to_string(), NaiveDate::from_ymd_opt(1997, 4, 10).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("이춘규".to_string(), "Lee Chungyu".to_string(), "李春揆".to_string(), NaiveDate::from_ymd_opt(1989, 3, 27).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("당이페이".to_string(), "Dang Yifei".to_string(), "党毅飞".to_string(), NaiveDate::from_ymd_opt(1995, 6, 17).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
        ]
    ));
    teams.push(Team::new(
        "수려한합천".to_string(),
        vec![
            Player::new("원성진".to_string(), "Weon Seongjin".to_string(), "元晟溱".to_string(), NaiveDate::from_ymd_opt(1985, 7, 15).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("한우진".to_string(), "Han Woojin".to_string(), "韩友赈".to_string(), NaiveDate::from_ymd_opt(2005, 6, 12).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("송지훈".to_string(), "Song Jihoon".to_string(), "宋知勋".to_string(), NaiveDate::from_ymd_opt(1998, 2, 23).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("한태희".to_string(), "Han Taehee".to_string(), "韩态熙".to_string(), NaiveDate::from_ymd_opt(1993, 9, 17).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("윤성식".to_string(), "Yun Seongsik".to_string(), "尹圣植".to_string(), NaiveDate::from_ymd_opt(2000, 6, 25).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("김승진".to_string(), "Kim Seungjin".to_string(), "金升珍".to_string(), NaiveDate::from_ymd_opt(2006, 5, 19).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
        ]
    ));
    teams.push(Team::new(
        "마한의 심장 영암".to_string(),
        vec![
            Player::new("안성준".to_string(), "An Sungjoon".to_string(), "安成浚".to_string(), NaiveDate::from_ymd_opt(1991, 9, 16).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("설현준".to_string(), "Seol Hyunjun".to_string(), "偰玹准".to_string(), NaiveDate::from_ymd_opt(1999, 1, 29).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("최철한".to_string(), "Choi Cheolhan".to_string(), "崔哲瀚".to_string(), NaiveDate::from_ymd_opt(1985, 3, 12).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("박종훈".to_string(), "Park Jonghoon".to_string(), "朴钟勋".to_string(), NaiveDate::from_ymd_opt(2000, 1, 14).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("엄동건".to_string(), "Eom Donggeon".to_string(), "严动虔".to_string(), NaiveDate::from_ymd_opt(2000, 6, 9).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("쉬하오훙".to_string(), "Xu Haohong".to_string(), "许皓鋐".to_string(), NaiveDate::from_ymd_opt(2001, 4, 30).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
        ]
    ));
    teams.push(Team::new(
        "정관장천녹".to_string(),
        vec![
            Player::new("변상일".to_string(), "Byun Sangil".to_string(), "卞相壹".to_string(), NaiveDate::from_ymd_opt(1997, 1, 14).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("홍성지".to_string(), "Hong Seongji".to_string(), "洪性志".to_string(), NaiveDate::from_ymd_opt(1987, 8, 7).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("김정현(大)".to_string(), "Kim Junghyun".to_string(), "金庭贤".to_string(), NaiveDate::from_ymd_opt(1991, 4, 12).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("한상훈".to_string(), "Han Sanghoon".to_string(), "韩尙勋".to_string(), NaiveDate::from_ymd_opt(1988, 5, 16).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("김승구".to_string(), "Kim Seunggu".to_string(), "金丞求".to_string(), NaiveDate::from_ymd_opt(2006, 6, 13).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("박상진".to_string(), "Park Sangjin".to_string(), "朴常镇".to_string(), NaiveDate::from_ymd_opt(2001, 5, 19).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
        ]
    ));
    teams.push(Team::new(
        "울산 고려아연".to_string(),
        vec![
            Player::new("신민준".to_string(), "Shin Minjun".to_string(), "申旻埈".to_string(), NaiveDate::from_ymd_opt(1999, 1, 11).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("이창석".to_string(), "Lee Changseok".to_string(), "李昌锡".to_string(), NaiveDate::from_ymd_opt(1996, 4, 27).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("문민종".to_string(), "Moon Minjong".to_string(), "文敏钟".to_string(), NaiveDate::from_ymd_opt(2003, 2, 12).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("한상조".to_string(), "Han Sangcho".to_string(), "韩相朝".to_string(), NaiveDate::from_ymd_opt(1999, 9, 28).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("김채영".to_string(), "Kim Chaeyoung".to_string(), "金彩瑛".to_string(), NaiveDate::from_ymd_opt(1996, 1, 15).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("랴오위안허".to_string(), "Liao Yuanhe".to_string(), "廖元赫".to_string(), NaiveDate::from_ymd_opt(2000, 12, 20).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
        ]
    ));
    teams.push(Team::new(
        "바둑메카 의정부".to_string(),
        vec![
            Player::new("김명훈".to_string(), "Kim Myounghoon".to_string(), "金明训".to_string(), NaiveDate::from_ymd_opt(1997, 4, 7).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("박건호".to_string(), "Park Geunho".to_string(), "朴键昊".to_string(), NaiveDate::from_ymd_opt(1998, 6, 14).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("이원영".to_string(), "Lee Wonyoung".to_string(), "李元荣".to_string(), NaiveDate::from_ymd_opt(1992, 5, 8).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("허영호".to_string(), "Heo Yongho".to_string(), "许映皓".to_string(), NaiveDate::from_ymd_opt(1986, 7, 2).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("박재근".to_string(), "Park Jaekeun".to_string(), "朴材根".to_string(), NaiveDate::from_ymd_opt(1996, 4, 16).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("양카이원".to_string(), "Yang Kaiwen".to_string(), "杨楷文".to_string(), NaiveDate::from_ymd_opt(1997, 1, 28).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
        ]
    ));
    teams.push(Team::new(
        "Kixx".to_string(),
        vec![
            Player::new("신진서".to_string(), "Shin Jinseo".to_string(), "申真谞".to_string(), NaiveDate::from_ymd_opt(2000, 3, 17).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("박진솔".to_string(), "Park Jinsol".to_string(), "朴进率".to_string(), NaiveDate::from_ymd_opt(1986, 9, 7).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("김승재".to_string(), "Kim Seungjae".to_string(), "金升宰".to_string(), NaiveDate::from_ymd_opt(1992, 8, 11).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("백현우".to_string(), "Baek Hyeonwoo".to_string(), "白现宇".to_string(), NaiveDate::from_ymd_opt(2001, 2, 12).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("김창훈".to_string(), "Kim Changhoon".to_string(), "金昌勋".to_string(), NaiveDate::from_ymd_opt(1995, 8, 20).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
        ]
    ));
    teams.push(Team::new(
        "원익".to_string(),
        vec![
            Player::new("박정환".to_string(), "Park Junghwan".to_string(), "朴廷桓".to_string(), NaiveDate::from_ymd_opt(1993, 1, 11).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("이지현(남)".to_string(), "Lee Jihyun (m)".to_string(), "李志贤".to_string(), NaiveDate::from_ymd_opt(1992, 9, 30).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("박영훈".to_string(), "Park Yeonghun".to_string(), "朴永训".to_string(), NaiveDate::from_ymd_opt(1985, 4, 1).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("김진휘".to_string(), "Kim Jinhwi".to_string(), "金真辉".to_string(), NaiveDate::from_ymd_opt(1996, 1, 26).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("금지우".to_string(), "Geum Jiwoo".to_string(), "琴沚玗".to_string(), NaiveDate::from_ymd_opt(2001, 8, 29).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
            Player::new("구쯔하오".to_string(), "Gu Zihao".to_string(), "辜梓豪".to_string(), NaiveDate::from_ymd_opt(1998, 3, 13).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0),
        ]
    ));
    teams
}

pub fn execute_kbleague() {
    let mut teams = init_teams();
    let mut selected_teams: Vec<Team> = Vec::new();
    for _ in 0..2 {
        loop {
            println!("팀{}을 선택하세요: ", selected_teams.len() + 1);

            for (index, team) in teams.iter().enumerate() {
                println!("{}. {}", index + 1, team.team_name());
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
    if let Err(e) = utils::update_team_elo_ratings(&mut selected_teams) {
        println!("ELO 레이팅을 업데이트하는 동안 오류가 발생했습니다: {}", e);
        return;
    } else {
        println!("ELO 레이팅이 성공적으로 업데이트되었습니다.");
    }

    for selected_team in &mut selected_teams {
        loop {
            println!("\n{} 팀의 스쿼드:", selected_team.team_name());
            for (index, player) in selected_team.players().iter().enumerate() {
                println!("{}. {} (elo: {:.2})", index + 1, player.korean_name(), player.elo_rating());
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

            if selected_index > 0 && selected_index <= selected_team.players().len() {
                let removed_player = selected_team.remove_player(selected_index - 1);
                println!("{} 기사가 목록에서 제외되었습니다.", removed_player.korean_name());
            } else {
                println!("유효한 기사 번호를 입력해주세요.");
            }
        }

        println!("\n{} 팀의 기사에 대한 컨디션 가중치를 입력하세요.", selected_team.team_name());
        loop {
            for (index, player) in selected_team.players().iter().enumerate() {
                println!("{}. {} (elo: {:.2})\n    컨디션 가중치: {:.2}", index + 1, player.korean_name(), player.elo_rating(), player.condition_weight());
            }
            println!("컨디션 가중치를 입력할 기사를 선택하세요 (완료시 엔터): ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
            if input.trim().is_empty() {
                break;
            }
            let selected_index: usize = input.trim().parse().expect("정수를 입력해주세요.");

            if selected_index > 0 && selected_index <= selected_team.players().len() {
                let player = &mut selected_team.players_mut()[selected_index - 1];

                input.clear();
                println!("\n{} 기사의 컨디션 가중치를 입력하세요.(음수 입력 가능) (변경하지 않으려면 엔터): ", player.korean_name());
                io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
                if !input.trim().is_empty() {
                    player.set_condition_weight(input.trim().parse().expect("정수를 입력해주세요."));
                }
            } else {
                println!("유효한 기사 번호를 입력해주세요.");
            }
        }

        println!("\n{} 팀의 기사에 대한 게임속도 가중치를 입력하세요.", selected_team.team_name());
        loop {
            for (index, player) in selected_team.players().iter().enumerate() {
                println!("{}. {} (elo: {:.2})\n    장고(Rapid) 가중치: {:.2}\n    속기(Blitz) 가중치: {:.2}\n    초속기(Bullet) 가중치: {:.2}", index + 1, player.korean_name(), player.elo_rating(), player.rapid_weight(), player.blitz_weight(), player.bullet_weight());
            }
            println!("게임속도 가중치를 입력할 기사를 선택하세요 (완료시 엔터): ");
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
            if input.trim().is_empty() {
                break;
            }
            let selected_index: usize = input.trim().parse().expect("정수를 입력해주세요.");

            if selected_index > 0 && selected_index <= selected_team.players().len() {
                let player = &mut selected_team.players_mut()[selected_index - 1];

                input.clear();
                println!("\n{} 기사의 장고(Rapid) 가중치를 입력하세요.(음수 입력 가능) (변경하지 않으려면 엔터): ", player.korean_name());
                io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
                if !input.trim().is_empty() {
                    player.set_rapid_weight(input.trim().parse().expect("정수를 입력해주세요."));
                }
                input.clear();

                println!("{} 기사의 속기(Blitz) 가중치를 입력하세요.(음수 입력 가능) (변경하지 않으려면 엔터): ", player.korean_name());
                io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
                if !input.trim().is_empty() {
                    player.set_blitz_weight(input.trim().parse().expect("정수를 입력해주세요."));
                }
                input.clear();

                println!("{} 기사의 초속기(Bullet) 가중치를 입력하세요.(음수 입력 가능) (변경하지 않으려면 엔터): ", player.korean_name());
                io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
                if !input.trim().is_empty() {
                    player.set_bullet_weight(input.trim().parse().expect("정수를 입력해주세요."));
                }
            } else {
                println!("유효한 기사 번호를 입력해주세요.");
            }
        }
    }

    println!("\n업데이트된 스쿼드:");
    for team in selected_teams.iter() {
        println!("\n{}:", team.team_name());
        for player in team.players() {
            println!("{} (elo: {:.2}, 컨디션: {:.2}, 장고: {:.2}, 속기: {:.2}, 초속기: {:.2})", player.korean_name(), player.elo_rating(), player.condition_weight(), player.rapid_weight(), player.blitz_weight(), player.bullet_weight());
        }
    }

    let mut team1_all_lineups: Vec<Lineup> = Vec::new();
    let mut team2_all_lineups: Vec<Lineup> = Vec::new();

    for team_index in 0..2 {
        let team_players = selected_teams[team_index].players();
        let all_lineups = if team_index == 0 { &mut team1_all_lineups } else { &mut team2_all_lineups };

        for first_rapid in team_players {
            for second_blitz in team_players {
                if second_blitz == first_rapid { continue; }
                for third_blitz in team_players {
                    if third_blitz == first_rapid || third_blitz == second_blitz { continue; }
                    for forth_blitz in team_players {
                        if forth_blitz == first_rapid || forth_blitz == second_blitz || forth_blitz == third_blitz { continue; }
                        all_lineups.push(Lineup::new(first_rapid.clone(), second_blitz.clone(), third_blitz.clone(), forth_blitz.clone()));
                    }
                }
            }
        }
    }

    println!("\n상대전적을 업데이트 중...");
    match utils::generate_player_relativities(&selected_teams) {
        Ok(player_relativities) => {
            println!("\n라인업 메트릭스 생성 중...");
            let mut match_results_matrix: Vec<Vec<MatchResult>> = Vec::new();
            let mut team1_lineups_with_avg: Vec<(Lineup, f64)> = team1_all_lineups.iter().map(|lineup| {
                let avg_total_win_probability: f64 = team2_all_lineups.iter().map(|opponent_lineup| {
                    let match_result = utils::calculate_match_result(lineup.clone(), opponent_lineup.clone(), player_relativities.clone());
                    match_result.total_win_probability()
                }).sum::<f64>() / team2_all_lineups.len() as f64;
                (lineup.clone(), avg_total_win_probability)
            }).collect();

            let mut team2_lineups_with_avg: Vec<(Lineup, f64)> = team2_all_lineups.iter().map(|lineup| {
                let avg_total_win_probability: f64 = team1_all_lineups.iter().map(|opponent_lineup| {
                    let match_result = utils::calculate_match_result(opponent_lineup.clone(), lineup.clone(), player_relativities.clone());
                    match_result.total_win_probability()
                }).sum::<f64>() / team1_all_lineups.len() as f64;
                (lineup.clone(), avg_total_win_probability)
            }).collect();

            team1_lineups_with_avg.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            team2_lineups_with_avg.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            for (team1_lineup, _) in team1_lineups_with_avg {
                let mut row: Vec<MatchResult> = Vec::new();
                for (team2_lineup, _) in &team2_lineups_with_avg {
                    let match_result = utils::calculate_match_result(team1_lineup.clone(), team2_lineup.clone(), player_relativities.clone());
                    row.push(match_result);
                }
                match_results_matrix.push(row);
            }

            loop {
                println!("\n선택할 옵션:");
                println!("1. {}의 스쿼드", selected_teams[0].team_name());
                println!("2. {}의 스쿼드", selected_teams[1].team_name());
                println!("3. 양팀의 라인업 메트릭스를 Excel로 출력\n");
                println!("4. {} 최고 평균승률 라인업", selected_teams[0].team_name());
                println!("5. {} 베스트24 라인업에 대한 {} 최고 평균승률 라인업", selected_teams[1].team_name(), selected_teams[0].team_name());
                println!("6. {} 미니맥스 라인업(최선 + 상대 카운터픽)", selected_teams[0].team_name());
                println!("7. {} 예상라인업에 대한 {} 카운터픽(최고평균)", selected_teams[1].team_name(), selected_teams[0].team_name());
                println!("8. {} 예상라인업에 대한 {} 카운터픽(미니맥스)\n", selected_teams[1].team_name(), selected_teams[0].team_name());

                println!("9. 양측최선 라인업 승리확률");
                println!("10. 지정 라인업 승리확률");
                println!("11. 에이스 결정전 Excel로 출력");
                println!("12. 실시간 팀 승률\n");

                println!("13. 포스트시즌: {} 5인에 대한 {} 카운터픽(개발중..)", selected_teams[1].team_name(), selected_teams[0].team_name());
                println!("14. 포스트시즌: 5판 3선승제 3-1-1 순서 오더 실시간 최선의 라인업(개발중..)\n");
                println!("exit. 처음으로 돌아가기");

                let mut option = String::new();
                io::stdin().read_line(&mut option).expect("입력을 읽는 데 실패했습니다.");
                let option = option.trim();

                match option {
                    "1" => {
                        println!("\n{} 팀의 스쿼드:", selected_teams[0].team_name());
                        for player in selected_teams[0].players() {
                            println!("{} (elo: {:.2}, 컨디션: {:.2}, 장고: {:.2}, 속기: {:.2}, 초속기: {:.2})", player.korean_name(), player.elo_rating(), player.condition_weight(), player.rapid_weight(), player.blitz_weight(), player.bullet_weight());
                        }
                    },
                    "2" => {
                        println!("\n{} 팀의 스쿼드:", selected_teams[1].team_name());
                        for player in selected_teams[1].players() {
                            println!("{} (elo: {:.2}, 컨디션: {:.2}, 장고: {:.2}, 속기: {:.2}, 초속기: {:.2})", player.korean_name(), player.elo_rating(), player.condition_weight(), player.rapid_weight(), player.blitz_weight(), player.bullet_weight());
                        }
                    },
                    "3" => {
                        match utils::create_excel_from_relativities(player_relativities.clone(), match_results_matrix.clone()) {
                            Ok(_) => println!("Excel 파일이 성공적으로 생성되었습니다."),
                            Err(e) => println!("Excel 파일 생성 중 오류가 발생했습니다: {}", e),
                        }
                    },
                    "4" => {
                        let team1_filtered_lineups = utils::filter_team1_lineups(&selected_teams, &team1_all_lineups);

                        let mut avg_probabilities: Vec<(Lineup, f64, f64, f64, f64, f64, f64, f64, f64)> = Vec::new();
                        for lineup in &team1_filtered_lineups {
                            let mut total_win_prob = 0.0;
                            let mut three_one_prob = 0.0;
                            let mut four_zero_prob = 0.0;
                            let mut two_two_prob = 0.0;
                            let mut first_rapid_win_prob = 0.0;
                            let mut second_blitz_win_prob = 0.0;
                            let mut third_blitz_win_prob = 0.0;
                            let mut forth_blitz_win_prob = 0.0;
                            let mut count = 0.0;

                            for match_result in &match_results_matrix {
                                for result in match_result {
                                    if result.first_rapid().player1().korean_name() == lineup.first_rapid().korean_name() &&
                                    result.second_blitz().player1().korean_name() == lineup.second_blitz().korean_name() &&
                                    result.third_blitz().player1().korean_name() == lineup.third_blitz().korean_name() &&
                                    result.forth_blitz().player1().korean_name() == lineup.forth_blitz().korean_name() {
                                        total_win_prob += result.total_win_probability();
                                        three_one_prob += result.four_zero_probability() + result.three_one_probability();
                                        four_zero_prob += result.four_zero_probability();
                                        two_two_prob += result.two_two_probability();
                                        first_rapid_win_prob += result.first_rapid_win_probability();
                                        second_blitz_win_prob += result.second_blitz_win_probability();
                                        third_blitz_win_prob += result.third_blitz_win_probability();
                                        forth_blitz_win_prob += result.forth_blitz_win_probability();
                                        count += 1.0;
                                    }
                                }
                            }

                            if count > 0.0 {
                                avg_probabilities.push((
                                    lineup.clone(),
                                    total_win_prob / count,
                                    three_one_prob / count,
                                    four_zero_prob / count,
                                    two_two_prob / count,
                                    first_rapid_win_prob / count,
                                    second_blitz_win_prob / count,
                                    third_blitz_win_prob / count,
                                    forth_blitz_win_prob / count
                                ));
                            }
                        }


                        println!("========================");
                        avg_probabilities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_four_zero_prob, avg_two_two_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 총 승리확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid().korean_name(), avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz().korean_name(), avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz().korean_name(), avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz().korean_name(), avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_four_zero_prob);
                            println!("평균 동점 확률: {:.2}%", avg_two_two_prob);
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
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_four_zero_prob, avg_two_two_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 동점 없는 승리확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid().korean_name(), avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz().korean_name(), avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz().korean_name(), avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz().korean_name(), avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_four_zero_prob);
                            println!("평균 동점 확률: {:.2}%", avg_two_two_prob);
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
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_four_zero_prob, avg_two_two_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 완봉승확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid().korean_name(), avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz().korean_name(), avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz().korean_name(), avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz().korean_name(), avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_four_zero_prob);
                            println!("평균 동점 확률: {:.2}%", avg_two_two_prob);
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
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_four_zero_prob, avg_two_two_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 동점 확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid().korean_name(), avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz().korean_name(), avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz().korean_name(), avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz().korean_name(), avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_four_zero_prob);
                            println!("평균 동점 확률: {:.2}%", avg_two_two_prob);
                        } else {
                            println!("적합한 구성을 찾을 수 없습니다.");
                        }
                        println!("========================");

                        println!("\n계속하려면 엔터를 누르세요.");
                        let mut pause = String::new();
                        io::stdin().read_line(&mut pause).expect("입력을 읽는 데 실패했습니다.");
                    },
                    "5" => {
                        let team1_filtered_lineups = utils::filter_team1_lineups(&selected_teams, &team1_all_lineups);

                        let mut avg_probabilities: Vec<(Lineup, f64, f64, f64, f64, f64, f64, f64, f64)> = Vec::new();
                        for lineup in &team1_filtered_lineups {
                            let mut total_win_prob = 0.0;
                            let mut three_one_prob = 0.0;
                            let mut four_zero_prob = 0.0;
                            let mut two_two_prob = 0.0;
                            let mut first_rapid_win_prob = 0.0;
                            let mut second_blitz_win_prob = 0.0;
                            let mut third_blitz_win_prob = 0.0;
                            let mut forth_blitz_win_prob = 0.0;
                            let mut count = 0.0;

                            if let Some(match_result) = match_results_matrix.iter().find(|match_result| 
                                match_result.get(0).map_or(false, |result| 
                                    result.first_rapid().player1().korean_name() == lineup.first_rapid().korean_name() &&
                                    result.second_blitz().player1().korean_name() == lineup.second_blitz().korean_name() &&
                                    result.third_blitz().player1().korean_name() == lineup.third_blitz().korean_name() &&
                                    result.forth_blitz().player1().korean_name() == lineup.forth_blitz().korean_name()
                                )
                            ) {
                                for result in match_result.iter().take(24) {
                                    total_win_prob += result.total_win_probability();
                                    three_one_prob += result.four_zero_probability() + result.three_one_probability();
                                    four_zero_prob += result.four_zero_probability();
                                    two_two_prob += result.two_two_probability();
                                    first_rapid_win_prob += result.first_rapid_win_probability();
                                    second_blitz_win_prob += result.second_blitz_win_probability();
                                    third_blitz_win_prob += result.third_blitz_win_probability();
                                    forth_blitz_win_prob += result.forth_blitz_win_probability();
                                    count += 1.0;
                                }
                            }

                            if count > 0.0 {
                                avg_probabilities.push((
                                    lineup.clone(),
                                    total_win_prob / count,
                                    three_one_prob / count,
                                    four_zero_prob / count,
                                    two_two_prob / count,
                                    first_rapid_win_prob / count,
                                    second_blitz_win_prob / count,
                                    third_blitz_win_prob / count,
                                    forth_blitz_win_prob / count
                                ));
                            }
                        }


                        println!("========================");
                        avg_probabilities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_four_zero_prob, avg_two_two_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 총 승리확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid().korean_name(), avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz().korean_name(), avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz().korean_name(), avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz().korean_name(), avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_four_zero_prob);
                            println!("평균 동점 확률: {:.2}%", avg_two_two_prob);
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
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_four_zero_prob, avg_two_two_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 동점 없는 승리확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid().korean_name(), avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz().korean_name(), avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz().korean_name(), avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz().korean_name(), avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_four_zero_prob);
                            println!("평균 동점 확률: {:.2}%", avg_two_two_prob);
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
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_four_zero_prob, avg_two_two_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 완봉승확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid().korean_name(), avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz().korean_name(), avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz().korean_name(), avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz().korean_name(), avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_four_zero_prob);
                            println!("평균 동점 확률: {:.2}%", avg_two_two_prob);
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
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_four_zero_prob, avg_two_two_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 동점 확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid().korean_name(), avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz().korean_name(), avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz().korean_name(), avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz().korean_name(), avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_four_zero_prob);
                            println!("평균 동점 확률: {:.2}%", avg_two_two_prob);
                        } else {
                            println!("적합한 구성을 찾을 수 없습니다.");
                        }
                        println!("========================");

                        println!("\n계속하려면 엔터를 누르세요.");
                        let mut pause = String::new();
                        io::stdin().read_line(&mut pause).expect("입력을 읽는 데 실패했습니다.");
                    },
                    "6" => {
                        let team1_filtered_lineups = utils::filter_team1_lineups(&selected_teams, &team1_all_lineups);

                        let mut best_match_result: Option<&MatchResult> = None;
                        let mut highest_min_total_win_prob = 0.0;

                        for lineup in &team1_filtered_lineups {
                            let mut min_total_win_prob = std::f64::MAX;

                            for match_result in &match_results_matrix {
                                for result in match_result.iter() {
                                    if result.first_rapid().player1().korean_name() == lineup.first_rapid().korean_name() &&
                                    result.second_blitz().player1().korean_name() == lineup.second_blitz().korean_name() &&
                                    result.third_blitz().player1().korean_name() == lineup.third_blitz().korean_name() &&
                                    result.forth_blitz().player1().korean_name() == lineup.forth_blitz().korean_name() {
                                        if result.total_win_probability() < min_total_win_prob {
                                            min_total_win_prob = result.total_win_probability();
                                        }
                                    }
                                }
                            }

                            if min_total_win_prob != std::f64::MAX && min_total_win_prob > highest_min_total_win_prob {
                                highest_min_total_win_prob = min_total_win_prob;
                                best_match_result = match_results_matrix.iter().flatten().find(|&r| 
                                    r.first_rapid().player1().korean_name() == lineup.first_rapid().korean_name() &&
                                    r.second_blitz().player1().korean_name() == lineup.second_blitz().korean_name() &&
                                    r.third_blitz().player1().korean_name() == lineup.third_blitz().korean_name() &&
                                    r.forth_blitz().player1().korean_name() == lineup.forth_blitz().korean_name() &&
                                    (r.total_win_probability() - highest_min_total_win_prob).abs() < std::f64::EPSILON
                                );
                            }
                        }

                        println!("========================");
                        if let Some(best_result) = best_match_result {
                            let player1_best_tiebreaker_names: HashSet<String> = best_result.tiebreaker_relativities().iter()
                                .filter_map(|detail| detail.as_ref())
                                .map(|detail| detail.player1().korean_name().to_string())
                                .collect();
                            let player2_best_tiebreaker_names: HashSet<String> = best_result.tiebreaker_relativities().iter()
                                .filter_map(|detail| detail.as_ref())
                                .map(|detail| detail.player2().korean_name().to_string())
                                .collect();
                            println!("1국 장고(rapid): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", best_result.first_rapid().player1().korean_name(), best_result.first_rapid().player2().korean_name(), chrono::Utc::now().year() - 3, best_result.first_rapid().player1_wins(), best_result.first_rapid().player2_wins(), best_result.first_rapid_win_probability());
                            println!("2국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", best_result.second_blitz().player1().korean_name(), best_result.second_blitz().player2().korean_name(), chrono::Utc::now().year() - 3, best_result.second_blitz().player1_wins(), best_result.second_blitz().player2_wins(), best_result.second_blitz_win_probability());
                            println!("3국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", best_result.third_blitz().player1().korean_name(), best_result.third_blitz().player2().korean_name(), chrono::Utc::now().year() - 3, best_result.third_blitz().player1_wins(), best_result.third_blitz().player2_wins(), best_result.third_blitz_win_probability());
                            println!("4국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", best_result.forth_blitz().player1().korean_name(), best_result.forth_blitz().player2().korean_name(), chrono::Utc::now().year() - 3, best_result.forth_blitz().player1_wins(), best_result.forth_blitz().player2_wins(), best_result.forth_blitz_win_probability());
                            println!("\n4-0: {:.2}%", best_result.four_zero_probability());
                            println!("3-1: {:.2}%", best_result.three_one_probability());
                            println!("2-2: {:.2}% => ({}) vs ({}): {:.2}%", best_result.two_two_probability(), player1_best_tiebreaker_names.iter().cloned().collect::<Vec<_>>().join(", "), player2_best_tiebreaker_names.iter().cloned().collect::<Vec<_>>().join(", "), best_result.tiebreaker_win_probability());
                            println!("1-3: {:.2}%", best_result.one_three_probability());
                            println!("0-4: {:.2}%", best_result.zero_four_probability());
                            println!("\n총 승리확률: {:.2}%", best_result.total_win_probability());
                        } else {
                            println!("적합한 매치 결과를 찾을 수 없습니다.");
                        }
                        println!("========================");


                        println!("\n계속하려면 엔터를 누르세요.");
                        let mut pause = String::new();
                        io::stdin().read_line(&mut pause).expect("입력을 읽는 데 실패했습니다.");
                    },
                    "7" => {
                        let team1_filtered_lineups = utils::filter_team1_lineups(&selected_teams, &team1_all_lineups);

                        let unknown_player = Player::new("알 수 없음".to_string(), "unknown".to_string(), "未知".to_string(), NaiveDate::from_ymd_opt(2000, 1, 1).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0);

                        let mut team2_combination: Vec<&Player> = Vec::new();
                        println!("\n{} 팀의 스쿼드:", selected_teams[1].team_name());
                        let mut last_index = 0;
                        for (index, player) in selected_teams[1].players().iter().enumerate() {
                            println!("{}. {} (elo: {:.2}, 컨디션: {:.2}, 장고: {:.2}, 속기: {:.2}, 초속기: {:.2})", index + 1, player.korean_name(), player.elo_rating(), player.condition_weight(), player.rapid_weight(), player.blitz_weight(), player.bullet_weight());
                            last_index = index;
                        }
                        println!("{}. 알 수 없음", last_index + 2);
                        for i in 0..4 {
                            loop {
                                let mut input = String::new();
                                println!("\n{} 팀의 {}국 {} 기사 번호를 입력하세요:", selected_teams[1].team_name(), i + 1, if i == 0 { "장고(rapid)" } else { "속기(blitz)" });
                                io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
                                match input.trim().parse::<usize>() {
                                    Ok(num) if num > 0 && num <= selected_teams[1].players().len() => {
                                        team2_combination.push(&selected_teams[1].players()[num - 1]);
                                        break;
                                    },
                                    Ok(num) if num == selected_teams[1].players().len() + 1 => {
                                        team2_combination.push(&unknown_player);
                                        break;
                                    },
                                    _ => println!("잘못된 입력입니다. 다시 입력해주세요."),
                                }
                            }
                        }

                        let team2_unknown_player_count = team2_combination.iter().filter(|&player| player.english_name() == "unknown").count();
                        let team2_combinations = [36, 9, 4, 2, 1][team2_unknown_player_count.min(4)];

                        let mut avg_probabilities: Vec<(Lineup, f64, f64, f64, f64, f64, f64, f64, f64)> = Vec::new();
                        for lineup in &team1_filtered_lineups {
                            let mut total_win_prob = 0.0;
                            let mut three_one_prob = 0.0;
                            let mut four_zero_prob = 0.0;
                            let mut two_two_prob = 0.0;
                            let mut first_rapid_win_prob = 0.0;
                            let mut second_blitz_win_prob = 0.0;
                            let mut third_blitz_win_prob = 0.0;
                            let mut forth_blitz_win_prob = 0.0;
                            let mut count = 0.0;

                            for match_result in &match_results_matrix {
                                let filtered_results = match_result.iter().filter(|result| 
                                    team2_combination.iter().enumerate().all(|(index, player)| {
                                        match index {
                                            0 => player.english_name() == "unknown" || player.korean_name() == result.first_rapid().player2().korean_name(),
                                            1 => player.english_name() == "unknown" || player.korean_name() == result.second_blitz().player2().korean_name(),
                                            2 => player.english_name() == "unknown" || player.korean_name() == result.third_blitz().player2().korean_name(),
                                            3 => player.english_name() == "unknown" || player.korean_name() == result.forth_blitz().player2().korean_name(),
                                            _ => false,
                                        }
                                    })
                                );

                                for result in filtered_results.take(team2_combinations) {
                                    if result.first_rapid().player1().korean_name() == lineup.first_rapid().korean_name() &&
                                    result.second_blitz().player1().korean_name() == lineup.second_blitz().korean_name() &&
                                    result.third_blitz().player1().korean_name() == lineup.third_blitz().korean_name() &&
                                    result.forth_blitz().player1().korean_name() == lineup.forth_blitz().korean_name() {
                                        total_win_prob += result.total_win_probability();
                                        three_one_prob += result.four_zero_probability() + result.three_one_probability();
                                        four_zero_prob += result.four_zero_probability();
                                        two_two_prob += result.two_two_probability();
                                        first_rapid_win_prob += result.first_rapid_win_probability();
                                        second_blitz_win_prob += result.second_blitz_win_probability();
                                        third_blitz_win_prob += result.third_blitz_win_probability();
                                        forth_blitz_win_prob += result.forth_blitz_win_probability();
                                        count += 1.0;
                                    }
                                }
                            }

                            if count > 0.0 {
                                avg_probabilities.push((
                                    lineup.clone(),
                                    total_win_prob / count,
                                    three_one_prob / count,
                                    four_zero_prob / count,
                                    two_two_prob / count,
                                    first_rapid_win_prob / count,
                                    second_blitz_win_prob / count,
                                    third_blitz_win_prob / count,
                                    forth_blitz_win_prob / count
                                ));
                            }
                        }


                        println!("========================");
                        avg_probabilities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_four_zero_prob, avg_two_two_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 총 승리확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid().korean_name(), team2_combination[0].korean_name(), avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz().korean_name(), team2_combination[1].korean_name(), avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz().korean_name(), team2_combination[2].korean_name(), avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz().korean_name(), team2_combination[3].korean_name(), avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_four_zero_prob);
                            println!("평균 동점 확률: {:.2}%", avg_two_two_prob);
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
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_four_zero_prob, avg_two_two_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 동점 없는 승리확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid().korean_name(), team2_combination[0].korean_name(), avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz().korean_name(), team2_combination[1].korean_name(), avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz().korean_name(), team2_combination[2].korean_name(), avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz().korean_name(), team2_combination[3].korean_name(), avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_four_zero_prob);
                            println!("평균 동점 확률: {:.2}%", avg_two_two_prob);
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
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_four_zero_prob, avg_two_two_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 완봉승확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid().korean_name(), team2_combination[0].korean_name(), avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz().korean_name(), team2_combination[1].korean_name(), avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz().korean_name(), team2_combination[2].korean_name(), avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz().korean_name(), team2_combination[3].korean_name(), avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_four_zero_prob);
                            println!("평균 동점 확률: {:.2}%", avg_two_two_prob);
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
                        if let Some((best_lineup, avg_total_win_prob, avg_win_prob, avg_four_zero_prob, avg_two_two_prob, avg_first_rapid_win_prob, avg_second_blitz_win_prob, avg_third_blitz_win_prob, avg_forth_blitz_win_prob)) = avg_probabilities.first() {
                            println!("평균 동점 확률이 가장 높은 라인업");
                            println!("1국 장고(rapid): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.first_rapid().korean_name(), team2_combination[0].korean_name(), avg_first_rapid_win_prob);
                            println!("2국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.second_blitz().korean_name(), team2_combination[1].korean_name(), avg_second_blitz_win_prob);
                            println!("3국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.third_blitz().korean_name(), team2_combination[2].korean_name(), avg_third_blitz_win_prob);
                            println!("4국 속기(blitz): {} vs {} (평균 승리확률: {:.2}%)", best_lineup.forth_blitz().korean_name(), team2_combination[3].korean_name(), avg_forth_blitz_win_prob);
                            println!("\n평균 총 승리확률: {:.2}%", avg_total_win_prob);
                            println!("평균 동점 없는 승리확률: {:.2}%", avg_win_prob);
                            println!("평균 완봉승 확률: {:.2}%", avg_four_zero_prob);
                            println!("평균 동점 확률: {:.2}%", avg_two_two_prob);
                        } else {
                            println!("적합한 구성을 찾을 수 없습니다.");
                        }
                        println!("========================");

                        println!("\n계속하려면 엔터를 누르세요.");
                        let mut pause = String::new();
                        io::stdin().read_line(&mut pause).expect("입력을 읽는 데 실패했습니다.");
                    },
                    "8" => {
                        let team1_filtered_lineups = utils::filter_team1_lineups(&selected_teams, &team1_all_lineups);

                        let unknown_player = Player::new("알 수 없음".to_string(), "unknown".to_string(), "未知".to_string(), NaiveDate::from_ymd_opt(2000, 1, 1).expect("Invalid date"), 0.0, 0.0, 0.0, 0.0, 0.0);

                        let mut team2_combination: Vec<&Player> = Vec::new();
                        println!("\n{} 팀의 스쿼드:", selected_teams[1].team_name());
                        let mut last_index = 0;
                        for (index, player) in selected_teams[1].players().iter().enumerate() {
                            println!("{}. {} (elo: {:.2}, 컨디션: {:.2}, 장고: {:.2}, 속기: {:.2}, 초속기: {:.2})", index + 1, player.korean_name(), player.elo_rating(), player.condition_weight(), player.rapid_weight(), player.blitz_weight(), player.bullet_weight());
                            last_index = index;
                        }
                        println!("{}. 알 수 없음", last_index + 2);
                        for i in 0..4 {
                            loop {
                                let mut input = String::new();
                                println!("\n{} 팀의 {}국 {} 기사 번호를 입력하세요:", selected_teams[1].team_name(), i + 1, if i == 0 { "장고(rapid)" } else { "속기(blitz)" });
                                io::stdin().read_line(&mut input).expect("입력을 읽는 데 실패했습니다.");
                                match input.trim().parse::<usize>() {
                                    Ok(num) if num > 0 && num <= selected_teams[1].players().len() => {
                                        team2_combination.push(&selected_teams[1].players()[num - 1]);
                                        break;
                                    },
                                    Ok(num) if num == selected_teams[1].players().len() + 1 => {
                                        team2_combination.push(&unknown_player);
                                        break;
                                    },
                                    _ => println!("잘못된 입력입니다. 다시 입력해주세요."),
                                }
                            }
                        }

                        let mut best_match_result: Option<&MatchResult> = None;
                        let mut highest_min_total_win_prob = 0.0;

                        for lineup in &team1_filtered_lineups {
                            let mut min_total_win_prob = std::f64::MAX;
                            for match_result in &match_results_matrix {
                                let filtered_results = match_result.iter().filter(|result| 
                                    team2_combination.iter().enumerate().all(|(index, player)| {
                                        match index {
                                            0 => player.english_name() == "unknown" || player.korean_name() == result.first_rapid().player2().korean_name(),
                                            1 => player.english_name() == "unknown" || player.korean_name() == result.second_blitz().player2().korean_name(),
                                            2 => player.english_name() == "unknown" || player.korean_name() == result.third_blitz().player2().korean_name(),
                                            3 => player.english_name() == "unknown" || player.korean_name() == result.forth_blitz().player2().korean_name(),
                                            _ => false,
                                        }
                                    })
                                );
                                for result in filtered_results {
                                    if result.first_rapid().player1().korean_name() == lineup.first_rapid().korean_name() &&
                                    result.second_blitz().player1().korean_name() == lineup.second_blitz().korean_name() &&
                                    result.third_blitz().player1().korean_name() == lineup.third_blitz().korean_name() &&
                                    result.forth_blitz().player1().korean_name() == lineup.forth_blitz().korean_name() {
                                        if result.total_win_probability() < min_total_win_prob {
                                            min_total_win_prob = result.total_win_probability();
                                        }
                                    }
                                }
                            }

                            if min_total_win_prob != std::f64::MAX && min_total_win_prob > highest_min_total_win_prob {
                                highest_min_total_win_prob = min_total_win_prob;
                                best_match_result = match_results_matrix.iter().flatten().find(|&r| 
                                    r.first_rapid().player1().korean_name() == lineup.first_rapid().korean_name() &&
                                    r.second_blitz().player1().korean_name() == lineup.second_blitz().korean_name() &&
                                    r.third_blitz().player1().korean_name() == lineup.third_blitz().korean_name() &&
                                    r.forth_blitz().player1().korean_name() == lineup.forth_blitz().korean_name() &&
                                    (r.total_win_probability() - highest_min_total_win_prob).abs() < std::f64::EPSILON
                                );
                            }
                        }

                        println!("========================");
                        if let Some(best_result) = best_match_result {
                            let player1_best_tiebreaker_names: HashSet<String> = best_result.tiebreaker_relativities().iter()
                                .filter_map(|detail| detail.as_ref())
                                .map(|detail| detail.player1().korean_name().to_string())
                                .collect();
                            let player2_best_tiebreaker_names: HashSet<String> = best_result.tiebreaker_relativities().iter()
                                .filter_map(|detail| detail.as_ref())
                                .map(|detail| detail.player2().korean_name().to_string())
                                .collect();
                            println!("1국 장고(rapid): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", best_result.first_rapid().player1().korean_name(), best_result.first_rapid().player2().korean_name(), chrono::Utc::now().year() - 3, best_result.first_rapid().player1_wins(), best_result.first_rapid().player2_wins(), best_result.first_rapid_win_probability());
                            println!("2국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", best_result.second_blitz().player1().korean_name(), best_result.second_blitz().player2().korean_name(), chrono::Utc::now().year() - 3, best_result.second_blitz().player1_wins(), best_result.second_blitz().player2_wins(), best_result.second_blitz_win_probability());
                            println!("3국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", best_result.third_blitz().player1().korean_name(), best_result.third_blitz().player2().korean_name(), chrono::Utc::now().year() - 3, best_result.third_blitz().player1_wins(), best_result.third_blitz().player2_wins(), best_result.third_blitz_win_probability());
                            println!("4국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", best_result.forth_blitz().player1().korean_name(), best_result.forth_blitz().player2().korean_name(), chrono::Utc::now().year() - 3, best_result.forth_blitz().player1_wins(), best_result.forth_blitz().player2_wins(), best_result.forth_blitz_win_probability());
                            println!("\n4-0: {:.2}%", best_result.four_zero_probability());
                            println!("3-1: {:.2}%", best_result.three_one_probability());
                            println!("2-2: {:.2}% => ({}) vs ({}): {:.2}%", best_result.two_two_probability(), player1_best_tiebreaker_names.iter().cloned().collect::<Vec<_>>().join(", "), player2_best_tiebreaker_names.iter().cloned().collect::<Vec<_>>().join(", "), best_result.tiebreaker_win_probability());
                            println!("1-3: {:.2}%", best_result.one_three_probability());
                            println!("0-4: {:.2}%", best_result.zero_four_probability());
                            println!("\n총 승리확률: {:.2}%", best_result.total_win_probability());
                        } else {
                            println!("적합한 매치 결과를 찾을 수 없습니다.");
                        }
                        println!("========================");


                        println!("\n계속하려면 엔터를 누르세요.");
                        let mut pause = String::new();
                        io::stdin().read_line(&mut pause).expect("입력을 읽는 데 실패했습니다.");
                    },
                    "9" => {
                        let mut team1_best_lineup: Option<&MatchResult> = None;
                        let mut team2_best_lineup: Option<&MatchResult> = None;
                        let mut highest_min_total_win_prob = 0.0;
                        let mut lowest_max_total_win_prob = 100.0;
                        let mut team1_lowest_tiebreaker_prob = 100.0;
                        let mut team2_lowest_tiebreaker_prob = 100.0;
                        let mut team1_highest_perfect_prob = 0.0;
                        let mut team2_highest_perfect_prob = 0.0;

                        for lineup in &team1_all_lineups {
                            let mut min_total_win_prob = std::f64::MAX;

                            for match_result in &match_results_matrix {
                                for result in match_result.iter() {
                                    if result.first_rapid().player1().korean_name() == lineup.first_rapid().korean_name() &&
                                    result.second_blitz().player1().korean_name() == lineup.second_blitz().korean_name() &&
                                    result.third_blitz().player1().korean_name() == lineup.third_blitz().korean_name() &&
                                    result.forth_blitz().player1().korean_name() == lineup.forth_blitz().korean_name() {
                                        if result.total_win_probability() < min_total_win_prob || (
                                            result.total_win_probability() == min_total_win_prob &&
                                            result.two_two_probability() < team2_lowest_tiebreaker_prob
                                        ) || (
                                            result.total_win_probability() == min_total_win_prob &&
                                            result.two_two_probability() == team2_lowest_tiebreaker_prob &&
                                            result.four_zero_probability() < team2_highest_perfect_prob
                                        ) {
                                            min_total_win_prob = result.total_win_probability();
                                            team1_lowest_tiebreaker_prob = result.two_two_probability();
                                            team1_highest_perfect_prob = result.four_zero_probability();
                                        }
                                    }
                                }
                            }

                            if (min_total_win_prob != std::f64::MAX && min_total_win_prob > highest_min_total_win_prob) ||
                            (
                                min_total_win_prob == highest_min_total_win_prob &&
                                team1_lowest_tiebreaker_prob < team1_best_lineup.expect("REASON").two_two_probability()
                            ) || (
                                min_total_win_prob == highest_min_total_win_prob &&
                                team1_lowest_tiebreaker_prob == team1_best_lineup.expect("REASON").two_two_probability() &&
                                team1_highest_perfect_prob > team1_best_lineup.expect("REASON").four_zero_probability()
                            ) {
                                highest_min_total_win_prob = min_total_win_prob;
                                team1_best_lineup = match_results_matrix.iter().flatten().find(|&r| 
                                    r.first_rapid().player1().korean_name() == lineup.first_rapid().korean_name() &&
                                    r.second_blitz().player1().korean_name() == lineup.second_blitz().korean_name() &&
                                    r.third_blitz().player1().korean_name() == lineup.third_blitz().korean_name() &&
                                    r.forth_blitz().player1().korean_name() == lineup.forth_blitz().korean_name()
                                );
                            }
                        }

                        for lineup in &team1_all_lineups {
                            let mut max_total_win_prob = std::f64::MIN;

                            for match_result in &match_results_matrix {
                                for result in match_result.iter() {
                                    if result.first_rapid().player1().korean_name() == lineup.first_rapid().korean_name() &&
                                    result.second_blitz().player1().korean_name() == lineup.second_blitz().korean_name() &&
                                    result.third_blitz().player1().korean_name() == lineup.third_blitz().korean_name() &&
                                    result.forth_blitz().player1().korean_name() == lineup.forth_blitz().korean_name() {
                                        if result.total_win_probability() > max_total_win_prob || (
                                            result.total_win_probability() == max_total_win_prob &&
                                            result.two_two_probability() < team2_lowest_tiebreaker_prob
                                        ) || (
                                            result.total_win_probability() == max_total_win_prob &&
                                            result.two_two_probability() == team2_lowest_tiebreaker_prob &&
                                            result.four_zero_probability() == team2_highest_perfect_prob
                                        ) {
                                            max_total_win_prob = result.total_win_probability();
                                            team2_lowest_tiebreaker_prob = result.two_two_probability();
                                            team2_highest_perfect_prob = result.zero_four_probability();
                                        }
                                    }
                                }
                            }

                            if (max_total_win_prob != std::f64::MIN && max_total_win_prob < lowest_max_total_win_prob) || (
                                max_total_win_prob == lowest_max_total_win_prob &&
                                team2_lowest_tiebreaker_prob < team2_best_lineup.expect("REASON").two_two_probability()
                            ) || (
                                max_total_win_prob == lowest_max_total_win_prob &&
                                team2_lowest_tiebreaker_prob == team2_best_lineup.expect("REASON").two_two_probability() &&
                                team2_highest_perfect_prob > team2_best_lineup.expect("REASON").four_zero_probability()
                            ) {
                                lowest_max_total_win_prob = max_total_win_prob;
                                team2_best_lineup = match_results_matrix.iter().flatten().find(|&r| 
                                    r.first_rapid().player1().korean_name() == lineup.first_rapid().korean_name() &&
                                    r.second_blitz().player1().korean_name() == lineup.second_blitz().korean_name() &&
                                    r.third_blitz().player1().korean_name() == lineup.third_blitz().korean_name() &&
                                    r.forth_blitz().player1().korean_name() == lineup.forth_blitz().korean_name()
                                );
                            }
                        }

                        let best_match_result1 = match_results_matrix.iter().flatten().find(|&match_result| {
                            match (team1_best_lineup, team2_best_lineup) {
                                (Some(team1_lineup), Some(team2_lineup)) => {
                                    match_result.first_rapid().player1().korean_name() == team1_lineup.first_rapid().player1().korean_name() &&
                                    match_result.second_blitz().player1().korean_name() == team1_lineup.second_blitz().player1().korean_name() &&
                                    match_result.third_blitz().player1().korean_name() == team1_lineup.third_blitz().player1().korean_name() &&
                                    match_result.forth_blitz().player1().korean_name() == team1_lineup.forth_blitz().player1().korean_name() &&
                                    match_result.first_rapid().player2().korean_name() == team2_lineup.first_rapid().player2().korean_name() &&
                                    match_result.second_blitz().player2().korean_name() == team2_lineup.second_blitz().player2().korean_name() &&
                                    match_result.third_blitz().player2().korean_name() == team2_lineup.third_blitz().player2().korean_name() &&
                                    match_result.forth_blitz().player2().korean_name() == team2_lineup.forth_blitz().player2().korean_name()
                                },
                                _ => false,
                            }
                        });
                        let best_match_result2 = match_results_matrix.iter().flatten().find(|&match_result| {
                            match (team1_best_lineup, team2_best_lineup) {
                                (Some(team1_lineup), Some(team2_lineup)) => {
                                    match_result.first_rapid().player1().korean_name() == team1_lineup.first_rapid().player1().korean_name() &&
                                    match_result.second_blitz().player1().korean_name() == team1_lineup.second_blitz().player1().korean_name() &&
                                    match_result.third_blitz().player1().korean_name() == team1_lineup.third_blitz().player1().korean_name() &&
                                    match_result.forth_blitz().player1().korean_name() == team1_lineup.forth_blitz().player1().korean_name() &&
                                    match_result.first_rapid().player2().korean_name() == team2_lineup.first_rapid().player2().korean_name() &&
                                    match_result.second_blitz().player2().korean_name() == team2_lineup.second_blitz().player2().korean_name() &&
                                    match_result.third_blitz().player2().korean_name() == team2_lineup.forth_blitz().player2().korean_name() &&
                                    match_result.forth_blitz().player2().korean_name() == team2_lineup.third_blitz().player2().korean_name()
                                },
                                _ => false,
                            }
                        });

                        println!("========================");
                        if let Some(best_result) = best_match_result1 {
                            let player1_best_tiebreaker_names: HashSet<String> = best_result.tiebreaker_relativities().iter()
                                .filter_map(|detail| detail.as_ref())
                                .map(|detail| detail.player1().korean_name().to_string())
                                .collect();
                            let player2_best_tiebreaker_names: HashSet<String> = best_result.tiebreaker_relativities().iter()
                                .filter_map(|detail| detail.as_ref())
                                .map(|detail| detail.player2().korean_name().to_string())
                                .collect();
                            println!("1국 장고(rapid): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", best_result.first_rapid().player1().korean_name(), best_result.first_rapid().player2().korean_name(), chrono::Utc::now().year() - 3, best_result.first_rapid().player1_wins(), best_result.first_rapid().player2_wins(), best_result.first_rapid_win_probability());
                            println!("2국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", best_result.second_blitz().player1().korean_name(), best_result.second_blitz().player2().korean_name(), chrono::Utc::now().year() - 3, best_result.second_blitz().player1_wins(), best_result.second_blitz().player2_wins(), best_result.second_blitz_win_probability());
                            println!("3국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", best_result.third_blitz().player1().korean_name(), best_result.third_blitz().player2().korean_name(), chrono::Utc::now().year() - 3, best_result.third_blitz().player1_wins(), best_result.third_blitz().player2_wins(), best_result.third_blitz_win_probability());
                            println!("4국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", best_result.forth_blitz().player1().korean_name(), best_result.forth_blitz().player2().korean_name(), chrono::Utc::now().year() - 3, best_result.forth_blitz().player1_wins(), best_result.forth_blitz().player2_wins(), best_result.forth_blitz_win_probability());
                            println!("\n4-0: {:.2}%", best_result.four_zero_probability());
                            println!("3-1: {:.2}%", best_result.three_one_probability());
                            println!("2-2: {:.2}% => ({}) vs ({}): {:.2}%", best_result.two_two_probability(), player1_best_tiebreaker_names.iter().cloned().collect::<Vec<_>>().join(", "), player2_best_tiebreaker_names.iter().cloned().collect::<Vec<_>>().join(", "), best_result.tiebreaker_win_probability());
                            println!("1-3: {:.2}%", best_result.one_three_probability());
                            println!("0-4: {:.2}%", best_result.zero_four_probability());
                            println!("\n총 승리확률: {:.2}%", best_result.total_win_probability());
                        } else {
                            println!("적합한 매치 결과를 찾을 수 없습니다.");
                        }
                        if let Some(best_result) = best_match_result2 {
                            let player1_best_tiebreaker_names: HashSet<String> = best_result.tiebreaker_relativities().iter()
                                .filter_map(|detail| detail.as_ref())
                                .map(|detail| detail.player1().korean_name().to_string())
                                .collect();
                            let player2_best_tiebreaker_names: HashSet<String> = best_result.tiebreaker_relativities().iter()
                                .filter_map(|detail| detail.as_ref())
                                .map(|detail| detail.player2().korean_name().to_string())
                                .collect();
                            println!("1국 장고(rapid): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", best_result.first_rapid().player1().korean_name(), best_result.first_rapid().player2().korean_name(), chrono::Utc::now().year() - 3, best_result.first_rapid().player1_wins(), best_result.first_rapid().player2_wins(), best_result.first_rapid_win_probability());
                            println!("2국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", best_result.second_blitz().player1().korean_name(), best_result.second_blitz().player2().korean_name(), chrono::Utc::now().year() - 3, best_result.second_blitz().player1_wins(), best_result.second_blitz().player2_wins(), best_result.second_blitz_win_probability());
                            println!("3국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", best_result.third_blitz().player1().korean_name(), best_result.third_blitz().player2().korean_name(), chrono::Utc::now().year() - 3, best_result.third_blitz().player1_wins(), best_result.third_blitz().player2_wins(), best_result.third_blitz_win_probability());
                            println!("4국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", best_result.forth_blitz().player1().korean_name(), best_result.forth_blitz().player2().korean_name(), chrono::Utc::now().year() - 3, best_result.forth_blitz().player1_wins(), best_result.forth_blitz().player2_wins(), best_result.forth_blitz_win_probability());
                            println!("\n4-0: {:.2}%", best_result.four_zero_probability());
                            println!("3-1: {:.2}%", best_result.three_one_probability());
                            println!("2-2: {:.2}% => ({}) vs ({}): {:.2}%", best_result.two_two_probability(), player1_best_tiebreaker_names.iter().cloned().collect::<Vec<_>>().join(", "), player2_best_tiebreaker_names.iter().cloned().collect::<Vec<_>>().join(", "), best_result.tiebreaker_win_probability());
                            println!("1-3: {:.2}%", best_result.one_three_probability());
                            println!("0-4: {:.2}%", best_result.zero_four_probability());
                            println!("\n총 승리확률: {:.2}%", best_result.total_win_probability());
                        } else {
                            println!("적합한 매치 결과를 찾을 수 없습니다.");
                        }
                        println!("========================");


                        println!("\n계속하려면 엔터를 누르세요.");
                        let mut pause = String::new();
                        io::stdin().read_line(&mut pause).expect("입력을 읽는 데 실패했습니다.");
                    },
                    "10" => {
                        let team1_combination = utils::select_team_combination(&selected_teams[0]);
                        let team2_combination = utils::select_team_combination(&selected_teams[1]);

                        let match_result = match_results_matrix.iter().flatten().find(|&result| {
                            result.first_rapid().player1().korean_name() == team1_combination[0].korean_name() && 
                            result.second_blitz().player1().korean_name() == team1_combination[1].korean_name() && 
                            result.third_blitz().player1().korean_name() == team1_combination[2].korean_name() && 
                            result.forth_blitz().player1().korean_name() == team1_combination[3].korean_name() && 
                            result.first_rapid().player2().korean_name() == team2_combination[0].korean_name() && 
                            result.second_blitz().player2().korean_name() == team2_combination[1].korean_name() && 
                            result.third_blitz().player2().korean_name() == team2_combination[2].korean_name() && 
                            result.forth_blitz().player2().korean_name() == team2_combination[3].korean_name()
                        }).expect("매치 결과를 찾을 수 없습니다.");

                        let player1_best_tiebreaker_names: HashSet<String> = match_result.tiebreaker_relativities().iter()
                            .filter_map(|detail| detail.as_ref())
                            .map(|detail| detail.player1().korean_name().to_string())
                            .collect();
                        let player2_best_tiebreaker_names: HashSet<String> = match_result.tiebreaker_relativities().iter()
                            .filter_map(|detail| detail.as_ref())
                            .map(|detail| detail.player2().korean_name().to_string())
                            .collect();

                        println!("========================");
                        println!("1국 장고(rapid): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", match_result.first_rapid().player1().korean_name(), match_result.first_rapid().player2().korean_name(), chrono::Utc::now().year() - 3, match_result.first_rapid().player1_wins(), match_result.first_rapid().player2_wins(), match_result.first_rapid_win_probability());
                        println!("2국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", match_result.second_blitz().player1().korean_name(), match_result.second_blitz().player2().korean_name(), chrono::Utc::now().year() - 3, match_result.second_blitz().player1_wins(), match_result.second_blitz().player2_wins(), match_result.second_blitz_win_probability());
                        println!("3국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", match_result.third_blitz().player1().korean_name(), match_result.third_blitz().player2().korean_name(), chrono::Utc::now().year() - 3, match_result.third_blitz().player1_wins(), match_result.third_blitz().player2_wins(), match_result.third_blitz_win_probability());
                        println!("4국 속기(blitz): {} vs {} ({}~ 상대전적: {}-{}) (승리확률: {:.2}%)", match_result.forth_blitz().player1().korean_name(), match_result.forth_blitz().player2().korean_name(), chrono::Utc::now().year() - 3, match_result.forth_blitz().player1_wins(), match_result.forth_blitz().player2_wins(), match_result.forth_blitz_win_probability());
                        println!("\n4-0: {:.2}%", match_result.four_zero_probability());
                        println!("3-1: {:.2}%", match_result.three_one_probability());
                        println!("2-2: {:.2}% => ({}) vs ({}): {:.2}%", match_result.two_two_probability(), player1_best_tiebreaker_names.iter().cloned().collect::<Vec<_>>().join(", "), player2_best_tiebreaker_names.iter().cloned().collect::<Vec<_>>().join(", "), match_result.tiebreaker_win_probability());
                        println!("1-3: {:.2}%", match_result.one_three_probability());
                        println!("0-4: {:.2}%", match_result.zero_four_probability());
                        println!("\n총 승리확률: {:.2}%", match_result.total_win_probability());
                        println!("========================");

                        println!("\n계속하려면 엔터를 누르세요.");
                        let mut pause = String::new();
                        io::stdin().read_line(&mut pause).expect("입력을 읽는 데 실패했습니다.");
                    },
                    "11" => {
                        let team1_combination = utils::select_team_combination(&selected_teams[0]);
                        let team2_combination = utils::select_team_combination(&selected_teams[1]);

                        let outcomes = ["WWLL", "WLWL", "WLLW", "LWWL", "LWLW", "LLWW"];
                        let mut outcome_map: HashMap<&str, Vec<PlayerRelativity>> = HashMap::new();

                        for &outcome in outcomes.iter() {
                            let defeated_players: Vec<&Player> = outcome.chars().enumerate().map(|(i, result)| {
                                match result {
                                    'W' => team2_combination[i], // 승리는 상대 팀의 기사가 패배
                                    'L' => team1_combination[i], // 패배는 자신의 팀의 기사가 패배
                                    _ => unreachable!(),
                                }
                            }).collect();

                            let tiebreaker_relativities = player_relativities.iter()
                                .map(|relativity| {
                                    let player1_position = team1_combination.iter().position(|p| p.korean_name() == relativity.player1().korean_name());
                                    let player2_position = team2_combination.iter().position(|p| p.korean_name() == relativity.player2().korean_name());
                                    let player1_penalty = if let Some(pos) = player1_position {
                                        let base_penalty = match pos {
                                            0 => 1.0 / 1.04,
                                            1 => 1.0 / 1.02,
                                            _ => 1.0 / 1.08,
                                        };
                                        if defeated_players.contains(&team1_combination[pos]) {
                                            base_penalty * (1.0 / 1.10)
                                        } else {
                                            base_penalty
                                        }
                                    } else {
                                        1.0
                                    };

                                    let player2_penalty = if let Some(pos) = player2_position {
                                        let base_penalty = match pos {
                                            0 => 1.04,
                                            1 => 1.02,
                                            _ => 1.08,
                                        };
                                        if defeated_players.contains(&team2_combination[pos]) {
                                            base_penalty * 1.10
                                        } else {
                                            base_penalty
                                        }
                                    } else {
                                        1.0
                                    };

                                    let modified_bullet_win_probability = relativity.bullet_win_probability() * player1_penalty * player2_penalty;
                                    PlayerRelativity::new(
                                        relativity.player1().clone(),
                                        relativity.player2().clone(),
                                        relativity.player1_wins(),
                                        relativity.player2_wins(),
                                        relativity.elo_win_probability(),
                                        relativity.condition_win_probability(),
                                        relativity.rapid_win_probability(),
                                        relativity.blitz_win_probability(),
                                        modified_bullet_win_probability,
                                    )
                                })
                                .collect::<Vec<PlayerRelativity>>();

                            outcome_map.insert(outcome, tiebreaker_relativities);
                        }

                        match utils::create_excel_from_tiebreaker_relativities(outcome_map.clone()) {
                            Ok(_) => println!("Excel 파일이 성공적으로 생성되었습니다."),
                            Err(e) => println!("Excel 파일 생성 중 오류가 발생했습니다: {}", e),
                        }
                    },
                    "12" => {
                        let team1_combination = utils::select_team_combination(&selected_teams[0]);
                        let team2_combination = utils::select_team_combination(&selected_teams[1]);

                        let match_result = match_results_matrix.iter().flatten().find(|&result| {
                            result.first_rapid().player1().korean_name() == team1_combination[0].korean_name() && 
                            result.second_blitz().player1().korean_name() == team1_combination[1].korean_name() && 
                            result.third_blitz().player1().korean_name() == team1_combination[2].korean_name() && 
                            result.forth_blitz().player1().korean_name() == team1_combination[3].korean_name() && 
                            result.first_rapid().player2().korean_name() == team2_combination[0].korean_name() && 
                            result.second_blitz().player2().korean_name() == team2_combination[1].korean_name() && 
                            result.third_blitz().player2().korean_name() == team2_combination[2].korean_name() && 
                            result.forth_blitz().player2().korean_name() == team2_combination[3].korean_name()
                        }).expect("매치 결과를 찾을 수 없습니다.");

                        let rt = tokio::runtime::Builder::new_multi_thread()
                            .enable_all()
                            .build()
                            .unwrap();

                        rt.block_on(async {
                            utils::live_win_ratings(match_result.clone(), player_relativities.clone()).await;
                        });
                    },
                    "exit" => break,
                    _ => println!("잘못된 입력입니다. 다시 입력해주세요."),
                }
            }
        },
        Err(e) => println!("상대전적을 생성하는 동안 오류가 발생했습니다: {}", e),
    }
}

pub fn execute_kbleague_power_ranking() {
    let teams = init_teams();
    let mut team_relativities_matrix: Vec<Vec<TeamRelativity>> = Vec::new();

    for (index1, team1) in teams.iter().enumerate() {
        let mut row: Vec<TeamRelativity> = Vec::new();
        for (index2, team2) in teams.iter().enumerate() {
            if team1.team_name() == team2.team_name() { continue; }
            let mut selected_teams: Vec<Team> = Vec::new();
            selected_teams.push(teams[index1].clone());
            selected_teams.push(teams[index2].clone());
            println!("\n{} vs {}", team1.team_name(), team2.team_name());
            println!("ELO 레이팅을 업데이트 중...");
            if let Err(e) = utils::update_team_elo_ratings(&mut selected_teams) {
                println!("ELO 레이팅을 업데이트하는 동안 오류가 발생했습니다: {}", e);
                return;
            }

            let mut team1_all_lineups: Vec<Lineup> = Vec::new();
            let mut team2_all_lineups: Vec<Lineup> = Vec::new();
            for team_index in 0..2 {
                let team_players = selected_teams[team_index].players();
                let all_lineups = if team_index == 0 { &mut team1_all_lineups } else { &mut team2_all_lineups };

                for first_rapid in team_players {
                    for second_blitz in team_players {
                        if second_blitz == first_rapid { continue; }
                        for third_blitz in team_players {
                            if third_blitz == first_rapid || third_blitz == second_blitz { continue; }
                            for forth_blitz in team_players {
                                if forth_blitz == first_rapid || forth_blitz == second_blitz || forth_blitz == third_blitz { continue; }
                                all_lineups.push(Lineup::new(first_rapid.clone(), second_blitz.clone(), third_blitz.clone(), forth_blitz.clone()));
                            }
                        }
                    }
                }
            }

            println!("상대전적을 업데이트 중...");
            match utils::generate_player_relativities(&selected_teams) {
                Ok(player_relativities) => {
                    println!("라인업 메트릭스 생성 중...");
                    let mut match_results_matrix: Vec<Vec<MatchResult>> = Vec::new();
                    let mut team1_lineups_with_avg: Vec<(Lineup, f64)> = team1_all_lineups.iter().map(|lineup| {
                        let avg_total_win_probability: f64 = team2_all_lineups.iter().map(|opponent_lineup| {
                            let match_result = utils::calculate_match_result(lineup.clone(), opponent_lineup.clone(), player_relativities.clone());
                            match_result.total_win_probability()
                        }).sum::<f64>() / team2_all_lineups.len() as f64;
                        (lineup.clone(), avg_total_win_probability)
                    }).collect();

                    let mut team2_lineups_with_avg: Vec<(Lineup, f64)> = team2_all_lineups.iter().map(|lineup| {
                        let avg_total_win_probability: f64 = team1_all_lineups.iter().map(|opponent_lineup| {
                            let match_result = utils::calculate_match_result(opponent_lineup.clone(), lineup.clone(), player_relativities.clone());
                            match_result.total_win_probability()
                        }).sum::<f64>() / team1_all_lineups.len() as f64;
                        (lineup.clone(), avg_total_win_probability)
                    }).collect();

                    team1_lineups_with_avg.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                    team2_lineups_with_avg.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

                    for (team1_lineup, _) in team1_lineups_with_avg {
                        let mut row: Vec<MatchResult> = Vec::new();
                        for (team2_lineup, _) in &team2_lineups_with_avg {
                            let match_result = utils::calculate_match_result(team1_lineup.clone(), team2_lineup.clone(), player_relativities.clone());
                            row.push(match_result);
                        }
                        match_results_matrix.push(row);
                    }

                    let mut team1_best_lineup: Option<&MatchResult> = None;
                    let mut team2_best_lineup: Option<&MatchResult> = None;
                    let mut highest_min_total_win_prob = 0.0;
                    let mut lowest_max_total_win_prob = 100.0;
                    let mut team1_lowest_tiebreaker_prob = 100.0;
                    let mut team2_lowest_tiebreaker_prob = 100.0;
                    let mut team1_highest_perfect_prob = 0.0;
                    let mut team2_highest_perfect_prob = 0.0;

                    for lineup in &team1_all_lineups {
                        let mut min_total_win_prob = std::f64::MAX;

                        for match_result in &match_results_matrix {
                            for result in match_result.iter() {
                                if result.first_rapid().player1().korean_name() == lineup.first_rapid().korean_name() &&
                                result.second_blitz().player1().korean_name() == lineup.second_blitz().korean_name() &&
                                result.third_blitz().player1().korean_name() == lineup.third_blitz().korean_name() &&
                                result.forth_blitz().player1().korean_name() == lineup.forth_blitz().korean_name() {
                                    if result.total_win_probability() < min_total_win_prob || (
                                        result.total_win_probability() == min_total_win_prob &&
                                        result.two_two_probability() < team2_lowest_tiebreaker_prob
                                    ) || (
                                        result.total_win_probability() == min_total_win_prob &&
                                        result.two_two_probability() == team2_lowest_tiebreaker_prob &&
                                        result.four_zero_probability() < team2_highest_perfect_prob
                                    ) {
                                        min_total_win_prob = result.total_win_probability();
                                        team1_lowest_tiebreaker_prob = result.two_two_probability();
                                        team1_highest_perfect_prob = result.four_zero_probability();
                                    }
                                }
                            }
                        }

                        if (min_total_win_prob != std::f64::MAX && min_total_win_prob > highest_min_total_win_prob) ||
                        (
                            min_total_win_prob == highest_min_total_win_prob &&
                            team1_lowest_tiebreaker_prob < team1_best_lineup.expect("REASON").two_two_probability()
                        ) || (
                            min_total_win_prob == highest_min_total_win_prob &&
                            team1_lowest_tiebreaker_prob == team1_best_lineup.expect("REASON").two_two_probability() &&
                            team1_highest_perfect_prob > team1_best_lineup.expect("REASON").four_zero_probability()
                        ) {
                            highest_min_total_win_prob = min_total_win_prob;
                            team1_best_lineup = match_results_matrix.iter().flatten().find(|&r| 
                                r.first_rapid().player1().korean_name() == lineup.first_rapid().korean_name() &&
                                r.second_blitz().player1().korean_name() == lineup.second_blitz().korean_name() &&
                                r.third_blitz().player1().korean_name() == lineup.third_blitz().korean_name() &&
                                r.forth_blitz().player1().korean_name() == lineup.forth_blitz().korean_name()
                            );
                        }
                    }

                    for lineup in &team1_all_lineups {
                        let mut max_total_win_prob = std::f64::MIN;

                        for match_result in &match_results_matrix {
                            for result in match_result.iter() {
                                if result.first_rapid().player1().korean_name() == lineup.first_rapid().korean_name() &&
                                result.second_blitz().player1().korean_name() == lineup.second_blitz().korean_name() &&
                                result.third_blitz().player1().korean_name() == lineup.third_blitz().korean_name() &&
                                result.forth_blitz().player1().korean_name() == lineup.forth_blitz().korean_name() {
                                    if result.total_win_probability() > max_total_win_prob || (
                                        result.total_win_probability() == max_total_win_prob &&
                                        result.two_two_probability() < team2_lowest_tiebreaker_prob
                                    ) || (
                                        result.total_win_probability() == max_total_win_prob &&
                                        result.two_two_probability() == team2_lowest_tiebreaker_prob &&
                                        result.four_zero_probability() == team2_highest_perfect_prob
                                    ) {
                                        max_total_win_prob = result.total_win_probability();
                                        team2_lowest_tiebreaker_prob = result.two_two_probability();
                                        team2_highest_perfect_prob = result.zero_four_probability();
                                    }
                                }
                            }
                        }

                        if (max_total_win_prob != std::f64::MIN && max_total_win_prob < lowest_max_total_win_prob) || (
                            max_total_win_prob == lowest_max_total_win_prob &&
                            team2_lowest_tiebreaker_prob < team2_best_lineup.expect("REASON").two_two_probability()
                        ) || (
                            max_total_win_prob == lowest_max_total_win_prob &&
                            team2_lowest_tiebreaker_prob == team2_best_lineup.expect("REASON").two_two_probability() &&
                            team2_highest_perfect_prob > team2_best_lineup.expect("REASON").four_zero_probability()
                        ) {
                            lowest_max_total_win_prob = max_total_win_prob;
                            team2_best_lineup = match_results_matrix.iter().flatten().find(|&r| 
                                r.first_rapid().player1().korean_name() == lineup.first_rapid().korean_name() &&
                                r.second_blitz().player1().korean_name() == lineup.second_blitz().korean_name() &&
                                r.third_blitz().player1().korean_name() == lineup.third_blitz().korean_name() &&
                                r.forth_blitz().player1().korean_name() == lineup.forth_blitz().korean_name()
                            );
                        }
                    }

                    let best_match_result1 = match_results_matrix.iter().flatten().find(|&match_result| {
                        match (team1_best_lineup, team2_best_lineup) {
                            (Some(team1_lineup), Some(team2_lineup)) => {
                                match_result.first_rapid().player1().korean_name() == team1_lineup.first_rapid().player1().korean_name() &&
                                match_result.second_blitz().player1().korean_name() == team1_lineup.second_blitz().player1().korean_name() &&
                                match_result.third_blitz().player1().korean_name() == team1_lineup.third_blitz().player1().korean_name() &&
                                match_result.forth_blitz().player1().korean_name() == team1_lineup.forth_blitz().player1().korean_name() &&
                                match_result.first_rapid().player2().korean_name() == team2_lineup.first_rapid().player2().korean_name() &&
                                match_result.second_blitz().player2().korean_name() == team2_lineup.second_blitz().player2().korean_name() &&
                                match_result.third_blitz().player2().korean_name() == team2_lineup.third_blitz().player2().korean_name() &&
                                match_result.forth_blitz().player2().korean_name() == team2_lineup.forth_blitz().player2().korean_name()
                            },
                            _ => false,
                        }
                    });

                    let best_match_result2 = match_results_matrix.iter().flatten().find(|&match_result| {
                        match (team1_best_lineup, team2_best_lineup) {
                            (Some(team1_lineup), Some(team2_lineup)) => {
                                match_result.first_rapid().player1().korean_name() == team1_lineup.first_rapid().player1().korean_name() &&
                                match_result.second_blitz().player1().korean_name() == team1_lineup.second_blitz().player1().korean_name() &&
                                match_result.third_blitz().player1().korean_name() == team1_lineup.third_blitz().player1().korean_name() &&
                                match_result.forth_blitz().player1().korean_name() == team1_lineup.forth_blitz().player1().korean_name() &&
                                match_result.first_rapid().player2().korean_name() == team2_lineup.first_rapid().player2().korean_name() &&
                                match_result.second_blitz().player2().korean_name() == team2_lineup.second_blitz().player2().korean_name() &&
                                match_result.third_blitz().player2().korean_name() == team2_lineup.forth_blitz().player2().korean_name() &&
                                match_result.forth_blitz().player2().korean_name() == team2_lineup.third_blitz().player2().korean_name()
                            },
                            _ => false,
                        }
                    });

                    if let Some(best_result1) = best_match_result1 {
                        if let Some(best_result2) = best_match_result2 {
                            println!("총 승리확률: {:.2}%", (best_result1.total_win_probability() + best_result2.total_win_probability()) / 2.0);
                            row.push(TeamRelativity::new(
                                selected_teams[0].clone(),
                                selected_teams[1].clone(),
                                (best_result1.total_win_probability() + best_result2.total_win_probability()) / 2.0
                            ));
                        } else {
                            println!("적합한 매치 결과를 찾을 수 없습니다.");
                        }
                    } else {
                        println!("적합한 매치 결과를 찾을 수 없습니다.");
                    }
                },
                Err(e) => println!("상대전적을 생성하는 동안 오류가 발생했습니다: {}", e),
            }
        }
        team_relativities_matrix.push(row);
    }

    let _ = utils::create_excel_from_team(team_relativities_matrix);
}