//! Page routing, errors, and data structures.

use super::hit_counter::HitCounter;
use crate::data::AppDatabase;
use crate::service;
use crate::service::action;
use crate::web::PASSWORD_COOKIE;
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

#[rocket::get("/key/new")]
fn api_key(renderer: &State<Renderer<'_>>) -> RawHtml<String> {
    let context = ctx::ApiKeyGenerate::default();
    RawHtml(renderer.render(&context, &[]))
}

#[rocket::post("/key/new")]
pub async fn generate_api_key(
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>,
) -> Result<status::Custom<RawHtml<String>>, PageError> {
    println!("Generating new api key...");
    match action::generate_api_key(database.get_pool()).await {
        Ok(api_key) => {
            println!("Api Key: {}", api_key.to_base64());
            let context = ctx::ApiKeyGenerate::default();
            Ok(status::Custom(
                Status::Ok,
                RawHtml(renderer.render_with_data(context, ("api_key", api_key.to_base64()), &[])),
            ))
        }
        Err(e) => {
            dbg!(&e);
            Err(PageError::InternalError(format!("{}", e)))
        }
    }
}

#[rocket::post("/", data = "<form>")]
pub async fn new_clip(
    form: Form<Contextual<'_, form::NewClip>>,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>,
) -> Result<Redirect, (Status, RawHtml<String>)> {
    let form = form.into_inner();
    if let Some(value) = form.value {
        let req = service::ask::NewClip {
            title: value.title,
            content: value.content,
            password: value.password,
            expires_at: value.expires_at,
        };

        match action::new_clip(req, database.get_pool()).await {
            Ok(clip) => Ok(Redirect::to(uri!(get_clip(clip.shortcode)))),
            Err(_) => Err((
                Status::InternalServerError,
                RawHtml(renderer.render(
                    &ctx::Home::default(),
                    &["A server error occurred, please try again"],
                )),
            )),
        }
    } else {
        let errors = form
            .context
            .errors()
            .map(|err| {
                use rocket::form::error::ErrorKind;
                if let ErrorKind::Validation(msg) = &err.kind {
                    msg.as_ref()
                } else {
                    eprintln!("unhandled error: {}", err);
                    "An error occurred, please try again"
                }
            })
            .collect::<Vec<_>>();
        Err((
            Status::BadRequest,
            RawHtml(renderer.render_with_data(
                ctx::Home::default(),
                ("clip", &form.context),
                &errors,
            )),
        ))
    }
}

#[rocket::get("/clip/<shortcode>")]
pub async fn get_clip(
    shortcode: ShortCode,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>,
    hit_counter: &State<HitCounter>,
) -> Result<status::Custom<RawHtml<String>>, PageError> {
    fn render_with_status<T: ctx::PageContext + serde::Serialize + std::fmt::Debug>(
        status: Status,
        context: T,
        renderer: &Renderer,
    ) -> Result<status::Custom<RawHtml<String>>, PageError> {
        Ok(status::Custom(
            status,
            RawHtml(renderer.render(&context, &[])),
        ))
    }

    match action::get_clip(shortcode.clone().into(), database.get_pool()).await {
        Ok(clip) => {
            hit_counter.hit(shortcode.clone(), 1);
            let context = ctx::ClipView::new(clip);
            render_with_status(Status::Ok, context, renderer)
        }
        Err(e) => match e {
            ServiceError::PermissionError(_) => {
                let context = ctx::ClipRequirePassword::new(shortcode);
                render_with_status(Status::Unauthorized, context, renderer)
            }
            ServiceError::NotFound => Err(PageError::NotFound("clip not found".to_owned())),
            _ => Err(PageError::InternalError(format!("{}", e))),
        },
    }
}

