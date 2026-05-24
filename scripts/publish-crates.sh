#!/usr/bin/env bash
# Publie tous les crates RUEST sur crates.io dans l'ordre des dépendances.
# Prérequis : cargo login <API_TOKEN>  (https://crates.io/settings/tokens)
set -euo pipefail
cd "$(dirname "$0")/.."

DRY_RUN="${DRY_RUN:-}"
ALLOW_DIRTY="${ALLOW_DIRTY:---allow-dirty}"

publish() {
  local crate="$1"
  echo "==> Publishing ${crate}..."
  if [[ -n "$DRY_RUN" ]]; then
    cargo publish -p "$crate" --dry-run $ALLOW_DIRTY
  else
    cargo publish -p "$crate" $ALLOW_DIRTY
    echo "    Waiting for index propagation..."
    sleep 25
  fi
}

CRATES=(
  ruest-db-schema
  ruest-db-parser
  ruest-db-codegen
  ruest-db-runtime
  ruest-db-migrate
  ruest-di
  ruest-router
  ruest-config
  ruest-validation
  ruest-logger
  ruest-macros
  ruest-core
  ruest-http
  ruest-security
  ruest-testing
  ruest
  ruest-cli
)

for c in "${CRATES[@]}"; do
  publish "$c"
done

echo ""
echo "Terminé. Les utilisateurs peuvent faire :"
echo "  cargo add ruest"
echo "  cargo install ruest-cli"
