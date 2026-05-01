
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/magicsignaltrack/MagicSignalTrack.md")))]
#[::unity2::class(namespace = "Combat", name = "MagicSignalTrack")]
#[parent(crate::system::object::Object)]
pub struct MagicSignalTrack {
    #[rename(name = "m_Title")]
    pub m_title: ::unity2::Il2CppString,
    #[rename(name = "m_Help")]
    pub m_help: ::unity2::Il2CppString,
    #[rename(name = "m_IsSubBullet")]
    pub m_is_sub_bullet: bool,
    #[rename(name = "Signals")]
    pub signals: crate::system::collections::generic::list_1::List_1<
        crate::combat::magicsignal::MagicSignal,
    >,
}

#[cfg(feature = "combat-magicsignaltrack")]
#[::unity2::methods]
impl MagicSignalTrack {
    #[method(name = "get_EndTime", args = 0)]
    pub fn get_end_time(self) -> f32;

    #[method(name = "set_EndTime", args = 1)]
    pub fn set_end_time(self, value: f32) -> ();

    #[method(name = ".ctor", args = 3)]
    pub fn ctor(
        self,
        title: ::unity2::Il2CppString,
        help: ::unity2::Il2CppString,
        is_sub_bullet: bool,
    ) -> ();

    #[method(name = "Find", args = 1)]
    pub fn find(
        self,
        cmd: crate::combat::magiccommand::MagicCommand,
    ) -> crate::combat::magicsignal::MagicSignal;
}

#[cfg(feature = "combat-magicsignaltrack")]
impl MagicSignalTrack {
    pub fn new(
        title: ::unity2::Il2CppString,
        help: ::unity2::Il2CppString,
        is_sub_bullet: bool,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MagicSignalTrack),
                ::core::stringify!(new),
            )
        });
        <Self as IMagicSignalTrackMethods>::ctor(this, title, help, is_sub_bullet);
        this
    }
}
