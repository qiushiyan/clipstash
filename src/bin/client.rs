use clipstash::domain::clip::field::{Content, ExpiresAt, Password, ShortCode, Title};
use clipstash::service::ask::{GetClip, NewClip, UpdateClip};
use clipstash::web::api::{ApiKey, API_KEY_HEADER};
use clipstash::Clip;
use std::error::Error;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
enum Command {
    Get {
        shortcode: ShortCode,
        #[structopt(short, long, help = "password")]
        password: Option<String>,
    },
    New {
        #[structopt(short, long, help = "title")]
        title: Option<Title>,
        #[structopt(help = "content")]
        content: String,
        #[structopt(short, long, help = "password")]
        password: Option<Password>,
        #[structopt(short, long, help = "expiration date")]
        expires_at: Option<ExpiresAt>,
    },
    Update {
        shortcode: ShortCode,
        content: String,
        #[structopt(short, long, help = "password")]
        password: Option<Password>,
        #[structopt(short, long, help = "expiration date")]
        expires_at: Option<ExpiresAt>,
        #[structopt(short, long, help = "title")]
        title: Option<Title>,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "clipclient", about = "ClipStash API Client")]
struct Opt {
    #[structopt(subcommand)]
    command: Command,

    #[structopt(default_value = "http://127.0.0.1:8000", env = "CLIPSTASH_ADDR")]
    addr: String,

    #[structopt(long)]
    api_key: ApiKey,
}

fn get_clip(addr: &str, ask_svc: GetClip, api_key: ApiKey) -> Result<Clip, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/clip/{}", addr, ask_svc.shortcode.into_inner());
    let mut request = client.get(addr);
    request = match ask_svc.password.into_inner() {
        Some(password) => request.header(reqwest::header::COOKIE, format!("password={}", password)),
        None => request,
    };
    request = request.header(API_KEY_HEADER, api_key.to_base64());
    Ok(request.send()?.json()?)
}

fn new_clip(addr: &str, ask_svc: NewClip, api_key: ApiKey) -> Result<Clip, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/clip", addr);
    let mut request = client.post(addr);
    request = request.header(API_KEY_HEADER, api_key.to_base64());
    Ok(request.json(&ask_svc).send()?.json()?)
}

fn update_clip(addr: &str, ask_svc: UpdateClip, api_key: ApiKey) -> Result<Clip, Box<dyn Error>> {
    let client = reqwest::blocking::Client::builder().build()?;
    let addr = format!("{}/api/clip", addr);
    let mut request = client.put(addr);
    request = request.header(API_KEY_HEADER, api_key.to_base64());
    Ok(request.json(&ask_svc).send()?.json()?)
}

fn run(opt: Opt) -> Result<(), Box<dyn Error>> {
    match opt.command {
        Command::Get {
            shortcode,
            password,
        } => {
            let req = GetClip {
                password: Password::new(password.unwrap_or_default())?,
                shortcode,
            };
            let clip = get_clip(opt.addr.as_str(), req, opt.api_key)?;
            println!("{:#?}", clip);
            Ok(())
        }
        Command::New {
            title,
            content,
            password,
            expires_at,
        } => {
            let req = NewClip {
                title: title.unwrap_or_default(),
                content: Content::new(content.as_str())?,
                expires_at: expires_at.unwrap_or_default(),
                password: password.unwrap_or_default(),
            };
            let clip = new_clip(opt.addr.as_str(), req, opt.api_key)?;
            println!("{:#?}", clip);
            Ok(())
        }
        Command::Update {
            title,
            content,
            password,
            expires_at,
            shortcode,
        } => {
            let password = password.unwrap_or_default();
            let svc_req = GetClip {
                password: password.clone(),
                shortcode: shortcode.clone(),
            };
            let original_clip = get_clip(opt.addr.as_str(), svc_req, opt.api_key.clone())?;
            let svc_req = UpdateClip {
                title: title.unwrap_or(original_clip.title),
                content: Content::new(content.as_str())?,
                expires_at: expires_at.unwrap_or(original_clip.expires_at),
                password,
                shortcode,
            };
            let clip = update_clip(opt.addr.as_str(), svc_req, opt.api_key)?;
            println!("{:#?}", clip);
            Ok(())
        }
    }
}

fn main() {
    let opt = Opt::from_args();
    if let Err(e) = run(opt) {
        eprintln!("An error occurred: {}", e);
    }
}
