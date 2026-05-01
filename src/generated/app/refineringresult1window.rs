
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineringresult1window/RefineRingResult1Window.md")))]
#[::unity2::class(namespace = "App", name = "RefineRingResult1Window")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct RefineRingResult1Window {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_CharacterImageRootAnimator")]
    pub m_character_image_root_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_RingParamRootAnimator")]
    pub m_ring_param_root_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_NewAnimator")]
    pub m_new_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_CharacterImage")]
    pub m_character_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_FrameImage")]
    pub m_frame_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_NewObject")]
    pub m_new_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_NewText")]
    pub m_new_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_RingIconImage")]
    pub m_ring_icon_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_CharacterNameText")]
    pub m_character_name_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_RingParams")]
    pub m_ring_params:
        ::unity2::Array<crate::app::refineringresult1window::RefineRingResult1Window_RingParam>,
    #[rename(name = "m_SkillRootObject")]
    pub m_skill_root_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_SkillIconImage")]
    pub m_skill_icon_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_SkillNameText")]
    pub m_skill_name_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_SkillHelpText")]
    pub m_skill_help_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-refineringresult1window")]
#[::unity2::methods]
impl RefineRingResult1Window {
    #[method(name = "LoadPrefabAsync", args = 0)]
    pub fn load_prefab_async() -> ();

    #[method(name = "IsLoadingPrefab", args = 0)]
    pub fn is_loading_prefab() -> bool;

    #[method(name = "UnloadPrefab", args = 0)]
    pub fn unload_prefab() -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> crate::app::refineringresult1window::RefineRingResult1Window;

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "IsOpening", args = 0)]
    pub fn is_opening(self) -> bool;

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "IsClosing", args = 0)]
    pub fn is_closing(self) -> bool;

    #[method(name = "SetData", args = 2)]
    pub fn set_data(self, ring_data: crate::app::ringdata::RingData, is_new: bool) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-refineringresult1window")]
impl RefineRingResult1Window {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineRingResult1Window),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineRingResult1WindowMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/refineringresult1window/RefineRingResult1Window_RingParam.md")))]
#[::unity2::class(namespace = "App", name = "RefineRingResult1Window.RingParam")]
#[parent(crate::system::object::Object)]
pub struct RefineRingResult1Window_RingParam {
    #[rename(name = "m_TitleText")]
    pub m_title_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ValueText")]
    pub m_value_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-refineringresult1window")]
#[::unity2::methods]
impl RefineRingResult1Window_RingParam {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-refineringresult1window")]
impl RefineRingResult1Window_RingParam {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RefineRingResult1Window_RingParam),
                ::core::stringify!(new),
            )
        });
        <Self as IRefineRingResult1Window_RingParamMethods>::ctor(this);
        this
    }
}
