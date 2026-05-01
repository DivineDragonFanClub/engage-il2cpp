
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use crate::unity_engine::playables::playableasset::IPlayableAsset;
use crate::unity_engine::playables::playableasset::PlayableAsset;
use crate::unity_engine::scriptableobject::IScriptableObject;
use crate::unity_engine::scriptableobject::ScriptableObject;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomeventclip/MyRoomEventClip_EventClipType.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MyRoomEventClip_EventClipType {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MyRoomEventClip_EventClipType {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MyRoomEventClip.EventClipType";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MyRoomEventClip_EventClipType {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MyRoomEventClip_EventClipType {
    pub fn effect() -> Self {
        Self { value: 0 }
    }

    pub fn sound() -> Self {
        Self { value: 1 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomeventclip/MyRoomEventClip.md")))]
#[::unity2::class(namespace = "App", name = "MyRoomEventClip")]
#[parent(crate::unity_engine::playables::playableasset::PlayableAsset)]
pub struct MyRoomEventClip {
    #[rename(name = "eventType")]
    pub event_type: crate::app::myroomeventclip::MyRoomEventClip_EventClipType,
    #[rename(name = "eventLayer")]
    pub event_layer: crate::app::myroomeventclip::MyRoomEventClip_EventClipLayer,
    #[rename(name = "eventName")]
    pub event_name: ::unity2::Il2CppString,
    #[rename(name = "isOut")]
    pub is_out: bool,
}

#[cfg(feature = "app-myroomeventclip")]
#[::unity2::methods]
impl MyRoomEventClip {
    #[method(name = "get_clipCaps", args = 0)]
    pub fn get_clip_caps(self) -> crate::unity_engine::timeline::clipcaps::ClipCaps;

    #[method(name = "CreatePlayable", args = 2)]
    pub fn create_playable(
        self,
        graph: crate::unity_engine::playables::playablegraph::PlayableGraph,
        owner: crate::unity_engine::gameobject::GameObject,
    ) -> crate::unity_engine::playables::playable::Playable;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-myroomeventclip")]
impl MyRoomEventClip {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(MyRoomEventClip),
                ::core::stringify!(new),
            )
        });
        <Self as IMyRoomEventClipMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/myroomeventclip/MyRoomEventClip_EventClipLayer.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct MyRoomEventClip_EventClipLayer {
    pub value: i32,
}

impl ::unity2::ClassIdentity for MyRoomEventClip_EventClipLayer {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "MyRoomEventClip.EventClipLayer";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for MyRoomEventClip_EventClipLayer {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl MyRoomEventClip_EventClipLayer {
    pub fn background() -> Self {
        Self { value: 0 }
    }

    pub fn foreground() -> Self {
        Self { value: 1 }
    }
}
