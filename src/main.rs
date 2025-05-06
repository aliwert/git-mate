use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use colored::*;
use dialoguer::{Input, Password, Select};
use dirs::home_dir;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::Path;
use std::process::{Command, Output};
use std::str;

const CONFIG_DIR: &str = ".git_mate";
const CONFIG_FILE: &str = "config.json";
const GITIGNORE_API_URL: &str = "https://api.github.com/gitignore/templates";
const VERSION: &str = "0.1.0";

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    github_token: String,
    username: String,
    default_branch: Option<String>,
    default_license: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RepoInfo {
    name: String,
    description: String,
    private: bool,
    license: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct IssueInfo {
    title: String,
    body: String,
    labels: Vec<String>,
}
fn main() {
    let app = App::new("git_mate")
        .version(VERSION)
        .author("Git Mate CLI")
        .about("Automates pushing local projects to GitHub")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("init")
                .about("Initialize a new Git repository and prepare for GitHub")
                .arg(
                    Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .help("Repository name")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("description")
                        .long("desc")
                        .short("d")
                        .help("Repository description")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("private")
                        .long("private")
                        .short("p")
                        .help("Make repository private")
                        .takes_value(false),
                )
                .arg(
                    Arg::with_name("gitignore")
                        .long("gitignore")
                        .short("g")
                        .help("Add a .gitignore template")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("license")
                        .long("license")
                        .short("l")
                        .help("Add a license (e.g., MIT, Apache-2.0)")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("workflow")
                        .long("workflow")
                        .short("w")
                        .help("Add a GitHub Actions workflow")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("push")
                .about("Commit changes and push to GitHub")
                .arg(
                    Arg::with_name("message")
                        .long("message")
                        .short("m")
                        .help("Commit message")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("config")
                .about("Configure GitHub credentials")
                .arg(
                    Arg::with_name("token")
                        .long("token")
                        .short("t")
                        .help("GitHub Personal Access Token")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("username")
                        .long("username")
                        .short("u")
                        .help("GitHub username")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("default-branch")
                        .long("default-branch")
                        .help("Default branch name")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("branch")
                .about("Manage git branches")
                .subcommand(
                    SubCommand::with_name("create")
                        .about("Create a new branch")
                        .arg(
                            Arg::with_name("name")
                                .help("Branch name")
                                .required(true)
                                .index(1),
                        )
                        .arg(
                            Arg::with_name("checkout")
                                .long("checkout")
                                .short("c")
                                .help("Checkout the new branch")
                                .takes_value(false),
                        ),
                )
                .subcommand(SubCommand::with_name("list").about("List branches"))
                .subcommand(
                    SubCommand::with_name("switch")
                        .about("Switch to a branch")
                        .arg(
                            Arg::with_name("name")
                                .help("Branch name")
                                .required(true)
                                .index(1),
                        ),
                ),
        )
        .subcommand(
            SubCommand::with_name("gitignore")
                .about("Set up a .gitignore file")
                .arg(
                    Arg::with_name("template")
                        .help("Template name (e.g., Rust, Python, Node)")
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("issue")
                .about("Create a GitHub issue")
                .arg(
                    Arg::with_name("title")
                        .long("title")
                        .short("t")
                        .help("Issue title")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("body")
                        .long("body")
                        .short("b")
                        .help("Issue body")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("label")
                        .long("label")
                        .short("l")
                        .help("Issue label")
                        .takes_value(true)
                        .multiple(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("pr")
                .about("Create a pull request")
                .arg(
                    Arg::with_name("title")
                        .long("title")
                        .short("t")
                        .help("PR title")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("body")
                        .long("body")
                        .short("b")
                        .help("PR description")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("base")
                        .long("base")
                        .help("Base branch")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("head")
                        .long("head")
                        .help("Head branch")
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("workflow")
                .about("Set up GitHub Actions workflow")
                .arg(
                    Arg::with_name("type")
                        .help("Workflow type (ci, deploy, custom)")
                        .index(1)
                        .possible_values(&["ci", "deploy", "custom"]),
                ),
        );
    let matches = app.get_matches();
    match matches.subcommand() {
        ("init", Some(init_matches)) => init_command(init_matches),
        ("push", Some(push_matches)) => push_command(push_matches),
        ("config", Some(config_matches)) => config_command(config_matches),
        ("branch", Some(branch_matches)) => branch_command(branch_matches),
        ("gitignore", Some(gitignore_matches)) => gitignore_command(gitignore_matches),
        ("issue", Some(issue_matches)) => issue_command(issue_matches),
        ("pr", Some(pr_matches)) => pr_command(pr_matches),
        ("workflow", Some(workflow_matches)) => workflow_command(workflow_matches),
        _ => unreachable!(),
    }
}

fn init_command(matches: &ArgMatches) {
    // check if already a git repository
    if is_git_repository() {
        println!("{}", "This directory is already a git repository.".yellow());
    } else {
        // Initialize git repository
        match run_command("git", &["init"]) {
            Ok(_) => println!("{}", "Git repository initialized successfully.".green()),
            Err(e) => {
                println!("{} {}", "Failed to initialize git repository:".red(), e);
                return;
            }
        }
    }

    // get repository information
    let repo_info = get_repository_info(matches);

    // load configuration
    let config = match load_config() {
        Ok(config) => config,
        Err(_) => {
            println!(
                "{}",
                "No GitHub configuration found. Please run 'git_mate config' first.".red()
            );
            return;
        }
    };

    // set up .gitignore if requested
    if let Some(template) = matches.value_of("gitignore") {
        setup_gitignore(template, &config);
    }

    // set up license if requested
    if let Some(license) = matches.value_of("license") {
        setup_license(license, &config);
    }

    // create GitHub repository
    match create_github_repo(&config, &repo_info) {
        Ok(repo_url) => {
            println!("{} {}", "GitHub repository created:".green(), repo_url);

            // add remote
            match run_command("git", &["remote", "add", "origin", &repo_url]) {
                Ok(_) => println!("{}", "Remote 'origin' added successfully.".green()),
                Err(e) => println!("{} {}", "Failed to add remote:".red(), e),
            }

            // create README.md if it doesn't exist
            if !Path::new("README.md").exists() {
                match fs::write(
                    "README.md",
                    format!("# {}\n\n{}", repo_info.name, repo_info.description),
                ) {
                    Ok(_) => println!("{}", "Created README.md file.".green()),
                    Err(e) => println!("{} {}", "Failed to create README.md:".red(), e),
                }
            }

            // set up GitHub Actions workflow if requested
            if let Some(workflow_type) = matches.value_of("workflow") {
                setup_workflow(workflow_type);
            }

            // add all files
            match run_command("git", &["add", "."]) {
                Ok(_) => println!("{}", "Added files to staging area.".green()),
                Err(e) => println!("{} {}", "Failed to add files:".red(), e),
            }

            // initial commit
            match run_command("git", &["commit", "-m", "Initial commit"]) {
                Ok(_) => println!("{}", "Created initial commit.".green()),
                Err(e) => println!("{} {}", "Failed to create initial commit:".red(), e),
            }

            // get default branch from config or use main/master
            let default_branch = config.default_branch.unwrap_or_else(|| "main".to_string());

            // rename current branch if needed
            match get_current_branch() {
                Ok(current_branch) => {
                    if current_branch != default_branch {
                        match run_command("git", &["branch", "-M", &default_branch]) {
                            Ok(_) => println!("{} {}", "Renamed branch to".green(), default_branch),
                            Err(e) => println!("{} {}", "Failed to rename branch:".red(), e),
                        }
                    }
                }
                Err(_) => println!("{}", "Could not determine current branch.".yellow()),
            }

            // push to GitHub
            match run_command("git", &["push", "-u", "origin", &default_branch]) {
                Ok(_) => println!(
                    "{}",
                    "Project pushed to GitHub successfully!".green().bold()
                ),
                Err(e) => println!("{} {}", "Failed to push to GitHub:".red(), e),
            }
        }
        Err(e) => println!("{} {}", "Failed to create GitHub repository:".red(), e),
    }
}

fn push_command(matches: &ArgMatches) {
    if !is_git_repository() {
        println!(
            "{}",
            "Not a git repository. Run 'git_mate init' first.".red()
        );
        return;
    }

    // get commit message
    let message = match matches.value_of("message") {
        Some(msg) => msg.to_string(),
        None => {
            let default_msg = "Update";
            match Input::<String>::new()
                .with_prompt("Commit message")
                .default(default_msg.to_string())
                .interact()
            {
                Ok(msg) => msg,
                Err(_) => {
                    println!(
                        "{}",
                        "Failed to get commit message. Using default.".yellow()
                    );
                    default_msg.to_string()
                }
            }
        }
    };

    // check for uncommitted changes
    match run_command("git", &["status", "--porcelain"]) {
        Ok(output) => {
            let changes = str::from_utf8(&output.stdout).unwrap_or("").trim();

            if changes.is_empty() {
                println!("{}", "No changes to commit. Working tree clean.".yellow());

                // push anyway in case there are unpushed commits
                let current_branch = match get_current_branch() {
                    Ok(branch) => branch,
                    Err(e) => {
                        println!("{} {}", "Failed to get current branch:".red(), e);
                        return;
                    }
                };

                match run_command("git", &["push", "origin", &current_branch]) {
                    Ok(_) => println!("{}", "Pushed existing commits to GitHub.".green()),
                    Err(e) => println!("{} {}", "Failed to push:".red(), e),
                }

                return;
            }
        }
        Err(e) => println!("{} {}", "Failed to check git status:".yellow(), e),
    }

    // add all files
    match run_command("git", &["add", "."]) {
        Ok(_) => println!("{}", "Added files to staging area.".green()),
        Err(e) => {
            println!("{} {}", "Failed to add files:".red(), e);
            return;
        }
    }

    // commit changes
    match run_command("git", &["commit", "-m", &message]) {
        Ok(_) => println!("{}", "Changes committed successfully.".green()),
        Err(e) => {
            println!("{} {}", "Failed to commit changes:".red(), e);
            return;
        }
    }

    // push to github
    let current_branch = match get_current_branch() {
        Ok(branch) => branch,
        Err(e) => {
            println!("{} {}", "Failed to get current branch:".red(), e);
            return;
        }
    };

    match run_command("git", &["push", "origin", &current_branch]) {
        Ok(_) => println!(
            "{}",
            "Changes pushed to GitHub successfully!".green().bold()
        ),
        Err(e) => println!("{} {}", "Failed to push changes:".red(), e),
    }
}

fn config_command(matches: &ArgMatches) {
    let mut config = load_config().unwrap_or_else(|_| Config {
        github_token: String::new(),
        username: String::new(),
        default_branch: Some("main".to_string()),
        default_license: None,
    });

    // update token if provided
    if let Some(token) = matches.value_of("token") {
        config.github_token = token.to_string();
    }

    // update username if provided
    if let Some(username) = matches.value_of("username") {
        config.username = username.to_string();
    }

    // update default branch if provided
    if let Some(branch) = matches.value_of("default-branch") {
        config.default_branch = Some(branch.to_string());
    }

    // if no arguments provided, prompt interactively
    if !matches.is_present("token")
        && !matches.is_present("username")
        && !matches.is_present("default-branch")
    {
        println!("{}", "GitHub Configuration".cyan().bold());
        println!("{}", "Please provide your GitHub credentials.".cyan());

        // get username
        config.username = Input::new()
            .with_prompt("GitHub username")
            .default(config.username.clone())
            .interact()
            .unwrap_or_else(|_| config.username.clone());

        // get token
        config.github_token = Password::new()
            .with_prompt("GitHub Personal Access Token (with repo scope)")
            .with_confirmation("Confirm token", "Tokens don't match")
            .interact()
            .unwrap_or_else(|_| config.github_token);

        // get default branch
        config.default_branch = Some(
            Input::new()
                .with_prompt("Default branch name")
                .default(config.default_branch.unwrap_or_else(|| "main".to_string()))
                .interact()
                .unwrap_or_else(|_| "main".to_string()),
        );
    }

    // validate config.
    if config.github_token.is_empty() || config.username.is_empty() {
        println!("{}", "GitHub token and username are required.".red());
        return;
    }

    // save config.
    match save_config(&config) {
        Ok(_) => println!("{}", "Configuration saved successfully.".green()),
        Err(e) => println!("{} {}", "Failed to save configuration:".red(), e),
    }
}

fn branch_command(matches: &ArgMatches) {
    if !is_git_repository() {
        println!(
            "{}",
            "Not a git repository. Run 'git_mate init' first.".red()
        );
        return;
    }

    match matches.subcommand() {
        ("create", Some(create_matches)) => {
            let branch_name = create_matches.value_of("name").unwrap();

            // create the branch
            match run_command("git", &["branch", branch_name]) {
                Ok(_) => {
                    println!("{} {}", "Branch created:".green(), branch_name);

                    // checkout if requested
                    if create_matches.is_present("checkout") {
                        match run_command("git", &["checkout", branch_name]) {
                            Ok(_) => println!("{} {}", "Switched to branch:".green(), branch_name),
                            Err(e) => println!("{} {}", "Failed to switch branch:".red(), e),
                        }
                    }
                }
                Err(e) => println!("{} {}", "Failed to create branch:".red(), e),
            }
        }
        ("list", _) => match run_command("git", &["branch"]) {
            Ok(output) => {
                let branches = str::from_utf8(&output.stdout)
                    .unwrap_or("Could not parse branches")
                    .trim();
                println!("{}", "Branches:".cyan());
                println!("{}", branches);
            }
            Err(e) => println!("{} {}", "Failed to list branches:".red(), e),
        },
        ("switch", Some(switch_matches)) => {
            let branch_name = switch_matches.value_of("name").unwrap();

            match run_command("git", &["checkout", branch_name]) {
                Ok(_) => println!("{} {}", "Switched to branch:".green(), branch_name),
                Err(e) => println!("{} {}", "Failed to switch branch:".red(), e),
            }
        }
        _ => println!("{}", "Unknown branch subcommand".red()),
    }
}

fn gitignore_command(matches: &ArgMatches) {
    let config = match load_config() {
        Ok(config) => config,
        Err(_) => {
            println!(
                "{}",
                "No GitHub configuration found. Please run 'git_mate config' first.".red()
            );
            return;
        }
    };

    let template = match matches.value_of("template") {
        Some(template) => template.to_string(),
        None => {
            // interactive template selection
            let templates = get_gitignore_templates(&config);
            match templates {
                Ok(templates) => {
                    let selection = Select::new()
                        .with_prompt("Select a .gitignore template")
                        .items(&templates)
                        .interact();

                    match selection {
                        Ok(index) => templates[index].clone(),
                        Err(_) => {
                            println!("{}", "Template selection cancelled.".yellow());
                            return;
                        }
                    }
                }
                Err(e) => {
                    println!("{} {}", "Failed to fetch .gitignore templates:".red(), e);
                    return;
                }
            }
        }
    };

    setup_gitignore(&template, &config);
}

fn issue_command(matches: &ArgMatches) {
    if !is_git_repository() {
        println!(
            "{}",
            "Not a git repository. Run 'git_mate init' first.".red()
        );
        return;
    }

    let config = match load_config() {
        Ok(config) => config,
        Err(_) => {
            println!(
                "{}",
                "No GitHub configuration found. Please run 'git_mate config' first.".red()
            );
            return;
        }
    };

    let repo_name = match get_repo_name_from_remote() {
        Ok(name) => name,
        Err(e) => {
            println!("{} {}", "Failed to determine repository name:".red(), e);
            return;
        }
    };

    let issue = IssueInfo {
        title: matches
            .value_of("title")
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                Input::new()
                    .with_prompt("Issue title")
                    .interact()
                    .unwrap_or_else(|_| "New Issue".to_string())
            }),
        body: matches
            .value_of("body")
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                Input::new()
                    .with_prompt("Issue description")
                    .interact()
                    .unwrap_or_else(|_| "".to_string())
            }),
        labels: matches
            .values_of("label")
            .map(|vals| vals.map(|v| v.to_string()).collect())
            .unwrap_or_else(Vec::new),
    };

    match create_github_issue(&config, &repo_name, &issue) {
        Ok(url) => println!("{} {}", "Issue created:".green(), url),
        Err(e) => println!("{} {}", "Failed to create issue:".red(), e),
    }
}

fn pr_command(matches: &ArgMatches) {
    if !is_git_repository() {
        println!(
            "{}",
            "Not a git repository. Run 'git_mate init' first.".red()
        );
        return;
    }

    let config = match load_config() {
        Ok(config) => config,
        Err(_) => {
            println!(
                "{}",
                "No GitHub configuration found. Please run 'git_mate config' first.".red()
            );
            return;
        }
    };

    let repo_name = match get_repo_name_from_remote() {
        Ok(name) => name,
        Err(e) => {
            println!("{} {}", "Failed to determine repository name:".red(), e);
            return;
        }
    };

    let current_branch = match get_current_branch() {
        Ok(branch) => branch,
        Err(e) => {
            println!("{} {}", "Failed to get current branch:".red(), e);
            return;
        }
    };

    // get title
    let title = matches
        .value_of("title")
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            Input::new()
                .with_prompt("Pull request title")
                .default(format!("Merge {} into main", current_branch))
                .interact()
                .unwrap_or_else(|_| format!("Merge {} into main", current_branch))
        });

    // get body
    let body = matches
        .value_of("body")
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            Input::new()
                .with_prompt("Pull request description")
                .interact()
                .unwrap_or_else(|_| "".to_string())
        });

    // get base branch
    let base = matches
        .value_of("base")
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            let default = "main".to_string();
            Input::new()
                .with_prompt("Base branch (target)")
                .default(default)
                .interact()
                .unwrap_or_else(|_| "main".to_string())
        });

    // get head branch
    let head = matches
        .value_of("head")
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            Input::new()
                .with_prompt("Head branch (source)")
                .default(current_branch.clone())
                .interact()
                .unwrap_or_else(|_| current_branch.clone())
        });

    // push the current branch first to ensure it's on GitHub
    match run_command("git", &["push", "--set-upstream", "origin", &head]) {
        Ok(_) => println!("{} {}", "Pushed branch to GitHub:".green(), head),
        Err(e) => {
            println!("{} {}", "Failed to push branch:".red(), e);
            println!("{}", "Continuing to create PR anyway...".yellow());
        }
    }

    match create_github_pr(&config, &repo_name, &title, &body, &base, &head) {
        Ok(url) => println!("{} {}", "Pull request created:".green(), url),
        Err(e) => println!("{} {}", "Failed to create pull request:".red(), e),
    }
}

