
use crate::app::singletonclass_1::ISingletonClass_1;
use crate::app::singletonclass_1::SingletonClass_1;
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/keyhelp/KeyHelp_UI_Element.md")))]
#[::unity2::class(namespace = "App", name = "KeyHelp.UI.Element")]
#[parent(crate::system::object::Object)]
pub struct KeyHelp_UI_Element {
    #[rename(name = "m_Root")]
    pub m_root: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Text")]
    pub m_text: crate::tm_pro::textmeshprougui::TextMeshProUGUI,
}

#[cfg(feature = "app-keyhelp")]
#[::unity2::methods]
impl KeyHelp_UI_Element {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, root: crate::unity_engine::gameobject::GameObject) -> ();

    #[method(name = "SetVisible", args = 1)]
    pub fn set_visible(self, is_visible: bool) -> ();

    #[method(name = "SetIndex", args = 1)]
    pub fn set_index(self, index: i32) -> ();

    #[method(name = "SetText", args = 1)]
    pub fn set_text(self, text: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "app-keyhelp")]
impl KeyHelp_UI_Element {
    pub fn new(root: crate::unity_engine::gameobject::GameObject) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(KeyHelp_UI_Element),
                ::core::stringify!(new),
            )
        });
        <Self as IKeyHelp_UI_ElementMethods>::ctor(this, root);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/keyhelp/KeyHelp_UI.md")))]
#[::unity2::class(namespace = "App", name = "KeyHelp.UI")]
#[parent(crate::system::object::Object)]
pub struct KeyHelp_UI {
    #[static_field]
    #[rename(name = "PrefabPath")]
    pub prefab_path: ::unity2::Il2CppString,
    #[static_field]
    #[rename(name = "ElementNames")]
    pub element_names: ::unity2::Array<::unity2::Il2CppString>,
    #[rename(name = "m_PrefabHandle")]
    pub m_prefab_handle: crate::app::tresourcehandle_1::TResourceHandle_1<
        crate::unity_engine::gameobject::GameObject,
    >,
    #[rename(name = "m_GameObject")]
    pub m_game_object: crate::unity_engine::gameobject::GameObject,
    #[rename(name = "m_Elements")]
    pub m_elements: crate::system::collections::generic::dictionary_2::Dictionary_2<
        crate::app::keyhelp::KeyHelp_Type,
        crate::app::keyhelp::KeyHelp_UI_Element,
    >,
    #[rename(name = "m_Index")]
    pub m_index: i32,
}

#[cfg(feature = "app-keyhelp")]
#[::unity2::methods]
impl KeyHelp_UI {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "CreateAsync", args = 0)]
    pub fn create_async(self) -> ();

    #[method(name = "IsCreating", args = 0)]
    pub fn is_creating(self) -> bool;

    #[method(name = "Destroy", args = 0)]
    pub fn destroy(self) -> ();

    #[method(name = "SetVisible", args = 1)]
    pub fn set_visible(self, is_visible: bool) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add(self, r#type: crate::app::keyhelp::KeyHelp_Type, text: ::unity2::Il2CppString)
        -> ();

    #[method(name = "Add", args = 1)]
    pub fn add_2(self, key_help_id: ::unity2::Il2CppString) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "CreateImpl", args = 0)]
    pub fn create_impl(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-keyhelp")]
impl KeyHelp_UI {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(KeyHelp_UI),
                ::core::stringify!(new),
            )
        });
        <Self as IKeyHelp_UIMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/keyhelp/KeyHelp.md")))]
#[::unity2::class(namespace = "App", name = "KeyHelp")]
# [parent (crate :: app :: singletonclass_1 :: SingletonClass_1 < crate :: app :: keyhelp :: KeyHelp >)]
pub struct KeyHelp {
    #[rename(name = "m_UI")]
    pub m_ui: crate::app::keyhelp::KeyHelp_UI,
}

#[cfg(feature = "app-keyhelp")]
#[::unity2::methods]
impl KeyHelp {
    #[method(name = "IsCreating", args = 0)]
    pub fn is_creating() -> bool;

    #[method(name = "SetVisible", args = 1)]
    pub fn set_visible(is_visible: bool) -> ();

    #[method(name = "Add", args = 2)]
    pub fn add(r#type: crate::app::keyhelp::KeyHelp_Type, text: ::unity2::Il2CppString) -> ();

    #[method(name = "Add", args = 1)]
    pub fn add_2(key_help_id: ::unity2::Il2CppString) -> ();

    #[method(name = "Clear", args = 0)]
    pub fn clear() -> ();

    #[method(name = "OnCreate", args = 0)]
    pub fn on_create(self) -> ();

    #[method(name = "OnDispose", args = 0)]
    pub fn on_dispose(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-keyhelp")]
impl KeyHelp {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(KeyHelp),
                ::core::stringify!(new),
            )
        });
        <Self as IKeyHelpMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/keyhelp/KeyHelp_Type.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct KeyHelp_Type {
    pub value: i32,
}

impl ::unity2::ClassIdentity for KeyHelp_Type {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "KeyHelp.Type";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for KeyHelp_Type {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl KeyHelp_Type {
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

    pub fn plus() -> Self {
        Self { value: 9 }
    }

    pub fn minus() -> Self {
        Self { value: 10 }
    }

    pub fn num() -> Self {
        Self { value: 11 }
    }
}
