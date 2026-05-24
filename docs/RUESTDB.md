# RuestDB — base de données type Prisma pour RUEST

Un seul package : **`cargo add ruest-db`**

Voir [BRD&PRD_PRISMA.md](../BRD&PRD_PRISMA.md) pour la vision complète.

## Contenu du crate `ruest-db`

| Module | Rôle |
|--------|------|
| `schema` | AST — modèles, champs, attributs |
| `parser` | Lit `schema.ruest` |
| `codegen` | SQL PostgreSQL + client Rust généré |
| `runtime` | Pool SQLx async (`RuestDb`) |
| `migrate` | `ruestdb/migrations/` + CLI helpers |

## Installation

```bash
cargo add ruest-db
```

Dans le code généré (`generated/ruestdb/`), les imports pointent vers `ruest_db` :

```rust
use ruest_db::{RuestDb, RuestDbError, Row};
```

## Schema (`schema.ruest`)

```prisma
model User {
  id    String @id @default(uuid())
  email String @unique
  name  String
  posts Post[]
}

model Post {
  id     String @id @default(uuid())
  title  String
  userId String
  user   User @relation(fields: [userId], references: [id])
}
```

Types : `String`, `Int`, `Float`, `Boolean`, `DateTime`, `UUID`  
Attributs : `@id`, `@unique`, `@default(uuid())`, `@default(now())`, `@relation(...)`

## CLI (`ruest`)

```bash
ruest db init              # schema.ruest + ruestdb/migrations/
ruest generate             # alias : génère generated/ruestdb/
ruest db generate          # idem
ruest migrate dev          # crée migration + applique
ruest migrate deploy       # applique en attente
ruest migrate reset        # drop + réapplique (dev)
```

## Configuration

```env
DATABASE_URL=postgres://user:password@localhost:5432/app
```

## Utilisation

```rust
use ruest_db::RuestDb;

let db = RuestDb::connect_from_env().await?;
let client = RuestDbClient::new(db);

let users = client.user.find_many().await?;
```

API migrate / generate depuis Rust :

```rust
use ruest_db::{db_init, generate_client, parse_schema};
```

## Exemple

[`examples/ruest-db-demo/`](../examples/ruest-db-demo/) — build régénère le client depuis `schema.ruest`.

```bash
cd examples/ruest-db-demo
export DATABASE_URL=postgres://localhost/shop
ruest db init
ruest migrate dev
cargo run -p ruest-db-demo
```

## Intégration RUEST

Les repositories manuels peuvent être remplacés progressivement par le client généré. JWT / guards restent dans `ruest` + `ruest-security`.
