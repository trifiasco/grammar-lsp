# Grammar LSP

A Language Server Protocol (LSP) implementation in Rust that provides grammar and spelling checking for Markdown files using local LLM.

This is what it looks like in action:
![grammar-lsp-demo](./grammar-lsp-demo.png)

This is part of a deep dive article on how LSP work by building LSP servers with Rust.
See the full write-up here : [article](https://www.aroy.sh/posts/lsp-deep-dive/)

## Features

- Real-time grammar and spelling checking on file save
- Uses local LLM via Ollama (gemma3:4b model) 
- Displays diagnostics as warnings in Neovim
- Full document synchronization
- Minimal dependencies and clean implementation

## Prerequisites

1. **Rust and Cargo** - Install from [rustup.rs](https://rustup.rs/)
2. **Ollama** - Install from [ollama.com](https://ollama.com/)
3. **Neovim 0.5+** - With built-in LSP support
4. **gemma3:1b model** - Pull the model using Ollama

## Setup

### 1. Install Ollama and Model

```bash
# Install Ollama (if not already installed)
# Visit https://ollama.com/ for installation instructions

# Pull the gemma3:1b model
ollama pull gemma3:4b

# Start Ollama server (if not running)
ollama serve
```

### 2. Build the LSP Server

```bash
cd /path/to/grammar-lsp

# Build in release mode for better performance
cargo build --release

# Or build in debug mode
cargo build
```

The binary will be created at:
- Release: `target/release/grammar-lsp`
- Debug: `target/debug/grammar-lsp`

## Usage

### Starting the LSP

In Neovim, source the init.lua file:

```vim
:luafile /path/to/grammar-lsp/init.lua
```

The LSP will automatically attach to Markdown files (`.md` extension).

### Using the Grammar Checker

1. Open or create a Markdown file
2. Write some text with spelling or grammar errors
3. Save the file (`:w`)
4. Diagnostics will appear as warnings

### Viewing Diagnostics

- **Virtual text**: Warnings appear inline (yellow text)
- **Signs**: Markers in the gutter

### Debugging

Logs are written to `grammar-lsp.log` in the project directory:

```bash
# Watch logs in real-time
tail -f /path/to/grammar-lsp.log
```

Log output shows:
- Grammar check start/completion
- Ollama API calls and responses
- Parsing results
- Error messages


### How It Works

1. **File Save**: Neovim sends `textDocument/didSave` notification
2. **Grammar Check**: LSP calls Ollama HTTP API with JSON format
3. **Parse Response**: JSON response parsed into diagnostic issues
4. **Publish Diagnostics**: Issues sent back to Neovim as warnings
5. **Display**: Neovim shows warnings inline and in UI

## Implementation Details

### LSP Methods Implemented

- `initialize` - Advertises server capabilities
- `initialized` - Confirms initialization
- `textDocument/didOpen` - Stores document in memory
- `textDocument/didChange` - Updates document (full sync)
- `textDocument/didSave` - Triggers grammar check
- `textDocument/didClose` - Removes document and clears diagnostics
- `shutdown` - Cleanup

### Ollama Integration

Uses Ollama HTTP API (`http://localhost:11434/api/generate`) with:
- `format: "json"` - Ensures valid JSON responses
- `stream: false` - Non-streaming mode
- 60-second timeout
- Structured output schema


## Limitations

This is an **illustrative implementation** with intentional simplifications:

- No incremental sync (full document only)
- No caching of diagnostics
- No debouncing of API calls
- Basic error handling
- Single model hardcoded
- No configuration file support

For production use, consider adding:
- Configuration via `settings.json`
- Incremental document sync
- Debouncing/throttling
- Better error recovery
- Multi-model support
- Persistent caching

## Troubleshooting

### "Failed to start grammar-lsp"

- Check that the binary exists and is executable
- Verify the path in `init.lua` is correct
- Try running the binary directly: `./target/release/grammar-lsp`

### No diagnostics appearing

- Check Ollama is running: `ollama list`
- Verify model is available: `ollama list | grep gemma3:1b`
- Check logs: `tail -f grammar-lsp.log`
- Ensure you saved the file (`:w`)

### "Ollama request failed"

- Make sure Ollama server is running: `ollama serve`
- Check Ollama is accessible: `curl http://localhost:11434/`
- Verify port 11434 is not blocked

### "Ollama timeout"

- Model may be loading (first request is slow)
- Document may be too large
- Increase timeout in `src/main.rs`
- Try a smaller/faster model

### Diagnostics are inaccurate

- Small models (1B/2B) have limited accuracy
- Try larger models: `gemma2:9b` or `llama3:8b`
- Adjust the prompt in `src/main.rs` for better results
- Consider fine-tuning or using specialized models

## Resources

- [Tower LSP Documentation](https://docs.rs/tower-lsp/)
- [Ollama API Docs](https://github.com/ollama/ollama/blob/main/docs/api.md)
- [LSP Specification](https://microsoft.github.io/language-server-protocol/)
- [Neovim LSP Guide](https://neovim.io/doc/user/lsp.html)
