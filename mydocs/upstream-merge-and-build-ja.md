このプロジェクトは git@github.com:openai/codex.git をフォークし、そねっち好みのカスタマイズを加えたものである。

# アップストリーム取り込みとビルド手順（日本語）

このドキュメントは、フォーク（`origin`）に本家（`upstream`）の更新を取り込み、ローカルの CoT パッチ（thinking 可視化）を適用したうえでビルドするまでの手順をまとめたクイックガイドです。

- リモート想定: `origin` = 自分のフォーク, `upstream` = 本家
- 運用方針: マージ派（rebase は使わない）
- CoT 表示: パッチ方式で常時表示。マージ後に自動再適用（フック有効時）

## 初回セットアップ（未設定なら）

```bash
# Git フックをこのリポジトリの .githooks に切り替え
git config core.hooksPath .githooks

# マージ学習（rerere）を有効化（推奨）
git config rerere.enabled true

# 実行権限の付与
chmod +x scripts/cot-patch.sh .githooks/post-merge
```

## 標準フロー（毎回の実運用）

1) 本家の最新を取得して `main` を更新

```bash
git fetch upstream

git checkout main

git merge upstream/main
```

2) 作業ブランチに `main` を取り込む（例: `sonecchi`）

```bash
git switch sonecchi

# CoT パッチを一時的に外して退避
just cot-off

git stash push -u -m "temp: cot-off before merge"

# main を取り込み（マージ運用）
git merge main

# 競合が出たら解消 → ステージング → コミット
git status
# ファイルを直して
# git add <fixed-files>
# git commit
```

3) CoT パッチの再適用（ここが重要！）

- フックが有効ならマージ直後に自動再適用されます。状態を確認して、必要なら手動で ON にします。

```bash
# 状態確認
just cot-status

# 未適用なら再適用（← ここがビルドの“前”）
just cot-on
```

4) ビルド（`codex-rs` 配下で CLI をリリースビルド）

```bash
cd codex-rs/
cargo build -p codex-cli --release
```

## 補足・運用メモ

- stash 運用: 上の `stash` は一時退避用。基本は `pop` せず様子見し、問題なければ `git stash drop` で掃除。
- うまく当たらないとき: `just cot-off && just cot-on` の順で再試行。パッチ競合は解消してから再実行。
- 競合の把握: `git status` と `git diff --name-only --diff-filter=U` を併用すると把握が早いです。

## 関連ドキュメント

- `docs/cot-patch.md`: CoT パッチ（適用/解除/状態確認）
