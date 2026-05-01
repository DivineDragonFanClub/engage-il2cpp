
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/kaneko/Kaneko.md")))]
#[::unity2::class(namespace = "Combat", name = "Kaneko")]
#[parent(crate::system::object::Object)]
pub struct Kaneko {
    #[static_field]
    #[rename(name = "Epsilon")]
    pub epsilon: f32,
    #[static_field]
    #[rename(name = "s_DummyDisposable")]
    pub s_dummy_disposable: crate::combat::kaneko::Kaneko_DummyDisposable,
}

#[cfg(feature = "combat-kaneko")]
#[::unity2::methods]
impl Kaneko {
    #[method(name = "FindInChildren", args = 2)]
    pub fn find_in_children(
        self_: crate::unity_engine::transform::Transform,
        name: ::unity2::Il2CppString,
    ) -> crate::unity_engine::transform::Transform;

    #[method(name = "FindWordInChildren", args = 2)]
    pub fn find_word_in_children(
        self_: crate::unity_engine::transform::Transform,
        keyword: ::unity2::Il2CppString,
    ) -> crate::unity_engine::transform::Transform;

    #[method(name = "SetPositionAndForward", args = 3)]
    pub fn set_position_and_forward(
        t: crate::unity_engine::transform::Transform,
        position: crate::unity_engine::vector3::Vector3,
        forward: crate::unity_engine::vector3::Vector3,
    ) -> ();

    #[method(name = "Ancestor", args = 1)]
    pub fn ancestor(
        t: crate::unity_engine::transform::Transform,
    ) -> crate::unity_engine::transform::Transform;

    #[method(name = "Startup", args = 0)]
    pub fn startup() -> ();

    #[method(name = "Shutdown", args = 0)]
    pub fn shutdown() -> ();

    #[method(name = "Destroy", args = 1)]
    pub fn destroy(obj: crate::unity_engine::object_2::Object_2) -> ();

    #[method(name = "DestroyComponent", args = 1)]
    pub fn destroy_component(obj: crate::unity_engine::component::Component) -> ();

    #[method(name = "IsDestroyed", args = 1)]
    pub fn is_destroyed(obj: crate::unity_engine::object_2::Object_2) -> bool;

    #[method(name = "SelectOne", args = 1)]
    pub fn select_one(arg: ::unity2::Il2CppString) -> ::unity2::Il2CppString;

    #[method(name = "InBetween", args = 3)]
    pub fn in_between(s: ::unity2::Il2CppString, lc: u16, rc: u16) -> ::unity2::Il2CppString;

    #[method(name = "LineToList", args = 1)]
    pub fn line_to_list(
        line: ::unity2::Il2CppString,
    ) -> crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>;

    #[method(name = "ListToLine", args = 1)]
    pub fn list_to_line(
        list: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    ) -> ::unity2::Il2CppString;

    #[method(name = "EachChildren", args = 2)]
    pub fn each_children(
        self_: crate::unity_engine::transform::Transform,
        callback: crate::system::func_2::Func_2<crate::unity_engine::transform::Transform, bool>,
    ) -> ();

    #[method(name = "Hermite", args = 5)]
    pub fn hermite(v0: f32, t0: f32, v1: f32, t1: f32, s: f32) -> f32;

