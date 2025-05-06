# Git Mate - GitHub Project Management Made Easy

Git-Mate is a powerful command-line tool that automates GitHub workflows, from initializing repositories to managing branches, issues, and pull requests - all without manual Git commands.

## Installation

### Option 1: Install from cargo (Recommended)

If you have Rust and Cargo installed:

```bash
cargo install git_mate
```

### Option 2: Build from source

1. Clone the repository:

   ```bash
   git clone https://github.com/aliwert/git-mate.git
   cd git-mate
   ```

2. Build and install:
   ```bash
   cargo install --path .
   ```

## Initial Setup

Before using Git-Mate, configure it with your GitHub credentials:

```bash
git_mate config
```

You'll be prompted to enter:

- Your GitHub username
- A Personal Access Token (PAT) with "repo" scope
- Default branch name (defaults to "main")

To generate a GitHub PAT:

1. Go to GitHub → Settings → Developer settings → Personal access tokens → Generate new token
2. Give it a name and select the "repo" scope
3. Copy the generated token

## Core Commands

### Repository Management

```bash
# Initialize a new project and push to GitHub (interactive mode)
git_mate init

# Initialize with command-line arguments
git_mate init --name "my-project" --desc "My awesome project" --private

# Initialize with .gitignore template and license
git_mate init --gitignore Rust --license MIT

# Initialize with GitHub Actions workflow
git_mate init --workflow ci
```

### Commit & Push

```bash
# Commit and push changes
git_mate push

# Specify a commit message
git_mate push --message "Update documentation"
```

### Branch Management

```bash
# Create a new branch
git_mate branch create feature-branch

# Create and checkout a new branch
git_mate branch create feature-branch --checkout

# List all branches
git_mate branch list

# Switch to another branch
git_mate branch switch feature-branch
```

### .gitignore Templates

```bash
# Set up a .gitignore file with interactive template selection
git_mate gitignore

# Set up a specific template
git_mate gitignore Rust
```

### GitHub Issues

```bash
# Create an issue interactively
git_mate issue

# Create an issue with parameters
git_mate issue --title "Bug in login" --body "Login fails on Firefox" --label bug --label priority
```

### Pull Requests

```bash
# Create a PR interactively from current branch
git_mate pr

# Create a PR with parameters
git_mate pr --title "Add login feature" --body "Implements user authentication" --base main --head feature-branch
```

### GitHub Actions Workflows

```bash
# Set up a workflow interactively
git_mate workflow

# Set up a specific workflow
git_mate workflow ci  # Options: ci, deploy, custom
```

## Configuration Options

```bash
# Update GitHub token
git_mate config --token "new-github-token"

# Update GitHub username
git_mate config --username "new-username"

# Update default branch
git_mate config --default-branch "main"
```

## Features

- Simple, intuitive command-line interface
- Interactive prompts with smart defaults
- Automated Git operations
- GitHub API integration
- Branch, issue, and PR management
- .gitignore template selection
- License template application
- GitHub Actions workflow setup
- Secure credential storage
- Colored terminal output for better readability

## Need Help?

```bash
# Show general help
git_mate --help

# Show help for a specific command
git_mate init --help
git_mate push --help
```
