# Data Contracts Bundles

Canonical bundle-manifest and payload repository for data-contract ecosystem
bundle release assets.

## Discovery

- Canonical bundle manifests: `/specs/bundles`
- Bundler contracts and tooling specs: `/specs/**`
- Runtime entrypoint contracts: `specs/05_libraries/bundle_tooling/runner_entrypoints_v1.yaml`
- Canonical command registry: `data-contracts-bundles/scripts/bundle`

## Command Surface

The bundler CLI is explicit-only:

- `scripts/bundle resolve --runner <runner> --root <bundle_id> --out <dir> [--repo-root <dir>] [--source-*]`
- `scripts/bundle package --runner <runner> --root <bundle_id> --version <version> --out <dir> [--repo-root <dir>] [--source-*]`
- `scripts/bundle package-check --package <path> --sha256 <path>`
- `scripts/bundle list [--json]`
- `scripts/bundle info --bundle-id <bundle_id> [--json]`
- `scripts/bundle install --project-lock <path> --out <dir>`
- `scripts/bundle install-check --project-lock <path> --out <dir>`
- `scripts/bundle bootstrap --lock <path> --out <dir> [--force-bootstrap]`
- `scripts/bundle bootstrap-check --state <path>`
- `scripts/bundle outdated --project-lock <path> [--format text|json]`
- `scripts/bundle upgrade --project-lock <path> [--bundle-id <id>...] [--to <version|latest>] [--out <path|stdout>] [--dry-run|--write-lock] [--allow-major]`

No compatibility aliases are exposed by this CLI contract.

## Canonical Library Bundle IDs

- `data-contracts-library-core`
- `data-contracts-library-overlays`
- `data-contracts-library-bundle-tooling`
- `data-contracts-library-review-workflow`
- `data-contracts-runner-core`
- `data-contracts-runner-shared-behavior`
- `data-contracts-runner-contract-sets-shared`
- `data-contracts-lang-project-scaffold`
- `data-contracts-lang-rust-project-scaffold`
- `data-contracts-runner-contract`

## Release Assets

- `data-contract-bundle-<bundle_id>-v<bundle_version>.tar.gz`
- `data-contract-bundle-<bundle_id>-v<bundle_version>.tar.gz.sha256`
- `data-contract-bundle-<bundle_id>-v<bundle_version>.tar.gz.intoto.json`
