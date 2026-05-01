
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mappaneldangerall/MapPanelDangerAll_DangerType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapPanelDangerAll_DangerType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapPanelDangerAll_DangerType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapPanelDangerAll.DangerType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapPanelDangerAll_DangerType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapPanelDangerAll_DangerType {
    pub fn attack() -> Self {
        Self { value: 0 }
    }

    pub fn rod() -> Self {
        Self { value: 1 }
    }

    pub fn gunner() -> Self {
        Self { value: 2 }
    }

    pub fn num() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mappaneldangerall/MapPanelDangerAll_MeshIndex.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapPanelDangerAll_MeshIndex {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapPanelDangerAll_MeshIndex {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapPanelDangerAll.MeshIndex";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapPanelDangerAll_MeshIndex {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapPanelDangerAll_MeshIndex {
    pub fn attack_panel() -> Self {
        Self { value: 0 }
    }

    pub fn rod_panel() -> Self {
        Self { value: 1 }
    }

    pub fn gunner_panel() -> Self {
        Self { value: 2 }
    }

    pub fn frame_panel_for_gunner() -> Self {
        Self { value: 3 }
    }

    pub fn num() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mappaneldangerall/MapPanelDangerAll.md")))]
#[::unity2::class(namespace = "App", name = "MapPanelDangerAll")]
pub struct MapPanelDangerAll {
    #[rename(name = "m_AttackMaterial")]
    pub m_attack_material: crate::unity_engine::material::Material,
    #[rename(name = "m_RodMaterial")]
    pub m_rod_material: crate::unity_engine::material::Material,
    #[rename(name = "m_GunnerMaterial")]
    pub m_gunner_material: crate::unity_engine::material::Material,
    #[rename(name = "m_FrameForGunnerMaterial")]
    pub m_frame_for_gunner_material: crate::unity_engine::material::Material,
    #[rename(name = "m_Mode")]
    pub m_mode: crate::app::gameconfig::GameConfig_AllInfo,
    #[rename(name = "m_OldMode")]
    pub m_old_mode: crate::app::gameconfig::GameConfig_AllInfo,
    #[rename(name = "m_IsUpdate")]
    pub m_is_update: bool,
    #[rename(name = "m_IsVisible")]
    pub m_is_visible: bool,
    #[rename(name = "m_Alpha")]
    pub m_alpha: crate::app::interpolatorfloat::InterpolatorFloat,
    #[rename(name = "m_MiniMapFillMaterials")]
    pub m_mini_map_fill_materials: ::unity2::Array<crate::unity_engine::material::Material>,
    #[rename(name = "m_MiniMapFrameMaterials")]
    pub m_mini_map_frame_materials: ::unity2::Array<crate::unity_engine::material::Material>,
    #[rename(name = "m_AttackColor")]
    pub m_attack_color: crate::unity_engine::color::Color,
    #[rename(name = "m_RodColor")]
    pub m_rod_color: crate::unity_engine::color::Color,
    #[rename(name = "m_GunnerColor")]
    pub m_gunner_color: crate::unity_engine::color::Color,
}

#[cfg(feature = "app-mappaneldangerall")]
#[::unity2::methods]
impl MapPanelDangerAll {
    #[method(name = "get_SubMeshCount", args = 0)]
    pub fn get_sub_mesh_count(self) -> i32;

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = "OnValidate", args = 0)]
    pub fn on_validate(self) -> ();

    #[method(name = "GetSourceMaterials", args = 0)]
    pub fn get_source_materials(self) -> ::unity2::Array<crate::unity_engine::material::Material>;

    #[method(name = "GetSourceMaterialsForMiniMapFill", args = 0)]
    pub fn get_source_materials_for_mini_map_fill(
        self,
    ) -> ::unity2::Array<crate::unity_engine::material::Material>;

    #[method(name = "GetSourceMaterialsForMiniMapFrame", args = 0)]
    pub fn get_source_materials_for_mini_map_frame(
        self,
    ) -> ::unity2::Array<crate::unity_engine::material::Material>;

    #[method(name = "GetMode", args = 0)]
    pub fn get_mode(self) -> crate::app::gameconfig::GameConfig_AllInfo;

    #[method(name = "SetMode", args = 2)]
    pub fn set_mode(
        self,
        mode: crate::app::gameconfig::GameConfig_AllInfo,
        is_force_update: bool,
    ) -> ();

    #[method(name = "UpdateMode", args = 1)]
    pub fn update_mode(self, mode: crate::app::gameconfig::GameConfig_AllInfo) -> ();

    #[method(name = "UpdateVisible", args = 0)]
    pub fn update_visible(self) -> ();

    #[method(name = "UpdatePanelAlpha", args = 0)]
    pub fn update_panel_alpha(self) -> ();

    #[method(name = "SetPanelAlpha", args = 1)]
    pub fn set_panel_alpha(self, index: i32) -> ();

    #[method(name = "UpdateVertex", args = 1)]
    pub fn update_vertex(self, mode: crate::app::gameconfig::GameConfig_AllInfo) -> ();

    #[method(name = "SetVertex", args = 0)]
    pub fn set_vertex(self) -> ();

    #[method(name = "get_IsVisible", args = 0)]
    pub fn get_is_visible(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mappaneldangerall")]
impl MapPanelDangerAll {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapPanelDangerAll),
                ::core::stringify!(new),
            )
        });
        <Self as IMapPanelDangerAllMethods>::ctor(this);
        this
    }
}
