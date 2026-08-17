# prek-update-hook

A [prek][1]-only hook that checks whether the hook repositories pinned in your prek config (`prek.toml` or `.pre-commit-config.yaml`) are up to date, and can apply the updates for you.

Written in Rust, with no runtime dependencies. The hook binary only wraps `prek update` (a prek-specific command), so this repository is **not** compatible with upstream `pre-commit`.

## Usage

Add the repo to your `prek.toml`:

```toml
[[repos]]
hooks = [{ id = "prek-update" }]
repo = "https://github.com/paveltsialnou/prek-update-hook"
rev  = "v0.2.0"
```

The hook runs automatically before every commit (`pre-commit` stage). To apply the available updates, enable the fixer via `args`:

```toml
[[repos]]
hooks = [{ args = ["--fix"], id = "prek-update" }]
repo = "https://github.com/paveltsialnou/prek-update-hook"
rev  = "v0.2.0"
```

To skip specific repositories during fix:

```toml
[[repos]]
hooks = [{ args = ["--fix", "--exclude-repo=https://github.com/paveltsialnou/prek-update-hook"], id = "prek-update" }]
repo = "https://github.com/paveltsialnou/prek-update-hook"
rev  = "v0.2.0"
```

## Behavior

- **check mode** (default): runs `prek update --check` and exits non-zero when any pinned repository has a newer release available.
- **fix mode** (`--fix`): runs `prek update` and rewrites `rev` in your config to the latest versions.
- **`--exclude-repo=<REPO>`**: skip a specific repository during `--fix`. May be specified multiple times.

Additional arguments are forwarded to `prek update`, for example
`--cooldown-days`, `--repo`, or `--exclude-tag`.

## Development

Validate the manifest:

```sh
prek validate-manifest .pre-commit-hooks.yaml
```

Build and test:

```sh
cargo build
prek try-repo . prek-update --verbose
```

[1]: https://prek.j178.dev/
