
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineshopexchangeroot/RefineShopExchangeRoot.md")))]
#[::unity2::class(namespace = "App", name = "RefineShopExchangeRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct RefineShopExchangeRoot {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_TargetMenuObject")]
    pub m_target_menu_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_SourceMenuObject")]
    pub m_source_menu_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CountMenuObject")]
    pub m_count_menu_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ResultPopupObject")]
    pub m_result_popup_object: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-refineshopexchangeroot")]
#[::unity2::methods]
impl RefineShopExchangeRoot {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "CreateRoot", args = 0)]
    pub fn create_root() -> crate::app::refineshopexchangeroot::RefineShopExchangeRoot;

    #[method(name = "Create", args = 0)]
    pub fn create(self) -> ();

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetTargetMenuContent", args = 0)]
    pub fn get_target_menu_content(
        self,
    ) -> crate::app::refineshopexchangetargetmenucontent::RefineShopExchangeTargetMenuContent;

    #[method(name = "GetSourceMenuContent", args = 0)]
    pub fn get_source_menu_content(
        self,
    ) -> crate::app::refineshopexchangesourcemenucontent::RefineShopExchangeSourceMenuContent;

    #[method(name = "GetCountMenuContent", args = 0)]
    pub fn get_count_menu_content(
        self,
    ) -> crate::app::refineshopexchangecountmenucontent::RefineShopExchangeCountMenuContent;

    #[method(name = "GetResultPopup", args = 0)]
    pub fn get_result_popup(
        self,
    ) -> crate::app::refineshopexchangeresultpopup::RefineShopExchangeResultPopup;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-refineshopexchangeroot")]
impl RefineShopExchangeRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineShopExchangeRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineShopExchangeRootMethods>::ctor(this);
        this
    }
}
