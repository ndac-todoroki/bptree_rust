# B+-木

B+-treeを実装してみたもの

どうせならってことでRustで書いてみた．所有権とかRefCellのあたりが特に難しかった

## 動かし方

0. (rustupで)Rustを導入する
0. `rustup install nightly` (nightly版の導入)(unstableなAPIを使っているため)
0. このディレクトリに入り `rustup override nightly` (このディレクトリだけnightlyをつかう設定)
0. `cargo run` で `main.rs` の中身を実行可能

