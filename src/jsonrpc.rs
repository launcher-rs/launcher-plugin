use crate::error::LauncherError;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// JsonRPC 请求数据
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    pub method: Method,
    pub parameters: Vec<Option<String>>,
    pub settings: serde_json::Value,
    pub id: i32,
    pub error: serde_json::Value,
}

/// JsonRPC 请求方法
#[derive(Debug)]
pub enum Method {
    /// 查询方法
    Query,
    /// 设置菜单方法
    ContextMenu,
    /// 自定义方法
    /// #[serde(untagged)] 可以让 method":{"custom":"copy"} 变成 method":"copy"
    Custom(String),
}

/// 自定义序列化
impl Serialize for Method {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Method::Query => serializer.serialize_str("query"),
            Method::ContextMenu => serializer.serialize_str("context_menu"),
            Method::Custom(s) => serializer.serialize_str(s),
        }
    }
}

/// 自定义反序列化
impl<'de> Deserialize<'de> for Method {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "query" => Ok(Method::Query),
            "context_menu" => Ok(Method::ContextMenu),
            _ => Ok(Method::Custom(s)),
        }
    }
}

impl JsonRpcRequest {
    /// 将JsonRPC请求反序列化
    pub fn from(argument: &String) -> Result<Self, LauncherError> {
        let input = serde_json::from_str::<JsonRpcRequest>(argument)?;
        Ok(input)
    }

    /// 获取第一个参数
    pub fn get_parameter(&self) -> String {
        self.parameters[0].clone().unwrap_or("".to_string())
    }

    /// 设置响应
    pub fn set_response(&self, response: JsonRpcResponse) -> Result<(), LauncherError> {
        println!("{}", response.to_unicode_string()?);
        Ok(())
    }

    /// 设置简单响应
    pub fn set_simple_response<S: Into<String>>(
        &self,
        title: S,
        sub_title: S,
        icon_path: S,
        action: Option<JsonRpcAction>,
    ) -> Result<(), LauncherError> {
        let mut query_data = JsonRpcResponse::new();
        let item = JsonRpcResponseDataBuilder::new()
            .title(title)
            .sub_title(sub_title)
            .icon_path(icon_path)
            .json_rpcaction(action)
            .build();

        query_data.add_data(item);
        let response = query_data.to_unicode_string().unwrap();
        tracing::info!("simple_response响应数据: {}", response);

        println!("{}", response);
        Ok(())
    }

    /// 转换为字符串，方便调试
    pub fn to_string(&self) -> Result<String, LauncherError> {
        Ok(serde_json::to_string(&self)?)
    }
}

/// 点击后触发操作
/// 有些操作，没有反应【不知道咋回事】
/// 官方API:<https://www.flowlauncher.com/docs/#/json-rpc?id=api-list>
#[derive(Debug, Serialize, Clone)]
pub struct JsonRpcAction {
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "parameters")]
    pub parameters: Vec<String>,
}

impl JsonRpcAction {
    /// 自定义动作，需要自己手动处理响应
    /// method: 自定义名称
    pub fn custom<S: Into<String>>(method: S, parameters: Vec<S>) -> Self {
        Self {
            method: method.into(),
            parameters: parameters.into_iter().map(|s| s.into()).collect(),
        }
    }

    /// 官方API: change flow launcher query
    /// 更改流程启动器查询
    pub fn change_query<S: Into<String>>(query: S, requery: bool) -> Self {
        Self {
            method: "Flow.Launcher.ChangeQuery".to_string(),
            parameters: vec![query.into(), requery.to_string()],
        }
    }

    /// 官方API: restart Flow Launcher
    /// 重启 Flow Launcher
    pub fn restart_app() -> Self {
        Self {
            method: "Flow.Launcher.RestartApp".to_string(),
            parameters: vec![],
        }
    }

    /// 官方API:关闭 Flow Launcher
    pub fn close_app() -> Self {
        Self {
            method: "Flow.Launcher.CloseApp".to_string(),
            parameters: vec![],
        }
    }

    /// 官方API:Save everything, all of Flow Launcher and plugins' data and settings
    /// 保存所有内容，包括 Flow Launcher 和插件的所有数据和设置
    pub fn save_app_all_setings() -> Self {
        Self {
            method: "Flow.Launcher.SaveAppAllSettings".to_string(),
            parameters: vec![],
        }
    }

    /// 官方API: 检查更新
    pub fn check_for_new_update() -> Self {
        Self {
            method: "Flow.Launcher.CheckForNewUpdate".to_string(),
            parameters: vec![],
        }
    }

    /// 官方API: run shell commands
    /// 运行shell命令
    pub fn shell_run<S: Into<String>>(cmd: S) -> Self {
        Self {
            method: "Flow.Launcher.ShellRun".to_string(),
            parameters: vec![cmd.into()],
        }
    }

    /// 官方API:隐藏 Flow Launcher
    pub fn hide_app() -> Self {
        Self {
            method: "Flow.Launcher.HideApp".to_string(),
            parameters: vec![],
        }
    }

