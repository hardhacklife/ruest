#!/usr/bin/env bash
# Publie RUEST sur crates.io — 4 packages seulement.
#
# | Package        | Rôle                                      |
# |----------------|-------------------------------------------|
# | ruest-macros   | proc-macros (#[module], #[controller], …) |
# | ruest          | framework complet (DI, HTTP, JWT, …)      |
# | ruest-db       | RuestDB (schema, migrations, SQLx)        |
# | ruest-cli      | binaire `ruest`                           |
#
# Prérequis : cargo login + e-mail vérifié sur crates.io
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
  ruest-macros
  ruest
  ruest-db
  ruest-cli
)

for c in "${CRATES[@]}"; do
  publish "$c"
done

echo ""
echo "Terminé. Installation :"
echo "  cargo add ruest"
echo "  cargo add ruest-db          # optionnel"
echo "  cargo install ruest-cli"
