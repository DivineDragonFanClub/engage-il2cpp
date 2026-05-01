
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardstamproot/ProfileCardStampRoot.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardStampRoot")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct ProfileCardStampRoot {
    #[rename(name = "m_StampCountCaption")]
    pub m_stamp_count_caption: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_StampCountText")]
    pub m_stamp_count_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_StampObject")]
    pub m_stamp_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_StampRectTransform")]
    pub m_stamp_rect_transform: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_StampAnimator")]
    pub m_stamp_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_StampGroupRectTransform")]
    pub m_stamp_group_rect_transform: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_StampImage")]
    pub m_stamp_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_StampImageRectTransform")]
    pub m_stamp_image_rect_transform: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_StampIconRectTransform")]
    pub m_stamp_icon_rect_transform: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_FrameObject")]
    pub m_frame_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_FrameImage")]
    pub m_frame_image: crate::unity_engine::ui::image::Image,
    #[rename(name = "m_StampEffectRectTransform")]
    pub m_stamp_effect_rect_transform: crate::unity_engine::recttransform::RectTransform,
    #[rename(name = "m_StampEffectAnimator")]
    pub m_stamp_effect_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_StampKeyHelpObject")]
    pub m_stamp_key_help_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_StampKeyHelpAnimator")]
    pub m_stamp_key_help_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_FrameColorEnable")]
    pub m_frame_color_enable: crate::unity_engine::color::Color,
    #[rename(name = "m_FrameColorDisable")]
    pub m_frame_color_disable: crate::unity_engine::color::Color,
    #[rename(name = "m_StampCountCurrent")]
    pub m_stamp_count_current: i32,
    #[rename(name = "m_StampCountMax")]
    pub m_stamp_count_max: i32,
    #[rename(name = "m_InitialStampPosition")]
    pub m_initial_stamp_position: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_InitialStampSize")]
    pub m_initial_stamp_size: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_StampScale")]
    pub m_stamp_scale: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_IsStarted")]
    pub m_is_started: bool,
    #[rename(name = "m_RecordRotZ")]
    pub m_record_rot_z: f32,
    #[rename(name = "m_RecordScale")]
    pub m_record_scale: crate::unity_engine::vector2::Vector2,
    #[rename(name = "m_RecordImageScale")]
    pub m_record_image_scale: crate::unity_engine::vector2::Vector2,
}

