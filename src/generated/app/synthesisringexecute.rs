
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/synthesisringexecute/SynthesisRingExecute.md")))]
#[::unity2::class(namespace = "App", name = "SynthesisRingExecute")]
#[parent(crate::system::object::Object)]
pub struct SynthesisRingExecute {}

#[cfg(feature = "app-synthesisringexecute")]
#[::unity2::methods]
impl SynthesisRingExecute {
    #[method(name = "Do", args = 3)]
    pub fn r#do(
        base_ring_rnid: ::unity2::Il2CppString,
        count_base_ring: i32,
        count_piece_of_bonds: i32,
    ) -> bool;

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "app-synthesisringexecute")]
impl SynthesisRingExecute {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(SynthesisRingExecute),
                ::core::stringify!(new),
            )
        });
        <Self as ISynthesisRingExecuteMethods>::ctor(this);
        this
    }
}
