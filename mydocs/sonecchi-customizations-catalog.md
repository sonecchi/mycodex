# そねっちカスタム要件カタログ（mainベース再実装用）

最終更新: 2026-02-14

このドキュメントは、現行のカスタム運用ブランチ（例: `sonecchi-merge-20260104`）に入っている「そねっちカスタム」を、**要件（何をしたいか）**と**受け入れ基準（どうなってたらOKか）**と**実装ポイント（どこを触るか）**で整理したもの。

将来は「upstream `main` をベースに、ここに書いてあるカスタムを必要な分だけ再実装する」前提で書いてる。

そねっちの カスタム codex  作りをサポートして。


## 補足メモ（upstream/main 追従時の再確認用）

- `codex-rs/Cargo.toml` の `[profile.release]` は `lto = "thin"` にする（upstream 側で `fat` などに変わっていたら再適用）


## マスト（必ず残す）

### 1) TUI: セッション開始ヘッダーカードの拡張表示（ヘッダーカードのカスタム）

#### どんな機能追加(修正)か

TUI 起動直後に出る「セッション開始ヘッダーカード（カード型のやつ）」に、運用上見たい情報を**常時表示**する。

- 追加で表示する行（現行カスタム）
  - `sandbox:`（sandbox policy）
  - `network:`（enabled / restricted）
  - `approval:`（never / on-request など）
  - `summary:`（auto / concise / detailed / none）
  - `project_doc_max_bytes:`（※後述）
  - `model_auto_compact_token_limit:`（※後述）

※ `/status` のカードとは別。**セッション最上部のヘッダーカード**に出すのが要件。

#### 受け入れ基準（どうなってたらOKか）

- TUI を起動してセッションが設定された直後（最初のヘッダーカード）に、上の追加行が表示される
- `sandbox/network/approval/summary` の値が、実際のセッション設定と一致する
- ヘッダーカード表示の snapshot テストを追加/更新して、以下で確認できる
  - `codex-rs/tui/src/snapshots/codex_tui__history_cell__tests__session_header_snapshot_with_custom_fields.snap`

表示例（雰囲気）:

```
model:      gpt-5-codex high   /model to change
directory:  ~/PycharmProjects/mycodex
sandbox:    workspace-write
network:    restricted
approval:   on-request
summary:    detailed
project_doc_max_bytes: 32768 (32 KB)
model_auto_compact_token_limit: <default>
```

#### 実装方針（触るファイルとかも）

再実装するときは、基本は「ヘッダーカードを描画しているセル」を拡張するだけ。

- 触る場所（現行カスタムの参照実装）
  - `codex-rs/tui/src/history_cell.rs`
    - `new_session_info(...)` で `Config` から値を取り出してヘッダセルに渡す
    - `SessionHeaderHistoryCell` にフィールドを追加して保持する
    - `display_lines(...)` で行を追加して描画する

探し方（upstreamの変化に耐える用）:

- `rg -n "SessionHeaderHistoryCell" codex-rs/tui/src/history_cell.rs`
- `rg -n "fn new_session_info" codex-rs/tui/src/history_cell.rs`

#### 注意点（あれば）

- upstream 側でヘッダーカードの構造（ファイル/型/描画）が変わってても、要件は「**セッション先頭のカードに出す**」なので、同等のフックポイントに移植する。
- ラベル幅の揃えは、既存が `directory:` の幅基準だったりする。追加ラベルの見た目がズレたら、揃え方を見直す（ただし見た目最優先でOK）。
- TUI のスタイルは ratatui の `Stylize` を優先（プロジェクトのTUI規約に従う）。

---

### 2) TUI: `project_doc_max_bytes` をヘッダーカードに表示（project_doc_max_bytes のカスタム）

#### どんな機能追加(修正)か

ヘッダーカードに `project_doc_max_bytes:` 行を追加して、**バイト数**と**KB換算**を表示する。

