
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/talkcharactersignal/TalkCharacterSignal.md")))]
#[::unity2::class(namespace = "App", name = "TalkCharacterSignal")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct TalkCharacterSignal {
    #[rename(name = "_cp")]
    pub cp: crate::combat::character::Character,
    #[static_field]
    #[rename(name = "FootStepLabel")]
    pub foot_step_label: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "HubLabel")]
    pub hub_label: ::unity2::Il2CppString,
}

#[cfg(feature = "app-talkcharactersignal")]
#[::unity2::methods]
impl TalkCharacterSignal {
    #[method(name = "get_CP", args = 0)]
    pub fn get_cp(self) -> crate::combat::character::Character;

    #[method(name = "get_IsPlayer", args = 0)]
    pub fn get_is_player(self) -> bool;

    #[method(name = "set_IsPlayer", args = 1)]
    pub fn set_is_player(self, value: bool) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "左足接地", args = 0)]
    pub fn _unnamed(self) -> ();

    #[method(name = "音汎用", args = 1)]
    pub fn _unnamed_2(self, str_param: ::unity2::Il2CppString) -> ();

    #[method(name = "パーティクル", args = 1)]
    pub fn _unnamed_3(self, ev: crate::unity_engine::animationevent::AnimationEvent) -> ();

    #[method(name = "SetEnable", args = 2)]
    pub fn set_enable(go: crate::unity_engine::gameobject::GameObject, enabled: bool) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-talkcharactersignal")]
impl TalkCharacterSignal {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(TalkCharacterSignal),
                ::core::stringify!(new),
            )
        });
        <Self as ITalkCharacterSignalMethods>::ctor(this);
        this
    }
}
