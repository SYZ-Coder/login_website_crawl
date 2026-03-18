# 网站爬取项目 - 登录状态支持

## 项目概述

### 🚀 Bazel 构建部署完整指南

#### 📋 项目介绍

这是一个基于 Rust 的网站爬虫项目，专门用于爬取需要登录状态的网站。项目使用 Bazel 进行构建和部署，具有以下特点：

- 登录支持：自动处理网站登录认证
- 错误处理：完善的错误处理和重试机制
- 数据存储：模拟数据库存储爬取内容
- Web展示：通过Web界面展示爬取数据
- 配置管理：灵活的配置系统
- 数据导出功能 (export.rs)

  - 支持JSON、CSV、Excel格式导出
  - 包含完整的导出逻辑
  - 可直接使用

- 调度任务功能 (scheduler.rs)
  - 支持cron表达式调度
  - 包含任务管理功能
  - 可定时执行爬取任务

- 数据分析功能 (analysis.rs)
  - 内容分析、统计信息生成
  - 域名分析、内容类型检测
  - 完整的分析逻辑

- 爬取规则功能 (rules.rs)
  - 域名过滤、URL过滤
  - 跟随规则、提取规则
  - 灵活的爬取控制

- 性能监控功能 (monitor.rs)
  - 请求统计、响应时间监控
  - 成功率计算、性能报告
  - 实时监控能力

- 数据过滤功能 (filter.rs)
  - 内容长度过滤
  - URL过滤
  - 时间戳过滤
  - 内容关键词过滤

- 数据验证功能 (validation.rs)
  - URL验证
  - 内容验证
  - 时间戳验证
  - 综合验证

- 报告生成功能 (report.rs)
  - HTML报告生成
  - PDF报告生成（简化版）
  - 完整的报告模板

## 技术栈

- **编程语言**：Rust
- **爬虫库**：spider-rs/spider
- **构建工具**：Bazel
- **HTTP 客户端**：reqwest（用于登录请求）
- **JSON 处理**：serde_json

## 项目结构

```
D:\AI\openclaw\project_by_AI\login_website_crawl/
├── Cargo.toml          # Rust依赖配置
├── WORKSPACE          # Bazel工作空间配置
├── BUILD.bazel        # Bazel构建规则
├── README.md          # 项目说明
├── src/
│   ├── main.rs        # 主程序入口
│   ├── login.rs       # 登录功能
│   ├── crawler.rs     # 爬虫核心
│   ├── database.rs    # 数据库模拟
│   ├── config.rs      # 配置管理
│   ├── web_server.rs  # Web服务器
│   ├── export.rs      # 数据导出功能
│   ├── scheduler.rs   # 调度任务功能
│   ├── analysis.rs    # 数据分析功能
│   ├── rules.rs       # 爬取规则功能
│   ├── monitor.rs     # 性能监控功能
│   ├── filter.rs      # 数据过滤功能
│   ├── validation.rs  # 数据验证功能
│   └── report.rs      # 报告生成功能
├── tests/
│   ├── integration_test.rs  # 原始集成测试
│   └── comprehensive_test.rs  # 新增综合测试
└── web/
    └── index.html     # Web展示界面
```

## 开发步骤

1. 初始化 Rust 项目
2. 集成 spider-rs/spider 依赖
3. 实现登录功能模块
4. 配置 Bazel 构建系统
5. 编写测试用例
6. 实现增量爬取逻辑

## 使用方法

```bash
# 构建
bazel build //:main

# 运行
bazel run //:main

# 测试
bazel test //:integration_test
```

## 🔧 功能模块介绍

### 登录模块 (login.rs)

- 处理网站登录认证

- 支持CSRF令牌

- 自动重试机制

- 会话管理

### 爬虫模块 (crawler.rs)

- 使用spider-rs/spider库

- 支持增量爬取

- 错误处理和重试

- 并发控制

### 数据库模块 (database.rs)

- 内存模拟数据库

- 数据存储和查询

- 数据清空功能

### Web服务器 (web_server.rs)

- warp框架

- API接口提供

- 数据展示界面

### 配置模块 (config.rs)

- 重试参数配置
- 爬虫参数配置
- 灵活的自定义选项

### 功能模块使用示例

1. ### 数据导出

```
let exporter = export::create_exporter(export::ExportFormat::JSON, "output.json".to_string());
exporter.export(&data)?;
```

2. ### 调度任务

```
let scheduler = scheduler::create_scheduler();
scheduler.add_task(scheduler::ScheduledTask {
    name: "daily_crawl".to_string(),
    url: "https://example.com".to_string(),
    schedule: "0 0 * * *".to_string(), // 每天午夜
    enabled: true,
    last_run: None,
});
scheduler.run();
```



3. ### 数据分析

