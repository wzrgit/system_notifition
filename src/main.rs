use notify_rust::Notification;
use std::{thread, time::Duration};
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("system_notification")
        .version("0.1.0")
        .author("System Notification App")
        .about("Windows 系统通知应用 - 用 Rust 编写的通知工具")
        .arg(
            Arg::new("title")
                .short('t')
                .long("title")
                .value_name("TITLE")
                .help("通知标题")
        )
        .arg(
            Arg::new("message")
                .short('m')
                .long("message")
                .value_name("MESSAGE")
                .help("通知内容")
        )
        .arg(
            Arg::new("timeout")
                .long("timeout")
                .value_name("MILLISECONDS")
                .help("通知持续时间（毫秒）")
                .value_parser(clap::value_parser!(i32))
        )
        .arg(
            Arg::new("demo")
                .short('d')
                .long("demo")
                .help("运行演示模式，显示多个示例通知")
                .action(clap::ArgAction::SetTrue)
        )
        .get_matches();

    println!("🔔 Windows 通知应用启动中...");
    
    // 检查当前操作系统
    check_system_compatibility();
    
    // 根据命令行参数决定运行模式
    if matches.get_flag("demo") {
        println!("🎯 运行演示模式...");
        run_demo_mode();
    } else if let (Some(title), Some(message)) = (matches.get_one::<String>("title"), matches.get_one::<String>("message")) {
        println!("📝 发送自定义通知...");
        let timeout = matches.get_one::<i32>("timeout").copied().unwrap_or(5000);
        send_custom_notification(title, message, timeout);
    } else {
        println!("🎯 未指定参数，运行默认演示模式...");
        run_demo_mode();
    }
    
    println!("✅ 应用运行完成。在 Windows 上，请检查右下角的通知区域。");
}

fn check_system_compatibility() {
    if cfg!(target_os = "windows") {
        println!("🪟 检测到 Windows 系统，正在发送通知...");
    } else {
        println!("⚠️  注意：此应用专为 Windows 设计。");
        println!("📍 当前运行在 {} 系统上，将尝试发送通知（可能需要安装通知守护程序）...", 
                std::env::consts::OS);
    }
}

fn send_custom_notification(title: &str, message: &str, timeout: i32) {
    match create_notification(title, message, timeout).show() {
        Ok(_) => {
            println!("✅ 自定义通知已成功发送！");
            println!("📋 标题: {}", title);
            println!("💬 内容: {}", message);
            println!("⏰ 持续时间: {}ms", timeout);
        }
        Err(e) => {
            eprintln!("❌ 发送自定义通知时出错: {}", e);
            print_troubleshooting_info();
        }
    }
}

fn run_demo_mode() {
    println!("🚀 开始演示多种通知类型...");
    
    // 演示通知 1：欢迎信息
    demo_notification(
        "🎉 欢迎使用",
        "欢迎使用 Rust 通知应用！\n这是第一个演示通知。",
        3000,
        "欢迎通知"
    );
    
    thread::sleep(Duration::from_secs(2));
    
    // 演示通知 2：功能介绍
    demo_notification(
        "✨ 功能特性",
        "🔹 支持自定义标题和内容\n🔹 可设置持续时间\n🔹 支持表情符号和多行文本\n🔹 跨平台兼容",
        5000,
        "功能介绍"
    );
    
    thread::sleep(Duration::from_secs(3));
    
    // 演示通知 3：命令行使用
    demo_notification(
        "💻 命令行使用",
        "使用示例：\n📝 system_notification -t \"标题\" -m \"消息\"\n⏰ system_notification --timeout 8000 -d",
        6000,
        "使用说明"
    );
    
    thread::sleep(Duration::from_secs(2));
    
    // 演示通知 4：结束提示
    demo_notification(
        "🎯 演示完成",
        "所有演示通知已发送完毕！\n感谢使用 Rust 通知应用。\n\n🔗 使用 --help 查看更多选项",
        8000,
        "演示结束"
    );
}

fn demo_notification(title: &str, message: &str, timeout: i32, demo_type: &str) {
    match create_notification(title, message, timeout).show() {
        Ok(_) => {
            println!("✅ {} 已发送！", demo_type);
        }
        Err(e) => {
            eprintln!("❌ 发送 {} 时出错: {}", demo_type, e);
            if demo_type == "欢迎通知" {
                print_troubleshooting_info();
            }
        }
    }
}

fn create_notification(title: &str, body: &str, timeout_ms: i32) -> Notification {
    let mut notification = Notification::new();
    notification
        .summary(title)
        .body(body)
        .timeout(timeout_ms);
    
    // 在 Windows 上使用系统图标
    if cfg!(target_os = "windows") {
        notification.icon("info");
    }
    
    notification
}

fn print_troubleshooting_info() {
    eprintln!();
    eprintln!("🔧 故障排除信息：");
    eprintln!("1. 🪟 在 Windows 上，请确保系统通知已启用");
    eprintln!("2. 🐧 在 Linux 上，请确保安装了通知守护程序（如 notify-osd 或 dunst）");
    eprintln!("3. 🏢 某些企业环境可能会限制通知功能");
    eprintln!("4. 🔐 如果问题持续存在，请尝试以管理员权限运行");
    eprintln!("5. 📖 使用 --help 查看命令行选项");
    eprintln!();
}
