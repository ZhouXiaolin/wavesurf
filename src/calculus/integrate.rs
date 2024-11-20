use crate::Expression;
use super::IntegrationError;
use super::integration_rules::IntegrationTable;
use lazy_static::lazy_static;
lazy_static! {
    static ref INTEGRATION_TABLE: IntegrationTable = IntegrationTable::new();
}

impl Expression {
    pub fn integrate(&self, var: &str) -> Result<Expression, IntegrationError> {
        // First try to find a matching rule in the integration table
        if let Some(result) = INTEGRATION_TABLE.lookup(self, var) {
            return result;
        }

        // If no rule matches, fall back to the existing integration logic
        match self {
            Expression::Constant(c) => {
                // ∫c dx = cx
                Ok(Expression::multiply(
                    Expression::constant(*c),
                    Expression::variable(var),
                ))
            }
            Expression::Variable(name) => {
                if name == var {
                    // ∫x dx = x²/2
                    Ok(Expression::divide(
                        Expression::power(
                            Expression::variable(var),
                            Expression::constant(2.0),
                        ),
                        Expression::constant(2.0),
                    ))
                } else {
                    // ∫c dx = cx where c is another variable
                    Ok(Expression::multiply(
                        Expression::variable(name),
                        Expression::variable(var),
                    ))
                }
            }
            Expression::Add(left, right) => {
                // ∫(f + g) dx = ∫f dx + ∫g dx
                let left_int = (**left).clone().integrate(var)?;
                let right_int = (**right).clone().integrate(var)?;
                Ok(Expression::add(left_int, right_int))
            }
            Expression::Subtract(left, right) => {
                // ∫(f - g) dx = ∫f dx - ∫g dx
                let left_int = (**left).clone().integrate(var)?;
                let right_int = (**right).clone().integrate(var)?;
                Ok(Expression::subtract(left_int, right_int))
            }
            Expression::Multiply(left, right) => {
                match (&**left, &**right) {
                    (Expression::Constant(c), expr) | (expr, Expression::Constant(c)) => {
                        // ∫c*f dx = c*∫f dx
                        let int = expr.integrate(var)?;
                        Ok(Expression::multiply(Expression::constant(*c), int))
                    }
                    (Expression::Variable(name), Expression::Variable(name2)) if name == name2 => {
                        // ∫x² dx = x³/3
                        if name == var {
                            Ok(Expression::divide(
                                Expression::power(
                                    Expression::variable(name),
                                    Expression::constant(3.0)
                                ),
                                Expression::constant(3.0)
                            ))
                        } else {
                            Err(IntegrationError("Cannot integrate this product".to_string()))
                        }
                    }
                    _ => Err(IntegrationError("Integration of general products not implemented".to_string()))
                }
            }
            Expression::Power(base, exponent) => {
                match (&**base, &**exponent) {
                    (Expression::Variable(name), Expression::Constant(n)) if name == var => {
                        // ∫x^n dx = x^(n+1)/(n+1) for n ≠ -1
                        if (n - (-1.0)).abs() > 1e-10 {
                            Ok(Expression::divide(
                                Expression::power(
                                    Expression::variable(var),
                                    Expression::constant(n + 1.0),
                                ),
                                Expression::constant(n + 1.0),
                            ))
                        } else {
                            // ∫x^(-1) dx = ln|x|
                            Ok(Expression::ln(Expression::variable(var)))
                        }
                    }
                    _ => {
                        Err(IntegrationError("Integration of general powers not implemented".to_string()))
                    }
                }
            }
            Expression::Root(base, n) => {
                // Convert root to power and integrate
                let power = Expression::divide(Expression::constant(1.0), (**n).clone());
                Expression::power((**base).clone(), power).integrate(var)
            }
            Expression::Divide(num, den) => {
                match (&**den, &**num) {
                    // ∫ 1/x dx = ln|x| + C
                    (Expression::Variable(v), Expression::Constant(c)) if *c == 1.0 && v == var => {
                        Ok(Expression::multiply(
                            Expression::constant(1.0),
                            Expression::ln(Expression::variable(var))
                        ))
                    }
                    _ => Err(IntegrationError("Cannot integrate this division".to_string()))
                }
            }
            Expression::Sin(expr) => {
                match &**expr {
                    // ∫ sin(x) dx = -cos(x) + C
                    Expression::Variable(v) if v == var => {
                        Ok(Expression::multiply(
                            Expression::constant(-1.0),
                            Expression::cos(Expression::variable(var))
                        ))
                    }
                    _ => Err(IntegrationError("Cannot integrate sin of complex expression".to_string()))
                }
            }
            Expression::Cos(expr) => {
                match &**expr {
                    // ∫ cos(x) dx = sin(x) + C
                    Expression::Variable(v) if v == var => {
                        Ok(Expression::sin(Expression::variable(var)))
                    }
                    _ => Err(IntegrationError("Cannot integrate cos of complex expression".to_string()))
                }
            }
            Expression::Tan(expr) => {
                match &**expr {
                    // ∫ tan(x) dx = -ln|cos(x)| + C
                    Expression::Variable(v) if v == var => {
                        Ok(Expression::multiply(
                            Expression::constant(-1.0),
                            Expression::ln(Expression::cos(Expression::variable(var)))
                        ))
                    }
                    _ => Err(IntegrationError("Cannot integrate tan of complex expression".to_string()))
                }
            }
            Expression::Exp(expr) => {
                match &**expr {
                    // ∫ e^x dx = e^x + C
                    Expression::Variable(v) if v == var => {
                        Ok(Expression::exp(Expression::variable(var)))
                    }
                    _ => Err(IntegrationError("Cannot integrate exp of complex expression".to_string()))
                }
            }
            Expression::Ln(expr) => {
                match &**expr {
                    // ∫ ln(x) dx = x*ln(x) - x + C
                    Expression::Variable(v) if v == var => {
                        Ok(Expression::subtract(
                            Expression::multiply(
                                Expression::variable(var),
                                Expression::ln(Expression::variable(var))
                            ),
                            Expression::variable(var)
                        ))
                    }
                    _ => Err(IntegrationError("Cannot integrate ln of complex expression".to_string()))
                }
            }
            Expression::Arcsin(_expr) => {
                Err(IntegrationError("Integration of arcsin not implemented".to_string()))
            }
            Expression::Arccos(_expr) => {
                Err(IntegrationError("Integration of arccos not implemented".to_string()))
            }
            Expression::Arctan(_expr) => {
                Err(IntegrationError("Integration of arctan not implemented".to_string()))
            }
            Expression::Log(_base, _expr) => {
                Err(IntegrationError("Integration of logarithm with arbitrary base not implemented".to_string()))
            }
            Expression::Sinh(expr) => {
                match &**expr {
                    // ∫ sinh(x) dx = cosh(x) + C
                    Expression::Variable(v) if v == var => {
                        Ok(Expression::cosh(Expression::variable(var)))
                    }
                    _ => Err(IntegrationError("Cannot integrate sinh of complex expression".to_string()))
                }
            }
            Expression::Cosh(expr) => {
                match &**expr {
                    // ∫ cosh(x) dx = sinh(x) + C
                    Expression::Variable(v) if v == var => {
                        Ok(Expression::sinh(Expression::variable(var)))
                    }
                    _ => Err(IntegrationError("Cannot integrate cosh of complex expression".to_string()))
                }
            }
            Expression::Tanh(expr) => {
                match &**expr {
                    // ∫ tanh(x) dx = ln(cosh(x)) + C
                    Expression::Variable(v) if v == var => {
                        Ok(Expression::ln(Expression::cosh(Expression::variable(var))))
                    }
                    _ => Err(IntegrationError("Cannot integrate tanh of complex expression".to_string()))
                }
            }
        }
    }
}
