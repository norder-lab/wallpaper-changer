# README

## 1. 概要

指定ディレクトリ内の画像ファイルをランダムに選択肢, OSの壁紙に設定します.  

## 2. 開発環境

### 2.1. システム環境

OS: Windows11  
Terminal: Wezterm  

### 2.2. 依存環境

- [wallpaper for docs](https://docs.rs/wallpaper/latest/wallpaper/#example)

## 3. 推奨環境

OS: Windows11  
※ Rustの `wallpaper` クレートを利用しているため, 対応OSはそちらを参照.  

## 3. 使用方法

### 3.1. 現在の対象ディレクトリを確認

```shell
wallpaper get
```

### 3.2. 対象ディレクトリを変更

```shell
wallpaper set <directory path>
```

### 3.3. 壁紙をランダムに変更

```shell
wallpaper random
```

### 3.4. ヘルプ

```shell
wallpaper help
```

