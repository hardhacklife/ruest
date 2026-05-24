# RuestDB — base de données type Prisma pour RUEST

Voir [BRD&PRD_PRISMA.md](../BRD&PRD_PRISMA.md) pour la vision complète.

## MVP livré

| Composant | Crate | Rôle |
|-----------|-------|------|
| AST | `ruest-db-schema` | Modèles, champs, attributs |
| Parser | `ruest-db-parser` | Lit `schema.ruest` |
| Codegen | `ruest-db-codegen` | SQL PostgreSQL + client Rust |
| Runtime | `ruest-db-runtime` | Pool SQLx async |
| Migrations | `ruest-db-migrate` | `ruestdb/migrations/` + table `_ruestdb_migrations` |

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

## CLI

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

## Utilisation dans le code

```rust
let db = RuestDb::connect_from_env().await?;
let client = RuestDbClient::new(db);

let users = client.user.find_many().await?;
let one = client.user.find_unique(id).await?;
let created = client.user.create(CreateUser { email, name }).await?;
client.user.update(id, UpdateUser { name: Some("x".into()), ..Default::default() }).await?;
client.user.delete(id).await?;
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

Les repositories manuels (`*_repository.rs`) peuvent être remplacés progressivement par le client généré. Le module `AuthModule` / JWT reste indépendant.

## Roadmap (hors MVP)

- `ruest db pull` (introspection)
- Guards sur `#[get(..., guards = ...)]`
- MongoDB, GraphQL, dashboard
