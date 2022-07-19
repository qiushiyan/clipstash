//! Page routing, errors, and data structures.

use crate::data::AppDatabase;
use crate::service;
use crate::service::action;
use crate::web::{ctx, form, renderer::Renderer, PageError};
use crate::{ServiceError, ShortCode};
use rocket::form::{Contextual, Form};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::content::RawHtml;

use rocket::response::{status, Redirect};
use rocket::{uri, State};

#[rocket::get("/")]
fn home(renderer: &State<Renderer<'_>>) -> RawHtml<String> {
    let context = ctx::Home::default();
    RawHtml(renderer.render(&context, &[]))
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![home]
}

pub mod catcher {
    use rocket::Request;
    use rocket::{catch, catchers, Catcher};

    #[catch(default)]
    fn default(req: &Request) -> &'static str {
        "something went wrong"
    }

    #[catch(500)]
    fn internal_error(req: &Request) -> &'static str {
        eprintln!("Internal error: {:?}", req);
        "internal server error"
    }

    /// Catch missing data errors.
    #[catch(404)]
    fn not_found() -> &'static str {
        "404"
    }

    /// The [`catchers`](rocket::Catcher) which can be registered by [`rocket`].
    pub fn catchers() -> Vec<Catcher> {
        catchers![not_found, default, internal_error]
    }
}
