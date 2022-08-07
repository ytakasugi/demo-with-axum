# demo-with-axum

## 公式ドキュメント

* [Crate axum](https://docs.rs/axum/latest/axum/)
* [Crate sqlx](https://docs.rs/sqlx/latest/sqlx/index.html)

## 参考サイト

* [Rustの新しいWEBフレームワークaxumを触ってみた](https://zenn.dev/techno_tanoc/articles/99e54c82cb049f)
* [[Rust] sqlxを使ってみる](https://qiita.com/yagince/items/ffbff7d15420be1fc411)
* [Rust の新しい HTTP サーバーのクレート Axum をフルに活用してサーバーサイドアプリケーション開発をしてみる](https://blog-dry.com/entry/2021/12/26/002649)
* [Rust で GraphQL server を書いてみた](https://zenn.dev/takurinton/articles/bab60687f17c2b)
* [sqlxでPoolとTransactionのどちらも受け取れるようにする](https://qiita.com/FuJino/items/08b4c3298918191eab65)
* []()

## 参考実装

* [ndelvalle/rustapi](https://github.com/ndelvalle/rustapi)
* [yuk1ty/stock-metrics](https://github.com/yuk1ty/stock-metrics)

## 動作確認用

```
curl -X POST -H "Content-Type: application/json" -d '{"user_name": "test01", "e_mail": "test01@example.com"}' http://localhost:3000/users
curl -X PUT -H "Content-Type: application/json" -d '{"e_mail": "test01@example.com", "delete_flag": false}' http://localhost:3000/users/19
curl -X PUT -H "Content-Type: application/json" -d '{"e_mail": null, "delete_flag": true}' http://localhost:3000/users/19
curl -X DELETE http://localhost:3000/users/17
```
