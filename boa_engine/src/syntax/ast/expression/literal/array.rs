//! Array declaration Expression.

use crate::syntax::ast::visitor::{VisitWith, Visitor, VisitorMut};
use crate::syntax::ast::{expression::Expression, ContainsSymbol};
use crate::try_break;
use boa_interner::{Interner, ToInternedString};
use core::ops::ControlFlow;

/// An array is an ordered collection of data (either primitive or object depending upon the
/// language).
///
/// Arrays are used to store multiple values in a single variable.
/// This is compared to a variable that can store only one value.
///
/// Each item in an array has a number attached to it, called a numeric index, that allows you
/// to access it. In JavaScript, arrays start at index zero and can be manipulated with various
/// methods.
///
/// More information:
///  - [ECMAScript reference][spec]
///  - [MDN documentation][mdn]
///
/// [spec]: https://tc39.es/ecma262/#prod-ArrayLiteral
/// [mdn]: https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Array
#[cfg_attr(feature = "deser", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct ArrayLiteral {
    arr: Box<[Option<Expression>]>,
    has_trailing_comma_spread: bool,
}

impl ArrayLiteral {
    /// Crate a new array literal.
    pub(crate) fn new<A>(array: A, has_trailing_comma_spread: bool) -> Self
    where
        A: Into<Box<[Option<Expression>]>>,
    {
        Self {
            arr: array.into(),
            has_trailing_comma_spread,
        }
    }

    /// Indicates if a spread operator in the array literal has a trailing comma.
    /// This is a syntax error in some cases.
    pub(crate) fn has_trailing_comma_spread(&self) -> bool {
        self.has_trailing_comma_spread
    }

    #[inline]
    pub(crate) fn contains_arguments(&self) -> bool {
        self.arr
            .iter()
            .flatten()
            .any(Expression::contains_arguments)
    }

    #[inline]
    pub(crate) fn contains(&self, symbol: ContainsSymbol) -> bool {
        self.arr.iter().flatten().any(|expr| expr.contains(symbol))
    }
}

impl AsRef<[Option<Expression>]> for ArrayLiteral {
    #[inline]
    fn as_ref(&self) -> &[Option<Expression>] {
        &self.arr
    }
}

impl<T> From<T> for ArrayLiteral
where
    T: Into<Box<[Option<Expression>]>>,
{
    #[inline]
    fn from(decl: T) -> Self {
        Self {
            arr: decl.into(),
            has_trailing_comma_spread: false,
        }
    }
}

impl ToInternedString for ArrayLiteral {
    #[inline]
    fn to_interned_string(&self, interner: &Interner) -> String {
        let mut buf = String::from("[");
        let mut first = true;
        for e in &*self.arr {
            if first {
                first = false;
            } else {
                buf.push_str(", ");
            }
            if let Some(e) = e {
                buf.push_str(&e.to_interned_string(interner));
            }
        }
        buf.push(']');
        buf
    }
}

impl From<ArrayLiteral> for Expression {
    #[inline]
    fn from(arr: ArrayLiteral) -> Self {
        Self::ArrayLiteral(arr)
    }
}

impl VisitWith for ArrayLiteral {
    fn visit_with<'a, V>(&'a self, visitor: &mut V) -> ControlFlow<V::BreakTy>
    where
        V: Visitor<'a>,
    {
        for expr in self.arr.iter().flatten() {
            try_break!(visitor.visit_expression(expr));
        }
        ControlFlow::Continue(())
    }

    fn visit_with_mut<'a, V>(&'a mut self, visitor: &mut V) -> ControlFlow<V::BreakTy>
    where
        V: VisitorMut<'a>,
    {
        for expr in self.arr.iter_mut().flatten() {
            try_break!(visitor.visit_expression_mut(expr));
        }
        ControlFlow::Continue(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fmt() {
        crate::syntax::ast::test_formatting(
            r#"
            let a = [1, 2, 3, "words", "more words"];
            let b = [];
            "#,
        );
    }
}
