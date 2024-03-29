mod kbleague;
mod models;
mod utils;

use std::io;

fn main() {
    loop {
        println!("1. KB바둑리그");
        println!("2. KB바둑리그 파워랭킹");
        println!("3. 여자바둑리그(개발중)");
        println!("4. 여자바둑리그 파워랭킹(개발중)");
        println!("5. 시니어바둑리그(개발중)");
        println!("6. 시니어바둑리그 파워랭킹(개발중)");
        println!("exit. 종료");

        let mut option = String::new();
        io::stdin().read_line(&mut option).expect("입력을 읽는 데 실패했습니다.");
        let option = option.trim();

        match option {
            "1" => {
                kbleague::execute_kbleague();
            },
            "2" => {
                kbleague::execute_kbleague_power_ranking();
            },
            "exit" => break,
            _ => println!("잘못된 입력입니다. 다시 입력해주세요."),
        }
    }
}
