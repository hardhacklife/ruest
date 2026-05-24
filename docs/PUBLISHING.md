# Publier RUEST sur crates.io

Repo : [github.com/hardhacklife/ruest](https://github.com/hardhacklife/ruest)

## Ce que les utilisateurs obtiennent

```bash
# Framework dans un projet
cargo add ruest

# CLI globale (commande `ruest`)
cargo install ruest-cli
```

La doc API est générée automatiquement sur [docs.rs/ruest](https://docs.rs/ruest) après publication du crate `ruest`.

## Prérequis (une seule fois)

1. Compte [crates.io](https://crates.io/) (connexion GitHub possible).
2. Token API : [crates.io/settings/tokens](https://crates.io/settings/tokens) → **New token**.
3. Localement :

```bash
cargo login
# coller le token
```

4. Vérifier que les noms sont libres (ou vous appartiennent) :  
   `ruest`, `ruest-db`, `ruest-core`, `ruest-macros`, `ruest-cli`, etc.

5. Pousser le code **rebrandé** sur [github.com/hardhacklife/ruest](https://github.com/hardhacklife/ruest) — le repo doit contenir `ruest/`, `ruest-db/`, et plus `rustforge/` ni `forge-db/`.

## Test sans publier

```bash
chmod +x scripts/publish-crates.sh
# Valide le premier crate ; les suivants exigent les deps déjà sur crates.io
cargo publish -p ruest-db --dry-run --allow-dirty
cargo publish -p ruest --dry-run --allow-dirty   # après publication de toute la chaîne
```

`DRY_RUN=1 ./scripts/publish-crates.sh` ne peut pas simuler toute la chaîne : à partir du 2ᵉ crate, Cargo cherche les dépendances sur crates.io.

## Publication réelle

```bash
git tag v0.1.0
git push origin master --tags

./scripts/publish-crates.sh
```

Le script publie **4 crates** seulement (~25 s entre chaque upload) :

| Crate | Contenu |
|-------|---------|
| `ruest-macros` | `#[module]`, `#[controller]`, `#[routes]`, … (obligatoire séparé : proc-macro) |
| `ruest` | DI, core, HTTP Axum, router, config, validation, logger, security, testing |
| `ruest-db` | RuestDB (schema, migrations, client SQLx) |
| `ruest-cli` | commande `ruest` |

`cargo add ruest` tire `ruest-macros` en dépendance transitive — vous n’avez pas à l’ajouter à la main.

## Après publication

```bash
cargo add ruest@0.1
cargo add ruest-db@0.1   # base de données (optionnel)
cargo install ruest-cli
```

Créer une **GitHub Release** `v0.1.0` avec les notes de version.

## Dépannage

| Erreur | Action |
|--------|--------|
| `crate already exists` | Le nom est pris ; choisir un préfixe ou réclamer le crate |
| `dependency ... not found` | Publier les crates dépendantes d’abord (voir ordre dans le script) |
| `missing readme` | Vérifier `readme.workspace = true` dans le `Cargo.toml` du crate |
| `403` / auth | Refaire `cargo login` |

## Mise à jour de version

1. Incrémenter `version` dans `[workspace.package]` du `Cargo.toml` racine.
2. Commit + tag `v0.1.1`.
3. Relancer `./scripts/publish-crates.sh`.
