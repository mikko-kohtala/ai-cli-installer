# ai-cli-installer

A Rust-based CLI tool to manage AI development tools from the command line.

## Installation

```bash
make install
```

## Commands

### Check Versions
```bash
ai-cli-installer          # Show installed versions
ai-cli-installer check    # Check for latest versions available
```

### Interactive Install/Uninstall
```bash
ai-cli-installer install    # Select and install tools
ai-cli-installer uninstall  # Select and uninstall tools
```

## Supported Tools

- **Amp**
- **Claude Code**
- **Code CLIx**
- **Cursor CLI**
- **Copilot CLI**
- **Kilo Code CLI**
- **Gemini CLI**
- **Cline CLI** 

## Development

```bash
make build    # Build release binary
make clean    # Clean build artifacts
make test     # Run tests
```
