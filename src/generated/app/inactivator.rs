
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inactivator/Inactivator_Flags.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Inactivator_Flags {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Inactivator_Flags {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Inactivator.Flags";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Inactivator_Flags {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Inactivator_Flags {
    pub fn _unnamed() -> Self {
        Self { value: 1 }
    }

    pub fn gmap() -> Self {
        Self { value: 16 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inactivator/Inactivator.md")))]
#[::unity2::class(namespace = "App", name = "Inactivator")]
#[parent(crate::unity_engine::monobehaviour::MonoBehaviour)]
pub struct Inactivator {
    #[rename(name = "m_Flags")]
    pub m_flags: crate::app::inactivator::Inactivator_Flags,
    #[rename(name = "m_Variable")]
    pub m_variable: ::unity2::Il2CppString,
}

#[cfg(feature = "app-inactivator")]
#[::unity2::methods]
impl Inactivator {
    #[method(name = "IsUsed", args = 0)]
    pub fn is_used(self) -> bool;

    #[method(name = "GetKind", args = 0)]
    pub fn get_kind() -> crate::app::inactivator::Inactivator_Kind;

    #[method(name = "Awake", args = 0)]
    pub fn awake(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-inactivator")]
impl Inactivator {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Inactivator),
                ::core::stringify!(new),
            )
        });
        <Self as IInactivatorMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/inactivator/Inactivator_Kind.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct Inactivator_Kind {
    pub value: i32,
}

impl ::unity2::ClassIdentity for Inactivator_Kind {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "Inactivator.Kind";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for Inactivator_Kind {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl Inactivator_Kind {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn story() -> Self {
        Self { value: 1 }
    }

    pub fn encount() -> Self {
        Self { value: 2 }
    }

    pub fn kizuna() -> Self {
        Self { value: 3 }
    }

    pub fn hub() -> Self {
        Self { value: 4 }
    }

    pub fn gmap() -> Self {
        Self { value: 5 }
    }
}
