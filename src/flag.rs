#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Flag {
    key: String,
    app: String,
    env: String,
    value: FlagValue,
    version: u64,
    enabled: bool,
}

impl Flag {
    pub fn new<S, T, U>(
        key: S,
        app: T,
        env: U,
        value: FlagValue,
        version: u64,
        enabled: bool,
    ) -> Flag
    where
        S: Into<String>,
        T: Into<String>,
        U: Into<String>,
    {
        Flag {
            key: key.into(),
            app: app.into(),
            env: env.into(),
            value: value,
            version: version,
            enabled: enabled,
        }
    }

    pub fn eval(&self) -> Option<&FlagValue> {
        if self.enabled {
            Some(&self.value)
        } else {
            None
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_ver(&self, ver: u64) -> bool {
        self.version == ver
    }

    pub fn app(&self) -> &str {
        self.app.as_str()
    }

    pub fn env(&self) -> &str {
        self.env.as_str()
    }

    pub fn key(&self) -> &str {
        self.key.as_str()
    }

    pub fn path(&self) -> String {
        Flag::make_path(self.app(), self.env(), self.key())
    }

    pub fn make_path(app: &str, env: &str, key: &str) -> String {
        [app, "::", env, "::", key].concat()
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlagValue {
    Bool(bool),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_returns_some_if_enabled() {
        let f = Flag::new("key-string", "", "", FlagValue::Bool(true), 1, true);
        assert_eq!(f.eval(), Some(&FlagValue::Bool(true)));
    }

    #[test]
    fn test_returns_none_if_disabled() {
        let f = Flag::new("key-string", "", "", FlagValue::Bool(true), 1, false);
        assert_eq!(f.eval(), None);
    }

    #[test]
    fn test_returns_enabled_status() {
        let f1 = Flag::new("key-string", "", "", FlagValue::Bool(true), 1, true);
        let f2 = Flag::new("key-string", "", "", FlagValue::Bool(true), 1, false);
        assert_eq!(f1.is_enabled(), true);
        assert_eq!(f2.is_enabled(), false);
    }

    #[test]
    fn test_checks_version() {
        let f = Flag::new("key-string", "", "", FlagValue::Bool(true), 1, true);
        assert_eq!(f.is_ver(1), true);
        assert_eq!(f.is_ver(2), false);
    }
}