fn workflow_command(matches: &ArgMatches) {
    let workflow_type = matches.value_of("type").unwrap_or_else(|| {
        let options = vec!["Continuous Integration", "Deployment", "Custom"];
        let selection = Select::new()
            .with_prompt("Select a workflow type")
            .items(&options)
            .default(0)
            .interact()
            .unwrap_or(0);

        match selection {
            0 => "ci",
            1 => "deploy",
            _ => "custom",
        }
    });

    setup_workflow(workflow_type);
}

fn get_repository_info(matches: &ArgMatches) -> RepoInfo {
    let current_dir = Path::new(".")
        .canonicalize()
        .unwrap_or_else(|_| Path::new(".").to_path_buf());
    let default_name = current_dir
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let name = match matches.value_of("name") {
        Some(name) => name.to_string(),
        None => Input::new()
            .with_prompt("Repository name")
            .default(default_name)
            .interact()
            .unwrap(),
    };

    let description = match matches.value_of("description") {
        Some(desc) => desc.to_string(),
        None => Input::new()
            .with_prompt("Repository description")
            .default(String::from("Created with git_mate"))
            .interact()
            .unwrap(),
    };

    let private = if matches.is_present("private") {
        true
    } else {
        let options = vec!["Public", "Private"];
        let selection = Select::new()
            .with_prompt("Repository visibility")
            .default(0)
            .items(&options)
            .interact()
            .unwrap_or(0);
        selection == 1
    };

    let license = matches.value_of("license").map(|s| s.to_string());

    RepoInfo {
        name,
        description,
        private,
        license,
    }
}

