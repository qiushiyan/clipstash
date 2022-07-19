use derive_more::Constructor;
use serde::Serialize;

pub trait PageContext {
    fn title(&self) -> &str;
    fn template_path(&self) -> &str;
    fn parent(&self) -> &str;
}

#[derive(Debug, Serialize)]
pub struct Home {}

impl Default for Home {
    fn default() -> Self {
        Self {}
    }
}

impl PageContext for Home {
    fn title(&self) -> &str {
        "Stash your clipboard"
    }

    fn template_path(&self) -> &str {
        "home"
    }

    fn parent(&self) -> &str {
        "base"
    }
}

#[derive(Debug, Serialize, Constructor)]
pub struct ClipView {
    pub clip: crate::Clip,
}

impl PageContext for ClipView {
    fn title(&self) -> &str {
        "Clip"
    }

    fn template_path(&self) -> &str {
        "clip"
    }

    fn parent(&self) -> &str {
        "base"
    }
}

#[derive(Debug, Serialize, Constructor)]
pub struct ClipRequirePassword {
    shortcode: crate::ShortCode,
}

impl PageContext for ClipRequirePassword {
    fn title(&self) -> &str {
        "Password required"
    }

    fn template_path(&self) -> &str {
        "clip_need_password"
    }

    fn parent(&self) -> &str {
        "base"
    }
}
#[derive(Debug, Serialize)]
pub struct ApiKeyGenerate {}

impl Default for ApiKeyGenerate {
    fn default() -> Self {
        Self {}
    }
}

impl PageContext for ApiKeyGenerate {
    fn title(&self) -> &str {
        "Generate Api Key"
    }

    fn template_path(&self) -> &str {
        "generate_api_key"
    }

    fn parent(&self) -> &str {
        "base"
    }
}

#[derive(Debug, Serialize)]
pub struct ApiKey {}

impl Default for ApiKey {
    fn default() -> Self {
        Self {}
    }
}

impl PageContext for ApiKey {
    fn title(&self) -> &str {
        "Api Key"
    }

    fn template_path(&self) -> &str {
        "api_key"
    }

    fn parent(&self) -> &str {
        "base"
    }
}
