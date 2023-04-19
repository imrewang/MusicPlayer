use std::io;
use std::time::Duration; //表示时间跨度的 Duration 类型，通常用于系统超时。

use app::App;
//Crossterm 为 Windows 和 UNIX 系统提供清除、事件（输入）处理、样式、光标移动和终端操作。
use crossterm::terminal::{disable_raw_mode, enable_raw_mode}; //terminal终端模块提供与终端一起工作的功能。
use exitfailure::ExitFailure; //Some newtype wrappers to help with using
use handler::event::handle_event;
use rodio::OutputStream; //音频播放库。
use tui::backend::CrosstermBackend; //tui 是一个用于构建丰富的终端用户界面和仪表板的库。
use tui::Terminal;
use view::handle_theme;

mod app;
mod commands;
mod config;
mod file_ops;
mod handler;
mod music;
mod utils;
mod view;

fn main() -> Result<(), ExitFailure> {
    let init_config = config::init()?;
    let theme = handle_theme(init_config.theme);

    // Initialize terminal
    enable_raw_mode()?; //Enables raw mode.
    let stdout = io::stdout(); //当前进程的全局标准输出流的句柄。
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;

    // Initialize App state
    let (_stream, stream_handle) = OutputStream::try_default()?; //如果实现者实现了 Default，那么这将返回 Option::Some(Default::default())。否则返回 Option::None。
    let mut app = App::new(&mut terminal, &init_config.music_database, stream_handle)?;

    loop {
        app.update_window_height();
        view::draw(&mut app, &theme)?;

        //事件模块提供读取键盘、鼠标和终端调整大小事件的功能。
        if crossterm::event::poll(Duration::from_millis(100))? {
            //poll检查是否有可用的事件。如果事件可用则返回 Ok(true)，否则返回 Ok(false)。Ok(true) 保证后续对读取函数的调用不会阻塞。
            //from_millis从指定的毫秒数创建一个新的 Duration。
            if !handle_event(&mut app, &init_config.music_database)? {
                break;
            };
        }
        app.check_music_list();
    }

    disable_raw_mode()?;
    /*启用raw_mode时将设置这些模式：

    输入不会被转发到屏幕
    按下回车键时不会处理输入
    输入不会被行缓冲（输入逐字节发送到输入缓冲区）
    终端驱动程序不会处理退格键和 CTRL+C 等特殊键
    换行符将不会被处理因此 println! 不能用，用write！ 反而

    可以使用 enable_raw_mode 和 disable_raw_mode 函数启用/禁用原始模式。
    */

    Ok(())
}
