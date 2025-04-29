use clap::{App, AppSettings, Arg, SubCommand};

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
}
