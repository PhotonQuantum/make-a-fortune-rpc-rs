# Make A Fortune Rust RPC

[中文介绍](README_zh.md)

`make-a-fortune-rpc-rs` is an alternative RPC backend for [make-a-fortune](https://github.com/skyzh/make-a-fortune)
targeted at [SJTU No Comment](http://wukefenggao.cn/).

Please refer to [make-a-fortune](https://github.com/skyzh/make-a-fortune) for more information about the frontend and Python backend.

## Get Started
- `cargo build --release`
- edit `Rocket.toml` to modify settings if needed.
- `./target/release/fortune-rpc-rs`

## Hosted Instance

This RPC backend is automatically deployed to [fly.io](https://fly.io)

`https://fortune.fly.io`

Enter this URL in the login page of `make-a-fortune` and you are good to go.

## Privacy Warning

All your data (including your messages and personal token) is transferred through 
the RPC server when using `make-a-fortune` together with this backend,
just like any other RPC backend for `make-a-fortune`.

Use third-party `make-a-fortune-rpc-rs` instance if only you fully trust its owner.

## License

This project is licensed under [MIT License](LICENSE.md).

```text
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
```
