use crate::classify;
use crate::expr::Expr;
use crate::precedence::Precedence;

pub(crate) struct FixupContext {
    // Print expression such that it can be parsed back as a statement
    // consisting of the original expression.
    //
    // The effect of this is for binary operators in statement position to set
    // `leftmost_subexpression_in_stmt` when printing their left-hand operand.
    //
    //     (match x {}) - 1;  // match needs parens when LHS of binary operator
    //
    //     match x {};  // not when its own statement
    //
    #[cfg(feature = "full")]
    stmt: bool,

    // This is the difference between:
    //
    //     (match x {}) - 1;  // subexpression needs parens
    //
    //     let _ = match x {} - 1;  // no parens
    //
    // There are 3 distinguishable contexts in which `print_expr` might be
    // called with the expression `$match` as its argument, where `$match`
    // represents an expression of kind `ExprKind::Match`:
    //
    //   - stmt=false leftmost_subexpression_in_stmt=false
    //
    //     Example: `let _ = $match - 1;`
    //
    //     No parentheses required.
    //
    //   - stmt=false leftmost_subexpression_in_stmt=true
    //
    //     Example: `$match - 1;`
    //
    //     Must parenthesize `($match)`, otherwise parsing back the output as a
    //     statement would terminate the statement after the closing brace of
    //     the match, parsing `-1;` as a separate statement.
    //
    //   - stmt=true leftmost_subexpression_in_stmt=false
    //
    //     Example: `$match;`
    //
    //     No parentheses required.
    #[cfg(feature = "full")]
    leftmost_subexpression_in_stmt: bool,

    // Print expression such that it can be parsed as a match arm.
    //
    // This is almost equivalent to `stmt`, but the grammar diverges a tiny bit
    // between statements and match arms when it comes to braced macro calls.
    // Macro calls with brace delimiter terminate a statement without a
    // semicolon, but do not terminate a match-arm without comma.
    //
    //     m! {} - 1;  // two statements: a macro call followed by -1 literal
    //
    //     match () {
    //         _ => m! {} - 1,  // binary subtraction operator
    //     }
    //
    #[cfg(feature = "full")]
    match_arm: bool,

    // This is almost equivalent to `leftmost_subexpression_in_stmt`, other than
    // for braced macro calls.
    //
    // If we have `m! {} - 1` as an expression, the leftmost subexpression
    // `m! {}` will need to be parenthesized in the statement case but not the
    // match-arm case.
    //
    //     (m! {}) - 1;  // subexpression needs parens
    //
    //     match () {
    //         _ => m! {} - 1,  // no parens
    //     }
    //
    #[cfg(feature = "full")]
    leftmost_subexpression_in_match_arm: bool,

    // This is the difference between:
    //
    //     if let _ = (Struct {}) {}  // needs parens
    //
    //     match () {
    //         () if let _ = Struct {} => {}  // no parens
    //     }
    //
    #[cfg(feature = "full")]
    parenthesize_exterior_struct_lit: bool,

    // This is the difference between:
    //
    //     let _ = 1 + return 1;  // no parens if rightmost subexpression
    //
    //     let _ = 1 + (return 1) + 1;  // needs parens
    //
    #[cfg(feature = "full")]
    parenthesize_exterior_jump: bool,

    // This is the difference between:
    //
    //     let _ = (return) - 1;  // without paren, this would return -1
    //
    //     let _ = return + 1;  // no paren because '+' cannot begin expr
    //
    #[cfg(feature = "full")]
    next_operator_can_begin_expr: bool,

    // This is the difference between:
    //
    //     let _ = x as u8 + T;
    //
    //     let _ = (x as u8) < T;
    //
    // Without parens, the latter would want to parse `u8<T...` as a type.
    next_operator_can_begin_generics: bool,
}

