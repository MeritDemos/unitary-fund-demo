# Merit CLI Demo

A command-line interface tool that leverages various AI providers (OpenAI, Anthropic Claude, DeepSeek, Google Gemini) to assist with Git operations and repository analysis. This tool helps developers with:

- 📝 Generating meaningful commit messages based on changes
- 🔍 Analyzing file changes and their impact
- 👥 Analyzing contributor patterns and contributions

## Features

- **Smart Commit Messages**: Automatically generates conventional commit messages based on your changes
- **File Analysis**: Get detailed insights about the changes you've made
- **Contributor Analysis**: Understand contribution patterns and developer focus areas
- **Multiple AI Providers**: Support for various AI providers (OpenAI, Claude, DeepSeek, Gemini)
- **Interactive CLI**: User-friendly interface with clear prompts and options

## Prerequisites

- Rust and Cargo installed
- Git installed
- At least one API key from a supported AI provider

## Installation

1. Clone the repository:
```bash
git clone [repository-url]
cd merit-cli-demo
```

2. Build the project:
```bash
cargo build --release
```

## Configuration

Create a `.env` file in the project root with at least one of the following API keys:

```env
OPENAI_API_KEY=your_openai_api_key
ANTHROPIC_API_KEY=your_anthropic_api_key
DEEPSEEK_API_KEY=your_deepseek_api_key
GEMINI_API_KEY=your_gemini_api_key
```

The application will automatically detect available providers based on the API keys you've configured.

## Usage

Run the tool from your terminal:

```bash
cargo run
```

Or if you've built the release version:

```bash
./target/release/merit-cli-demo
```

You can optionally specify a repository path:

```bash
cargo run /path/to/repository
```

The tool will present an interactive menu with the following options:

1. **Generate Commit Message**: Analyzes your changes and suggests a conventional commit message
2. **Analyze File Changes**: Provides detailed analysis of the changes in your working directory
3. **Analyze Contributors**: Analyzes contribution patterns and developer activities

## Development

This project is built with Rust and uses several key dependencies:

- `tokio`: Async runtime
- `git2`: Git operations
- `reqwest`: HTTP client for API calls
- `dialoguer`: Interactive CLI components
- `dotenv`: Environment variable management