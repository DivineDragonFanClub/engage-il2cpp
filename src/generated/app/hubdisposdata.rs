
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdataarray_1::IStructDataArray_1;
use crate::app::structdataarray_1::StructDataArray_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubdisposdata/HubDisposData_IdleTypes.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct HubDisposData_IdleTypes {
    pub value: i32,
}

impl ::unity2::ClassIdentity for HubDisposData_IdleTypes {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubDisposData.IdleTypes";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubDisposData_IdleTypes {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl HubDisposData_IdleTypes {
    pub fn default() -> Self {
        Self { value: 0 }
    }

    pub fn only_neck_aim() -> Self {
        Self { value: 1 }
    }

    pub fn no_turn() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubdisposdata/HubDisposData_AccessTypes.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct HubDisposData_AccessTypes {
    pub value: i32,
}

impl ::unity2::ClassIdentity for HubDisposData_AccessTypes {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubDisposData.AccessTypes";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubDisposData_AccessTypes {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl HubDisposData_AccessTypes {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn menu() -> Self {
        Self { value: 1 }
    }

    pub fn door() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubdisposdata/HubDisposData_DisposTypes.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct HubDisposData_DisposTypes {
    pub value: i32,
}

impl ::unity2::ClassIdentity for HubDisposData_DisposTypes {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubDisposData.DisposTypes";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubDisposData_DisposTypes {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl HubDisposData_DisposTypes {
    pub fn none_ik() -> Self {
        Self { value: 0 }
    }

    pub fn use_ik() -> Self {
        Self { value: 1 }
    }

