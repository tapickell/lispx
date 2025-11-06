use crate::types::Expr;
use crate::types::Expr::*;

pub(crate) fn eval(ast: Expr) -> f32 {
    match ast {
        ENum(number) => number,
        EAdd(exp1, exp2) => eval(*exp1) + eval(*exp2),
        ESub(exp1, exp2) => eval(*exp1) - eval(*exp2),
        EMul(exp1, exp2) => eval(*exp1) * eval(*exp2),
        EDiv(exp1, exp2) => eval(*exp1) / eval(*exp2),
        EExp(exp1, exp2) => eval(*exp1).powf(eval(*exp2)),
    }
}

#[cfg(test)]
mod tests {
    use crate::evaluator::eval;
    use crate::types::Expr::*;

    #[test]
    fn eval_simple_example() {
        let ast = EAdd(Box::new(ENum(4.0)), Box::new(ENum(2.0)));
        assert_eq!(eval(ast), 6.0);
    }

    #[test]
    fn eval_large_example() {
        let ast = EAdd(
            Box::new(ENum(4.0)),
            Box::new(EMul(
                Box::new(EDiv(Box::new(ENum(40.0)), Box::new(ENum(20.0)))),
                Box::new(ENum(2.0)),
            )),
        );
        assert_eq!(eval(ast), 8.0);
    }
}
