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
pub struct ModuleInfo {
    pub name: String,
    pub size: u64,
}

pub fn get_dir_size(path: &Path) -> io::Result<u64> {
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

pub fn format_size(size: u64) -> String {
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

pub fn scan_node_modules() -> io::Result<Vec<ModuleInfo>> {
    scan_modules_dir(Path::new("node_modules"))
}

pub fn scan_modules_dir(node_modules: &Path) -> io::Result<Vec<ModuleInfo>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;
    
    #[test]
    fn test_format_size() {
        assert_eq!(format_size(500), "500 B");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1500), "1.46 KB");
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_size(1024 * 1024 * 2 + 1024 * 100), "2.10 MB");
    }
    
    #[test]
    fn test_get_dir_size() -> io::Result<()> {
        // Create a temporary directory
        let temp_dir = tempdir()?;
        let temp_path = temp_dir.path();
        
        // Create a file with known content
        let file_path = temp_path.join("test_file.txt");
        let content = "Hello, world!";
        let mut file = File::create(&file_path)?;
        file.write_all(content.as_bytes())?;
        
        // Create a subdirectory with a file
        let subdir_path = temp_path.join("subdir");
        fs::create_dir(&subdir_path)?;
        let subfile_path = subdir_path.join("subfile.txt");
        let subcontent = "This is a test file in a subdirectory";
        let mut subfile = File::create(&subfile_path)?;
        subfile.write_all(subcontent.as_bytes())?;
        
        // Expected size is the sum of both file contents
        let expected_size = (content.len() + subcontent.len()) as u64;
        let actual_size = get_dir_size(temp_path)?;
        
        assert_eq!(actual_size, expected_size);
        Ok(())
    }
    
    #[test]
    fn test_scan_modules_dir() -> io::Result<()> {
        // Create a mock node_modules directory structure
        let temp_dir = tempdir()?;
        let mock_node_modules = temp_dir.path();
        
        // Create a few mock modules with different sizes
        let modules = vec![
            ("small-module", 100),
            ("medium-module", 500),
            ("large-module", 1000)
        ];
        
        for (name, size) in &modules {
            let module_path = mock_node_modules.join(name);
            fs::create_dir(&module_path)?;
            let file_path = module_path.join("index.js");
            let content = "a".repeat(*size);
            let mut file = File::create(file_path)?;
            file.write_all(content.as_bytes())?;
        }
        
        // Scan the mock node_modules directory
        let result = scan_modules_dir(mock_node_modules)?;
        
        // Check that we have all expected modules
        assert_eq!(result.len(), modules.len());
        
        // Check that they're sorted by size (largest first)
        assert_eq!(result[0].name, "large-module");
        assert_eq!(result[1].name, "medium-module");
        assert_eq!(result[2].name, "small-module");
        
        // Check actual sizes
        assert_eq!(result[0].size, 1000);
        assert_eq!(result[1].size, 500);
        assert_eq!(result[2].size, 100);
        
        Ok(())
    }
}