fn create_github_repo(config: &Config, repo_info: &RepoInfo) -> Result<String, String> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static("Git-Mate-CLI"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("token {}", config.github_token))
            .map_err(|e| e.to_string())?,
    );

    let repo_data = json!({
        "name": repo_info.name,
        "description": repo_info.description,
        "private": repo_info.private,
        "auto_init": false,
        "license_template": repo_info.license
    });

    let res = client
        .post("https://api.github.com/user/repos")
        .headers(headers)
        .json(&repo_data)
        .send()
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let json: Value = res.json().map_err(|e| e.to_string())?;
        if let Some(ssh_url) = json["ssh_url"].as_str() {
            Ok(ssh_url.to_string())
        } else if let Some(clone_url) = json["clone_url"].as_str() {
            Ok(clone_url.to_string())
        } else {
            Err("Failed to get repository URL from GitHub response".to_string())
        }
    } else {
        let status = res.status();
        let text = res.text().unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("GitHub API error ({}): {}", status, text))
    }
}

fn get_config_path() -> Result<std::path::PathBuf, String> {
    let home = home_dir().ok_or_else(|| "Could not find home directory".to_string())?;
    let config_dir = home.join(CONFIG_DIR);
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;
    }
    Ok(config_dir.join(CONFIG_FILE))
}

