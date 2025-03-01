use crate::{
    AllowWarnDeny, RuleWithSeverity, c,
    rule::Rule,
    rules::{
        EslintArrayCallbackReturn, EslintConstructorSuper, EslintCurly, EslintDefaultCase,
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
        EslintNoSetterReturn, EslintNoShadowRestrictedNames, EslintNoSpacedFunc,
        EslintNoSparseArrays, EslintNoTemplateCurlyInString, EslintNoTernary,
        EslintNoThisBeforeSuper, EslintNoThrowLiteral, EslintNoUndef, EslintNoUndefined,
        EslintNoUnexpectedMultiline, EslintNoUnneededTernary, EslintNoUnreachable,
        EslintNoUnsafeFinally, EslintNoUnsafeNegation, EslintNoUnsafeOptionalChaining,
        EslintNoUnusedExpressions, EslintNoUnusedLabels, EslintNoUnusedPrivateClassMembers,
        EslintNoUnusedVars, EslintNoUselessCall, EslintNoUselessCatch, EslintNoUselessConcat,
        EslintNoUselessConstructor, EslintNoUselessEscape, EslintNoUselessRename, EslintNoVar,
        EslintNoVoid, EslintNoWith, EslintOperatorAssignment, EslintPreferExponentiationOperator,
        EslintPreferNumericLiterals, EslintPreferObjectHasOwn, EslintPreferObjectSpread,
        EslintPreferPromiseRejectErrors, EslintPreferRestParams, EslintPreferSpread, EslintRadix,
        EslintRequireAwait, EslintRequireYield, EslintSortImports, EslintSortKeys, EslintSortVars,
        EslintSymbolDescription, EslintUnicodeBom, EslintUseIsnan, EslintValidTypeof,
        EslintVarsOnTop, EslintYoda,
    },
};
use serde_json::json;

pub struct EslintRules {
    rules: Vec<RuleWithSeverity>,
}

