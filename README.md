# rust-guide-sample-beginner-api

rust-guide 初級ガイド用のサンプルリポジトリです。axum を使ったシンプルな Web API です。

## セットアップ

```sh
git clone https://github.com/shinagawa-web/rust-guide-sample-beginner-api.git
cd rust-guide-sample-beginner-api
```

## コマンド

```sh
# ビルド
cargo build

# 起動（http://localhost:3000）
cargo run

# テスト
cargo test

# フォーマット
cargo fmt

# リント
cargo clippy
```

## エンドポイント

| メソッド | パス          | 説明               |
|----------|---------------|--------------------|
| GET      | /users        | ユーザー一覧を返す |
| GET      | /users/:id    | ユーザー詳細を返す |
