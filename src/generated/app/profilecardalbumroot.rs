
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardalbumroot/ProfileCardAlbumRoot.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardAlbumRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct ProfileCardAlbumRoot {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_ProfileCardRoot")]
    pub m_profile_card_root: crate::app::profilecardroot::ProfileCardRoot,
    #[rename(name = "m_AlbumListMenuContent")]
    pub m_album_list_menu_content:
        crate::app::profilecardalbumlistmenucontent::ProfileCardAlbumListMenuContent,
    #[rename(name = "m_PageText")]
    pub m_page_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_PageMaxText")]
    pub m_page_max_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ArrowGroupObject")]
    pub m_arrow_group_object: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-profilecardalbumroot")]
#[::unity2::methods]
impl ProfileCardAlbumRoot {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "CreateRoot", args = 0)]
    pub fn create_root() -> crate::app::profilecardalbumroot::ProfileCardAlbumRoot;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "GetProfileCardRoot", args = 0)]
    pub fn get_profile_card_root(self) -> crate::app::profilecardroot::ProfileCardRoot;

    #[method(name = "GetAlbumListMenuContent", args = 0)]
    pub fn get_album_list_menu_content(
        self,
    ) -> crate::app::profilecardalbumlistmenucontent::ProfileCardAlbumListMenuContent;

    #[method(name = "SetPageNum", args = 2)]
    pub fn set_page_num(self, num: i32, warning_color: bool) -> ();

    #[method(name = "SetPageMax", args = 1)]
    pub fn set_page_max(self, max: i32) -> ();

    #[method(name = "SetActiveArrowGroup", args = 1)]
    pub fn set_active_arrow_group(self, enabled: bool) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "IsClosing", args = 0)]
    pub fn is_closing(self) -> bool;

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();
}

#[cfg(feature = "app-profilecardalbumroot")]
impl ProfileCardAlbumRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardAlbumRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardAlbumRootMethods>::ctor(this);
        this
    }
}
