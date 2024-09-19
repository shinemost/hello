use std::fs::read_to_string;

use anyhow::Result;

fn main() -> Result<()> {
    let html = render()?;
    println!("{}", html);
    Ok(())
}

fn render() -> Result<String> {
    let file = std::env::var("MARKDOWN")?;
    let source = read_to_string(file)?;
    Ok(source)
}

// 如果你想要设计自己的错误类型，同时给调用者提供具体的信息时，就使用 thiserror，例如当你在开发一个三方库代码时。
// 如果你只想要简单，就使用 anyhow，例如在自己的应用服务中。
