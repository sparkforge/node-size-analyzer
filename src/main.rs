use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Row, Table, Wrap},
    Terminal,
};
use std::{collections::HashMap, fs, io, path::Path};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

#[derive(Debug, Deserialize, Serialize)]
struct PackageJson {
    name: Option<String>,
    version: Option<String>,
    description: Option<String>,
    author: Option<String>,
    license: Option<String>,
    homepage: Option<String>,
    repository: Option<Repository>,
    dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    dev_dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "peerDependencies")]
    peer_dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "optionalDependencies")]
    optional_dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "publishConfig")]
    publish_config: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Repository {
    #[serde(rename = "type")]
    repo_type: Option<String>,
    url: Option<String>,
}

impl Repository {
    fn to_string(&self) -> Option<String> {
        match &self.url {
            Some(url) => Some(url.clone()),
            None => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub size: u64,
    pub dependency_count: Option<usize>,
    pub last_updated: Option<String>,
    pub license: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub author: Option<String>,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub files_count: Option<usize>,
    pub file_types: Option<Vec<(String, usize)>>,  // (extension, count)
    pub is_dev_dependency: bool,
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
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            
            // Create a basic module info
            let mut module = ModuleInfo {
                name,
                size,
                dependency_count: None,
                last_updated: None,
                license: None,
                version: None,
                description: None,
                author: None,
                homepage: None,
                repository: None,
                files_count: None,
                file_types: None,
                is_dev_dependency: false,
            };
            
            // Try to get additional info from package.json
            let package_json_path = path.join("package.json");
            if package_json_path.exists() {
                if let Ok(json_content) = fs::read_to_string(&package_json_path) {
                    if let Ok(package_json) = serde_json::from_str::<PackageJson>(&json_content) {
                        module.version = package_json.version;
                        module.description = package_json.description;
                        module.license = package_json.license;
                        module.author = package_json.author;
                        module.homepage = package_json.homepage;
                        module.repository = package_json.repository.and_then(|r| r.to_string());
                        
                        // Count dependencies
                        let mut dep_count = 0;
                        if let Some(deps) = &package_json.dependencies {
                            dep_count += deps.len();
                        }
                        if let Some(deps) = &package_json.dev_dependencies {
                            dep_count += deps.len();
                        }
                        if let Some(deps) = &package_json.peer_dependencies {
                            dep_count += deps.len();
                        }
                        if let Some(deps) = &package_json.optional_dependencies {
                            dep_count += deps.len();
                        }
                        
                        module.dependency_count = Some(dep_count);
                    }
                }
            }
            
            // Count files and get file types
            let mut files_count = 0;
            let mut file_extensions: HashMap<String, usize> = HashMap::new();
            
            for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    files_count += 1;
                    
                    if let Some(extension) = entry.path().extension() {
                        let ext = extension.to_string_lossy().to_string().to_lowercase();
                        *file_extensions.entry(ext).or_insert(0) += 1;
                    } else {
                        *file_extensions.entry("(no extension)".to_string()).or_insert(0) += 1;
                    }
                }
            }
            
            module.files_count = Some(files_count);
            
            // Convert file_extensions HashMap to Vec and sort by count
            let mut file_types: Vec<(String, usize)> = file_extensions.into_iter().collect();
            file_types.sort_by(|a, b| b.1.cmp(&a.1));
            module.file_types = Some(file_types);
            
            // Get last modified time
            if let Ok(metadata) = fs::metadata(&path) {
                if let Ok(modified) = metadata.modified() {
                    if let Ok(modified_time) = modified.elapsed() {
                        let seconds_ago = modified_time.as_secs();
                        let last_updated = if seconds_ago < 60 {
                            format!("{} seconds ago", seconds_ago)
                        } else if seconds_ago < 3600 {
                            format!("{} minutes ago", seconds_ago / 60)
                        } else if seconds_ago < 86400 {
                            format!("{} hours ago", seconds_ago / 3600)
                        } else {
                            format!("{} days ago", seconds_ago / 86400)
                        };
                        module.last_updated = Some(last_updated);
                    }
                }
            }
            
            modules.push(module);
        }
    }

    modules.sort_by(|a, b| b.size.cmp(&a.size));
    Ok(modules)
}

enum AppMode {
    List,
    Detail,
}

struct AppState {
    modules: Vec<ModuleInfo>,
    scroll_offset: usize,
    selected_index: Option<usize>,
    mode: AppMode,
}

