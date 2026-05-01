
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/amiibotopsequence/AmiiboTopSequence.md")))]
#[::unity2::class(namespace = "App", name = "AmiiboTopSequence")]
# [parent (crate :: app :: singletonprocinst_1 :: SingletonProcInst_1 < crate :: app :: amiibotopsequence :: AmiiboTopSequence >)]
pub struct AmiiboTopSequence {
    #[rename(name = "m_menuResult")]
    pub m_menu_result: crate::app::amiibotopmenu::AmiiboTopMenu_MenuResult,
    #[rename(name = "m_AccessoryShopChangeRoot")]
    pub m_accessory_shop_change_root: crate::app::accessoryshopchangeroot::AccessoryShopChangeRoot,
}

#[cfg(feature = "app-amiibotopsequence")]
#[::unity2::methods]
impl AmiiboTopSequence {
    #[method(name = "CreateBind", args = 1)]
    pub fn create_bind(super_: crate::app::procinst::ProcInst) -> ();

    #[method(name = "CreateDesc", args = 0)]
    pub fn create_desc(self) -> ::unity2::Array<crate::app::procdesc::ProcDesc>;

    #[method(name = "LoadResources", args = 0)]
    pub fn load_resources(self) -> ();

    #[method(name = "IsLoadingResources", args = 0)]
    pub fn is_loading_resources(self) -> bool;

    #[method(name = "OpenTitle", args = 0)]
    pub fn open_title(self) -> ();

    #[method(name = "CloseTitle", args = 0)]
    pub fn close_title(self) -> ();

    #[method(name = "UnloadResources", args = 0)]
    pub fn unload_resources(self) -> ();

    #[method(name = "CreateTopMenu", args = 0)]
    pub fn create_top_menu(self) -> ();

    #[method(name = "CreateAccessoryMenu", args = 0)]
    pub fn create_accessory_menu(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-amiibotopsequence")]
impl AmiiboTopSequence {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(AmiiboTopSequence),
                ::core::stringify!(new),
            )
        });
        <Self as IAmiiboTopSequenceMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/amiibotopsequence/AmiiboTopSequence_Label.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct AmiiboTopSequence_Label {
    pub value: i32,
}

impl ::unity2::ClassIdentity for AmiiboTopSequence_Label {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "AmiiboTopSequence.Label";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for AmiiboTopSequence_Label {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl AmiiboTopSequence_Label {
    pub fn init() -> Self {
        Self { value: 0 }
    }

    pub fn entry() -> Self {
        Self { value: 1 }
    }

    pub fn amiibo() -> Self {
        Self { value: 2 }
    }

    pub fn accessory() -> Self {
        Self { value: 3 }
    }

    pub fn sound() -> Self {
        Self { value: 4 }
    }

    pub fn exit() -> Self {
        Self { value: 5 }
    }

    pub fn end() -> Self {
        Self { value: 6 }
    }
}
