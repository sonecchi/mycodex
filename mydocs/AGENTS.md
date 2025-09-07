# このプロジェクトについて
本家のcodex(ubstream)をforkしてそねっち用にカスタマイズしたものです。


```
git remote -v
origin	git@github.com:sonecchi/mycodex.git (fetch)
origin	git@github.com:sonecchi/mycodex.git (push)
upstream	git@github.com:openai/codex.git (fetch)
upstream	git@github.com:openai/codex.git (push)
```

# カスタマイズ内容


# システムプロンプト
codex-rs/core/prompt.md


# ビルド方法
cd codx-rs
cargo build --release

ビルドに成功すると以下に実行ファイルが作られる。
codex-rs/target/release/codex

