use minijinja::{Environment, Error, ErrorKind};
use std::path::PathBuf;

pub struct TemplateEngine {
    env: Environment<'static>,
}

impl TemplateEngine {
    pub fn new(template_dir: PathBuf) -> Self {
        let mut env = Environment::new();

        env.set_loader(move |name| {
            let path = template_dir.join(name);

            match std::fs::read_to_string(&path) {
                Ok(src) => Ok(Some(src)),
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                    Ok(None)
                }
                Err(e) => Err(Error::new(
                    ErrorKind::InvalidOperation,
                    format!("template `{}`: {}", name, e),
                )),
            }
        });

        Self { env }
    }

    pub fn render(
        &self,
        name: &str,
        ctx: &serde_json::Value,
    ) -> Result<String, String> {
        let tmpl = self.env.get_template(name)
            .map_err(|e| e.to_string())?;

        let value = minijinja::Value::from_serialize(ctx);
        tmpl.render(value).map_err(|e| e.to_string())
    }
}