impl FixupContext {
    /// The default amount of fixing is minimal fixing. Fixups should be turned
    /// on in a targeted fashion where needed.
    pub const NONE: Self = FixupContext {
        #[cfg(feature = "full")]
        stmt: false,
        #[cfg(feature = "full")]
        leftmost_subexpression_in_stmt: false,
        #[cfg(feature = "full")]
        match_arm: false,
        #[cfg(feature = "full")]
        leftmost_subexpression_in_match_arm: false,
        #[cfg(feature = "full")]
        parenthesize_exterior_struct_lit: false,
        #[cfg(feature = "full")]
        parenthesize_exterior_jump: false,
        #[cfg(feature = "full")]
        next_operator_can_begin_expr: false,
        next_operator_can_begin_generics: false,
    };

    /// Create the initial fixup for printing an expression in statement
    /// position.
    #[cfg(feature = "full")]
    pub fn new_stmt() -> Self {
        FixupContext {
            stmt: true,
            ..FixupContext::NONE
        }
    }

    /// Create the initial fixup for printing an expression as the right-hand
    /// side of a match arm.
    #[cfg(feature = "full")]
    pub fn new_match_arm() -> Self {
        FixupContext {
            match_arm: true,
            ..FixupContext::NONE
        }
    }

    /// Create the initial fixup for printing an expression as the "condition"
    /// of an `if` or `while`. There are a few other positions which are
    /// grammatically equivalent and also use this, such as the iterator
    /// expression in `for` and the scrutinee in `match`.
    #[cfg(feature = "full")]
    pub fn new_condition() -> Self {
        FixupContext {
            parenthesize_exterior_struct_lit: true,
            ..FixupContext::NONE
        }
    }

    /// Transform this fixup into the one that should apply when printing the
    /// leftmost subexpression of the current expression.
    ///
    /// The leftmost subexpression is any subexpression that has the same first
    /// token as the current expression, but has a different last token.
    ///
    /// For example in `$a + $b` and `$a.method()`, the subexpression `$a` is a
    /// leftmost subexpression.
    ///
    /// Not every expression has a leftmost subexpression. For example neither
    /// `-$a` nor `[$a]` have one.
    pub fn leftmost_subexpression(self) -> Self {
        FixupContext {
            #[cfg(feature = "full")]
            stmt: false,
            #[cfg(feature = "full")]
            leftmost_subexpression_in_stmt: self.stmt || self.leftmost_subexpression_in_stmt,
            #[cfg(feature = "full")]
            match_arm: false,
            #[cfg(feature = "full")]
            leftmost_subexpression_in_match_arm: self.match_arm
                || self.leftmost_subexpression_in_match_arm,
            #[cfg(feature = "full")]
            parenthesize_exterior_jump: true,
            ..self
        }
    }

    /// Transform this fixup into the one that should apply when printing a
    /// leftmost subexpression followed by a `.` or `?` token, which confer
    /// different statement boundary rules compared to other leftmost
    /// subexpressions.
    pub fn leftmost_subexpression_with_dot(self) -> Self {
        FixupContext {
            #[cfg(feature = "full")]
            stmt: self.stmt || self.leftmost_subexpression_in_stmt,
            #[cfg(feature = "full")]
            leftmost_subexpression_in_stmt: false,
            #[cfg(feature = "full")]
            match_arm: self.match_arm || self.leftmost_subexpression_in_match_arm,
            #[cfg(feature = "full")]
            leftmost_subexpression_in_match_arm: false,
            #[cfg(feature = "full")]
            parenthesize_exterior_jump: true,
            ..self
        }
    }

    /// Transform this fixup into the one that should apply when printing a
    /// leftmost subexpression followed by punctuation that is legal as the
    /// first token of an expression.
    pub fn leftmost_subexpression_with_begin_operator(
        self,
        #[cfg(feature = "full")] next_operator_can_begin_expr: bool,
        next_operator_can_begin_generics: bool,
    ) -> Self {
        FixupContext {
            #[cfg(feature = "full")]
            next_operator_can_begin_expr,
            next_operator_can_begin_generics,
            ..self.leftmost_subexpression()
        }
    }

