# マンデルブロ集合

## 並列処理をしない場合

まずは並列処理を行わない場合の処理結果を見てみる。

```bash
# コンパイル時に最適化を行う
cargo build --release

# 実行時間を計測する
time target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20

# 結果は下記となった
target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20
    3.59s user
    0.04s system
    93% cpu
    3.874 total
```

出力結果は下記となる。

![](mandel.png)

## 並列処理をした場合

```bash
# コンパイル時に最適化を行う
cargo build --release

# 実行時間を計測する
time target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20

# 結果は下記となった
target/release/mandelbrot mandel.png 4000x3000 -1.20,0.35 -1,0.20
    3.75s user
    0.02s system
    251% cpu
    1.499 total
```

## 結果

total 時間を見ると、マンデルブロ集合の計算部分の時間を大体 4 分の 1 にすることができた
