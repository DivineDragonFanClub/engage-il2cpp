
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortiesequenceringselect/SortieSequenceRingSelect_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SortieSequenceRingSelect_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SortieSequenceRingSelect_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "SortieSequenceRingSelect.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SortieSequenceRingSelect_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SortieSequenceRingSelect_Label {
    pub fn r#return() -> Self {
        Self { value: 0 }
    }

    pub fn decide() -> Self {
        Self { value: 1 }
    }

    pub fn end_wait() -> Self {
        Self { value: 2 }
    }

    pub fn end() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/sortiesequenceringselect/SortieSequenceRingSelect.md")))]
#[::unity2::class(namespace = "App", name = "SortieSequenceRingSelect")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: sortiesequenceringselect :: SortieSequenceRingSelect >)]
pub struct SortieSequenceRingSelect {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "HelpPrefabPath")]
    pub help_prefab_path: ::unity2::Il2CppString,
    #[rename(name = "m_GameObject")]
    pub m_game_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Window")]
    pub m_window: crate::app::ringselectroot::RingSelectRoot,
    #[rename(name = "m_RingMenu")]
    pub m_ring_menu: crate::app::ringselectmenu::RingSelectMenu,
}

#[cfg(feature = "app-sortiesequenceringselect")]
#[::unity2::methods]
impl SortieSequenceRingSelect {
    #[method(name = "get_RingMenu", args = 0)]
    pub fn get_ring_menu(self) -> crate::app::ringselectmenu::RingSelectMenu;

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "LoadRes", args = 0)]
    pub fn load_res(self) -> ();

    #[method(name = "IsLoadingRes", args = 0)]
    pub fn is_loading_res(self) -> bool;

    #[method(name = "Open", args = 0)]
    pub fn open(self) -> ();

    #[method(name = "MenuTick", args = 0)]
    pub fn menu_tick(self) -> ();

    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ();

    #[method(name = "Decide", args = 0)]
    pub fn decide(self) -> ();

    #[method(name = "IsPlayEngageDecide", args = 0)]
    pub fn is_play_engage_decide(self) -> bool;

    #[method(name = "GodRespondVoice", args = 0)]
    pub fn god_respond_voice(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-sortiesequenceringselect")]
impl SortieSequenceRingSelect {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SortieSequenceRingSelect),
                ::core::stringify!(new),
            )
        });
        <Self as ISortieSequenceRingSelectMethods>::ctor(this);
        this
    }
}
