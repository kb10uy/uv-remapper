# uv-remapper
[キッシュちゃんパンツパッチ](https://github.com/TenteEEEE/quiche_pantie_patch) を Lua でお手軽に

* 🚗 Considerably Fast
* 🙆 Relatively Easy to Use
* 🤖 Not well Tested

## 概要
自作のモデル用に キッシュちゃんパンツパッチ の patcher を書こうとしたんですが、 Numpy がよくわからなかったので Rust と Lua でそれっぽいものを自作しました。以上。

## 使用方法
0. uv-remapper をパスの通っている場所に配置します。 `cargo install --path .` を使うのが楽ですが……
1. 元画像を用意します。
2. リマッピングスクリプトを書きます。 `scripts/template.lua` をコピーして作ると楽です。 `scripts` ディレクトリの他のスクリプトも参考にしてください。
3. シェル/コマンドプロンプトで次を実行します:
    - `uv-remapper <スクリプト名> <出力画像ファイル名>`


## ライセンス
* このプログラム自体は [Apache License 2.0](https://www.apache.org/licenses/LICENSE-2.0) でライセンスされています。
* 出力した画像については AL2.0 は適用されません。元画像のライセンスを参照してください。
