
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/androidjni/AndroidJNI.md")))]
#[::unity2::class(namespace = "UnityEngine", name = "AndroidJNI")]
#[parent(crate::system::object::Object)]
pub struct AndroidJNI {}

#[cfg(feature = "unity_engine-androidjni")]
#[::unity2::methods]
impl AndroidJNI {
    #[method(name = "FindClass", args = 1)]
    pub fn find_class(name: ::unity2::Il2CppString) -> ::unity2::IntPtr;

    #[method(name = "FromReflectedMethod", args = 1)]
    pub fn from_reflected_method(ref_method: ::unity2::IntPtr) -> ::unity2::IntPtr;

    #[method(name = "ExceptionOccurred", args = 0)]
    pub fn exception_occurred() -> ::unity2::IntPtr;

    #[method(name = "ExceptionClear", args = 0)]
    pub fn exception_clear() -> ();

    #[method(name = "NewGlobalRef", args = 1)]
    pub fn new_global_ref(obj: ::unity2::IntPtr) -> ::unity2::IntPtr;

    #[method(name = "DeleteGlobalRef", args = 1)]
    pub fn delete_global_ref(obj: ::unity2::IntPtr) -> ();

    #[method(name = "NewWeakGlobalRef", args = 1)]
    pub fn new_weak_global_ref(obj: ::unity2::IntPtr) -> ::unity2::IntPtr;

    #[method(name = "DeleteWeakGlobalRef", args = 1)]
    pub fn delete_weak_global_ref(obj: ::unity2::IntPtr) -> ();

    #[method(name = "NewLocalRef", args = 1)]
    pub fn new_local_ref(obj: ::unity2::IntPtr) -> ::unity2::IntPtr;

    #[method(name = "DeleteLocalRef", args = 1)]
    pub fn delete_local_ref(obj: ::unity2::IntPtr) -> ();

