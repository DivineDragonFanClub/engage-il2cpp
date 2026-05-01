
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/switch/switchvideoplayer/SwitchVideoPlayer_Event.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct SwitchVideoPlayer_Event {
    pub value: i32,
}

impl ::unity2::ClassIdentity for SwitchVideoPlayer_Event {
    const NAMESPACE: &'static str = "UnityEngine.Switch";

    const NAME: &'static str = "SwitchVideoPlayer.Event";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for SwitchVideoPlayer_Event {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl SwitchVideoPlayer_Event {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn created() -> Self {
        Self { value: 1 }
    }

    pub fn end_of_stream() -> Self {
        Self { value: 2 }
    }

    pub fn loop_point_reached() -> Self {
        Self { value: 3 }
    }

    pub fn first_frame_ready() -> Self {
        Self { value: 4 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/switch/switchvideoplayer/SwitchVideoPlayer.md")))]
#[::unity2::class(namespace = "UnityEngine.Switch", name = "SwitchVideoPlayer")]
#[parent(crate::system::object::Object)]
pub struct SwitchVideoPlayer {
    #[rename(name = "m_Ptr")]
    pub m_ptr: ::unity2::IntPtr,
    #[rename(name = "m_MovieEvent")]
    pub m_movie_event:
        crate::unity_engine::switch::switchvideoplayer::SwitchVideoPlayer_MovieEventDelegate,
    #[static_field]
    #[rename(name = "OnMovieEvent")]
    pub on_movie_event:
        crate::unity_engine::switch::switchvideoplayer::SwitchVideoPlayer_MovieEventDelegate,
}

#[cfg(feature = "unity_engine-switch-switchvideoplayer")]
#[::unity2::methods]
impl SwitchVideoPlayer {
    #[method(name = "FireMovieEvent", args = 1)]
    pub fn fire_movie_event(
        event_value: crate::unity_engine::switch::switchvideoplayer::SwitchVideoPlayer_Event,
    ) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/switch/switchvideoplayer/SwitchVideoPlayer_MovieEventDelegate.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Switch",
    name = "SwitchVideoPlayer.MovieEventDelegate"
)]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct SwitchVideoPlayer_MovieEventDelegate {}

#[cfg(feature = "unity_engine-switch-switchvideoplayer")]
#[::unity2::methods]
impl SwitchVideoPlayer_MovieEventDelegate {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 1)]
    pub fn invoke(
        self,
        eventtype: crate::unity_engine::switch::switchvideoplayer::SwitchVideoPlayer_Event,
    ) -> ();
}

#[cfg(feature = "unity_engine-switch-switchvideoplayer")]
impl SwitchVideoPlayer_MovieEventDelegate {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SwitchVideoPlayer_MovieEventDelegate),
                ::core::stringify!(new),
            )
        });
        <Self as ISwitchVideoPlayer_MovieEventDelegateMethods>::ctor(this, object, method);
        this
    }
}
