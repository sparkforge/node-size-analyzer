use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Row, Table},
    Terminal,
};
use std::{fs, io, path::Path};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

#[derive(Debug)]
struct ModuleInfo {
    name: String,
    size: u64,
}

fn get_dir_size(path: &Path) -> io::Result<u64> {
    let mut total = 0;
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            total += fs::metadata(&path)?.len();
        } else if path.is_dir() {
            total += get_dir_size(&path)?;
        }
    }
    Ok(total)
}

fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;

    if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

fn scan_node_modules() -> io::Result<Vec<ModuleInfo>> {
    let node_modules = Path::new("node_modules");
    let mut modules = Vec::new();

    for entry in fs::read_dir(node_modules)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let size = get_dir_size(&path)?;
            modules.push(ModuleInfo {
                name: path.file_name().unwrap().to_string_lossy().into_owned(),
                size,
            });
        }
    }

    modules.sort_by(|a, b| b.size.cmp(&a.size));
    Ok(modules)
}

fn run_app() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let modules = scan_node_modules()?;
    
    loop {
        terminal.draw(|f| {
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(size);

            let rows: Vec<Row> = modules
                .iter()
                .map(|m| {
                    Row::new(vec![
                        m.name.clone(),
                        format_size(m.size),
                    ])
                })
                .collect();

            let table = Table::new(rows)
                .header(Row::new(vec!["Module", "Size"]).style(Style::default().fg(Color::Yellow)))
                .block(Block::default().title("Node Modules Size").borders(Borders::ALL))
                .widths(&[
                    Constraint::Percentage(70),
                    Constraint::Percentage(30),
                ]);

            f.render_widget(table, chunks[0]);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    Ok(())
}

fn main() -> io::Result<()> {
    run_app()
}
