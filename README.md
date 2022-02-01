[![codecov](https://codecov.io/gh/Twilight-Struggle/web_sample/branch/main/graph/badge.svg?token=QX2AV5UEFQ)](https://codecov.io/gh/Twilight-Struggle/web_sample)

# 適当

## 見た目

■□□

(リセット) (ラベル:Goal, Error)

## ゲームへの入出力

- POST リセット…初期状態へ遷移 → その後の盤面
- POST もとの駒の位置と移動後の駒の位置 →(Ok(Goaled, or None), Err)+その後の盤面

# 実装ステップ

## バックエンド

`/app/reset`→{初期状態にする; return 盤面 JSON}

`app/move`{return 移動 JSON}→{ return Option(盤面 JSON)}

### ボード状態の管理

actix-web は多数のスレッドをたちあげる模様。この場合ボードロジックは複数作る？
→acitx-web の Shared Mutable State と HashMap を組み合わせる。Uuid ごとにロジックインスタンスを紐付け
← 結局全部の API コマンドで送信データ、受信データを統一したが、現在との差分だけのほうが適切？

### log

log は`tracing`クレートを利用して作成した。
テキストの JSON 形式から簡単な実装に変更しようとしたため、`tracing_subscriber`の fmt を利用して log を取っている。(結局ドキュメントなどを読まなくてはならなかったため、余計に面倒だった。)
出力の形式や拾う spen のイベントなどをデフォルトから少し変更している。
またテストに対する出力部はテキストをそのままうつした。

## フロントエンド

react を使用した。チュートリアルそのまんまなので特に言うことはない。
CORS 制限を回避するためには、react の proxy 機能を使用する。
package.json に proxy としてバックエンドの URL を指定する。
https://reffect.co.jp/react/front-react-back-node

# 継続的開発の枠組み

docker を利用したバックエンドのデプロイ →(フロントエンドのデプロイ)→ 両者含めたデプロイをローカルで行う。
両者が統合されるため、CI 機能を作成できるようになる…はず。クラウドを利用してデプロイしてみてもいいだろう。CD は実装は難しいか？

## デプロイ

バックエンドデプロイ → フロントエンドデプロイ → 統合してデプロイの計画

### バックエンドのデプロイ

docker と通常起動で許可するアドレスを変更する →dockerignore ファイル作成 → ビルドステージとランタイムステージに分割

### フロントエンドバックエンドの統合

`npm run build`で生成される build ディレクトリをバックエンドから返せば良い。
build 以下の index.html 及び build ディレクトリを返却するようにバックエンド側に設定する。
https://zenn.dev/tminasen/articles/00c31072100e5d0e861f

また先程のページも参考になる。
https://reffect.co.jp/react/front-react-back-node

### フロントエンド + バックエンドのデプロイ

[バックエンドのデプロイ](#バックエンドのデプロイ)に react のビルドステージを追加し、ランタイムステージでコピーしてくれば良い。
ビルドはトップディレクトリで以下のコマンドで実行できる。

```sh
$ docker build --tag web_sample --file Dockerfile .
```

実行は以下の通り

```sh
$ docker run -p 8000:8000 web_sample
```

ブラウザで localhost:8000 にアクセスすれば実行できる。

### クラウドを利用してデプロイ

GCP cloud run?
ここは動物サッカー実装後に行う。

## CI

GitHub Action を使用する。
codecov を使い coverage テストを行った。
フロントエンドの CI は準備中

### react のテスト

Jest を使用する。

#### インストールしたもの

@testing-library/react をインストール。これは react テストに使用する標準ライブラリ。create react app を使用するとデフォルトで入っているようだ。
msw はサーバモック用。

@babel/preset-react, @babel/preset-typescript これは入れないとうまく Jest が react+typescript を読み込めない。
これら 2 つを読み込むために babel.config.js を追加した。Jest に失敗したときに表示されたサイトから引用。

#### テスト用ファイル

testing-library/react の example からの引用。最初のおまじないがないと node.js ネイティブのテストだと勘違いするようだ。