    #[method(name = "NewObject", args = 3)]
    pub fn new_object(
        clazz: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> ::unity2::IntPtr;

    #[method(name = "GetObjectClass", args = 1)]
    pub fn get_object_class(obj: ::unity2::IntPtr) -> ::unity2::IntPtr;

    #[method(name = "GetMethodID", args = 3)]
    pub fn get_method_id(
        clazz: ::unity2::IntPtr,
        name: ::unity2::Il2CppString,
        sig: ::unity2::Il2CppString,
    ) -> ::unity2::IntPtr;

    #[method(name = "GetStaticMethodID", args = 3)]
    pub fn get_static_method_id(
        clazz: ::unity2::IntPtr,
        name: ::unity2::Il2CppString,
        sig: ::unity2::Il2CppString,
    ) -> ::unity2::IntPtr;

    #[method(name = "NewString", args = 1)]
    pub fn new_string(chars: ::unity2::Il2CppString) -> ::unity2::IntPtr;

    #[method(name = "NewStringFromStr", args = 1)]
    pub fn new_string_from_str(chars: ::unity2::Il2CppString) -> ::unity2::IntPtr;

    #[method(name = "GetStringChars", args = 1)]
    pub fn get_string_chars(str: ::unity2::IntPtr) -> ::unity2::Il2CppString;

    #[method(name = "CallStringMethod", args = 3)]
    pub fn call_string_method(
        obj: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> ::unity2::Il2CppString;

    #[method(name = "CallObjectMethod", args = 3)]
    pub fn call_object_method(
        obj: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> ::unity2::IntPtr;

    #[method(name = "CallIntMethod", args = 3)]
    pub fn call_int_method(
        obj: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> i32;

    #[method(name = "CallBooleanMethod", args = 3)]
    pub fn call_boolean_method(
        obj: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> bool;

    #[method(name = "CallShortMethod", args = 3)]
    pub fn call_short_method(
        obj: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> i16;

    #[method(name = "CallSByteMethod", args = 3)]
    pub fn call_s_byte_method(
        obj: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> i8;

    #[method(name = "CallCharMethod", args = 3)]
    pub fn call_char_method(
        obj: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> u16;

    #[method(name = "CallFloatMethod", args = 3)]
    pub fn call_float_method(
        obj: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> f32;

    #[method(name = "CallDoubleMethod", args = 3)]
    pub fn call_double_method(
        obj: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> f64;

    #[method(name = "CallLongMethod", args = 3)]
    pub fn call_long_method(
        obj: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> i64;

    #[method(name = "CallStaticStringMethod", args = 3)]
    pub fn call_static_string_method(
        clazz: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> ::unity2::Il2CppString;

    #[method(name = "CallStaticObjectMethod", args = 3)]
    pub fn call_static_object_method(
        clazz: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> ::unity2::IntPtr;

    #[method(name = "CallStaticIntMethod", args = 3)]
    pub fn call_static_int_method(
        clazz: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> i32;

    #[method(name = "CallStaticBooleanMethod", args = 3)]
    pub fn call_static_boolean_method(
        clazz: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> bool;

    #[method(name = "CallStaticShortMethod", args = 3)]
    pub fn call_static_short_method(
        clazz: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> i16;

    #[method(name = "CallStaticSByteMethod", args = 3)]
    pub fn call_static_s_byte_method(
        clazz: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> i8;

    #[method(name = "CallStaticCharMethod", args = 3)]
    pub fn call_static_char_method(
        clazz: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> u16;

    #[method(name = "CallStaticFloatMethod", args = 3)]
    pub fn call_static_float_method(
        clazz: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> f32;

    #[method(name = "CallStaticDoubleMethod", args = 3)]
    pub fn call_static_double_method(
        clazz: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> f64;

    #[method(name = "CallStaticLongMethod", args = 3)]
    pub fn call_static_long_method(
        clazz: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> i64;

    #[method(name = "CallStaticVoidMethod", args = 3)]
    pub fn call_static_void_method(
        clazz: ::unity2::IntPtr,
        method_id: ::unity2::IntPtr,
        args: ::unity2::Array<crate::unity_engine::jvalue::jvalue>,
    ) -> ();

    #[method(name = "ToBooleanArray", args = 1)]
    pub fn to_boolean_array(array: ::unity2::Array<bool>) -> ::unity2::IntPtr;

    #[method(name = "ToByteArray", args = 1)]
    pub fn to_byte_array(array: ::unity2::Array<u8>) -> ::unity2::IntPtr;

    #[method(name = "ToSByteArray", args = 1)]
    pub fn to_s_byte_array(array: ::unity2::Array<i8>) -> ::unity2::IntPtr;

    #[method(name = "ToCharArray", args = 1)]
    pub fn to_char_array(array: ::unity2::Array<u16>) -> ::unity2::IntPtr;

    #[method(name = "ToShortArray", args = 1)]
    pub fn to_short_array(array: ::unity2::Array<i16>) -> ::unity2::IntPtr;

    #[method(name = "ToIntArray", args = 1)]
    pub fn to_int_array(array: ::unity2::Array<i32>) -> ::unity2::IntPtr;

    #[method(name = "ToLongArray", args = 1)]
    pub fn to_long_array(array: ::unity2::Array<i64>) -> ::unity2::IntPtr;

    #[method(name = "ToFloatArray", args = 1)]
    pub fn to_float_array(array: ::unity2::Array<f32>) -> ::unity2::IntPtr;

    #[method(name = "ToDoubleArray", args = 1)]
    pub fn to_double_array(array: ::unity2::Array<f64>) -> ::unity2::IntPtr;

    #[method(name = "ToObjectArray", args = 2)]
    pub fn to_object_array(
        array: ::unity2::Array<::unity2::IntPtr>,
        array_class: ::unity2::IntPtr,
    ) -> ::unity2::IntPtr;

    #[method(name = "FromBooleanArray", args = 1)]
    pub fn from_boolean_array(array: ::unity2::IntPtr) -> ::unity2::Array<bool>;

    #[method(name = "FromByteArray", args = 1)]
    pub fn from_byte_array(array: ::unity2::IntPtr) -> ::unity2::Array<u8>;

    #[method(name = "FromSByteArray", args = 1)]
    pub fn from_s_byte_array(array: ::unity2::IntPtr) -> ::unity2::Array<i8>;

    #[method(name = "FromCharArray", args = 1)]
    pub fn from_char_array(array: ::unity2::IntPtr) -> ::unity2::Array<u16>;

    #[method(name = "FromShortArray", args = 1)]
    pub fn from_short_array(array: ::unity2::IntPtr) -> ::unity2::Array<i16>;

    #[method(name = "FromIntArray", args = 1)]
    pub fn from_int_array(array: ::unity2::IntPtr) -> ::unity2::Array<i32>;

    #[method(name = "FromLongArray", args = 1)]
    pub fn from_long_array(array: ::unity2::IntPtr) -> ::unity2::Array<i64>;

    #[method(name = "FromFloatArray", args = 1)]
    pub fn from_float_array(array: ::unity2::IntPtr) -> ::unity2::Array<f32>;

    #[method(name = "FromDoubleArray", args = 1)]
    pub fn from_double_array(array: ::unity2::IntPtr) -> ::unity2::Array<f64>;

    #[method(name = "GetArrayLength", args = 1)]
    pub fn get_array_length(array: ::unity2::IntPtr) -> i32;

    #[method(name = "NewObjectArray", args = 3)]
    pub fn new_object_array(
        size: i32,
        clazz: ::unity2::IntPtr,
        obj: ::unity2::IntPtr,
    ) -> ::unity2::IntPtr;

    #[method(name = "GetObjectArrayElement", args = 2)]
    pub fn get_object_array_element(array: ::unity2::IntPtr, index: i32) -> ::unity2::IntPtr;

    #[method(name = "SetObjectArrayElement", args = 3)]
    pub fn set_object_array_element(
        array: ::unity2::IntPtr,
        index: i32,
        obj: ::unity2::IntPtr,
    ) -> ();
}
