# パッチ一覧と運用メモ

このドキュメントは、リポジトリに存在するローカルパッチの目的・内容・適用方法をまとめたものです。現状のパッチは以下の2系統＋1（ビルド時プロンプト選択）です。

## 管理方式（全体像）

- 変更差分: リポジトリ直下の `changes.patch`（UIまわり）と `prompt-build.patch`（ビルド時プロンプト選択）
- 適用スクリプト: `scripts/cot-patch.sh`（`status|on|off|ensure`）
- Gitフック: `.githooks/post-merge`（マージ後に自動再適用）
- justエイリアス: ルート/`codex-rs` の `justfile` に `cot-status` / `cot-on` / `cot-off`

## 現在の主なパッチ

1) CoT（thinking）可視化ストリーミング強化

- 目的: エージェントの推論デルタ（reasoning）を「標準ビュー」にライブ表示し、作業中の手がかりを即時に見える化する。
- 主な変更点:
  - `codex-rs/tui/src/chatwidget.rs`
    - `on_agent_reasoning_delta`: 推論デルタを通常ストリームへ流し込み、見出し（ヘッダ）を抑制して連続表示。ステータスヘッダ更新は継続。
    - `on_agent_reasoning_final`: 表示用ストリームをセパレータ付きでフラッシュし、全文はトランスクリプトとして履歴に格納。
  - `codex-rs/tui/src/streaming/controller.rs`
    - `suppress_header_for_current_stream` を追加し、当該ストリームのみヘッダ描画を抑制可能に。
  - `codex-rs/tui/src/streaming/mod.rs`
    - `HeaderEmitter::suppress_for_stream` を追加（ストリーム単位でヘッダ既出扱い）。
  - スナップショット更新（挙動変化の反映）
    - `codex-rs/tui/src/chatwidget/snapshots/*`
    - `codex-rs/tui/tests/fixtures/ideal-binary-response.txt`
- 効果: 「分析中…」のような思考過程が通常ビューに逐次現れ、ユーザーのフィードバックや割り込み判断がしやすくなる。

2) カスタムプロンプトの引数展開（`/name args` → `{{args}}` 置換）

- 目的: スラッシュコマンド風の入力でカスタムプロンプトに任意の引数を渡し、本文中の `{{args}}` を置換して投入できるようにする。
- 主な変更点:
  - 新規: `codex-rs/tui/src/bottom_pane/custom_prompt_args.rs`
    - `parse_slash_and_args("/foo bar baz") -> ("foo", "bar baz")`
    - `expand_custom_prompt(...)`（補助）
  - `codex-rs/tui/src/bottom_pane/chat_composer.rs`
    - ポップアップ選択時やEnter送信時に、先頭行が `"/name args"` 形式なら該当カスタムプロンプトの `{{args}}` を置換して送信。
  - `codex-rs/tui/src/bottom_pane/mod.rs`
    - `mod custom_prompt_args;` を追加しモジュール化。
- 使い方:
  - 例: カスタムプロンプト `my-prompt` の本文に `{{args}}` を含めておき、入力欄で `/my-prompt ここが置換される` と打つ。
  - 送信時に本文の `{{args}}` が `ここが置換される` に差し替わる。

3) ビルド時プロンプト選択（`prompt.local.md` 優先ビルド）

- 目的: 実行時の動的読込をやめ、ビルド時に `prompt.local.md` が存在すればそれを優先的に埋め込む。これにより保存直後のレースで「Instructions are not valid」にならないようにする。
- 主な変更点:
  - `codex-rs/core/build.rs` を追加し、`prompt.local.md` / `prompt.md` のどちらかを `$OUT_DIR/base_instructions.txt` にコピー＋ヒューマンリーダブルなソース名を `CODEX_BASE_INSTRUCTIONS_HUMAN` として埋め込む。
  - `codex-rs/core/src/client_common.rs` で `include_str!(concat!(env!("OUT_DIR"), "/base_instructions.txt"))` を使用し、実行時読込ロジックを撤廃。
  - 併せて `base_instructions_source_human()` はビルド時に設定した環境変数を返す。
  - ドキュメント: `mydocs/prompt-local-override.md` を更新。

備考: `scripts/cot-patch.sh` は複数パッチに対応（`changes.patch` と `prompt-build.patch` を順次判定・適用）。

## 運用コマンド（適用/解除/状態確認）

- 状態確認: `just cot-status`（両パッチの適用状態を個別に表示）
- 適用（ON）: `just cot-on`（未適用のパッチのみ適用）
- 解除（OFF）: `just cot-off`（適用済みのパッチのみ取り消し）
- マージ後の自動再適用: `.githooks/post-merge` が `scripts/cot-patch.sh ensure` を実行

メモ: 現在のワークツリーでは `scripts/cot-patch.sh status` 実行時に「[cot] patch is applied ✅」が表示される想定（適用済み）。

## スナップショットテストについて

- UIやテキスト出力が変わるため、TUI周辺のスナップショットが更新対象になります。
- レビュー/反映の流れ（参考）:
  - 生成: `cargo test -p codex-tui`
  - 保留確認: `cargo insta pending-snapshots -p codex-tui`
  - 個別確認: `cargo insta show -p codex-tui path/to/file.snap.new`
  - 一括受理（意図的変更のみ）: `cargo insta accept -p codex-tui`

## パッチ更新のヒント

- 上流の変更で `changes.patch` が当たらない場合は、いったん競合を解消してから `just cot-on` を再実行。
- `changes.patch` を更新したいときの一例（慣れた人向け）:
  - 上流相当（パッチ非適用状態）とローカルカスタマイズ（適用状態）の差分を `git diff` で再生成して `changes.patch` を差し替える。
  - チーム運用では「どのブランチとの差分か」を明記して作り直すと安全。

## 参考ファイル一覧

- `changes.patch`
- `scripts/cot-patch.sh`
- `.githooks/post-merge`
- `justfile`, `codex-rs/justfile`
- ドキュメント: `docs/cot-patch.md`, `mydocs/upstream-merge-and-build-ja.md`
