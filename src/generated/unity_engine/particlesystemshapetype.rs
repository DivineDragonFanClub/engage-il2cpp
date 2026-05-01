
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/particlesystemshapetype/ParticleSystemShapeType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ParticleSystemShapeType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ParticleSystemShapeType {
    const NAMESPACE: &'static str = "UnityEngine";

    const NAME: &'static str = "ParticleSystemShapeType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ParticleSystemShapeType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ParticleSystemShapeType {
    pub fn sphere() -> Self {
        Self { value: 0 }
    }

    pub fn sphere_shell() -> Self {
        Self { value: 1 }
    }

    pub fn hemisphere() -> Self {
        Self { value: 2 }
    }

    pub fn hemisphere_shell() -> Self {
        Self { value: 3 }
    }

    pub fn cone() -> Self {
        Self { value: 4 }
    }

    pub fn r#box() -> Self {
        Self { value: 5 }
    }

    pub fn mesh() -> Self {
        Self { value: 6 }
    }

    pub fn cone_shell() -> Self {
        Self { value: 7 }
    }

    pub fn cone_volume() -> Self {
        Self { value: 8 }
    }

    pub fn cone_volume_shell() -> Self {
        Self { value: 9 }
    }

    pub fn circle() -> Self {
        Self { value: 10 }
    }

    pub fn circle_edge() -> Self {
        Self { value: 11 }
    }

    pub fn single_sided_edge() -> Self {
        Self { value: 12 }
    }

    pub fn mesh_renderer() -> Self {
        Self { value: 13 }
    }

    pub fn skinned_mesh_renderer() -> Self {
        Self { value: 14 }
    }

    pub fn box_shell() -> Self {
        Self { value: 15 }
    }

    pub fn box_edge() -> Self {
        Self { value: 16 }
    }

    pub fn donut() -> Self {
        Self { value: 17 }
    }

    pub fn rectangle() -> Self {
        Self { value: 18 }
    }

    pub fn sprite() -> Self {
        Self { value: 19 }
    }

    pub fn sprite_renderer() -> Self {
        Self { value: 20 }
    }
}
