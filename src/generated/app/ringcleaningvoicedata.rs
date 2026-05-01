
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringcleaningvoicedata/RingCleaningVoiceData_Situation.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RingCleaningVoiceData_Situation {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RingCleaningVoiceData_Situation {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RingCleaningVoiceData.Situation";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RingCleaningVoiceData_Situation {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RingCleaningVoiceData_Situation {
    pub fn start() -> Self {
        Self { value: 0 }
    }

    pub fn clean_strongly_not_dirty() -> Self {
        Self { value: 1 }
    }

    pub fn clean_dirty() -> Self {
        Self { value: 2 }
    }

    pub fn clean_strongly_diry() -> Self {
        Self { value: 3 }
    }

    pub fn finish() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringcleaningvoicedata/RingCleaningVoiceData_VoiceLabel.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct RingCleaningVoiceData_VoiceLabel {
    pub value: i32,
}

impl ::unity2::ClassIdentity for RingCleaningVoiceData_VoiceLabel {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "RingCleaningVoiceData.VoiceLabel";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for RingCleaningVoiceData_VoiceLabel {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl RingCleaningVoiceData_VoiceLabel {
    pub fn touch01() -> Self {
        Self { value: 0 }
    }

    pub fn touch02() -> Self {
        Self { value: 1 }
    }

    pub fn touch03() -> Self {
        Self { value: 2 }
    }

    pub fn touch04() -> Self {
        Self { value: 3 }
    }

    pub fn touch05() -> Self {
        Self { value: 4 }
    }

    pub fn touch06() -> Self {
        Self { value: 5 }
    }

    pub fn touch07() -> Self {
        Self { value: 6 }
    }

    pub fn touch08() -> Self {
        Self { value: 7 }
    }

    pub fn touch09() -> Self {
        Self { value: 8 }
    }

    pub fn touch10() -> Self {
        Self { value: 9 }
    }

    pub fn dirt01() -> Self {
        Self { value: 10 }
    }

    pub fn dirt02() -> Self {
        Self { value: 11 }
    }

    pub fn dirt03() -> Self {
        Self { value: 12 }
    }

    pub fn thank01() -> Self {
        Self { value: 13 }
    }

    pub fn thank02() -> Self {
        Self { value: 14 }
    }

    pub fn thank03() -> Self {
        Self { value: 15 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/ringcleaningvoicedata/RingCleaningVoiceData.md")))]
#[::unity2::class(namespace = "App", name = "RingCleaningVoiceData")]
# [parent (crate :: app :: structdataarray_1 :: StructDataArray_1 < crate :: app :: ringcleaningvoicedata :: RingCleaningVoiceData >)]
pub struct RingCleaningVoiceData {
    #[static_field]
    #[rename(name = "EventNames")]
    pub event_names: ::unity2::Array<::unity2::Il2CppString>,
}

#[cfg(feature = "app-ringcleaningvoicedata")]
#[::unity2::methods]
impl RingCleaningVoiceData {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Label", args = 0)]
    pub fn get_label(self) -> crate::app::ringcleaningvoicedata::RingCleaningVoiceData_VoiceLabel;

    #[method(name = "set_Label", args = 1)]
    pub fn set_label(
        self,
        value: crate::app::ringcleaningvoicedata::RingCleaningVoiceData_VoiceLabel,
    ) -> ();

    #[method(name = "get_PlaySituation", args = 0)]
    pub fn get_play_situation(
        self,
    ) -> crate::app::ringcleaningvoicedata::RingCleaningVoiceData_Situation;

    #[method(name = "set_PlaySituation", args = 1)]
    pub fn set_play_situation(
        self,
        value: crate::app::ringcleaningvoicedata::RingCleaningVoiceData_Situation,
    ) -> ();

    #[method(name = "get_IsPlayCompleted", args = 0)]
    pub fn get_is_play_completed(self) -> bool;

    #[method(name = "set_IsPlayCompleted", args = 1)]
    pub fn set_is_play_completed(self, value: bool) -> ();

    #[method(name = "get_UnitFaceAnim", args = 0)]
    pub fn get_unit_face_anim(self) -> ::unity2::Il2CppString;

    #[method(name = "set_UnitFaceAnim", args = 1)]
    pub fn set_unit_face_anim(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_GodFaceAnim", args = 0)]
    pub fn get_god_face_anim(self) -> ::unity2::Il2CppString;

    #[method(name = "set_GodFaceAnim", args = 1)]
    pub fn set_god_face_anim(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetEventName", args = 0)]
    pub fn get_event_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetRandomVoiceEvent", args = 3)]
    pub fn get_random_voice_event(
        god: crate::app::godunit::GodUnit,
        situation: crate::app::ringcleaningvoicedata::RingCleaningVoiceData_Situation,
        is_finish: bool,
    ) -> crate::app::ringcleaningvoicedata::RingCleaningVoiceData;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-ringcleaningvoicedata")]
impl RingCleaningVoiceData {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(RingCleaningVoiceData),
                ::core::stringify!(new),
            )
        });
        <Self as IRingCleaningVoiceDataMethods>::ctor(this);
        this
    }
}
