use crate::{
    AllowWarnDeny, ConfigStore, LintOptions, Linter, RuleWithSeverity, c,
    config::{LintConfig, OxlintOverrides},
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
        EslintVarsOnTop, EslintYoda, ImportDefault, ImportExport, ImportFirst,
        ImportMaxDependencies, ImportNamed, ImportNamespace, ImportNoAbsolutePath, ImportNoAmd,
        ImportNoCommonjs, ImportNoCycle, ImportNoDefaultExport, ImportNoDuplicates,
        ImportNoDynamicRequire, ImportNoMutableExports, ImportNoNamedAsDefault,
        ImportNoNamedAsDefaultMember, ImportNoNamedDefault, ImportNoNamespace, ImportNoSelfImport,
        ImportNoWebpackLoaderSyntax, ImportUnambiguous, OxcApproxConstant,
        OxcBadArrayMethodOnArguments, OxcBadBitwiseOperator, OxcBadCharAtComparison,
        OxcBadComparisonSequence, OxcBadMinMaxFunc, OxcBadObjectLiteralComparison,
        OxcBadReplaceAllArg, OxcConstComparisons, OxcDoubleComparisons, OxcErasingOp,
        OxcMisrefactoredAssignOp, OxcMissingThrow, OxcNoAccumulatingSpread, OxcNoAsyncAwait,
        OxcNoAsyncEndpointHandlers, OxcNoBarrelFile, OxcNoConstEnum, OxcNoMapSpread,
        OxcNoOptionalChaining, OxcNoRedundantConstructorInit, OxcNoRestSpreadProperties,
        OxcNumberArgOutOfRange, OxcOnlyUsedInRecursion, OxcUninvokedArrayCallback, PromiseAvoidNew,
        PromiseCatchOrReturn, PromiseNoCallbackInPromise, PromiseNoNesting, PromiseNoNewStatics,
        PromiseNoPromiseInCallback, PromiseNoReturnInFinally, PromiseParamNames,
        PromisePreferAwaitToCallbacks, PromisePreferAwaitToThen, PromiseSpecOnly,
        PromiseValidParams, ReactButtonHasType, ReactCheckedRequiresOnchangeOrReadonly,
        ReactExhaustiveDeps, ReactIframeMissingSandbox, ReactJsxBooleanValue,
        ReactJsxCurlyBracePresence, ReactJsxKey, ReactJsxNoCommentTextnodes,
        ReactJsxNoDuplicateProps, ReactJsxNoScriptUrl, ReactJsxNoTargetBlank, ReactJsxNoUndef,
        ReactJsxNoUselessFragment, ReactJsxPropsNoSpreadMulti, ReactNoArrayIndexKey,
        ReactNoChildrenProp, ReactNoDanger, ReactNoDangerWithChildren, ReactNoDirectMutationState,
        ReactNoFindDomNode, ReactNoIsMounted, ReactNoRenderReturnValue, ReactNoSetState,
        ReactNoStringRefs, ReactNoUnescapedEntities, ReactNoUnknownProperty,
        ReactPerfJsxNoJsxAsProp, ReactPerfJsxNoNewArrayAsProp, ReactPerfJsxNoNewFunctionAsProp,
        ReactPerfJsxNoNewObjectAsProp, ReactPreferEs6Class, ReactReactInJsxScope,
        ReactRequireRenderReturn, ReactRulesOfHooks, ReactSelfClosingComp, ReactStylePropObject,
        ReactVoidDomElementsNoChildren, RuleEnum, TypescriptAdjacentOverloadSignatures,
        TypescriptArrayType, TypescriptBanTsComment, TypescriptBanTslintComment,
        TypescriptBanTypes, TypescriptConsistentGenericConstructors,
        TypescriptConsistentIndexedObjectStyle, TypescriptConsistentTypeDefinitions,
        TypescriptConsistentTypeImports, TypescriptExplicitFunctionReturnType,
        TypescriptNoConfusingNonNullAssertion, TypescriptNoDuplicateEnumValues,
        TypescriptNoDynamicDelete, TypescriptNoEmptyInterface, TypescriptNoEmptyObjectType,
        TypescriptNoExplicitAny, TypescriptNoExtraNonNullAssertion, TypescriptNoExtraneousClass,
        TypescriptNoImportTypeSideEffects, TypescriptNoInferrableTypes, TypescriptNoMisusedNew,
        TypescriptNoNamespace, TypescriptNoNonNullAssertedNullishCoalescing,
        TypescriptNoNonNullAssertedOptionalChain, TypescriptNoNonNullAssertion,
        TypescriptNoRequireImports, TypescriptNoThisAlias, TypescriptNoUnnecessaryTypeConstraint,
        TypescriptNoUnsafeDeclarationMerging, TypescriptNoUnsafeFunctionType,
        TypescriptNoUselessEmptyExport, TypescriptNoVarRequires, TypescriptNoWrapperObjectTypes,
        TypescriptPreferAsConst, TypescriptPreferEnumInitializers, TypescriptPreferForOf,
        TypescriptPreferFunctionType, TypescriptPreferLiteralEnumMember,
        TypescriptPreferNamespaceKeyword, TypescriptPreferTsExpectError,
        TypescriptTripleSlashReference,
    },
};
use lint_mode::LintMode;
use react_config::{ReactConfig, ReactRuntime};
use serde_json::json;
pub mod commons;
pub mod lint_mode;
pub mod react_config;
pub mod rules;

