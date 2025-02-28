use serde_json::json;

use crate::{
    AllowWarnDeny, ConfigStore, LintOptions, Linter, RuleWithSeverity,
    rule::Rule,
    rules::{
        EslintArrayCallbackReturn, EslintConstructorSuper, EslintCurly, EslintDefaultCase,
        EslintDefaultCaseLast, EslintDefaultParamLast, EslintEqeqeq, EslintForDirection,
        EslintFuncNames, EslintFuncStyle, EslintGetterReturn, EslintGroupedAccessorPairs,
        EslintGuardForIn, EslintInitDeclarations, EslintMaxClassesPerFile, EslintMaxDepth,
        EslintMaxLines, EslintMaxLinesPerFunction, EslintMaxNestedCallbacks, EslintMaxParams,
        EslintNewCap, EslintNoAlert, EslintNoArrayConstructor, EslintNoAsyncPromiseExecutor,
        EslintNoAwaitInLoop, EslintNoBitwise, EslintNoCaller, EslintNoCaseDeclarations,
        EslintNoClassAssign, EslintNoCompareNegZero, EslintNoCondAssign, EslintNoUnusedVars,
        RuleEnum,
    },
};

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

#[derive(Debug, PartialEq, Eq)]
pub enum LintMode {
    Development,
    Production,
    None,
}

pub struct CustomLinter {
    mode: LintMode,
}

impl CustomLinter {
    pub fn new() -> Self {
        Self { mode: LintMode::Development }
    }

    fn get_eslint_rules(&self) -> Vec<RuleWithSeverity> {
        vec![
            c!(2, EslintArrayCallbackReturn, json!({"checkForEach": true, "allowImplicit": false})),
            // TODO: prefer 2
            c!(
                1,
                EslintNoUnusedVars,
                json!({
                    "vars":"all",
                    "args":"all",
                    "caughtErrors":"all",
                    "varsIgnorePattern":"^_",
                    "argsIgnorePattern":"^_",
                    "caughtErrorsIgnorePattern":"^_",
                    "destructuredArrayIgnorePattern":"^_",
                    "ignoreRestSiblings":false,
                    "ignoreClassWithStaticInitBlock":false,
                    "reportUsedIgnorePattern":true
                })
            ),
            c!(2, EslintConstructorSuper),
            c!(2, EslintCurly, json!(["all"])),
            c!(2, EslintDefaultCaseLast),
            c!(2, EslintDefaultParamLast),
            c!(2, EslintEqeqeq, json!(["always",{"null":"always"}])),
            c!(2, EslintForDirection),
            c!(2, EslintFuncNames, json!(["as-needed"])),
            c!(2, EslintFuncStyle, json!(["declaration", { "allowArrowFunctions": true }])),
            c!(2, EslintGetterReturn, json!(["always", { "allowImplicit": false }])),
            c!(2, EslintGroupedAccessorPairs, json!(["anyOrder"])),
            c!(2, EslintGuardForIn),
            // TODO: 理论上应该检测循环内必须初始化，其他的不管，但是好像无法这样配置 , 所以关闭
            c!(0, EslintInitDeclarations, json!(["always", { "ignoreForLoopInit": true }])),
            // TODO: 限制一个文件只能有一个 class , 我个人认为是合理的。
            c!(2, EslintMaxClassesPerFile, json!(["1", { "ignoreExpressions": false }])),
            // TODO: 代码嵌套层数，实际代码中还是很难做到的，函数抽多了名字也不好起，先关闭，后续再说
            c!(0, EslintMaxDepth, json!(["10"])),
            // TODO: 实际代码中还是很难做到的，函数抽多了名字也不好起，先关闭，后续再说
            c!(
                0,
                EslintMaxLinesPerFunction,
                json!(["100", { "skipComments": true, "skipBlankLines": true, "iifes": true }])
            ),
            // TODO: 这个我觉得实际工作中还是比较容易做到 1000 行以内的。
            c!(
                2,
                EslintMaxLines,
                json!(["1000", { "skipBlankLines": true, "skipComments": true }])
            ),
            // TODO: 这个我觉得实际工作中还是比较容易做到 cb 嵌套控制在 10 以内的
            c!(2, EslintMaxNestedCallbacks, json!(["10"])),
            c!(2, EslintMaxParams, json!(["5"])),
            // TODO: 比较难做到，先关闭
            c!(0, EslintNewCap),
            c!(if self.mode == LintMode::Production { 2 } else { 1 }, EslintNoAlert),
            c!(2, EslintNoArrayConstructor),
            c!(2, EslintNoAsyncPromiseExecutor),
            // TODO: 这里为什么是 1 ? 因为 js 的某些并发控制可能需要在循环内 await Promise.race()
            c!(1, EslintNoAwaitInLoop),
            // TODO: 位运算某些场景下挺好用的，可以取代传数组
            c!(0, EslintNoBitwise),
            c!(2, EslintNoCaller),
            c!(2, EslintNoCaseDeclarations),
            c!(2, EslintNoClassAssign),
            c!(2, EslintNoCompareNegZero),
            c!(2, EslintNoCondAssign, json!(["except-parens"])),
        ]
    }

    pub fn lint(&self) {
        let options = LintOptions::default();

        // c!(EslintArrayCallbackReturn);

        let base_rules = vec![RuleWithSeverity::new(
            RuleEnum::EslintDefaultCase(EslintDefaultCase::from_configuration(
                serde_json::Value::Null,
            )),
            AllowWarnDeny::Allow,
        )];

        // let config = ConfigStore::new(base_rules, base_config, overrides);

        // let linter = Linter::new(options, config);
    }
}
