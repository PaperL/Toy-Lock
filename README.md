# Toy-Lock
> A simple encryption algorithm implemented in Rust

- 一个简单的按位胡搞对称加密算法
- 输入输出格式为 `i64`
- ~~不保证正确性~~
- 适用场景
  - 需要一个将纯数字用户 ID 加密的对称加密算法
  - 密文需要方便用户手动输入，故为比 ID 长度多 1 位的纯数字串
  - 需要保证强度

## 使用方法

- 使用 `cargo build` 或 `cargo build --release` 指令编译
- 使用 `cargo run` 或 `cargo run --release` 指令编译运行
  - 带参数运行指令形如 `cargo run --release -- argA argB`
- CLI 示例：

```shell
$ ./toy_lock # 无参数直接运行，程序提供文字提示
Lock or Unlock [L/u]:
Input plaintext:  12345
Ciphertext:       189390
$ ./toy_lock_110 l # 带参数运行，程序输入输出为简洁格式
3546
51321
```

- 参数说明
  - `l`：加密
  - `u`：解密
  - `debug`：输出调试信息（若干个算法过程参数）

