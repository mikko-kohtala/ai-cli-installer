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
ai-cli-installer list     # Show installed versions (alias)
ai-cli-installer check    # Show installed versions (alias)
```

### Install Tools

```bash
# Interactive mode - select from a menu
ai-cli-installer install
ai-cli-installer add       # Alias for install

# Direct installation - specify tool name
ai-cli-installer install claude
ai-cli-installer add claude
```

### Uninstall Tools

```bash
# Interactive mode - select from a menu
ai-cli-installer uninstall
ai-cli-installer remove    # Alias for uninstall

# Direct uninstallation - specify tool name
ai-cli-installer uninstall claude
ai-cli-installer remove claude
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
