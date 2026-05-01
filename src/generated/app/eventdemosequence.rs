
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::procscenesequence_1::IProcSceneSequence_1;
use crate::app::procscenesequence_1::ProcSceneSequence_1;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventdemosequence/EventDemoSequence_CmdInfo.md")))]
#[::unity2::class(namespace = "App", name = "EventDemoSequence.CmdInfo")]
#[parent(crate::system::object::Object)]
pub struct EventDemoSequence_CmdInfo {}

#[cfg(feature = "app-eventdemosequence")]
#[::unity2::methods]
impl EventDemoSequence_CmdInfo {
    #[method(name = "get_Func", args = 0)]
    pub fn get_func(self) -> crate::app::eventdemosequence::EventDemoSequence_CmdFunc;

    #[method(name = "set_Func", args = 1)]
    pub fn set_func(self, value: crate::app::eventdemosequence::EventDemoSequence_CmdFunc) -> ();

    #[method(name = "get_CmdName", args = 0)]
    pub fn get_cmd_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_CmdName", args = 1)]
    pub fn set_cmd_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Args", args = 0)]
    pub fn get_args(self) -> ::unity2::Array<::unity2::Il2CppString>;

    #[method(name = "set_Args", args = 1)]
    pub fn set_args(self, value: ::unity2::Array<::unity2::Il2CppString>) -> ();

    #[method(name = "get_RepeatCounter", args = 0)]
    pub fn get_repeat_counter(self) -> i32;

    #[method(name = "set_RepeatCounter", args = 1)]
    pub fn set_repeat_counter(self, value: i32) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        func: crate::app::eventdemosequence::EventDemoSequence_CmdFunc,
        cmd_name: ::unity2::Il2CppString,
        args: ::unity2::Array<::unity2::Il2CppString>,
    ) -> ();

    #[method(name = "IncRepetCounter", args = 0)]
    pub fn inc_repet_counter(self) -> ();
}

#[cfg(feature = "app-eventdemosequence")]
impl EventDemoSequence_CmdInfo {
    pub fn new(
        func: crate::app::eventdemosequence::EventDemoSequence_CmdFunc,
        cmd_name: ::unity2::Il2CppString,
        args: ::unity2::Array<::unity2::Il2CppString>,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventDemoSequence_CmdInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IEventDemoSequence_CmdInfoMethods>::ctor(this, func, cmd_name, args);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventdemosequence/EventDemoSequence_CmdFunc.md")))]
#[::unity2::class(namespace = "App", name = "EventDemoSequence.CmdFunc")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct EventDemoSequence_CmdFunc {}

#[cfg(feature = "app-eventdemosequence")]
#[::unity2::methods]
impl EventDemoSequence_CmdFunc {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(
        self,
        cmd_func_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;
}

#[cfg(feature = "app-eventdemosequence")]
impl EventDemoSequence_CmdFunc {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventDemoSequence_CmdFunc),
                ::core::stringify!(new),
            )
        });
        <Self as IEventDemoSequence_CmdFuncMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventdemosequence/EventDemoSequence_RotateFader.md")))]
#[::unity2::class(namespace = "App", name = "EventDemoSequence.RotateFader")]
#[parent(crate::system::object::Object)]
pub struct EventDemoSequence_RotateFader {
    #[rename(name = "m_rot")]
    pub m_rot: f32,
    #[rename(name = "m_rotFrom")]
    pub m_rot_from: f32,
    #[rename(name = "m_rotTo")]
    pub m_rot_to: f32,
    #[rename(name = "m_time")]
    pub m_time: f32,
    #[rename(name = "m_duration")]
    pub m_duration: f32,
}

#[cfg(feature = "app-eventdemosequence")]
#[::unity2::methods]
impl EventDemoSequence_RotateFader {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = "Set", args = 2)]
    pub fn set(self, rot: f32, msec: f32) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> f32;
}

#[cfg(feature = "app-eventdemosequence")]
impl EventDemoSequence_RotateFader {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventDemoSequence_RotateFader),
                ::core::stringify!(new),
            )
        });
        <Self as IEventDemoSequence_RotateFaderMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventdemosequence/EventDemoSequence_CharacterWork.md")))]
#[::unity2::class(namespace = "App", name = "EventDemoSequence.CharacterWork")]
#[parent(crate::system::object::Object)]
pub struct EventDemoSequence_CharacterWork {}

#[cfg(feature = "app-eventdemosequence")]
#[::unity2::methods]
impl EventDemoSequence_CharacterWork {
    #[method(name = "get_Pid", args = 0)]
    pub fn get_pid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Pid", args = 1)]
    pub fn set_pid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_PidForCreate", args = 0)]
    pub fn get_pid_for_create(self) -> ::unity2::Il2CppString;

