# Node Size Analyzer

A fast CLI tool to analyze and visualize the size of your node_modules dependencies using a terminal UI.

![Screenshot of Node Size Analyzer](https://raw.githubusercontent.com/sparkforge/node-size-analyzer/main/screenshot.png)

## Features

- Interactive terminal UI using ratatui
- Real-time size calculation of node_modules
- Sorted display by size
- Human-readable size formatting (B, KB, MB)
- Cross-platform support (Windows, MacOS, Linux)

## Installation

### Using Cargo

```bash
cargo install node-size-analyzer
```

### From Releases

Download the pre-built binary for your platform from the [releases page](https://github.com/sparkforge/node-size-analyzer/releases).

#### Linux/MacOS

```bash
chmod +x node-size-linux  # or node-size-macos
./node-size-linux
```

#### Windows

```bash
node-size-windows.exe
```

## Usage

1. Navigate to your project directory containing node_modules
2. Run `node-size`
3. Press 'q' to exit

## Building from Source

```bash
git clone https://github.com/sparkforge/node-size-analyzer.git
cd node-size-analyzer
cargo build --release
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [ratatui](https://github.com/ratatui-org/ratatui)
- Terminal handling by [crossterm](https://github.com/crossterm-rs/crossterm)
