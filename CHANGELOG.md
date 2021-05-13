# Version 0.0.1 (May 09th 2021)

- Proposed Todo App CLI
- To add features and create documentation

# Version 0.0.3 (May 13th 2021)

- Accepts arguments via `get_args()` function, implemented in [`args.rs`](./src/utils/io/args.rs)
- Added `--help` and `--version` commands, implemented in [`manual.rs`](./src/utils/commands/manual.rs) and [`commands.rs`](./src/utils/commands/commands.rs)
- Throws error and warnings with meaningful messages, implemented in [`errors.rs`](./src/utils/error/errors.rs) and [`warnings.rs`](./src/utils/warnings/warnings.rs) respectively
- Created `user_input()` function to return inputted data, implemented in [`input.rs`](./src/utils/io/input.rs)
- Apply font style using escape codes, implemented in [`color_texts.rs`](./src/utils/ui/color_texts.rs)
