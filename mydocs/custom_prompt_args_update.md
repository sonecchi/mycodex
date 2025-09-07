# Custom Prompt Args 対応 変更サマリ

本変更では、カスタムスラッシュプロンプトに引数を渡す `{{args}}` 展開機能を追加しました。

## 変更ファイル

- `codex-rs/tui/src/bottom_pane/custom_prompt_args.rs`（新規）
  - `parse_slash_and_args(first_line) -> Option<(String, String)>`
    - 先頭行が `/name ARG…` 形式の場合に `(name, args)` を抽出。
  - `expand_custom_prompt(prompts, name, args) -> Option<String>`
    - `name` に一致するカスタムプロンプト本文中の `{{args}}` を `args` で置換（未含有ならそのまま返却）。

- `codex-rs/tui/src/bottom_pane/mod.rs`
  - `mod custom_prompt_args;` を追加してモジュールを有効化。

- `codex-rs/tui/src/bottom_pane/chat_composer.rs`
  - スラッシュポップアップでカスタムプロンプトを Enter した際：
    - クリア前の先頭行から `/name ARG…` を解析し、選択したプロンプト名と一致する場合に `{{args}}` を展開して送信。
  - 通常送信（ポップアップ非表示）で先頭行が `/name ARG…` の場合：
    - 対応するカスタムプロンプトを探し、`{{args}}` を展開して送信（本文全体を置換）。

- `docs/prompts.md`
  - 「Passing Arguments to Custom Prompts」節を追加。
  - 使い方、挙動、制限事項、動作確認例を追記。

## 実装ポイント / 注意点

- 解析は「先頭行のみ」を対象とし、名前は `/` 直後の最初のトークン、`args` はその後ろの最初の空白以降（trim 済み）。
- `{{args}}` が本文に存在しない場合はそのまま送信（置換なし）。
- スラッシュポップアップからの Enter では、入力中の `/name` と選択したプロンプト名が一致する場合にのみ `args` を適用。
- ビルトインと同名のプロンプトは（従来どおり）ポップアップから除外。

## 動作確認（例）

```bash
echo 'Hello, {{args}}!' > ~/.codex/prompts/greeter.md
```

TUI で以下を入力：

```
/greeter Alice and Bob
```

→ 送信内容：`Hello, Alice and Bob!`

## 既知の制限

- テンプレート変数は `{{args}}` のみ対応。他の形式（`{{1}}` など）は未実装。
- 複数行の入力がある場合でも解析対象は先頭行のみ。

