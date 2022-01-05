# 適当
## 見た目
■□□

(リセット) (ラベル:Goal, Error)

## ゲームへの入出力
- POSTリセット…初期状態へ遷移→その後の盤面
- POSTもとの駒の位置と移動後の駒の位置→(Ok(Goaled, or None), Err)+その後の盤面

# 実装ステップ
## バックエンド
`/app/reset`→{ 初期状態にする; return 盤面JSON}

`app/move`{return 移動JSON}→{ return Option(盤面JSON)}

### ボード状態の管理
actix-webは多数のスレッドをたちあげる模様。この場合ボードロジックは複数作る？
→acitx-webのShared Mutable StateとHashMapを組み合わせる。Uuidごとにロジックインスタンスを紐付け
←結局全部のAPIコマンドで送信データ、受信データを統一したが、現在との差分だけのほうが適切？

### log
logは`tracing`クレートを利用して作成した。
テキストのJSON形式から簡単な実装に変更しようとしたため、`tracing_subscriber`のfmtを利用してlogを取っている。(結局ドキュメントなどを読まなくてはならなかったため、余計に面倒だった。)
出力の形式や拾うspenのイベントなどをデフォルトから少し変更している。
またテストに対する出力部はテキストをそのままうつした。
