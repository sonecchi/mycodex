# ローカルプロンプトの優先読込（そねっち仕様）

このドキュメントは、Codex 実行時に使用されるプロンプトの選択ロジックと、ローカル専用のプロンプト運用方法（そねっち仕様）をまとめたものです。

- 対象バイナリ: `codex`（TUI 起動時）
- 実装箇所: `codex-rs/core/src/client_common.rs`

## 読み込み優先順位
次の順で最初に見つかったものを使用します。

1. 環境変数で指定: `CODEX_PROMPT_LOCAL=/abs/path/to/prompt.md`
2. リポジトリ内のローカル上書き: `codex-rs/core/prompt.local.md`
3. デフォルトの埋め込み: `codex-rs/core/prompt.md`

備考: 3 はビルド時にバイナリへ埋め込まれた内容です（実行時にファイル読み込みは行いません）。

## 起動時の表示（確認用）
TUI を起動すると、使用中のプロンプトが履歴先頭に表示されます（リリースビルド時）。

- 例（環境変数指定）: `Using prompt: CODEX_PROMPT_LOCAL=/abs/path/to/prompt.md`
- 例（ローカル上書き）: `Using prompt: /…/codex-rs/core/prompt.local.md`
- 例（デフォルト）: `Using prompt: codex-rs/core/prompt.md`

デバッグビルドでは表示しません（スナップショットテスト安定のため）。

## おすすめ運用（ローカル専用）
個人用のプロンプトは Git 管理に載せず、ローカルで完結させるのが安全です。

- 手順（最短）
  - `cp codex-rs/core/prompt.md codex-rs/core/prompt.local.md`
  - （任意・手元保護）`git update-index --skip-worktree codex-rs/core/prompt.local.md`
  - 以降は `prompt.local.md` を自由に編集。起動時に自動で優先使用されます。

- 代替（環境変数で外部ファイルを指定）
  - `CODEX_PROMPT_LOCAL=/abs/path/prompt.md ./target/release/codex`

## 旧運用（merge=ours）について
`.gitattributes` の `codex-rs/core/prompt.md merge=ours` は撤廃しました。マージ戦略に依存せず、ランタイムでローカル上書きを選ぶ現在の方式に統一します。

## ビルドと実行の例
- リリースビルド: `cd codex-rs && cargo build -p codex-cli --release`
- 実行: `./target/release/codex`

## トラブルシュート
- 起動時に「Using prompt: …」が出ない
  - デバッグビルドかも: リリースビルドで確認してください。
  - TUI 経路で起動しているか: `codex` 無引数で TUI が起動します。`codex exec` は TUI を使わないため表示されません。
