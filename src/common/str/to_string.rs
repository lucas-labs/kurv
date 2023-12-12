use {
    serde::Serialize,
    serde_yaml::to_string,
};

pub fn str<T>(t: &T) -> String where T: Serialize {
    match to_string(t) {
        Ok(s) => s,

        // if we couldn't serialize to string with serde
        // we return te struct name 
        Err(_) => format!("{}", std::any::type_name::<T>())
    }
}

pub trait ToString {
    fn str(&self) -> String;
}

impl<T> ToString for T where T: Serialize {
    fn str(&self) -> String {
        str(self)
    }
}
