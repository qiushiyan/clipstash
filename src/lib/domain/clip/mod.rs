pub mod field;

#[derive(Debug)]
pub struct Clip {
    id: String,
    title: String,
    content: String,
    shortcode: String,
    created_at: String,
    expires_at: String,
    password: String,
    hits: i32,
}
