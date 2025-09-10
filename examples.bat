@echo off
REM Windows 批处理示例文件 - 演示 system_notification 的各种使用方法
REM 请确保已经使用 cargo build --release 编译了项目

echo 开始演示 Windows 通知应用...
echo.

echo 1. 运行演示模式
target\release\system_notification.exe --demo
echo.

echo 2. 发送简单通知
target\release\system_notification.exe -t "简单通知" -m "这是一个简单的测试通知"
echo.

echo 3. 发送带超时的通知
target\release\system_notification.exe -t "定时通知" -m "这个通知将显示10秒" --timeout 10000
echo.

echo 4. 发送多行通知
target\release\system_notification.exe -t "多行通知" -m "第一行内容\n第二行内容\n✨ 带表情符号的第三行"
echo.

echo 5. 发送重要提醒
target\release\system_notification.exe -t "重要提醒 ⚠️" -m "这是一个重要的系统提醒\n请及时查看！" --timeout 15000
echo.

echo 演示完成！请检查 Windows 通知区域查看效果。
pause