fn load_config() -> Result<Config, String> {
    let config_path = get_config_path()?;
    if !config_path.exists() {
        return Err("Config file does not exist".to_string());
    }
    let config_str = fs::read_to_string(config_path).map_err(|e| e.to_string())?;
    serde_json::from_str(&config_str).map_err(|e| e.to_string())
}

fn save_config(config: &Config) -> Result<(), String> {
    let config_path = get_config_path()?;
    let config_str = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(config_path, config_str).map_err(|e| e.to_string())
}

fn run_command(command: &str, args: &[&str]) -> Result<Output, String> {
    Command::new(command)
        .args(args)
        .output()
        .map_err(|e| e.to_string())
        .and_then(|output| {
            if output.status.success() {
                Ok(output)
            } else {
                let error = str::from_utf8(&output.stderr)
                    .unwrap_or("Unknown error")
                    .trim();
                Err(error.to_string())
            }
        })
}

fn is_git_repository() -> bool {
    Path::new(".git").exists()
}

fn get_current_branch() -> Result<String, String> {
    let output = run_command("git", &["rev-parse", "--abbrev-ref", "HEAD"])?;
    let branch = str::from_utf8(&output.stdout)
        .map_err(|e| e.to_string())?
        .trim()
        .to_string();
    Ok(branch)
}

