use crate::data::AppDatabase;
use crate::domain::clip;
use crate::domain::clip::field::ShortCode;
use crate::service::{self, action, ServiceError, ask};
use crate::web::{context, form, renderer::Renderer, PageError, PASSWORD_COOKIE};

use clap::error::ErrorKind;
use rocket::form::{Contextual, Form};
use rocket::http::{Cookie, CookieJar, Status};
use rocket::response::{content::RawHtml, status, Redirect};
use rocket::{uri, State};

#[rocket::get("/")]
fn home(renderer: &State<Renderer<'_>>) -> RawHtml<String> {
    let context = context::Home::default();
    RawHtml(renderer.render(context, &[]))
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
            content: value.content,
            title: value.title,
            expires: value.expires,
            password: value.password,
        };
        match action::new_clip(req, database.get_pool()).await {
            Ok(clip) => Ok(Redirect::to(uri!(get_clip(shortcode = clip.shortcode)))),
            Err(e) => {
                eprintln!("internal error: {}", e);
                Err((
                    Status::InternalServerError,
                    RawHtml(renderer.render(
                        context::Home::default(),
                        &["A server error occurred. Please try again"],
                    )),
                ))
            }
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
                    "An error ocurred, please try again"
                }
            })
            .collect::<Vec<_>>();
        Err((
            Status::BadRequest,
            RawHtml(renderer.render_with_data(
                context::Home::default(),
                ("clip", &form.context),
                &errors,
            )),
        ))
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
                let context = context::ViewClip::new(clip);
                // Raw Password in a cookie, who does this?? 
                //I need to do this better but I'm no web developer ...
                // Maybe add a table in the DB with tokens and a lists of clips that you can see
                // If the password is right, check if you have a token, asign you one if not
                // then add this clipboard-token entry to the Table.
                // I dont know how efficient this could be
                // maybe make it so the key is the shortcode and the token is a value
                // this way the search should be much faster if the clipboard is not to be shared
                // with more people than there are current tokens
                // still I dont know if this could complicate the expire date of tokens in the database
                // probably another table where you have tokens and their expire date
                // this way you can do a join + where to expire them 
                //(needing to clean up afterwards in the shorcode-token table?)
                // while the database access is still fast without joins
                cookies.add(Cookie::new(
                    PASSWORD_COOKIE,
                    form.password.clone().into_inner().unwrap_or_default(),
                ));
                Ok(RawHtml(renderer.render(context, &[])))
            },
            Err(e) => match e {
                ServiceError::PermissionError(e) => {
                    let context = context::PasswordRequired::new(shortcode);
                    Ok(RawHtml(renderer.render(context, &[e.as_str()])))
                },
                _ => Err(PageError::Internal("server error".to_owned()))
            },
        }
    } else {
        let context = context::PasswordRequired::new(shortcode);
        Ok(RawHtml(renderer.render(
            context,
            &["A password is required to view the clip"])
        ))
    }
}

#[rocket::get("/clip/<shortcode>")]
pub async fn get_clip(
    shortcode: ShortCode,
    database: &State<AppDatabase>,
    renderer: &State<Renderer<'_>>,
) -> Result<status::Custom<RawHtml<String>>, PageError> {
    fn render_with_status<T: context::PageContext + serde::Serialize + std::fmt::Debug>(
        status: Status,
        context: T,
        renderer: &Renderer,
    ) -> Result<status::Custom<RawHtml<String>>, PageError> {
        Ok(status::Custom(
            status,
            RawHtml(renderer.render(context, &[])),
        ))
    }
    match action::get_clip(shortcode.clone().into(), database.get_pool()).await {
        Ok(clip) => {
            let context = context::ViewClip::new(clip);
            render_with_status(Status::Ok, context, renderer)
        }
        Err(e) => match e {
            ServiceError::PermissionError(_) => {
                let context = context::PasswordRequired::new(shortcode);
                render_with_status(Status::Unauthorized, context, renderer)
            }
            ServiceError::NotFound => Err(PageError::NotFound("Clip not found".to_owned())),
            _ => Err(PageError::Internal("server error".to_owned())),
        },
    }
}

#[rocket::get("/clip/raw/<shortcode>")]
pub async fn get_raw_clip(
    cookies: &CookieJar<'_>,
    shortcode: &str,
    database: &State<AppDatabase>
) -> Result<status::Custom<String>, Status> {
    use crate::domain::clip::field::Password;
    let req = ask::GetClip {
        shortcode: shortcode.into(),
        password: cookies
            .get(PASSWORD_COOKIE)
            .map(|cookie| cookie.value())
            .and_then(|raw_password| Password::new(raw_password.to_owned()).ok())
            .unwrap_or_default()
    };
    match action::get_clip(req, database.get_pool()).await {
        Ok(clip) => {
            Ok(status::Custom(Status::Ok, clip.content.into_inner()))
        },
        Err(e) => match e {
            ServiceError::PermissionError(msg) => Ok(status::Custom(Status::Unauthorized, msg)),
            ServiceError::NotFound => Err(Status::NotFound),
            _ => Err(Status::InternalServerError),
        }
    }
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![home, get_clip, new_clip, submit_clip_password, get_raw_clip]
}

pub mod catcher {
    use rocket::{catch, catchers, Catcher, Request};

    #[catch(default)]
    fn default(req: &Request) -> &'static str {
        eprintln!("General error: {:?}", req);
        "something went wrong ..."
    }

    #[catch(500)]
    fn internal_error(req: &Request) -> &'static str {
        eprintln!("Internal error: {:?}", req);
        "internal server error"
    }

    #[catch(404)]
    fn not_found() -> &'static str {
        "404"
    }

    pub fn catchers() -> Vec<Catcher> {
        catchers![default, internal_error, not_found]
    }
}
