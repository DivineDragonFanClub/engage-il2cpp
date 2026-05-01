
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mascotaccchangecontent/MascotAccChangeContent.md")))]
#[::unity2::class(namespace = "App", name = "MascotAccChangeContent")]
#[parent(crate::app::basicmenucontent::BasicMenuContent)]
pub struct MascotAccChangeContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "UIName")]
    pub ui_name: ::unity2::Il2CppString,
    #[rename(name = "m_category1")]
    pub m_category1: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_category2")]
    pub m_category2: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_content1")]
    pub m_content1: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_content2")]
    pub m_content2: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_exchangeCursor")]
    pub m_exchange_cursor: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ChoiceAcc")]
    pub m_choice_acc: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_AccName")]
    pub m_acc_name: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_AccIcon")]
    pub m_acc_icon: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_AccParts")]
    pub m_acc_parts: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_AccHelp")]
    pub m_acc_help: crate::unity_engine::gameobject::GameObject,
}

#[cfg(feature = "app-mascotaccchangecontent")]
#[::unity2::methods]
impl MascotAccChangeContent {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::mascotaccchangecontent::MascotAccChangeContent;

    #[method(name = "Destroy", args = 0)]
    pub fn destroy() -> ();

    #[method(name = "SetActiveParts", args = 1)]
    pub fn set_active_parts(self, r#type: crate::app::mascotaccdata::MascotAccData_PartsType)
        -> ();

    #[method(name = "SetHelpText", args = 2)]
    pub fn set_help_text(
        self,
        mascot_data: crate::app::mascotaccdata::MascotAccData,
        acc_data: crate::app::accessorydata::AccessoryData,
    ) -> ();

    #[method(name = "UpdateEquipText", args = 2)]
    pub fn update_equip_text(
        self,
        text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
        aid: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "UpdateEquipAcc", args = 0)]
    pub fn update_equip_acc(self) -> ();

    #[method(name = "AfterBuild", args = 0)]
    pub fn after_build(self) -> ();

    #[method(name = "GetMenuItemContentMax", args = 0)]
    pub fn get_menu_item_content_max(self) -> i32;

    #[method(name = "CalcW", args = 0)]
    pub fn calc_w(self) -> f32;

    #[method(name = "CalcH", args = 0)]
    pub fn calc_h(self) -> f32;

    #[method(name = "CalcCursorMovedPosY", args = 1)]
    pub fn calc_cursor_moved_pos_y(self, menu_item_index: i32) -> f32;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mascotaccchangecontent")]
impl MascotAccChangeContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MascotAccChangeContent),
                ::core::stringify!(new),
            )
        });
        <Self as IMascotAccChangeContentMethods>::ctor(this);
        this
    }
}
