---
sidebar_position: 4
sidebar_label: Config Components
---

# Config Components

## Config components — `#[derive(ComponentConfig)]`

A config struct whose fields are read from environment variables once at registration time.

```rust
use axumstart_components::ComponentConfig;

#[derive(Clone, ComponentConfig)]
struct AppConfig {
    #[env_var("DATABASE_URL")]
    database_url: String,
    #[env_var("PORT", 8080)]
    port: u16,
}
```

`#[env_var("VAR")]` panics at resolution time if the variable is unset or fails to parse.
`#[env_var("VAR", default)]` falls back to `default` (any expression of the field's type) if the
variable is unset, but still panics on a parse failure.
