# Version 0.0.1 (May 09th 2021)

- Proposed Todo App CLI
- To add features and create documentation

# Version 0.1.0 (May 13th 2021)

- Accepts arguments via `get_args()` function, implemented in [`args.rs`](./src/utils/io/args.rs)
- Added `--help` and `--version` commands, implemented in `manual.rs` and [`commands.rs`](./src/utils/commands/commands.rs), **NOTE: A change has been in `utils` folder, please check v0.1.3**
- Throws error and warnings with meaningful messages, implemented in [`errors.rs`](./src/utils/error/errors.rs) and [`warnings.rs`](./src/utils/warnings/warnings.rs) respectively
- Created `user_input()` function to return inputted data, implemented in [`input.rs`](./src/utils/io/input.rs)
- Apply font style using escape codes, implemented in [`color_texts.rs`](./src/utils/ui/color_texts.rs)

# Version 0.1.2 (May 14th 2021)
- Changes made in all comments for functions
- Code inside `manual.rs` has been shifted to [`commands.rs`](./src/utils/commands/commands.rs)
- Connection with database is done successfully
- Add and read method has been implemented