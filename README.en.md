# Joint Go Strategy Matrix - JGSM
![JGSM](https://github.com/baduk-opensource-kr/jgsm-rs/assets/36529903/f2b45d6e-b9c1-4812-a7c8-5835ae8ae312)  
Joint Go Strategy Matrix - JGSM

Outputs a matrix that predicts the outcome of team games such as Go leagues and calculates the best order.

## View in Other Languages
[한국어](./README.md), [日本語](./README.jp.md), [**English**](./README.en.md)

# Running the Release (For General Users)
[release](https://github.com/baduk-opensource-kr/jgsm-rs/releases/latest).

[geckodriver](https://github.com/mozilla/geckodriver/releases).

To run real-time match predictions, you need to download and run [geckodriver](https://github.com/mozilla/geckodriver/releases).

Clicking the [release](https://github.com/baduk-opensource-kr/jgsm-rs/releases/latest) link will allow you to install the executable file for JGSM.

# Building and Running from Source (For Developers)
```
git clone https://github.com/baduk-opensource-kr/jgsm-rs.git
cd jgsm-rs
cargo build
cargo run
```

# etc
- [x] Conversion logic between goratings ELO and Baetaeil ELO
- [x] Output of both teams' lineup matrix to Excel
- [x] Output of win probability for specified lineup
- [x] Output of average best lineup
- [x] Output of average best lineup for the best 24 lineups of the opposing team (best members of the opposing team)
- [x] Output of counterpick immune lineup (Counterpick: the lineup with the lowest win probability, the lineup with the highest win probability when counterpicked)
- [x] Output of counterpick for the expected lineup of the opposing team
- [x] Output of predicted win rate for ace deciding matches
- [x] Output of real-time team win prediction
- [ ] Output of real-time lineup applied with postseason rules
- [ ] Output of best lineup and win probability for the Legend League, Senior Baduk League, Women's Baduk League, Chinese First Division League, and Nongshim Cup

# License
This program is licensed under the following license: [MPL2.0 license](/LICENSE) 


