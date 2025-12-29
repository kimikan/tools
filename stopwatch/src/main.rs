use crossterm::{
  event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
  backend::{Backend, CrosstermBackend},
  layout::{Alignment, Constraint, Direction, Layout},
  style::{Color, Style, Stylize},
  widgets::{Block, Borders, Paragraph},
  Frame, Terminal,
};
use std::{io, time::{Duration, Instant}};
use tui_big_text::{BigText, PixelSize};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // --- 1. 初始化终端 ---
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  // --- 2. 运行秒表逻辑 ---
  let start_time = Instant::now();
  let tick_rate = Duration::from_millis(100);

  loop {
    let elapsed = start_time.elapsed();

    // 绘制 UI
    terminal.draw(|f| ui(f, elapsed))?;

    // 处理退出事件 (按 'q' 键)
    if event::poll(tick_rate)? {
      if let Event::Key(key) = event::read()? {
        if let KeyCode::Char('q') = key.code {
          break;
        }
      }
    }
  }

  // --- 3. 恢复终端 ---
  disable_raw_mode()?;
  execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
  terminal.show_cursor()?;

  Ok(())
}

fn ui(f: &mut Frame, elapsed: Duration) {
  // 将屏幕分为三块：标题、中央大字、底部提示
  let chunks = Layout::default()
      .direction(Direction::Vertical)
      .constraints([
        Constraint::Length(3), // 标题
        Constraint::Min(0),    // 大字区域自动填充
        Constraint::Length(3), // 底部状态栏
      ])
      .split(f.size());

  // 1. 标题栏
  let title = Paragraph::new(" 5G 测速任务实时监控 ")
      .alignment(Alignment::Center)
      .block(Block::default().borders(Borders::ALL).fg(Color::Cyan));
  f.render_widget(title, chunks[0]);

  // 2. 中央大字 (秒表核心)
  let total_secs = elapsed.as_millis();
  let time_str = format!("{:03}", total_secs); // 显示 001, 002...

  // 创建大字组件
  let big_text = BigText::builder()
      .pixel_size(PixelSize::Full) // 使用全方块字符 █
      .style(Style::default().fg(Color::Yellow))
      .lines(vec![time_str.into()])
      .build();

  // 为了让大字居中，我们再嵌套一层垂直布局
  let area = Layout::default()
      .direction(Direction::Vertical)
      .constraints([
        Constraint::Percentage(30), // 上留白
        Constraint::Length(8),      // 大字高度
        Constraint::Min(0),
      ])
      .split(chunks[1])[1];

  f.render_widget(big_text, area);

  // 3. 底部状态栏
  let status = Paragraph::new("状态: 正在上传... | 按 'q' 退出测试")
      .style(Style::default().bg(Color::DarkGray))
      .alignment(Alignment::Left);
  f.render_widget(status, chunks[2]);
}