fn get_repo_name_from_remote() -> Result<String, String> {
    // try to get the remote URL
    let output = run_command("git", &["remote", "get-url", "origin"])?;
    let remote_url = str::from_utf8(&output.stdout)
        .map_err(|e| e.to_string())?
        .trim();

    // parse the repository name from the URL
    // format could be:
    // - git@github.com:username/repo.git
    // - https://github.com/username/repo.git

    if remote_url.contains("github.com") {
        let parts: Vec<&str> = if remote_url.starts_with("git@") {
            // SSH format
            remote_url.split(':').collect()
        } else {
            // HTTPS format
            remote_url.split('/').collect()
        };

        if parts.len() >= 2 {
            let repo_part = parts[parts.len() - 1].replace(".git", "");
            let username_part = parts[parts.len() - 2].replace("github.com:", "");

            return Ok(format!("{}/{}", username_part, repo_part));
        }
    }

    Err("Could not parse repository name from remote URL".to_string())
}

fn get_gitignore_templates(config: &Config) -> Result<Vec<String>, String> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static("Git-Mate-CLI"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("token {}", config.github_token))
            .map_err(|e| e.to_string())?,
    );

    let res = client
        .get(GITIGNORE_API_URL)
        .headers(headers)
        .send()
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let templates: Vec<String> = res.json().map_err(|e| e.to_string())?;
        Ok(templates)
    } else {
        Err(format!(
            "Failed to fetch gitignore templates: {}",
            res.status()
        ))
    }
}

