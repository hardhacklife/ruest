# ForgeDB — base de données type Prisma pour RustForge

Voir [BRD&PRD_PRISMA.md](../BRD&PRD_PRISMA.md) pour la vision complète.

## MVP livré

| Composant | Crate | Rôle |
|-----------|-------|------|
| AST | `forgedb-schema` | Modèles, champs, attributs |
| Parser | `forgedb-parser` | Lit `schema.forge` |
| Codegen | `forgedb-codegen` | SQL PostgreSQL + client Rust |
| Runtime | `forgedb-runtime` | Pool SQLx async |
| Migrations | `forgedb-migrate` | `forgedb/migrations/` + table `_forgedb_migrations` |

## Schema (`schema.forge`)

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
forge db init              # schema.forge + forgedb/migrations/
forge generate             # alias : génère generated/forgedb/
forge db generate          # idem
forge migrate dev          # crée migration + applique
forge migrate deploy       # applique en attente
forge migrate reset        # drop + réapplique (dev)
```

## Configuration

```env
DATABASE_URL=postgres://user:password@localhost:5432/app
```

## Utilisation dans le code

```rust
let db = ForgeDb::connect_from_env().await?;
let client = ForgeDbClient::new(db);

let users = client.user.find_many().await?;
let one = client.user.find_unique(id).await?;
let created = client.user.create(CreateUser { email, name }).await?;
client.user.update(id, UpdateUser { name: Some("x".into()), ..Default::default() }).await?;
client.user.delete(id).await?;
```

## Exemple

[`examples/forgedb-demo/`](../examples/forgedb-demo/) — build régénère le client depuis `schema.forge`.

```bash
cd examples/forgedb-demo
export DATABASE_URL=postgres://localhost/shop
forge db init
forge migrate dev
cargo run -p forgedb-demo
```

## Intégration RustForge

Les repositories manuels (`*_repository.rs`) peuvent être remplacés progressivement par le client généré. Le module `AuthModule` / JWT reste indépendant.

## Roadmap (hors MVP)

- `forge db pull` (introspection)
- Guards sur `#[get(..., guards = ...)]`
- MongoDB, GraphQL, dashboard