    pub fn swiming() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubdisposdata/HubDisposData_TimezoneFlags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct HubDisposData_TimezoneFlags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for HubDisposData_TimezoneFlags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubDisposData.TimezoneFlags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubDisposData_TimezoneFlags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl HubDisposData_TimezoneFlags {
    pub fn morning() -> Self {
        Self { value: 1 }
    }

    pub fn day() -> Self {
        Self { value: 2 }
    }

    pub fn evening() -> Self {
        Self { value: 4 }
    }

    pub fn night() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubdisposdata/HubDisposData.md")))]
#[::unity2::class(namespace = "App", name = "HubDisposData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: hubdisposdata :: HubDisposData >)]
pub struct HubDisposData {}

#[cfg(feature = "app-hubdisposdata")]
#[::unity2::methods]
impl HubDisposData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Locator", args = 0)]
    pub fn get_locator(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Locator", args = 1)]
    pub fn set_locator(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ParentLocator", args = 0)]
    pub fn get_parent_locator(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ParentLocator", args = 1)]
    pub fn set_parent_locator(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IsMustChild", args = 0)]
    pub fn get_is_must_child(self) -> bool;

    #[method(name = "set_IsMustChild", args = 1)]
    pub fn set_is_must_child(self, value: bool) -> ();

    #[method(name = "get_FadeDistance", args = 0)]
    pub fn get_fade_distance(self) -> f32;

    #[method(name = "set_FadeDistance", args = 1)]
    pub fn set_fade_distance(self, value: f32) -> ();

    #[method(name = "get_Priority", args = 0)]
    pub fn get_priority(self) -> i32;

    #[method(name = "set_Priority", args = 1)]
    pub fn set_priority(self, value: i32) -> ();

    #[method(name = "get_Chapter", args = 0)]
    pub fn get_chapter(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Chapter", args = 1)]
    pub fn set_chapter(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Phase", args = 0)]
    pub fn get_phase(self) -> crate::app::hubdisposdata::HubDisposData_PhaseType;

    #[method(name = "set_Phase", args = 1)]
    pub fn set_phase(self, value: crate::app::hubdisposdata::HubDisposData_PhaseType) -> ();

    #[method(name = "get_TimezoneFlag", args = 0)]
    pub fn get_timezone_flag(self) -> crate::app::hubdisposdata::HubDisposData_TimezoneFlags;

    #[method(name = "set_TimezoneFlag", args = 1)]
    pub fn set_timezone_flag(
        self,
        value: crate::app::hubdisposdata::HubDisposData_TimezoneFlags,
    ) -> ();

    #[method(name = "get_FlagName", args = 0)]
    pub fn get_flag_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_FlagName", args = 1)]
    pub fn set_flag_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AnyCondition", args = 0)]
    pub fn get_any_condition(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AnyCondition", args = 1)]
    pub fn set_any_condition(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ContentType", args = 0)]
    pub fn get_content_type(self) -> crate::app::hubdisposdata::HubDisposData_Type;

    #[method(name = "set_ContentType", args = 1)]
    pub fn set_content_type(self, value: crate::app::hubdisposdata::HubDisposData_Type) -> ();

    #[method(name = "get_AID", args = 0)]
    pub fn get_aid(self) -> ::unity2::Il2CppString;

    #[method(name = "set_AID", args = 1)]
    pub fn set_aid(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_TalkPattern", args = 0)]
    pub fn get_talk_pattern(self) -> ::unity2::Il2CppString;

    #[method(name = "set_TalkPattern", args = 1)]
    pub fn set_talk_pattern(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_HelpLabel", args = 0)]
    pub fn get_help_label(self) -> ::unity2::Il2CppString;

    #[method(name = "set_HelpLabel", args = 1)]
    pub fn set_help_label(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_MainLabel", args = 0)]
    pub fn get_main_label(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MainLabel", args = 1)]
    pub fn set_main_label(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ScriptName", args = 0)]
    pub fn get_script_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ScriptName", args = 1)]
    pub fn set_script_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_AccessType", args = 0)]
    pub fn get_access_type(self) -> crate::app::hubdisposdata::HubDisposData_AccessTypes;

    #[method(name = "set_AccessType", args = 1)]
    pub fn set_access_type(self, value: crate::app::hubdisposdata::HubDisposData_AccessTypes)
        -> ();

    #[method(name = "get_IdleBodyName", args = 0)]
    pub fn get_idle_body_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_IdleBodyName", args = 1)]
    pub fn set_idle_body_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IdleFaceName", args = 0)]
    pub fn get_idle_face_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_IdleFaceName", args = 1)]
    pub fn set_idle_face_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_IdleType", args = 0)]
    pub fn get_idle_type(self) -> crate::app::hubdisposdata::HubDisposData_IdleTypes;

    #[method(name = "set_IdleType", args = 1)]
    pub fn set_idle_type(self, value: crate::app::hubdisposdata::HubDisposData_IdleTypes) -> ();

    #[method(name = "get_DisabledAnim", args = 0)]
    pub fn get_disabled_anim(self) -> bool;

    #[method(name = "set_DisabledAnim", args = 1)]
    pub fn set_disabled_anim(self, value: bool) -> ();

    #[method(name = "get_DisabledTalk", args = 0)]
    pub fn get_disabled_talk(self) -> bool;

    #[method(name = "set_DisabledTalk", args = 1)]
    pub fn set_disabled_talk(self, value: bool) -> ();

    #[method(name = "get_IgnoreStory", args = 0)]
    pub fn get_ignore_story(self) -> bool;

    #[method(name = "set_IgnoreStory", args = 1)]
    pub fn set_ignore_story(self, value: bool) -> ();

    #[method(name = "get_Bind", args = 0)]
    pub fn get_bind(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Bind", args = 1)]
    pub fn set_bind(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_DisposType", args = 0)]
    pub fn get_dispos_type(self) -> crate::app::hubdisposdata::HubDisposData_DisposTypes;

    #[method(name = "set_DisposType", args = 1)]
    pub fn set_dispos_type(self, value: crate::app::hubdisposdata::HubDisposData_DisposTypes)
        -> ();

    #[method(name = "get_AccessAngle", args = 0)]
    pub fn get_access_angle(self) -> f32;

    #[method(name = "set_AccessAngle", args = 1)]
    pub fn set_access_angle(self, value: f32) -> ();

    #[method(name = "get_MoveName", args = 0)]
    pub fn get_move_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_MoveName", args = 1)]
    pub fn set_move_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Area", args = 0)]
    pub fn get_area(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Area", args = 1)]
    pub fn set_area(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Layer", args = 0)]
    pub fn get_layer(self) -> i32;

    #[method(name = "set_Layer", args = 1)]
    pub fn set_layer(self, value: i32) -> ();

    #[method(name = "get_DisabledMiniMap", args = 0)]
    pub fn get_disabled_mini_map(self) -> bool;

    #[method(name = "set_DisabledMiniMap", args = 1)]
    pub fn set_disabled_mini_map(self, value: bool) -> ();

    #[method(name = "get_Weight", args = 0)]
    pub fn get_weight(self) -> f32;

    #[method(name = "set_Weight", args = 1)]
    pub fn set_weight(self, value: f32) -> ();

    #[method(name = "get_OptimizeType", args = 0)]
    pub fn get_optimize_type(self) -> i32;

    #[method(name = "set_OptimizeType", args = 1)]
    pub fn set_optimize_type(self, value: i32) -> ();

    #[method(name = "IsChair", args = 0)]
    pub fn is_chair(self) -> bool;

    #[method(name = "IsPick", args = 0)]
    pub fn is_pick(self) -> bool;

    #[method(name = "IsSquat", args = 0)]
    pub fn is_squat(self) -> bool;

    #[method(name = "IsSitUp", args = 0)]
    pub fn is_sit_up(self) -> bool;

    #[method(name = "IsSit", args = 0)]
    pub fn is_sit(self) -> bool;

    #[method(name = "IsSing", args = 0)]
    pub fn is_sing(self) -> bool;

    #[method(name = "IsFish", args = 0)]
    pub fn is_fish(self) -> bool;

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-hubdisposdata")]
impl HubDisposData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(HubDisposData),
                ::core::stringify!(new),
            )
        });
        <Self as IHubDisposDataMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubdisposdata/HubDisposData_Type.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct HubDisposData_Type {
    pub value: i32,
}

impl ::unity2::ClassIdentity for HubDisposData_Type {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubDisposData.Type";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubDisposData_Type {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl HubDisposData_Type {
    pub fn r#move() -> Self {
        Self { value: 0 }
    }

    pub fn event() -> Self {
        Self { value: 1 }
    }

    pub fn priority_character() -> Self {
        Self { value: 2 }
    }

    pub fn random_character() -> Self {
        Self { value: 3 }
    }

    pub fn god() -> Self {
        Self { value: 4 }
    }

    pub fn normal_character() -> Self {
        Self { value: 5 }
    }

    pub fn item() -> Self {
        Self { value: 6 }
    }

    pub fn animal() -> Self {
        Self { value: 7 }
    }

    pub fn material() -> Self {
        Self { value: 8 }
    }

    pub fn animal_item() -> Self {
        Self { value: 9 }
    }

    pub fn num() -> Self {
        Self { value: 10 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/hubdisposdata/HubDisposData_PhaseType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct HubDisposData_PhaseType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for HubDisposData_PhaseType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "HubDisposData.PhaseType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for HubDisposData_PhaseType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl HubDisposData_PhaseType {
    pub fn any() -> Self {
        Self { value: 0 }
    }

    pub fn only() -> Self {
        Self { value: 1 }
    }

    pub fn on_and_after() -> Self {
        Self { value: 2 }
    }
}
