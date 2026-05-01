
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/moon_sharp/interpreter/interop/igeneratoruserdatadescriptor/IGeneratorUserDataDescriptor.md")))]
#[::unity2::class(
    namespace = "MoonSharp.Interpreter.Interop",
    name = "IGeneratorUserDataDescriptor"
)]
pub struct IGeneratorUserDataDescriptor {}

#[cfg(feature = "moon_sharp-interpreter-interop-igeneratoruserdatadescriptor")]
#[::unity2::methods]
impl IGeneratorUserDataDescriptor {
    #[method(name = "Generate", args = 1)]
    pub fn generate(
        self,
        r#type: ::unity2::SystemType,
    ) -> crate::moon_sharp::interpreter::interop::iuserdatadescriptor::IUserDataDescriptor;
}