#[cfg(feature = "app-profilecardstamproot")]
#[::unity2::methods]
impl ProfileCardStampRoot {
    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "Reuse", args = 0)]
    pub fn reuse(self) -> ();

    #[method(name = "SetStampCountCurrent", args = 1)]
    pub fn set_stamp_count_current(self, count: i32) -> ();

    #[method(name = "SetStampCountMax", args = 1)]
    pub fn set_stamp_count_max(self, max: i32) -> ();

    #[method(name = "SetStampImage", args = 1)]
    pub fn set_stamp_image(
        self,
        stamp_data: crate::app::profilecardstampdata::ProfileCardStampData,
    ) -> ();

    #[method(name = "GetStampObject", args = 0)]
    pub fn get_stamp_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "SetStampObjectActive", args = 1)]
    pub fn set_stamp_object_active(self, actived: bool) -> ();

    #[method(name = "GetStampPosition", args = 0)]
    pub fn get_stamp_position(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "SetStampPosition", args = 1)]
    pub fn set_stamp_position(self, position: crate::unity_engine::vector2::Vector2) -> ();

    #[method(name = "ResetStampPosition", args = 0)]
    pub fn reset_stamp_position(self) -> ();

    #[method(name = "MoveStamp", args = 3)]
    pub fn move_stamp(
        self,
        movement: crate::unity_engine::vector2::Vector2,
        range: crate::unity_engine::rect::Rect,
        is_frame: bool,
    ) -> ();

    #[method(name = "GetStampRotation", args = 0)]
    pub fn get_stamp_rotation(self) -> f32;

    #[method(name = "ResetStampRotation", args = 0)]
    pub fn reset_stamp_rotation(self) -> ();

    #[method(name = "RotateStamp", args = 1)]
    pub fn rotate_stamp(self, rotation: f32) -> ();

    #[method(name = "GetStampScale", args = 0)]
    pub fn get_stamp_scale(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "ResetStampScale", args = 0)]
    pub fn reset_stamp_scale(self) -> ();

    #[method(name = "SetStampScale", args = 3)]
    pub fn set_stamp_scale(
        self,
        scaling: crate::unity_engine::vector2::Vector2,
        min: f32,
        max: f32,
    ) -> ();

    #[method(name = "ScaleStamp", args = 3)]
    pub fn scale_stamp(
        self,
        scaling: crate::unity_engine::vector2::Vector2,
        min: f32,
        max: f32,
    ) -> ();

    #[method(name = "ScaleStamp", args = 3)]
    pub fn scale_stamp_2(self, scaling: f32, min: f32, max: f32) -> ();

    #[method(name = "AddStampScale", args = 3)]
    pub fn add_stamp_scale(self, scaling: f32, min: f32, max: f32) -> ();

    #[method(name = "SaveTransform", args = 0)]
    pub fn save_transform(self) -> ();

    #[method(name = "LoadTransform", args = 0)]
    pub fn load_transform(self) -> ();

    #[method(name = "GetStampRect", args = 0)]
    pub fn get_stamp_rect(self) -> crate::app::profilecardroot::ProfileCardRoot_RectInfo;

    #[method(name = "GetIconRect", args = 0)]
    pub fn get_icon_rect(self) -> crate::app::profilecardroot::ProfileCardRoot_RectInfo;

    #[method(name = "GetStampImageScale", args = 0)]
    pub fn get_stamp_image_scale(self) -> crate::unity_engine::vector2::Vector2;

    #[method(name = "ResetStampImageScale", args = 0)]
    pub fn reset_stamp_image_scale(self) -> ();

    #[method(name = "ScaleStampImage", args = 3)]
    pub fn scale_stamp_image(
        self,
        scaling: crate::unity_engine::vector2::Vector2,
        min: f32,
        max: f32,
    ) -> ();

    #[method(name = "SetStampImageScale", args = 3)]
    pub fn set_stamp_image_scale(
        self,
        scaling: crate::unity_engine::vector2::Vector2,
        min: f32,
        max: f32,
    ) -> ();

    #[method(name = "PlayIdleStampAnimation", args = 0)]
    pub fn play_idle_stamp_animation(self) -> ();

    #[method(name = "PlayPushStampAnimation", args = 0)]
    pub fn play_push_stamp_animation(self) -> ();

    #[method(name = "PlayCatchStampAnimation", args = 0)]
    pub fn play_catch_stamp_animation(self) -> ();

    #[method(name = "PlayHoldStampAnimation", args = 0)]
    pub fn play_hold_stamp_animation(self) -> ();

    #[method(name = "PlayDeleteStampAnimation", args = 0)]
    pub fn play_delete_stamp_animation(self) -> ();

    #[method(name = "PlayEffectAnimation", args = 0)]
    pub fn play_effect_animation(self) -> ();

    #[method(name = "SetKeyHelpActive", args = 1)]
    pub fn set_key_help_active(self, actived: bool) -> ();

    #[method(name = "OpenKeyHelpWindow", args = 0)]
    pub fn open_key_help_window(self) -> ();

    #[method(name = "CloseKeyHelpWindow", args = 0)]
    pub fn close_key_help_window(self) -> ();

    #[method(name = "SetStampFrameActive", args = 1)]
    pub fn set_stamp_frame_active(self, is_active: bool) -> ();

    #[method(name = "SetStampFrameColor", args = 1)]
    pub fn set_stamp_frame_color(self, is_enable: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-profilecardstamproot")]
impl ProfileCardStampRoot {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardStampRoot),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardStampRootMethods>::ctor(this);
        this
    }
}
