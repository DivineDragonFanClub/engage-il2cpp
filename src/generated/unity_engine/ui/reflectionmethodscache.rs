
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/reflectionmethodscache/ReflectionMethodsCache_Raycast3DCallback.md")))]
#[::unity2::class(
    namespace = "UnityEngine.UI",
    name = "ReflectionMethodsCache.Raycast3DCallback"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ReflectionMethodsCache_Raycast3DCallback {}

#[cfg(feature = "unity_engine-ui-reflectionmethodscache")]
#[::unity2::methods]
impl ReflectionMethodsCache_Raycast3DCallback {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 4)]
    pub fn invoke(
        self,
        r: crate::unity_engine::ray::Ray,
        hit: crate::unity_engine::raycasthit::RaycastHit,
        f: f32,
        i: i32,
    ) -> bool;
}

#[cfg(feature = "unity_engine-ui-reflectionmethodscache")]
impl ReflectionMethodsCache_Raycast3DCallback {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ReflectionMethodsCache_Raycast3DCallback),
                ::core::stringify!(new),
            )
        });
        <Self as IReflectionMethodsCache_Raycast3DCallbackMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/reflectionmethodscache/ReflectionMethodsCache_RaycastAllCallback.md")))]
#[::unity2::class(
    namespace = "UnityEngine.UI",
    name = "ReflectionMethodsCache.RaycastAllCallback"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ReflectionMethodsCache_RaycastAllCallback {}

#[cfg(feature = "unity_engine-ui-reflectionmethodscache")]
#[::unity2::methods]
impl ReflectionMethodsCache_RaycastAllCallback {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 3)]
    pub fn invoke(
        self,
        r: crate::unity_engine::ray::Ray,
        f: f32,
        i: i32,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>;
}

#[cfg(feature = "unity_engine-ui-reflectionmethodscache")]
impl ReflectionMethodsCache_RaycastAllCallback {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ReflectionMethodsCache_RaycastAllCallback),
                ::core::stringify!(new),
            )
        });
        <Self as IReflectionMethodsCache_RaycastAllCallbackMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/reflectionmethodscache/ReflectionMethodsCache.md")))]
#[::unity2::class(namespace = "UnityEngine.UI", name = "ReflectionMethodsCache")]
#[parent(crate::system::object::Object)]
pub struct ReflectionMethodsCache {
# [rename (name = "raycast3D")] pub raycast3_d : crate :: unity_engine :: ui :: reflectionmethodscache :: ReflectionMethodsCache_Raycast3DCallback ,
# [rename (name = "raycast3DAll")] pub raycast3_d_all : crate :: unity_engine :: ui :: reflectionmethodscache :: ReflectionMethodsCache_RaycastAllCallback ,
# [rename (name = "getRaycastNonAlloc")] pub get_raycast_non_alloc : crate :: unity_engine :: ui :: reflectionmethodscache :: ReflectionMethodsCache_GetRaycastNonAllocCallback ,
# [rename (name = "raycast2D")] pub raycast2_d : crate :: unity_engine :: ui :: reflectionmethodscache :: ReflectionMethodsCache_Raycast2DCallback ,
# [rename (name = "getRayIntersectionAll")] pub get_ray_intersection_all : crate :: unity_engine :: ui :: reflectionmethodscache :: ReflectionMethodsCache_GetRayIntersectionAllCallback ,
# [rename (name = "getRayIntersectionAllNonAlloc")] pub get_ray_intersection_all_non_alloc : crate :: unity_engine :: ui :: reflectionmethodscache :: ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback ,
# [static_field] # [rename (name = "s_ReflectionMethodsCache")] pub s_reflection_methods_cache : crate :: unity_engine :: ui :: reflectionmethodscache :: ReflectionMethodsCache ,
}

#[cfg(feature = "unity_engine-ui-reflectionmethodscache")]
#[::unity2::methods]
impl ReflectionMethodsCache {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "get_Singleton", args = 0)]
    pub fn get_singleton() -> crate::unity_engine::ui::reflectionmethodscache::ReflectionMethodsCache;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-ui-reflectionmethodscache")]
impl ReflectionMethodsCache {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ReflectionMethodsCache),
                ::core::stringify!(new),
            )
        });
        <Self as IReflectionMethodsCacheMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/reflectionmethodscache/ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback.md")))]
