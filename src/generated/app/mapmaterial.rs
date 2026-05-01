
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapmaterial/MapMaterial.md")))]
#[::unity2::class(namespace = "App", name = "MapMaterial")]
#[parent(crate::system::object::Object)]
pub struct MapMaterial {
    #[rename(name = "m_List")]
    pub m_list: crate::system::collections::generic::list_1::List_1<
        crate::app::mapmaterial::MapMaterial_Node,
    >,
}

#[cfg(feature = "app-mapmaterial")]
#[::unity2::methods]
impl MapMaterial {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Find", args = 3)]
    pub fn find(
        self,
        kind: crate::app::mapmaterial::MapMaterial_Kinds,
        material: ::unity2::Il2CppString,
        property: ::unity2::Il2CppString,
    ) -> crate::app::mapmaterial::MapMaterial_Node;

    #[method(name = "TryAdd", args = 3)]
    pub fn try_add(
        self,
        kind: crate::app::mapmaterial::MapMaterial_Kinds,
        material: ::unity2::Il2CppString,
        property: ::unity2::Il2CppString,
    ) -> crate::app::mapmaterial::MapMaterial_Node;

    #[method(name = "get_Count", args = 0)]
    pub fn get_count(self) -> i32;

    #[method(name = "get_Item", args = 1)]
    pub fn get_item(self, i: i32) -> crate::app::mapmaterial::MapMaterial_Node;

    #[method(name = "GetEnumerator", args = 0)]
    pub fn get_enumerator(
        self,
    ) -> crate::system::collections::generic::ienumerator_1::IEnumerator_1<
        crate::app::mapmaterial::MapMaterial_Node,
    >;
}

#[cfg(feature = "app-mapmaterial")]
impl MapMaterial {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapMaterial),
                ::core::stringify!(new),
            )
        });
        <Self as IMapMaterialMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapmaterial/MapMaterial_Node.md")))]
#[::unity2::class(namespace = "App", name = "MapMaterial.Node")]
#[parent(crate::system::object::Object)]
pub struct MapMaterial_Node {
    #[rename(name = "kind")]
    pub kind: crate::app::mapmaterial::MapMaterial_Kinds,
    #[rename(name = "material")]
    pub material: ::unity2::Il2CppString,
    #[rename(name = "property")]
    pub property: ::unity2::Il2CppString,
    #[rename(name = "value")]
    pub value: f32,
    #[rename(name = "color")]
    pub color: crate::unity_engine::color::Color,
}

#[cfg(feature = "app-mapmaterial")]
#[::unity2::methods]
impl MapMaterial_Node {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapmaterial")]
impl MapMaterial_Node {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapMaterial_Node),
                ::core::stringify!(new),
            )
        });
        <Self as IMapMaterial_NodeMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapmaterial/MapMaterial_Kinds.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapMaterial_Kinds {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapMaterial_Kinds {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapMaterial.Kinds";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapMaterial_Kinds {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapMaterial_Kinds {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn float() -> Self {
        Self { value: 1 }
    }

    pub fn color() -> Self {
        Self { value: 2 }
    }
}
