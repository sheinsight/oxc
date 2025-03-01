#[macro_export]
macro_rules! c {
    ($severity:expr,$name:ident,$config:expr) => {
        RuleWithSeverity::new(
            RuleEnum::$name($name::from_configuration($config)),
            match $severity {
                0 => AllowWarnDeny::Allow,
                1 => AllowWarnDeny::Warn,
                2 => AllowWarnDeny::Deny,
                _ => panic!("Severity must be 0 (Allow), 1 (Warn), or 2 (Deny)"),
            },
        )
    };
    ($severity:expr, $name:ident) => {
        c!($severity, $name, serde_json::Value::Null)
    }; // ($name:ident) => {
       //     c!(2, $name, serde_json::Value::Null)
       // };
}
