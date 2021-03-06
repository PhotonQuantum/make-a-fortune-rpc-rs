# 愿无可奉告安息

由于法律原因，无可奉告作者已被约谈并决定停止服务。其服务器将在 2021 年 04 月 12 日 21:00 关闭。愿无可奉告安息。

因此，本后端此后将无实际意义。本仓库已被归档。

# 闷声发财 RPC - Rust

[English Readme](README.md)

`闷声发财 RPC - Rust` 是 [闷声发财](https://github.com/skyzh/make-a-fortune) 后端 RPC 服务的 Rust 实现，
当前支持 [上海交通大学 - 无可奉告](http://wukefenggao.cn/) 树洞社区。

前往 [闷声发财](https://github.com/skyzh/make-a-fortune) 了解前端与 Python 后端实现的详情。

## 快速上手

- `cargo build --release`
- 如有需要，可通过 `Rocket.toml` 修改配置
- `./target/release/fortune-rpc-rs`

## 已部署实例

本实例已自动部署到 [fly.io](https://fly.io) 平台。

`https://fortune.fly.dev`

同时，本实例也部署在了国内服务器上。注意其带宽很小，可能会偶发断线。

`https://fortune.lightquantum:9108`

在闷声发财登录页面的 `RPC 服务器` 设置处输入该 URL 即可使用。

## 隐私警告

如同任何「闷声发财」的 RPC 实现，连接本 RPC 实现使用「闷声发财」时，您的所有消息（包括用户令牌、发送与接收的内容）也都将通过 RPC 服务器传输。

使用第三方 RPC 实例时，请务必确保您信任 RPC 服务器的提供者。

## 许可协议

本项目以 [MIT License](LICENSE.md) 许可发布。

```text
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
```
