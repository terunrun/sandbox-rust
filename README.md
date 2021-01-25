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

# プロジェクトのビルド
$ cargo build
# プロジェクトの実行
$ cargo run
# プロジェクトのテスト
$ cargo test
# プロジェクトのドキュメントのビルド
$ cargo doc
# ライブラリをcrates.ioに公開
$ cargo publish
```

5. お好みでエディタサポートツールを導入する。
> [Rust support for Visual Studio Code](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)など。

## 参考サイト
[Rust を始めるための資料集](https://blog-dry.com/entry/2021/01/23/141936)
