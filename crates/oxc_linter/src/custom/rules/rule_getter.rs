use serde_json::{Map, Value};

pub trait RuleGetter {
    fn get_dev_override_rules() -> Map<String, Value>;
    fn get_def_rules() -> Map<String, Value>;
}
