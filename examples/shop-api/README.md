# shop-api

Deuxième exemple RustForge — structure complète du [README](../../README.md) :

- `config/` — port via `PORT` (défaut **3001**)
- `common/` — helpers partagés
- `modules/customers/` — dto, entities, repository, service, controller
- `modules/orders/` — idem + injection croisée `Inject<CustomerService>`

## Lancer

```bash
# depuis la racine du workspace
cargo run -p shop-api
```

## Endpoints

| Méthode | URL | Description |
|---------|-----|-------------|
| GET | http://localhost:3001/health | Health check |
| GET | http://localhost:3001/customers/ | Liste clients |
| POST | http://localhost:3001/customers/ | Créer client (démo) |
| GET | http://localhost:3001/orders/ | Liste commandes |
| POST | http://localhost:3001/orders/ | Créer commande (vérifie le client) |

## Flux de test

1. `POST /customers/` — crée un client (email fixe dans le handler démo)
2. Copier l'`id` du client depuis la réponse JSON
3. Adapter `orders_controller.rs` avec cet `customer_id` ou étendre l’API (Phase 2 : extracteurs body)

## Différences avec `basic-api`

| | basic-api | shop-api |
|---|-----------|----------|
| Port | 3000 | 3001 |
| Structure | modules plats | dto/ + entities/ + **repository** |
| Erreurs | `AppResult` | `AppResult` + **`forge_err!`** |
| DI | 2 modules | orders → **Inject&lt;CustomerService&gt;** |
| Config | inline | **`config::port()`** |
