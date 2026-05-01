
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::app::singletonprocinst_1::ISingletonProcInst_1;
use crate::app::singletonprocinst_1::SingletonProcInst_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/classchangesequence/ClassChangeSequence.md")))]
#[::unity2::class(namespace = "Combat", name = "ClassChangeSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: combat :: classchangesequence :: ClassChangeSequence >)]
pub struct ClassChangeSequence {
    #[static_field]
    #[rename(name = "SceneName")]
    pub scene_name: ::unity2::Il2CppString,
    #[rename(name = "m_Before")]
    pub m_before: crate::app::unit::Unit,
    #[rename(name = "m_After")]
    pub m_after: crate::app::unit::Unit,
    #[rename(name = "m_Status")]
    pub m_status: ::unity2::Array<crate::combat::charactergamestatus::CharacterGameStatus>,
    #[rename(name = "m_Characters")]
    pub m_characters: ::unity2::Array<crate::combat::character::Character>,
    #[rename(name = "m_ZoneHandle")]
    pub m_zone_handle: crate::app::resourcehandle_2::ResourceHandle_2,
    #[rename(name = "m_ChangeEffect")]
    pub m_change_effect: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_GameObjects")]
    pub m_game_objects: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
}

#[cfg(feature = "combat-classchangesequence")]
#[::unity2::methods]
impl ClassChangeSequence {
    #[method(name = "get_GlobalAssetPath", args = 0)]
    pub fn get_global_asset_path(self) -> ::unity2::Il2CppString;

    #[method(name = "IsExist", args = 0)]
    pub fn is_exist() -> bool;

    #[method(name = "CreateBind", args = 5)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        before: crate::app::unit::Unit,
        after: crate::app::unit::Unit,
        do_fade: bool,
        camera_mode: crate::app::viewmode::ViewMode_Mode,
    ) -> ();

    #[method(name = "get_ReturnMode", args = 0)]
    pub fn get_return_mode(self) -> crate::app::viewmode::ViewMode_Mode;

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        before: crate::app::unit::Unit,
        after: crate::app::unit::Unit,
        camera_mode: crate::app::viewmode::ViewMode_Mode,
    ) -> ();

    #[method(name = "Load", args = 0)]
    pub fn load(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "ChangeAnimation", args = 0)]
    pub fn change_animation(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "Change", args = 0)]
    pub fn change(self) -> ();

    #[method(name = "PoseAnimation", args = 0)]
    pub fn pose_animation(self) -> crate::system::collections::ienumerator::IEnumerator;

    #[method(name = "Telop", args = 0)]
    pub fn telop(self) -> ();

    #[method(name = "DisplayParams", args = 0)]
    pub fn display_params(self) -> ();

    #[method(name = "Exit", args = 0)]
    pub fn exit(self) -> ();

    #[method(name = "IsCharacterLoading", args = 0)]
    pub fn is_character_loading(self) -> bool;

    #[method(name = "ExitAfter", args = 0)]
    pub fn exit_after(self) -> ();

    #[method(name = "StartBeforeBGM", args = 0)]
    pub fn start_before_bgm(self) -> ();

    #[method(name = "StartSkipBGM", args = 0)]
    pub fn start_skip_bgm(self) -> ();

    #[method(name = "StartAfterBGM", args = 0)]
    pub fn start_after_bgm(self) -> ();

    #[method(name = "ResumeBGM", args = 0)]
    pub fn resume_bgm(self) -> ();

    #[method(name = "StartSE", args = 0)]
    pub fn start_se(self) -> ();

    #[method(name = "StopSE", args = 0)]
    pub fn stop_se(self) -> ();

    #[method(name = "SetupGameStatus", args = 0)]
    pub fn setup_game_status(self) -> ();

    #[method(name = "CreateCharacter", args = 4)]
    pub fn create_character(
        self,
        side: i32,
        preload_anim: crate::combat::preloadanims::PreloadAnims,
        locator: crate::combat::basecombatlocation::BaseCombatLocation,
        visible: bool,
    ) -> crate::system::collections::ienumerator::IEnumerator;
}

#[cfg(feature = "combat-classchangesequence")]
impl ClassChangeSequence {
    pub fn new(
        before: crate::app::unit::Unit,
        after: crate::app::unit::Unit,
        camera_mode: crate::app::viewmode::ViewMode_Mode,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ClassChangeSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IClassChangeSequenceMethods>::ctor(this, before, after, camera_mode);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/classchangesequence/ClassChangeSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ClassChangeSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ClassChangeSequence_Label {
    const NAMESPACE: &'static str = "Combat";

    const NAME: &'static str = "ClassChangeSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ClassChangeSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ClassChangeSequence_Label {
    pub fn skip_fade_in() -> Self {
        Self { value: 0 }
    }

    pub fn skip_fade_out() -> Self {
        Self { value: 1 }
    }
}
