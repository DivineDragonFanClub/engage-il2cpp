
use crate::app::procinst::IProcInst;
use crate::app::procinst::ProcInst;
use crate::system::delegate::Delegate;
use crate::system::delegate::IDelegate;
use crate::system::multicastdelegate::IMulticastDelegate;
use crate::system::multicastdelegate::MulticastDelegate;
use crate::system::object::IObject;
use crate::system::object::Object;
use ::unity2::prelude::*;

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardcommentmenu/ProfileCardCommentMenu.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardCommentMenu")]
#[parent(crate::app::procinst::ProcInst)]
pub struct ProfileCardCommentMenu {
    #[rename(name = "m_DisposeEventHandler")]
    pub m_dispose_event_handler:
        crate::app::profilecardcommentmenu::ProfileCardCommentMenu_DisposeEventHandler,
    #[rename(name = "m_MessageSelectMenuContent")]
    pub m_message_select_menu_content:
        crate::app::profilecardmessageselectmenucontent::ProfileCardMessageSelectMenuContent,
    #[rename(name = "m_MessageListMenuContent")]
    pub m_message_list_menu_content:
        crate::app::profilecardmessagelistmenucontent::ProfileCardMessageListMenuContent,
    #[rename(name = "m_ProfileCardRoot")]
    pub m_profile_card_root: crate::app::profilecardroot::ProfileCardRoot,
    #[rename(name = "m_ProfileCardCommentIndexMenu")]
    pub m_profile_card_comment_index_menu:
        crate::app::profilecardcommentindexmenu::ProfileCardCommentIndexMenu,
    #[rename(name = "m_ProfileCardCommentListMenu")]
    pub m_profile_card_comment_list_menu:
        crate::app::profilecardcommentlistmenu::ProfileCardCommentListMenu,
    #[rename(name = "m_MyProfileCardTemp")]
    pub m_my_profile_card_temp: crate::app::profilecard::ProfileCard,
    #[rename(name = "m_CommentTempArray")]
    pub m_comment_temp_array:
        ::unity2::Array<crate::app::profilecardcommentdata::ProfileCardCommentData>,
    #[rename(name = "m_DecidedMessageIndex")]
    pub m_decided_message_index: i32,
}

#[cfg(feature = "app-profilecardcommentmenu")]
#[::unity2::methods]
impl ProfileCardCommentMenu {
    #[method(name = "CreateBind", args = 7)]
    pub fn create_bind(
        super_: crate::app::procinst::ProcInst,
        message_select_menu_content : crate :: app :: profilecardmessageselectmenucontent :: ProfileCardMessageSelectMenuContent,
        message_list_menu_content : crate :: app :: profilecardmessagelistmenucontent :: ProfileCardMessageListMenuContent,
        profile_card_root: crate::app::profilecardroot::ProfileCardRoot,
        my_profile_card_temp: crate::app::profilecard::ProfileCard,
        initial_index: i32,
        dispose_event_handler : crate :: app :: profilecardcommentmenu :: ProfileCardCommentMenu_DisposeEventHandler,
    ) -> crate::app::profilecardcommentmenu::ProfileCardCommentMenu;

    #[method(name = ".ctor", args = 7)]
    pub fn ctor(
        self,
        super_: crate::app::procinst::ProcInst,
        message_select_menu_content : crate :: app :: profilecardmessageselectmenucontent :: ProfileCardMessageSelectMenuContent,
        message_list_menu_content : crate :: app :: profilecardmessagelistmenucontent :: ProfileCardMessageListMenuContent,
        profile_card_root: crate::app::profilecardroot::ProfileCardRoot,
        profile_card: crate::app::profilecard::ProfileCard,
        initial_index: i32,
        dispose_event_handler : crate :: app :: profilecardcommentmenu :: ProfileCardCommentMenu_DisposeEventHandler,
    ) -> ();

    #[method(name = "OnDisposeChildren", args = 0)]
    pub fn on_dispose_children(self) -> ();
}

#[cfg(feature = "app-profilecardcommentmenu")]
impl ProfileCardCommentMenu {
    pub fn new(
        super_: crate::app::procinst::ProcInst,
        message_select_menu_content : crate :: app :: profilecardmessageselectmenucontent :: ProfileCardMessageSelectMenuContent,
        message_list_menu_content : crate :: app :: profilecardmessagelistmenucontent :: ProfileCardMessageListMenuContent,
        profile_card_root: crate::app::profilecardroot::ProfileCardRoot,
        profile_card: crate::app::profilecard::ProfileCard,
        initial_index: i32,
        dispose_event_handler : crate :: app :: profilecardcommentmenu :: ProfileCardCommentMenu_DisposeEventHandler,
    ) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardCommentMenu),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardCommentMenuMethods>::ctor(
            this,
            super_,
            message_select_menu_content,
            message_list_menu_content,
            profile_card_root,
            profile_card,
            initial_index,
            dispose_event_handler,
        );
        this
    }
}

#[cfg_attr(doc, doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/", "docs/app/profilecardcommentmenu/ProfileCardCommentMenu_DisposeEventHandler.md")))]
#[::unity2::class(namespace = "App", name = "ProfileCardCommentMenu.DisposeEventHandler")]
#[parent(crate::system::multicastdelegate::MulticastDelegate)]
pub struct ProfileCardCommentMenu_DisposeEventHandler {}

#[cfg(feature = "app-profilecardcommentmenu")]
#[::unity2::methods]
impl ProfileCardCommentMenu_DisposeEventHandler {
    #[method(name = ".ctor", args = 2)]
    pub fn ctor(self, object: crate::system::object::Object, method: ::unity2::IntPtr) -> ();

    #[method(name = "Invoke", args = 0)]
    pub fn invoke(self) -> ();
}

#[cfg(feature = "app-profilecardcommentmenu")]
impl ProfileCardCommentMenu_DisposeEventHandler {
    pub fn new(object: crate::system::object::Object, method: ::unity2::IntPtr) -> Self {
        let this = <Self as ::unity2::FromIlInstance>::instantiate().unwrap_or_else(|| {
            panic!(
                "{}::{} failed to instantiate",
                ::core::stringify!(ProfileCardCommentMenu_DisposeEventHandler),
                ::core::stringify!(new),
            )
        });
        <Self as IProfileCardCommentMenu_DisposeEventHandlerMethods>::ctor(this, object, method);
        this
    }
}
