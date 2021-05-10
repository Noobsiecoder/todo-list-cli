<h1 align="center">Welcome to todo-list-cli 👋</h1>
<p>
  <img alt="Version" src="https://img.shields.io/badge/version-0.1.0-blue.svg?cacheSeconds=2592000" />
  <img alt="Build" src="https://img.shields.io/badge/build-unstable-red.svg?cacheSeconds=2592000" />
  <a href="https://github.com/Noobsiecoder/todo-list-cli#readme" target="_blank">
    <img alt="Documentation" src="https://img.shields.io/badge/documentation-yes-brightgreen.svg" />
  </a>
  <a href="https://opensource.org/licenses/MIT" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  </a>
</p>

> A Todo List CLI created using Rust and SQLite

### 🏠 [Homepage](https://github.com/Noobsiecoder/todo-list-cli)

### ✨ [Webpage](https://github.com/Noobsiecoder/todo-list-cli)

## Features

**Version 1.0.0 Proposal**

- CRUD operation for :-
  - Tasks
  - Notes for each tasks
  - Start and end date
  - Deadline Time
  - Mark Important
  - Status of completion

## I/O Operation commands

### Create task

```bash
todo add
```

### Display tasks

```bash
todo read --sort=["start", "end", "dl", "imp", "comp"]
```

| tkid | Tasks       | Note                             |  Start   |   End    | Deadline | Important | Completed |
| :--: | :---------- | :------------------------------- | :------: | :------: | :------: | :-------: | :-------: |
|  1   | Hit the gym | Take gym bag, don't forget towel | 12-05-21 | 12-05-21 | 06:55 AM |    Yes    |    No     |
|  2   | Eat Food    | Have eggs and fruits             | 12-05-21 | 13-05-21 | 08:00 AM |    Yes    |    Yes    |

### Update task

```bash
todo edit
```

### Delete task

```bash
todo del
```

## Install

```sh
cargo build
```

## Usage

```sh
cargo run
```

## Run tests

```sh
cargo test <file_name>
```

## Author

👤 **Abhishek Sriram**

- Website: https://github.com/Noobsiecoder/
- Github: [@Noobsiecoder](https://github.com/Noobsiecoder)

## Show your support

Give a ⭐️ if this project helped you!

## 📝 License

Copyright © 2021 [Abhishek Sriram](https://github.com/Noobsiecoder).<br />
This project is [MIT](https://opensource.org/licenses/MIT) licensed.

---