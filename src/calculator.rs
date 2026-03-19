#[derive(Debug, PartialEq)]
pub enum CalcError {
    DivisionByZero,
    InvalidInput(String),
}

/// 解析并计算 "a op b" 格式的表达式，如 "10 / 2"
/// 支持 + - * /，除数为 0 或格式错误返回对应 Err
pub fn calculate(expr: &str) -> Result<f64, CalcError> {
    // TODO: split -> parse -> match op -> 返回 Ok 或 Err
    let lis = expr.split_whitespace().collect::<Vec<_>>();
    let a = lis[0]
        .parse::<f64>()
        .map_err(|_| CalcError::InvalidInput(lis[0].to_string()))?;
    let b = lis[2]
        .parse::<f64>()
        .map_err(|_| CalcError::InvalidInput(lis[0].to_string()))?;
    match lis[1] {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 {
                Err(CalcError::DivisionByZero)
            } else {
                Ok(a / b)
            }
        }
        _ => Err(CalcError::InvalidInput(lis[1].to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(calculate("3 + 4"), Ok(7.0));
    }
    #[test]
    fn test_div() {
        assert_eq!(calculate("10 / 2"), Ok(5.0));
    }
    #[test]
    fn test_zero() {
        assert_eq!(calculate("5 / 0"), Err(CalcError::DivisionByZero));
    }
    #[test]
    fn test_bad() {
        assert!(matches!(calculate("abc"), Err(CalcError::InvalidInput(_))));
    }
}