```
let analyzer = analysis::create_analyzer(data);
let analysis_result = analyzer.analyze_content();
let stats = analyzer.generate_stats(100, 85, 15, 2.5);
```



4. ### 爬取规则

```
let rules = rules::create_crawl_rules();
rules.add_allowed_domain("example.com".to_string());
rules.add_blocked_url("/admin/".to_string());
rules.add_follow_rule("/articles/".to_string(), rules::FollowAction::Allow);
```



5. ### 性能监控

```
let monitor = monitor::create_monitor();
monitor.record_request(Duration::from_secs(2), true);
let report = monitor.get_report();
```



6. ### 数据过滤

```js
let filter = filter::create_filter();
filter.add_filter(Box::new(filter::ContentLengthFilter::new(100, 10000)));
let filtered_data = filter.filter_data(&data);
```



7. ### 数据验证

```cust
let validator = validation::create_validator();
validator.add_validator(Box::new(validation::UrlValidator::new(Some("https".to_string()), vec!["example.com".to_string()])));
let results = validator.validate_data(&data);
```



8. ### 报告生成

```
let generator = report::create_report_generator(data, metrics, analysis, validation_results);
let html_report = generator.generate_html_report()?;
generator.save_report("report.html")?;
```



## 🛠️ Bazel 构建部署步骤

### 步骤 1: 安装 Bazel

****

```
# Windows
choco install bazel
# 或者从 https://bazel.build/install 下载安装
```

### 步骤 2: 初始化项目

```
# 进入项目目录
cd D:\AI\openclaw\project_by_AI\login_website_crawl

# 安装 Rust 和 Cargo（如果尚未安装）
# 从 https://rustup.rs/ 下载并安装
```

### 步骤 3: 构建 Bazel 项目

```
# 构建主程序
bazel build //:main

# 构建测试
bazel build //:integration_test

# 构建Web服务器
bazel build //:web_server
```

### 步骤 4: 运行项目

```
# 运行主程序（爬虫+Web服务器）
bazel run //:main

# 或者直接运行Rust程序
cargo run
```

### 步骤 5: 访问Web界面

打开浏览器访问：

```
http://127.0.0.1:3030
```

## 📊 详细操作指南

1. ### 构建项目

```
# 确保在项目目录下
cd D:\AI\openclaw\project_by_AI\login_website_crawl

# 构建所有目标
bazel build //:*
```

2. ### 运行爬虫

```
# 运行主程序（包含爬虫和Web服务器）
bazel run //:main

# 或者使用Rust运行
cargo run
```

3. ### 运行测试

```
# 运行所有测试
bazel test //:*

# 运行特定测试
bazel test //:integration_test
```

4. ### 清理构建

```
# 清理Bazel构建缓存
bazel clean

# 清理所有构建输出
bazel clean --expunge
```

## 🎯 配置说明

修改登录配置

在 src/main.rs 中修改：

```
let credentials = login::LoginCredentials {
    username: "your_username".to_string(),
    password: "your_password".to_string(),
    csrf_token: None,
};
```

修改爬虫配置

在 src/main.rs 中修改：

```
let crawler_config = config::CrawlerConfig {
    max_depth: 3,          // 爬取深度
    concurrency: 4,        // 并发数
    request_timeout: Duration::from_secs(30),
    user_agent: "Mozilla/5.0...".to_string(),
    retry_config: config::RetryConfig {
        max_retries: 3,
        initial_delay: Duration::from_secs(1),
        max_delay: Duration::from_secs(30),
        backoff_factor: 2.0,
    },
};
```

## 📱 Web界面功能

1. 数据展示：显示爬取的URL、内容预览和时间戳

1. 刷新按钮：实时更新数据

1. 清空按钮：清除所有数据

1. 链接跳转：点击URL在新标签页打开

## 🛡️ 错误处理机制

项目包含完善的错误处理：

- 登录失败重试：自动重试登录

- 网络请求重试：指数退避重试策略

- 状态码处理：针对不同HTTP状态码的特殊处理

- 详细日志：记录所有操作和错误信息

## 📋 新手使用指南

1. 安装依赖：确保安装了Bazel和Rust

1. 修改配置：根据目标网站修改登录凭据

1. 构建项目：运行 bazel build //:*

1. 运行项目：运行 bazel run //:main

1. 访问界面：打开浏览器访问 http://127.0.0.1:3030

## 🎉 项目特点

- ✅ Bazel构建：使用bazel bundle进行构建和测试
- ✅ 登录支持：处理需要登录的网站
- ✅ 错误处理：完善的错误处理和重试机制
- ✅ Web展示：通过JS界面展示数据
- ✅ 数据库模拟：内存存储爬取内容
- ✅ 配置灵活：可自定义各种参数



## 注意事项

- 确保目标网站允许爬取
- 遵守网站的 robots.txt 规则
- 处理登录状态的会话管理
- 实现适当的错误处理和重试机制





