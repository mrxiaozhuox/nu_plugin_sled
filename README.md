## nu_plugin_sled

This plugin can help you manage your [`sled`](https://github.com/spacejam/sled?tab=readme-ov-file) database in nushell.

### Usage:

```nushell
> { name: "nu_plugin_sled", id: 1, update_at: (date now) } | sled-save db

> sled-open db
╭───────────┬────────────────╮
│ id        │ 1              │
│ name      │ nu_plugin_sled │
│ update_at │ 1731923169     │
╰───────────┴────────────────╯


> sled-open db | update id 10086 | sled-save db

> sled-open db
╭───────────┬────────────────╮
│ id        │ 10086          │
│ name      │ nu_plugin_sled │
│ update_at │ 1731923169     │
╰───────────┴────────────────╯
```

### Data Structure

`nu_plugin_sled` using [`rmp-serde`](https://crates.io/crates/rmp-serde) convert struct & data to binary.