    /// 官方API:显示 Flow Launcher
    pub fn show_app() -> Self {
        Self {
            method: "Flow.Launcher.ShowApp".to_string(),
            parameters: vec![],
        }
    }

    /// 官方API: show messagebox
    /// 显示消息框
    pub fn show_msg<S: Into<String>>(title: S, sub_title: S, ico_path: S) -> Self {
        Self {
            method: "Flow.Launcher.ShowMsg".to_string(),
            parameters: vec![title.into(), sub_title.into(), ico_path.into()],
        }
    }

    /// 官方API: 获取当前语言的翻译
    pub fn get_translation() -> Self {
        Self {
            method: "Flow.Launcher.GetTranslation".to_string(),
            parameters: vec![],
        }
    }
    /// 官方API: 打开设置对话框
    pub fn open_setting_dialog() -> Self {
        Self {
            method: "Flow.Launcher.OpenSettingDialog".to_string(),
            parameters: vec![],
        }
    }

    /// 官方API: 获取所有插件
    pub fn get_all_plugins() -> Self {
        Self {
            method: "Flow.Launcher.GetAllPlugins".to_string(),
            parameters: vec![],
        }
    }

    /// 官方API: 启动加载动画
    pub fn start_loading_bar() -> Self {
        Self {
            method: "Flow.Launcher.StartLoadingBar".to_string(),
            parameters: vec![],
        }
    }

    /// 官方API: 关闭加载动画
    pub fn stop_loading_bar() -> Self {
        Self {
            method: "Flow.Launcher.StopLoadingBar".to_string(),
            parameters: vec![],
        }
    }

    /// 官方API: 重新加载插件数据
    pub fn reload_all_plugin_data() -> Self {
        Self {
            method: "Flow.Launcher.ReloadAllPluginData".to_string(),
            parameters: vec![],
        }
    }
}

/// JsonRPC 响应数据
#[derive(Debug, Serialize)]
pub struct JsonRpcResponseData {
    #[serde(rename = "Title")]
    pub title: String,
    #[serde(rename = "SubTitle")]
    pub sub_title: String,
    /// 排名....
    #[serde(rename = "score")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
    #[serde(rename = "IcoPath")]
    pub ico_path: String,
    #[serde(rename = "JsonRPCAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_rpcaction: Option<JsonRpcAction>,
}

/// JsonRPC 响应数据
#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
    #[serde(rename = "result")]
    pub result: Vec<JsonRpcResponseData>,
}

impl JsonRpcResponse {
    pub fn new() -> Self {
        Self { result: Vec::new() }
    }
    /// 添加响应数据
    pub fn add_data(&mut self, data: JsonRpcResponseData) {
        self.result.push(data);
    }

    /// 有中文会报错，使用unicode转义
    pub fn to_unicode_string(&self) -> Result<String, LauncherError> {
        let data = serde_json::to_string(&self)?;
        Ok(self.escape_non_ascii(&data))
    }

    /// 将中文转换为 unicode
    fn escape_non_ascii(&self, s: &str) -> String {
        let mut escaped = String::with_capacity(s.len());
        for ch in s.chars() {
            if ch.is_ascii() {
                escaped.push(ch);
            } else {
                let code = ch as u32;
                escaped.push_str(&format!("\\u{:04x}", code));
            }
        }
        escaped
    }
}

/// JsonRpcResponse 构建器
pub struct JsonRpcResponseDataBuilder {
    /// 标题
    pub title: String,
    /// 副标题
    pub sub_title: String,
    /// 排名分数,0-100,分数越大越靠前
    pub score: Option<i32>,
    /// 显示图标
    pub ico_path: String,
    /// 点击触发操作
    pub json_rpcaction: Option<JsonRpcAction>,
}
impl JsonRpcResponseDataBuilder {
    pub fn new() -> JsonRpcResponseDataBuilder {
        Self {
            title: "".to_string(),
            sub_title: "".to_string(),
            score: None,
            ico_path: "".to_string(),
            json_rpcaction: None,
        }
    }
    /// 设置标题
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = title.into();
        self
    }

    /// 设置副标题
    pub fn sub_title<S: Into<String>>(mut self, sub_title: S) -> Self {
        self.sub_title = sub_title.into();
        self
    }

    /// 设置Score
    pub fn score(mut self, score: i32) -> Self {
        self.score = Some(score);
        self
    }

    /// 设置图标
    pub fn icon_path<S: Into<String>>(mut self, ico_path: S) -> Self {
        self.ico_path = ico_path.into();
        self
    }

    /// 设置点击触发执行操作
    pub fn json_rpcaction(mut self, json_rpcaction: Option<JsonRpcAction>) -> Self {
        self.json_rpcaction = json_rpcaction;
        self
    }

    /// 构建JsonRpcResponseData 响应数据
    pub fn build(&self) -> JsonRpcResponseData {
        JsonRpcResponseData {
            title: self.title.clone(),
            sub_title: self.sub_title.clone(),
            score: self.score,
            ico_path: self.ico_path.clone(),
            json_rpcaction: self.json_rpcaction.clone(),
        }
    }
}
