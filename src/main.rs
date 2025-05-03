use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use colored::*;
use dialoguer::Input;
use std::fs;
use std::path::Path;
use std::str;
/*const CONFIG_DIR: &str = ".aliwert";
const CONFIG_FILE: &str = "config.json";
const GITIGNORE_API_URL: &str = "https://api.github.com/gitignore/templates";*/
const VERSION: &str = "0.1.0";

fn main() {
    let app = App::new("aliwert")
        .version(VERSION)
        .author("Aliwert CLI")
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
        ("init", Some(sub_matches)) => init_command(sub_matches),
        _ => unreachable!("Subcommand should be handled above"),
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
                "No GitHub configuration found. Please run 'aliwert config' first.".red()
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
            "Not a git repository. Run 'aliwert init' first.".red()
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
            .default(config.username)
            .interact()
            .unwrap_or_else(|_| config.username);

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
            "Not a git repository. Run 'aliwert init' first.".red()
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
