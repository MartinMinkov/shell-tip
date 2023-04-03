This Rust program is a command-line tool that leverages the OpenAI GPT-3.5 Turbo model to provide guidance and assistance for Linux shell commands. The AI-powered tool helps users navigate the Linux terminal by suggesting appropriate commands, explaining their usage, and offering best practices.

# Features

- Retrieves assistance and guidance for Linux shell commands from GPT-3.5 Turbo model.
- Displays response in a formatted and user-friendly way.
- Configurable parameters such as temperature and number of choices.

# Installation

To install the Linux Shell Assistant, follow these steps:

Make sure you have Rust and Cargo installed on your system. If not, follow the instructions at [rustup.rs](https://rustup.rs/) to install Rust.

Clone this repository:

```bash
git clone https://github.com/yourusername/shell-tip.git
```

Change into the cloned repository:

```bash
cd shell-tip
```

Build and install the binary:

```bash
./scripts/install.sh
```

# Usage

Run the Linux Shell Assistant by executing the following command:

```bash
shell-tip [FLAGS] [OPTIONS] <message>
```

## Parameters

- message:
  - The message to send to the AI. This should be the question or command you need help with.
- number_of_choices, -n:
  - The number of choices you'd like the AI to provide (default: 1).
- temperature, -t:
  - Controls the randomness of the AI's output. Higher values make the output more random, while lower values make it more focused (default: 0.6).

# Examples

Get help with a specific command:

```bash
shell-tip "How do I list all files in a directory?"
```

Get multiple suggestions with a higher temperature:

```bash
shell-tip -n 3 -t 0.8 "How do I find the size of a directory?"
```