    #[method(name = "Hermite", args = 5)]
    pub fn hermite_2(
        v0: crate::unity_engine::vector3::Vector3,
        t0: crate::unity_engine::vector3::Vector3,
        v1: crate::unity_engine::vector3::Vector3,
        t1: crate::unity_engine::vector3::Vector3,
        s: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Lerp", args = 5)]
    pub fn lerp(
        v0: crate::unity_engine::vector3::Vector3,
        t0: crate::unity_engine::vector3::Vector3,
        v1: crate::unity_engine::vector3::Vector3,
        t1: crate::unity_engine::vector3::Vector3,
        s: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "CatmullRom", args = 5)]
    pub fn catmull_rom(
        p0: crate::unity_engine::vector3::Vector3,
        p1: crate::unity_engine::vector3::Vector3,
        p2: crate::unity_engine::vector3::Vector3,
        p3: crate::unity_engine::vector3::Vector3,
        s: f32,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "Sigmoid", args = 2)]
    pub fn sigmoid(x: f32, a: f32) -> f32;

    #[method(name = "QuaternionFromNormal", args = 1)]
    pub fn quaternion_from_normal(
        yaxis: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::quaternion::Quaternion;

    #[method(name = "fixedTime", args = 1)]
    pub fn fixed_time(state_info: crate::unity_engine::animatorstateinfo::AnimatorStateInfo)
        -> f32;

    #[method(name = "fixedLength", args = 1)]
    pub fn fixed_length(
        state_info: crate::unity_engine::animatorstateinfo::AnimatorStateInfo,
    ) -> f32;

    #[method(name = "timeLength", args = 1)]
    pub fn time_length(c: crate::unity_engine::animationcurve::AnimationCurve) -> f32;

    #[method(name = "IsNull", args = 1)]
    pub fn is_null(s: ::unity2::Il2CppString) -> bool;

    #[method(name = "IsNotNull", args = 1)]
    pub fn is_not_null(s: ::unity2::Il2CppString) -> bool;

    #[method(name = "GetOnlyOne", args = 1)]
    pub fn get_only_one(
        l: crate::system::collections::generic::list_1::List_1<::unity2::Il2CppString>,
    ) -> ::unity2::Il2CppString;

    #[method(name = "IsZero", args = 1)]
    pub fn is_zero(v: crate::unity_engine::vector3::Vector3) -> bool;

    #[method(name = "IsNotZero", args = 1)]
    pub fn is_not_zero(v: crate::unity_engine::vector3::Vector3) -> bool;

    #[method(name = "CallEditorStaticFunction", args = 3)]
    pub fn call_editor_static_function(
        class_name: ::unity2::Il2CppString,
        func_name: ::unity2::Il2CppString,
        args: ::unity2::Array<crate::system::object::Object>,
    ) -> ();

    #[method(name = "GetManhattanDistance", args = 4)]
    pub fn get_manhattan_distance(x0: i32, y0: i32, x1: i32, y1: i32) -> i32;

    #[method(name = "RoundupTime", args = 1)]
    pub fn roundup_time(time: f32) -> f32;

    #[method(name = "RounddownTime", args = 1)]
    pub fn rounddown_time(time: f32) -> f32;

    #[method(name = "Assert", args = 5)]
    pub fn assert(
        exp: bool,
        msg: ::unity2::Il2CppString,
        filename: ::unity2::Il2CppString,
        line: i32,
        funcname: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "Assert", args = 4)]
    pub fn assert_2(
        msg: ::unity2::Il2CppString,
        filename: ::unity2::Il2CppString,
        line: i32,
        funcname: ::unity2::Il2CppString,
    ) -> ();

    #[method(name = "LimitGroundNormal", args = 1)]
    pub fn limit_ground_normal(
        nrm: crate::unity_engine::vector3::Vector3,
    ) -> crate::unity_engine::vector3::Vector3;

    #[method(name = "GUIFitToScreen", args = 0)]
    pub fn gui_fit_to_screen() -> ();

    #[method(name = "GUIReset", args = 0)]
    pub fn gui_reset() -> ();

    #[method(name = "GetFov", args = 1)]
    pub fn get_fov(cam: crate::unity_engine::component::Component) -> f32;

    #[method(name = ".cctor", args = 0)]
    pub fn cctor() -> ();
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/kaneko/Kaneko_GUIFitScope.md")))]
#[::unity2::class(namespace = "Combat", name = "Kaneko.GUIFitScope")]
#[parent(crate::system::object::Object)]
pub struct Kaneko_GUIFitScope {}

#[cfg(feature = "combat-kaneko")]
#[::unity2::methods]
impl Kaneko_GUIFitScope {
    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();

    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();
}

#[cfg(feature = "combat-kaneko")]
impl Kaneko_GUIFitScope {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Kaneko_GUIFitScope),
                ::core::stringify!(new),
            )
        });
        <Self as IKaneko_GUIFitScopeMethods>::ctor(this);
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/kaneko/Kaneko_Screen.md")))]
#[::unity2::class(namespace = "Combat", name = "Kaneko.Screen")]
#[parent(crate::system::object::Object)]
pub struct Kaneko_Screen {
    #[static_field]
    #[rename(name = "width")]
    pub width: f32,
    #[static_field]
    #[rename(name = "height")]
    pub height: f32,
}

#[cfg(feature = "combat-kaneko")]
#[::unity2::methods]
impl Kaneko_Screen {
    #[method(name = "get_WH", args = 0)]
    pub fn get_wh() -> crate::unity_engine::vector2::Vector2;
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/combat/kaneko/Kaneko_DummyDisposable.md")))]
#[::unity2::class(namespace = "Combat", name = "Kaneko.DummyDisposable")]
#[parent(crate::system::object::Object)]
pub struct Kaneko_DummyDisposable {}

#[cfg(feature = "combat-kaneko")]
#[::unity2::methods]
impl Kaneko_DummyDisposable {
    #[method(name = "Dispose", args = 0)]
    pub fn dispose(self) -> ();

    #[method(name = ".ctor", args = 0)]
    pub fn ctor(self) -> ();
}

#[cfg(feature = "combat-kaneko")]
impl Kaneko_DummyDisposable {
    pub fn new() -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(Kaneko_DummyDisposable),
                ::core::stringify!(new),
            )
        });
        <Self as IKaneko_DummyDisposableMethods>::ctor(this);
        this
    }
}