// pub use commons::c;

pub struct CustomLinter {
    mode: LintMode,
    react: ReactConfig,
}

impl CustomLinter {
    pub fn new() -> Self {
        Self { mode: LintMode::Development, react: ReactConfig::default() }
    }

    pub fn with_react_config(mut self, react: ReactConfig) -> Self {
        self.react = react;
        self
    }

    fn get_eslint_rules(&self) -> Vec<RuleWithSeverity> {
        let severity_by_mode = if self.mode == LintMode::Production { 2 } else { 1 };

        vec![
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
            c!(severity_by_mode, EslintNoConsole, json!({ "allow": ["info", "warn", "error"] })),
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
            c!(2, EslintNoUnsafeOptionalChaining, json!({ "disallowArithmeticOperators": false })),
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
        ]
    }

    // 所有路径相关的都是通过 oxc_resolver 去解析 tsconfig.json 的 path 来实现的
    fn get_import_rules(&self) -> Vec<RuleWithSeverity> {
        vec![
            c!(2, ImportDefault),
            c!(2, ImportExport),
            c!(
                2,
                ImportFirst,
                json!({
                    "absoluteFirst": false
                })
            ),
            c!(
                0,
                ImportMaxDependencies,
                json!({
                    "max": 10
                })
            ),
            c!(2, ImportNamed),
            c!(
                2,
                ImportNamespace,
                json!({
                    "allowComputed": false
                })
            ),
            c!(
                2,
                ImportNoAbsolutePath,
                json!({
                    "esmodule": true,
                    "commonjs": true,
                    "amd": true
                })
            ),
            c!(0, ImportNoAmd),
            c!(0, ImportNoCommonjs),
            c!(
                2,
                ImportNoCycle,
                json!({
                    "maxDepth": 3,
                    "ignoreTypes": true,
                    "ignoreExternal": true,
                    "allowUnsafeDynamicCyclicDependency": false
                })
            ),
            c!(0, ImportNoDefaultExport),
            // TODO 跟 EslintNoDuplicateImports 有点重复
            c!(0, ImportNoDuplicates),
            // TODO 这个确实不好，但是限制不住，很多场景可能确实需要
            c!(0, ImportNoDynamicRequire),
            c!(2, ImportNoMutableExports),
            c!(2, ImportNoNamedAsDefaultMember),
            c!(2, ImportNoNamedAsDefault),
            c!(2, ImportNoNamedDefault),
            c!(1, ImportNoNamespace),
            c!(2, ImportNoSelfImport),
            c!(2, ImportNoWebpackLoaderSyntax),
            // TODO 赞同这个规范，但是总感觉有些场景存在这个必要性，所以先设置警告
            c!(1, ImportUnambiguous),
        ]
    }