- 配置: `summary:` の直下（この順序が要件）
- 表示形式:
  - 例: `project_doc_max_bytes: 32768 (32 KB)`
  - 端数が出るときは小数1桁（例: `33000 (32.2 KB)`）

#### 受け入れ基準

- `project_doc_max_bytes` が変更されているとき（例: `-c project_doc_max_bytes=...` や config.toml）に、ヘッダーカードで即確認できる

#### 実装方針（触るファイルとかも）

- `codex-rs/tui/src/history_cell.rs`
  - `Config.project_doc_max_bytes` をヘッダセルへ渡す
  - `project_doc_max_bytes` を `usize` で保持
  - `bytes / 1024.0` を使ってKB表記を組み立てる
    - “ほぼ整数KB”なら `32 KB` のように整数表示（現行カスタムは「誤差 < 0.05」判定）

#### 注意点

- これは「ヘッダーカードへの表示追加」が主目的。project doc の生成/トリミング挙動自体は upstream の実装に依存する。

---

### 3) TUI: `model_auto_compact_token_limit` をヘッダーカードに表示（model_auto_compact_token_limit のカスタム）

#### どんな機能追加(修正)か

ヘッダーカードに `model_auto_compact_token_limit:` 行を追加して、現在の設定値を見える化する。

- 配置: `project_doc_max_bytes:` の直下（この順序が要件）
- 値の扱い:
  - `None` のときは `<default>` と表示（モデル既定値のまま）
  - 正の数のときは桁区切りで見やすく（例: `200,000`）

#### 受け入れ基準

- `~/.codex/config.toml` で `model_auto_compact_token_limit` を設定したら、ヘッダーカードで即確認できる
- `None` の場合も「未設定」が分かる（`<default>`）

#### 実装方針（触るファイルとかも）

- `codex-rs/tui/src/history_cell.rs`
  - `Config.model_auto_compact_token_limit: Option<i64>` をヘッダセルへ渡す
  - 表示時に `Some(v) if v >= 0` のときは “桁区切りフォーマット”を使う（現行は `format_with_separators(v)` 相当）

#### 注意点

- 自動コンパクトを無効化したいなら、`~/.codex/config.toml` に `model_auto_compact_token_limit = 0` を設定する（`0` は disable）。
  - ちなみに「めちゃデカい値にして実質停止」は、`context_window * 0.9` にクランプされるので基本効かない（例: `gpt-5.2-codex` なら上限 `244,800`）。

---

### 4) `notify` フックが動くこと（`~/.codex/config.toml`）

#### どんな機能追加(修正)か

`~/.codex/config.toml` の `notify = [...]` に書いたスクリプトが、**エージェントが1ターン完了するたびに実行**されること。

現行運用例:

```toml
notify = ["/home/sonecchi/PycharmProjects/mycodex/myscripts/codex-notify.sh"]
```

仕様（重要）:

- `notify` は「argv 配列」で、**最後に JSON（イベントpayload）が1引数として追加されて呼ばれる**
- つまり `notify` 側に JSON 引数は書かない（Codexが付ける）

#### 受け入れ基準

- 上の設定で、ターン完了ごとに `codex-notify.sh` が実行される
- スクリプトの「最後の引数」が JSON になっていて、最低限 `type = agent-turn-complete` が入っている

#### 実装方針（触るファイルとかも）

これは基本 upstream の機能として存在してるはずなので、再実装というより **“mainベースでも消えてないことを確認”** が主。

確認ポイント:

- 設定スキーマ/読み込みに `notify` があること（例: `codex-rs/core/src/config/mod.rs`）
- ターン完了時に notifier が呼ばれてること（core側のどこか）
- upstream には notify のE2Eテストがあるので、それが生きてるかも参考になる

#### 注意点

- `notify` は `std::process::Command` 的に直接起動される想定なので、`"~/..."` は展開されないことが多い（**絶対パス推奨**）。
- `myscripts/` はローカル運用前提（git管理外）なので、ファイル実体・実行権限（`chmod +x`）は自分で維持する。
