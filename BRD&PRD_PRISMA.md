# BRD & PRD — Système Base de Données type Prisma pour Rust

# Nom du projet

RuestDB

---

# 1. BUSINESS REQUIREMENTS DOCUMENT (BRD)

# 1.1 Vision du projet

Créer un système de gestion de base de données moderne inspiré de Prisma pour l’écosystème Rust.

Le système doit permettre aux développeurs de :

* définir leurs modèles dans un fichier schema,
* générer automatiquement un client Rust type-safe,
* créer des migrations,
* manipuler la base de données facilement,
* améliorer fortement l’expérience développeur.

Le système doit être simple, rapide, type-safe et optimisé pour Rust.

---

# 1.2 Objectif principal

Créer un MVP permettant :

1. Définition des modèles avec un DSL.
2. Génération automatique du client Rust.
3. Support PostgreSQL.
4. CRUD automatique.
5. Génération de migrations.
6. Requêtes async type-safe.

---

# 1.3 Problèmes actuels

## Complexité des ORMs Rust

Les ORMs Rust actuels sont souvent :

* complexes,
* verbeux,
* difficiles pour les débutants.

---

## Mauvaise DX

Les développeurs doivent souvent :

* écrire beaucoup de code,
* gérer SQL manuellement,
* gérer trop de configuration.

---

## Manque d’outils intégrés

Il manque souvent :

* migrations simples,
* génération automatique,
* introspection,
* dashboard DB.

---

# 1.4 Public cible

## Développeurs backend Rust

## Développeurs venant de Prisma

## Développeurs Node.js et TypeScript

## Startups et SaaS

---

# 1.5 Valeur ajoutée

| Fonctionnalité         | Valeur                |
| ---------------------- | --------------------- |
| Type safety            | Réduction des erreurs |
| Génération automatique | Productivité élevée   |
| Async Rust             | Haute performance     |
| Schema DSL             | Simplicité            |
| Migrations             | Gestion DB moderne    |
| PostgreSQL             | Base robuste          |

---

# 1.6 Objectif MVP

Le MVP doit être :

* simple,
* stable,
* rapide,
* facile à maintenir.

Le MVP ne doit pas chercher à recréer Prisma complet.

---

# 2. PRODUCT REQUIREMENTS DOCUMENT (PRD)

# 2.1 Vision technique

Le système doit fonctionner comme ceci :

```bash
ruest generate
ruest migrate dev
```

Puis :

```rust
let users = db.user.find_many().await?;
```

---

# 2.2 Scope MVP

# Fonctionnalités incluses

## Schema DSL

Créer un fichier :

```text
schema.ruest
```

---

## PostgreSQL uniquement

Le MVP supporte seulement PostgreSQL.

---

## CRUD basique

Support :

* create,
* find_one,
* find_many,
* update,
* delete.

---

## Relations simples

Support :

* one-to-many,
* many-to-one.

---

## Migrations

Génération automatique des migrations SQL.

---

## Client Rust généré

Génération automatique d’un client type-safe.

---

# Hors scope MVP

Pas de :

* GraphQL,
* MongoDB,
* distributed database,
* sharding,
* read replicas,
* dashboard UI,
* cache,
* realtime sync,
* query optimizer avancé.

---

# 2.3 Architecture MVP

## Structure

```text
ruest-db/
├── ruest-schema/
├── ruest-parser/
├── ruest-client/
├── ruest-migrate/
├── ruest-query/
└── ruest-cli/
```

---

# 2.4 Schema DSL

## Objectif

Permettre une définition simple des modèles.

## Exemple

```prisma
model User {
  id        String   @id @default(uuid())
  email     String   @unique
  name      String
  posts     Post[]
}

model Post {
  id        String @id
  title     String
  userId    String
  user      User @relation(fields: [userId], references: [id])
}
```

---

# 2.5 Types supportés MVP

## Types primitifs

```text
String
Int
Float
Boolean
DateTime
UUID
```

---

# 2.6 Attributs supportés MVP

## Support

