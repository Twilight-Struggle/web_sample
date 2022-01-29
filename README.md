[![codecov](https://codecov.io/gh/Twilight-Struggle/anisoc/branch/main/graph/badge.svg?token=QX2AV5UEFQ)](https://codecov.io/gh/Twilight-Struggle/anisoc)

# 適当
## 見た目
■□□

(リセット) (ラベル:Goal, Error)

## ゲームへの入出力
- POSTリセット…初期状態へ遷移→その後の盤面
- POSTもとの駒の位置と移動後の駒の位置→(Ok(Goaled, or None), Err)+その後の盤面

# 実装ステップ
## バックエンド
`/app/reset`→{初期状態にする; return 盤面JSON}

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

## フロントエンド
reactを使用した。チュートリアルそのまんまなので特に言うことはない。
CORS制限を回避するためには、reactのproxy機能を使用する。
package.jsonにproxyとしてバックエンドのURLを指定する。
https://reffect.co.jp/react/front-react-back-node

# 継続的開発の枠組み
dockerを利用したバックエンドのデプロイ→(フロントエンドのデプロイ)→両者含めたデプロイをローカルで行う。
両者が統合されるため、CI機能を作成できるようになる…はず。クラウドを利用してデプロイしてみてもいいだろう。CDは実装は難しいか？

## デプロイ
バックエンドデプロイ→フロントエンドデプロイ→統合してデプロイの計画

### バックエンドのデプロイ
dockerと通常起動で許可するアドレスを変更する→dockerignoreファイル作成→ビルドステージとランタイムステージに分割

### フロントエンドバックエンドの統合
`npm run build`で生成されるbuildディレクトリをバックエンドから返せば良い。
build以下のindex.html及びbuildディレクトリを返却するようにバックエンド側に設定する。
https://zenn.dev/tminasen/articles/00c31072100e5d0e861f

また先程のページも参考になる。
https://reffect.co.jp/react/front-react-back-node

### フロントエンド + バックエンドのデプロイ
[バックエンドのデプロイ](#バックエンドのデプロイ)にreactのビルドステージを追加し、ランタイムステージでコピーしてくれば良い。
ビルドはトップディレクトリで以下のコマンドで実行できる。

```sh
$ docker build --tag anisoc --file Dockerfile .
```

実行は以下の通り
```sh
$ docker run -p 8000:8000 anisoc
```

ブラウザでlocalhost:8000にアクセスすれば実行できる。

### クラウドを利用してデプロイ
GCP cloud run?
ここは動物サッカー実装後に行う。

## CI
GitHub Actionを使用する。