    fn get_react_rules(&self) -> Vec<RuleWithSeverity> {
        vec![
            c!(
                2,
                ReactButtonHasType,
                json!({
                    "button": true,
                    "submit": true,
                    "reset": true
                })
            ),
            c!(
                2,
                ReactCheckedRequiresOnchangeOrReadonly,
                json!({
                    "ignore_missing_properties": true,
                    "ignore_exclusive_checked_attribute": true
                })
            ),
            // TODO 过于教条，其实有些场景内部依赖的值就是不会变化的。先警告
            c!(1, ReactExhaustiveDeps),
            c!(2, ReactIframeMissingSandbox),
            c!(
                2,
                ReactJsxBooleanValue,
                json!(["never",{
                    "exceptions":[],
                    "assumeUndefinedIsFalse":true
                }])
            ),
            c!(
                1,
                ReactJsxCurlyBracePresence,
                json!({
                    "props": "always",
                    "children": "always",
                    "propElementValues": "always"
                })
            ),
            c!(2, ReactJsxKey),
            c!(2, ReactJsxNoCommentTextnodes),
            c!(2, ReactJsxNoDuplicateProps),
            c!(0, ReactJsxNoScriptUrl),
            c!(0, ReactJsxNoTargetBlank),
            c!(0, ReactJsxNoUndef),
            c!(0, ReactJsxNoUselessFragment),
            c!(2, ReactJsxPropsNoSpreadMulti),
            // TODO 规则是对的，实际工作中很难落地
            c!(1, ReactNoArrayIndexKey),
            c!(2, ReactNoChildrenProp),
            c!(2, ReactNoDangerWithChildren),
            // TODO 有些时候还是需要注入 html 的
            c!(0, ReactNoDanger),
            c!(2, ReactNoDirectMutationState),
            c!(1, ReactNoFindDomNode),
            c!(1, ReactNoIsMounted),
            c!(2, ReactNoRenderReturnValue),
            c!(2, ReactNoSetState),
            c!(
                2,
                ReactNoStringRefs,
                json!({
                    "noTemplateLiterals":true
                })
            ),
            c!(1, ReactNoUnescapedEntities),
            // 感觉官方代码还没写完，所以先关闭
            c!(
                0,
                ReactNoUnknownProperty,
                json!({
                    "ignore":[]
                })
            ),
            c!(0, ReactPreferEs6Class),
            // TODO 要跟 jsx-runtime 联动
            c!(
                if self.react.runtime == ReactRuntime::Automatic { 2 } else { 0 },
                ReactReactInJsxScope
            ),
            c!(2, ReactRequireRenderReturn),
            c!(1, ReactRulesOfHooks),
            c!(
                1,
                ReactSelfClosingComp,
                json!({
                    "component":true,
                    "html":true
                })
            ),
            c!(
                2,
                ReactStylePropObject,
                json!({
                    "allow":[]
                })
            ),
            c!(2, ReactVoidDomElementsNoChildren),
        ]
    }

    fn get_react_perf_rules(&self) -> Vec<RuleWithSeverity> {
        vec![
            // 实际工作中几乎不可能满足这个要求
            c!(0, ReactPerfJsxNoJsxAsProp),
            c!(1, ReactPerfJsxNoNewArrayAsProp),
            // 可能要考虑未来跟 react compile 联动
            c!(1, ReactPerfJsxNoNewFunctionAsProp),
            c!(1, ReactPerfJsxNoNewObjectAsProp),
        ]
    }

    fn get_typescript_rules(&self) -> Vec<RuleWithSeverity> {
        vec![
            c!(2, TypescriptAdjacentOverloadSignatures),
            c!(0, TypescriptArrayType),
            c!(0, TypescriptBanTsComment),
            c!(2, TypescriptBanTslintComment),
            c!(
                2,
                TypescriptBanTypes,
                json!({
                    "types": {
                        "String": {
                          "message": "Use string instead",
                          "fixWith": "string"
                        },
                        "Number": {
                          "message": "Use number instead",
                          "fixWith": "number"
                        },
                        "Boolean": {
                          "message": "Use boolean instead",
                          "fixWith": "boolean"
                        },
                        "Object": {
                          "message": "Use Record<string, unknown> instead"
                        },
                        "Function": {
                          "message": "Use specific function type instead"
                        }
                      }
                })
            ),
            c!(0, TypescriptConsistentGenericConstructors),
            c!(0, TypescriptConsistentIndexedObjectStyle),
            c!(0, TypescriptConsistentTypeDefinitions),
            c!(0, TypescriptConsistentTypeImports),
            // TODO  isolated_declarations 需要验证
            c!(
                1,
                TypescriptExplicitFunctionReturnType,
                json!({
                    "allowExpressions": true,                    // 允许函数表达式不声明返回类型
                    "allowTypedFunctionExpressions": true,       // 允许已有类型声明的函数表达式
                    "allowDirectConstAssertionInArrowFunctions": true,  // 允许箭头函数中的 const 断言
                    "allowConciseArrowFunctionExpressionsStartingWithVoid": true,  // 允许以 void 开始的简洁箭头函数
                    "allowFunctionsWithoutTypeParameters": true,  // 允许无类型参数的函数
                    "allowedNames": [ ],  // 允许的函数名列表
                    "allowHigherOrderFunctions": true,           // 允许高阶函数
                    "allowIIFEs": true
                })
            ),
            c!(2, TypescriptNoConfusingNonNullAssertion),
            c!(2, TypescriptNoDuplicateEnumValues),
            c!(1, TypescriptNoDynamicDelete),
            c!(1, TypescriptNoEmptyInterface),
            c!(
                1,
                TypescriptNoEmptyObjectType,
                json!({
                    "allowInterfaces": "never",
                    "allowObjectTypes": "never",
                    "allowWithName": ""
                })
            ),
            c!(1, TypescriptNoExplicitAny),
            c!(2, TypescriptNoExtraNonNullAssertion),
            c!(0, TypescriptNoExtraneousClass),
            c!(1, TypescriptNoImportTypeSideEffects),
            c!(0, TypescriptNoInferrableTypes),
            c!(1, TypescriptNoMisusedNew),
            c!(
                1,
                TypescriptNoNamespace,
                json!({
                    "allowDeclarations": false,
                    "allowDefinitionFiles": true
                })
            ),
            c!(2, TypescriptNoNonNullAssertedNullishCoalescing),
            c!(2, TypescriptNoNonNullAssertedOptionalChain),
            c!(2, TypescriptNoNonNullAssertion),
            c!(1, TypescriptNoRequireImports),
            c!(1, TypescriptNoThisAlias),
            c!(1, TypescriptNoUnnecessaryTypeConstraint),
            c!(2, TypescriptNoUnsafeDeclarationMerging),
            c!(0, TypescriptNoUnsafeFunctionType),
            c!(2, TypescriptNoUselessEmptyExport),
            c!(2, TypescriptNoVarRequires),
            c!(2, TypescriptNoWrapperObjectTypes),
            c!(1, TypescriptPreferAsConst),
            c!(2, TypescriptPreferEnumInitializers),
            c!(1, TypescriptPreferForOf),
            c!(0, TypescriptPreferFunctionType),
            c!(2, TypescriptPreferLiteralEnumMember),
            c!(1, TypescriptPreferNamespaceKeyword),
            c!(2, TypescriptPreferTsExpectError),
            c!(1, TypescriptTripleSlashReference),
        ]
    }

