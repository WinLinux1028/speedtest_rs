# 特徴
- HTTPで通信します

# 使い方(サーバー)
```
cargo run -p server --release -- 127.0.0.1:4545
```
エンドポイントは`http://127.0.0.1:4545/`になります

# 使い方(クライアント)
```
cargo run -p client --release
```