impl EslintRules {
    fn default() -> Self {
        let severity_by_mode = true;
        Self {
            rules: vec![
                c!(
                    2,
                    EslintArrayCallbackReturn,
                    json!({
                        "checkForEach": true,
                        "allowImplicit": false
                    })
                ),
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
                c!(
                    2,
                    EslintEqeqeq,
                    json!(["always",{
                        "null":"always"
                    }])
                ),
                c!(2, EslintForDirection),
                c!(2, EslintFuncNames, json!(["as-needed"])),
                c!(
                    2,
                    EslintFuncStyle,
                    json!(["declaration", {
                        "allowArrowFunctions": true
                    }])
                ),
                c!(
                    2,
                    EslintGetterReturn,
                    json!(["always", {
                    "allowImplicit": false
                    }])
                ),
                c!(2, EslintGroupedAccessorPairs, json!(["anyOrder"])),
                c!(2, EslintGuardForIn),
                // TODO: 理论上应该检测循环内必须初始化，其他的不管，但是好像无法这样配置 , 所以关闭
                c!(
                    0,
                    EslintInitDeclarations,
                    json!(["always", {
                       "ignoreForLoopInit": true
                    }])
                ),
                // TODO: 限制一个文件只能有一个 class , 我个人认为是合理的。
                c!(
                    2,
                    EslintMaxClassesPerFile,
                    json!(["1", {
                        "ignoreExpressions": false
                    }])
                ),
                // TODO: 代码嵌套层数，实际代码中还是很难做到的，函数抽多了名字也不好起，先关闭，后续再说
                c!(0, EslintMaxDepth, json!(["10"])),
                // TODO: 实际代码中还是很难做到的，函数抽多了名字也不好起，先关闭，后续再说
                c!(
                    0,
                    EslintMaxLinesPerFunction,
                    json!(["100", {
                        "skipComments": true,
                        "skipBlankLines": true,
                        "iifes": true
                    }])
                ),
                // TODO: 这个我觉得实际工作中还是比较容易做到 1000 行以内的。
                c!(
                    2,
                    EslintMaxLines,
                    json!(["1000", {
                        "skipBlankLines": true,
                        "skipComments": true
                    }])
                ),
                // TODO: 这个我觉得实际工作中还是比较容易做到 cb 嵌套控制在 10 以内的
                c!(2, EslintMaxNestedCallbacks, json!(["10"])),
                c!(2, EslintMaxParams, json!(["5"])),
                // TODO: 比较难做到，先关闭
                c!(0, EslintNewCap),
                c!(severity_by_mode, EslintNoAlert),
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
                c!(
                    severity_by_mode,
                    EslintNoConsole,
                    json!({ "allow": ["info", "warn", "error"] })
                ),
                c!(2, EslintNoConstAssign),
                c!(2, EslintNoConstantBinaryExpression),
                // TODO: 循环内不检查
                c!(
                    2,
                    EslintNoConstantCondition,
                    json!({
                        "checkLoops":false
                    })
                ),
                c!(2, EslintNoConstructorReturn),
                c!(0, EslintNoContinue),
                c!(2, EslintNoControlRegex),
                c!(severity_by_mode, EslintNoDebugger),
                c!(2, EslintNoDeleteVar),
                c!(2, EslintNoDivRegex),
                c!(2, EslintNoDupeClassMembers),
                c!(2, EslintNoDupeElseIf),
                c!(2, EslintNoDupeKeys),
                c!(2, EslintNoDuplicateCase),
                // TODO: 有可能跟 ts 的 import type 冲突，后续需要验证
                c!(
                    2,
                    EslintNoDuplicateImports,
                    json!({
                        "includeExports":true
                    })
                ),
                c!(
                    0,
                    EslintNoElseReturn,
                    json!({
                        "allowElseIf":true
                    })
                ),
                c!(2, EslintNoEmptyCharacterClass),
                c!(severity_by_mode, EslintNoEmptyFunction),
                c!(severity_by_mode, EslintNoEmptyPattern),
                c!(severity_by_mode, EslintNoEmptyStaticBlock),
                c!(
                    severity_by_mode,
                    EslintNoEmpty,
                    json!({
                        "allowEmptyCatch":true
                    })
                ),
                c!(2, EslintNoEqNull),
                // 允许这种逃逸手段
                // const indirect = eval;
                // indirect('var x = 1');
                c!(
                    severity_by_mode,
                    EslintNoEval,
                    json!({
                        "allowIndirect":true
                    })
                ),
                c!(2, EslintNoExAssign),
                c!(2, EslintNoExtendNative, json!({"exceptions":[]})),
                c!(
                    2,
                    EslintNoExtraBooleanCast,
                    json!({
                        "enforceForLogicalOperands":true
                    })
                ),
                c!(2, EslintNoExtraLabel),
                c!(
                    2,
                    EslintNoFallthrough,
                    json!({
                        "allowEmptyCase":true,
                        "reportUnusedFallthroughComment":true
                    })
                ),
                c!(2, EslintNoFuncAssign),
                c!(2, EslintNoGlobalAssign, json!({"exceptions":[]})),
                c!(2, EslintNoImportAssign),
                c!(2, EslintNoInnerDeclarations, json!({"config":{"Functions":true}})),
                c!(2, EslintNoInvalidRegexp, json!({"allowConstructorFlags":[]})),
                c!(2, EslintNoIrregularWhitespace),
                c!(2, EslintNoIterator),
                c!(2, EslintNoLabelVar),
                c!(0, EslintNoLabels, json!({"allowLoop":true, "allowSwitch":true})),
                c!(2, EslintNoLoneBlocks),
                c!(2, EslintNoLossOfPrecision),
                // TODO: 这个规则的配置项太多了，先关闭
                c!(0, EslintNoMagicNumbers),
                c!(2, EslintNoMultiAssign),
                c!(2, EslintNoMultiStr),
                c!(2, EslintNoNegatedCondition),
                // TODO: 感觉完全禁止嵌套三元表达式也不太现实
                c!(1, EslintNoNestedTernary),
                // TODO 微前端需要
                c!(1, EslintNoNewFunc),
                c!(2, EslintNoNewNativeNonconstructor),
                c!(2, EslintNoNewWrappers),
                c!(2, EslintNoNew),
                c!(2, EslintNoNonoctalDecimalEscape),
                c!(2, EslintNoObjCalls),
                c!(1, EslintNoObjectConstructor),
                c!(0, EslintNoPlusplus),
                c!(2, EslintNoProto),
                c!(2, EslintNoPrototypeBuiltins),
                c!(2, EslintNoRedeclare),
                c!(2, EslintNoRegexSpaces),
                c!(0, EslintNoRestrictedGlobals),
                c!(0, EslintNoRestrictedImports),
                c!(0, EslintNoReturnAssign),
                // TODO javascript:void(0); 这个有些场景会用
                c!(0, EslintNoScriptUrl),
                c!(2, EslintNoSelfAssign),
                c!(2, EslintNoSelfCompare),
                c!(2, EslintNoSetterReturn),
                c!(2, EslintNoShadowRestrictedNames),
                c!(2, EslintNoSpacedFunc),
                c!(2, EslintNoSparseArrays),
                c!(1, EslintNoTemplateCurlyInString),
                c!(0, EslintNoTernary),
                c!(2, EslintNoThisBeforeSuper),
                c!(2, EslintNoThrowLiteral),
                // TODO 应该利用全局的 globals 配置 跟 define 联动配置
                c!(2, EslintNoUndef),
                c!(2, EslintNoUndefined),
                c!(2, EslintNoUnexpectedMultiline),
                c!(1, EslintNoUnneededTernary, json!({ "defaultAssignment": true })),
                c!(2, EslintNoUnreachable),
                c!(2, EslintNoUnsafeFinally),
                c!(2, EslintNoUnsafeNegation, json!({ "enforceForOrderingRelations": true })),
                c!(
                    2,
                    EslintNoUnsafeOptionalChaining,
                    json!({ "disallowArithmeticOperators": false })
                ),
                c!(
                    1,
                    EslintNoUnusedExpressions,
                    json!({
                        "allowShortCircuit": true,
                        "allowTernary": true,
                        "allowTaggedTemplates": true,
                        "enforceForJSX": false
                    })
                ),
                c!(2, EslintNoUnusedLabels),
                c!(1, EslintNoUnusedPrivateClassMembers),
                c!(1, EslintNoUselessCall),
                c!(1, EslintNoUselessCatch),
                c!(1, EslintNoUselessConcat),
                c!(1, EslintNoUselessConstructor),
                c!(1, EslintNoUselessEscape),
                c!(1, EslintNoUselessRename),
                c!(2, EslintNoVar),
                c!(0, EslintNoVoid, json!({ "allowAsStatement": true })),
                // TODO 微前端需要
                c!(1, EslintNoWith),
                c!(1, EslintOperatorAssignment, json!({ "mode": "always" })),
                c!(1, EslintPreferExponentiationOperator),
                c!(1, EslintPreferNumericLiterals),
                c!(2, EslintPreferObjectHasOwn),
                c!(2, EslintPreferObjectSpread),
                c!(2, EslintPreferPromiseRejectErrors, json!({"allowThenable":true})),
                c!(2, EslintPreferRestParams),
                c!(2, EslintPreferSpread),
                c!(0, EslintRadix),
                c!(2, EslintRequireAwait),
                c!(2, EslintRequireYield),
                c!(0, EslintSortImports),
                c!(0, EslintSortKeys),
                c!(0, EslintSortVars),
                c!(2, EslintSymbolDescription),
                c!(0, EslintUnicodeBom),
                c!(2, EslintUseIsnan),
                c!(2, EslintValidTypeof, json!({ "requireStringLiterals": true })),
                c!(2, EslintVarsOnTop),
                c!(0, EslintYoda, json!({ "never": true })),
            ],
        }
    }
}