    /// Transform this fixup into the one that should apply when printing any
    /// subexpression that is neither a leftmost subexpression nor surrounded in
    /// delimiters.
    ///
    /// This is for any subexpression that has a different first token than the
    /// current expression, and is not surrounded by a paren/bracket/brace. For
    /// example the `$b` in `$a + $b` and `-$b`, but not the one in `[$b]` or
    /// `$a.f($b)`.
    pub fn subsequent_subexpression(self) -> Self {
        FixupContext {
            #[cfg(feature = "full")]
            stmt: false,
            #[cfg(feature = "full")]
            leftmost_subexpression_in_stmt: false,
            #[cfg(feature = "full")]
            match_arm: false,
            #[cfg(feature = "full")]
            leftmost_subexpression_in_match_arm: false,
            ..self
        }
    }

    /// Determine whether parentheses are needed around the given expression to
    /// head off an unintended statement boundary.
    ///
    /// The documentation on `FixupContext::leftmost_subexpression_in_stmt` has
    /// examples.
    #[cfg(feature = "full")]
    pub fn would_cause_statement_boundary(self, expr: &Expr) -> bool {
        (self.leftmost_subexpression_in_stmt && !classify::requires_semi_to_be_stmt(expr))
            || ((self.stmt || self.leftmost_subexpression_in_stmt) && matches!(expr, Expr::Let(_)))
            || (self.leftmost_subexpression_in_match_arm
                && !classify::requires_comma_to_be_match_arm(expr))
    }

    /// Determine whether parentheses are needed around the given `let`
    /// scrutinee.
    ///
    /// In `if let _ = $e {}`, some examples of `$e` that would need parentheses
    /// are:
    ///
    ///   - `Struct {}.f()`, because otherwise the `{` would be misinterpreted
    ///     as the opening of the if's then-block.
    ///
    ///   - `true && false`, because otherwise this would be misinterpreted as a
    ///     "let chain".
    #[cfg(feature = "full")]
    pub fn needs_group_as_let_scrutinee(self, expr: &Expr) -> bool {
        self.parenthesize_exterior_struct_lit && classify::confusable_with_adjacent_block(expr)
            || self.trailing_precedence(expr) < Precedence::Let
    }

    /// Determines the effective precedence of a left subexpression. Some
    /// expressions have lower precedence when adjacent to particular operators.
    pub fn leading_precedence(self, expr: &Expr) -> Precedence {
        #[cfg(feature = "full")]
        if self.next_operator_can_begin_expr {
            // Decrease precedence of value-less jumps when followed by an
            // operator that would otherwise get interpreted as beginning a
            // value for the jump.
            if let Expr::Break(_) | Expr::Return(_) | Expr::Yield(_) = expr {
                return Precedence::Jump;
            }
        }
        self.precedence(expr)
    }

    /// Determines the effective precedence of a right subexpression. Some
    /// expressions have higher precedence on the right side of a binary
    /// operator than on the left.
    pub fn trailing_precedence(self, expr: &Expr) -> Precedence {
        #[cfg(feature = "full")]
        if !self.parenthesize_exterior_jump {
            match expr {
                // Increase precedence of expressions that extend to the end of
                // current statement or group.
                Expr::Break(_)
                | Expr::Closure(_)
                | Expr::Let(_)
                | Expr::Return(_)
                | Expr::Yield(_) => {
                    return Precedence::Prefix;
                }
                Expr::Range(e) if e.start.is_none() => return Precedence::Prefix,
                _ => {}
            }
        }
        self.precedence(expr)
    }

    fn precedence(self, expr: &Expr) -> Precedence {
        if self.next_operator_can_begin_generics {
            if let Expr::Cast(cast) = expr {
                if classify::trailing_unparameterized_path(&cast.ty) {
                    return Precedence::MIN;
                }
            }
        }
        Precedence::of(expr)
    }
}

impl Copy for FixupContext {}

impl Clone for FixupContext {
    fn clone(&self) -> Self {
        *self
    }
}