fn setup_gitignore(template: &str, config: &Config) {
    println!(
        "{} {}",
        "Setting up .gitignore with template:".cyan(),
        template
    );

    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static("Git-Mate-CLI"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("token {}", config.github_token))
            .unwrap_or_else(|_| HeaderValue::from_static("")),
    );

    let url = format!("{}/{}", GITIGNORE_API_URL, template);

    match client.get(&url).headers(headers).send() {
        Ok(res) => {
            if res.status().is_success() {
                match res.json::<Value>() {
                    Ok(json) => {
                        if let Some(content) = json["source"].as_str() {
                            // check if .gitignore already exists
                            let gitignore_path = Path::new(".gitignore");
                            let content = if gitignore_path.exists() {
                                let existing =
                                    fs::read_to_string(gitignore_path).unwrap_or_default();
                                format!("{}\n\n# Added by Git-Mate\n{}", existing, content)
                            } else {
                                format!(
                                    "# Created by Git-Mate using {} template\n{}",
                                    template, content
                                )
                            };

                            match fs::write(gitignore_path, content) {
                                Ok(_) => {
                                    println!("{}", ".gitignore file created successfully.".green())
                                }
                                Err(e) => println!("{} {}", "Failed to write .gitignore:".red(), e),
                            }
                        } else {
                            println!("{}", "Invalid response format from GitHub API.".red());
                        }
                    }
                    Err(e) => println!("{} {}", "Failed to parse GitHub API response:".red(), e),
                }
            } else {
                println!(
                    "{} {}",
                    "Failed to fetch gitignore template:".red(),
                    res.status()
                );
            }
        }
        Err(e) => println!("{} {}", "Failed to connect to GitHub API:".red(), e),
    }
}

