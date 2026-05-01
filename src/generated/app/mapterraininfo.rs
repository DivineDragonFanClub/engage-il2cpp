
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapterraininfo/MapTerrainInfo.md")))]
#[::unity2::class(namespace = "App", name = "MapTerrainInfo")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: mapterraininfo :: MapTerrainInfo >)]
pub struct MapTerrainInfo {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_PrefabHandle")]
    pub m_prefab_handle: crate::app::tresourcehandle_1::TResourceHandle_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_MapTerrainInfoSingles")]
    pub m_map_terrain_info_singles:
        ::unity2::Array<crate::app::mapterraininfo::MapTerrainInfo_MapTerrainInfoSingle>,
    #[static_field]
    #[rename(name = "m_CalcUnit")]
    pub m_calc_unit: crate::app::unit::Unit,
    #[static_field]
    #[rename(name = "m_BattleInfo")]
    pub m_battle_info: crate::app::battleinfo::BattleInfo,
}

#[cfg(feature = "app-mapterraininfo")]
#[::unity2::methods]
impl MapTerrainInfo {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "ShowAll", args = 0)]
    pub fn show_all(self) -> ();

    #[method(name = "HideAll", args = 0)]
    pub fn hide_all(self) -> ();

    #[method(name = "EventShowAll", args = 0)]
    pub fn event_show_all(self) -> ();

    #[method(name = "EventHideAll", args = 0)]
    pub fn event_hide_all(self) -> ();

    #[method(name = "IsShowAny", args = 0)]
    pub fn is_show_any(self) -> bool;

    #[method(name = "GetCurrentUnit", args = 2)]
    pub fn get_current_unit(x: i32, z: i32) -> crate::app::unit::Unit;

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "CreateObjects", args = 0)]
    pub fn create_objects(self) -> ();

    #[method(name = "DeleteObjects", args = 0)]
    pub fn delete_objects(self) -> ();

    #[method(name = "get_Left", args = 0)]
    pub fn get_left(self) -> crate::app::mapterraininfo::MapTerrainInfo_MapTerrainInfoSingle;

    #[method(name = "get_Right", args = 0)]
    pub fn get_right(self) -> crate::app::mapterraininfo::MapTerrainInfo_MapTerrainInfoSingle;

    #[method(name = "get_Edit", args = 0)]
    pub fn get_edit(self) -> crate::app::mapterraininfo::MapTerrainInfo_MapTerrainInfoSingle;

    #[method(name = "set_Edit", args = 1)]
    pub fn set_edit(
        self,
        value: crate::app::mapterraininfo::MapTerrainInfo_MapTerrainInfoSingle,
    ) -> ();

    #[method(name = "CreateEdit", args = 0)]
    pub fn create_edit(self) -> ();

    #[method(name = "DeleteEdit", args = 0)]
    pub fn delete_edit(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mapterraininfo")]
impl MapTerrainInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapTerrainInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IMapTerrainInfoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapterraininfo/MapTerrainInfo_Side.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapTerrainInfo_Side {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapTerrainInfo_Side {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapTerrainInfo.Side";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapTerrainInfo_Side {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapTerrainInfo_Side {
    pub fn left() -> Self {
        Self { value: 0 }
    }

    pub fn right() -> Self {
        Self { value: 1 }
    }

    pub fn num() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapterraininfo/MapTerrainInfo_MapTerrainInfoSingle.md")))]
#[::unity2::class(namespace = "App", name = "MapTerrainInfo.MapTerrainInfoSingle")]
#[parent(crate::system::object::Object)]
pub struct MapTerrainInfo_MapTerrainInfoSingle {
    #[rename(name = "m_GameObject")]
    pub m_game_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ElemGameObjects")]
    pub m_elem_game_objects: ::unity2::Array<crate::unity_engine::gameobject::GameObject>,
    #[rename(name = "m_ElemTitleTextMeshes")]
    pub m_elem_title_text_meshes: ::unity2::Array<crate::tm_pro::textmeshprougui::TextMeshProUGUI>,
    #[rename(name = "m_ElemValueTextMeshes")]
    pub m_elem_value_text_meshes: ::unity2::Array<crate::tm_pro::textmeshprougui::TextMeshProUGUI>,
    #[rename(name = "m_IsValid")]
    pub m_is_valid: bool,
    #[rename(name = "m_IsShow")]
    pub m_is_show: bool,
    #[rename(name = "m_IsEventShow")]
    pub m_is_event_show: bool,
    #[rename(name = "m_Border")]
    pub m_border: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_EffectRoot")]
    pub m_effect_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_SkillRoot")]
    pub m_skill_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Alignment")]
    pub m_alignment: crate::app::mapterraininfoalignment::MapTerrainInfoAlignment,
    #[rename(name = "m_Side")]
    pub m_side: crate::app::mapterraininfo::MapTerrainInfo_Side,
}

#[cfg(feature = "app-mapterraininfo")]
#[::unity2::methods]
impl MapTerrainInfo_MapTerrainInfoSingle {
    #[method(name = "CreateObjects", args = 2)]
    pub fn create_objects(
        self,
        prefab_obj: crate::unity_engine::gameobject::GameObject,
        side: crate::app::mapterraininfo::MapTerrainInfo_Side,
    ) -> ();

    #[method(name = "CreateElement", args = 3)]
    pub fn create_element(
        self,
        elem: crate::app::mapterraininfo::MapTerrainInfo_Element,
        parent_obj: crate::unity_engine::gameobject::GameObject,
        obj_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "CreatePairElement", args = 3)]
    pub fn create_pair_element(
        self,
        elem: crate::app::mapterraininfo::MapTerrainInfo_Element,
        parent_obj: crate::unity_engine::gameobject::GameObject,
        obj_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "DestroyObjects", args = 0)]
    pub fn destroy_objects(self) -> ();

    #[method(name = "Show", args = 0)]
    pub fn show(self) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "EventShow", args = 0)]
    pub fn event_show(self) -> ();

    #[method(name = "EventHide", args = 0)]
    pub fn event_hide(self) -> ();

    #[method(name = "IsShow", args = 0)]
    pub fn is_show(self) -> bool;

    #[method(name = "SetVisible", args = 1)]
    pub fn set_visible(self, is_visible: bool) -> ();

    #[method(name = "GetTerrain", args = 2)]
    pub fn get_terrain(x: i32, z: i32) -> crate::app::terraindata_2::TerrainData_2;

    #[method(name = "Set", args = 2)]
    pub fn set(self, x: i32, z: i32) -> ();

    #[method(name = "Set", args = 1)]
    pub fn set_2(self, terrain: crate::app::terraindata_2::TerrainData_2) -> ();

    #[method(name = "SetImpl", args = 4)]
    pub fn set_impl(
        self,
        terrain: crate::app::terraindata_2::TerrainData_2,
        overlap: crate::app::terraindata_2::TerrainData_2,
        x: i32,
        z: i32,
    ) -> ();

    #[method(name = "SetElement", args = 5)]
    pub fn set_element(
        self,
        element: crate::app::mapterraininfo::MapTerrainInfo_Element,
        name: ::unity2::Il2CppString,
        value: i32,
        is_force: bool,
        is_color_reverse: bool,
    ) -> ();

    #[method(name = "SetElement", args = 4)]
    pub fn set_element_2(
        self,
        element: crate::app::mapterraininfo::MapTerrainInfo_Element,
        name: ::unity2::Il2CppString,
        value: ::unity2::Il2CppString,
        is_force: bool,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapterraininfo")]
impl MapTerrainInfo_MapTerrainInfoSingle {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapTerrainInfo_MapTerrainInfoSingle),
                ::core::stringify!(new),
            )
        });
        <Self as IMapTerrainInfo_MapTerrainInfoSingleMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapterraininfo/MapTerrainInfo_Element.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapTerrainInfo_Element {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapTerrainInfo_Element {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapTerrainInfo.Element";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapTerrainInfo_Element {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapTerrainInfo_Element {
    pub fn effects() -> Self {
        Self { value: 0 }
    }

    pub fn avoid() -> Self {
        Self { value: 1 }
    }

    pub fn def() -> Self {
        Self { value: 2 }
    }

    pub fn res() -> Self {
        Self { value: 3 }
    }

    pub fn heal() -> Self {
        Self { value: 4 }
    }

    pub fn damage() -> Self {
        Self { value: 5 }
    }

    pub fn r#move() -> Self {
        Self { value: 6 }
    }

    pub fn player_def() -> Self {
        Self { value: 7 }
    }

    pub fn enemy_def() -> Self {
        Self { value: 8 }
    }

    pub fn stun() -> Self {
        Self { value: 9 }
    }

    pub fn other() -> Self {
        Self { value: 10 }
    }

    pub fn sight() -> Self {
        Self { value: 11 }
    }

    pub fn cannon_root() -> Self {
        Self { value: 12 }
    }

    pub fn stock() -> Self {
        Self { value: 13 }
    }

    pub fn attack() -> Self {
        Self { value: 14 }
    }

    pub fn hit() -> Self {
        Self { value: 15 }
    }

    pub fn title() -> Self {
        Self { value: 16 }
    }

    pub fn terrain_name() -> Self {
        Self { value: 17 }
    }

    pub fn overlap() -> Self {
        Self { value: 18 }
    }

    pub fn num() -> Self {
        Self { value: 19 }
    }
}
