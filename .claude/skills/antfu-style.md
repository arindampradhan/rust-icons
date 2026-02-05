# antfu-style

Apply Anthony Fu's coding philosophy to Rust development.

## Activation

Use when writing new code, reviewing PRs, or refactoring.

## Principles

### 1. Simplicity First
- Avoid over-engineering
- Prefer explicit over implicit
- One way to do things when possible

### 2. Developer Experience
- Clear error messages with context
- Helpful documentation
- Intuitive APIs

### 3. Composability
- Small, focused functions
- Traits for shared behavior
- Feature flags over conditional compilation when cleaner

### 4. Type Safety
- Leverage Rust's type system fully
- Prefer `Option`/`Result` over sentinel values
- Use newtypes for domain concepts

### 5. Code Organization
```rust
// Good: Clear module boundaries
mod components;
mod hooks;
mod utils;

// Good: Re-export public API
pub use components::*;
pub use hooks::*;
```

### 6. Error Handling
```rust
// Good: Contextual errors
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IconError {
    #[error("icon '{name}' not found in collection '{collection}'")]
    NotFound { name: String, collection: String },

    #[error("failed to fetch from API: {0}")]
    ApiError(#[from] reqwest::Error),
}
```

### 7. Testing
```rust
// Good: Tests close to code
impl IconSearch {
    pub fn search(&self, query: &str) -> Vec<Icon> {
        // ...
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_finds_exact_match() {
        // ...
    }
}
```

## Anti-Patterns to Avoid

- Premature abstraction
- Deep nesting
- Magic strings/numbers
- Implicit dependencies
- Overly clever code
