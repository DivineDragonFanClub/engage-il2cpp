
use crate::app::basicmenucontent::BasicMenuContent;
use crate::app::basicmenucontent::IBasicMenuContent;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relaymessagemenucontent/RelayMessageMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "RelayMessageMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct RelayMessageMenuContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_KindIcon")]
    pub m_kind_icon:
        ::unity2::Array<crate::app::relaymessagemenucontent::RelayMessageMenuContent_KindIcon>,
    #[rename(name = "m_ActiveKindIcon")]
    pub m_active_kind_icon: crate::system::collections::generic::list_1::List_1<
        crate::app::relaymessagemenucontent::RelayMessageMenuContent_KindIcon,
    >,
}

#[cfg(feature = "app-relaymessagemenucontent")]
#[::unity2::methods]
impl RelayMessageMenuContent {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 1)]
    pub fn create(
        kind: crate::app::relaystampdata::RelayStampData_Kinds,
    ) -> crate::app::relaymessagemenucontent::RelayMessageMenuContent;

    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "Setup", args = 1)]
    pub fn setup(self, kind: crate::app::relaystampdata::RelayStampData_Kinds) -> ();

    #[method(name = "SetToPrevKind", args = 1)]
    pub fn set_to_prev_kind(
        self,
        now: crate::app::relaystampdata::RelayStampData_Kinds,
    ) -> crate::app::relaystampdata::RelayStampData_Kinds;

    #[method(name = "SetToNextKind", args = 1)]
    pub fn set_to_next_kind(
        self,
        now: crate::app::relaystampdata::RelayStampData_Kinds,
    ) -> crate::app::relaystampdata::RelayStampData_Kinds;

    #[method(name = "IsFirstKind", args = 1)]
    pub fn is_first_kind(self, kind: crate::app::relaystampdata::RelayStampData_Kinds) -> bool;

    #[method(name = "IsLastKind", args = 1)]
    pub fn is_last_kind(self, kind: crate::app::relaystampdata::RelayStampData_Kinds) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relaymessagemenucontent")]
impl RelayMessageMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayMessageMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayMessageMenuContentMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/relaymessagemenucontent/RelayMessageMenuContent_KindIcon.md")))]
#[::unity2::class(namespace = "App", name = "RelayMessageMenuContent.KindIcon")]
#[parent(crate::system::object::Object)]
pub struct RelayMessageMenuContent_KindIcon {
    #[rename(name = "m_Image")]
    pub m_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_Kind")]
    pub m_kind: crate::app::relaystampdata::RelayStampData_Kinds,
}

#[cfg(feature = "app-relaymessagemenucontent")]
#[::unity2::methods]
impl RelayMessageMenuContent_KindIcon {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-relaymessagemenucontent")]
impl RelayMessageMenuContent_KindIcon {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RelayMessageMenuContent_KindIcon),
                ::core::stringify!(new),
            )
        });
        <Self as IRelayMessageMenuContent_KindIconMethods>::ctor(this);
        this
    }
}
