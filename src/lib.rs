//! # rocket-response - Provides enum for variable Rocket Responses
//!
//! This [crate] provides 3 enums to be flexible in returning [Responses].
//!
//! 1. [RocketResponse] provides all non-generic [Response] types.
//! 2. [RocketResponseGeneric] provides [Response]-types non-generic
//!    and generic using a single generic type.
//! 3. [RocketResponseGeneric2] allows a different [Flash](rocket::response::Flash) type.
//!
//! If you miss any [Response], you are welcome to open an [issue]
//! or even better provide a pull-request!
//!
//! Because it is difficult to decide on the generics what might be useful, your usecases are really
//! welcome in an [issue].
//!
//! ## Usage
//!
//! For usage add the crate to your dependencies
//!
//! ```toml
//! [dependencies]
//! rocket-response = { version = "0.0.1-rc.2" }
//! ```
//!
//! ## Features
//!
//! You can depend on a couple of features, which provide additional types.
//!
//! * json
//! * msgpack
//! * templates-handlebars or templates-tera
//!
//! ```toml
//! [dependencies]
//! rocket-response = { version = "0.0.1-rc.2", features = ["json", "templates-tera"] }
//! ```
//!
//!
//! [Response]: rocket::response::Response
//! [Responses]: rocket::response::Response
//! [issue]: https://github.com/kolbma/rocket-response/issues

#![deny(unsafe_code)]
#![deny(warnings)]
#![deny(clippy::all)]
#![deny(missing_docs)]
#![deny(rustdoc::missing_doc_code_examples)]

#[cfg(any(feature = "json", feature = "msgpack"))]
use rocket::serde;
use rocket::{
    fs::NamedFile,
    http::Status,
    response::{
        content::{RawCss, RawHtml, RawJavaScript, RawJson, RawMsgPack, RawText, RawXml},
        status::{
            Accepted, BadRequest, Conflict, Created, Forbidden, NoContent, NotFound, Unauthorized,
        },
        Flash, Redirect,
    },
    serde::Serialize,
    tokio, Responder,
};
#[cfg(any(feature = "templates-tera", feature = "templates-handlebars"))]
use rocket_dyn_templates::Template;
use std::fs::File;

