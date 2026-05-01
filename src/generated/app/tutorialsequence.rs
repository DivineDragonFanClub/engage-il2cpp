
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tutorialsequence/TutorialSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TutorialSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TutorialSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TutorialSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TutorialSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TutorialSequence_Label {
    pub fn load() -> Self {
        Self { value: 0 }
    }

    pub fn unload() -> Self {
        Self { value: 1 }
    }

    pub fn end() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tutorialsequence/TutorialSequence_LanguageType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct TutorialSequence_LanguageType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for TutorialSequence_LanguageType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "TutorialSequence.LanguageType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for TutorialSequence_LanguageType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl TutorialSequence_LanguageType {
    pub fn japanese() -> Self {
        Self { value: 0 }
    }

    pub fn english() -> Self {
        Self { value: 1 }
    }

    pub fn french() -> Self {
        Self { value: 2 }
    }

    pub fn spanish() -> Self {
        Self { value: 3 }
    }

    pub fn german() -> Self {
        Self { value: 4 }
    }

    pub fn italian() -> Self {
        Self { value: 5 }
    }

    pub fn traditional() -> Self {
        Self { value: 6 }
    }

    pub fn simplified() -> Self {
        Self { value: 7 }
    }

    pub fn korean() -> Self {
        Self { value: 8 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/tutorialsequence/TutorialSequence.md")))]
#[::unity2::class(namespace = "App", name = "TutorialSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: tutorialsequence :: TutorialSequence >)]
pub struct TutorialSequence {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "SpriteAtlasPaths")]
    pub sprite_atlas_paths: ::unity2::Array<::unity2::Il2CppString>,
    #[static_field]
    #[rename(name = "CapCommonName")]
    pub cap_common_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "USCommonName")]
    pub us_common_name: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "EUCommonName")]
    pub eu_common_name: ::unity2::Il2CppString,
    #[rename(name = "m_TutorialObject")]
    pub m_tutorial_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_MainSpriteAtlas")]
    pub m_main_sprite_atlas: crate::unity_engine::u2d::spriteatlas::SpriteAtlas,
    #[rename(name = "m_SubSpriteAtlas")]
    pub m_sub_sprite_atlas: crate::unity_engine::u2d::spriteatlas::SpriteAtlas,
    #[rename(name = "m_TutorialData")]
    pub m_tutorial_data:
        crate::system::collections::generic::list_1::List_1<crate::app::tutorialdata::TutorialData>,
    #[rename(name = "m_TutorialID")]
    pub m_tutorial_id: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "FirstPage")]
    pub first_page: i32,
    #[rename(name = "m_Page")]
    pub m_page: i32,
    #[rename(name = "m_IsUpdate")]
    pub m_is_update: bool,
    #[rename(name = "m_PrefabHandle")]
    pub m_prefab_handle: crate::app::tresourcehandle_1::TResourceHandle_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_MainHandle")]
    pub m_main_handle: crate::app::tresourcehandle_1::TResourceHandle_1<
        crate::unity_engine::u2d::spriteatlas::SpriteAtlas,
    >,
    #[rename(name = "m_SubHandle")]
    pub m_sub_handle: crate::app::tresourcehandle_1::TResourceHandle_1<
        crate::unity_engine::u2d::spriteatlas::SpriteAtlas,
    >,
    #[static_field]
    #[rename(name = "ClassChangeKey")]
    pub class_change_key: ::unity2::Il2CppString,
}

#[cfg(feature = "app-tutorialsequence")]
#[::unity2::methods]
impl TutorialSequence {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, id: ::unity2::Il2CppString) -> ();

    #[method(name = "LoadData", args = 0)]
    pub fn load_data(self) -> ();

    #[method(name = "GetLanguage", args = 0)]
    pub fn get_language() -> crate::app::language::Language_Langs;

    #[method(name = "GetSpriteAtlasPath", args = 0)]
    pub fn get_sprite_atlas_path() -> ::unity2::Il2CppString;

    #[method(name = "GetImageName", args = 2)]
    pub fn get_image_name(
        sprite_atlas: crate::unity_engine::u2d::spriteatlas::SpriteAtlas,
        page: i32,
    ) -> ::unity2::Il2CppString;

    #[method(name = "UnloadData", args = 0)]
    pub fn unload_data(self) -> ();

    #[method(name = "IsLoadData", args = 0)]
    pub fn is_load_data(self) -> bool;

    #[method(name = "PrepareTutorial", args = 0)]
    pub fn prepare_tutorial(self) -> ();

    #[method(name = "CheckTutorial", args = 0)]
    pub fn check_tutorial(self) -> ();

    #[method(name = "Init", args = 0)]
    pub fn init(self) -> ();

    #[method(name = "OpenWindow", args = 0)]
    pub fn open_window(self) -> ();

    #[method(name = "WaitOpenWindow", args = 0)]
    pub fn wait_open_window(self) -> ();

    #[method(name = "TutorialTick", args = 0)]
    pub fn tutorial_tick(self) -> ();

    #[method(name = "IsCloseWindow", args = 0)]
    pub fn is_close_window(self) -> bool;

    #[method(name = "CloseWindow", args = 0)]
    pub fn close_window(self) -> ();

    #[method(name = "WaitCloseWindow", args = 0)]
    pub fn wait_close_window(self) -> ();

    #[method(name = "SetPage", args = 0)]
    pub fn set_page(self) -> ();

    #[method(name = "SetArrow", args = 0)]
    pub fn set_arrow(self) -> ();

    #[method(name = "CreateBind", args = 2)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst, tid: ::unity2::Il2CppString) -> ();

    #[method(name = "TryCreateBind", args = 2)]
    pub fn try_create_bind(
        super_: crate::app::procinst::ProcInst,
        tid: ::unity2::Il2CppString,
    ) -> bool;

    #[method(name = "IsClassChanged", args = 0)]
    pub fn is_class_changed() -> bool;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-tutorialsequence")]
impl TutorialSequence {
    pub fn new(id: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TutorialSequence),
                ::core::stringify!(new),
            )
        });
        <Self as ITutorialSequenceMethods>::ctor(this, id);
        this
    }
}
