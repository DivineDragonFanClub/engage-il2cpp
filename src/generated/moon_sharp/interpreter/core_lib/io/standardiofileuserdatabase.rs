
use crate::moon_sharp::interpreter::core_lib::io::fileuserdatabase::FileUserDataBase;
use crate::moon_sharp::interpreter::core_lib::io::fileuserdatabase::IFileUserDataBase;
use crate::moon_sharp::interpreter::core_lib::io::streamfileuserdatabase::IStreamFileUserDataBase;
use crate::moon_sharp::interpreter::core_lib::io::streamfileuserdatabase::StreamFileUserDataBase;
use crate::moon_sharp::interpreter::refidobject::IRefIdObject;
use crate::moon_sharp::interpreter::refidobject::RefIdObject;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/core_lib/io/standardiofileuserdatabase/StandardIOFileUserDataBase.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.CoreLib.IO",
    name = "StandardIOFileUserDataBase"
)]
#[parent(
    crate::moon_sharp::interpreter::core_lib::io::streamfileuserdatabase::StreamFileUserDataBase
)]
pub struct StandardIOFileUserDataBase {}

#[cfg(feature = "moon_sharp-interpreter-core_lib-io-standardiofileuserdatabase")]
#[::unity2::methods]
impl StandardIOFileUserDataBase {
    #[method(name = "Close", args = 0)]
    pub fn close(self) -> ::unity2::Il2CppString;

    #[method(name = "CreateInputStream", args = 1)]
    pub fn create_input_stream (stream : crate :: system :: io :: stream :: Stream) -> crate :: moon_sharp :: interpreter :: core_lib :: io :: standardiofileuserdatabase :: StandardIOFileUserDataBase ;

    #[method(name = "CreateOutputStream", args = 1)]
    pub fn create_output_stream (stream : crate :: system :: io :: stream :: Stream) -> crate :: moon_sharp :: interpreter :: core_lib :: io :: standardiofileuserdatabase :: StandardIOFileUserDataBase ;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-core_lib-io-standardiofileuserdatabase")]
impl StandardIOFileUserDataBase {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(StandardIOFileUserDataBase),
                ::core::stringify!(new),
            )
        });
        <Self as IStandardIOFileUserDataBaseMethods>::ctor(this);
        this
    }
}
