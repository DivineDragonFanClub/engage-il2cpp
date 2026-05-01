
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencegod/MapSequenceGod_Kind.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapSequenceGod_Kind {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapSequenceGod_Kind {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSequenceGod.Kind";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSequenceGod_Kind {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapSequenceGod_Kind {
    pub fn engage_start() -> Self {
        Self { value: 0 }
    }

    pub fn engage_link() -> Self {
        Self { value: 1 }
    }

    pub fn god_change() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencegod/MapSequenceGod_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MapSequenceGod_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MapSequenceGod_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MapSequenceGod.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MapSequenceGod_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MapSequenceGod_Label {
    pub fn detail() -> Self {
        Self { value: 0 }
    }

    pub fn simple() -> Self {
        Self { value: 1 }
    }

    pub fn end() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencegod/MapSequenceGod.md")))]
#[::unity2::class(namespace = "App", name = "MapSequenceGod")]
#[parent(crate::system::object::Object)]
pub struct MapSequenceGod {}

#[cfg(feature = "app-mapsequencegod")]
#[::unity2::methods]
impl MapSequenceGod {
    #[method(name = "CreateBindEngageSimple", args = 1)]
    pub fn create_bind_engage_simple(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindEngageStart", args = 1)]
    pub fn create_bind_engage_start(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindEngageLink", args = 1)]
    pub fn create_bind_engage_link(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindGodChange", args = 1)]
    pub fn create_bind_god_change(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateBindEngageCancel", args = 1)]
    pub fn create_bind_engage_cancel(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapsequencegod")]
impl MapSequenceGod {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceGod),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceGodMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencegod/MapSequenceGod_ProcEngageStart.md")))]
#[::unity2::class(namespace = "App", name = "MapSequenceGod.ProcEngageStart")]
#[parent(crate::app::mapsequencegod::MapSequenceGod_ProcEngage)]
pub struct MapSequenceGod_ProcEngageStart {
    #[rename(name = "m_Kind")]
    pub m_kind: crate::app::mapsequencegod::MapSequenceGod_Kind,
    #[rename(name = "m_IsSimple")]
    pub m_is_simple: bool,
}

#[cfg(feature = "app-mapsequencegod")]
#[::unity2::methods]
impl MapSequenceGod_ProcEngageStart {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, kind: crate::app::mapsequencegod::MapSequenceGod_Kind, is_simple: bool)
        -> ();

    #[method(name = "PlayEffect", args = 1)]
    pub fn play_effect(self, wait: f32) -> ();

    #[method(name = "PlayEffectSimple", args = 0)]
    pub fn play_effect_simple(self) -> ();

    #[method(name = "PlayEffectDetail", args = 0)]
    pub fn play_effect_detail(self) -> ();

    #[method(name = "Demo", args = 0)]
    pub fn demo(self) -> ();

    #[method(name = "Apply", args = 0)]
    pub fn apply(self) -> ();

    #[method(name = "ApplyEngage", args = 0)]
    pub fn apply_engage(self) -> ();

    #[method(name = "ApplyGodChange", args = 0)]
    pub fn apply_god_change(self) -> ();

    #[method(name = "IsSimple", args = 0)]
    pub fn is_simple(self) -> bool;

    #[method(name = "Branch", args = 0)]
    pub fn branch(self) -> ();

    #[method(name = "CreateBind", args = 3)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        kind: crate::app::mapsequencegod::MapSequenceGod_Kind,
        is_simple: bool,
    ) -> ();
}

#[cfg(feature = "app-mapsequencegod")]
impl MapSequenceGod_ProcEngageStart {
    pub fn new(kind: crate::app::mapsequencegod::MapSequenceGod_Kind, is_simple: bool) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceGod_ProcEngageStart),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceGod_ProcEngageStartMethods>::ctor(this, kind, is_simple);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencegod/MapSequenceGod_ProcEngageCancel.md")))]
#[::unity2::class(namespace = "App", name = "MapSequenceGod.ProcEngageCancel")]
#[parent(crate::app::mapsequencegod::MapSequenceGod_ProcEngage)]
pub struct MapSequenceGod_ProcEngageCancel {}

#[cfg(feature = "app-mapsequencegod")]
#[::unity2::methods]
impl MapSequenceGod_ProcEngageCancel {
    #[method(name = "Cancel", args = 0)]
    pub fn cancel(self) -> ();

    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapsequencegod")]
impl MapSequenceGod_ProcEngageCancel {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceGod_ProcEngageCancel),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceGod_ProcEngageCancelMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/mapsequencegod/MapSequenceGod_ProcEngage.md")))]
#[::unity2::class(namespace = "App", name = "MapSequenceGod.ProcEngage")]
#[parent(crate::app::procinst::ProcInst)]
pub struct MapSequenceGod_ProcEngage {}

#[cfg(feature = "app-mapsequencegod")]
#[::unity2::methods]
impl MapSequenceGod_ProcEngage {
    #[method(name = "IsUsableInfo", args = 0)]
    pub fn is_usable_info(self) -> bool;

    #[method(name = "IsNeedToDeploy", args = 0)]
    pub fn is_need_to_deploy(self) -> bool;

    #[method(name = "MoveWait", args = 0)]
    pub fn move_wait(self) -> ();

    #[method(name = "HideInfoUIForDetail", args = 0)]
    pub fn hide_info_ui_for_detail(self) -> ();

    #[method(name = "HideInfoUIForSimple", args = 0)]
    pub fn hide_info_ui_for_simple(self) -> ();

    #[method(name = "UpdateImage", args = 0)]
    pub fn update_image(self) -> ();

    #[method(name = "UpdateInfoUI", args = 0)]
    pub fn update_info_ui(self) -> ();

    #[method(name = "ShowInfoUI", args = 0)]
    pub fn show_info_ui(self) -> ();

    #[method(name = "UpdateDeploy", args = 0)]
    pub fn update_deploy(self) -> ();

    #[method(name = "UpdateMapRoute", args = 0)]
    pub fn update_map_route(self) -> ();

    #[method(name = "UpdateMapCursor", args = 0)]
    pub fn update_map_cursor(self) -> ();

    #[method(name = "IsUpdateMapRoute", args = 0)]
    pub fn is_update_map_route(self) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-mapsequencegod")]
impl MapSequenceGod_ProcEngage {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MapSequenceGod_ProcEngage),
                ::core::stringify!(new),
            )
        });
        <Self as IMapSequenceGod_ProcEngageMethods>::ctor(this);
        this
    }
}
