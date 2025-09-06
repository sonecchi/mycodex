# Git タグ運用メモ（作成・削除・再汚染防止・戻し方）

## タグの作り方

- Annotated タグ: 履歴にメッセージが残る（推奨）
  - `git tag -a v1.2.3 -m "release v1.2.3"`
- Lightweight タグ: ただのポインタ（メッセージなし）
  - `git tag v1.2.3`
- 特定コミットに付ける: SHA やブランチ名を指定
  - `git tag -a hotfix-20250906 <commit-sha> -m "hotfix snapshot"`
- 署名付き（GPG 設定がある場合）
  - `git tag -s v1.2.3 -m "signed release"`
- 付け直し（上書き）
  - `git tag -f v1.2.3 <new-commit>`（リモートにも反映するなら `git push origin -f v1.2.3`）

## タグのプッシュ/フェッチ

- 単体プッシュ: `git push origin v1.2.3`
- まとめてプッシュ: `git push origin --tags`
- 追随プッシュ（annotated で参照されるタグのみ）
  - 設定: `git config push.followTags true`
  - 実行: `git push --follow-tags`
- 取得: `git fetch origin --tags`／`git fetch upstream --tags`

## タグの確認/表示/検索

- 一覧: `git tag -l`
- 並び替え: `git tag -l --sort=creatordate`
- 詳細表示: `git show v1.2.3`
- 到達確認（例: upstream/main に到達するタグ）
  - `git tag --merged upstream/main`

## タグで戻す/作業ブランチを切る

- タグから新規ブランチを作る（安全）
  - `git switch -c restore-<tag> <tag>`
- 直接巻き戻す（破壊的）
  - `git reset --hard <tag>`（必要なら `git push origin --force-with-lease`）

## タグの削除（ローカル/リモート）

- 全ローカルタグ削除: `git tag -l | xargs -n1 git tag -d` 🗑️
- パターン指定削除: `git tag -l 'v1.*' | xargs -n1 git tag -d`
- リモート一覧: `git ls-remote --tags origin`
- リモート個別削除: `git push origin --delete v1.2.3`（同義: `git push origin :refs/tags/v1.2.3`）
- リモート全削除（例・慎重に）
  - `git ls-remote --tags origin | awk '{print $2}' | sed 's#refs/tags/##' | grep -v '\^{}' | xargs -n1 -I{} git push origin :refs/tags/{}`

## 上流タグの再汚染防止（upstream からタグを取らない）

- 無効化: `git config remote.upstream.tagOpt --no-tags`
- 確認: `git config --get-all remote.upstream.tagOpt`
- 戻す: `git config --unset remote.upstream.tagOpt`（必要なら）

## 設定の確認（総覧）

- `git config -l`

---
メモ: タグは「ブランチに属さない」リポ全体の参照。`git push origin sonecchi` ではタグの削除/作成は反映されないので、タグは明示的に push/delete する。

