
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/structheader/StructHeader_Param.md")))]
#[::unity2::class(namespace = "App", name = "StructHeader.Param")]
#[parent(crate::system::object::Object)]
pub struct StructHeader_Param {
    #[rename(name = "_type")]
    pub r#type: ::unity2::Il2CppString,
}

#[cfg(feature = "app-structheader")]
#[::unity2::methods]
impl StructHeader_Param {
    #[method(name = "get_name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_type", args = 0)]
    pub fn get_type(self) -> ::unity2::Il2CppString;

    #[method(name = "set_type", args = 1)]
    pub fn set_type(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_ident", args = 0)]
    pub fn get_ident(self) -> ::unity2::Il2CppString;

    #[method(name = "set_ident", args = 1)]
    pub fn set_ident(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_min", args = 0)]
    pub fn get_min(self) -> ::unity2::Il2CppString;

    #[method(name = "set_min", args = 1)]
    pub fn set_min(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_max", args = 0)]
    pub fn get_max(self) -> ::unity2::Il2CppString;

    #[method(name = "set_max", args = 1)]
    pub fn set_max(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_chg", args = 0)]
    pub fn get_chg(self) -> ::unity2::Il2CppString;

    #[method(name = "set_chg", args = 1)]
    pub fn set_chg(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-structheader")]
impl StructHeader_Param {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StructHeader_Param),
                ::core::stringify!(new),
            )
        });
        <Self as IStructHeader_ParamMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/structheader/StructHeader.md")))]
#[::unity2::class(namespace = "App", name = "StructHeader")]
#[parent(crate::system::object::Object)]
pub struct StructHeader {}

#[cfg(feature = "app-structheader")]
#[::unity2::methods]
impl StructHeader {
    #[method(name = "get_param", args = 0)]
    pub fn get_param(
        self,
    ) -> crate::system::collections::generic::list_1::List_1<
        crate::app::structheader::StructHeader_Param,
    >;

    #[method(name = "set_param", args = 1)]
    pub fn set_param(
        self,
        value: crate::system::collections::generic::list_1::List_1<
            crate::app::structheader::StructHeader_Param,
        >,
    ) -> ();

    #[method(name = "get_sheetName", args = 0)]
    pub fn get_sheet_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_sheetName", args = 1)]
    pub fn set_sheet_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_filePath", args = 0)]
    pub fn get_file_path(self) -> ::unity2::Il2CppString;

    #[method(name = "set_filePath", args = 1)]
    pub fn set_file_path(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "GetArrayIdent", args = 0)]
    pub fn get_array_ident(self) -> ::unity2::Il2CppString;
}

#[cfg(feature = "app-structheader")]
impl StructHeader {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StructHeader),
                ::core::stringify!(new),
            )
        });
        <Self as IStructHeaderMethods>::ctor(this);
        this
    }
}
