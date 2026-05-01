
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/versusviolationtopcontent/VersusViolationTopContent.md")))]
#[::unity2::class(namespace = "App", name = "VersusViolationTopContent")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct VersusViolationTopContent {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_CaptureImage")]
    pub m_capture_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_CategoryText")]
    pub m_category_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ReasonText")]
    pub m_reason_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CaptureTitle")]
    pub m_capture_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CategoryTitle")]
    pub m_category_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ReasonTitle")]
    pub m_reason_title: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_CategoryWindow")]
    pub m_category_window: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_ReasonWindow")]
    pub m_reason_window: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "EnableAlpha")]
    pub enable_alpha: f32,
    #[rename(name = "DisableAlpha")]
    pub disable_alpha: f32,
}

#[cfg(feature = "app-versusviolationtopcontent")]
#[::unity2::methods]
impl VersusViolationTopContent {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::versusviolationtopcontent::VersusViolationTopContent;

    #[method(name = "Destroy", args = 1)]
    pub fn destroy(content: crate::app::versusviolationtopcontent::VersusViolationTopContent)
        -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "SetCapture", args = 1)]
    pub fn set_capture(self, tex: crate::unity_engine::texture2d::Texture2D) -> ();

    #[method(name = "SetCategoryText", args = 1)]
    pub fn set_category_text(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = "SetReasonText", args = 1)]
    pub fn set_reason_text(self, text: ::unity2::Il2CppString) -> ();

    #[method(name = "HideCategory", args = 0)]
    pub fn hide_category(self) -> ();

    #[method(name = "ShowCategory", args = 0)]
    pub fn show_category(self) -> ();

    #[method(name = "HideReason", args = 0)]
    pub fn hide_reason(self) -> ();

    #[method(name = "ShowReason", args = 0)]
    pub fn show_reason(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-versusviolationtopcontent")]
impl VersusViolationTopContent {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(VersusViolationTopContent),
                ::core::stringify!(new),
            )
        });
        <Self as IVersusViolationTopContentMethods>::ctor(this);
        this
    }
}