fn setup_license(license: &str, config: &Config) {
    println!("{} {}", "Setting up license:".cyan(), license);

    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static("Git-Mate-CLI"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("token {}", config.github_token))
            .unwrap_or_else(|_| HeaderValue::from_static("")),
    );

    let url = format!("https://api.github.com/licenses/{}", license);

    match client.get(&url).headers(headers).send() {
        Ok(res) => {
            if res.status().is_success() {
                match res.json::<Value>() {
                    Ok(json) => {
                        if let Some(content) = json["body"].as_str() {
                            match fs::write("LICENSE", content) {
                                Ok(_) => {
                                    println!("{}", "LICENSE file created successfully.".green())
                                }
                                Err(e) => println!("{} {}", "Failed to write LICENSE:".red(), e),
                            }
                        } else {
                            println!("{}", "Invalid response format from GitHub API.".red());
                        }
                    }
                    Err(e) => println!("{} {}", "Failed to parse GitHub API response:".red(), e),
                }
            } else {
                println!("{} {}", "Failed to fetch license:".red(), res.status());
            }
        }
        Err(e) => println!("{} {}", "Failed to connect to GitHub API:".red(), e),
    }
}

fn create_github_issue(
    config: &Config,
    repo_name: &str,
    issue: &IssueInfo,
) -> Result<String, String> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static("Git-Mate-CLI"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("token {}", config.github_token))
            .map_err(|e| e.to_string())?,
    );

    let issue_data = json!({
        "title": issue.title,
        "body": issue.body,
        "labels": issue.labels
    });

    let url = format!("https://api.github.com/repos/{}/issues", repo_name);

    let res = client
        .post(&url)
        .headers(headers)
        .json(&issue_data)
        .send()
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let json: Value = res.json().map_err(|e| e.to_string())?;
        if let Some(html_url) = json["html_url"].as_str() {
            Ok(html_url.to_string())
        } else {
            Err("Failed to get issue URL from GitHub response".to_string())
        }
    } else {
        let status = res.status();
        let text = res.text().unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("GitHub API error ({}): {}", status, text))
    }
}

