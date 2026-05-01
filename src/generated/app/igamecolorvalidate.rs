
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/igamecolorvalidate/IGameColorValidate.md")))]
#[::unity2::class(namespace = "App", name = "IGameColorValidate")]
pub struct IGameColorValidate {}

#[cfg(feature = "app-igamecolorvalidate")]
#[::unity2::methods]
impl IGameColorValidate {
    #[method(name = "OnGameColorValidate", args = 0)]
    pub fn on_game_color_validate(self) -> ();
}
