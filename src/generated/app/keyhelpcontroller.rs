
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/keyhelpcontroller/KeyHelpController.md")))]
#[::unity2::class(namespace = "App", name = "KeyHelpController")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct KeyHelpController {
    #[rename(name = "m_HelpObject")]
    pub m_help_object: crate::system::collections::generic::list_1::List_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_Interval")]
    pub m_interval: f32,
}

#[cfg(feature = "app-keyhelpcontroller")]
#[::unity2::methods]
impl KeyHelpController {
    #[method(name = "SetKeyHelpMessage", args = 1)]
    pub fn set_key_help_message(self, key_help_id: ::unity2::Il2CppString) -> ();

    #[method(name = "Hide", args = 0)]
    pub fn hide(self) -> ();

    #[method(name = "SetText", args = 2)]
    pub fn set_text(
        self,
        game_object: crate::unity_engine::gameobject::GameObject,
        text: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-keyhelpcontroller")]
impl KeyHelpController {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(KeyHelpController),
                ::core::stringify!(new),
            )
        });
        <Self as IKeyHelpControllerMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/keyhelpcontroller/KeyHelpController_Type.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct KeyHelpController_Type {
    pub value: i32,
}

impl ::unity2::ClassIdentity for KeyHelpController_Type {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "KeyHelpController.Type";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for KeyHelpController_Type {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl KeyHelpController_Type {
    pub fn a() -> Self {
        Self { value: 0 }
    }

    pub fn b() -> Self {
        Self { value: 1 }
    }

    pub fn x() -> Self {
        Self { value: 2 }
    }

    pub fn y() -> Self {
        Self { value: 3 }
    }

    pub fn l() -> Self {
        Self { value: 4 }
    }

    pub fn r() -> Self {
        Self { value: 5 }
    }

    pub fn lr() -> Self {
        Self { value: 6 }
    }

    pub fn zl() -> Self {
        Self { value: 7 }
    }

    pub fn zr() -> Self {
        Self { value: 8 }
    }

    pub fn zlr() -> Self {
        Self { value: 9 }
    }

    pub fn plus() -> Self {
        Self { value: 10 }
    }

    pub fn minus() -> Self {
        Self { value: 11 }
    }

    pub fn up() -> Self {
        Self { value: 12 }
    }

    pub fn down() -> Self {
        Self { value: 13 }
    }

    pub fn up_down() -> Self {
        Self { value: 14 }
    }

    pub fn left() -> Self {
        Self { value: 15 }
    }

    pub fn right() -> Self {
        Self { value: 16 }
    }

    pub fn left_right() -> Self {
        Self { value: 17 }
    }

    pub fn stick_l() -> Self {
        Self { value: 18 }
    }

    pub fn stick_r() -> Self {
        Self { value: 19 }
    }

    pub fn num() -> Self {
        Self { value: 20 }
    }
}
