# Docker Official の Rust イメージを使います。
# ビルド用のイメージを builder と名付けます。
FROM rust:latest AS builder

# /todo でビルドを行うことにします。
WORKDIR /todo

# Cargo.toml のみを先にイメージに追加する。
COPY Cargo.toml Cargo.toml

# ビルドに必要なファイルをイメージにコピーします。

# ビルドするために何もしないソースコードを入れておく。
RUN mkdir src
RUN echo "fn main(){}" > src/main.rs

# 上記で作成したコードと依存クレートをビルドする。
RUN cargo build --release

# アプリケーションのコードをイメージにコピーする。
COPY ./src ./src
COPY ./templates ./templates

# 先ほどビルドした生成物のうち、アプリケーションのもののみを削除する。
RUN rm -f target/release/deps/todo*

# 改めてアプリケーションをビルドする。
RUN cargo build --release

# リリース用イメージには debian を使用します。
FROM rust:latest

# builder イメージから todo のみをコピーして /usr/local/bin に配置します。
COPY --from=builder /todo/target/release/todo /usr/local/bin/todo

# コンテナ起動時に Web アプリを実行します。
CMD ["todo"]
