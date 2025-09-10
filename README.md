# System Notification

一个用 Rust 编写的 Windows 系统通知应用程序。

## 功能特性

- 🪟 专为 Windows 系统设计，完全支持 Windows 通知区域
- 🔔 支持发送系统通知到 Windows 通知区域（右下角）
- 🎨 支持多行文本和表情符号
- ⏰ 可自定义通知持续时间
- 💻 支持命令行参数，可编程使用
- 🎯 内置演示模式，展示各种通知类型
- 🔧 包含完整的错误处理和故障排除信息
- 🖥️ 跨平台兼容（主要针对 Windows，在其他系统上会给出适当提示）

## 安装和编译

确保已安装 Rust 和 Cargo（推荐使用 Rust 1.89.0 或更高版本）：

```bash
# 克隆项目
git clone <repository-url>
cd system_notifition

# 编译项目（发布版本）
cargo build --release

# 或者直接运行（开发版本）
cargo run
```

## 使用方法

### 1. 演示模式（默认）

```bash
# 运行默认演示，显示多个示例通知
cargo run

# 或显式运行演示模式
cargo run -- --demo
```

### 2. 自定义通知

```bash
# 发送自定义通知
cargo run -- -t "我的标题" -m "我的消息内容"

# 带超时时间的自定义通知（毫秒）
cargo run -- -t "重要提醒" -m "这是一个重要消息" --timeout 10000

# 多行消息示例
cargo run -- -t "详细信息" -m "第一行内容\n第二行内容\n✨ 支持表情符号"
```

### 3. 命令行选项

```bash
# 查看所有可用选项
cargo run -- --help

# 查看版本信息
cargo run -- --version
```

#### 可用参数：

- `-t, --title <TITLE>`: 设置通知标题
- `-m, --message <MESSAGE>`: 设置通知内容
- `--timeout <MILLISECONDS>`: 设置通知持续时间（毫秒）
- `-d, --demo`: 运行演示模式
- `-h, --help`: 显示帮助信息
- `-V, --version`: 显示版本信息

### 4. 在 Windows 上的使用体验

在 Windows 系统上运行时，您将看到：

1. **通知外观**：
   - 通知会出现在屏幕右下角
   - 使用 Windows 原生通知样式
   - 包含系统图标和自定义内容

2. **交互方式**：
   - 通知会在指定时间后自动消失
   - 可以点击通知进行交互（如果应用支持）
   - 会在 Windows 通知中心保留历史记录

3. **系统集成**：
   - 遵循 Windows 通知设置
   - 支持免打扰模式
   - 与系统主题保持一致

## 故障排除

### Windows 系统

如果通知无法正常显示：

1. **检查通知设置**：
   - 打开设置 → 系统 → 通知和操作
   - 确保通知功能已启用
   - 检查应用通知权限

2. **免打扰模式**：
   - 确保未开启专注助手/免打扰模式
   - 检查时间设置是否影响通知显示

3. **权限问题**：
   - 尝试以管理员权限运行
   - 检查防火墙或安全软件设置

### 其他系统

- **Linux**：需要安装通知守护程序（如 `notify-osd`、`dunst` 或 `notification-daemon`）
- **macOS**：系统会尝试使用原生通知，但可能需要额外权限
- **企业环境**：某些企业策略可能会限制通知功能

## 技术细节

### 依赖项

- `notify-rust` (4.11.7): 跨平台通知库
- `clap` (4.5.47): 命令行参数解析

### 编译要求

- Rust 1.89.0+ (2024 edition)
- Cargo 包管理器
- 在 Windows 上可能需要 Visual Studio Build Tools

### 架构设计

- 使用条件编译优化不同平台的行为
- 模块化设计便于扩展新功能
- 完整的错误处理确保程序稳定性

## 开发和扩展

### 修改通知内容

编辑 `src/main.rs` 文件中的相关函数：

- `run_demo_mode()`: 修改演示通知
- `create_notification()`: 调整通知参数
- `print_troubleshooting_info()`: 更新故障排除信息

### 添加新功能

常见扩展方向：

- 添加通知图标支持
- 实现通知点击回调
- 添加配置文件支持
- 集成系统托盘功能

## 示例用法

```bash
# 快速测试
cargo run -- -t "测试" -m "Hello, World!"

# 长时间显示
cargo run -- -t "重要通知" -m "请注意查看" --timeout 15000

# 演示所有功能
cargo run -- --demo
```

## 许可证

请参考项目许可证文件。

## 贡献

欢迎提交 Issue 和 Pull Request 来改进这个项目！
