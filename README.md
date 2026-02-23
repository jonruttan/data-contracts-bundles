# data-contracts-bundles

Canonical librarian repository for Data Contracts bundle manifests and bundle package release assets.

## Purpose

This repository is the canonical source for:
- bundle manifests under `specs/bundles/`
- bundle package publishing metadata and workflows
- canonical scaffold bundles:
  - `data-contracts-lang-project-scaffold`
  - `data-contracts-lang-rust-project-scaffold`

Bundle package asset naming is canonical:
- `data-contract-bundle-<bundle_id>-v<bundle_version>.tar.gz`
- `data-contract-bundle-<bundle_id>-v<bundle_version>.tar.gz.sha256`
- `data-contract-bundle-<bundle_id>-v<bundle_version>.tar.gz.intoto.json`

## CLI

Producer operations are provided by:
- `scripts/bundle resolve`
- `scripts/bundle package`
- `scripts/bundle package-check`

## Schemas

- `specs/schema/bundle_manifest_v1.yaml`
- `specs/schema/resolved_bundle_lock_v1.yaml`
