use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use async_trait::async_trait;

use crate::Container;
use crate::Scope;

/// Describes how a type is provided to the DI container.
#[derive(Clone)]
pub struct ProviderDescriptor {
    pub type_name: &'static str,
    pub scope: Scope,
    pub factory: Arc<dyn ProviderFactory>,
}

/// Async factory that builds a service instance.
pub type AsyncFactoryFn =
    dyn Fn(Container) -> Pin<Box<dyn Future<Output = Arc<dyn Any + Send + Sync>> + Send>>
        + Send
        + Sync;

/// Synchronous factory for simple services.
pub type SyncFactoryFn = dyn Fn() -> Arc<dyn Any + Send + Sync> + Send + Sync;

pub trait ProviderFactory: Send + Sync {
    fn create_sync(&self, _container: &Container) -> Option<Arc<dyn Any + Send + Sync>> {
        None
    }

    fn create_async(
        &self,
        _container: &Container,
    ) -> Option<Pin<Box<dyn Future<Output = Arc<dyn Any + Send + Sync>> + Send>>> {
        None
    }
}

struct SyncFactoryWrapper<F>(F);

impl<F> ProviderFactory for SyncFactoryWrapper<F>
where
    F: Fn() -> Arc<dyn Any + Send + Sync> + Send + Sync,
{
    fn create_sync(&self, _container: &Container) -> Option<Arc<dyn Any + Send + Sync>> {
        Some((self.0)())
    }
}

struct AsyncFactoryWrapper<F>(F);

impl<F> ProviderFactory for AsyncFactoryWrapper<F>
where
    F: Fn(Container) -> Pin<Box<dyn Future<Output = Arc<dyn Any + Send + Sync>> + Send>>
        + Send
        + Sync,
{
    fn create_async(
        &self,
        container: &Container,
    ) -> Option<Pin<Box<dyn Future<Output = Arc<dyn Any + Send + Sync>> + Send>>> {
        Some((self.0)(container.clone()))
    }
}

/// Trait implemented by types that register themselves as providers.
#[async_trait]
pub trait Provider: Send + Sync + 'static {
    type Output: Send + Sync + 'static;

    async fn provide(container: &Container) -> Arc<Self::Output>;
}

/// Factory-based provider registration.
pub struct FactoryProvider<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: Send + Sync + 'static> FactoryProvider<T> {
    pub fn sync<F>(type_name: &'static str, scope: Scope, factory: F) -> ProviderDescriptor
    where
        F: Fn() -> Arc<T> + Send + Sync + 'static,
    {
        ProviderDescriptor {
            type_name,
            scope,
            factory: Arc::new(SyncFactoryWrapper(move || {
                let value: Arc<T> = factory();
                value as Arc<dyn Any + Send + Sync>
            })),
        }
    }

    pub fn from_instance(type_name: &'static str, instance: Arc<T>) -> ProviderDescriptor {
        let instance_any: Arc<dyn Any + Send + Sync> = instance;
        ProviderDescriptor {
            type_name,
            scope: Scope::Singleton,
            factory: Arc::new(SyncFactoryWrapper(move || Arc::clone(&instance_any))),
        }
    }

    pub fn async_factory<F, Fut>(
        type_name: &'static str,
        scope: Scope,
        factory: F,
    ) -> ProviderDescriptor
    where
        F: Fn(Container) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Arc<T>> + Send + 'static,
    {
        ProviderDescriptor {
            type_name,
            scope,
            factory: Arc::new(AsyncFactoryWrapper(
                move |container| -> Pin<Box<dyn Future<Output = Arc<dyn Any + Send + Sync>> + Send>> {
                    let fut = factory(container);
                    Box::pin(async move {
                        let value: Arc<T> = fut.await;
                        value as Arc<dyn Any + Send + Sync>
                    })
                },
            )),
        }
    }
}

/// Register a type that implements `Default` as a singleton provider.
pub fn default_provider<T>(type_name: &'static str, scope: Scope) -> ProviderDescriptor
where
    T: Default + Send + Sync + 'static,
{
    FactoryProvider::<T>::sync(type_name, scope, || Arc::new(T::default()))
}
