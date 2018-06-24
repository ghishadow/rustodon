use askama::Template;
use db;
use db::models::{Account, Status};
use rocket::request::FlashMessage;
use routes::ui::view_helpers::*;
use std::ops::Deref;

macro_rules! HtmlTemplate {
    ($x:tt) => {{
        $x {
            _parent: BaseTemplate {
                revision: ::GIT_REV,
            }
        }
    }};

    ($x:tt, $flash: ident) => {{
        $x {
            _parent: BaseTemplate {
                flash: $flash,
                revision: ::GIT_REV,
            }
        }
    }};

    ($x:tt, { $( $y:ident : $z:expr ),* }) => {{
        $x {
            $( $y: $z ),*
            ,_parent: BaseTemplate {
                flash: None,
                revision: ::GIT_REV,
            }
        }
    }};
    ($x:tt, $flash: ident, { $( $y:ident : $z:expr ),* }) => {{
        $x {
            $( $y: $z ),*
            ,_parent: BaseTemplate {
                flash: $flash,
                revision: ::GIT_REV,
            }
        }
    }};
}

macro_rules! PerhapsHtmlTemplate {
    ($x:tt) => {{
        Ok(Some($x {
            _parent: BaseTemplate {
                revision: ::GIT_REV,
            }
        }))
    }};

    ($x:tt, $flash: ident) => {{
        Ok(Some($x {
            _parent: BaseTemplate {
                flash: $flash,
                revision: ::GIT_REV,
            }
        }))
    }};

    ($x:tt, { $( $y:ident : $z:expr ),* }) => {{
        {
            let tmpl = $x {
                $( $y: $z ),*
                ,_parent: BaseTemplate {
                    flash: None,
                    revision: ::GIT_REV,
                }
            };
            Ok(Some(tmpl))
        }
    }};

    ($x:tt, $flash: ident, { $( $y:ident : $z:expr ),* }) => {{
        {
            let tmpl = $x {
                $( $y: $z ),*
                ,_parent: BaseTemplate {
                    flash: $flash,
                    revision: ::GIT_REV,
                }
            };
            Ok(Some(tmpl))
        }
    }};
}

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate<'a> {
    pub revision: &'a str,
    pub flash:    Option<FlashMessage>,
}

#[derive(Template)]
#[template(path = "status.html")]
pub struct StatusTemplate<'a> {
    pub status:  Status,
    pub account: Account,
    pub _parent: BaseTemplate<'a>,
}

#[derive(Template)]
#[template(path = "user.html")]
pub struct UserTemplate<'a> {
    pub account_to_show: Account,
    pub account: Option<Account>,
    pub statuses: Vec<Status>,
    pub prev_page_id: Option<i64>,
    pub connection: db::Connection,
    pub _parent: BaseTemplate<'a>,
}

#[derive(Template)]
#[template(path = "edit_profile.html")]
pub struct EditProfileTemplate<'a> {
    pub account: Account,
    pub _parent: BaseTemplate<'a>,
}

#[derive(Template)]
#[template(path = "signin.html")]
pub struct SigninTemplate<'a> {
    pub _parent: BaseTemplate<'a>,
}

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignupTemplate<'a> {
    pub _parent: BaseTemplate<'a>,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub account: Option<Account>,
    pub _parent: BaseTemplate<'a>,
}
