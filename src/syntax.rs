#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
enum SyntaxKind {
    KwFalse,
    KwNone,
    KwTrue,
    KwAnd,
    KwAs,
    KwAssert,
    KwAsync,
    KwAwait,
    KwBreak,
    KwClass,
    KwContinue,
    KwDef,
    KwDel,
    KwElif,
    KwElse,
    KwExcept,
    KwFinally,
    KwFor,
    KwFrom,
    KwGlobal,
    KwIf,
    KwImport,
    KwIn,
    KwIs,
    KwLambda,
    KwNonlocal,
    KwNot,
    KwOr,
    KwPass,
    KwRaise,
    KwReturn,
    KwTry,
    KwWhile,
    KwWith,
    KwYield,
}
use SyntaxKind::*;
impl SyntaxKind {
    fn is_keyword(ident: &str) -> Option<SyntaxKind> {}
}