    #[method(name = "set_PidForCreate", args = 1)]
    pub fn set_pid_for_create(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Character", args = 0)]
    pub fn get_character(self) -> crate::combat::character::Character;

    #[method(name = "set_Character", args = 1)]
    pub fn set_character(self, value: crate::combat::character::Character) -> ();

    #[method(name = "get_Appearance", args = 0)]
    pub fn get_appearance(self) -> crate::combat::characterappearance::CharacterAppearance;

    #[method(name = "set_Appearance", args = 1)]
    pub fn set_appearance(
        self,
        value: crate::combat::characterappearance::CharacterAppearance,
    ) -> ();

    #[method(name = "get_PositionLocater", args = 0)]
    pub fn get_position_locater(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_PositionLocater", args = 1)]
    pub fn set_position_locater(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "get_RotateFader", args = 0)]
    pub fn get_rotate_fader(self) -> crate::app::eventdemosequence::EventDemoSequence_RotateFader;

    #[method(name = "set_RotateFader", args = 1)]
    pub fn set_rotate_fader(
        self,
        value: crate::app::eventdemosequence::EventDemoSequence_RotateFader,
    ) -> ();

    #[method(name = "get_RotateYTo", args = 0)]
    pub fn get_rotate_y_to(self) -> f32;

    #[method(name = "set_RotateYTo", args = 1)]
    pub fn set_rotate_y_to(self, value: f32) -> ();

    #[method(name = "get_AnimStatehash", args = 0)]
    pub fn get_anim_statehash(self) -> i32;

    #[method(name = "set_AnimStatehash", args = 1)]
    pub fn set_anim_statehash(self, value: i32) -> ();

    #[method(name = "get_EquipWeaponAsset", args = 0)]
    pub fn get_equip_weapon_asset(self) -> crate::combat::characterasset::CharacterAsset;

    #[method(name = "set_EquipWeaponAsset", args = 1)]
    pub fn set_equip_weapon_asset(self, value: crate::combat::characterasset::CharacterAsset)
        -> ();

    #[method(name = "get_AnimatorResourceHandle", args = 0)]
    pub fn get_animator_resource_handle(self) -> crate::app::resourcehandle_2::ResourceHandle_2;

    #[method(name = "set_AnimatorResourceHandle", args = 1)]
    pub fn set_animator_resource_handle(
        self,
        value: crate::app::resourcehandle_2::ResourceHandle_2,
    ) -> ();

    #[method(name = "get_FishingRodResourceHandle", args = 0)]
    pub fn get_fishing_rod_resource_handle(self) -> crate::app::resourcehandle_2::ResourceHandle_2;

    #[method(name = "set_FishingRodResourceHandle", args = 1)]
    pub fn set_fishing_rod_resource_handle(
        self,
        value: crate::app::resourcehandle_2::ResourceHandle_2,
    ) -> ();

    #[method(name = "get_FishingRodObject", args = 0)]
    pub fn get_fishing_rod_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_FishingRodObject", args = 1)]
    pub fn set_fishing_rod_object(self, value: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        pid: ::unity2::Il2CppString,
        pid_for_create: ::unity2::Il2CppString,
        appearance: crate::combat::characterappearance::CharacterAppearance,
    ) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();
}

#[cfg(feature = "app-eventdemosequence")]
impl EventDemoSequence_CharacterWork {
    pub fn new(
        pid: ::unity2::Il2CppString,
        pid_for_create: ::unity2::Il2CppString,
        appearance: crate::combat::characterappearance::CharacterAppearance,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventDemoSequence_CharacterWork),
                ::core::stringify!(new),
            )
        });
        <Self as IEventDemoSequence_CharacterWorkMethods>::ctor(
            this,
            pid,
            pid_for_create,
            appearance,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventdemosequence/EventDemoSequence_SplitViewWork.md")))]
#[::unity2::class(namespace = "App", name = "EventDemoSequence.SplitViewWork")]
#[parent(crate::system::object::Object)]
pub struct EventDemoSequence_SplitViewWork {
    #[rename(name = "m_renderTextureHandle")]
    pub m_render_texture_handle: crate::app::resourcehandle_2::ResourceHandle_2,
}

#[cfg(feature = "app-eventdemosequence")]
#[::unity2::methods]
impl EventDemoSequence_SplitViewWork {
    #[method(name = "get_IsInitialized", args = 0)]
    pub fn get_is_initialized(self) -> bool;

    #[method(name = "set_IsInitialized", args = 1)]
    pub fn set_is_initialized(self, value: bool) -> ();

    #[method(name = "get_IsUVRectForGpuBoostMode", args = 0)]
    pub fn get_is_uv_rect_for_gpu_boost_mode(self) -> bool;

    #[method(name = "set_IsUVRectForGpuBoostMode", args = 1)]
    pub fn set_is_uv_rect_for_gpu_boost_mode(self, value: bool) -> ();

    #[method(name = "get_SplitViewName", args = 0)]
    pub fn get_split_view_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_SplitViewName", args = 1)]
    pub fn set_split_view_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_SplitViewImage", args = 0)]
    pub fn get_split_view_image(self) -> crate::unity_engine::ui::rawimage::RawImage;

    #[method(name = "set_SplitViewImage", args = 1)]
    pub fn set_split_view_image(self, value: crate::unity_engine::ui::rawimage::RawImage) -> ();

    #[method(name = "get_RenderImage", args = 0)]
    pub fn get_render_image(self) -> crate::unity_engine::ui::rawimage::RawImage;

    #[method(name = "set_RenderImage", args = 1)]
    pub fn set_render_image(self, value: crate::unity_engine::ui::rawimage::RawImage) -> ();

    #[method(name = "get_RenderTexture", args = 0)]
    pub fn get_render_texture(self) -> crate::unity_engine::rendertexture::RenderTexture;

    #[method(name = "set_RenderTexture", args = 1)]
    pub fn set_render_texture(self, value: crate::unity_engine::rendertexture::RenderTexture)
        -> ();

    #[method(name = "get_Camera", args = 0)]
    pub fn get_camera(self) -> crate::unity_engine::camera::Camera;

    #[method(name = "set_Camera", args = 1)]
    pub fn set_camera(self, value: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "get_ColorFader", args = 0)]
    pub fn get_color_fader(self) -> crate::app::eventdemosequence::EventDemoSequence_ColorFader;

    #[method(name = "set_ColorFader", args = 1)]
    pub fn set_color_fader(
        self,
        value: crate::app::eventdemosequence::EventDemoSequence_ColorFader,
    ) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        split_view_name: ::unity2::Il2CppString,
        render_texture_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "InitSplitViewImage", args = 1)]
    pub fn init_split_view_image(self, split_view_name: ::unity2::Il2CppString) -> ();

    #[method(name = "LoadRenderTexture", args = 1)]
    pub fn load_render_texture(self, render_texture_name: ::unity2::Il2CppString) -> ();

    #[method(name = "SetActive", args = 1)]
    pub fn set_active(self, is_active: bool) -> ();

    #[method(name = "SetRenderImageUVRectWH", args = 1)]
    pub fn set_render_image_uv_rect_wh(self, is_gpu_boost: bool) -> ();

    #[method(name = "SetRenderImageUVRectWH", args = 2)]
    pub fn set_render_image_uv_rect_wh_2(self, width: f32, height: f32) -> ();

    #[method(name = "DisableCamera", args = 0)]
    pub fn disable_camera(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();
}

