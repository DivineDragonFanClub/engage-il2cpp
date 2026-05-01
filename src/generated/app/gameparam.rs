
use crate::app::structbase::IStructBase;
use crate::app::structbase::StructBase;
use crate::app::structdata_1::IStructData_1;
use crate::app::structdata_1::StructData_1;
use crate::app::structtemplate_1::IStructTemplate_1;
use crate::app::structtemplate_1::StructTemplate_1;
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

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameparam/GameParam_Holder.md")))]
#[::unity2::class(namespace = "App", name = "GameParam.Holder")]
#[parent(crate::system::object::Object)]
pub struct GameParam_Holder {
    #[rename(name = "m_Name")]
    pub m_name: ::unity2::Il2CppString,
    #[rename(name = "m_Param")]
    pub m_param: crate::app::gameparam::GameParam,
}

#[cfg(feature = "app-gameparam")]
#[::unity2::methods]
impl GameParam_Holder {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, name: ::unity2::Il2CppString) -> ();

    #[method(name = "GetParam", args = 0)]
    pub fn get_param(self) -> crate::app::gameparam::GameParam;

    #[method(name = "Clear", args = 0)]
    pub fn clear(self) -> ();

    #[method(name = "GetInt", args = 0)]
    pub fn get_int(self) -> i32;

    #[method(name = "GetFloat", args = 0)]
    pub fn get_float(self) -> f32;

    #[method(name = "GetBool", args = 0)]
    pub fn get_bool(self) -> bool;
}

#[cfg(feature = "app-gameparam")]
impl GameParam_Holder {
    pub fn new(name: ::unity2::Il2CppString) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameParam_Holder),
                ::core::stringify!(new),
            )
        });
        <Self as IGameParam_HolderMethods>::ctor(this, name);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameparam/GameParam_ParamFunction.md")))]
#[::unity2::class(namespace = "App", name = "GameParam.ParamFunction")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct GameParam_ParamFunction {}

#[cfg(feature = "app-gameparam")]
#[::unity2::methods]
impl GameParam_ParamFunction {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 2)]
    pub fn invoke(
        self,
        param: crate::app::gameparam::GameParam,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();
}

#[cfg(feature = "app-gameparam")]
impl GameParam_ParamFunction {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameParam_ParamFunction),
                ::core::stringify!(new),
            )
        });
        <Self as IGameParam_ParamFunctionMethods>::ctor(this, object, method);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameparam/GameParam_Kind.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct GameParam_Kind {
    pub value: i32,
}

impl ::unity2::ClassIdentity for GameParam_Kind {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "GameParam.Kind";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for GameParam_Kind {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl GameParam_Kind {
    pub fn none() -> Self {
        Self { value: 0 }
    }

    pub fn value() -> Self {
        Self { value: 1 }
    }

    pub fn open() -> Self {
        Self { value: 2 }
    }

    pub fn close() -> Self {
        Self { value: 3 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/gameparam/GameParam.md")))]
#[::unity2::class(namespace = "App", name = "GameParam")]
# [parent (crate :: app :: structdata_1 :: StructData_1 < crate :: app :: gameparam :: GameParam >)]
pub struct GameParam {}

#[cfg(feature = "app-gameparam")]
#[::unity2::methods]
impl GameParam {
    #[method(name = "Load", args = 0)]
    pub fn load() -> ();

    #[method(name = "get_Name", args = 0)]
    pub fn get_name(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Name", args = 1)]
    pub fn set_name(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_English", args = 0)]
    pub fn get_english(self) -> ::unity2::Il2CppString;

    #[method(name = "set_English", args = 1)]
    pub fn set_english(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Value", args = 0)]
    pub fn get_value(self) -> f32;

    #[method(name = "set_Value", args = 1)]
    pub fn set_value(self, value: f32) -> ();

    #[method(name = "get_Min", args = 0)]
    pub fn get_min(self) -> f32;

    #[method(name = "set_Min", args = 1)]
    pub fn set_min(self, value: f32) -> ();

    #[method(name = "get_Max", args = 0)]
    pub fn get_max(self) -> f32;

    #[method(name = "set_Max", args = 1)]
    pub fn set_max(self, value: f32) -> ();

    #[method(name = "get_Step", args = 0)]
    pub fn get_step(self) -> f32;

    #[method(name = "set_Step", args = 1)]
    pub fn set_step(self, value: f32) -> ();

    #[method(name = "get_Enum", args = 0)]
    pub fn get_enum(self) -> ::unity2::Il2CppString;

    #[method(name = "set_Enum", args = 1)]
    pub fn set_enum(self, value: ::unity2::Il2CppString) -> ();

    #[method(name = "get_Initial", args = 0)]
    pub fn get_initial(self) -> f32;

    #[method(name = "set_Initial", args = 1)]
    pub fn set_initial(self, value: f32) -> ();

    #[method(name = "OnBuild", args = 0)]
    pub fn on_build(self) -> ();

    #[method(name = "GetDebugName", args = 0)]
    pub fn get_debug_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetGroupName", args = 0)]
    pub fn get_group_name(self) -> ::unity2::Il2CppString;

    #[method(name = "GetKind", args = 0)]
    pub fn get_kind(self) -> crate::app::gameparam::GameParam_Kind;

    #[method(name = "Reset", args = 0)]
    pub fn reset(self) -> ();

    #[method(name = "GetString", args = 0)]
    pub fn get_string(self) -> ::unity2::Il2CppString;

    #[method(name = "GetBool", args = 1)]
    pub fn get_bool(name: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetInt", args = 1)]
    pub fn get_int(name: ::unity2::Il2CppString) -> i32;

    #[method(name = "GetFloat", args = 1)]
    pub fn get_float(name: ::unity2::Il2CppString) -> f32;

    #[method(name = "GetColor", args = 3)]
    pub fn get_color(
        name_r: ::unity2::Il2CppString,
        name_g: ::unity2::Il2CppString,
        name_b: ::unity2::Il2CppString,
    ) -> crate::unity_engine::color::Color;

    #[method(name = "GetColor", args = 4)]
    pub fn get_color_2(
        name_r: ::unity2::Il2CppString,
        name_g: ::unity2::Il2CppString,
        name_b: ::unity2::Il2CppString,
        name_a: ::unity2::Il2CppString,
    ) -> crate::unity_engine::color::Color;

    #[method(name = "TryGetGroupIndex", args = 1)]
    pub fn try_get_group_index(group: ::unity2::Il2CppString) -> i32;

    #[method(name = "EachFunction", args = 3)]
    pub fn each_function(
        func: crate::app::gameparam::GameParam_ParamFunction,
        index: i32,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> i32;

    #[method(name = "EachFunction", args = 3)]
    pub fn each_function_2(
        func: crate::app::gameparam::GameParam_ParamFunction,
        group: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-gameparam")]
impl GameParam {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(GameParam),
                ::core::stringify!(new),
            )
        });
        <Self as IGameParamMethods>::ctor(this);
        this
    }
}
