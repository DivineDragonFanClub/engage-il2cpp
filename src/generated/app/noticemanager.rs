
use crate::app::singletonmonobehaviour_1::ISingletonMonoBehaviour_1;
use crate::app::singletonmonobehaviour_1::SingletonMonoBehaviour_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use crate::unity_engine::behaviour::Behaviour;
use crate::unity_engine::behaviour::IBehaviour;
use crate::unity_engine::component::Component;
use crate::unity_engine::component::IComponent;
use crate::unity_engine::monobehaviour::IMonoBehaviour;
use crate::unity_engine::monobehaviour::MonoBehaviour;
use crate::unity_engine::object_2::IObject_2;
use crate::unity_engine::object_2::Object_2;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/noticemanager/NoticeManager_Kinds.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct NoticeManager_Kinds {
    pub value: i32,
}

impl ::unity2::ClassIdentity for NoticeManager_Kinds {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "NoticeManager.Kinds";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for NoticeManager_Kinds {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl NoticeManager_Kinds {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn facility() -> Self {
        Self { value: 1 }
    }

    pub fn kizuna() -> Self {
        Self { value: 2 }
    }

    pub fn tutorial() -> Self {
        Self { value: 3 }
    }

    pub fn notebook() -> Self {
        Self { value: 4 }
    }

    pub fn ring_list() -> Self {
        Self { value: 5 }
    }

    pub fn num() -> Self {
        Self { value: 6 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/noticemanager/NoticeManager.md")))]
#[::unity2::class(namespace = "App", name = "NoticeManager")]
# [parent (crate :: app :: singletonmonobehaviour_1 :: SingletonMonoBehaviour_1 < crate :: app :: noticemanager :: NoticeManager >)]
pub struct NoticeManager {
    #[rename(name = "m_AchievementPopUp")]
    pub m_achievement_pop_up: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Animator")]
    pub m_animator: crate::unity_engine::animator::Animator,
    #[rename(name = "m_TextMesh")]
    pub m_text_mesh: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
    #[rename(name = "m_ClearObject")]
    pub m_clear_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Queues")]
    pub m_queues: ::unity2::Array<
        crate::system::collections::generic::queue_1::Queue_1<::unity2::Il2CppString>,
    >,
}

#[cfg(feature = "app-noticemanager")]
#[::unity2::methods]
impl NoticeManager {
    #[method(name = "Add", args = 2)]
    pub fn add(
        kind: crate::app::noticemanager::NoticeManager_Kinds,
        text: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear() -> ();

    #[method(name = "Stop", args = 0)]
    pub fn stop() -> ();

    #[method(name = "CanKizunaTalk", args = 0)]
    pub fn can_kizuna_talk() -> bool;

    #[method(name = "UpdateKizunaTalk", args = 0)]
    pub fn update_kizuna_talk() -> ();

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = "Start", args = 0)]
    pub fn start(self) -> ();

    #[method(name = "IsFinish", args = 1)]
    pub fn is_finish(self, animator: crate::unity_engine::animator::Animator) -> bool;

    #[method(name = "CanShow", args = 0)]
    pub fn can_show(self) -> bool;

    #[method(name = "Show", args = 2)]
    pub fn show(self, text: ::unity2::Il2CppString, clear: bool) -> ();

    #[method(name = "Update", args = 0)]
    pub fn update(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-noticemanager")]
impl NoticeManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(NoticeManager),
                ::core::stringify!(new),
            )
        });
        <Self as INoticeManagerMethods>::ctor(this);
        this
    }
}
