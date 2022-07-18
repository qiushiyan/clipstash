use crate::web::ctx;
use handlebars;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnumErorr {
    #[error("render error: {0}")]
    RenderError(#[from] handlebars::RenderError),
}

pub struct Renderer<'a>(handlebars::Handlebars<'a>);

impl<'a> Renderer<'a> {
    /// register template renderer
    pub fn new(template_dir: std::path::PathBuf) -> Self {
        let mut renderer = handlebars::Handlebars::new();
        renderer
            .register_templates_directory(".hbs", &template_dir)
            .expect("failed to registry handlebars renderer");
        Self(renderer)
    }
    /// Convert a serializable struct into a `serde_json::Value`.
    pub fn to_value<S>(s: &S) -> serde_json::Value
    where
        S: serde::Serialize + std::fmt::Debug,
    {
        match serde_json::to_value(&s) {
            Ok(v) => v,
            Err(_) => serde_json::Value::Null,
        }
    }

    pub fn render<C>(&self, context: &C, errors: &[&str]) -> String
    where
        C: ctx::PageContext + serde::Serialize + std::fmt::Debug,
    {
        let mut value = Self::to_value(&context);
        if let Some(value) = value.as_object_mut() {
            value.insert("_errors".to_owned(), errors.into());
            value.insert("_title".to_owned(), context.title().into());
            value.insert("_base".to_owned(), context.parent().into());
        }
        self.do_render(context.template_path(), value)
    }

    fn do_render(&self, path: &str, ctx: serde_json::Value) -> String {
        self.0.render(path, &ctx).expect("error rendering template")
    }

    /// Renders a page, along with serialized data and any errors.
    pub fn render_with_data<C, D>(&self, context: C, data: (&str, D), errors: &[&str]) -> String
    where
        C: ctx::PageContext + serde::Serialize + std::fmt::Debug,
        D: serde::Serialize + std::fmt::Debug,
    {
        use handlebars::to_json;

        let mut value = Self::to_value(&context);
        if let Some(value) = value.as_object_mut() {
            value.insert("_errors".into(), errors.into());
            value.insert("_title".into(), context.title().into());
            value.insert("_base".into(), context.parent().into());
            value.insert(data.0.into(), to_json(data.1));
        }
        self.do_render(context.template_path(), value)
    }
}
