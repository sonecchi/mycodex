# Codex CLI のビルド（通常ビューで思考表示）クイックメモ

- コンテキスト: `codex/codex-rs/tui/src/history_cell.rs` を変更して通常ビューに reasoning（思考）をライブ表示。トランスクリプト記録は維持。ヘッダー文言は `tui/src/streaming/mod.rs` で「俺っち」にカスタム。
  - 変更点: `on_agent_reasoning_delta` でライブにストリーミングし、トランスクリプト側は `TranscriptOnlyHistoryCell` のまま。

## リポジトリのクローン
- `cd ~/PycharmProjects/sample`
- `git clone git@github.com:sonecchi/mycodex.git`
- `cd mycodex/codex-rs`

## 事前準備（Ubuntu/Debian）
- `sudo apt-get update && sudo apt-get install -y build-essential pkg-config libssl-dev`
- Rust（rustup）インストール: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- 反映: `source ~/.cargo/env`
fishシェルの場合は
source "$HOME/.cargo/env.fish"

## ビルド
プロジェクトルートから
- `cargo build --manifest-path codex-rs/Cargo.toml -p codex-cli --release`
もしくは cd codex-rs 後に
- `cargo build -p codex-cli --release`

## 実行
- `./target/release/codex`
もしくはエイリアスを設定
ln -sfTv /home/sonecchi/PycharmProjects/mycodex/codex-rs/target/release/codex ~/.local/bin/mycodex



## メモ
- 初回ビルドは依存取得とリンクで時間がかかる。2回目以降は速い。
- 高速化 Tips: `sudo apt-get install -y lld` 後に `RUSTFLAGS="-C link-arg=-fuse-ld=lld" cargo build -p codex-cli --release`

