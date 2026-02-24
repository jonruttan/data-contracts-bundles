# Bundle Specs Index

Canonical bundle manifests discovered by `scripts/bundle`:

- `/specs/bundles/data_contracts_library_core_bundle_v1.yaml`
- `/specs/bundles/data_contracts_library_overlays_bundle_v1.yaml`
- `/specs/bundles/data_contracts_library_bundle_tooling_bundle_v1.yaml`
- `/specs/bundles/data_contracts_library_review_workflow_bundle_v1.yaml`
- `/specs/bundles/data_contracts_runner_core_bundle_v1.yaml`
- `/specs/bundles/data_contracts_runner_shared_behavior_bundle_v1.yaml`
- `/specs/bundles/data_contracts_runner_contract_sets_shared_bundle_v1.yaml`
- `/specs/bundles/data_contracts_lang_project_scaffold_bundle_v1.yaml`
- `/specs/bundles/data_contracts_lang_rust_project_scaffold_bundle_v1.yaml`
- `/specs/bundles/runner_contract_bundle_v1.yaml`
- `/specs/bundles/spec_core_maintenance_bundle_v1.yaml`
- `/specs/bundles/project_docs_maintenance_bundle_v1.yaml`

Bundler runtime commands are described by:

- `/specs/contracts/bundler_cli_contract_v1.md` (tooling contract)
- `/specs/05_libraries/bundle_tooling/runner_entrypoints_v1.yaml` (runner entrypoints)
- `/specs/05_libraries/bundle_tooling/bundler_job_contracts.spec.md` (job assertions)

Installers and operator scripts should discover upgrades via:

- `scripts/bundle list`
- `scripts/bundle info --bundle-id <id>`
- `scripts/bundle outdated --project-lock <path>`
- `scripts/bundle upgrade --project-lock <path> --dry-run`
