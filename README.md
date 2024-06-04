# A Rusty Shell

This is just a simple shell made in Rust as a learning project.
It was based on Codecrafter's shell course: [https://app.codecrafters.io/courses/shell/]

It has the following builtin shell commands:

- pwd: See current dir
- cd: Go to a directory. Accepts both relative, absolute paths and home dir (~)
- type: Check if argument is a type
- Echo: Just prints back argument given

You can also execute programs either by giving the path for it, or based on PATH env var.
