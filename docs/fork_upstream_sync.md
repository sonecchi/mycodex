# フォークに本家（upstream）のアップデートを取り込む手順

このドキュメントは、GitHub でフォーク（origin）に対して本家リポジトリ（upstream）の更新を取り込む定番フローをまとめたものです。デフォルトブランチを `main` として記載しています（必要に応じて読み替えてください）。

## 1. upstream を登録（初回のみ）

```bash
git remote add upstream <本家のURL>
# 例（SSH）
# git remote add upstream git@github.com:openai/codex.git
# 例（HTTPS）
# git remote add upstream https://github.com/openai/codex.git

# 登録確認
git remote -v
```

## 2. 本家の最新を取得

```bash
git fetch upstream
```

## 3. デフォルトブランチ（main）を更新

- マージ派（履歴を保ったまま）。基本は fast-forward を推奨:

```bash
git checkout main
git merge --ff-only upstream/main
```

- リベース派（履歴を直線に保つ。競合時は解消が必要）:

```bash
git checkout main
git rebase upstream/main
```

## 4. フォーク（origin）へ反映

```bash
# マージした場合
git push origin main

# リベースした場合（履歴が書き換わるので注意）
git push --force-with-lease origin main
```

## 5. 開発ブランチを最新化

```bash
# 開発ブランチに切り替え
git checkout my-feature

# どちらか好みで
# 直線履歴にしたい場合
git rebase main
# もしくは通常のマージ
git merge main

# リベースして履歴が変わった場合は（保険付き強制）
git push --force-with-lease
```

## 小ワザ・補足

- 本家のデフォルトブランチ名を確認する:

```bash
git remote show upstream  # "HEAD branch: ..." を確認
```

- 一発取り込み（main 上で実行）:

```bash
git pull --rebase upstream main
```

- リモート URL を HTTPS から SSH に切り替え（origin の例）:

```bash
git remote set-url origin git@github.com:<your-account>/<your-repo>.git
# 例: git remote set-url origin git@github.com:sonecchi/mycodex.git
```

- SSH 接続テストと動作確認:

```bash
ssh -T git@github.com
git fetch origin
```

- 事故防止メモ:
  - `--force` ではなく `--force-with-lease` を使うと、他人の更新を誤って上書きしづらいです。
  - リベース運用中は、プッシュ前に `git status` / `git log --oneline --graph --decorate -n 10` で履歴を確認しましょう。

---

以上で、本家の更新をフォークに取り込むフローは完了です。必要に応じてブランチ名やリモート名（`origin`/`upstream`）を読み替えて使ってください。
