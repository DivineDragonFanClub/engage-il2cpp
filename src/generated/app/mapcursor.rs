
use crate::app::bitfield32::BitField32;
use crate::app::bitfield32::IBitField32;
use crate::app::bitfieldcommon::BitFieldCommon;
use crate::app::bitfieldcommon::IBitFieldCommon;
use crate::app::bitfieldtemplate32_1::BitFieldTemplate32_1;
use crate::app::bitfieldtemplate32_1::IBitFieldTemplate32_1;
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapcursor/MapCursor_AnimType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapCursor_AnimType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapCursor_AnimType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapCursor.AnimType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapCursor_AnimType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapCursor_AnimType {
    pub fn none() -> Self {
        Self { value: 1 }
    }

    pub fn r#in() -> Self {
        Self { value: 2 }
    }

    pub fn out() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapcursor/MapCursor_Flag.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapCursor_Flag {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapCursor_Flag {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapCursor.Flag";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapCursor_Flag {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapCursor_Flag {
    pub fn hide() -> Self {
        Self { value: 1 }
    }

    pub fn auto() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapcursor/MapCursor_CursorTopType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapCursor_CursorTopType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapCursor_CursorTopType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapCursor.CursorTopType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapCursor_CursorTopType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapCursor_CursorTopType {
    pub fn pointer() -> Self {
        Self { value: 0 }
    }

    pub fn attack() -> Self {
        Self { value: 1 }
    }

    pub fn talk() -> Self {
        Self { value: 2 }
    }

    pub fn rod() -> Self {
        Self { value: 3 }
    }

    pub fn dance() -> Self {
        Self { value: 4 }
    }

    pub fn cannon() -> Self {
        Self { value: 5 }
    }

    pub fn num() -> Self {
        Self { value: 6 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapcursor/MapCursor_DistanceMode.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapCursor_DistanceMode {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapCursor_DistanceMode {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapCursor.DistanceMode";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapCursor_DistanceMode {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapCursor_DistanceMode {
    pub fn near() -> Self {
        Self { value: 0 }
    }

    pub fn middle() -> Self {
        Self { value: 1 }
    }

    pub fn far() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapcursor/MapCursor_FlagField.md")))]
#[::unity2::class(namespace = "App", name = "MapCursor.FlagField")]
# [parent (crate :: app :: bitfieldtemplate32_1 :: BitFieldTemplate32_1 < crate :: app :: mapcursor :: MapCursor_Flag >)]
pub struct MapCursor_FlagField {}

#[cfg(feature = "app-mapcursor")]
#[::unity2::methods]
impl MapCursor_FlagField {
    #[method(name = "ToInt", args = 1)]
    pub fn to_int(self, value: crate::app::mapcursor::MapCursor_Flag) -> i32;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapcursor")]
impl MapCursor_FlagField {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapCursor_FlagField),
                ::core::stringify!(new),
            )
        });
        <Self as IMapCursor_FlagFieldMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapcursor/MapCursor.md")))]
#[::unity2::class(namespace = "App", name = "MapCursor")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: mapcursor :: MapCursor >)]
pub struct MapCursor {
    #[static_field]
    #[rename(name = "MapCursorMoveOffset")]
    pub map_cursor_move_offset: f32,
    #[static_field]
    #[rename(name = "MOVE_WAIT_FRAME")]
    pub move_wait_frame: i32,
    #[static_field]
    #[rename(name = "MOVE_CENTER_FRAME")]
    pub move_center_frame: i32,
    #[rename(name = "m_Pos")]
    pub m_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_OldPos")]
    pub m_old_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_OrigPos")]
    pub m_orig_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_Rotate")]
    pub m_rotate: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_Move")]
    pub m_move: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_EnterPos")]
    pub m_enter_pos: crate::unity_engine::vector3::Vector3,
    #[rename(name = "m_MoveCount")]
    pub m_move_count: i32,
    #[rename(name = "m_IsRotateXSoundPlaying")]
    pub m_is_rotate_x_sound_playing: bool,
    #[static_field]
    #[rename(name = "m_CursorTop")]
    pub m_cursor_top: crate::app::mapcursor::MapCursor_CursorTopType,
    #[static_field]
    #[rename(name = "m_CursorTopAnim")]
    pub m_cursor_top_anim: crate::app::mapcursor::MapCursor_AnimType,
    #[static_field]
    #[rename(name = "m_CursorBottomAnim")]
    pub m_cursor_bottom_anim: crate::app::mapcursor::MapCursor_AnimType,
    #[rename(name = "m_CursorIconBill")]
    pub m_cursor_icon_bill: crate::unity_engine::transform::Transform,
    #[rename(name = "m_DistanceDir")]
    pub m_distance_dir: i32,
    #[rename(name = "m_AnalogCount")]
    pub m_analog_count: i32,
    #[rename(name = "m_CenterCount")]
    pub m_center_count: i32,
    #[rename(name = "m_DistanceScale")]
    pub m_distance_scale: f32,
    #[rename(name = "m_MapCursorMoveType")]
    pub m_map_cursor_move_type: crate::app::gameconfig::GameConfig_MapCursorMoveTyep,
    #[rename(name = "m_IsLockMoveType")]
    pub m_is_lock_move_type: bool,
    #[rename(name = "m_CursorTopObj")]
    pub m_cursor_top_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_CursorBottomObj")]
    pub m_cursor_bottom_obj: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_PositionTop")]
    pub m_position_top: crate::app::interpolatorvector3::InterpolatorVector3,
    #[rename(name = "m_PositionBottom")]
    pub m_position_bottom: crate::app::interpolatorvector3::InterpolatorVector3,
    #[rename(name = "m_CursorMind")]
    pub m_cursor_mind: crate::app::mapmind::MapMind_Type,
    #[rename(name = "m_Color")]
    pub m_color: crate::unity_engine::color::Color,
    #[rename(name = "m_Flags")]
    pub m_flags: crate::app::mapcursor::MapCursor_FlagField,
}

#[cfg(feature = "app-mapcursor")]
#[::unity2::methods]
impl MapCursor {
    #[method(name = "GetCursorMind", args = 0)]
    pub fn get_cursor_mind() -> crate::app::mapmind::MapMind_Type;

    #[method(name = "SetCursorMind", args = 1)]
    pub fn set_cursor_mind(mind: crate::app::mapmind::MapMind_Type) -> ();

    #[method(name = "GetCameraTilt", args = 1)]
    pub fn get_camera_tilt(mode: crate::app::mapcursor::MapCursor_DistanceMode) -> f32;

    #[method(name = "GetCameraTiltClamp", args = 1)]
    pub fn get_camera_tilt_clamp(mode: crate::app::mapcursor::MapCursor_DistanceMode) -> f32;

    #[method(name = "GetCameraTilt", args = 1)]
    pub fn get_camera_tilt_2(ratio: f32) -> f32;

    #[method(name = "GetCameraTiltMin", args = 0)]
    pub fn get_camera_tilt_min() -> f32;

    #[method(name = "GetCameraTiltMax", args = 0)]
    pub fn get_camera_tilt_max() -> f32;

    #[method(name = "GetCameraDistance", args = 1)]
    pub fn get_camera_distance(mode: crate::app::mapcursor::MapCursor_DistanceMode) -> f32;

    #[method(name = "GetCameraTiltClamp", args = 2)]
    pub fn get_camera_tilt_clamp_2(value: f32, is_margin: bool) -> f32;

    #[method(name = "GetCameraTiltRate", args = 1)]
    pub fn get_camera_tilt_rate(tilt: f32) -> f32;

    #[method(name = "GetCameraDistanceMode", args = 1)]
    pub fn get_camera_distance_mode(tilt: f32) -> crate::app::mapcursor::MapCursor_DistanceMode;

    #[method(name = "GetCameraDistance", args = 0)]
    pub fn get_camera_distance_2(self) -> f32;

    #[method(name = "GetCameraRotate", args = 0)]
    pub fn get_camera_rotate(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetBackMode", args = 1)]
    pub fn get_back_mode(
        self,
        mode: crate::app::mapcursor::MapCursor_DistanceMode,
    ) -> crate::app::mapcursor::MapCursor_DistanceMode;

    #[method(name = "GetZoomMode", args = 1)]
    pub fn get_zoom_mode(
        self,
        mode: crate::app::mapcursor::MapCursor_DistanceMode,
    ) -> crate::app::mapcursor::MapCursor_DistanceMode;

    #[method(name = "GetNextMode", args = 2)]
    pub fn get_next_mode(
        self,
        mode: crate::app::mapcursor::MapCursor_DistanceMode,
        dir: i32,
    ) -> crate::app::mapcursor::MapCursor_DistanceMode;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "OnTick", args = 0)]
    pub fn on_tick(self) -> ();

    #[method(name = "OnUpdate", args = 0)]
    pub fn on_update(self) -> ();

    #[method(name = "ToggleCursorMoveSize", args = 0)]
    pub fn toggle_cursor_move_size(self) -> ();

    #[method(name = "CalcCursorTopPos", args = 0)]
    pub fn calc_cursor_top_pos(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "CalcCursorBottomPos", args = 0)]
    pub fn calc_cursor_bottom_pos(self) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "IsChangeCursorTop", args = 0)]
    pub fn is_change_cursor_top() -> bool;

    #[method(name = "IsChangeCursorBottom", args = 0)]
    pub fn is_change_cursor_bottom() -> bool;

    #[method(name = "GetCursorTime", args = 2)]
    pub fn get_cursor_time(
        self,
        prev: crate::unity_engine::vector3::Vector3,
        next: crate::unity_engine::vector3::Vector3,
    ) -> f32;

    #[method(name = "GetCursorSpeed", args = 0)]
    pub fn get_cursor_speed(self) -> f32;

    #[method(name = "GetMovePos", args = 3)]
    pub fn get_move_pos(self, prev: f32, next: f32, speed: f32) -> f32;

    #[method(name = "GetMovePos", args = 3)]
    pub fn get_move_pos_2(
        self,
        prev: crate::unity_engine::vector3::Vector3,
        next: crate::unity_engine::vector3::Vector3,
        speed: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetDigitalMove", args = 1)]
    pub fn get_digital_move(self, button: crate::nn::hid::npadbutton::NpadButton) -> f32;

    #[method(name = "SetCursorTop", args = 1)]
    pub fn set_cursor_top(mind: crate::app::mapmind::MapMind_Type) -> ();

    #[method(name = "ApplyCursorTop", args = 1)]
    pub fn apply_cursor_top(new_cursor_top: crate::app::mapcursor::MapCursor_CursorTopType) -> ();

    #[method(name = "GetMapPointerTransform", args = 1)]
    pub fn get_map_pointer_transform(
        r#type: crate::app::mapcursor::MapCursor_CursorTopType,
    ) -> crate::unity_engine::transform::Transform;

    #[method(name = "SetActive", args = 2)]
    pub fn set_active(game_object: crate::unity_engine::gameobject::GameObject, value: bool) -> ();

    #[method(name = "PlayCursorAnim", args = 0)]
    pub fn play_cursor_anim() -> ();

    #[method(name = "PlayCursorTopAnim", args = 0)]
    pub fn play_cursor_top_anim() -> ();

    #[method(name = "PlayCursorBottomAnim", args = 0)]
    pub fn play_cursor_bottom_anim() -> ();

    #[method(name = "TryBorderSound", args = 2)]
    pub fn try_border_sound(
        self,
        old: crate::unity_engine::vector3::Vector3,
        pos: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "CalcDigitalMove", args = 0)]
    pub fn calc_digital_move(self) -> ();

    #[method(name = "CalcAnalogMove", args = 0)]
    pub fn calc_analog_move(self) -> ();

    #[method(name = "UpdateEnterPos", args = 0)]
    pub fn update_enter_pos(self) -> ();

    #[method(name = "ResetEnterPos", args = 0)]
    pub fn reset_enter_pos(self) -> ();

    #[method(name = "AdjustPosition", args = 0)]
    pub fn adjust_position(self) -> ();

    #[method(name = "TickInputReset", args = 0)]
    pub fn tick_input_reset(self) -> ();

    #[method(name = "TickInputMove", args = 1)]
    pub fn tick_input_move(self, is_trigger: bool) -> ();

    #[method(name = "TryPlayCameraMoveSe", args = 0)]
    pub fn try_play_camera_move_se(self) -> ();

    #[method(name = "TryStopCameraMoveSe", args = 0)]
    pub fn try_stop_camera_move_se(self) -> ();

    #[method(name = "TickInputRotate", args = 0)]
    pub fn tick_input_rotate(self) -> ();

    #[method(name = "TickInputDanger", args = 0)]
    pub fn tick_input_danger(self) -> ();

    #[method(name = "CommitCamera", args = 0)]
    pub fn commit_camera(self) -> ();

    #[method(name = "CommitCameraPosition", args = 1)]
    pub fn commit_camera_position(self, speed: f32) -> ();

    #[method(name = "CommitCameraRotate", args = 0)]
    pub fn commit_camera_rotate(self) -> ();

    #[method(name = "UpdatePosition", args = 0)]
    pub fn update_position(self) -> ();

    #[method(name = "ClampPosition", args = 1)]
    pub fn clamp_position(self, pos: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "Setup", args = 0)]
    pub fn setup() -> ();

    #[method(name = "GetCursorUnit", args = 0)]
    pub fn get_cursor_unit() -> crate::app::unit::Unit;

    #[method(name = "GetX", args = 0)]
    pub fn get_x() -> i32;

    #[method(name = "GetZ", args = 0)]
    pub fn get_z() -> i32;

    #[method(name = "GetNoClampX", args = 0)]
    pub fn get_no_clamp_x() -> i32;

    #[method(name = "GetNoClampZ", args = 0)]
    pub fn get_no_clamp_z() -> i32;

    #[method(name = "Set", args = 4)]
    pub fn set(x: i32, z: i32, speed: f32, is_update_enter_pos: bool) -> ();

    #[method(name = "Set", args = 3)]
    pub fn set_2(unit: crate::app::unit::Unit, speed: f32, is_update_enter_pos: bool) -> ();

    #[method(name = "Set", args = 3)]
    pub fn set_3(
        pos: crate::unity_engine::vector3::Vector3,
        speed: f32,
        is_update_enter_pos: bool,
    ) -> ();

    #[method(name = "TrySet", args = 3)]
    pub fn try_set(
        pos: crate::unity_engine::vector3::Vector3,
        speed: f32,
        is_update_enter_pos: bool,
    ) -> bool;

    #[method(name = "Instant", args = 0)]
    pub fn instant() -> ();

    #[method(name = "SetColor", args = 1)]
    pub fn set_color(self, color: crate::unity_engine::color::Color) -> ();

    #[method(name = "SetColor", args = 0)]
    pub fn set_color_2(self) -> ();

    #[method(name = "get_Color", args = 0)]
    pub fn get_color(self) -> crate::unity_engine::color::Color;

    #[method(name = "Show", args = 0)]
    pub fn show() -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide() -> ();

    #[method(name = "IsVisible", args = 0)]
    pub fn is_visible() -> bool;

    #[method(name = "SetVisible", args = 1)]
    pub fn set_visible(enable: bool) -> ();

    #[method(name = "InstantImpl", args = 0)]
    pub fn instant_impl(self) -> ();

    #[method(name = "SetImpl", args = 3)]
    pub fn set_impl(
        self,
        pos: crate::unity_engine::vector3::Vector3,
        speed: f32,
        is_update_enter_pos: bool,
    ) -> ();

    #[method(name = "UpdateVisible", args = 0)]
    pub fn update_visible(self) -> ();

    #[method(name = "PlayCameraMoveSe", args = 0)]
    pub fn play_camera_move_se() -> ();

    #[method(name = "StopCameraMoveSe", args = 0)]
    pub fn stop_camera_move_se() -> ();

    #[method(name = "GetDistanceMode", args = 0)]
    pub fn get_distance_mode() -> crate::app::mapcursor::MapCursor_DistanceMode;

    #[method(name = "SetDistanceMode", args = 1)]
    pub fn set_distance_mode(mode: crate::app::mapcursor::MapCursor_DistanceMode) -> ();

    #[method(name = "SetDistanceScale", args = 1)]
    pub fn set_distance_scale(scale: f32) -> ();

    #[method(name = "GetDistanceRateForSound", args = 0)]
    pub fn get_distance_rate_for_sound() -> f32;

    #[method(name = "GetMapCursorMoveType", args = 0)]
    pub fn get_map_cursor_move_type() -> crate::app::gameconfig::GameConfig_MapCursorMoveTyep;

    #[method(name = "SetMapCursorMoveType", args = 1)]
    pub fn set_map_cursor_move_type(
        r#type: crate::app::gameconfig::GameConfig_MapCursorMoveTyep,
    ) -> ();

    #[method(name = "get_IsLockMoveType", args = 0)]
    pub fn get_is_lock_move_type() -> bool;

    #[method(name = "set_IsLockMoveType", args = 1)]
    pub fn set_is_lock_move_type(value: bool) -> ();

    #[method(name = "GetRotate", args = 0)]
    pub fn get_rotate() -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetPos", args = 0)]
    pub fn get_pos() -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GetEnterPos", args = 0)]
    pub fn get_enter_pos() -> crate::unity_engine::vector3::Vector3;

    #[method(name = "SetEnterPos", args = 1)]
    pub fn set_enter_pos(pos: crate::unity_engine::vector3::Vector3) -> ();

    #[method(name = "Tick", args = 0)]
    pub fn tick() -> ();

    #[method(name = "TickMove", args = 1)]
    pub fn tick_move(is_trigger: bool) -> ();

    #[method(name = "TickRotate", args = 0)]
    pub fn tick_rotate() -> ();

    #[method(name = "OnBind", args = 0)]
    pub fn on_bind(self) -> ();

    #[method(name = "OnUnbind", args = 0)]
    pub fn on_unbind(self) -> ();

    #[method(name = "get_Version", args = 0)]
    pub fn get_version(self) -> i32;

    #[method(name = "OnSerialize", args = 1)]
    pub fn on_serialize(self, stream: crate::app::stream_2::Stream_2) -> ();

    #[method(name = "OnDeserialize", args = 2)]
    pub fn on_deserialize(self, stream: crate::app::stream_2::Stream_2, version: i32) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mapcursor")]
impl MapCursor {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapCursor),
                ::core::stringify!(new),
            )
        });
        <Self as IMapCursorMethods>::ctor(this);
        this
    }
}
