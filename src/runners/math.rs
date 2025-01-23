use crate::enums::MathExpr;

/// Evaluate a math expression.
pub fn eval(expression: &MathExpr) -> f64 {
    // Match expression type
    match expression {
        MathExpr::Num(x) => *x,
        MathExpr::Neg(a) => -eval(a),
        MathExpr::Add(a, b) => eval(a) + eval(b),
        MathExpr::Sub(a, b) => eval(a) - eval(b),
        MathExpr::Mul(a, b) => eval(a) * eval(b),
        MathExpr::Div(a, b) => eval(a) / eval(b),
    }
}
