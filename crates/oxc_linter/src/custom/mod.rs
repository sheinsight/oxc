use crate::{
    AllowWarnDeny, ConfigStore, ConfigStoreBuilder, FixKind, FrameworkFlags, LintOptions, Linter,
    Oxlintrc, RuleWithSeverity, c,
    config::{
        LintConfig, OxlintOverrides, OxlintRules,
        overrides::{GlobSet, OxlintOverride},
    },
    merge_object_value,
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
use oxc_index::IndexVec;
use react_config::{ReactConfig, ReactRuntime};
use rules::{
    eslint::EslintRuleGetter, oxc::OxcRuleGetter, promise::PromiseRuleGetter,
    react::ReactRuleGetter, react_perf::ReactPerfRuleGetter, rule_getter::RuleGetter,
    typescript::TypescriptRuleGetter,
};
use serde_json::json;
pub mod lint_mode;
pub mod macros;
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

    pub fn lint(&self) {
        let options = LintOptions::default();

        let eslint = EslintRuleGetter::get_def_rules();
        let react = ReactRuleGetter::get_def_rules();
        let react_perf = ReactPerfRuleGetter::get_def_rules();
        let oxc = OxcRuleGetter::get_def_rules();
        let promise = PromiseRuleGetter::get_def_rules();
        let typescript = TypescriptRuleGetter::get_def_rules();

        let mut merged = serde_json::Map::new();
        merged.extend(eslint);
        merged.extend(react);
        merged.extend(react_perf);
        merged.extend(oxc);
        merged.extend(promise);
        merged.extend(typescript);

        let overrides = json!([{
            "files": ["*.test.ts", "*.spec.ts","*.js","*.jsx","*.ts","*.tsx"],
            "rules": EslintRuleGetter::get_dev_override_rules()
        }]);

        let rc = serde_json::from_value::<Oxlintrc>(json!({
            "plugins": ["import", "typescript", "unicorn","oxc","promise","react","react-perf"],
            "env": {
              "browser": true
            },
            "globals": {
              "foo": "readonly"
            },
            "settings": {},
            "rules": merged,
            "overrides":overrides
        }))
        .unwrap();

        let config = ConfigStoreBuilder::from_oxlintrc(true, rc).build().unwrap();

        let lint = Linter::new(
            LintOptions { fix: FixKind::None, framework_hints: FrameworkFlags::NextOnly },
            config,
        );

        // lint.run(&path, semantic, module_record);
    }
}
