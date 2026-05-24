#!/usr/bin/env bash
# Publie RUEST sur crates.io.
#
# Noms crates.io (package) vs code (import) :
#   ruest-framework  →  use ruest::   (le nom « ruest » est pris par un autre crate)
#   ruest-macros     →  (transitif, proc-macros)
#   ruest-db         →  use ruest_db::
#   ruest-cli        →  binaire `ruest`
#
# Prérequis : cargo login + e-mail vérifié sur https://crates.io/settings/profile
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
    if cargo publish -p "$crate" $ALLOW_DIRTY; then
      echo "    Waiting for index propagation..."
      sleep 25
    else
      echo "    Échec — voir le message ci-dessus."
      exit 1
    fi
  fi
}

# Déjà publié ? Commentez la ligne.
CRATES=(
  # ruest-macros
  ruest-framework
  ruest-db
  ruest-cli
)

for c in "${CRATES[@]}"; do
  publish "$c"
done

echo ""
echo "Terminé. Installation :"
echo "  cargo add ruest-framework   # dans le code : use ruest::prelude::*;"
echo "  cargo add ruest-db          # optionnel"
echo "  cargo install ruest-cli     # commande: ruest"
