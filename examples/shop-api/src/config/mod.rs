//! Configuration (convention over configuration — variables d'environnement).

/// Port HTTP (défaut 3001 pour ne pas entrer en conflit avec basic-api).
pub fn port() -> u16 {
    std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3001)
}
