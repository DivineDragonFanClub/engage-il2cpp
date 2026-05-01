
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortiesubmenucontent/SortieSubMenuContent.md")))]
#[::unity2::class(namespace = "App", name = "SortieSubMenuContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct SortieSubMenuContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_objFrmContent")]
    pub m_obj_frm_content: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-sortiesubmenucontent")]
#[::unity2::methods]
impl SortieSubMenuContent {
    #[method(name = "CalcW", args = 0)]
    pub fn calc_w(self) -> f32;

    #[method(name = "CalcH", args = 0)]
    pub fn calc_h(self) -> f32;

    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::sortiesubmenucontent::SortieSubMenuContent;

    #[method(name = "InitObjReference", args = 0)]
    pub fn init_obj_reference(self) -> ();

    #[method(name = "BuildMenuItemContent", args = 0)]
    pub fn build_menu_item_content(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-sortiesubmenucontent")]
impl SortieSubMenuContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieSubMenuContent),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieSubMenuContentMethods>::ctor(this);
        this
    }
}
