use std::{path::Path, rc::Rc, sync::Arc};

use crate::{
    LintPlugins, Message,
    config::{OxlintEnv, OxlintGlobals, OxlintSettings},
};
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
        EslintNoConstAssign, EslintNoConstantBinaryExpression, EslintNoConstantCondition,
        EslintNoConstructorReturn, EslintNoContinue, EslintNoControlRegex, EslintNoDebugger,
        EslintNoDeleteVar, EslintNoDivRegex, EslintNoDupeClassMembers, EslintNoDupeElseIf,
        EslintNoDupeKeys, EslintNoDuplicateCase, EslintNoDuplicateImports, EslintNoElseReturn,
        EslintNoEmpty, EslintNoEmptyCharacterClass, EslintNoEmptyFunction, EslintNoEmptyPattern,
        EslintNoEmptyStaticBlock, EslintNoEqNull, EslintNoEval, EslintNoExAssign,
        EslintNoExtendNative, EslintNoExtraBooleanCast, EslintNoExtraLabel, EslintNoFallthrough,
        EslintNoFuncAssign, EslintNoGlobalAssign, EslintNoImportAssign, EslintNoInnerDeclarations,
        EslintNoInvalidRegexp, EslintNoIrregularWhitespace, EslintNoIterator, EslintNoLabelVar,
        EslintNoLabels, EslintNoLoneBlocks, EslintNoLossOfPrecision, EslintNoMagicNumbers,
        EslintNoMultiAssign, EslintNoMultiStr, EslintNoNegatedCondition, EslintNoNestedTernary,
        EslintNoNew, EslintNoNewFunc, EslintNoNewNativeNonconstructor, EslintNoNewWrappers,
        EslintNoNonoctalDecimalEscape, EslintNoObjCalls, EslintNoObjectConstructor,
        EslintNoPlusplus, EslintNoProto, EslintNoPrototypeBuiltins, EslintNoRedeclare,
        EslintNoRegexSpaces, EslintNoRestrictedGlobals, EslintNoRestrictedImports,
        EslintNoReturnAssign, EslintNoScriptUrl, EslintNoSelfAssign, EslintNoSelfCompare,
        EslintNoSetterReturn, EslintNoShadowRestrictedNames, EslintNoSparseArrays,
        EslintNoTemplateCurlyInString, EslintNoTernary, EslintNoThisBeforeSuper,
        EslintNoThrowLiteral, EslintNoUndef, EslintNoUndefined, EslintNoUnexpectedMultiline,
        EslintNoUnneededTernary, EslintNoUnreachable, EslintNoUnsafeFinally,
        EslintNoUnsafeNegation, EslintNoUnsafeOptionalChaining, EslintNoUnusedExpressions,
        EslintNoUnusedLabels, EslintNoUnusedPrivateClassMembers, EslintNoUnusedVars,
        EslintNoUselessCall, EslintNoUselessCatch, EslintNoUselessConcat,
        EslintNoUselessConstructor, EslintNoUselessEscape, EslintNoUselessRename, EslintNoVar,
        EslintNoVoid, EslintNoWith, EslintPreferExponentiationOperator,
        EslintPreferNumericLiterals, EslintPreferObjectHasOwn, EslintPreferObjectSpread,
        EslintPreferPromiseRejectErrors, EslintPreferRestParams, EslintPreferSpread, EslintRadix,
        EslintRequireAwait, EslintRequireYield, EslintSortImports, EslintSortKeys, EslintSortVars,
        EslintSymbolDescription, EslintUnicodeBom, EslintUseIsnan, EslintValidTypeof,
        EslintVarsOnTop, EslintYoda, RuleEnum,
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
        if is_dev {
            warn!(EslintNoDebugger, json!({}))
        } else {
            deny!(EslintNoDebugger, json!({}))
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
        deny!(EslintNoConstAssign, json!({})),
        deny!(EslintNoConstantBinaryExpression, json!({})),
        warn!(EslintNoConstantCondition, json!({})),
        deny!(EslintNoConstructorReturn, json!({})),
        allow!(EslintNoContinue, json!({})),
        deny!(EslintNoControlRegex, json!({})),
        deny!(EslintNoDeleteVar, json!({})),
        allow!(EslintNoDivRegex, json!({})),
        deny!(EslintNoDupeClassMembers, json!({})),
        deny!(EslintNoDupeElseIf, json!({})),
        deny!(EslintNoDupeKeys, json!({})),
        deny!(EslintNoDuplicateCase, json!({})),
        // TODO 可能跟 ts 的那个 import type 冲突
        deny!(EslintNoDuplicateImports, json!({})),
        allow!(EslintNoElseReturn, json!({})),
        deny!(EslintNoEmptyCharacterClass, json!({})),
        warn!(EslintNoEmptyFunction, json!({})),
        warn!(EslintNoEmptyPattern, json!({})),
        warn!(EslintNoEmptyStaticBlock, json!({})),
        warn!(EslintNoEmpty, json!({})),
        deny!(EslintNoEqNull, json!({})),
        // TODO 微前端需要用，所以设置为 warn
        warn!(EslintNoEval, json!({})),
        deny!(EslintNoExAssign, json!({})),
        deny!(EslintNoExtendNative, json!({})),
        warn!(EslintNoExtraBooleanCast, json!({"enforceForLogicalOperands":true})),
        deny!(EslintNoExtraLabel, json!({})),
        warn!(EslintNoFallthrough, json!({"allowEmptyCases":true})),
        deny!(EslintNoFuncAssign, json!({})),
        deny!(EslintNoGlobalAssign, json!({"exceptions":[]})),
        deny!(EslintNoImportAssign, json!({})),
        // 实际上只要禁用了 var 的使用，就只剩函数的场景会触发，因为只有 var、function 才会牵扯到提升问题
        deny!(EslintNoInnerDeclarations, json!({"functions":true})),
        deny!(EslintNoInvalidRegexp, json!({})),
        deny!(EslintNoIrregularWhitespace, json!({})),
        deny!(EslintNoIterator, json!({})),
        deny!(EslintNoLabelVar, json!({})),
        allow!(EslintNoLabels, json!({})),
        deny!(EslintNoLoneBlocks, json!({})),
        deny!(EslintNoLossOfPrecision, json!({})),
        warn!(
            EslintNoMagicNumbers,
            json!([{
              "ignore": ["-1", "0", "1", "2"],
              "ignoreArrayIndexes": true,
              "ignoreDefaultValues": true,
              "ignoreClassFieldInitialValues": true,
              "enforceConst":true,
              "detectObjects":false,
              "ignoreEnums":true,
              "ignoreNumericLiteralTypes":true,
              "ignoreReadonlyClassProperties":true,
              "ignoreTypeIndexes":true
            }])
        ),
        deny!(EslintNoMultiAssign, json!({})),
        deny!(EslintNoMultiStr, json!({})),
        allow!(EslintNoNegatedCondition, json!({})),
        allow!(EslintNoNestedTernary, json!({})),
        allow!(EslintNoNewFunc, json!({})),
        deny!(EslintNoNewNativeNonconstructor, json!({})),
        deny!(EslintNoNewWrappers, json!({})),
        warn!(EslintNoNew, json!({})),
        deny!(EslintNoNonoctalDecimalEscape, json!({})),
        deny!(EslintNoObjCalls, json!({})),
        warn!(EslintNoObjectConstructor, json!({})),
        allow!(EslintNoPlusplus, json!({})),
        deny!(EslintNoProto, json!({})),
        deny!(EslintNoPrototypeBuiltins, json!({})),
        deny!(EslintNoRedeclare, json!({"builtinGlobals":true})),
        deny!(EslintNoRegexSpaces, json!({})),
        allow!(EslintNoRestrictedGlobals, json!({})),
        allow!(EslintNoRestrictedImports, json!({})),
        allow!(EslintNoReturnAssign, json!({})),
        allow!(EslintNoScriptUrl, json!({})),
        deny!(EslintNoSelfAssign, json!({})),
        deny!(EslintNoSelfCompare, json!({})),
        deny!(EslintNoSetterReturn, json!({})),
        deny!(EslintNoShadowRestrictedNames, json!({})),
        deny!(EslintNoSparseArrays, json!({})),
        warn!(EslintNoTemplateCurlyInString, json!({})),
        allow!(EslintNoTernary, json!({})),
        deny!(EslintNoThisBeforeSuper, json!({})),
        deny!(EslintNoThrowLiteral, json!({})),
        // TODO 这个应该跟 define 联动配置
        warn!(EslintNoUndef, json!({})),
        allow!(EslintNoUndefined, json!({})),
        deny!(EslintNoUnexpectedMultiline, json!({})),
        warn!(EslintNoUnneededTernary, json!({"defaultAssignment":true})),
        warn!(EslintNoUnreachable, json!({})),
        deny!(EslintNoUnsafeFinally, json!({})),
        warn!(EslintNoUnsafeNegation, json!({"enforceForOrderingRelations":true})),
        deny!(EslintNoUnsafeOptionalChaining, json!({"disallowArithmeticOperators":true})),
        allow!(
            EslintNoUnusedExpressions,
            json!({
              "allowShortCircuit":true,
              "allowTernary":true,
              "allowTaggedTemplates":true,
              "enforceForJSX":true
            })
        ),
        warn!(EslintNoUnusedLabels, json!({})),
        warn!(EslintNoUnusedPrivateClassMembers, json!({})),
        warn!(EslintNoUselessCall, json!({})),
        warn!(EslintNoUselessCatch, json!({})),
        warn!(EslintNoUselessConcat, json!({})),
        warn!(EslintNoUselessConstructor, json!({})),
        warn!(EslintNoUselessEscape, json!({})),
        deny!(
            EslintNoUselessRename,
            json!({
              "ignoreDestructuring":false,
              "ignoreImport":true,
              "ignoreExport":true
            })
        ),
        deny!(EslintNoVar, json!({})),
        allow!(EslintNoVoid, json!({})),
        // TODO 微前端需要
        allow!(EslintNoWith, json!({})),
        warn!(EslintPreferExponentiationOperator, json!({})),
        allow!(EslintPreferNumericLiterals, json!({})),
        warn!(EslintPreferObjectHasOwn, json!({})),
        warn!(EslintPreferObjectSpread, json!({})),
        warn!(EslintPreferPromiseRejectErrors, json!({"allowEmptyReject":false})),
        warn!(EslintPreferRestParams, json!({})),
        warn!(EslintPreferSpread, json!({})),
        warn!(EslintRadix, json!(["as-needed"])),
        deny!(EslintRequireAwait, json!({})),
        deny!(EslintRequireYield, json!({})),
        allow!(EslintSortImports, json!({})),
        allow!(EslintSortKeys, json!({})),
        allow!(EslintSortVars, json!({})),
        deny!(EslintSymbolDescription, json!({})),
        allow!(EslintUnicodeBom, json!({})),
        deny!(
            EslintUseIsnan,
            json!({
                "enforceForSwitchCase":true,
                "enforceForIndexOf":true
            })
        ),
        deny!(
            EslintValidTypeof,
            json!({
              "requireStringLiterals":true
            })
        ),
        allow!(EslintVarsOnTop, json!({})),
        warn!(
            EslintYoda,
            json!({
              "exceptRange": true,
              "onlyEquality": true
            })
        ),
    ];

    let c = LintConfig {
        plugins: LintPlugins::ESLINT,
        settings: OxlintSettings::default(),
        env: OxlintEnv::from_iter(vec!["es6".to_string(), "browser".to_string()]),
        globals: OxlintGlobals::default(),
        path: None,
    };

    let config = ConfigStore::new(rules, c, config::OxlintOverrides::default());

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
