# Joint Go Strategy Matrix(共同の囲碁戦略マトリックス) - JGSM
![田忌賽馬](https://github.com/baduk-opensource-kr/jgsm-rs/assets/36529903/f2b45d6e-b9c1-4812-a7c8-5835ae8ae312)  
Joint Go Strategy Matrix(共同の囲碁戦略マトリックス) - JGSM: 田忌賽馬

KB囲碁リーグなどのチームゲームで勝敗を予測し、最善のオーダーを算出するマトリックスを出力します。

## 他の言語で見る
[한국어](./README.md), [**日本語**](./README.ja.md)

# リリースを実行する（一般ユーザー向け）
[release](https://github.com/baduk-opensource-kr/jgsm-rs/releases/tag/release).
リンクをクリックして、実行ファイルをダウンロードすれば使用できます。

# ソースコードをビルドして実行する（開発者）
Install dependencies
```
git clone https://github.com/baduk-opensource-kr/jgsm-rs.git
cd jgsm-rs
cargo build
cargo run
```

# その他
- [x] goratings ELOとベテイルELO間の換算ロジック
- [x] 両チームのラインアップメトリクスをExcelに出力
- [x] 指定ラインアップに対する勝利確率の出力
- [x] 平均ベストラインアップの出力
- [x] 対戦チームベスト24ラインアップ(対戦チームのベストメンバー)の平均ベストラインアップの出力
- [x] カウンターピック免疫ラインアップの出力(カウンターピック：該当ラインアップの最も勝利確率が低いラインアップ、カウンターピックがかかった場合の最も勝利確率が高いラインアップ)
- [x] 対戦チーム予想ラインアップのカウンターピックの出力
- [ ] エース決定戦予測勝率の出力
- [ ] ポストシーズンルールを適用したリアルタイムラインアップの出力
- [ ] レジェンドリーグ、シニア囲碁リーグ、女子囲碁リーグ、中国甲級リーグ、農心杯のベストラインアップ及び勝利確率の出力

# License
このプログラムは以下のライセンスに従います。 [MPL2.0 license](/LICENSE) 
