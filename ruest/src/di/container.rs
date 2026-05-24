use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::{DiError, ProviderDescriptor, Scope};

type InstanceMap = HashMap<TypeId, Arc<dyn Any + Send + Sync>>;

/// Thread-safe DI container with **monomorphized** resolution via `get::<T>()`.
///
/// Prefer [`register_singleton`](Self::register_singleton) for services — no `dyn` factory on the hot path.
#[derive(Clone, Default)]
pub struct Container {
    providers: Arc<RwLock<HashMap<TypeId, ProviderDescriptor>>>,
    singletons: Arc<RwLock<InstanceMap>>,
    resolving: Arc<RwLock<Vec<TypeId>>>,
}

impl Container {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a provider descriptor (advanced / async factories).
    pub fn register<T: Send + Sync + 'static>(&self, descriptor: ProviderDescriptor) {
        let type_id = TypeId::of::<T>();
        self.providers
            .write()
            .expect("container providers lock poisoned")
            .insert(type_id, descriptor);
    }

    /// Register a singleton directly (compile-time friendly, no factory trait object).
    pub fn register_singleton<T: Send + Sync + 'static>(&self, instance: Arc<T>) {
        let type_id = TypeId::of::<T>();
        self.singletons
            .write()
            .expect("container singletons lock poisoned")
            .insert(type_id, instance);
    }

    /// Register `T: Default` as a singleton (used by `#[service]` macro).
    pub fn register_default<T: Default + Send + Sync + 'static>(&self) {
        self.register_singleton(Arc::new(T::default()));
    }

    /// Register a pre-built singleton instance (alias).
    pub fn register_instance<T: Send + Sync + 'static>(&self, instance: Arc<T>) {
        self.register_singleton(instance);
    }

    /// Resolve type `T` — monomorphized at each call site (no runtime type name).
    pub fn get<T: Send + Sync + 'static>(&self) -> Result<Arc<T>, DiError> {
        let type_id = TypeId::of::<T>();

        if let Some(existing) = self.singletons.read().expect("lock").get(&type_id) {
            return downcast_arc::<T>(existing.clone());
        }

        let descriptor = self
            .providers
            .read()
            .expect("lock")
            .get(&type_id)
            .cloned()
            .ok_or(DiError::not_found::<T>())?;

        self.guard_circular(type_id)?;

        if descriptor.scope == Scope::Singleton {
            if let Some(existing) = self.singletons.read().expect("lock").get(&type_id) {
                self.resolving.write().expect("lock").pop();
                return downcast_arc::<T>(existing.clone());
            }
        }

        let instance = self.resolve_descriptor(&descriptor)?;
        self.resolving.write().expect("lock").pop();

        let arc = downcast_arc::<T>(instance)?;

        if descriptor.scope == Scope::Singleton {
            self.singletons
                .write()
                .expect("lock")
                .insert(type_id, arc.clone());
        }

        Ok(arc)
    }

    /// Async resolution for providers with async factories.
    pub async fn get_async<T: Send + Sync + 'static>(&self) -> Result<Arc<T>, DiError> {
        let type_id = TypeId::of::<T>();

        if let Some(existing) = self.singletons.read().expect("lock").get(&type_id) {
            return downcast_arc::<T>(existing.clone());
        }

        let descriptor = self
            .providers
            .read()
            .expect("lock")
            .get(&type_id)
            .cloned()
            .ok_or(DiError::not_found::<T>())?;

        self.guard_circular(type_id)?;

        let instance = if let Some(fut) = descriptor.factory.create_async(self) {
            fut.await
        } else if let Some(sync) = descriptor.factory.create_sync(self) {
            sync
        } else {
            self.resolving.write().expect("lock").pop();
            return Err(DiError::ResolutionFailed {
                type_name: descriptor.type_name,
                reason: "async factory required".into(),
            });
        };

        self.resolving.write().expect("lock").pop();

        let arc = downcast_arc::<T>(instance)?;

        if descriptor.scope == Scope::Singleton {
            self.singletons
                .write()
                .expect("lock")
                .insert(type_id, arc.clone());
        }

        Ok(arc)
    }

    pub fn request_scope(&self) -> RequestScope<'_> {
        RequestScope {
            parent: self,
            request_instances: RwLock::new(HashMap::new()),
        }
    }

    fn guard_circular(&self, type_id: TypeId) -> Result<(), DiError> {
        let mut resolving = self.resolving.write().expect("lock");
        if resolving.contains(&type_id) {
            return Err(DiError::CircularDependency(format!("{type_id:?}")));
        }
        resolving.push(type_id);
        Ok(())
    }

    fn resolve_descriptor(
        &self,
        descriptor: &ProviderDescriptor,
    ) -> Result<Arc<dyn Any + Send + Sync>, DiError> {
        if let Some(instance) = descriptor.factory.create_sync(self) {
            return Ok(instance);
        }
        Err(DiError::ResolutionFailed {
            type_name: descriptor.type_name,
            reason: "sync factory required; use get_async for async providers".into(),
        })
    }
}

/// Request-scoped resolution context.
pub struct RequestScope<'a> {
    parent: &'a Container,
    request_instances: RwLock<InstanceMap>,
}

impl<'a> RequestScope<'a> {
    pub fn get<T: Send + Sync + 'static>(&self) -> Result<Arc<T>, DiError> {
        let type_id = TypeId::of::<T>();

        if let Some(existing) = self.request_instances.read().expect("lock").get(&type_id) {
            return downcast_arc::<T>(existing.clone());
        }

        let instance = self.parent.get::<T>()?;
        self.request_instances
            .write()
            .expect("lock")
            .insert(type_id, instance.clone());
        downcast_arc::<T>(instance)
    }
}

fn downcast_arc<T: Send + Sync + 'static>(
    value: Arc<dyn Any + Send + Sync>,
) -> Result<Arc<T>, DiError> {
    Arc::downcast::<T>(value)
        .map_err(|_| DiError::ResolutionFailed {
            type_name: std::any::type_name::<T>(),
            reason: "type mismatch in DI container".into(),
        })
}
