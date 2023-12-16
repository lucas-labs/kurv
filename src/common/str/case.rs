use regex_lite::Regex;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum WordCase {
    Lower,
    Upper,
    Capital,
}

impl WordCase {
    fn mutate(&self, word: &str) -> String {
        use WordCase::*;
        match self {
            Lower => word.to_lowercase(),
            Upper => word.to_uppercase(),
            Capital => {
                let mut chars = word.chars();
                if let Some(c) = chars.next() {
                    c.to_uppercase()
                        .chain(chars.as_str().to_lowercase().chars())
                        .collect()
                } else {
                    String::new()
                }
            }
        }
    }
}

pub trait StrCase<T: AsRef<str>> {
    /// <u>case convertion</u> › to `kebab-case`
    fn kebab(&self) -> String;

    /// <u>case convertion</u> › to `snake_case`
    fn snake(&self) -> String;

    /// <u>case convertion</u> › to `camelCase`
    fn camel(&self) -> String;

    /// <u>case convertion</u> › to `PascalCase`
    fn pascal(&self) -> String;

    /// <u>case convertion</u> › to `Title Case`
    fn title(&self) -> String;

    /// <u>case convertion</u> › to `CONSTANT_CASE`
    fn constant(&self) -> String;
}

impl<T: AsRef<str>> StrCase<T> for T
where
    String: PartialEq<T>,
{
    fn kebab(&self) -> String {
        convert(self, Some(WordCase::Lower), "-")
    }

    fn snake(&self) -> String {
        convert(self, Some(WordCase::Lower), "_")
    }

    fn camel(&self) -> String {
        convert(self, Some(WordCase::Capital), "")
    }

    fn pascal(&self) -> String {
        convert(self, Some(WordCase::Capital), "")
    }

    fn title(&self) -> String {
        convert(self, Some(WordCase::Capital), " ")
    }

    fn constant(&self) -> String {
        convert(self, Some(WordCase::Upper), "_")
    }
}

fn convert<T>(s: T, case: Option<WordCase>, delim: &str) -> String
where
    T: AsRef<str>,
{
    let words = split(s);

    if let Some(c) = case {
        let words = words.iter().map(|s| s.as_ref()).collect::<Vec<&str>>();
        words.iter().map(|s| c.mutate(s)).collect()
    } else {
        words.join(&delim)
    }
}

pub fn split<T>(s: T) -> Vec<String>
where
    T: AsRef<str>,
{
    let boundaries: [Regex; 4] = [
        // split on whitespace, hyphens, and underscores
        Regex::new(r"[\s\-_]").unwrap(),
        // split on lower-to-upper case
        Regex::new(r"(?P<lower>[a-z])(?P<upper>[A-Z])").unwrap(),
        // split on upper-to-upper-lower case
        Regex::new(r"(?P<upper>[A-Z])(?P<upper2>[A-Z][a-z])").unwrap(),
        // split on upper-to-lower case
        Regex::new(r"(?P<upper>[A-Z])(?P<lower>[a-z])").unwrap(),
    ];

    let mut words = vec![s.as_ref().to_string()];
    for re in &boundaries {
        words = words
            .iter()
            .flat_map(|s| re.split(s))
            .map(|s| s.to_string())
            .collect();
    }
    words
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}
