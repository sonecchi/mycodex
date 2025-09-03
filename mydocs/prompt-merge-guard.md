# prompt.md の保護運用（merge=ours）とTIPS

このドキュメントは、`codex-rs/core/prompt.md` を上流マージ時に常にローカル（ours）を優先して守るための設定と、運用TIPSをまとめたものです。

## 目的

- 将来の upstream 取り込みで `prompt.md` が意図せず改変されるリスクを回避する。
- 役立つ更新は手動で確認して取り込む（自動検知は行わない）。

## 今回の対応（このリポで実施済み）

- `.gitattributes` を追加して対象ファイルを ours 固定に設定:
  - `codex-rs/core/prompt.md merge=ours`

## そねっちが実行した設定（参考）

- マージドライバ定義（1回だけ必要）
  - リポジトリローカル:
    - `git config merge.ours.name "Keep ours merge driver"`
    - `git config merge.ours.driver true`

これにより、該当ファイルはマージ時に自動的に「ローカル側（ours）」が採用されます。

## 手動確認の流れ（ポリシー）

- upstream の変更は自動検知しない。必要時に手動チェックを行う。
  - `git fetch upstream`
  - `git diff --name-only upstream/main -- codex-rs/core/prompt.md`
  - 差分を取り込みたい場合のみ、ピンポイントで取得してレビュー:
    - `git checkout upstream/main -- codex-rs/core/prompt.md`
    - 確認後に `git add` → `git commit`

## TIPS（設定と適用状況の確認）

- ours マージドライバの定義が存在するか確認:
  - `git config --show-origin --get-regexp '^merge\.ours\.'`
  - ざっくり全ドライバを見る: `git config -l | grep '^merge\..*\.driver'`

- 特定ファイルに `merge=ours` が効いているか確認:
  - `git check-attr merge -- codex-rs/core/prompt.md`
  - 期待出力: `codex-rス/core/prompt.md: merge: ours`

- リポ内で `merge=ours` が付与されているファイル一覧:
  - `git ls-files | xargs -n1 -I{} git check-attr merge -- '{}' | awk -F': ' '$3=="ours" {print $1}' | sort -u`

- 宣言箇所（属性ファイル）を検索:
  - `rg -n --hidden -g '**/.gitattributes' 'merge=ours'`
  - 備考: `.git/info/attributes` に書くパターンもある（リポ外ローカル設定）。

## 一時的に upstream 版を取り込みたいとき

- 一時的に ours を無効化する場合は `.gitattributes` の該当行をコメントアウトしてコミット→マージ→元に戻す、でもOK。
- もしくは前述のコマンドで直接 upstream 版を取り寄せてレビュー＆コミット。

## 注意点（トレードオフ）

- `merge=ours` は静かにローカル勝ちにするため、上流の改善も自動では入らない。必要に応じて手動で差分確認・取り込みを行うこと。

