# launcher-plugin

[![crates.io version](https://img.shields.io/crates/v/launcher-plugin.svg)](https://crates.io/crates/launcher-plugin)
[![Downloads](https://img.shields.io/crates/d/launcher-plugin.svg)](https://crates.io/crates/launcher-plugin)
[![Documentation](https://docs.rs/launcher-plugin/badge.svg)](https://docs.rs/launcher-plugin)

Rust 辅助库，帮助搜索启动器插件处理 JSON-RPC 请求、生成结果，并构造常用的结果动作。库本身不启动宿主，也不绑定某个启动器的 UI 或运行时。

## 安装

```toml
[dependencies]
launcher-plugin = "0.2"
```

## 最小示例

宿主以命令行参数传入 JSON 请求。插件将单行 JSON 响应写入标准输出；日志写入标准错误，避免污染响应。

```rust
use launcher_plugin::jsonrpc::{
    JsonRpcRequest, JsonRpcResponse, JsonRpcResponseDataBuilder, Method,
};
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let argument = env::args().nth(1).ok_or("缺少 JSON-RPC 请求参数")?;
    let request = JsonRpcRequest::from(argument)?;

    match &request.method {
        Method::Query => {
            let query = request.parameter_or_default(0);
            let mut response = JsonRpcResponse::default();
            response.add_data(
                JsonRpcResponseDataBuilder::default()
                    .title(format!("搜索：{query}"))
                    .sub_title("这是一个插件结果")
                    .score(50)
                    .build(),
            );
            request.set_response(response)?;
        }
        Method::ContextMenu => request.set_response(JsonRpcResponse::default())?,
        Method::Custom(method) => eprintln!("未处理的方法：{method}"),
    }
    Ok(())
}
```

## 安全读取请求参数

```rust
if let Some(query) = request.parameter(0) {
    eprintln!("查询词：{query}");
}

let query_or_empty = request.parameter_or_default(0);
```

空数组或 `null` 参数不会导致 panic；按需选择返回 `Option` 的 `parameter()` 或带默认值的 `parameter_or_default()`。

## 生成简单结果

```rust
request.set_simple_response(
    "结果标题",
    "结果说明",
    "Images/icon.png",
    None,
)?;
```

也可以用 `JsonRpcResponseDataBuilder` 构造多条结果。可选字段包括分数、图标路径和 `JsonRpcAction`。

## 常用动作

```rust
use launcher_plugin::jsonrpc::JsonRpcAction;

let action = JsonRpcAction::change_query("下一步", true);
let action = JsonRpcAction::open_setting_dialog();
let action = JsonRpcAction::custom("copy_text", vec!["要复制的内容"]);
```

具体可用动作取决于承载插件的启动器实现。自定义动作由插件进程按 `method` 名称自行分发。

## 0.2.0 更新内容

- 请求字段支持默认值，省略 `parameters`、`settings`、`id` 或 `error` 时仍可解析；默认方法为 `query`。
- 新增 `parameter(index)` 和 `parameter_or_default(index)`，空参数或 `null` 不会引发 panic。
- 响应直接用 `serde_json` 序列化为 UTF-8 JSON，支持中文、emoji 和其他 Unicode 字符；写 stdout 的错误会返回给调用方。
- `set_simple_response()` 不再对序列化结果 `unwrap()`，并允许标题、副标题和图标使用不同字符串类型。
- Builder 实现 `Default`，`build()` 消耗 builder，避免不必要的字符串克隆。
- `JsonRpcResponse::to_json_string()` 是推荐的响应序列化 API。

## 设计约定

- stdout 是机器可读的响应通道；日志和诊断信息应写到 stderr。
- JSON 使用 UTF-8。不要手工将 Unicode 字符转换成 `\\uXXXX`。
- 请求参数可能缺失或为 `null`，优先使用安全参数访问方法。
- 插件设置和动作能力由宿主协议决定，使用前确认目标宿主支持对应字段或动作。

## 开发

```bash
cargo fmt
cargo check
```

## 许可

MIT
