use std::marker::PhantomData;
use std::sync::Arc;

use super::Container;
use super::DiError;

/// Wrapper type for automatic dependency injection in controllers and services.
///
/// ```ignore
/// pub struct UserController {
///     service: Inject<UserService>,
/// }
/// ```
#[derive(Debug)]
pub struct Inject<T> {
    inner: Arc<T>,
}

impl<T: Send + Sync + 'static> Inject<T> {
    /// Resolve `T` from the given container.
    pub fn resolve(container: &Container) -> Result<Self, DiError> {
        let inner = container.get::<T>()?;
        Ok(Self { inner })
    }

    /// Access the injected instance.
    pub fn get(&self) -> &T {
        &self.inner
    }

    /// Clone the underlying `Arc`.
    pub fn arc(&self) -> Arc<T> {
        Arc::clone(&self.inner)
    }
}

impl<T> std::ops::Deref for Inject<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

/// Marker for types that can be constructed via DI without manual registration.
pub trait Injectable: Sized + Send + Sync + 'static {
    fn construct(container: &Container) -> Result<Self, DiError>;
}

/// Helper for optional manual injection outside of macro-generated code.
pub struct Injector<T> {
    _marker: PhantomData<T>,
}

impl<T: Send + Sync + 'static> Injector<T> {
    pub fn from_container(container: &Container) -> Result<Inject<T>, DiError> {
        Inject::resolve(container)
    }
}
