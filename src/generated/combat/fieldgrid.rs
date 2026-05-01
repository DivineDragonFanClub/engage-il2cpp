
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/fieldgrid/FieldGrid.md")))]
#[::unity2::class(namespace = "Combat", name = "FieldGrid")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct FieldGrid {}

#[cfg(feature = "combat-fieldgrid")]
#[::unity2::methods]
impl FieldGrid {
    #[method(name = "FindGrid", args = 0)]
    pub fn find_grid() -> crate::combat::fieldgrid::FieldGrid;

    #[method(name = "get_HasGrid", args = 0)]
    pub fn get_has_grid() -> bool;

    #[method(name = "ToggleGrid", args = 0)]
    pub fn toggle_grid() -> ();

    #[method(name = "MakeGrid", args = 0)]
    pub fn make_grid() -> ();

    #[method(name = "GetPlayFieldRect", args = 0)]
    pub fn get_play_field_rect() -> crate::combat::fieldgrid::FieldGrid_PlayFieldRect;

    #[method(name = "MakeGridMesh", args = 1)]
    pub fn make_grid_mesh(
        field: crate::combat::fieldgrid::FieldGrid_PlayFieldRect,
    ) -> crate::unity_engine::mesh::Mesh;

    #[method(name = "AddCoordinate", args = 2)]
    pub fn add_coordinate(
        parent: crate::unity_engine::gameobject::GameObject,
        field: crate::combat::fieldgrid::FieldGrid_PlayFieldRect,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-fieldgrid")]
impl FieldGrid {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(FieldGrid),
                ::core::stringify!(new),
            )
        });
        <Self as IFieldGridMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/fieldgrid/FieldGrid_PlayFieldRect.md")))]
#[repr(C)]
#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub struct FieldGrid_PlayFieldRect {
    pub play_field: crate::unity_engine::rect::Rect,
    pub cell_size: f32,
}

impl ::unity2::ClassIdentity for FieldGrid_PlayFieldRect {
    const NAMESPACE: &'static str = "Combat";

    const NAME: &'static str = "FieldGrid.PlayFieldRect";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for FieldGrid_PlayFieldRect {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

#[cfg(feature = "combat-fieldgrid")]
#[::unity2::methods(value)]
impl FieldGrid_PlayFieldRect {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, rect: crate::unity_engine::rect::Rect, size: f32) -> ();
}
