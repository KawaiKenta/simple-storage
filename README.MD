# simple-storage
Rustで作成したシンプルなファイル保存ライブラリです。

## コマンド例
```bash
$ curl -X GET 'localhost:3000/download?filename=Dockerfile'
```
```bash
$ curl -X PUT 'localhost:3000/upload?filename=Cargo.toml' --data-binary @Cargo.toml
```