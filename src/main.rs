use std::{
    io::{self, stdout},
    time::Duration,
};
use chrono::Local;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, BorderType, Paragraph},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Main loop
    let tick_rate = Duration::from_millis(500);
    loop {
        terminal.draw(|f| {
             let size = f.size();
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(20),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                ])
                .split(size);
            let time = Local::now().format("%H:%M:%S").to_string();

            let time_text = format_time(&time);

             let paragraph = Paragraph::new(time_text)
                .block(Block::default().borders(ratatui::widgets::Borders::ALL).border_type(BorderType::Rounded))
                 .alignment(Alignment::Center);
            f.render_widget(paragraph, layout[1]);


        })?;

        if event::poll(tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || (key.code == KeyCode::Esc || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL)))
                {
                    break;
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn format_time(time: &str) -> Text {
    let mut text = Text::from("");
    let hour = &time[0..2];
    let minutes = &time[3..5];
    let seconds = &time[6..8];

    let large_hour = format_large_digit(hour);
    let large_minutes = format_large_digit(minutes);
    let large_seconds = format_large_digit(seconds);

    // Determine the maximum number of lines
    let max_lines = large_hour.len().max(large_minutes.len()).max(large_seconds.len());

    for i in 0..max_lines {
        let mut line = Line::from("");

        // Add hour part (if available)
        if i < large_hour.len() {
            line.spans.extend(large_hour[i].spans.clone());
        }

        // Add space in the middle
        line.spans.push(Span::raw("   "));

        // Add minutes part (if available)
        if i < large_minutes.len() {
            line.spans.extend(large_minutes[i].spans.clone());
        }

        // Add space in the middle
        line.spans.push(Span::raw("   "));

        // Add seconds part (if available)
        if i < large_seconds.len() {
            line.spans.extend(large_seconds[i].spans.clone());
        }

        text.lines.push(line);
    }

    text
}

fn format_large_digit(digit: &str) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    let mut char_lines: Vec<String> = Vec::new();


    for c in digit.chars() {
        let char_lines_for_c = match c {
             '0' => vec![
                " ██████ ".to_string(),
                "█      █".to_string(),
                "█      █".to_string(),
                "█      █".to_string(),
                "█      █".to_string(),
                "█      █".to_string(),
                " ██████ ".to_string(),
            ],
            '1' => vec![
                "   ██   ".to_string(),
                "  ███   ".to_string(),
                "   ██   ".to_string(),
                "   ██   ".to_string(),
                 "   ██   ".to_string(),
                 "   ██   ".to_string(),
                " █████  ".to_string(),
            ],
            '2' => vec![
                " ██████ ".to_string(),
                "█      █".to_string(),
                "       █".to_string(),
                 " █████  ".to_string(),
                "█       ".to_string(),
                 "█       ".to_string(),
                "████████".to_string(),
            ],
             '3' => vec![
                 " ██████ ".to_string(),
                 "      █ ".to_string(),
                "  █████ ".to_string(),
                 "      █ ".to_string(),
                 "      █ ".to_string(),
                 "  █████ ".to_string(),
                " ██████ ".to_string(),
            ],
            '4' => vec![
                "█     █".to_string(),
                "█     █".to_string(),
                "█     █".to_string(),
                 " █████  ".to_string(),
                  "      █".to_string(),
                  "      █".to_string(),
                "      █".to_string(),
            ],
            '5' => vec![
                " ███████".to_string(),
                "█       ".to_string(),
                "██████  ".to_string(),
                 "      █".to_string(),
                  "      █".to_string(),
                  "      █".to_string(),
                "███████".to_string(),
            ],
            '6' => vec![
                " ██████ ".to_string(),
                "█       ".to_string(),
                 "██████ ".to_string(),
                "█     █".to_string(),
                 "█     █".to_string(),
                 "█     █".to_string(),
                 " █████ ".to_string(),

            ],
           '7' => vec![
                "███████".to_string(),
                 "     █".to_string(),
                 "    █ ".to_string(),
                "   █  ".to_string(),
                  "  █   ".to_string(),
                  "  █  ".to_string(),
                "  █  ".to_string(),
            ],
           '8' => vec![
                 " ██████ ".to_string(),
                "█      █".to_string(),
                 " ██████ ".to_string(),
                 "█      █".to_string(),
                "█      █".to_string(),
                "█      █".to_string(),
                " ██████ ".to_string(),
            ],
           '9' => vec![
                " ███████".to_string(),
                 "█      █".to_string(),
                " ███████".to_string(),
                  "      █".to_string(),
                  "      █".to_string(),
                   "      █".to_string(),
                "  █████ ".to_string(),
            ],
            _ => vec![
                 "        ".to_string(),
                 "        ".to_string(),
                "        ".to_string(),
                "        ".to_string(),
                 "        ".to_string(),
                "        ".to_string(),
                "        ".to_string(),
            ]
        };

        if char_lines.is_empty()
         {
             char_lines = char_lines_for_c
         }else {
           for i in 0..char_lines.len()
           {
             char_lines[i].push_str(&char_lines_for_c[i]);
            }
          }

    }
   for line in char_lines
   {
      let mut line_out = Line::from("");
      for c in line.chars()
      {
        let span = Span::styled(c.to_string(), Style::default().fg(Color::Green));
         line_out.spans.push(span)
      }

        lines.push(line_out);

   }

     lines
}