#[cfg(feature = "app-eventdemosequence")]
impl EventDemoSequence_SplitViewWork {
    pub fn new(
        split_view_name: ::unity2::Il2CppString,
        render_texture_name: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventDemoSequence_SplitViewWork),
                ::core::stringify!(new),
            )
        });
        <Self as IEventDemoSequence_SplitViewWorkMethods>::ctor(
            this,
            split_view_name,
            render_texture_name,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventdemosequence/EventDemoSequence_WeightFader.md")))]
#[::unity2::class(namespace = "App", name = "EventDemoSequence.WeightFader")]
#[parent(crate::system::object::Object)]
pub struct EventDemoSequence_WeightFader {
    #[rename(name = "m_wgt")]
    pub m_wgt: f32,
    #[rename(name = "m_wgtFrom")]
    pub m_wgt_from: f32,
    #[rename(name = "m_wgtTo")]
    pub m_wgt_to: f32,
    #[rename(name = "m_time")]
    pub m_time: f32,
    #[rename(name = "m_duration")]
    pub m_duration: f32,
}

#[cfg(feature = "app-eventdemosequence")]
#[::unity2::methods]
impl EventDemoSequence_WeightFader {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> f32;

    #[method(name = "Set", args = 2)]
    pub fn set(self, wgt: f32, msec: f32) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> f32;
}

#[cfg(feature = "app-eventdemosequence")]
impl EventDemoSequence_WeightFader {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventDemoSequence_WeightFader),
                ::core::stringify!(new),
            )
        });
        <Self as IEventDemoSequence_WeightFaderMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventdemosequence/EventDemoSequence_EffectWork.md")))]
#[::unity2::class(namespace = "App", name = "EventDemoSequence.EffectWork")]
#[parent(crate::system::object::Object)]
pub struct EventDemoSequence_EffectWork {
    #[rename(name = "m_effectResourceObject")]
    pub m_effect_resource_object: crate::app::resourceobject::ResourceObject,
}

#[cfg(feature = "app-eventdemosequence")]
#[::unity2::methods]
impl EventDemoSequence_EffectWork {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        effect_path: ::unity2::Il2CppString,
        parent_trans: crate::unity_engine::transform::Transform,
    ) -> ();

    #[method(name = "LoadEffect", args = 2)]
    pub fn load_effect(
        self,
        effect_path: ::unity2::Il2CppString,
        parent_trans: crate::unity_engine::transform::Transform,
    ) -> ();

    #[method(name = "SetupAfterLoad", args = 1)]
    pub fn setup_after_load(
        effect_prefab_object: crate::unity_engine::gameobject::GameObject,
    ) -> ();

    #[method(name = "DeleteEffect", args = 0)]
    pub fn delete_effect(self) -> ();
}

