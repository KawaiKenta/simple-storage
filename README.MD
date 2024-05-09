# simple-storage
Rustで作成したシンプルなファイル保存ライブラリです。

## コマンド例
download
```bash
$ curl -X GET 'localhost:3000/download?filename=Cargo.toml'
```
upload
```bash
curl -X POST -F file=@Cargo.toml 'localhost:3000/upload'
```
list uploaded files
```bash
$ curl -X GET 'localhost:3000/list'
```