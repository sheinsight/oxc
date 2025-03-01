use std::{path::Path, rc::Rc, sync::Arc};

use crate::{
    ConfigStoreBuilder, FixKind, FrameworkFlags, LintOptions, Linter, ModuleRecord, Oxlintrc,
};
use lint_mode::LintMode;
use miette::{GraphicalTheme, NamedSource, miette};
use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use rules::{
    eslint::EslintRuleGetter,
    oxc::OxcRuleGetter,
    promise::PromiseRuleGetter,
    react::{ReactConfig, ReactRuleGetter},
    react_perf::ReactPerfRuleGetter,
    rule_getter::RuleGetter,
    typescript::{TypescriptConfig, TypescriptRuleGetter},
};
use serde_json::{Map, Value, json};

pub mod lint_mode;
pub mod macros;
pub mod rules;

// pub use commons::c;

pub struct CustomLinter {
    mode: LintMode,
    react: Option<ReactConfig>,
    ts: Option<TypescriptConfig>,
}

impl CustomLinter {
    pub fn new() -> Self {
        Self { mode: LintMode::Development, react: None, ts: None }
    }

    pub fn with_ts(mut self, ts: TypescriptConfig) -> Self {
        self.ts = Some(ts);
        self
    }

    pub fn with_react_config(mut self, react: ReactConfig) -> Self {
        self.react = Some(react);
        self
    }

    fn get_def_plugins(&self) -> Vec<Value> {
        json!(["eslint", "unicorn", "import", "promise", "oxc"])
            .as_array()
            .map_or(vec![], |plugins| plugins.to_owned())
    }

    fn get_def_rules(&self) -> Map<String, Value> {
        let eslint = EslintRuleGetter::new().get_def_rules();
        let oxc = OxcRuleGetter::new().get_def_rules();
        let promise = PromiseRuleGetter::new().get_def_rules();
        let mut merged = Map::new();
        merged.extend(eslint);
        merged.extend(oxc);
        merged.extend(promise);
        merged
    }

    fn source_type_from_path<P: AsRef<Path>>(&self, path: P) -> oxc_span::SourceType {
        match path.as_ref().extension().and_then(|ext| ext.to_str()) {
            Some("ts") | Some("cts") | Some("mts") => oxc_span::SourceType::ts(),
            Some("tsx") => oxc_span::SourceType::tsx(),
            Some("jsx") => oxc_span::SourceType::jsx(),
            Some("cjs") => oxc_span::SourceType::cjs(),
            Some("mjs") => oxc_span::SourceType::mjs(),
            _ => oxc_span::SourceType::mjs(),
        }
    }

    fn get_overrides(&self) -> Vec<Value> {
        let mut overrides = json!([]).as_array().map_or(vec![], |overrides| overrides.to_owned());

        if let Some(ts) = &self.ts {
            let ts_rules = TypescriptRuleGetter::new(ts.clone()).get_def_rules();
            let ts_plugins =
                json!(["typescript"]).as_array().map_or(vec![], |plugins| plugins.to_owned());
            overrides.push(json!({
                "files": ["*.ts", "*.tsx", "*.cts", "*.mts"],
                "plugins": ts_plugins,
                "rules": ts_rules,
            }));
        }

        if let Some(react) = &self.react {
            let mut react_rules = ReactRuleGetter::new(react.clone()).get_def_rules();
            react_rules.extend(ReactPerfRuleGetter::new().get_def_rules());
            let react_plugins = json!(["react", "react-perf"])
                .as_array()
                .map_or(vec![], |plugins| plugins.to_owned());
            overrides.push(json!({
                "files": ["*.jsx", "*.tsx"],
                "plugins": react_plugins,
                "rules": react_rules,
            }));
        }

        overrides
    }

    fn get_linter_config(&self) -> Oxlintrc {
        let def_plugin = self.get_def_plugins();
        let def_rules = self.get_def_rules();
        let overrides = self.get_overrides();

        serde_json::from_value::<Oxlintrc>(json!({
            "plugins": def_plugin,
            "env": {
              "browser": true
            },
            "globals": {
              "foo": "readonly"
            },
            "settings": {},
            "rules": def_rules,
            "overrides":overrides
        }))
        .unwrap()
    }

    fn convert_severity(&self, severity: oxc_diagnostics::Severity) -> miette::Severity {
        match severity {
            oxc_diagnostics::Severity::Error => miette::Severity::Error,
            oxc_diagnostics::Severity::Warning => miette::Severity::Warning,
            oxc_diagnostics::Severity::Advice => miette::Severity::Advice,
        }
    }

    fn init_miette() {
        miette::set_hook(Box::new(|_| {
            Box::new(
                miette::MietteHandlerOpts::new()
                    .tab_width(4)
                    .terminal_links(true)
                    .unicode(true)
                    .color(true)
                    .with_cause_chain()
                    .build(),
            )
        }))
        .unwrap();
    }

    pub fn lint<P: AsRef<Path>>(&self, path: P) -> Result<(), String> {
        let path = path.as_ref();

        let Ok(source_code) = std::fs::read_to_string(&path) else {
            return Err(format!("Failed to read file: {}", path.display()));
        };

        println!("{}", source_code);

        let allocator = Allocator::default();
        let source_type = self.source_type_from_path(&path);
        let parser = Parser::new(&allocator, &source_code, source_type);
        let parser_return = parser.parse();
        let program = allocator.alloc(&parser_return.program);
        let semantic_builder_return =
            SemanticBuilder::new().with_check_syntax_error(false).with_cfg(true).build(program);
        let semantic = semantic_builder_return.semantic;
        let module_record =
            Arc::new(ModuleRecord::new(&path, &parser_return.module_record, &semantic));
        let semantic = Rc::new(semantic);

        let rc = self.get_linter_config();

        println!("{}", serde_json::to_string(&rc).unwrap());

        let config = ConfigStoreBuilder::from_oxlintrc(true, rc).build().unwrap();

        let lint = Linter::new(
            LintOptions { fix: FixKind::None, framework_hints: FrameworkFlags::NextOnly },
            config,
        );

        let res = lint.run(&path, semantic, module_record);

        Self::init_miette();

        let source = NamedSource::new(&path.display().to_string(), source_code.clone());

        for msg in res {
            println!("{:?}", msg.error);
            let err = msg.error;

            let severity = self.convert_severity(err.severity);
            let url = err.url.as_ref().unwrap().to_string();
            let labels = err.labels.clone().unwrap_or_default();
            let help = err.help.as_ref().map_or_else(|| "".to_string(), |help| help.to_string());
            let scope = err.code.scope.as_ref().unwrap();
            let number = err.code.number.as_ref().unwrap();

            let miette_report = miette!(
                severity = severity,
                url = url,
                labels = labels,
                help = help,
                code = format!("{}/{}", scope, number),
                "{}",
                err.message
            )
            .with_source_code(source.clone());

            eprintln!("{:?}", miette_report);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lint() {
        let linter = CustomLinter::new()
            .with_react_config(ReactConfig::default())
            .with_ts(TypescriptConfig::default());
        linter
            .lint("/Users/ityuany/GitRepository/csp-new/packages/csp-common-system/src/pages/cpcs-public-opinion/appeal-config/list.tsx");
    }
}
