use serde_json::{Map, Value, json};

use super::rule_getter::RuleGetter;

pub struct ReactRuleGetter;

impl RuleGetter for ReactRuleGetter {
    fn get_dev_override_rules() -> Map<String, Value> {
        json!({}).as_object().map_or(Map::new(), |map| map.to_owned())
    }

    fn get_def_rules() -> Map<String, Value> {
        json!({
          "react/jsx-key":2,
          "react/jsx-no-duplicate-props":2,
          "react/jsx-no-target-blank":[2,{
            "enforceDynamicLinks": "always",
            "warnOnSpreadAttributes":true,
            "allow_referrer":true,
            "links":true,
            "forms":false
          }],
          "react/jsx-no-undef":2,
          "react/jsx-props-no-spread-multi":2,
          "react/no-children-prop":2,
          "react/no-danger-with-children":2,
          "react/no-direct-mutation-state":2,
          "react/no-is-mounted":2,
          "react/no-string-refs":2,
          "react/void-dom-elements-no-children":2,
          "react/button-has-type":2,
          "react/iframe-missing-sandbox":2,
          "react/jsx-no-comment-textnodes":2,
          "react/no-array-index-key":1,
          "react/no-render-return-value":1,
          "react/jsx-boolean-value":1,
          "react/no-find-dom-node":1,
          "react/no-unknown-property":1,
          "react/self-closing-comp":1,
          "react/no-danger":0,
          "react/jsx-no-script-url":0,
          "react/jsx-no-useless-fragment":0,
          "react/prefer-es6-class":0,
          "react/style-prop-object":2,
          "react/checked-requires-onchange-or-readonly":2,
          "react/no-unescaped-entities":2,
          "react/rules-of-hooks":2,
          "react/jsx-curly-brace-presence": [1, {
            "props": "always",
            "children": "always",
            "propElementValues": "always"
          }],
          "react/no-set-state":0,
          "react/react-in-jsx-scope":2
        })
        .as_object()
        .map_or(Map::new(), |map| map.to_owned())
    }
}
