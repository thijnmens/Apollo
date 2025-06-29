# ![Apollo](https://github.com/user-attachments/assets/1eae7e64-f975-45ae-b953-3dcb570c3b58)

<a href="https://codecov.io/github/thijnmens/apollo" > 
 <img src="https://codecov.io/github/thijnmens/apollo/graph/badge.svg?token=2ASUHDWY90" alt="codecov"/> 
</a>
<a href="https://img.shields.io/crates/v/apollo-logger.svg">
    <img src="https://img.shields.io/crates/v/apollo-logger.svg" alt="crates" />
</a>

**A logging library for Rust**

> [!WARNING]
> This crate is still under development!

# Feature List
- [x] Different logging levels
- [x] Colors in the terminal
- [ ] Automatically writes to log file
- [ ] Fully customizable

# Usage
```rust
use apollo_logger::Apollo;

fn main() {
    let l = Apollo::new();
    // or
    // use apollo_logger::levels::Levels;
    // let l = Apollo { logging_level: Levels::DEBUG };

    l.debug("This is a debug message");
    l.info("This is an info message");
    l.warn("This is a warning message");
    l.error("This is an error message");
    l.critical("This is a critical error message");
}

```
which will output

![Screenshot 2025-06-29 164908](https://github.com/user-attachments/assets/01b483aa-2907-46d2-9fdc-b97d105c01ec)
