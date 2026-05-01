
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/io/bindumpbinarywriter/BinDumpBinaryWriter.md")))]
#[::unity2::class(namespace = "MoonSharp.Interpreter.IO", name = "BinDumpBinaryWriter")]
pub struct BinDumpBinaryWriter {
    #[rename(name = "m_StringMap")]
    pub m_string_map: crate::system::collections::generic::dictionary_2::Dictionary_2<
        ::unity2::Il2CppString,
        i32,
    >,
}

#[cfg(feature = "moon_sharp-interpreter-io-bindumpbinarywriter")]
#[::unity2::methods]
impl BinDumpBinaryWriter {
    #[method(name = ".ctor", args = 1)]
    pub fn ctor(self, s: crate::system::io::stream::Stream) -> ();

    #[method(name = "Write", args = 1)]
    pub fn write(self, value: u32) -> ();

    #[method(name = "Write", args = 1)]
    pub fn write_2(self, value: i32) -> ();

    #[method(name = "Write", args = 1)]
    pub fn write_3(self, value: ::unity2::Il2CppString) -> ();
}

#[cfg(feature = "moon_sharp-interpreter-io-bindumpbinarywriter")]
impl BinDumpBinaryWriter {
    pub fn new(s: crate::system::io::stream::Stream) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(BinDumpBinaryWriter),
                ::core::stringify!(new),
            )
        });
        <Self as IBinDumpBinaryWriterMethods>::ctor(this, s);
        this
    }
}
