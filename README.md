# 随机抽选
欢迎使用随机抽选。

本程序由zhangxuan开发，于2023年11月1日启动，此为完整版本。

## 使用方法

1. 下载本程序

一般地， 你可以直接在本页面的Releases中下载最新版本。

2.设置
打开“设置”， 根据你的需求， 选择你想要的选项。

3. 开始
点击“开始”， 程序会自动开始随机抽选。  

## 从源码构建
如果你想从源码构建本程序， 你需要安装:
 - Rust (Cargo);
 - cargo-bundle 

然后， 你可以使用以下命令构建本程序:
```bash
cargo install cargo-bundle
cargo build --release
```
然后， 你可以在`target/release`中找到本程序。
