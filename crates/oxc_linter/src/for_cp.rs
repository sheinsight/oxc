use std::{path::Path, rc::Rc, sync::Arc};

use crate::Message;
use oxc_semantic::Semantic;
use serde_json::json;

use crate::{
    AllowWarnDeny, ConfigStore, LintOptions, Linter, ModuleRecord, RuleWithSeverity,
    config::{self, LintConfig},
    rule::Rule,
    rules::{
        EslintArrayCallbackReturn, EslintConstructorSuper, EslintDefaultCase,
        EslintDefaultCaseLast, EslintDefaultParamLast, EslintEqeqeq, EslintForDirection,
        EslintFuncNames, EslintFuncStyle, EslintGetterReturn, EslintGroupedAccessorPairs,
        EslintGuardForIn, EslintInitDeclarations, EslintMaxClassesPerFile, EslintMaxDepth,
        EslintMaxLines, EslintMaxLinesPerFunction, EslintMaxNestedCallbacks, EslintMaxParams,
        EslintNewCap, EslintNoAlert, EslintNoArrayConstructor, EslintNoAsyncPromiseExecutor,
        EslintNoAwaitInLoop, EslintNoBitwise, EslintNoCaller, EslintNoCaseDeclarations,
        EslintNoClassAssign, EslintNoCompareNegZero, EslintNoCondAssign, EslintNoConsole,
        EslintNoUnusedVars, RuleEnum,
    },
};

macro_rules! create {
    ($rule_type:ident, $json:expr) => {
        RuleWithSeverity::new(
            RuleEnum::$rule_type($rule_type::from_configuration($json)),
            AllowWarnDeny::Deny,
        )
    };

    ($rule_type:ident, $json:expr, $severity:expr) => {
        RuleWithSeverity::new(
            RuleEnum::$rule_type($rule_type::from_configuration($json)),
            $severity,
        )
    }; // 无配置版本
       // ($rule_type:ident) => {
       //     RuleWithSeverity::new(
       //         RuleEnum::$rule_type($rule_type::from_configuration($json)),
       //         AllowWarnDeny::Deny,
       //     )
       // };
}

macro_rules! deny {
    ($rule_type:ident, $json:expr) => {
        RuleWithSeverity::new(
            RuleEnum::$rule_type($rule_type::from_configuration($json)),
            AllowWarnDeny::Deny,
        )
    };
}

macro_rules! warn {
    ($rule_type:ident, $json:expr) => {
        RuleWithSeverity::new(
            RuleEnum::$rule_type($rule_type::from_configuration($json)),
            AllowWarnDeny::Warn,
        )
    };
}

macro_rules! allow {
    ($rule_type:ident, $json:expr) => {
        RuleWithSeverity::new(
            RuleEnum::$rule_type($rule_type::from_configuration($json)),
            AllowWarnDeny::Allow,
        )
    };
}

#[derive(PartialEq)]
pub enum LintMode {
    PRODUCTION,
    DEVELOPMENT,
    NONE,
}

