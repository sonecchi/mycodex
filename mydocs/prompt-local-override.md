# ローカルプロンプトの優先読込（そねっち仕様）

このドキュメントは、Codex 実行時に使用されるプロンプトの選択ロジックと、ローカル専用のプロンプト運用方法（そねっち仕様）をまとめたものです。

- 対象バイナリ: `codex`（TUI 起動時）
- 実装箇所: `codex-rs/core/build.rs`, `codex-rs/core/src/client_common.rs`

## 読み込み優先順位
次の順でビルド時に埋め込むプロンプトを決定します。

1. リポジトリ内のローカル上書き: `codex-rs/core/prompt.local.md`（存在すれば優先）
2. デフォルト: `codex-rs/core/prompt.md`

備考: いずれもビルド時にバイナリへ埋め込まれ、実行時にファイル読み込みは行いません。

## 起動時の表示（確認用）
TUI を起動すると、使用中のプロンプトが履歴先頭に表示されます（リリースビルド時）。

- 例（ローカル上書き）: `Using prompt: /…/codex-rs/core/prompt.local.md`
- 例（デフォルト）: `Using prompt: codex-rs/core/prompt.md`

デバッグビルドでは表示しません（スナップショットテスト安定のため）。

## おすすめ運用（ローカル専用）
個人用のプロンプトは Git 管理に載せず、ローカルで完結させるのが安全です（変更反映には再ビルドが必要）。

- 手順（最短）
  - `cp codex-rs/core/prompt.md codex-rs/core/prompt.local.md`
  - （任意・手元保護）`git update-index --skip-worktree codex-rs/core/prompt.local.md`
  - 以降は `prompt.local.md` を編集したら再ビルドで反映されます。

## 旧運用（merge=ours）について
`.gitattributes` の `codex-rs/core/prompt.md merge=ours` は撤廃済み。ランタイムでのローカル上書きは廃止し、ビルド時選択に統一しました。

## ビルドと実行の例
- リリースビルド: `cd codex-rs && cargo build -p codex-cli --release`
- 実行: `./target/release/codex`

## トラブルシュート
- 起動時に「Using prompt: …」が出ない
  - デバッグビルドかも: リリースビルドで確認してください。
  - TUI 経路で起動しているか: `codex` 無引数で TUI が起動します。`codex exec` は TUI を使わないため表示されません。
  - `prompt.local.md` を変更したら再ビルドが必要です。
