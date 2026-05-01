
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versusmapeditthemeselectcontent/VersusMapEditThemeSelectContent.md")))]
#[::unity2::class(namespace = "App", name = "VersusMapEditThemeSelectContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct VersusMapEditThemeSelectContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_NavigationText")]
    pub m_navigation_text: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_NavigationPointList")]
    pub m_navigation_point_list: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
}

#[cfg(feature = "app-versusmapeditthemeselectcontent")]
#[::unity2::methods]
impl VersusMapEditThemeSelectContent {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::versusmapeditthemeselectcontent::VersusMapEditThemeSelectContent;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "SetNavigation", args = 1)]
    pub fn set_navigation(
        self,
        category : crate :: app :: profilecardthemeofeditmapdata :: ProfileCardThemeOfEditMapData_Categories,
    ) -> ();

    #[method(name = "SetInfoMessage", args = 0)]
    pub fn set_info_message(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-versusmapeditthemeselectcontent")]
impl VersusMapEditThemeSelectContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusMapEditThemeSelectContent),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusMapEditThemeSelectContentMethods>::ctor(this);
        this
    }
}
