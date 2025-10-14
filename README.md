# Virtual Currency Tracker

一个基于 Rust 开发的实时加密货币价格追踪工具，通过 WebSocket 连接币安（Binance）交易流，命令行实时监控和展示加密货币价格信息。

## 功能特性

- 🔄 **实时数据更新**：通过 WebSocket 实时接收币安交易数据
- 📊 **多币种监控**：支持同时监控多个交易对（BTC/USDT、ETH/USDT 等）
- ⏱️ **时间窗口分析**：支持 1 小时、4 小时、1 天等不同时间窗口的价格变化统计
- 🎨 **终端 UI 显示**：彩色终端界面实时展示价格信息
- ⚡ **高性能异步**：基于 Tokio 异步运行时，性能优异

## 项目结构

```
virtual_currency_tracker/
├── src/
│   ├── main.rs                      # 程序入口
│   ├── models/
│   │   ├── mod.rs                   # 模块声明
│   │   ├── app_config.rs            # 应用配置模型
│   │   ├── trader_stream.rs         # 交易流数据模型
│   │   └── virtual_currency.rs      # 虚拟货币数据模型
│   └── trader/
│       ├── mod.rs                   # 交易模块声明
│       └── trader_manager.rs        # 交易管理器
├── config.toml                      # 配置文件
├── Cargo.toml                       # Rust 项目配置
└── Dockerfile                       # Docker 镜像构建文件
```

## 核心模块

### 数据模型

- **VirtualCurrency** (`src/models/virtual_currency.rs:4`)：虚拟货币数据结构
  - 最新成交价 (price)
  - 价格变动 (price_change)
  - 价格变动百分比 (price_change_percent)
  - 时间窗口内最高价 (high_price)
  - 时间窗口内最低价 (low_price)
  - 交易对符号 (symbol)

- **AppConfig** (`src/models/app_config.rs:5`)：应用配置
  - 时间窗口大小 (window_size)
  - 监听的币种列表 (listen_currency)
  - WebSocket URL 生成方法

### 交易管理

- **TraderManager** (`src/trader/trader_manager.rs:11`)：交易数据管理器
  - 维护交易流数据映射
  - 更新实时交易数据
  - 终端界面刷新和显示

## 依赖项

主要依赖包：

- **tokio**: 异步运行时
- **tokio-tungstenite**: WebSocket 客户端
- **serde & serde_json**: 数据序列化/反序列化
- **crossterm**: 终端 UI 控制
- **anyhow**: 错误处理
- **toml**: 配置文件解析

## 配置说明

编辑 `config.toml` 文件配置监控参数：

```toml
# 监听窗口变化：
# 1h - 1小时内价格变化
# 4h - 4小时内价格变化
# 1d - 1天内价格变化
window_size = '1h'

# 需要监听的币种
listen_currency = [
    'btcusdt',
    'ethusdt',
]
```

## 运行方式

### 本地运行

1. 确保已安装 Rust 工具链（推荐 1.83+）

2. 克隆项目并进入目录：
```bash
cd virtual_currency_tracker
```

3. 编译并运行：
```bash
cargo run --release
```

### Docker 运行

1. 构建 Docker 镜像：
```bash
docker build -t virtual-currency-tracker .
```

2. 运行容器：
```bash
docker run --rm virtual-currency-tracker
```