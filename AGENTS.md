# Repository Guidelines

## Project Structure & Module Organization

This is a small Rust Cargo project for LeetCode solutions.

- `Cargo.toml` defines the `leetcode` package and Rust edition.
- `src/main.rs` is the binary entry point and declares solution modules with `mod ...;`.
- `src/*.rs` contains individual problem solutions. Current files use Chinese LeetCode problem titles, for example `src/最长回文子串.rs`.

Keep each problem in its own module file. When adding a new solution, add the file under `src/` and declare it from `src/main.rs` if it should be compiled.

## Build, Test, and Development Commands

Use Cargo for all local workflows:

- `cargo check` verifies the project compiles quickly.
- `cargo build` builds the debug binary.
- `cargo run` runs `src/main.rs`.
- `cargo test` runs unit tests and documentation tests.
- `cargo fmt` formats Rust files using `rustfmt`.
- `cargo clippy --all-targets --all-features` runs lint checks when Clippy is installed.

Run `cargo fmt` and `cargo test` before submitting changes.

## Coding Style & Naming Conventions

Follow standard Rust formatting via `cargo fmt`; use 4-space indentation and idiomatic Rust names:

- Functions and variables: `snake_case`.
- Types and structs: `PascalCase`.
- Constants: `SCREAMING_SNAKE_CASE`.
- Module files: match the problem title or a clear lowercase identifier.

LeetCode plugin markers such as `// @lc code=start` and `// @lc code=end` may remain in solution files. Keep problem metadata comments intact when they help identify the source problem.

## Testing Guidelines

There is no separate `tests/` directory yet. Prefer adding focused unit tests in the same solution file with:

```rust
#[cfg(test)]
mod tests {
    use super::*;
}
```

Name tests after the behavior or sample case, such as `sample_babad_returns_palindrome`. Cover LeetCode sample inputs plus edge cases like empty-like boundaries, single-character strings, duplicates, and minimum or maximum constraints where practical.

## Commit & Pull Request Guidelines

The current Git history only contains `init`, so no detailed convention is established. Use short, imperative commit messages such as `add longest palindrome solution` or `test palindrome edge cases`.

Pull requests should include:

- A brief description of the problem or change.
- The relevant LeetCode problem number or link when applicable.
- The commands run locally, especially `cargo fmt` and `cargo test`.
- Notes about incomplete solutions, known failures, or performance tradeoffs.

## Agent-Specific Instructions

Keep generated changes narrowly scoped. Do not rewrite unrelated solution files or alter LeetCode metadata unless the task requires it.

## LeetCode Practice Workflow

The user is practicing the LeetCode China "Top 100 Liked" study plan:

- Study plan URL: `https://leetcode.cn/studyplan/top-100-liked/`
- When the user says `下一题`, provide the next problem in this study plan, including the problem link and LeetCode problem number.
- When the user says `给我一些思路`, explain the current problem's approach progressively. Start with a brute-force idea, explain its limitations, then guide toward the optimal solution. Do not provide code in this step.
- 当用户说`实现`，在对话中提供完整的代码。除非用户明确要求修改文件，否则不要编辑存储库文件。
- When the user says `检查代码`, only review the user's code and reasoning. If the solving idea is incorrect, state that the approach is not viable and explain the correct solving idea. If the algorithm is viable but the implementation is wrong, point out the specific bug or mistake. If a more optimal solution exists, mention that an optimal solution exists and briefly describe its direction, but do not provide complete code in this step.
- When the user says `给我同类型的题目`, recommend problems of the same type or pattern. These recommendations are not limited to the Top 100 Liked study plan.

Track the current problem based on the conversation and active solution file when possible. If the next problem or current problem is ambiguous, ask a concise clarifying question before proceeding.
