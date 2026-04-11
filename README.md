# Rust Practice

このリポジトリは、Rust を 20 日間で基礎から実践まで学ぶための学習用ワークスペースです。

## このリポジトリでやること

- Rust の構文、所有権、借用、`struct`、`enum` を学ぶ
- `cargo` を使った開発フローに慣れる
- テスト、リファクタリング、エラーハンドリングを学ぶ
- GitHub を使ったモダンな開発の流れを練習する
- 最後に小さな CLI プロダクトを作る

## 学習プラン

20日間の詳細な学習内容は以下を参照してください。

- [study-plan-20days.md](C:/dev/rust_practice/study-plan-20days.md)

## ディレクトリ構成

```text
rust_practice/
  notes/       学んだこと、気づき、振り返りを書く
  exercises/   日ごとの小さな練習問題やサンプルコード
  products/    後半で作る小さなプロダクト
  README.md
  study-plan-20days.md
```

## 毎日の進め方

1. `study-plan-20days.md` でその日の内容を確認する
2. `exercises/` または `products/` にコードを書く
3. `cargo check` または `cargo run` で動作確認する
4. `cargo test` でテストを実行する
5. 学んだことを `notes/` に短く残す
6. 最後に `git commit` して区切りをつける

## 最低限よく使うコマンド

```powershell
cargo new exercises/day01_hello_rust
cargo run
cargo check
cargo test
cargo fmt
cargo clippy
```

## コミットの残し方

コミットメッセージは、最初はシンプルに以下の形で十分です。

```text
feat: day 1 hello rust
test: add tests for bmi calculator
docs: update study notes
```

## 学習メモの書き方

各日の終わりに、少なくとも次の 3 つを残すのがおすすめです。

- 今日理解したこと
- まだ曖昧なこと
- 明日やること

## ゴール

このリポジトリのゴールは、Rust の文法を覚えるだけではなく、次の状態に入ることです。

- 小さな Rust アプリを自力で作れる
- テストを書きながら改善できる
- GitHub ベースで変更を管理できる
- 次に Web API や非同期処理へ進む土台がある
