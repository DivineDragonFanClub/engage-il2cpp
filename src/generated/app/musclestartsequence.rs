
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/musclestartsequence/MuscleStartSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MuscleStartSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MuscleStartSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MuscleStartSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MuscleStartSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MuscleStartSequence_Label {
    pub fn initialize() -> Self {
        Self { value: 0 }
    }

    pub fn select_type() -> Self {
        Self { value: 1 }
    }

    pub fn check_new_difficult() -> Self {
        Self { value: 2 }
    }

    pub fn annouce_new_difficult() -> Self {
        Self { value: 3 }
    }

    pub fn select_difficult() -> Self {
        Self { value: 4 }
    }

    pub fn select_assist() -> Self {
        Self { value: 5 }
    }

    pub fn create_game_sequence() -> Self {
        Self { value: 6 }
    }

    pub fn continue_check() -> Self {
        Self { value: 7 }
    }

    pub fn play_repeat() -> Self {
        Self { value: 8 }
    }

    pub fn exit() -> Self {
        Self { value: 9 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/musclestartsequence/MuscleStartSequence.md")))]
#[::unity2::class(namespace = "App", name = "MuscleStartSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MuscleStartSequence {
    #[rename(name = "m_SelectedType")]
    pub m_selected_type: i32,
    #[rename(name = "m_SelectedLevel")]
    pub m_selected_level: i32,
    #[rename(name = "m_AssistDeside")]
    pub m_assist_deside: i32,
    #[rename(name = "m_IsCreateGameSequence")]
    pub m_is_create_game_sequence: bool,
    #[rename(name = "m_IsCotinueExercise")]
    pub m_is_cotinue_exercise: bool,
    #[rename(name = "m_TalkerChara")]
    pub m_talker_chara: crate::combat::character::Character,
    #[rename(name = "m_AnnouceDifficult")]
    pub m_annouce_difficult:
        crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    #[rename(name = "m_IsAnnounceVoiceOnce")]
    pub m_is_announce_voice_once: bool,
}

#[cfg(feature = "app-musclestartsequence")]
#[::unity2::methods]
impl MuscleStartSequence {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources(self) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "OpenTitleBar", args = 0)]
    pub fn open_title_bar(self) -> ();

    #[method(name = "CloseTitleBar", args = 0)]
    pub fn close_title_bar(self) -> ();

    #[method(name = "OpenExerciseTypeMenu", args = 0)]
    pub fn open_exercise_type_menu(self) -> ();

    #[method(name = "Tutorial", args = 0)]
    pub fn tutorial(self) -> ();

    #[method(name = "CheckNewDifficult", args = 0)]
    pub fn check_new_difficult(self) -> ();

    #[method(name = "AnnounceNewDifficult", args = 0)]
    pub fn announce_new_difficult(self) -> ();

    #[method(name = "CheckAnnounceDifficult", args = 0)]
    pub fn check_announce_difficult(self) -> bool;

    #[method(name = "OpenDifficultMenu", args = 0)]
    pub fn open_difficult_menu(self) -> ();

    #[method(name = "CreateAssistDialog", args = 0)]
    pub fn create_assist_dialog(self) -> ();

    #[method(name = "CreateGameSequence", args = 0)]
    pub fn create_game_sequence(self) -> ();

    #[method(name = "CreateContinueDialog", args = 0)]
    pub fn create_continue_dialog(self) -> ();

    #[method(name = "CheckContinue", args = 0)]
    pub fn check_continue(self) -> bool;

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = "UnloadResources", args = 0)]
    pub fn unload_resources(self) -> ();

    #[method(name = "VoiceSelectType", args = 0)]
    pub fn voice_select_type(self) -> ();

    #[method(name = "VoiceNewDifficult", args = 0)]
    pub fn voice_new_difficult(self) -> ();

    #[method(name = "VoiceSelectDifficult", args = 0)]
    pub fn voice_select_difficult(self) -> ();

    #[method(name = "VoiceFinishExercise", args = 0)]
    pub fn voice_finish_exercise(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();
}

#[cfg(feature = "app-musclestartsequence")]
impl MuscleStartSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MuscleStartSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMuscleStartSequenceMethods>::ctor(this);
        this
    }
}