```text
@id
@unique
@default
@relation
```

---

# 2.7 Parser Schema

## Objectif

Créer un parser qui :

* lit le schema,
* génère un AST,
* valide les relations,
* valide les types.

---

# 2.8 Génération SQL

## Objectif

Transformer le schema DSL vers SQL PostgreSQL.

## Exemple

```sql
CREATE TABLE users (
  id UUID PRIMARY KEY,
  email TEXT UNIQUE,
  name TEXT
);
```

---

# 2.9 Migrations

## Commandes

```bash
ruest migrate dev
ruest migrate deploy
ruest migrate reset
```

## Fonctionnalités

* génération SQL,
* exécution migrations,
* historique migrations.

---

# 2.10 Client Rust généré

## Objectif

Créer automatiquement un client Rust.

## Exemple

```rust
let users = db.user.find_many().await?;
```

---

# 2.11 Query Builder MVP

## Queries supportées

### Find many

```rust
db.user.find_many().await?
```

### Find unique

```rust
db.user.find_unique(id).await?
```

### Create

```rust
db.user.create(data).await?
```

### Update

```rust
db.user.update(id, data).await?
```

### Delete

```rust
db.user.delete(id).await?
```

---

# 2.12 Runtime DB

## Technologies recommandées

| Domaine       | Technologie    |
| ------------- | -------------- |
| Runtime async | Tokio          |
| Base SQL      | SQLx           |
| Serialization | Serde          |
| Parser DSL    | pest / chumsky |
| CLI           | clap           |

---

# 2.13 Connection Manager

## Fonctionnalités

* connection pooling,
* async connections,
* retry,
* timeout.

---

# 2.14 Configuration

## Exemple

```env
DATABASE_URL=postgres://user:password@localhost:5432/app
```

---

# 2.15 CLI MVP

## Commandes

```bash
ruest db init
ruest generate
ruest migrate dev
ruest db pull
```

---

# 2.16 Introspection MVP

## Fonctionnalité

Lire PostgreSQL et générer :

```text
schema.ruest
```

---

# 2.17 Génération de code

## Génération

Le système doit générer :

```text
generated/
├── client/
├── entities/
├── types/
└── queries/
```

---

# 2.18 Sécurité MVP

## Obligatoire

* prepared statements,
* query parameterization,
* validation schema,
* timeout DB,
* connection pool sécurisé.

---

# 2.19 Performance MVP

## Objectifs

* async-first,
* faible consommation mémoire,
* faible latence,
* connection pooling.

---

# 2.20 Developer Experience

## Priorités

### Simplicité

Le développeur ne doit pas écrire du SQL complexe.

---

### Type Safety

Toutes les requêtes doivent être fortement typées.

---

### Génération automatique

Le système doit générer un maximum de code.

---

# 2.21 Roadmap MVP

# Phase 1

Créer :

* parser schema,
* AST,
* génération SQL.

---

# Phase 2

Créer :

* migrations,
* PostgreSQL connection,
* client Rust.

---

# Phase 3

Créer :

* relations,
* query builder,
* introspection.

---

# 2.22 Livrables attendus

L’IA doit produire :

1. parser schema,
2. AST,
3. générateur SQL,
4. système migrations,
5. client Rust généré,
6. query builder,
7. support PostgreSQL,
8. CLI minimale,
9. exemples CRUD,
10. tests.

---

# 2.23 Contraintes importantes

## Simplicité

Le MVP doit rester simple.

---

## Pas de sur-ingénierie

Ne pas recréer Prisma complet immédiatement.

---

## Priorité DX

La priorité est :

* simplicité,
* stabilité,
* génération automatique,
* type safety.

---

# 2.24 Résultat final attendu

Le résultat final attendu est un système base de données moderne pour Rust permettant :

* définition simple des modèles,
* génération automatique,
* requêtes type-safe,
* migrations automatiques,
* excellente expérience développeur.

Le système doit offrir une expérience proche de Prisma tout en profitant des performances et de la sécurité de Rust.
