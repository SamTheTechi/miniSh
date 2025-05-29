# mini\_sh - A Minimal Bash-Inspired Shell in Rust using libc

## Supported Features

* **Command Parsing**: Multiple commands separated by `;` are parsed and executed.
* **Built-in Commands**:

  * `cd`: Change the current working directory.
  * `aliases"`: Define custom aliases.
  * `exports`: Export or append to environment variables.
  * `exit`: Exit the shell.
* **Configuration Support**:

  * Reads a `~/minish.conf` file at startup to preload aliases and environment exports.
* **Command Execution**:

  * Executes non-built-in commands via `execvp` using libc.
  * Forks and waits for child processes to finish.

## Purpose

`mini_sh` is a simple, shell written in Rust. It is inspired by the Unix Bash shell and is intended to:

* Teach the fundamentals of building a shell from scratch.
* Demonstrate system-level programming in Rust using `libc`.
* Explore command parsing, process management, and environment configuration.

## 🐞 Bugs & Contributions
Found a bug? Want to contribute? Feel free to open an issue or submit a pull request!

## 📜 License
This project is licensed under the **MIT License**.

---

Made with ❤️ by [Sameer Gupta](#)
