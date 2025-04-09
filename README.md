# env-expand

A simple cross-platform Rust utility to expand environment variable placeholders in strings.

Supports:
- Unix-style: `$VAR`, `${VAR}`
- Windows-style: `%VAR%`

Missing environment variables are replaced with empty strings by default. In the future they will error out


## Usage

Add to your project:

```rust
mod env_expand;

use env_expand::expand_env_vars;

fn main() {
    std::env::set_var("USERNAME", "alice");
    let input = "Hello $USERNAME!";
    
    match expand_env_vars(input) {
```
