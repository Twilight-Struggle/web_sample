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