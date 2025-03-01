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

#[macro_export]
macro_rules! merge_object_value {
    // 基本情况：单个值
    ($a:ident) => { $a };

    // 两个值的情况
    ($a:ident, $b:ident) => {
        serde_json::json!({
            ...$a,
            ...$b
        })
    };

    // 递归处理三个或更多值
    ($a:ident, $b:ident, $($rest:ident),+) => {{
        let temp = merge_object_value!($a, $b);
        merge_object_value!(temp, $($rest),+)
    }};
}
