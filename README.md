# Skeletons to start with cyberfabric and modkit

You can copy/paste what's inside `Init` or use the modules from `Modules` as a base for your projects.

These templates are intended to work with [cyberfabric-cli](https://github.com/cyberfabric/cf-cli)

Check the `README.md` in each of the modules to see the architecture and how it works.

## Developing templates

Generate a project manually with `cargo generate --path Init --name my-project` or `cargo generate --path Modules/<template> --name my-module`.

The [cf-cli](https://github.com/cyberfabric/cf-cli) leverage this cargo-generate tool for the use cases inside cyberfabric.

Validate every template from the repo root with `bacon`. The default Bacon job fans out one validation per template from `bacon.toml`, while `scripts/validate-templates.sh` validates a single template at a time. Each validation writes generated output under `.bacon/validate-templates`, keyed by template path and generated project name, uses its own Cargo target directory there, then runs `cargo clippy --workspace --all-targets --all-features -- -D warnings` and `cargo test --workspace --all-targets --all-features` on the generated output.

## Testing

With `bacon`:

```bash
bacon
```

If you want to validate a single template with `bacon`, run one of the named jobs from `bacon.toml`:

```bash
bacon validate-init
bacon validate-api-db-handler
bacon validate-api-fetcher
bacon validate-background-worker
bacon validate-api-gateway
bacon validate-db-handler
bacon validate-rest-api
```

Without `bacon`, run the validation script directly from the repo root:

```bash
./scripts/validate-templates.sh Init
./scripts/validate-templates.sh Modules/api-db-handler generated-template
./scripts/validate-templates.sh Modules/api-fetcher generated-template
./scripts/validate-templates.sh Modules/background-worker generated-template
./scripts/validate-templates.sh Modules/api-gateway generated-template
./scripts/validate-templates.sh Modules/db-handler generated-template
./scripts/validate-templates.sh Modules/rest-api generated-template
```

To validate every template without `bacon`:

```bash
./scripts/validate-templates.sh Init
./scripts/validate-templates.sh Modules/api-db-handler generated-template
./scripts/validate-templates.sh Modules/api-fetcher generated-template
./scripts/validate-templates.sh Modules/background-worker generated-template
./scripts/validate-templates.sh Modules/api-gateway generated-template
./scripts/validate-templates.sh Modules/db-handler generated-template
./scripts/validate-templates.sh Modules/rest-api generated-template
```

If you want bacon to watch the whole repository from the root, run `bacon --watch .`.
