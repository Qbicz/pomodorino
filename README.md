# Pomodorino - Rust command-line pomodoro timer

[![Actions Status](https://github.com/Qbicz/pomodorino/workflows/Cargo%20Build%20&%20Test/badge.svg)](https://github.com/Qbicz/pomodorino/actions)

🍅🦀

[Pomodoro Technique](https://en.wikipedia.org/wiki/Pomodoro_Technique)

### VS Code setup for efficient Rust workflow

This will autoformat your code on save, and also provide `cargo clippy` insights directly in the editor.

Open Preferences (Ctrl + Shit + P) and add:

```
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer",
        "editor.formatOnSave": true,
    },
    "rust-analyzer.checkOnSave.command": "clippy",
```