fn render_detail_view(module: &ModuleInfo, area: Rect, f: &mut ratatui::Frame) {
    let block = Block::default()
        .title(format!("Module Details: {}", module.name))
        .borders(Borders::ALL);
    
    let inner_area = block.inner(area);
    f.render_widget(block, area);
    
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),  // Basic info
            Constraint::Length(1),  // Separator 
            Constraint::Min(5),     // File types
        ].as_ref())
        .split(inner_area);
    
    // Basic info section
    let mut info_text = Vec::new();
    info_text.push(Line::from(vec![
        Span::styled("Size: ", Style::default().fg(Color::Yellow)),
        Span::raw(format_size(module.size)),
    ]));
    
    if let Some(version) = &module.version {
        info_text.push(Line::from(vec![
            Span::styled("Version: ", Style::default().fg(Color::Yellow)),
            Span::raw(version),
        ]));
    }
    
    if let Some(license) = &module.license {
        info_text.push(Line::from(vec![
            Span::styled("License: ", Style::default().fg(Color::Yellow)),
            Span::raw(license),
        ]));
    }
    
    if let Some(deps) = module.dependency_count {
        info_text.push(Line::from(vec![
            Span::styled("Dependencies: ", Style::default().fg(Color::Yellow)),
            Span::raw(deps.to_string()),
        ]));
    }
    
    if let Some(files) = module.files_count {
        info_text.push(Line::from(vec![
            Span::styled("Files: ", Style::default().fg(Color::Yellow)),
            Span::raw(files.to_string()),
        ]));
    }
    
    if let Some(last_updated) = &module.last_updated {
        info_text.push(Line::from(vec![
            Span::styled("Last Updated: ", Style::default().fg(Color::Yellow)),
            Span::raw(last_updated),
        ]));
    }
    
    if let Some(description) = &module.description {
        info_text.push(Line::from(vec![
            Span::styled("Description: ", Style::default().fg(Color::Yellow)),
            Span::raw(description),
        ]));
    }
    
    let basic_info = Paragraph::new(info_text)
        .block(Block::default().borders(Borders::NONE))
        .wrap(Wrap { trim: true });
    
    f.render_widget(basic_info, chunks[0]);
    
    // File types section
    let mut file_type_text = Vec::new();
    file_type_text.push(Line::from(
        Span::styled("File Types:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
    ));
    
    if let Some(file_types) = &module.file_types {
        for (ext, count) in file_types.iter().take(10) {  // Limit to top 10 types
            file_type_text.push(Line::from(vec![
                Span::styled(format!("{}: ", ext), Style::default().fg(Color::Blue)),
                Span::raw(count.to_string()),
                Span::raw(" files"),
            ]));
        }
        
        if file_types.len() > 10 {
            file_type_text.push(Line::from(
                Span::styled("(and more...)", Style::default().fg(Color::DarkGray))
            ));
        }
    } else {
        file_type_text.push(Line::from(
            Span::styled("No file type information available", Style::default().fg(Color::DarkGray))
        ));
    }
    
    let file_types_info = Paragraph::new(file_type_text)
        .block(Block::default().borders(Borders::NONE))
        .wrap(Wrap { trim: true });
    
    f.render_widget(file_types_info, chunks[2]);
    
    // Links and navigation help at the bottom
    let help_text = Text::from(vec![
        Line::from(vec![
            Span::styled("ESC", Style::default().fg(Color::Yellow)),
            Span::raw(" to return to list view | "),
            Span::styled("q", Style::default().fg(Color::Yellow)),
            Span::raw(" to quit"),
        ]),
    ]);
    
    let help_paragraph = Paragraph::new(help_text)
        .style(Style::default().fg(Color::White))
        .alignment(ratatui::layout::Alignment::Center);
    
    let help_area = Rect::new(
        area.x + 1,
        area.y + area.height - 2,
        area.width - 2,
        1,
    );
    
    f.render_widget(help_paragraph, help_area);
}