#[cfg(feature = "app-eventdemosequence")]
impl EventDemoSequence_EffectWork {
    pub fn new(
        effect_path: ::unity2::Il2CppString,
        parent_trans: crate::unity_engine::transform::Transform,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventDemoSequence_EffectWork),
                ::core::stringify!(new),
            )
        });
        <Self as IEventDemoSequence_EffectWorkMethods>::ctor(this, effect_path, parent_trans);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventdemosequence/EventDemoSequence_EventCmdSeq.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct EventDemoSequence_EventCmdSeq {
    pub value: i32,
}

impl ::unity2::ClassIdentity for EventDemoSequence_EventCmdSeq {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "EventDemoSequence.EventCmdSeq";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for EventDemoSequence_EventCmdSeq {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl EventDemoSequence_EventCmdSeq {
    pub fn before_talk() -> Self {
        Self { value: 0 }
    }

    pub fn talk() -> Self {
        Self { value: 1 }
    }

    pub fn after_talk() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventdemosequence/EventDemoSequence.md")))]
#[::unity2::class(namespace = "App", name = "EventDemoSequence")]
# [parent (crate :: app :: procscenesequence_1 :: ProcSceneSequence_1 < crate :: app :: eventdemosequence :: EventDemoSequence >)]
pub struct EventDemoSequence {
    #[static_field]
    #[rename(name = "CameraGroupName")]
    pub camera_group_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "CurrentCameraName")]
    pub current_camera_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "LightDefaultRotateOffsetX")]
    pub light_default_rotate_offset_x: f32,
    #[static_field]
    #[rename(name = "LightDefaultRotateOffsetY")]
    pub light_default_rotate_offset_y: f32,
    #[static_field]
    #[rename(name = "LightDefaultRotateOffsetZ")]
    pub light_default_rotate_offset_z: f32,
    #[static_field]
    #[rename(name = "LightGroupName")]
    pub light_group_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "CurrentLightName")]
    pub current_light_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "UseSceneName")]
    pub use_scene_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "UseLayerName")]
    pub use_layer_name: ::unity2::Il2CppString,
    #[rename(name = "m_demoName")]
    pub m_demo_name: ::unity2::Il2CppString,
    #[rename(name = "m_messFileName")]
    pub m_mess_file_name: ::unity2::Il2CppString,
    #[rename(name = "m_curMessLabelIndex")]
    pub m_cur_mess_label_index: i32,
    #[rename(name = "m_curMessLabel")]
    pub m_cur_mess_label: ::unity2::Il2CppString,
    #[rename(name = "m_cmdInfosExecBefore")]
    pub m_cmd_infos_exec_before:
        ::unity2::Array<crate::app::eventdemosequence::EventDemoSequence_CmdInfo>,
    #[rename(name = "m_cmdInfosExecAfter")]
    pub m_cmd_infos_exec_after:
        ::unity2::Array<crate::app::eventdemosequence::EventDemoSequence_CmdInfo>,
    #[rename(name = "m_curCmdInfoIndex")]
    pub m_cur_cmd_info_index: i32,
    #[rename(name = "m_eventCmdSeq")]
    pub m_event_cmd_seq: crate::app::eventdemosequence::EventDemoSequence_EventCmdSeq,
    #[rename(name = "m_eventCmdTextExecBefore")]
    pub m_event_cmd_text_exec_before:
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            ::unity2::Il2CppString,
        >,
    #[rename(name = "m_eventCmdTextExecAfter")]
    pub m_event_cmd_text_exec_after:
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            ::unity2::Il2CppString,
        >,
    #[rename(name = "m_funcDictionary")]
    pub m_func_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::eventdemosequence::EventDemoSequence_CmdFunc,
    >,
    #[rename(name = "m_isPuppetTalkPause")]
    pub m_is_puppet_talk_pause: bool,
    #[rename(name = "m_isFadeOutInStart")]
    pub m_is_fade_out_in_start: bool,
    #[rename(name = "m_lightSetupInfo")]
    pub m_light_setup_info: crate::app::eventdemosequence::EventDemoSequence_LightSetupInfo,
    #[rename(name = "m_cubeMaterialHandle")]
    pub m_cube_material_handle: crate::app::resourcehandle_2::ResourceHandle_2,
    #[rename(name = "m_messLoadedList")]
    pub m_mess_loaded_list:
        crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[rename(name = "m_characterWorkDictionary")]
    pub m_character_work_dictionary:
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            crate::app::eventdemosequence::EventDemoSequence_CharacterWork,
        >,
    #[rename(name = "m_splitViewWorkDictionary")]
    pub m_split_view_work_dictionary:
        crate::system::collections::generic::dictionary_2::Dictionary_2<
            ::unity2::Il2CppString,
            crate::app::eventdemosequence::EventDemoSequence_SplitViewWork,
        >,
    #[rename(name = "m_effectWorkDictionary")]
    pub m_effect_work_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        crate::app::eventdemosequence::EventDemoSequence_EffectWork,
    >,
    #[rename(name = "m_telopEffectResourceObject")]
    pub m_telop_effect_resource_object: crate::app::resourceobject::ResourceObject,
    #[rename(name = "m_PictureController")]
    pub m_picture_controller: crate::app::eventpicturecontroller::EventPictureController,
}

