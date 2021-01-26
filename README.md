# [Rust](https://www.rust-lang.org/ja)

## インストール
1. ターミナルで以下のコマンドを実行する。
> RustupはRustのインストーラ兼バージョン管理ツール。
```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 上記コマンド実行によりRustのツール群が「~/.cargo/bin」にインストールされるので、このパスをshプロファイルに記述し再読み込みする。

3. ターミナルで以下のコマンドを実行し、バージョンが表示されたらインストール成功。
```sh
$ rustc --version
rustc 1.49.0 (e1884a8e3 2020-12-29)
```

4. ターミナルで以下のコマンドを実行し、バージョンが表示されたらcargoのインストールも成功している。
> CargoはRustビルドツールおよびパッケージマネージャ。
```sh
$ cargo --version
cargo 1.49.0 (d00d64df9 2020-12-05)

# プロジェクトのテスト
$ cargo test
# プロジェクトのドキュメントのビルド
$ cargo doc
# ライブラリをcrates.ioに公開
$ cargo publish
```

5. お好みでエディタサポートツールを導入する。
> [Rust support for Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)など。

## プロジェクト作成・実行・テストなど
1. ターミナルで以下のコマンドを実行してプロジェクトを作成する。
> Cargo.tomlとsrc/main.rsが作成される。前者がいわゆるマニフェストであり、パッケージ依存関係もここに記述される。 
```sh
$ cargo new hello-rust
$ ll -lR hello-rust
total 8
drwxr-xr-x  5 terunrun  staff  160  1 25 22:48 ..
-rw-r--r--  1 terunrun  staff  223  1 25 22:48 Cargo.toml
drwxr-xr-x  4 terunrun  staff  128  1 25 22:48 .
drwxr-xr-x  3 terunrun  staff   96  1 25 22:48 src

hello-rust/src:
total 8
drwxr-xr-x  4 terunrun  staff  128  1 25 22:48 ..
drwxr-xr-x  3 terunrun  staff   96  1 25 22:48 .
-rw-r--r--  1 terunrun  staff   45  1 25 22:48 main.rs
```

2. ターミナルで作成されたディレクトリ配下に移動し、以下のコマンドを実行してアプリケーションを実行する。
> Cargo.tomlが存在しないディレクトリではアプリケーションが実行できない。
> 実行するとCargo.lockとtargetディレクトリが作成される。
```sh
$ cd hello-rust/
$ cargo run
   Compiling hello-rust v0.1.0 (/Users/terunrun/git/github/sandbox-rust/hello-rust)
    Finished dev [unoptimized + debuginfo] target(s) in 0.65s
     Running `target/debug/hello-rust`
Hello, world!
$ ll
total 16
drwxr-xr-x  5 terunrun  staff  160  1 25 22:48 ..
-rw-r--r--  1 terunrun  staff  223  1 25 22:48 Cargo.toml
drwxr-xr-x  3 terunrun  staff   96  1 25 22:48 src
-rw-r--r--  1 terunrun  staff  141  1 25 22:56 Cargo.lock
drwxr-xr-x  6 terunrun  staff  192  1 25 22:56 .
drwxr-xr-x@ 5 terunrun  staff  160  1 25 22:56 target
```

# ビルド
1. src配下のモジュールやCargo.tomlを更新し、ターミナルで以下のコマンドを実行する。
> Cargo.tomlの記述内容に従いパッケージ（Rust風に言うとクレート、crate）がインストールされ、Cargo.lockにその状態が記述される。
```sh
$ cargo build
```

# Cargoを使わずにビルド＆実行
```sh
# 実行ファイルを作成
$ rustc main.rs
# 実行
$ ./main
```

## 参考サイト
[Rust を始めるための資料集](https://blog-dry.com/entry/2021/01/23/141936)  
[Tour of Rust](https://tourofrust.com/)
