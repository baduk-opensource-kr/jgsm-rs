# Joint Go Strategy Matrix(공동의 바둑 전략 메트릭스) - JGSM
![전기새마](https://github.com/baduk-opensource-kr/jgsm-rs/assets/36529903/f2b45d6e-b9c1-4812-a7c8-5835ae8ae312)  
Joint Go Strategy Matrix(공동의 바둑 전략 메트릭스) - JGSM: 전기새마

바둑리그등의 팀 게임에서 승부를 예측하고 최선의 오더를 산출하는 메트릭스를 출력합니다.

## 다른 언어로 보기
[**한국어**](./README.md), [日本語](./README.jp.md), [English](./README.en.md)

# 릴리즈 실행하기(일반 사용자)
[release](https://github.com/baduk-opensource-kr/jgsm-rs/releases/latest).

[geckodriver](https://github.com/mozilla/geckodriver/releases).

실시간 승부예측을 실행하려면 [geckodriver](https://github.com/mozilla/geckodriver/releases)를 다운받아 실행하여야 합니다.

[release](https://github.com/baduk-opensource-kr/jgsm-rs/releases/latest) 링크를 클릭하면 전기새마의 실행파일을 설치할 수 있습니다.

# 소스코드를 빌드하여 실행하기(개발자)
```
git clone https://github.com/baduk-opensource-kr/jgsm-rs.git
cd jgsm-rs
cargo build
cargo run
```

# 기타
- [x] goratings ELO와 배태일 ELO사이의 환산로직
- [x] 양팀의 라인업 메트릭스를 Excel로 출력
- [x] 지정 라인업에 대한 승리확률 출력
- [x] 평균 베스트 라인업 출력
- [x] 상대팀 베스트24 라인업(상대팀의 베스트멤버)의 평균 베스트 라인업 출력
- [x] 카운터픽 면역 라인업 출력(카운터픽: 해당 라인업의 가장 승리확률이 낮은 라인업, 카운터픽이 걸렸을 시 승리확률이 가장 높은 라인업)
- [x] 상대팀 예상 라인업의 카운터픽 출력
- [x] 에이스결정전 예측승률 출력
- [x] 실시간 팀 승부예측 출력
- [ ] 포스트시즌 룰에 적용하여 대한 실시간 라인업 출력
- [ ] 레전드리그, 시니어바둑리그, 여자바둑리그, 중국갑조리그, 농심배의 베스트 라인업 및 승리확률 출력

# License
이 프로그램은 다음 라이선스를 따르고 있습니다. [MPL2.0 license](/LICENSE) 
