
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/switch/performance/Performance_PerformanceMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Performance_PerformanceMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Performance_PerformanceMode {
    const NAMESPACE: &'static str = "UnityEngine.Switch";

    const NAME: &'static str = "Performance.PerformanceMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Performance_PerformanceMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Performance_PerformanceMode {
    pub fn invalid() -> Self {
        Self { value: -1 }
    }

    pub fn normal() -> Self {
        Self { value: 0 }
    }

    pub fn boost() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/switch/performance/Performance.md")))]
#[::unity2::class(namespace = "UnityEngine.Switch", name = "Performance")]
#[parent(crate::system::object::Object)]
pub struct Performance {}

#[cfg(feature = "unity_engine-switch-performance")]
#[::unity2::methods]
impl Performance {
    #[method(name = "SetCpuBoostMode", args = 1)]
    pub fn set_cpu_boost_mode(
        mode: crate::unity_engine::switch::performance::Performance_CpuBoostMode,
    ) -> ();

    #[method(name = "SetCpuBoostMode_Internal", args = 1)]
    pub fn set_cpu_boost_mode_internal(
        mode: crate::unity_engine::switch::performance::Performance_CpuBoostMode,
    ) -> ();

    #[method(name = "get_mode", args = 0)]
    pub fn get_mode() -> crate::unity_engine::switch::performance::Performance_PerformanceMode;

    #[method(name = "GetMode_Internal", args = 0)]
    pub fn get_mode_internal(
    ) -> crate::unity_engine::switch::performance::Performance_PerformanceMode;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/switch/performance/Performance_CpuBoostMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Performance_CpuBoostMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Performance_CpuBoostMode {
    const NAMESPACE: &'static str = "UnityEngine.Switch";

    const NAME: &'static str = "Performance.CpuBoostMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Performance_CpuBoostMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Performance_CpuBoostMode {
    pub fn normal() -> Self {
        Self { value: 0 }
    }

    pub fn fast_load() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/switch/performance/Performance_Debug.md")))]
#[::unity2::class(namespace = "UnityEngine.Switch", name = "Performance.Debug")]
#[parent(crate::system::object::Object)]
pub struct Performance_Debug {}

#[cfg(feature = "unity_engine-switch-performance")]
#[::unity2::methods]
impl Performance_Debug {
    #[method(name = "SetHUDMode", args = 1)]
    pub fn set_hud_mode(
        mode: crate::unity_engine::switch::performance::Performance_Debug_HudMode,
    ) -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/switch/performance/Performance_Debug_HudMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Performance_Debug_HudMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Performance_Debug_HudMode {
    const NAMESPACE: &'static str = "UnityEngine.Switch";

    const NAME: &'static str = "Performance.Debug.HudMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Performance_Debug_HudMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Performance_Debug_HudMode {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn basic() -> Self {
        Self { value: 1 }
    }

    pub fn memory() -> Self {
        Self { value: 2 }
    }

    pub fn gpu_counters() -> Self {
        Self { value: 3 }
    }
}
