
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortiesequenceunitselect/SortieSequenceUnitSelect_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SortieSequenceUnitSelect_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SortieSequenceUnitSelect_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SortieSequenceUnitSelect.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SortieSequenceUnitSelect_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SortieSequenceUnitSelect_Label {
    pub fn selection_unit() -> Self {
        Self { value: 0 }
    }

    pub fn to_troop() -> Self {
        Self { value: 1 }
    }

    pub fn ring_select() -> Self {
        Self { value: 2 }
    }

    pub fn to_inventory() -> Self {
        Self { value: 3 }
    }

    pub fn to_trade() -> Self {
        Self { value: 4 }
    }

    pub fn to_skill() -> Self {
        Self { value: 5 }
    }

    pub fn to_class_change() -> Self {
        Self { value: 6 }
    }

    pub fn end() -> Self {
        Self { value: 7 }
    }

    pub fn end_from_troop() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortiesequenceunitselect/SortieSequenceUnitSelect.md")))]
#[::unity2::class(namespace = "App", name = "SortieSequenceUnitSelect")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: sortiesequenceunitselect :: SortieSequenceUnitSelect >)]
pub struct SortieSequenceUnitSelect {
    #[static_field]
    #[rename(name = "ResNameC")]
    pub res_name_c: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "SortieResNameC")]
    pub sortie_res_name_c: ::unity2::Il2CppString,
    #[rename(name = "m_GameObject")]
    pub m_game_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Window")]
    pub m_window: crate::app::unitselectroot::UnitSelectRoot,
    #[rename(name = "m_unitSelectMenu")]
    pub m_unit_select_menu: crate::app::basicmenu::BasicMenu,
    #[rename(name = "m_rootAnimator")]
    pub m_root_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_menuSelect")]
    pub m_menu_select: crate::app::basicmenuselect::BasicMenuSelect,
}

#[cfg(feature = "app-sortiesequenceunitselect")]
#[::unity2::methods]
impl SortieSequenceUnitSelect {
    #[method(name = "CreateBindSortie", args = 1)]
    pub fn create_bind_sortie(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindInventory", args = 1)]
    pub fn create_bind_inventory(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindRingSelect", args = 1)]
    pub fn create_bind_ring_select(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindCommon", args = 2)]
    pub fn create_bind_common(
        super_: crate::app::procinst::ProcInst,
        mode: crate::app::sortieselectionunitmanager::SortieSelectionUnitManager_Modes,
    ) -> ();

    #[method(name = ".ctor", args = 1)]
    pub fn ctor(
        self,
        mode: crate::app::sortieselectionunitmanager::SortieSelectionUnitManager_Modes,
    ) -> ();

    #[method(name = "Tutorial", args = 0)]
    pub fn tutorial(self) -> ();

    #[method(name = "CheckSkillOpenTutorial", args = 0)]
    pub fn check_skill_open_tutorial(self) -> ();

    #[method(name = "GetLoadResName", args = 0)]
    pub fn get_load_res_name(self) -> ::unity2::Il2CppString;

    #[method(name = "LoadRes", args = 0)]
    pub fn load_res(self) -> ();

    #[method(name = "IsLoadingRes", args = 0)]
    pub fn is_loading_res(self) -> bool;

    #[method(name = "Open", args = 0)]
    pub fn open(self) -> ();

    #[method(name = "AfterOpen", args = 0)]
    pub fn after_open(self) -> ();

    #[method(name = "ResetSelect", args = 0)]
    pub fn reset_select(self) -> ();

    #[method(name = "MenuTick", args = 0)]
    pub fn menu_tick(self) -> ();

    #[method(name = "CloseAll", args = 0)]
    pub fn close_all(self) -> ();

    #[method(name = "IsClosed", args = 0)]
    pub fn is_closed(self) -> bool;

    #[method(name = "PostClosed", args = 0)]
    pub fn post_closed(self) -> ();

    #[method(name = "DispAll", args = 0)]
    pub fn disp_all(self) -> ();

    #[method(name = "HideHeaderKeyHelp", args = 0)]
    pub fn hide_header_key_help(self) -> ();

    #[method(name = "Release", args = 0)]
    pub fn release(self) -> ();

    #[method(name = "SettingTitle", args = 0)]
    pub fn setting_title(self) -> ();

    #[method(name = "CloseTitle", args = 0)]
    pub fn close_title(self) -> ();
}

#[cfg(feature = "app-sortiesequenceunitselect")]
impl SortieSequenceUnitSelect {
    pub fn new(
        mode: crate::app::sortieselectionunitmanager::SortieSelectionUnitManager_Modes,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieSequenceUnitSelect),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieSequenceUnitSelectMethods>::ctor(this, mode);
        this
    }
}