#[::unity2::class(
    namespace = "UnityEngine.UI",
    name = "ReflectionMethodsCache.GetRayIntersectionAllNonAllocCallback"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback {}

#[cfg(feature = "unity_engine-ui-reflectionmethodscache")]
#[::unity2::methods]
impl ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 4)]
    pub fn invoke(
        self,
        r: crate::unity_engine::ray::Ray,
        results: ::unity2::Array<crate::unity_engine::raycasthit2d::RaycastHit2D>,
        f: f32,
        i: i32,
    ) -> i32;
}

#[cfg(feature = "unity_engine-ui-reflectionmethodscache")]
impl ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ReflectionMethodsCache_GetRayIntersectionAllNonAllocCallback),
                ::core::stringify!(new),
            )
        });
        <Self as IReflectionMethodsCache_GetRayIntersectionAllNonAllocCallbackMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/reflectionmethodscache/ReflectionMethodsCache_GetRayIntersectionAllCallback.md")))]
#[::unity2::class(
    namespace = "UnityEngine.UI",
    name = "ReflectionMethodsCache.GetRayIntersectionAllCallback"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ReflectionMethodsCache_GetRayIntersectionAllCallback {}

#[cfg(feature = "unity_engine-ui-reflectionmethodscache")]
#[::unity2::methods]
impl ReflectionMethodsCache_GetRayIntersectionAllCallback {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 3)]
    pub fn invoke(
        self,
        r: crate::unity_engine::ray::Ray,
        f: f32,
        i: i32,
    ) -> ::unity2::Array<crate::unity_engine::raycasthit2d::RaycastHit2D>;
}

#[cfg(feature = "unity_engine-ui-reflectionmethodscache")]
impl ReflectionMethodsCache_GetRayIntersectionAllCallback {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ReflectionMethodsCache_GetRayIntersectionAllCallback),
                ::core::stringify!(new),
            )
        });
        <Self as IReflectionMethodsCache_GetRayIntersectionAllCallbackMethods>::ctor(
            this, object, method,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/reflectionmethodscache/ReflectionMethodsCache_Raycast2DCallback.md")))]
#[::unity2::class(
    namespace = "UnityEngine.UI",
    name = "ReflectionMethodsCache.Raycast2DCallback"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ReflectionMethodsCache_Raycast2DCallback {}

#[cfg(feature = "unity_engine-ui-reflectionmethodscache")]
#[::unity2::methods]
impl ReflectionMethodsCache_Raycast2DCallback {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 4)]
    pub fn invoke(
        self,
        p1: crate::unity_engine::vector2::Vector2,
        p2: crate::unity_engine::vector2::Vector2,
        f: f32,
        i: i32,
    ) -> crate::unity_engine::raycasthit2d::RaycastHit2D;
}

#[cfg(feature = "unity_engine-ui-reflectionmethodscache")]
impl ReflectionMethodsCache_Raycast2DCallback {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ReflectionMethodsCache_Raycast2DCallback),
                ::core::stringify!(new),
            )
        });
        <Self as IReflectionMethodsCache_Raycast2DCallbackMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/ui/reflectionmethodscache/ReflectionMethodsCache_GetRaycastNonAllocCallback.md")))]
#[::unity2::class(
    namespace = "UnityEngine.UI",
    name = "ReflectionMethodsCache.GetRaycastNonAllocCallback"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ReflectionMethodsCache_GetRaycastNonAllocCallback {}

#[cfg(feature = "unity_engine-ui-reflectionmethodscache")]
#[::unity2::methods]
impl ReflectionMethodsCache_GetRaycastNonAllocCallback {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 4)]
    pub fn invoke(
        self,
        r: crate::unity_engine::ray::Ray,
        results: ::unity2::Array<crate::unity_engine::raycasthit::RaycastHit>,
        f: f32,
        i: i32,
    ) -> i32;
}

#[cfg(feature = "unity_engine-ui-reflectionmethodscache")]
impl ReflectionMethodsCache_GetRaycastNonAllocCallback {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ReflectionMethodsCache_GetRaycastNonAllocCallback),
                ::core::stringify!(new),
            )
        });
        <Self as IReflectionMethodsCache_GetRaycastNonAllocCallbackMethods>::ctor(
            this, object, method,
        );
        this
    }
}
