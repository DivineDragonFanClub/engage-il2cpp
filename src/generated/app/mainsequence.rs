
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::procscenesequence_1::IProcSceneSequence_1;
use crate::app::procscenesequence_1::ProcSceneSequence_1;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainsequence/MainSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MainSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MainSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MainSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MainSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MainSequence_Label {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn startup() -> Self {
        Self { value: 1 }
    }

    pub fn title_loop() -> Self {
        Self { value: 2 }
    }

    pub fn title_loop_from_main_menu() -> Self {
        Self { value: 3 }
    }

    pub fn main_menu() -> Self {
        Self { value: 4 }
    }

    pub fn chapter() -> Self {
        Self { value: 5 }
    }

    pub fn gmap() -> Self {
        Self { value: 6 }
    }

    pub fn kizuna() -> Self {
        Self { value: 7 }
    }

    pub fn hub() -> Self {
        Self { value: 8 }
    }

    pub fn hub_to_save_position() -> Self {
        Self { value: 9 }
    }

    pub fn ending() -> Self {
        Self { value: 10 }
    }

    pub fn next_chapter() -> Self {
        Self { value: 11 }
    }

    pub fn map() -> Self {
        Self { value: 12 }
    }

    pub fn complete() -> Self {
        Self { value: 13 }
    }

    pub fn game_over() -> Self {
        Self { value: 14 }
    }

    pub fn chapter_save() -> Self {
        Self { value: 15 }
    }

    pub fn after_chapter_save() -> Self {
        Self { value: 16 }
    }

    pub fn set_save_data_load_target() -> Self {
        Self { value: 17 }
    }

    pub fn save_data_load() -> Self {
        Self { value: 18 }
    }

    pub fn save_data_load_failed() -> Self {
        Self { value: 19 }
    }

    pub fn save_data_version_failed() -> Self {
        Self { value: 20 }
    }

    pub fn data_load_failed() -> Self {
        Self { value: 21 }
    }

    pub fn after_load_failed() -> Self {
        Self { value: 22 }
    }

    pub fn contents_resume() -> Self {
        Self { value: 23 }
    }

    pub fn relay_debug() -> Self {
        Self { value: 24 }
    }

    pub fn relay() -> Self {
        Self { value: 25 }
    }

    pub fn versus() -> Self {
        Self { value: 26 }
    }

    pub fn challenge() -> Self {
        Self { value: 27 }
    }

    pub fn back_to_title() -> Self {
        Self { value: 28 }
    }

    pub fn end() -> Self {
        Self { value: 29 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mainsequence/MainSequence.md")))]
#[::unity2::class(namespace = "App", name = "MainSequence")]
# [parent (crate :: app :: procscenesequence_1 :: ProcSceneSequence_1 < crate :: app :: mainsequence :: MainSequence >)]
pub struct MainSequence {
    #[static_field]
    #[rename(name = "s_JumpLabel")]
    pub s_jump_label: crate::app::mainsequence::MainSequence_Label,
    #[static_field]
    #[rename(name = "s_FakeLabel")]
    pub s_fake_label: crate::app::mainsequence::MainSequence_Label,
    #[static_field]
    #[rename(name = "s_Initialized")]
    pub s_initialized: bool,
}

#[cfg(feature = "app-mainsequence")]
#[::unity2::methods]
impl MainSequence {
    #[method(name = "JumpToLabel", args = 0)]
    pub fn jump_to_label(self) -> ();

    #[method(name = "JumpToLabelNoGCCollect", args = 0)]
    pub fn jump_to_label_no_gc_collect(self) -> ();

    #[method(name = "JumpToLabelImpl", args = 1)]
    pub fn jump_to_label_impl(self, is_gc_collect: bool) -> ();

    #[method(name = "GetCurrent", args = 1)]
    pub fn get_current(super_: crate::app::procinst::ProcInst) -> crate::app::procinst::ProcInst;

    #[method(name = "GetCurrent", args = 0)]
    pub fn get_current_2() -> crate::app::procinst::ProcInst;

    #[method(name = "SetJumpSeq", args = 1)]
    pub fn set_jump_seq(label: crate::app::mainsequence::MainSequence_Label) -> ();

    #[method(name = "Initialize", args = 0)]
    pub fn initialize(self) -> ();

    #[method(name = "PostInitialize", args = 0)]
    pub fn post_initialize(self) -> ();

    #[method(name = "LoadPublic", args = 0)]
    pub fn load_public(self) -> ();

    #[method(name = "LoadResource", args = 0)]
    pub fn load_resource(self) -> ();

    #[method(name = "PostLoadResource", args = 0)]
    pub fn post_load_resource(self) -> ();

    #[method(name = "BeginSilentVolume", args = 0)]
    pub fn begin_silent_volume(self) -> ();

    #[method(name = "EndSilentVolume", args = 0)]
    pub fn end_silent_volume(self) -> ();

    #[method(name = "OnPersistent", args = 0)]
    pub fn on_persistent(self) -> ();

    #[method(name = "BranchStart", args = 0)]
    pub fn branch_start(self) -> ();

    #[method(name = "BranchChapterStart", args = 0)]
    pub fn branch_chapter_start(self) -> ();

    #[method(name = "LoadChapterBank", args = 0)]
    pub fn load_chapter_bank(self) -> ();

    #[method(name = "TryJumpToKizuna", args = 0)]
    pub fn try_jump_to_kizuna(self) -> ();

    #[method(name = "TryJumpToContinueMap", args = 0)]
    pub fn try_jump_to_continue_map(self) -> ();

    #[method(name = "TryJumpToHub", args = 0)]
    pub fn try_jump_to_hub(self) -> ();

    #[method(name = "TryJumpToGmap", args = 0)]
    pub fn try_jump_to_gmap(self) -> ();

    #[method(name = "TryJumpToNextChapter", args = 0)]
    pub fn try_jump_to_next_chapter(self) -> ();

    #[method(name = "HubToSavePosition", args = 0)]
    pub fn hub_to_save_position(self) -> ();

    #[method(name = "GameReset", args = 0)]
    pub fn game_reset(self) -> ();

    #[method(name = "AutoSave", args = 0)]
    pub fn auto_save(self) -> ();

    #[method(name = "SetSaveDataLoadTarget", args = 0)]
    pub fn set_save_data_load_target(self) -> ();

    #[method(name = "SaveDataLoad", args = 0)]
    pub fn save_data_load(self) -> ();

    #[method(name = "SaveDataLoadResult", args = 0)]
    pub fn save_data_load_result(self) -> ();

    #[method(name = "SaveDataRelease", args = 0)]
    pub fn save_data_release(self) -> ();

    #[method(name = "SaveDataNormalize", args = 0)]
    pub fn save_data_normalize(self) -> ();

    #[method(name = "SaveDataBranchFirst", args = 0)]
    pub fn save_data_branch_first(self) -> ();

    #[method(name = "SaveDataBranchSecond", args = 0)]
    pub fn save_data_branch_second(self) -> ();

    #[method(name = "SaveDataLoadFailed", args = 0)]
    pub fn save_data_load_failed(self) -> ();

    #[method(name = "SaveDataVersionFailed", args = 0)]
    pub fn save_data_version_failed(self) -> ();

    #[method(name = "DataLoadFailed", args = 0)]
    pub fn data_load_failed(self) -> ();

    #[method(name = "DeleteTemporary", args = 0)]
    pub fn delete_temporary(self) -> ();

    #[method(name = "GameSoundReset", args = 0)]
    pub fn game_sound_reset(self) -> ();

    #[method(name = "DumpMemory", args = 1)]
    pub fn dump_memory(self, label: crate::app::mainsequence::MainSequence_Label) -> ();

    #[method(name = "DumpMemory", args = 1)]
    pub fn dump_memory_2(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "LoadLogo", args = 0)]
    pub fn load_logo(self) -> ();

    #[method(name = "ShowLogo", args = 0)]
    pub fn show_logo(self) -> ();

    #[method(name = "ShowIcon", args = 0)]
    pub fn show_icon(self) -> ();

    #[method(name = "Create", args = 0)]
    pub fn create() -> ();

    #[method(name = "IsInitialized", args = 0)]
    pub fn is_initialized() -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-mainsequence")]
impl MainSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MainSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMainSequenceMethods>::ctor(this);
        this
    }
}
