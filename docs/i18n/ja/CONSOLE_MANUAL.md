# INFO Desktop コンソールモード：簡易マニュアル

状態：0.1.5 に含めてリリース済み。2 つのモード：開いているアプリウィンドウ経由と単独。

## 概要
`info-desktop-cli` プログラムはウィンドウと同じことを行います：キューの表示、ルートの説明、項目の作成と編集、公開、XLSX 設定の操作。デバイス鍵はウィンドウと同じ OS の安全なストアから取得され、出力されません。

## モード
- **開いているウィンドウ経由（共有接続）。** アプリウィンドウで「このウィンドウ経由でコンソールを動作させる」にチェックを入れます（既定はオフ）。ウィンドウが開いている間、コンソールはその接続を使用します：VPN と鍵は設定済みで、鍵は渡されません。入口はこのコンピューター内のみ（127.0.0.1）で、ユーザーの設定フォルダーに保存された一回限りのセッションパスワードで保護されます。ウィンドウはチェックボックスの横にコンソールの全操作を表示します。
- **単独。** ウィンドウは不要：コンソールが接続プロファイルと OS ストアの鍵でサーバーに直接接続します。ウィンドウは接続のたびにプロファイル（アドレス、ポート、デバイス番号、サーバー証明書；鍵なし）を書き込みます。手動で設定することもできます（`profile set`）。
- 選択：`--mode auto`（既定：ウィンドウが開いていて許可していればウィンドウ、そうでなければ単独）、`--mode app`（ウィンドウのみ）、`--mode alone`（単独のみ）。`--mode` なしで明示的な `--profile` を指定すると単独になります。

## 初回起動
1. いつも通りアプリウィンドウで接続します（Windows で当方のサーバーを使う場合は VPN をオンにしてください）。
2. プロファイルはウィンドウが自分で書き込みます。手動が必要な場合のみ：`info-desktop-cli profile set --server アドレス --port ポート --device-id 番号 --ca-file ファイル`；ファイルには登録ファイルの `server_ca_der` の値が入っています。
3. 接続を確認：`info-desktop-cli status`。

## コマンド
引数なしで `info-desktop-cli` を実行すると完全なヘルプが表示されます。

| 作業 | コマンド |
|---|---|
| チャンネル一覧 | `channels` |
| キュー | `queue list [--limit N] [--cursor N] [--channel コード] [--bucket グループ]` |
| 出典付きの項目 | `queue show ID` |
| このチャンネルにある理由 | `why ID --channel コード [--scope route\|auto\|digest\|instant\|relevance]` |
| 項目の作成 | `create --channel コード --title テキスト (--text テキスト \| --text-file ファイル) [--url URL]` |
| 項目の編集 | `save ID --title テキスト (--text テキスト \| --text-file ファイル) [--channel コード]` |
| スキップ、保留、公開 | `skip ID`、`defer ID [--minutes N]`、`publish ID` |
| XLSX 設定 | `settings status`、`settings download --out ファイル`、`settings check ファイル`、`settings probe [--hours N]`、`settings probe-status`、`settings apply [--without-probe] [--yes]`、`settings rollback [--yes]` |
| テーマと単語 | `themes list`、`themes test ...`、`themes save ...` |

## 設定の変更
まず `settings check`（適用せずにファイルを検査）、次に `settings probe` と `settings probe-status`（「何が変わるか」）、最後に `settings apply`。`--yes` がなければコマンドは適用される内容を表示するだけです。検査したファイルとまったく同じものが適用されます（ダイジェストで確認）。試行なしでは `--without-probe` を付けない限り拒否します。ロールバック：`settings rollback --yes`。

公開（`publish`）と通常の編集操作には確認は不要です：実行者の裁量と責任で行い、投稿はいつでもチャンネルで削除できます。

## 終了コード
0 成功、1 コマンドエラー、2 引数の誤り、3 接続または鍵なし、4 サーバーが拒否（権限なし、競合）。

## まだ行っていないこと
グラフィカルセッションのないサーバーでの動作（所有者が保留）。2026 年 11 月 5 日に Windows で実サーバーを使って確認（読み取り）：`status`、`channels`、`queue list`、ウィンドウ経由の `why`、単独および自動の `status`。未確認：書き込みコマンド、XLSX とテーマ、macOS と Linux。
