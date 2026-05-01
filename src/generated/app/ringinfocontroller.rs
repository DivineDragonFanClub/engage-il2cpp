
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringinfocontroller/RingInfoController.md")))]
#[::unity2::class(namespace = "App", name = "RingInfoController")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct RingInfoController {
    #[rename(name = "m_RingName")]
    pub m_ring_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_RingConditionIconImage")]
    pub m_ring_condition_icon_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_GodName")]
    pub m_god_name: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_RingImage")]
    pub m_ring_image: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Message")]
    pub m_message: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ConditionIcons")]
    pub m_condition_icons:
        crate::system::collections::generic::list_1::List_1<crate::unity_engine::sprite::Sprite>,
}

#[cfg(feature = "app-ringinfocontroller")]
#[::unity2::methods]
impl RingInfoController {
    #[method(name = "GetHelpText", args = 2)]
    pub fn get_help_text(
        self,
        god: crate::app::godunit::GodUnit,
        r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetRingNameText", args = 2)]
    pub fn get_ring_name_text(
        self,
        god: crate::app::godunit::GodUnit,
        r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ::unity2::Il2CppString;

    #[method(name = "SetStatus", args = 2)]
    pub fn set_status(
        self,
        god: crate::app::godunit::GodUnit,
        r#type: crate::app::ringcleaningsequence::RingCleaningSequence_GodType,
    ) -> ();

    #[method(name = "set_RingImageTexture", args = 1)]
    pub fn set_ring_image_texture(
        self,
        value: crate::unity_engine::rendertexture::RenderTexture,
    ) -> ();

    #[method(name = "Open", args = 0)]
    pub fn open(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-ringinfocontroller")]
impl RingInfoController {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingInfoController),
                ::core::stringify!(new),
            )
        });
        <Self as IRingInfoControllerMethods>::ctor(this);
        this
    }
}