/// The non-generic [Responses](rocket::response::Response).
///
/// ## Example usage
///
/// ```rust
/// use rocket::{get, response::{self, Redirect}};
/// use rocket_response::RocketResponse;
///
/// #[get("/<id>")]
/// pub(crate) fn route_example(id: usize) -> RocketResponse {
///     match id {
///         0 => RocketResponse::NoContent(response::status::NoContent),
///         1 => RocketResponse::Redirect(Redirect::to("/admin")),
///         _ => RocketResponse::StaticStr("Hello world"),
///     }
/// }
/// ```
#[derive(Responder)]
pub enum RocketResponse {
    /// see [rocket::response::status::Accepted]
    Accepted(Accepted<&'static str>),
    /// see [rocket::response::status::BadRequest]
    BadRequest(BadRequest<&'static str>),
    /// see [rocket::response::status::Conflict]
    Conflict(Conflict<&'static str>),
    /// see [rocket::response::status::Created]
    Created(Created<&'static str>),
    /// see [rocket::response::content::RawCss]
    Css(RawCss<&'static str>),
    /// see [File]
    File(File),
    /// see [rocket::response::Flash]
    Flash(Flash<&'static str>),
    /// see [rocket::response::status::Forbidden]
    Forbidden(Forbidden<&'static str>),
    /// see [rocket::response::content::RawHtml]
    Html(RawHtml<&'static str>),
    /// see [rocket::response::content::RawJavaScript]
    JavaScript(RawJavaScript<&'static str>),
    /// see [rocket::response::content::RawJson]
    Json(RawJson<&'static str>),
    /// see [rocket::response::content::RawMsgPack]
    MsgPack(RawMsgPack<&'static str>),
    /// see [NamedFile](rocket::fs::NamedFile)
    NamedFiled(NamedFile),
    /// see [rocket::response::status::NotFound]
    NotFound(NotFound<&'static str>),
    /// see [NoContent](rocket::response::status::NoContent)
    NoContent(NoContent),
    /// see [rocket::response::content::RawText]
    Plain(RawText<&'static str>),
    /// see [Redirect](rocket::response::Redirect)
    Redirect(Redirect),

    #[cfg(feature = "json")]
    /// see [rocket::serde::json::Json]
    SerdeJson(serde::json::Json<&'static str>),
    #[cfg(feature = "msgpack")]
    /// see [rocket::serde::msgpack::MsgPack]
    SerdeMsgPack(serde::msgpack::MsgPack<&'static str>),
    #[cfg(feature = "json")]
    /// see [Value](rocket::serde::json::Value)
    SerdeValue(serde::json::Value),

    /// see [slice](std::slice)
    StaticSlice(&'static [u8]),
    /// see [str]
    StaticStr(&'static str),
    /// see [String]
    String(String),
    /// see [Status](rocket::http::Status)
    Status(Status),

    #[cfg(any(feature = "templates-tera", feature = "templates-handlebars"))]
    /// see [Template](rocket_dyn_templates::Template)
    Template(Template),

    /// see [File](rocket::tokio::fs::File)
    TokioFile(tokio::fs::File),
    /// see [Unauthorized](rocket::response::status::Unauthorized)
    Unauthorized(Unauthorized<&'static str>),
    /// see [Vec](std::vec::Vec)
    Vec(Vec<u8>),
    /// see [Xml](rocket::response::content::RawXml)
    Xml(RawXml<&'static str>),
}

/// The non-generic and generic [Responses](rocket::response::Response) with a single type.
///
/// ## Example usage
///
/// ```rust
/// use rocket::{get, response::{self, status, Redirect}};
/// use rocket_response::RocketResponseGeneric as RocketResponse;
///
/// #[get("/<id>")]
/// pub(crate) fn route_response_generic(id: usize) -> RocketResponse<&'static str> {
///     match id {
///         0 => RocketResponse::NoContent(status::NoContent),
///         1 => RocketResponse::Unauthorized(status::Unauthorized(Some(
///             "admin need authentication",
///         ))),
///         _ => RocketResponse::Html(response::content::RawHtml(
///            "<html><body>Hello world</body></html",
///         )),
///     }
/// }
/// ```
#[derive(Responder)]
pub enum RocketResponseGeneric<T>
where
    T: Serialize,
{
    /// see [rocket::response::status::Accepted]
    Accepted(Accepted<T>),
    /// see [rocket::response::status::BadRequest]
    BadRequest(BadRequest<T>),
    /// see [rocket::response::status::Conflict]
    Conflict(Conflict<T>),
    /// see [rocket::response::status::Created]
    Created(Created<T>),
    /// see [rocket::response::content::RawCss]
    Css(RawCss<T>),
    /// see [File]
    File(File),
    /// see [rocket::response::Flash]
    Flash(Flash<T>),
    /// see [rocket::response::status::Forbidden]
    Forbidden(Forbidden<T>),
    /// see [rocket::response::content::RawHtml]
    Html(RawHtml<T>),
    /// see [rocket::response::content::RawJavaScript]
    JavaScript(RawJavaScript<T>),
    /// see [rocket::response::content::RawJson]
    Json(RawJson<T>),
    /// see [rocket::response::content::RawMsgPack]
    MsgPack(RawMsgPack<T>),
    /// see [NamedFile](rocket::fs::NamedFile)
    NamedFiled(NamedFile),
    /// see [rocket::response::status::NotFound]
    NotFound(NotFound<T>),
    /// see [NoContent](rocket::response::status::NoContent)
    NoContent(NoContent),
    /// see [rocket::response::content::RawText]
    Plain(RawText<T>),
    /// see [Redirect](rocket::response::Redirect)
    Redirect(Redirect),

    #[cfg(feature = "json")]
    /// see [rocket::serde::json::Json]
    SerdeJson(serde::json::Json<T>),
    #[cfg(feature = "msgpack")]
    /// see [rocket::serde::msgpack::MsgPack]
    SerdeMsgPack(serde::msgpack::MsgPack<T>),
    #[cfg(feature = "json")]
    /// see [Value](rocket::serde::json::Value)
    SerdeValue(serde::json::Value),

    /// see [slice](std::slice)
    StaticSlice(&'static [u8]),
    /// see [str]
    StaticStr(&'static str),
    /// see [String]
    String(String),
    /// see [Status](rocket::http::Status)
    Status(Status),

    #[cfg(any(feature = "templates-tera", feature = "templates-handlebars"))]
    /// see [Template](rocket_dyn_templates::Template)
    Template(Template),

    /// see [File](rocket::tokio::fs::File)
    TokioFile(tokio::fs::File),
    /// see [Unauthorized](rocket::response::status::Unauthorized)
    Unauthorized(Unauthorized<T>),
    /// see [Vec](std::vec::Vec)
    Vec(Vec<u8>),
    /// see [Xml](rocket::response::content::RawXml)
    Xml(RawXml<T>),
}

/// The non-generic and generic [Responses](rocket::response::Response) with 2 types.
///
/// ## Example usage
///
/// ```rust
/// use rocket::{get, response::{self, status, Redirect}};
/// use rocket_response::RocketResponseGeneric2 as RocketResponse;
///
/// #[get("/<id>")]
/// pub(crate) fn rocket_response_generic2(
///     id: usize,
/// ) -> RocketResponse<&'static str, Redirect> {
///     match id {
///         0 => RocketResponse::Flash(response::Flash::error(
///           Redirect::to("/"),
///             format!("Invalid id {}", id),
///         )),
///         1 => RocketResponse::Unauthorized(status::Unauthorized(Some(
///             "admin need authentication",
///         ))),
///         _ => RocketResponse::Html(response::content::RawHtml(
///             "<html><body>Hello world</body></html",
///         )),
///     }
/// }
/// ```
#[derive(Responder)]
pub enum RocketResponseGeneric2<T, U>
where
    T: Serialize,
{
    /// see [rocket::response::status::Accepted]
    Accepted(Accepted<T>),
    /// see [rocket::response::status::BadRequest]
    BadRequest(BadRequest<T>),
    /// see [rocket::response::status::Conflict]
    Conflict(Conflict<T>),
    /// see [rocket::response::status::Created]
    Created(Created<T>),
    /// see [rocket::response::content::RawCss]
    Css(RawCss<T>),
    /// see [File]
    File(File),
    /// with generic type U  
    /// see [rocket::response::Flash]
    Flash(Flash<U>),
    /// see [rocket::response::status::Forbidden]
    Forbidden(Forbidden<T>),
    /// see [rocket::response::content::RawHtml]
    Html(RawHtml<T>),
    /// see [rocket::response::content::RawJavaScript]
    JavaScript(RawJavaScript<T>),
    /// see [rocket::response::content::RawJson]
    Json(RawJson<T>),
    /// see [rocket::response::content::RawMsgPack]
    MsgPack(RawMsgPack<T>),
    /// see [NamedFile](rocket::fs::NamedFile)
    NamedFiled(NamedFile),
    /// see [NoContent](rocket::response::status::NoContent)
    NotFound(NotFound<T>),
    /// see [rocket::response::status::NoContent]
    NoContent(NoContent),
    /// see [rocket::response::content::RawText]
    Plain(RawText<T>),
    /// see [rocket::response::Redirect]
    Redirect(Redirect),

    #[cfg(feature = "json")]
    /// see [rocket::serde::json::Json]
    SerdeJson(serde::json::Json<T>),
    #[cfg(feature = "msgpack")]
    /// see [rocket::serde::msgpack::MsgPack]
    SerdeMsgPack(serde::msgpack::MsgPack<T>),
    #[cfg(feature = "json")]
    /// see [rocket::serde::json::Value]
    SerdeValue(serde::json::Value),

    /// see [slice](std::slice)
    StaticSlice(&'static [u8]),
    /// see [str]
    StaticStr(&'static str),
    /// see [String]
    String(String),
    /// see [Status](rocket::http::Status)
    Status(Status),

    #[cfg(any(feature = "templates-tera", feature = "templates-handlebars"))]
    /// see [Template](rocket_dyn_templates::Template)
    Template(Template),

    /// see [File](rocket::tokio::fs::File)
    TokioFile(tokio::fs::File),
    /// see [Unauthorized](rocket::response::status::Unauthorized)
    Unauthorized(Unauthorized<T>),
    /// see [Vec](std::vec::Vec)
    Vec(Vec<u8>),
    /// see [Xml](rocket::response::content::RawXml)
    Xml(RawXml<T>),
}

#[cfg(test)]
mod tests {
    use super::{RocketResponse, RocketResponseGeneric, RocketResponseGeneric2};
    use rocket::{
        get,
        http::ContentType,
        http::Status,
        local::blocking::Client,
        response::{self, status, Redirect},
        routes,
    };

    #[get("/response/<id>")]
    pub(crate) fn route_response(id: usize) -> RocketResponse {
        match id {
            0 => RocketResponse::NoContent(response::status::NoContent),
            1 => RocketResponse::Redirect(Redirect::to("/admin")),
            _ => RocketResponse::StaticStr("Hello world"),
        }
    }

    #[get("/response_generic/<id>")]
    pub(crate) fn route_response_generic(id: usize) -> RocketResponseGeneric<&'static str> {
        match id {
            0 => RocketResponseGeneric::NoContent(status::NoContent),
            1 => RocketResponseGeneric::Unauthorized(status::Unauthorized(Some(
                "admin need authentication",
            ))),
            _ => RocketResponseGeneric::Html(response::content::RawHtml(
                "<html><body>Hello world</body></html",
            )),
        }
    }

    #[get("/response_generic2/<id>")]
    pub(crate) fn route_response_generic2(
        id: usize,
    ) -> RocketResponseGeneric2<&'static str, Redirect> {
        match id {
            0 => RocketResponseGeneric2::Flash(response::Flash::error(
                Redirect::to("/"),
                format!("Invalid id {}", id),
            )),
            1 => RocketResponseGeneric2::Unauthorized(status::Unauthorized(Some(
                "admin need authentication",
            ))),
            _ => RocketResponseGeneric2::Html(response::content::RawHtml(
                "<html><body>Hello world</body></html",
            )),
        }
    }

    #[test]
    fn test_rocket_response() {
        let rocket = rocket::build().mount("/", routes![route_response]);
        let client = Client::tracked(rocket).expect("no rocket instance");
        let req = client.get("/response/2");
        let res = req.dispatch();

        assert_eq!(Status::Ok, res.status());
        assert_eq!(ContentType::Plain, res.content_type().unwrap());
    }

    #[test]
    fn test_rocket_response_generic() {
        let rocket = rocket::build().mount("/", routes![route_response_generic]);
        let client = Client::tracked(rocket).expect("no rocket instance");
        let req = client.get("/response_generic/0");
        let res = req.dispatch();

        assert_eq!(Status::NoContent, res.status());
    }

    #[test]
    fn test_rocket_response_generic2() {
        let rocket = rocket::build().mount("/", routes![route_response_generic2]);
        let client = Client::tracked(rocket).expect("no rocket instance");
        let req = client.get("/response_generic2/0");
        let res = req.dispatch();

        assert_eq!(Status::SeeOther, res.status());
    }
}