#[cfg(feature = "app-eventdemosequence")]
#[::unity2::methods]
impl EventDemoSequence {
    #[method(name = "AnalysisCmdText", args = 1)]
    pub fn analysis_cmd_text(
        self,
        cmd_texts: ::unity2::Il2CppString,
    ) -> ::unity2::Array<crate::app::eventdemosequence::EventDemoSequence_CmdInfo>;

    #[method(name = "GetCmdInfoFromCmdLines", args = 1)]
    pub fn get_cmd_info_from_cmd_lines(
        self,
        cmd_line: ::unity2::Il2CppString,
    ) -> crate::app::eventdemosequence::EventDemoSequence_CmdInfo;

    #[method(name = "ExecEventCmd", args = 2)]
    pub fn exec_event_cmd(
        self,
        cmd_infos: ::unity2::Array<crate::app::eventdemosequence::EventDemoSequence_CmdInfo>,
        cmd_info_index: i32,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "ExecEventCmdImpl", args = 1)]
    pub fn exec_event_cmd_impl(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncLabel", args = 1)]
    pub fn func_label(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncJump", args = 1)]
    pub fn func_jump(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncWait", args = 1)]
    pub fn func_wait(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncVariant", args = 1)]
    pub fn func_variant(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "SetBackGround", args = 1)]
    pub fn set_back_ground(self, sky_box_material_name: ::unity2::Il2CppString) -> ();

    #[method(name = "GetSkyBoxMaterialNameAuto", args = 1)]
    pub fn get_sky_box_material_name_auto(
        self,
        base_name: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "FuncSetBackGround", args = 1)]
    pub fn func_set_back_ground(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncSetBackGroundAuto", args = 1)]
    pub fn func_set_back_ground_auto(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "DisableCameras", args = 0)]
    pub fn disable_cameras(self) -> ();

    #[method(name = "CreateCurrentCamera", args = 3)]
    pub fn create_current_camera(
        self,
        src_camera_name: ::unity2::Il2CppString,
        is_chara_camera_target_hero_female: bool,
        is_split_view_camera: bool,
    ) -> crate::unity_engine::camera::Camera;

    #[method(name = "CreateCamera", args = 4)]
    pub fn create_camera(
        self,
        camera_name: ::unity2::Il2CppString,
        src_camera_name: ::unity2::Il2CppString,
        is_chara_camera_target_hero_female: bool,
        is_split_view_camera: bool,
    ) -> crate::unity_engine::camera::Camera;

    #[method(name = "GetSrcCamera", args = 2)]
    pub fn get_src_camera(
        self,
        src_camera_name: ::unity2::Il2CppString,
        is_warning: bool,
    ) -> crate::unity_engine::camera::Camera;

    #[method(name = "CreateParentObjectOfCamera", args = 1)]
    pub fn create_parent_object_of_camera(
        self,
        camera: crate::unity_engine::camera::Camera,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "SetCameraForChara", args = 2)]
    pub fn set_camera_for_chara(
        self,
        camera: crate::unity_engine::camera::Camera,
        character: crate::combat::character::Character,
    ) -> ();

    #[method(name = "GetCharacterCameraAdjustObject", args = 1)]
    pub fn get_character_camera_adjust_object(
        self,
        character: crate::combat::character::Character,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "InitCharacterCameraAdjustTransform", args = 1)]
    pub fn init_character_camera_adjust_transform(
        self,
        character: crate::combat::character::Character,
    ) -> ();

    #[method(name = "SetCameraForScene", args = 1)]
    pub fn set_camera_for_scene(self, camera: crate::unity_engine::camera::Camera) -> ();

    #[method(name = "FuncCameraSetCharaCamera", args = 1)]
    pub fn func_camera_set_chara_camera(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "SetCharaCameraNoDelay", args = 1)]
    pub fn set_chara_camera_no_delay(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncCameraSetSceneCamera", args = 1)]
    pub fn func_camera_set_scene_camera(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "SetSceneCameraNoDelay", args = 1)]
    pub fn set_scene_camera_no_delay(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "GetSplitViewWork", args = 2)]
    pub fn get_split_view_work(
        self,
        split_view_name: ::unity2::Il2CppString,
        is_warning: bool,
    ) -> crate::app::eventdemosequence::EventDemoSequence_SplitViewWork;

    #[method(name = "FuncSplitViewBeginSplitViewCameraOnly", args = 1)]
    pub fn func_split_view_begin_split_view_camera_only(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncSplitViewEndSplitViewCameraOnly", args = 1)]
    pub fn func_split_view_end_split_view_camera_only(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncSplitViewCreate", args = 1)]
    pub fn func_split_view_create(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncSplitViewSetCharaCamera", args = 1)]
    pub fn func_split_view_set_chara_camera(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncSplitViewSetSceneCamera", args = 1)]
    pub fn func_split_view_set_scene_camera(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncSplitViewSetActive", args = 1)]
    pub fn func_split_view_set_active(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncSplitViewPlayAnim", args = 1)]
    pub fn func_split_view_play_anim(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncSplitViewWaitAnimEnd", args = 1)]
    pub fn func_split_view_wait_anim_end(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "DisableLights", args = 0)]
    pub fn disable_lights(self) -> ();

    #[method(name = "SetupLight", args = 1)]
    pub fn setup_light(
        self,
        light_setup_info: crate::app::eventdemosequence::EventDemoSequence_LightSetupInfo,
    ) -> ();

    #[method(name = "FuncLightSetup", args = 1)]
    pub fn func_light_setup(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncLightSetupAuto", args = 1)]
    pub fn func_light_setup_auto(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "SetLightCommon", args = 2)]
    pub fn set_light_common(
        self,
        light_name: ::unity2::Il2CppString,
        rot_offset: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "FuncFadeIn", args = 1)]
    pub fn func_fade_in(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncFadeOut", args = 1)]
    pub fn func_fade_out(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncWhiteFadeIn", args = 1)]
    pub fn func_white_fade_in(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncWhiteFadeOut", args = 1)]
    pub fn func_white_fade_out(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FadeInOutImpl", args = 3)]
    pub fn fade_in_out_impl(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
        color: crate::unity_engine::color::Color,
        is_fade_in: bool,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "GetCharacterWork", args = 2)]
    pub fn get_character_work(
        self,
        pid: ::unity2::Il2CppString,
        is_warning: bool,
    ) -> crate::app::eventdemosequence::EventDemoSequence_CharacterWork;

    #[method(name = "GetCharacterAppearance", args = 2)]
    pub fn get_character_appearance(
        self,
        pid: ::unity2::Il2CppString,
        is_warning: bool,
    ) -> crate::combat::characterappearance::CharacterAppearance;

    #[method(name = "GetCharacterLocator", args = 2)]
    pub fn get_character_locator(
        self,
        pid: ::unity2::Il2CppString,
        is_warning: bool,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "GetCharacter", args = 2)]
    pub fn get_character(
        self,
        pid: ::unity2::Il2CppString,
        is_warning: bool,
    ) -> crate::combat::character::Character;

    #[method(name = "FindCharacterLocator", args = 1)]
    pub fn find_character_locator(
        self,
        pos_string: ::unity2::Il2CppString,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "PlayCharacterAnim", args = 4)]
    pub fn play_character_anim(
        self,
        character: crate::combat::character::Character,
        facial_anim_name: ::unity2::Il2CppString,
        body_anim_name: ::unity2::Il2CppString,
        transition_duration: f32,
    ) -> ();

    #[method(name = "GetAssetTable", args = 2)]
    pub fn get_asset_table(
        self,
        pid: ::unity2::Il2CppString,
        cloth_type: crate::app::eventdemosequence::EventDemoSequence_ClothType,
    ) -> crate::app::assettable::AssetTable_Result;

    #[method(name = "FuncCharacterCreate", args = 1)]
    pub fn func_character_create(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncCharacterDelete", args = 1)]
    pub fn func_character_delete(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncCharacterAdjustPos", args = 1)]
    pub fn func_character_adjust_pos(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncCharacterShowHide", args = 1)]
    pub fn func_character_show_hide(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncCharacterSetAnimator", args = 1)]
    pub fn func_character_set_animator(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncCharacterPlayMotion", args = 1)]
    pub fn func_character_play_motion(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncCharacterWaitMotion", args = 1)]
    pub fn func_character_wait_motion(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "SetCharacterAngle", args = 3)]
    pub fn set_character_angle(
        self,
        character_self: crate::combat::character::Character,
        character_eye_target: crate::combat::character::Character,
        character_head_target: crate::combat::character::Character,
    ) -> ();

    #[method(name = "FuncCharacterSetAngle", args = 1)]
    pub fn func_character_set_angle(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "SetCharacterAngle", args = 3)]
    pub fn set_character_angle_2(
        self,
        self_pid: ::unity2::Il2CppString,
        eye_target_pid: ::unity2::Il2CppString,
        head_target_pid: ::unity2::Il2CppString,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncCharacterResetAngle", args = 1)]
    pub fn func_character_reset_angle(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncCharacterSetRotate", args = 1)]
    pub fn func_character_set_rotate(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "SetCharacterRotate", args = 4)]
    pub fn set_character_rotate(
        self,
        character: crate::combat::character::Character,
        character_work: crate::app::eventdemosequence::EventDemoSequence_CharacterWork,
        rotate_y: f32,
        sec: f32,
    ) -> ();

    #[method(name = "FuncCharacterEquipWeapon", args = 1)]
    pub fn func_character_equip_weapon(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncCharacterEquipNoWeapon", args = 1)]
    pub fn func_character_equip_no_weapon(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncCharacterEquipFishingRod", args = 1)]
    pub fn func_character_equip_fishing_rod(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncCharacterEquipNoFishingRod", args = 1)]
    pub fn func_character_equip_no_fishing_rod(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "GetEffectWork", args = 2)]
    pub fn get_effect_work(
        self,
        effect_name: ::unity2::Il2CppString,
        is_warning: bool,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EffectWork;

    #[method(name = "FuncCreateEffect", args = 1)]
    pub fn func_create_effect(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncDeleteEffect", args = 1)]
    pub fn func_delete_effect(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "DeleteTelopEffect", args = 0)]
    pub fn delete_telop_effect(self) -> ();

    #[method(name = "FuncCreateTelopEffect", args = 1)]
    pub fn func_create_telop_effect(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncDeleteTelopEffect", args = 1)]
    pub fn func_delete_telop_effect(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncPausePuppetTalk", args = 1)]
    pub fn func_pause_puppet_talk(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncMessageLoad", args = 1)]
    pub fn func_message_load(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncTalkFaceBegin", args = 1)]
    pub fn func_talk_face_begin(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncPictureShow", args = 1)]
    pub fn func_picture_show(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncPictureHide", args = 1)]
    pub fn func_picture_hide(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncChapterTitleShow", args = 1)]
    pub fn func_chapter_title_show(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "FuncSoundEvent", args = 1)]
    pub fn func_sound_event(
        self,
        cmd_info: crate::app::eventdemosequence::EventDemoSequence_CmdInfo,
    ) -> crate::app::eventdemosequence::EventDemoSequence_EventCmdResult;

    #[method(name = "SetupCommnads", args = 0)]
    pub fn setup_commnads(self) -> ();

    #[method(name = ".ctor", args = 2)]
    pub fn ctor(
        self,
        mess_file_name_without_ext: ::unity2::Il2CppString,
        demo_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "FindGameObject", args = 1)]
    pub fn find_game_object(
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = "IsSkip", args = 0)]
    pub fn is_skip(self) -> bool;

    #[method(name = "IsHeroFemale", args = 0)]
    pub fn is_hero_female(self) -> bool;

    #[method(name = "IsPidHero", args = 1)]
    pub fn is_pid_hero(self, pid: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "PushFade", args = 0)]
    pub fn push_fade(self) -> ();

    #[method(name = "PopFade", args = 0)]
    pub fn pop_fade(self) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "End", args = 0)]
    pub fn end(self) -> ();

    #[method(name = "LoadEventCmd", args = 0)]
    pub fn load_event_cmd(self) -> ();

    #[method(name = "UnloadEventCmd", args = 0)]
    pub fn unload_event_cmd(self) -> ();

    #[method(name = "LoadDemoScene", args = 0)]
    pub fn load_demo_scene(self) -> ();

    #[method(name = "UnloadDemoScene", args = 0)]
    pub fn unload_demo_scene(self) -> ();

    #[method(name = "InitAfterLoadScene", args = 0)]
    pub fn init_after_load_scene(self) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "TickImpl", args = 1)]
    pub fn tick_impl(self, label: ::unity2::Il2CppString) -> bool;

    #[method(name = "Persistent", args = 0)]
    pub fn persistent(self) -> ();

    #[method(name = "OnShutdown", args = 0)]
    pub fn on_shutdown(self) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        mess_file_name_without_ext: ::unity2::Il2CppString,
        demo_name: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "GetGodRelianceMessFileName", args = 2)]
    pub fn get_god_reliance_mess_file_name(
        mess_file_name_without_ext: ::unity2::Il2CppString,
        demo_name: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-eventdemosequence")]
impl EventDemoSequence {
    pub fn new(
        mess_file_name_without_ext: ::unity2::Il2CppString,
        demo_name: ::unity2::Il2CppString,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventDemoSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IEventDemoSequenceMethods>::ctor(this, mess_file_name_without_ext, demo_name);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventdemosequence/EventDemoSequence_LightSetupInfo.md")))]
#[::unity2::class(namespace = "App", name = "EventDemoSequence.LightSetupInfo")]
#[parent(crate::system::object::Object)]
pub struct EventDemoSequence_LightSetupInfo {}

#[cfg(feature = "app-eventdemosequence")]
#[::unity2::methods]
impl EventDemoSequence_LightSetupInfo {
    #[method(name = "get_LightName", args = 0)]
    pub fn get_light_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_LightName", args = 1)]
    pub fn set_light_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_RotOffset", args = 0)]
    pub fn get_rot_offset(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "set_RotOffset", args = 1)]
    pub fn set_rot_offset(self, value: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "get_ParentCameraObject", args = 0)]
    pub fn get_parent_camera_object(self) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "set_ParentCameraObject", args = 1)]
    pub fn set_parent_camera_object(self, value: crate::unity_engine::gameobject::GameObject)
        -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-eventdemosequence")]
impl EventDemoSequence_LightSetupInfo {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventDemoSequence_LightSetupInfo),
                ::core::stringify!(new),
            )
        });
        <Self as IEventDemoSequence_LightSetupInfoMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventdemosequence/EventDemoSequence_ColorFader.md")))]
#[::unity2::class(namespace = "App", name = "EventDemoSequence.ColorFader")]
#[parent(crate::system::object::Object)]
pub struct EventDemoSequence_ColorFader {
    #[rename(name = "m_color")]
    pub m_color: crate::unity_engine::color::Color,
    #[rename(name = "m_colorFrom")]
    pub m_color_from: crate::unity_engine::color::Color,
    #[rename(name = "m_colorTo")]
    pub m_color_to: crate::unity_engine::color::Color,
    #[rename(name = "m_time")]
    pub m_time: f32,
    #[rename(name = "m_duration")]
    pub m_duration: f32,
}

#[cfg(feature = "app-eventdemosequence")]
#[::unity2::methods]
impl EventDemoSequence_ColorFader {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "Get", args = 0)]
    pub fn get(self) -> crate::unity_engine::color::Color;

    #[method(name = "Set", args = 2)]
    pub fn set(self, color: crate::unity_engine::color::Color, msec: f32) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> crate::unity_engine::color::Color;
}

#[cfg(feature = "app-eventdemosequence")]
impl EventDemoSequence_ColorFader {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventDemoSequence_ColorFader),
                ::core::stringify!(new),
            )
        });
        <Self as IEventDemoSequence_ColorFaderMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventdemosequence/EventDemoSequence_EventCmdResult.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct EventDemoSequence_EventCmdResult {
    pub value: i32,
}

impl ::unity2::ClassIdentity for EventDemoSequence_EventCmdResult {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "EventDemoSequence.EventCmdResult";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for EventDemoSequence_EventCmdResult {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl EventDemoSequence_EventCmdResult {
    pub fn r#continue() -> Self {
        Self { value: 0 }
    }

    pub fn continue_next_frame() -> Self {
        Self { value: 1 }
    }

    pub fn retry() -> Self {
        Self { value: 2 }
    }

    pub fn retry_next_frame() -> Self {
        Self { value: 3 }
    }

    pub fn cmd_not_found() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventdemosequence/EventDemoSequence_SoundEnv_EventNameData.md")))]
#[::unity2::class(namespace = "App", name = "EventDemoSequence.SoundEnv.EventNameData")]
#[parent(crate::system::object::Object)]
pub struct EventDemoSequence_SoundEnv_EventNameData {
    #[rename(name = "m_MaterialName")]
    pub m_material_name: ::unity2::Il2CppString,
    #[rename(name = "m_EventName")]
    pub m_event_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-eventdemosequence")]
#[::unity2::methods]
impl EventDemoSequence_SoundEnv_EventNameData {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-eventdemosequence")]
impl EventDemoSequence_SoundEnv_EventNameData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventDemoSequence_SoundEnv_EventNameData),
                ::core::stringify!(new),
            )
        });
        <Self as IEventDemoSequence_SoundEnv_EventNameDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventdemosequence/EventDemoSequence_ClothType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct EventDemoSequence_ClothType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for EventDemoSequence_ClothType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "EventDemoSequence.ClothType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for EventDemoSequence_ClothType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl EventDemoSequence_ClothType {
    pub fn normal() -> Self {
        Self { value: 0 }
    }

    pub fn default_job() -> Self {
        Self { value: 1 }
    }

    pub fn plain() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/eventdemosequence/EventDemoSequence_SoundEnv.md")))]
#[::unity2::class(namespace = "App", name = "EventDemoSequence.SoundEnv")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct EventDemoSequence_SoundEnv {
    #[rename(name = "m_EventNameDataList")]
    pub m_event_name_data_list: crate::system::collections::generic::list_1::List_1<
        crate::app::eventdemosequence::EventDemoSequence_SoundEnv_EventNameData,
    >,
    #[rename(name = "m_EventNameDictionary")]
    pub m_event_name_dictionary: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        ::unity2::Il2CppString,
    >,
    #[rename(name = "m_SoundEventName")]
    pub m_sound_event_name: ::unity2::Il2CppString,
}

#[cfg(feature = "app-eventdemosequence")]
#[::unity2::methods]
impl EventDemoSequence_SoundEnv {
    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "OnDestroy", args = 0)]
    pub fn on_destroy(self) -> ();

    #[method(name = "GetEventName", args = 1)]
    pub fn get_event_name(self, material_name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "PlayEnv", args = 1)]
    pub fn play_env(self, material_name: ::unity2::Il2CppString) -> ();

    #[method(name = "StopCurrentEnv", args = 0)]
    pub fn stop_current_env(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-eventdemosequence")]
impl EventDemoSequence_SoundEnv {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(EventDemoSequence_SoundEnv),
                ::core::stringify!(new),
            )
        });
        <Self as IEventDemoSequence_SoundEnvMethods>::ctor(this);
        this
    }
}
