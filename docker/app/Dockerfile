FROM rust:1.58.0 as dev

WORKDIR /work/

# キー=バリュー
# cargoの出力ディレクトリの指定
ENV CARGO_TARGET_DIR=/tmp/target \
  DEBIAN_FRONTEND=noninteractive \
  LC_CTYPE=ja_JP.utf8 \
  LANG=ja_JP.utf8

RUN apt-get update \
  && apt-get upgrade -y \
  && apt-get install -y -q \
  ca-certificates \
  locales \
  apt-transport-https\
  libssl-dev \
  libpq-dev \
  pkg-config \
  curl \
  build-essential \
  libdbus-1-dev \
  libsqlite3-dev \
  mariadb-client \
  git \
  wget
RUN  echo "ja_JP UTF-8" > /etc/locale.gen
RUN  locale-gen
RUN  echo "install rust tools"
RUN  rustup component add rustfmt
RUN  cargo install cargo-watch cargo-make

CMD ["cargo","watch","-x","run"]
