
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/unity_engine/experimental/rendering/universal/shadowcastergroup2dmanager/ShadowCasterGroup2DManager.md")))]
#[::unity2::class(
    namespace = "UnityEngine.Experimental.Rendering.Universal",
    name = "ShadowCasterGroup2DManager"
)]
#[parent(crate::system::object::Object)]
pub struct ShadowCasterGroup2DManager {
# [static_field] # [rename (name = "s_ShadowCasterGroups")] pub s_shadow_caster_groups : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: experimental :: rendering :: universal :: shadowcastergroup2d :: ShadowCasterGroup2D > ,
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-shadowcastergroup2dmanager")]
#[::unity2::methods]
impl ShadowCasterGroup2DManager {
    #[method(name = "get_shadowCasterGroups", args = 0)]
    pub fn get_shadow_caster_groups () -> crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: experimental :: rendering :: universal :: shadowcastergroup2d :: ShadowCasterGroup2D > ;

    #[method(name = "AddShadowCasterGroupToList", args = 2)]
    pub fn add_shadow_caster_group_to_list(
        shadow_caster : crate :: unity_engine :: experimental :: rendering :: universal :: shadowcastergroup2d :: ShadowCasterGroup2D,
        list : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: experimental :: rendering :: universal :: shadowcastergroup2d :: ShadowCasterGroup2D >,
    ) -> ();

    #[method(name = "RemoveShadowCasterGroupFromList", args = 2)]
    pub fn remove_shadow_caster_group_from_list(
        shadow_caster : crate :: unity_engine :: experimental :: rendering :: universal :: shadowcastergroup2d :: ShadowCasterGroup2D,
        list : crate :: system :: collections :: generic :: list_1 :: List_1 < crate :: unity_engine :: experimental :: rendering :: universal :: shadowcastergroup2d :: ShadowCasterGroup2D >,
    ) -> ();

    #[method(name = "FindTopMostCompositeShadowCaster", args = 1)]
    pub fn find_top_most_composite_shadow_caster (shadow_caster : crate :: unity_engine :: experimental :: rendering :: universal :: shadowcaster2d :: ShadowCaster2D) -> crate :: unity_engine :: experimental :: rendering :: universal :: compositeshadowcaster2d :: CompositeShadowCaster2D ;

    #[method(name = "AddToShadowCasterGroup", args = 2)]
    pub fn add_to_shadow_caster_group(
        shadow_caster : crate :: unity_engine :: experimental :: rendering :: universal :: shadowcaster2d :: ShadowCaster2D,
        shadow_caster_group : crate :: unity_engine :: experimental :: rendering :: universal :: shadowcastergroup2d :: ShadowCasterGroup2D,
    ) -> bool;

    #[method(name = "RemoveFromShadowCasterGroup", args = 2)]
    pub fn remove_from_shadow_caster_group(
        shadow_caster : crate :: unity_engine :: experimental :: rendering :: universal :: shadowcaster2d :: ShadowCaster2D,
        shadow_caster_group : crate :: unity_engine :: experimental :: rendering :: universal :: shadowcastergroup2d :: ShadowCasterGroup2D,
    ) -> ();

    #[method(name = "AddGroup", args = 1)]
    pub fn add_group(
        group : crate :: unity_engine :: experimental :: rendering :: universal :: shadowcastergroup2d :: ShadowCasterGroup2D,
    ) -> ();

    #[method(name = "RemoveGroup", args = 1)]
    pub fn remove_group(
        group : crate :: unity_engine :: experimental :: rendering :: universal :: shadowcastergroup2d :: ShadowCasterGroup2D,
    ) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg(feature = "unity_engine-experimental-rendering-universal-shadowcastergroup2dmanager")]
impl ShadowCasterGroup2DManager {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ShadowCasterGroup2DManager),
                ::core::stringify!(new),
            )
        });
        <Self as IShadowCasterGroup2DManagerMethods>::ctor(this);
        this
    }
}
