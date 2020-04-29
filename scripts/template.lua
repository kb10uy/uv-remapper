function Initialize(loader)
  --[[
    使用する画像をここで読み込みます。厳密には読み込みを予約します。

    loader:load(key, filename)
    ===========================================================
    key に後で参照する文字列(一意ならば任意)、 filename に読み込む画像ファイル名を
    指定します。
  ]]

  -- loader:load("image_key", "filename.png");
  return loader;
end

function Run(queue)
  --[[
    パッチ操作をここに書きます。

    UVR.create_range(x, y, w, h) -> Range (table)
    ===========================================================
    正規化された座標系で長方形の範囲を表すオブジェクトを作成します。
    x, y, w, h は全て [0, 1] の範囲の値を指定してください。

    UVR.create_lattice(w, h) -> Lattice (float[][][])
    ===========================================================
    UV マップを変形するラティスを作成します。
    w, h にはそれぞれ横/縦のラティスのブロック数を指定するため、実際に UV 値を
    指定できる要素数はそれぞれに +1 したものとなります。
    また、 Lua は 1-based indexing なので注意してください。
    (e.g. (10, 10) で作成したら x, y 共に 1 ～ 11 のインデックス)
    返り値は float の 3 次元配列のため、 Lua で普通に操作することができます。
    インデックス順は lattice[y][x][0 for U, 1 for V] となります。

    queue:push(image, range, mask, lattice) -> nil
    ===========================================================
    パッチを実行します。厳密には実行を予約します。
    image (必須), mask (任意) は上記 Initialize 関数で登録した参照名を指定します。
    mask を使わない場合は nil を指定してください。
    range (必須) はどこにパッチするかを Range オブジェクトで指定します。
    lattice(任意) で変形を指定できます。使わない場合は nil を指定してください。
  ]]

  --[[
    local range = UVR.create_range(0, 0, 1, 1);
    local lattice = UVR.create_lattice(10, 10);
    queue:push("image_key", range, "mask_key", lattice);
  ]]
  return queue;
end
