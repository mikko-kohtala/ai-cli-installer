# Tool Support Status

This document tracks the implementation status for each AI CLI tool across different operations.

## Legend

- ✅ Implemented
- ⚠️ Partial/Needs work
- ❌ Not implemented
- 🔍 Research needed

## Tools Status Table

| Tool            | Version Check               | Install              | Uninstall                     | Upgrade            | Documentation                                   |
| --------------- | --------------------------- | -------------------- | ----------------------------- | ------------------ | ----------------------------------------------- |
| **Amp**         | 🔍 Not sure yet             | ⚠️ Need to implement | 🔍 Not sure yet               | 🔍 `amp update`    | 🔍 TBD                                          |
| **Claude Code** | ✅ `claude --version`       | ⚠️ Custom (manual)   | ❌ Not implemented            | ❌ Not implemented | 🔍 TBD                                          |
| **Codex**       | ✅ `codex --version`        | ✅ `npm install -g`  | ✅ `npm uninstall -g`         | ❌ Not implemented | 🔍 TBD                                          |
| **Cursor CLI**  | 🔍 `cursor-agent --version` | ⚠️ Need to implement | 🔍 Not sure yet               | 🔍 Not sure yet    | ✅ [Docs](https://cursor.com/docs/cli/overview) |
| **Copilot CLI** | ✅ `copilot --version`      | ✅ `npm install -g`  | ✅ `npm uninstall -g`         | ❌ Not implemented | ✅ [Docs](https://docs.github.com/en/copilot/concepts/agents/about-copilot-cli) |
| **Kilo**        | ✅ `kilo --version`         | ✅ GitHub binary     | ✅ Remove from /usr/local/bin | ❌ Not implemented | 🔍 TBD                                          |
| **Gemini**      | ✅ `gemini --version`       | ✅ `npm install -g`  | ✅ `npm uninstall -g`         | ❌ Not implemented | 🔍 TBD                                          |
| **Cline**       | ✅ `cline version`          | ✅ GitHub binary     | ✅ Remove from /usr/local/bin | ❌ Not implemented | 🔍 TBD                                          |

## Detailed Implementation Notes

### Amp

- **Install**: `curl -fsSL https://ampcode.com/install.sh | bash`
- **Uninstall**: 🔍 Research needed
- **Upgrade**: `amp update` command exists
- **Version Check**: 🔍 Research needed - need to find the right command

### Claude

- **Install**: Requires manual download from https://claude.ai
- **Uninstall**: Need to research (likely removing from Applications or bin)
- **Upgrade**: Need to research if there's a CLI command

### Codex

- **Package**: `@openai/codex` on NPM
- Full NPM lifecycle support

### Cursor CLI

- **Documentation**: https://cursor.com/docs/cli/overview
- **Install**: Need to research - likely via npm or download from website
- **Uninstall**: Need to research
- **Upgrade**: Need to research if there's a CLI command
- **Version Check**: Need to verify `cursor-agent --version` works

### Copilot CLI

- **Documentation**: https://docs.github.com/en/copilot/concepts/agents/about-copilot-cli
- **Package**: `@github/copilot` on NPM
- Full NPM lifecycle support

### Kilo

- **Repository**: `Kilo-Org/kilocode` on GitHub
- Binary installation from GitHub releases

### Gemini

- **Package**: `@google/gemini-cli` on NPM
- Full NPM lifecycle support

### Cline

- **Repository**: `cline/cline` on GitHub
- Binary installation from GitHub releases

## Next Steps

1. Research and document all tool documentation pages
2. Research Cursor CLI (`cursor-agent`) - verify version check, install, uninstall, upgrade commands
3. Research Amp's version check command
4. Research Amp's uninstall process
5. Implement Amp's install command using the install script
6. Implement Cursor CLI support
7. Research upgrade commands for all tools
8. Implement upgrade functionality in the CLI