fn create_github_pr(
    config: &Config,
    repo_name: &str,
    title: &str,
    body: &str,
    base: &str,
    head: &str,
) -> Result<String, String> {
    let client = Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static("Git-Mate-CLI"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("token {}", config.github_token))
            .map_err(|e| e.to_string())?,
    );

    let pr_data = json!({
        "title": title,
        "body": body,
        "head": head,
        "base": base
    });

    let url = format!("https://api.github.com/repos/{}/pulls", repo_name);

    let res = client
        .post(&url)
        .headers(headers)
        .json(&pr_data)
        .send()
        .map_err(|e| e.to_string())?;

    if res.status().is_success() {
        let json: Value = res.json().map_err(|e| e.to_string())?;
        if let Some(html_url) = json["html_url"].as_str() {
            Ok(html_url.to_string())
        } else {
            Err("Failed to get PR URL from GitHub response".to_string())
        }
    } else {
        let status = res.status();
        let text = res.text().unwrap_or_else(|_| "Unknown error".to_string());
        Err(format!("GitHub API error ({}): {}", status, text))
    }
}

fn setup_workflow(workflow_type: &str) {
    // create .github/workflows directory if it doesn't exist
    let workflows_dir = Path::new(".github").join("workflows");
    fs::create_dir_all(&workflows_dir).unwrap_or_else(|e| {
        println!("{} {}", "Failed to create workflows directory:".red(), e);
        return;
    });

    let (filename, content) = match workflow_type {
        "ci" => (
            "ci.yml",
            r#"name: CI
        
            on:
            push:
            branches: [ main, master ]
            pull_request:
            branches: [ main, master ]
            
            jobs:
            build:
            runs-on: ubuntu-latest
            steps:
            - uses: actions/checkout@v3
    
            - name: Set up environment
            run: echo "Setting up environment"
      
            - name: Build
            run: echo "Building project"
      
            - name: Run tests
            run: echo "Running tests"
            "#,
        ),
        "deploy" => (
            "deploy.yml",
            r#"name: Deploy

            on:
            push:
            branches: [ main, master ]

            jobs:
            deploy:
            runs-on: ubuntu-latest
    
            steps:
            - uses: actions/checkout@v3
    
            - name: Set up environment
            run: echo "Setting up environment"
      
            - name: Build
            run: echo "Building project"
      
            - name: Deploy to production
            run: echo "Deploying to production"
            "#,
        ),
        _ => (
            "custom.yml",
            r#"name: Custom Workflow

            on:
            push:
            branches: [ main, master ]
            pull_request:
            branches: [ main, master ]

            jobs:
            custom:
            runs-on: ubuntu-latest
    
            steps:
            - uses: actions/checkout@v3
    
            - name: Custom step 1
            run: echo "Running custom step 1"
      
            - name: Custom step 2
            run: echo "Running custom step 2"
            "#,
        ),
    };

    let file_path = workflows_dir.join(filename);
    match fs::write(&file_path, content) {
        Ok(_) => println!(
            "{} {}",
            "GitHub Actions workflow created:".green(),
            file_path.display()
        ),
        Err(e) => println!("{} {}", "Failed to create workflow file:".red(), e),
    }
}