pub fn run_lint<'a>(
    options: LintOptions,
    path: &Path,
    semantic: Rc<Semantic<'a>>,
    module_record: Arc<ModuleRecord>,
    mode: LintMode,
) -> Vec<Message<'a>> {
    let x = create!(EslintEqeqeq, json!(["always", {"null":"always"}]));

    let is_dev = mode != LintMode::PRODUCTION;

    let rules = vec![
        deny!(
            EslintArrayCallbackReturn,
            json!({
                "checkForEach": true,
                "allowImplicit": false
            })
        ),
        deny!(
            EslintNoUnusedVars,
            json!({
                "vars": "all",
                "args": "all",
                "caughtErrors": "all",
                "varsIgnorePattern": "^_",
                "argsIgnorePattern": "^_",
                "caughtErrorsIgnorePattern": "^_",
                "destructuredArrayIgnorePattern": "^_",
                "ignoreRestSiblings": false,
                "ignoreClassWithStaticInitBlock": false,
                "reportUsedIgnorePattern": true
            })
        ),
        deny!(
            EslintDefaultCase,
            json!({
                // commentPattern: "^skip\\sdefault"
            })
        ),
        deny!(EslintDefaultCaseLast, json!({})),
        deny!(EslintDefaultParamLast, json!({})),
        deny!(EslintConstructorSuper, json!({})),
        deny!(EslintEqeqeq, json!(["always", {"null":"always"}])),
        deny!(EslintForDirection, json!({})),
        deny!(EslintFuncNames, json!(["as-needed", {"generators":"as-needed"}])),
        deny!(
            EslintFuncStyle,
            json!(["declaration", {"allowArrowFunctions":true,"namedExports":"ignore"}])
        ),
        deny!(EslintGetterReturn, json!({"allowImplicit":false})),
        warn!(EslintGroupedAccessorPairs, json!(["anyOrder"])),
        deny!(EslintGuardForIn, json!({})),
        allow!(EslintInitDeclarations, json!(["always", {"ignoreForLoopInit":true}])),
        warn!(EslintMaxClassesPerFile, json!({"max":1})),
        warn!(EslintMaxDepth, json!({"max":10})),
        warn!(
            EslintMaxLinesPerFunction,
            json!({"max":300,"skipComments":true,"skipBlankLines":true,"IIFEs":false})
        ),
        warn!(EslintMaxLines, json!({"max":3000,"skipComments":true,"skipBlankLines":true})),
        warn!(EslintMaxNestedCallbacks, json!({"max":10})),
        warn!(EslintMaxParams, json!({"max":6})),
        allow!(EslintNewCap, json!({})),
        if is_dev { warn!(EslintNoAlert, json!({})) } else { deny!(EslintNoAlert, json!({})) },
        if is_dev {
            warn!(EslintNoConsole, json!({"allow":["error","warn","info"]}))
        } else {
            deny!(EslintNoConsole, json!({"allow":["error","warn","info"]}))
        },
        deny!(EslintNoArrayConstructor, json!({})),
        deny!(EslintNoAsyncPromiseExecutor, json!({})),
        warn!(EslintNoAwaitInLoop, json!({})),
        allow!(EslintNoBitwise, json!({})),
        deny!(EslintNoCaller, json!({})),
        deny!(EslintNoCaseDeclarations, json!({})),
        deny!(EslintNoClassAssign, json!({})),
        deny!(EslintNoCompareNegZero, json!({})),
        deny!(EslintNoCondAssign, json!(["except-parens"])),
    ];

    let config = ConfigStore::new(rules, LintConfig::default(), config::OxlintOverrides::default());

    let linter = Linter::new(options, config);
    linter.run(path, semantic, module_record)
}

#[cfg(test)]
mod tests {
    use miette::{LabeledSpan, miette};

    use super::*;

    fn render_lint_result<'a>(source_code: &'static str, res: Vec<Message<'a>>) {
        miette::set_hook(Box::new(|_| {
            Box::new(
                miette::MietteHandlerOpts::new()
                    .color(true)
                    .unicode(true)
                    .force_graphical(true)
                    .with_cause_chain()
                    .context_lines(10)
                    .build(),
            )
        }))
        .unwrap();
        for r in res {
            let spans = r.error.labels.clone().unwrap();

            let mut labels = vec![];
            println!("{:?}", r.error);
            let message = r.error.message.to_string();

            for span in spans {
                // let lp = LabeledSpan::at_offset(span.offset(), message.clone());
                let lp =
                    LabeledSpan::at(span.offset()..span.offset() + span.len(), message.clone());
                labels.push(lp);
            }

            let h = r.error.help.as_ref().map_or("".to_string(), |s| s.to_string());

            // println!("{:?}", r.error.severity);

            let scope = r.error.code.scope.as_ref().unwrap().to_string();
            let number = r.error.code.number.as_ref().unwrap().to_string();

            let severity = match r.error.severity {
                oxc_diagnostics::Severity::Error => miette::Severity::Error,
                oxc_diagnostics::Severity::Warning => miette::Severity::Warning,
                oxc_diagnostics::Severity::Advice => miette::Severity::Advice,
            };

            let report = miette!(
                severity = severity,
                url = r.error.url.as_ref().unwrap().to_string(),
                labels = labels,
                help = h.as_str(),
                "{}/{}",
                scope,
                number
            )
            .with_source_code(source_code);

            eprintln!("{:?}", report);
        }
    }

    #[test]
    fn test_create_rules() {
        let source_code = r#"
              console.log(foo)
          "#;
        let p = Path::new("./aA.tsx");

        let allocator = oxc_allocator::Allocator::default();

        let source_type = oxc_span::SourceType::tsx();

        let parse = oxc_parser::Parser::new(&allocator, &source_code, source_type).parse();

        let program = allocator.alloc(&parse.program);

        let semantic_return = oxc_semantic::SemanticBuilder::new()
            .with_check_syntax_error(false)
            // TODO 很多场景下是不需要开启的，只有 oxlint 下需要开启，这可能对性能会产生一定的影响
            .with_cfg(true)
            .build(program);

        let semantic = semantic_return.semantic;

        let res = run_lint(
            LintOptions::default(),
            &p,
            Rc::new(semantic),
            Arc::new(ModuleRecord::default()),
            LintMode::PRODUCTION,
        );
        render_lint_result(source_code, res);
    }
}