#[rocket::post("/clip/<shortcode>", data = "<form>")]
pub async fn submit_clip_password(
    cookies: &CookieJar<'_>,
    form: Form<Contextual<'_, form::GetPasswordProtectedClip>>,
    shortcode: ShortCode,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>,
) -> Result<RawHtml<String>, PageError> {
    if let Some(form) = &form.value {
        let req = service::ask::GetClip {
            shortcode: shortcode.clone(),
            password: form.password.clone(),
        };

        match action::get_clip(req, database.get_pool()).await {
            Ok(clip) => {
                let context = ctx::ClipView::new(clip);
                cookies.add(Cookie::new(
                    PASSWORD_COOKIE,
                    form.password.clone().into_inner().unwrap_or_default(),
                ));
                Ok(RawHtml(renderer.render(&context, &[])))
            }
            Err(e) => match e {
                ServiceError::PermissionError(_) => {
                    let context = ctx::ClipRequirePassword::new(shortcode);
                    Ok(RawHtml(renderer.render(&context, &["incorrect password"])))
                }
                ServiceError::NotFound => Err(PageError::NotFound("clip not found".to_owned())),
                _ => Err(PageError::InternalError(format!("{}", e))),
            },
        }
    } else {
        let context = ctx::ClipRequirePassword::new(shortcode);
        Ok(RawHtml(renderer.render(&context, &[])))
    }
}

#[rocket::get("/clip/raw/<shortcode>")]
pub async fn get_raw_clip(
    cookies: &CookieJar<'_>,
    shortcode: ShortCode,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>,
    hit_counter: &State<HitCounter>,
) -> Result<status::Custom<String>, Status> {
    use crate::domain::clip::field::Password;

    let req = service::ask::GetClip {
        shortcode: shortcode.clone(),
        password: cookies
            .get(PASSWORD_COOKIE)
            .map(|c| c.value())
            .map(|raw| Password::new(raw.to_string()).ok())
            .flatten()
            .unwrap_or_else(|| Password::default()),
    };

    match action::get_clip(req, database.get_pool()).await {
        Ok(clip) => {
            hit_counter.hit(shortcode.clone(), 1);
            Ok(status::Custom(Status::Ok, clip.content.into_inner()))
        }
        Err(e) => match e {
            ServiceError::PermissionError(_) => {
                let context = ctx::ClipRequirePassword::new(shortcode);
                Ok(status::Custom(
                    Status::Unauthorized,
                    renderer.render(&context, &[]),
                ))
            }
            ServiceError::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        },
    }
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![
        home,
        get_clip,
        new_clip,
        submit_clip_password,
        get_raw_clip,
        api_key,
        generate_api_key,
    ]
}

pub mod catcher {
    use rocket::Request;
    use rocket::{catch, catchers, Catcher};

    #[catch(default)]
    fn default(_: &Request) -> &'static str {
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

#[cfg(test)]
pub mod test {
    use crate::data::AppDatabase;
    use crate::web::test::client;
    use rocket::http::Status;

    #[test]
    fn gets_home() {
        let client = client();
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn error_on_missing_clip() {
        let client = client();
        let response = client.get("/clip/notexist").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }
    #[test]
    fn requires_password_when_applicable() {
        use crate::domain::clip::field::{Content, ExpiresAt, Password, Title};
        use crate::service;
        use rocket::http::{ContentType, Cookie};

        let rt = tokio::runtime::Runtime::new().expect("failed to spawn tokio runtime");

        let client = client();
        let db = client.rocket().state::<AppDatabase>().unwrap();

        let req = service::ask::NewClip {
            title: Title::default(),
            content: Content::new("content").unwrap(),
            expires_at: ExpiresAt::default(),
            password: Password::new("123".to_owned()).unwrap(),
        };
        let clip = rt
            .block_on(async move { service::action::new_clip(req, db.get_pool()).await })
            .unwrap();

        // Block clip when no password is provided
        let response = client
            .get(format!("/clip/{}", clip.shortcode.as_str()))
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
        let response = client
            .get(format!("/clip/raw/{}", clip.shortcode.as_str()))
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);

        // Get clip when the password is provided
        let response = client
            .post(format!("/clip/{}", clip.shortcode.as_str()))
            .header(ContentType::Form)
            .body("password=123")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        // Get clip when the password is provided
        let response = client
            .get(format!("/clip/raw/{}", clip.shortcode.as_str()))
            .cookie(Cookie::new("password", "123"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        // Get clip when the password is provided, but incorrect
        let response = client
            .get(format!("/clip/raw/{}", clip.shortcode.as_str()))
            .cookie(Cookie::new("password", "abc"))
            .dispatch();
        assert_eq!(response.status(), Status::Unauthorized);
    }
}
