use gitql_ast::expression::Expr;
use gitql_ast::statement::NullsOrderPolicy;
use gitql_ast::statement::OrderByStatement;
use gitql_ast::statement::SortingOrder;
use gitql_ast::statement::Statement;
use gitql_core::environment::Environment;

use crate::context::ParserContext;
use crate::diagnostic::Diagnostic;
use crate::parser::consume_token_or_error;
use crate::parser::is_current_token;
use crate::parser::parse_expression;
use crate::token::Token;
use crate::token::TokenKind;

pub(crate) fn parse_order_by_statement(
    context: &mut ParserContext,
    env: &mut Environment,
    tokens: &[Token],
    position: &mut usize,
) -> Result<Statement, Box<Diagnostic>> {
    // Consume `ORDER` keyword
    *position += 1;

    context.inside_order_by = true;

    // Consume `BY` keyword
    consume_token_or_error(
        tokens,
        position,
        TokenKind::By,
        "Expect keyword `BY` after keyword `ORDER",
    )?;

    let mut arguments: Vec<Box<dyn Expr>> = vec![];
    let mut sorting_orders: Vec<SortingOrder> = vec![];
    let mut null_ordering_policies: Vec<NullsOrderPolicy> = vec![];

    loop {
        let argument = parse_expression(context, env, tokens, position)?;
        let sorting_order = parse_sorting_order(tokens, position)?;
        let null_ordering_policy = parse_order_by_nulls_policy(tokens, position, &sorting_order)?;

        arguments.push(argument);
        sorting_orders.push(sorting_order);
        null_ordering_policies.push(null_ordering_policy);

        if is_current_token(tokens, position, TokenKind::Comma) {
            // Consume `,` keyword
            *position += 1;
        } else {
            break;
        }
    }

    context.inside_order_by = false;

    Ok(Statement::OrderBy(OrderByStatement {
        arguments,
        sorting_orders,
        nulls_order_policies: null_ordering_policies,
    }))
}

fn parse_sorting_order(
    tokens: &[Token],
    position: &mut usize,
) -> Result<SortingOrder, Box<Diagnostic>> {
    let mut sorting_order = SortingOrder::Ascending;
    if *position >= tokens.len() {
        return Ok(sorting_order);
    }

    // Parse `ASC` or `DESC`
    if is_asc_or_desc(&tokens[*position]) {
        if tokens[*position].kind == TokenKind::Descending {
            sorting_order = SortingOrder::Descending;
        }

        // Consume `ASC or DESC` keyword
        *position += 1;
        return Ok(sorting_order);
    }

    // Parse `USING <Operator>`
    if tokens[*position].kind == TokenKind::Using {
        // Consume `USING` keyword
        *position += 1;

        if *position < tokens.len() && is_order_by_using_operator(&tokens[*position]) {
            if tokens[*position].kind == TokenKind::Greater {
                sorting_order = SortingOrder::Descending;
            }

            // Consume `> or <` keyword
            *position += 1;
            return Ok(sorting_order);
        }

        return Err(Diagnostic::error("Expect `>` or `<` after `USING` keyword")
            .with_location(tokens[*position - 1].location)
            .as_boxed());
    }

    // Return default sorting order
    Ok(sorting_order)
}

fn parse_order_by_nulls_policy(
    tokens: &[Token],
    position: &mut usize,
    sorting_order: &SortingOrder,
) -> Result<NullsOrderPolicy, Box<Diagnostic>> {
    // Check for `NULLs FIRST` or `NULLs LAST`
    if is_current_token(tokens, position, TokenKind::Nulls) {
        // Consume `NULLs` keyword
        *position += 1;

        // Consume `FIRST` and return NUlls First policy
        if is_current_token(tokens, position, TokenKind::First) {
            *position += 1;
            return Ok(NullsOrderPolicy::NullsFirst);
        }

        // Consume `LAST` and return NUlls Last policy
        if is_current_token(tokens, position, TokenKind::Last) {
            *position += 1;
            return Ok(NullsOrderPolicy::NullsLast);
        }

        return Err(Diagnostic::error("Unexpected NULL ordering policy")
            .add_note("Null ordering policy must be `FIRST` or `LAST`")
            .add_help("Please use `NULL FIRST` or `NULL LAST`")
            .with_location(tokens[*position].location)
            .as_boxed());
    }

    let default_null_ordering_policy = match sorting_order {
        SortingOrder::Ascending => NullsOrderPolicy::NullsLast,
        SortingOrder::Descending => NullsOrderPolicy::NullsFirst,
    };

    Ok(default_null_ordering_policy)
}

#[inline(always)]
fn is_order_by_using_operator(token: &Token) -> bool {
    matches!(token.kind, TokenKind::Greater | TokenKind::Less)
}

#[inline(always)]
fn is_asc_or_desc(token: &Token) -> bool {
    matches!(token.kind, TokenKind::Ascending | TokenKind::Descending)
}
