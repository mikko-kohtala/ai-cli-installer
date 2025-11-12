# Tool Support Status

This document tracks the implementation status for each AI CLI tool across different operations.

**⚠️ Note: Currently supports macOS only**

## Legend

- ✅ Implemented
- ⚠️ Partial/Needs work
- ❌ Not implemented
- 🔍 Research needed

## Amp

**Documentation**: [Docs](https://ampcode.com/manual)

| Operation     | Status             | Command/Method                                      |
| ------------- | ------------------ | --------------------------------------------------- |
| Version Check | ✅ Implemented     | `amp --version`                                     |
| Install       | ✅ Implemented     | `curl -fsSL https://ampcode.com/install.sh \| bash` |
| Uninstall     | ❌ Not implemented |                                                     |
| Upgrade       | ❌ Not implemented |                                                     |

## Claude Code

**Documentation**: [Docs](https://code.claude.com/docs)

| Operation     | Status             | Command/Method     |
| ------------- | ------------------ | ------------------ |
| Version Check | ✅ Implemented     | `claude --version` |
| Install       | ❌ Not implemented |                    |
| Uninstall     | ❌ Not implemented |                    |
| Upgrade       | ❌ Not implemented |                    |

## Codex CLI

**Documentation**: [Docs](https://developers.openai.com/codex/cli/)

| Operation     | Status             | Command/Method    |
| ------------- | ------------------ | ----------------- |
| Version Check | ✅ Implemented     | `codex --version` |
| Install       | ❌ Not implemented |                   |
| Uninstall     | ❌ Not implemented |                   |
| Upgrade       | ❌ Not implemented |                   |

## Cursor CLI

**Documentation**: [Docs](https://cursor.com/docs/cli/overview)

| Operation     | Status             | Command/Method           |
| ------------- | ------------------ | ------------------------ |
| Version Check | ✅ Implemented     | `cursor-agent --version` |
| Install       | ❌ Not implemented |                          |
| Uninstall     | ❌ Not implemented |                          |
| Upgrade       | ❌ Not implemented |                          |

## Copilot CLI

**Documentation**: [Docs](https://docs.github.com/en/copilot/concepts/agents/about-copilot-cli)

| Operation     | Status             | Command/Method      |
| ------------- | ------------------ | ------------------- |
| Version Check | ✅ Implemented     | `copilot --version` |
| Install       | ❌ Not implemented |                     |
| Uninstall     | ❌ Not implemented |                     |
| Upgrade       | ❌ Not implemented |                     |

## Kilo Code CLI

**Documentation**: [Docs](https://kilocode.ai/docs/cli)

| Operation     | Status             | Command/Method   |
| ------------- | ------------------ | ---------------- |
| Version Check | ✅ Implemented     | `kilo --version` |
| Install       | ❌ Not implemented |                  |
| Uninstall     | ❌ Not implemented |                  |
| Upgrade       | ❌ Not implemented |                  |

## Gemini CLI

**Documentation**: [Docs](https://docs.cloud.google.com/gemini/docs/codeassist/gemini-cli)

| Operation     | Status             | Command/Method     |
| ------------- | ------------------ | ------------------ |
| Version Check | ✅ Implemented     | `gemini --version` |
| Install       | ❌ Not implemented |                    |
| Uninstall     | ❌ Not implemented |                    |
| Upgrade       | ❌ Not implemented |                    |

## Cline CLI

**Documentation**: [Docs](https://docs.cline.bot/cline-cli/overview)

| Operation     | Status             | Command/Method  |
| ------------- | ------------------ | --------------- |
| Version Check | ✅ Implemented     | `cline version` |
| Install       | ❌ Not implemented |                 |
| Uninstall     | ❌ Not implemented |                 |
| Upgrade       | ❌ Not implemented |                 |

## Next Steps

1. Research Cursor CLI (`cursor-agent`) - verify version check, install, uninstall, upgrade commands
2. Research Amp's uninstall process
3. Research Claude Code install/uninstall/upgrade process
4. Implement install commands for all tools
5. Research upgrade commands for all tools
6. Implement upgrade functionality in the CLI
