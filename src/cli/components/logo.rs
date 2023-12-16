use super::Component;


const LOGO_STR : &str = 
r#"<dim>
▀▀████
  ████
  ████   ▀█▀  ▀████  ▀███   ▀███ ▄██▄ ▀███▀   ▀▀█▀
  ████  ▄▀     ████   ███    ███▀ ██▀  ▀██▄    ▞
  ████▅██▃     ████   ███    ███        ███▄  ▞
  ████ ▀██▆    ████  ▄███    ███         ███ ▞
▄▄████▄ ▄███▄   ▀██▅▀ ███▄  ▄███▄▄        ███
</dim>"#;

pub struct Logo { }

impl Component for Logo { 
    fn render(&self) -> String {
        LOGO_STR.to_string()
    }
}

