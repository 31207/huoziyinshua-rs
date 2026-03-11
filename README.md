# huoziyinshua-rs

Rust版本的“活字印刷”工具，用于从中文文本生成音频，通过拼音映射到音频样本，并支持各种音频变换。使用了电棍otto的音源。

## 功能

- 从文本生成音频数据
- 支持原声大碟替换特定短语
- 音频变换：调整音量、反转、失真、回声、平滑、变速

## 安装

确保安装了Rust。然后克隆仓库：

```bash
git clone <repo-url>
cd huoziyinshua-rs
cargo build --release
```

## 使用

### CLI

运行CLI生成音频：

```bash
cargo run
```

这会生成output.wav文件。

### 库

在你的Cargo.toml中添加：

```toml
[dependencies]
huoziyinshua_rs = { path = "huoziyinshua_rs" }
```

然后在代码中使用：

```rust
use huoziyinshua_rs::Huoziyinshua;

let mut huoziyinshua = Huoziyinshua::new("./sources")?;
huoziyinshua.generate("你的文本", true)?;
huoziyinshua.save_wav("./output.wav")?;
```

## 示例

见 `huoziyinshua_rs_cli/src/main.rs`

## 依赖
- symphonia: 音频解码库-
- hound: WAV文件处理
- pinyin: 中文转拼音
- 其他见 `Cargo.toml`

## 许可证

MIT