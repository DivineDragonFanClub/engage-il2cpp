
use crate::system::object::IObject;
use crate::system::object::Object;
use crate::system::r#enum::Enum;
use crate::system::r#enum::IEnum;
use crate::system::valuetype::IValueType;
use crate::system::valuetype::ValueType;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/scriptutil/ScriptUtil_MenuCondtion.md")))]
#[repr(C)]
#[derive(
    ::core::clone::Clone,
    ::core::marker::Copy,
    ::core::fmt::Debug,
    ::core::cmp::PartialEq,
    ::core::cmp::Eq,
)]
pub struct ScriptUtil_MenuCondtion {
    pub value: i32,
}

impl ::unity2::ClassIdentity for ScriptUtil_MenuCondtion {
    const NAMESPACE: &'static str = "App";

    const NAME: &'static str = "ScriptUtil.MenuCondtion";

    fn class() -> ::unity2::Class {
        static CACHE: ::std::sync::OnceLock<::unity2::Class> = ::std::sync::OnceLock::new();

        *CACHE.get_or_init(|| ::unity2::Class::lookup(Self::NAMESPACE, Self::NAME))
    }
}

impl ::unity2::IlType for ScriptUtil_MenuCondtion {
    fn il_type() -> &'static ::unity2::il2cpp::Il2CppType {
        &<Self as ::unity2::ClassIdentity>::class()
            .raw()
            ._1
            .byval_arg
    }
}

impl ScriptUtil_MenuCondtion {
    pub fn hide() -> Self {
        Self { value: 0 }
    }

    pub fn enable() -> Self {
        Self { value: 1 }
    }

    pub fn disable() -> Self {
        Self { value: 2 }
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/scriptutil/ScriptUtil.md")))]
#[::unity2::class(namespace = "App", name = "ScriptUtil")]
#[parent(crate::system::object::Object)]
pub struct ScriptUtil {
    #[static_field]
    #[rename(name = "MAX_CURSOR_STACK")]
    pub max_cursor_stack: i32,
}

#[cfg(feature = "app-scriptutil")]
#[::unity2::methods]
impl ScriptUtil {
    #[method(name = "GetInt", args = 1)]
    pub fn get_int(value: crate::moon_sharp::interpreter::dynvalue::DynValue) -> i32;

    #[method(name = "GetFloat", args = 1)]
    pub fn get_float(value: crate::moon_sharp::interpreter::dynvalue::DynValue) -> f32;

    #[method(name = "GetString", args = 1)]
    pub fn get_string(
        value: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetBool", args = 1)]
    pub fn get_bool(value: crate::moon_sharp::interpreter::dynvalue::DynValue) -> bool;

    #[method(name = "IsString", args = 2)]
    pub fn is_string(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        index: i32,
    ) -> bool;

    #[method(name = "TryGetType", args = 2)]
    pub fn try_get_type(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        index: i32,
    ) -> crate::moon_sharp::interpreter::datatype::DataType;

    #[method(name = "TryGetInt", args = 3)]
    pub fn try_get_int(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        index: i32,
        nothing: i32,
    ) -> i32;

    #[method(name = "TryGetFloat", args = 3)]
    pub fn try_get_float(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        index: i32,
        nothing: f32,
    ) -> f32;

    #[method(name = "TryGetHash", args = 3)]
    pub fn try_get_hash(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        index: i32,
        nothing: i32,
    ) -> i32;

    #[method(name = "TryGetString", args = 3)]
    pub fn try_get_string(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        index: i32,
        nothing: ::unity2::Il2CppString,
    ) -> ::unity2::Il2CppString;

    #[method(name = "TryGetBool", args = 3)]
    pub fn try_get_bool(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        index: i32,
        nothing: bool,
    ) -> bool;

    #[method(name = "TryGetForce", args = 3)]
    pub fn try_get_force(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        index: i32,
        nothing: crate::app::force::Force_Type,
    ) -> crate::app::force::Force_Type;

    #[method(name = "TryGetUnit", args = 3)]
    pub fn try_get_unit(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        index: i32,
        warning: bool,
    ) -> crate::app::unit::Unit;

    #[method(name = "TryMapGetUnit", args = 3)]
    pub fn try_map_get_unit(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        index: i32,
        warning: bool,
    ) -> crate::app::unit::Unit;

    #[method(name = "TryGetGodUnit", args = 3)]
    pub fn try_get_god_unit(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        index: i32,
        warning: bool,
    ) -> crate::app::godunit::GodUnit;

    #[method(name = "TryGetItem", args = 3)]
    pub fn try_get_item(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        index: i32,
        warning: bool,
    ) -> crate::app::itemdata::ItemData;

    #[method(name = "TryGetFunc", args = 2)]
    pub fn try_get_func(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        index: i32,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "TryGetValue", args = 2)]
    pub fn try_get_value(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        index: i32,
    ) -> crate::moon_sharp::interpreter::dynvalue::DynValue;

    #[method(name = "TryGetArgs", args = 2)]
    pub fn try_get_args(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        index: i32,
    ) -> ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>;

    #[method(name = "TryGetGameObject", args = 2)]
    pub fn try_get_game_object(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        index: i32,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "TryGetLocator", args = 2)]
    pub fn try_get_locator(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        index: i32,
    ) -> crate::unity_engine::gameobject::GameObject;

    #[method(name = "GetFuncName", args = 1)]
    pub fn get_func_name(
        func: crate::moon_sharp::interpreter::dynvalue::DynValue,
    ) -> ::unity2::Il2CppString;

    #[method(name = "GetScriptPath", args = 1)]
    pub fn get_script_path(name: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "Yield", args = 0)]
    pub fn r#yield() -> ();

    #[method(name = "Call", args = 2)]
    pub fn call(
        func: crate::moon_sharp::interpreter::dynvalue::DynValue,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "AddCoroutine", args = 2)]
    pub fn add_coroutine(
        func: crate::moon_sharp::interpreter::dynvalue::DynValue,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "JumpCoroutine", args = 2)]
    pub fn jump_coroutine(
        func: crate::moon_sharp::interpreter::dynvalue::DynValue,
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
    ) -> ();

    #[method(name = "GetSequence", args = 0)]
    pub fn get_sequence() -> crate::app::procinst::ProcInst;

    #[method(name = "TryGetMenuCondition", args = 2)]
    pub fn try_get_menu_condition(
        args: ::unity2::Array<crate::moon_sharp::interpreter::dynvalue::DynValue>,
        index: i32,
    ) -> crate::app::scriptutil::ScriptUtil_MenuCondtion;

    #[method(name = "GetCursorStackName", args = 1)]
    pub fn get_cursor_stack_name(index: i32) -> ::unity2::Il2CppString;

    #[method(name = "CreateCursorStack", args = 0)]
    pub fn create_cursor_stack() -> ();

    #[method(name = "ClearCursorStack", args = 0)]
    pub fn clear_cursor_stack() -> ();

    #[method(name = "ClearCursorStack", args = 1)]
    pub fn clear_cursor_stack_2(index: i32) -> ();

    #[method(name = "GetCursorStack", args = 1)]
    pub fn get_cursor_stack(index: i32) -> i32;

    #[method(name = "SetCursorStack", args = 2)]
    pub fn set_cursor_stack(index: i32, value: i32) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "app-scriptutil")]
impl ScriptUtil {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ScriptUtil),
                ::core::stringify!(new),
            )
        });
        <Self as IScriptUtilMethods>::ctor(this);
        this
    }
}
