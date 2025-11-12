use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use serde::Deserialize;
use std::process::Command;

#[derive(Parser)]
#[command(name = "ai-cli-installer")]
#[command(about = "Check and manage AI CLI tools versions", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Check latest versions available
    Check,
    /// Upgrade all tools (not implemented yet)
    Upgrade,
}

#[derive(Debug)]
struct ToolVersion {
    name: String,
    installed: Option<String>,
    latest: Option<String>,
}

impl ToolVersion {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            installed: None,
            latest: None,
        }
    }

    fn with_installed(mut self, version: Option<String>) -> Self {
        self.installed = version;
        self
    }

    fn with_latest(mut self, version: Option<String>) -> Self {
        self.latest = version;
        self
    }
}

#[derive(Deserialize)]
struct NpmPackageInfo {
    version: String,
}

#[derive(Deserialize)]
struct GitHubRelease {
    tag_name: String,
}

fn check_command(cmd: &str, args: &[&str]) -> Option<String> {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}

fn get_claude_version() -> ToolVersion {
    let installed = check_command("claude", &["--version"])
        .and_then(|s| s.lines().next().map(|l| l.to_string()));
    ToolVersion::new("Claude").with_installed(installed)
}

fn get_amp_version() -> ToolVersion {
    let installed = check_command("amp", &["--version"])
        .and_then(|s| s.lines().next().map(|l| l.to_string()));
    ToolVersion::new("Amp").with_installed(installed)
}

fn get_codex_version() -> ToolVersion {
    let installed = check_command("codex", &["--version"]);
    ToolVersion::new("Codex").with_installed(installed)
}

fn get_cursor_version() -> ToolVersion {
    let installed = check_command(
        "defaults",
        &[
            "read",
            "/Applications/Cursor.app/Contents/Info.plist",
            "CFBundleShortVersionString",
        ],
    );
    ToolVersion::new("Cursor").with_installed(installed)
}

fn get_copilot_version() -> ToolVersion {
    let installed = check_command("copilot", &["--version"])
        .and_then(|s| s.lines().next().map(|l| l.to_string()));
    ToolVersion::new("Copilot").with_installed(installed)
}

fn get_kilo_version() -> ToolVersion {
    let installed = check_command("kilo", &["--version"]);
    ToolVersion::new("Kilo").with_installed(installed)
}

fn get_gemini_version() -> ToolVersion {
    let installed = check_command("gemini", &["--version"]);
    ToolVersion::new("Gemini").with_installed(installed)
}

fn get_cline_version() -> ToolVersion {
    let installed = check_command("cline", &["version"]).and_then(|output| {
        output
            .lines()
            .find(|line| line.contains("Cline CLI Version:"))
            .and_then(|line| line.split_whitespace().nth(3).map(|v| {
                let core = output
                    .lines()
                    .find(|l| l.contains("Cline Core Version:"))
                    .and_then(|l| l.split_whitespace().nth(3))
                    .unwrap_or("");
                format!("{} (Core: {})", v, core)
            }))
    });
    ToolVersion::new("Cline").with_installed(installed)
}

async fn get_npm_latest(package: &str) -> Option<String> {
    let url = format!("https://registry.npmjs.org/{}", package);
    let response = reqwest::get(&url).await.ok()?;
    let info: NpmPackageInfo = response.json().await.ok()?;
    Some(info.version)
}

async fn get_github_latest(repo: &str) -> Option<String> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "ai-cli-installer")
        .send()
        .await
        .ok()?;
    let release: GitHubRelease = response.json().await.ok()?;
    Some(release.tag_name)
}

fn print_version(tool: &ToolVersion, check_latest: bool) {
    let status = match &tool.installed {
        Some(version) => {
            let version_str = format!("{}", version);
            if check_latest {
                if let Some(latest) = &tool.latest {
                    if version.contains(latest) || latest.contains(version) {
                        format!("{} ✓", version_str.green())
                    } else {
                        format!("{} → {} available", version_str.yellow(), latest.bright_blue())
                    }
                } else {
                    version_str.green().to_string()
                }
            } else {
                version_str.green().to_string()
            }
        }
        None => "not installed".red().to_string(),
    };

    println!("{:12} {}", format!("{}:", tool.name).bold(), status);
}

async fn get_all_versions() -> Vec<ToolVersion> {
    vec![
        get_claude_version(),
        get_amp_version(),
        get_codex_version(),
        get_cursor_version(),
        get_copilot_version(),
        get_kilo_version(),
        get_gemini_version(),
        get_cline_version(),
    ]
}

async fn check_latest_versions(tools: &mut [ToolVersion]) {
    println!("{}", "Checking latest versions...".cyan());

    // Fetch latest versions in parallel
    let handles = vec![
        tokio::spawn(get_npm_latest("@openai/codex")),
        tokio::spawn(get_npm_latest("@github/copilot")),
        tokio::spawn(get_npm_latest("@google/gemini-cli")),
        tokio::spawn(get_github_latest("cline/cline")),
        tokio::spawn(get_github_latest("Kilo-Org/kilocode")),
    ];

    let results = futures::future::join_all(handles).await;

    // Update tools with latest versions
    for tool in tools.iter_mut() {
        tool.latest = match tool.name.as_str() {
            "Codex" => results[0].as_ref().ok().and_then(|r| r.clone()),
            "Copilot" => results[1].as_ref().ok().and_then(|r| r.clone()),
            "Gemini" => results[2].as_ref().ok().and_then(|r| r.clone()),
            "Cline" => results[3].as_ref().ok().and_then(|r| r.clone()),
            "Kilo" => results[4].as_ref().ok().and_then(|r| r.clone()),
            _ => None,
        };
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    println!("\n{}", "🤖 AI Tools Version Checker".bright_cyan().bold());
    println!("{}\n", "=".repeat(40).bright_cyan());

    match cli.command {
        None => {
            // Show installed versions
            let tools = get_all_versions().await;
            for tool in &tools {
                print_version(tool, false);
            }
        }
        Some(Commands::Check) => {
            // Check latest versions
            let mut tools = get_all_versions().await;
            check_latest_versions(&mut tools).await;
            println!();
            for tool in &tools {
                print_version(tool, true);
            }
        }
        Some(Commands::Upgrade) => {
            println!("{}", "Upgrade functionality coming soon!".yellow());
            println!("This will upgrade all AI CLI tools to their latest versions.");
        }
    }

    println!();
    Ok(())
}