    fn get_promise_rules(&self) -> Vec<RuleWithSeverity> {
        vec![
            c!(1, PromiseAvoidNew),
            c!(2, PromiseCatchOrReturn, json!({ "allowFinally": true })),
            c!(0, PromiseNoCallbackInPromise),
            c!(2, PromiseNoNesting),
            c!(2, PromiseNoNewStatics),
            c!(1, PromiseNoPromiseInCallback),
            c!(2, PromiseNoReturnInFinally),
            c!(0, PromiseParamNames),
            c!(0, PromisePreferAwaitToCallbacks),
            c!(1, PromisePreferAwaitToThen),
            c!(2, PromiseSpecOnly, json!({ "allowedMethods": [] })),
            c!(0, PromiseValidParams),
        ]
    }

    fn get_oxc_rules(&self) -> Vec<RuleWithSeverity> {
        vec![
            c!(2, OxcApproxConstant),
            c!(2, OxcBadArrayMethodOnArguments),
            // TODO 规则是对的，但是边界场景太多，所以临时设定为 1， 以后考虑放开 2
            c!(1, OxcBadBitwiseOperator),
            c!(2, OxcBadCharAtComparison),
            c!(2, OxcBadComparisonSequence),
            c!(2, OxcBadMinMaxFunc),
            c!(2, OxcBadObjectLiteralComparison),
            c!(2, OxcBadReplaceAllArg),
            c!(2, OxcConstComparisons),
            c!(2, OxcDoubleComparisons),
            c!(2, OxcErasingOp),
            c!(2, OxcMisrefactoredAssignOp),
            c!(2, OxcMissingThrow),
            c!(1, OxcNoAccumulatingSpread),
            c!(0, OxcNoAsyncAwait),
            c!(0, OxcNoAsyncEndpointHandlers),
            c!(0, OxcNoBarrelFile),
            c!(2, OxcNoConstEnum),
            c!(1, OxcNoMapSpread),
            c!(0, OxcNoOptionalChaining),
            c!(1, OxcNoRedundantConstructorInit),
            c!(0, OxcNoRestSpreadProperties),
            c!(2, OxcNumberArgOutOfRange),
            c!(1, OxcOnlyUsedInRecursion),
            // TODO 跟 EslintArrayCallbackReturn 冲突
            c!(0, OxcUninvokedArrayCallback),
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

        let lint_config = LintConfig::default();

        let overrides = OxlintOverrides::default();

        // overrides.push(OxlintOverride {
        //     files: GlobSet::new(vec!["**/*.ts", "**/*.tsx"]).unwrap(),
        //     ..Default::default()
        // });

        // let config = ConfigStore::new(base_rules, lint_config, overrides);

        // let linter = Linter::new(options, config);
    }
}