fn run_app() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let modules = scan_node_modules()?;
    
    let mut app_state = AppState {
        modules,
        scroll_offset: 0,
        selected_index: None,
        mode: AppMode::List,
    };
    
    loop {
        terminal.draw(|f| {
            let size = f.size();
            
            match app_state.mode {
                AppMode::List => {
                    let chunks = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints([Constraint::Percentage(100)].as_ref())
                        .split(size);
    
                    // Calculate visible area based on terminal size
                    // Subtract 4 for header row and borders
                    let max_visible_items = (chunks[0].height as usize).saturating_sub(4);
                    
                    // Ensure scroll offset doesn't go beyond available items
                    let total_items = app_state.modules.len();
                    if app_state.scroll_offset > total_items.saturating_sub(max_visible_items) {
                        app_state.scroll_offset = total_items.saturating_sub(max_visible_items);
                    }
                    
                    // Create rows from visible range of modules
                    let selected_style = Style::default().bg(Color::DarkGray);
                    
                    let rows: Vec<Row> = app_state.modules
                        .iter()
                        .enumerate()
                        .skip(app_state.scroll_offset)
                        .take(max_visible_items)
                        .map(|(i, m)| {
                            let style = match app_state.selected_index {
                                Some(selected) if selected == i => selected_style,
                                _ => Style::default(),
                            };
                            
                            Row::new(vec![
                                m.name.clone(),
                                format_size(m.size),
                            ]).style(style)
                        })
                        .collect();
    
                    // Create scroll indicator for title
                    let scroll_indicator = if total_items > max_visible_items {
                        format!(" [{}-{}/{}]", 
                            app_state.scroll_offset + 1, 
                            (app_state.scroll_offset + rows.len()).min(total_items),
                            total_items)
                    } else {
                        String::new()
                    };
    
                    let title = format!("Node Modules Size{}", scroll_indicator);
    
                    let table = Table::new(rows)
                        .header(Row::new(vec!["Module", "Size"]).style(Style::default().fg(Color::Yellow)))
                        .block(Block::default()
                            .title(title)
                            .borders(Borders::ALL))
                        .widths(&[
                            Constraint::Percentage(70),
                            Constraint::Percentage(30),
                        ]);
    
                    f.render_widget(table, chunks[0]);
                    
                    // Add help text at the bottom
                    let help_text = Text::from(vec![
                        Line::from(vec![
                            Span::styled(" ↑/↓: Navigate | ", Style::default().fg(Color::Gray)),
                            Span::styled("Enter: ", Style::default().fg(Color::Yellow)),
                            Span::styled("View Details | ", Style::default().fg(Color::Gray)),
                            Span::styled("q: ", Style::default().fg(Color::Yellow)),
                            Span::styled("Quit", Style::default().fg(Color::Gray)),
                        ]),
                    ]);
                    
                    let help_paragraph = Paragraph::new(help_text)
                        .style(Style::default().fg(Color::White))
                        .alignment(ratatui::layout::Alignment::Center);
                    
                    let help_area = Rect::new(
                        chunks[0].x,
                        chunks[0].y + chunks[0].height - 1,
                        chunks[0].width,
                        1,
                    );
                    
                    f.render_widget(help_paragraph, help_area);
                },
                AppMode::Detail => {
                    if let Some(idx) = app_state.selected_index {
                        if let Some(module) = app_state.modules.get(idx) {
                            // Add 10% padding on all sides
                            let detail_area = Rect::new(
                                size.x + size.width / 10,
                                size.y + size.height / 10,
                                size.width * 8 / 10,
                                size.height * 8 / 10,
                            );
                            
                            // First render background
                            f.render_widget(Clear, detail_area);
                            
                            // Then render detail view
                            render_detail_view(module, detail_area, f);
                        }
                    }
                },
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match app_state.mode {
                AppMode::List => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app_state.selected_index.is_none() {
                            app_state.selected_index = Some(app_state.scroll_offset);
                        } else if let Some(selected) = app_state.selected_index {
                            if selected > 0 {
                                app_state.selected_index = Some(selected - 1);
                                
                                // Adjust scroll if necessary
                                if selected < app_state.scroll_offset + 1 {
                                    app_state.scroll_offset = app_state.scroll_offset.saturating_sub(1);
                                }
                            }
                        }
                    },
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app_state.selected_index.is_none() {
                            app_state.selected_index = Some(app_state.scroll_offset);
                        } else if let Some(selected) = app_state.selected_index {
                            if selected < app_state.modules.len() - 1 {
                                app_state.selected_index = Some(selected + 1);
                                
                                // Get visible height
                                let visible_height = terminal.size()?.height as usize - 4;
                                
                                // Adjust scroll if necessary
                                if selected >= app_state.scroll_offset + visible_height - 1 {
                                    app_state.scroll_offset += 1;
                                }
                            }
                        }
                    },
                    KeyCode::PageUp => {
                        // Terminal size - 4 (header + borders)
                        let page_size = terminal.size()?.height as usize - 4;
                        app_state.scroll_offset = app_state.scroll_offset.saturating_sub(page_size);
                        
                        // Also adjust selected item
                        if let Some(selected) = app_state.selected_index {
                            let new_selected = selected.saturating_sub(page_size);
                            app_state.selected_index = Some(new_selected);
                        }
                    },
                    KeyCode::PageDown => {
                        // Terminal size - 4 (header + borders)
                        let page_size = terminal.size()?.height as usize - 4;
                        let max_scroll = app_state.modules.len().saturating_sub(page_size);
                        
                        app_state.scroll_offset = (app_state.scroll_offset + page_size).min(max_scroll);
                        
                        // Also adjust selected item
                        if let Some(selected) = app_state.selected_index {
                            let new_selected = (selected + page_size).min(app_state.modules.len() - 1);
                            app_state.selected_index = Some(new_selected);
                        }
                    },
                    KeyCode::Home => {
                        app_state.scroll_offset = 0;
                        if app_state.selected_index.is_some() {
                            app_state.selected_index = Some(0);
                        }
                    },
                    KeyCode::End => {
                        // Go to last page
                        let max_visible_items = terminal.size()?.height as usize - 4;
                        app_state.scroll_offset = app_state.modules.len().saturating_sub(max_visible_items);
                        
                        if app_state.selected_index.is_some() {
                            app_state.selected_index = Some(app_state.modules.len() - 1);
                        }
                    },
                    KeyCode::Enter => {
                        if app_state.selected_index.is_some() {
                            app_state.mode = AppMode::Detail;
                        }
                    },
                    _ => {}
                },
                AppMode::Detail => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Esc => app_state.mode = AppMode::List,
                    _ => {}
                },
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
    
    #[test]
    fn test_module_info_with_package_json() -> io::Result<()> {
        // Create a temporary directory
        let temp_dir = tempdir()?;
        let mock_node_modules = temp_dir.path();
        
        // Create a module with package.json
        let module_name = "test-module";
        let module_path = mock_node_modules.join(module_name);
        fs::create_dir(&module_path)?;
        
        // Create some files to count
        fs::create_dir_all(&module_path.join("src"))?;
        let js_file_path = module_path.join("src/index.js");
        let js_content = "console.log('Hello, World!');";
        let mut js_file = File::create(js_file_path)?;
        js_file.write_all(js_content.as_bytes())?;
        
        let ts_file_path = module_path.join("src/types.ts");
        let ts_content = "export type Test = { name: string; };";
        let mut ts_file = File::create(ts_file_path)?;
        ts_file.write_all(ts_content.as_bytes())?;
        
        // Create a package.json with test data
        let package_json_path = module_path.join("package.json");
        let package_json_content = r#"{
            "name": "test-module",
            "version": "1.0.0",
            "description": "A test module",
            "author": "Test Author",
            "license": "MIT",
            "homepage": "https://example.com",
            "repository": {
                "type": "git",
                "url": "https://github.com/test/test-module"
            },
            "dependencies": {
                "dep1": "^1.0.0",
                "dep2": "^2.0.0"
            },
            "devDependencies": {
                "devdep1": "^1.0.0"
            }
        }"#;
        let mut package_json_file = File::create(package_json_path)?;
        package_json_file.write_all(package_json_content.as_bytes())?;
        
        // Scan the mock node_modules directory
        let result = scan_modules_dir(mock_node_modules)?;
        
        // Check that we have our module
        assert_eq!(result.len(), 1);
        let module = &result[0];
        
        // Check basic info
        assert_eq!(module.name, module_name);
        
        // Check package.json derived info
        assert_eq!(module.version, Some("1.0.0".to_string()));
        assert_eq!(module.description, Some("A test module".to_string()));
        assert_eq!(module.author, Some("Test Author".to_string()));
        assert_eq!(module.license, Some("MIT".to_string()));
        assert_eq!(module.homepage, Some("https://example.com".to_string()));
        assert_eq!(module.repository, Some("https://github.com/test/test-module".to_string()));
        
        // Check dependency count (2 deps + 1 dev dep = 3)
        assert_eq!(module.dependency_count, Some(3));
        
        // Check files count (package.json + 2 source files = 3)
        assert_eq!(module.files_count, Some(3));
        
        // Check file types
        if let Some(file_types) = &module.file_types {
            // Convert to HashMap for easier checking
            let file_types_map: HashMap<_, _> = file_types.iter().cloned().collect();
            
            // Should have .js and .ts files
            assert_eq!(file_types_map.get("js"), Some(&1));
            assert_eq!(file_types_map.get("ts"), Some(&1));
            assert_eq!(file_types_map.get("json"), Some(&1));
        } else {
            panic!("No file types found");
        }
        
        Ok(())
    }
    
    #[test]
    fn test_app_state_init() {
        let modules = vec![
            ModuleInfo {
                name: "test1".to_string(),
                size: 100,
                dependency_count: None,
                last_updated: None,
                license: None,
                version: None,
                description: None,
                author: None,
                homepage: None,
                repository: None,
                files_count: None,
                file_types: None,
                is_dev_dependency: false,
            },
            ModuleInfo {
                name: "test2".to_string(),
                size: 200,
                dependency_count: None,
                last_updated: None,
                license: None,
                version: None,
                description: None,
                author: None,
                homepage: None,
                repository: None,
                files_count: None,
                file_types: None,
                is_dev_dependency: false,
            }
        ];
        
        let app_state = AppState {
            modules: modules.clone(),
            scroll_offset: 0,
            selected_index: None,
            mode: AppMode::List,
        };
        
        // Check initial state
        assert_eq!(app_state.modules.len(), 2);
        assert_eq!(app_state.scroll_offset, 0);
        assert_eq!(app_state.selected_index, None);
        
        // Check that we're in list mode
        match app_state.mode {
            AppMode::List => {},
            _ => panic!("Expected AppMode::List"),
        }
    }
}
