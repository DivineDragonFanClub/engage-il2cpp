
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/muscleexercisesequence/MuscleExerciseSequence.md")))]
#[::unity2::class(namespace = "App", name = "MuscleExerciseSequence")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MuscleExerciseSequence {
    #[rename(name = "m_RootPrefab")]
    pub m_root_prefab: crate::app::muscleexerciseprefab::MuscleExercisePrefab,
}

#[cfg(feature = "app-muscleexercisesequence")]
#[::unity2::methods]
impl MuscleExerciseSequence {
    #[method(name = "get_SelectType", args = 0)]
    pub fn get_select_type(self) -> i32;

    #[method(name = "set_SelectType", args = 1)]
    pub fn set_select_type(self, value: i32) -> ();

    #[method(name = "get_SelectLevel", args = 0)]
    pub fn get_select_level(self) -> i32;

    #[method(name = "set_SelectLevel", args = 1)]
    pub fn set_select_level(self, value: i32) -> ();

    #[method(name = "get_IsAssist", args = 0)]
    pub fn get_is_assist(self) -> bool;

    #[method(name = "set_IsAssist", args = 1)]
    pub fn set_is_assist(self, value: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources(self) -> ();

    #[method(name = "IsLoading", args = 0)]
    pub fn is_loading(self) -> bool;

    #[method(name = "Setup", args = 0)]
    pub fn setup(self) -> ();

    #[method(name = "CheckJumpSquat", args = 0)]
    pub fn check_jump_squat(self) -> bool;

    #[method(name = "CheckJumpPushUp", args = 0)]
    pub fn check_jump_push_up(self) -> bool;

    #[method(name = "CheckJumpSitUp", args = 0)]
    pub fn check_jump_sit_up(self) -> bool;

    #[method(name = "Tick", args = 0)]
    pub fn tick(self) -> ();

    #[method(name = "CreatePushUpSequence", args = 0)]
    pub fn create_push_up_sequence(self) -> ();

    #[method(name = "CreateSitUpSequence", args = 0)]
    pub fn create_sit_up_sequence(self) -> ();

    #[method(name = "CreateSquatSequence", args = 0)]
    pub fn create_squat_sequence(self) -> ();

    #[method(name = "IncreasePlayCounter", args = 0)]
    pub fn increase_play_counter(self) -> ();

    #[method(name = "Final", args = 0)]
    pub fn r#final(self) -> ();

    #[method(name = "UnloadResources", args = 0)]
    pub fn unload_resources(self) -> ();

    #[method(name = "CreateBind", args = 4)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        r#type: i32,
        level: i32,
        is_assist: bool,
    ) -> ();

    #[method(name = "RegistFlag", args = 0)]
    pub fn regist_flag() -> ();
}

#[cfg(feature = "app-muscleexercisesequence")]
impl MuscleExerciseSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MuscleExerciseSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IMuscleExerciseSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/muscleexercisesequence/MuscleExerciseSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MuscleExerciseSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MuscleExerciseSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MuscleExerciseSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MuscleExerciseSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MuscleExerciseSequence_Label {
    pub fn init() -> Self {
        Self { value: 0 }
    }

    pub fn tick() -> Self {
        Self { value: 1 }
    }

    pub fn execute_push_up() -> Self {
        Self { value: 2 }
    }

    pub fn execute_sit_up() -> Self {
        Self { value: 3 }
    }

    pub fn execute_squat() -> Self {
        Self { value: 4 }
    }

    pub fn r#final() -> Self {
        Self { value: 5 }
    }
}
