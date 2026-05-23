/// Concatène deux segments de chemin HTTP en `&'static str` (compile-time friendly).
///
/// Les macros passent des littéraux ; le résultat est alloué une fois au démarrage
/// si nécessaire, mais **pas** par requête.
#[inline]
pub fn join_paths(prefix: &str, suffix: &str) -> &'static str {
    let prefix = prefix.trim_end_matches('/');
    let suffix = if suffix.starts_with('/') {
        suffix
    } else {
        // suffix non-literal possible : allocation unique au bootstrap
        let owned = format!("/{suffix}");
        return Box::leak(owned.into_boxed_str());
    };

    if suffix == "/" || suffix.is_empty() {
        let path = if prefix.is_empty() { "/" } else { prefix };
        return Box::leak(format!("{path}/").into_boxed_str());
    }

    let combined = if prefix.is_empty() {
        suffix.to_string()
    } else {
        format!("{prefix}{suffix}")
    };
    Box::leak(combined.into_boxed_str())
}

/// Variante sans allocation quand prefix et suffix sont des littéraux connus à la macro.
#[macro_export]
macro_rules! static_path {
    ($prefix:literal, $suffix:literal) => {{
        const PATH: &str = concat!($prefix, $suffix);
        PATH
    }};
}
