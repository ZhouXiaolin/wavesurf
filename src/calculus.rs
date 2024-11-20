use crate::expression::Expression;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct IntegrationError(String);

impl fmt::Display for IntegrationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for IntegrationError {}

impl Expression {
    pub fn differentiate(&self, var: &str) -> Expression {
        match self {
            Expression::Constant(_) => Expression::constant(0.0),
            Expression::Variable(name) => {
                if name == var {
                    Expression::constant(1.0)
                } else {
                    Expression::constant(0.0)
                }
            }
            Expression::Add(left, right) => {
                Expression::add(left.differentiate(var), right.differentiate(var))
            }
            Expression::Subtract(left, right) => {
                Expression::subtract(left.differentiate(var), right.differentiate(var))
            }
            Expression::Multiply(left, right) => {
                // Product rule: d(u*v) = u*dv + v*du
                let du_v = Expression::multiply(left.differentiate(var), (**right).clone());
                let u_dv = Expression::multiply((**left).clone(), right.differentiate(var));
                Expression::add(du_v, u_dv)
            }
            Expression::Divide(left, right) => {
                // Quotient rule: d(u/v) = (v*du - u*dv)/(v^2)
                let v_du = Expression::multiply((**right).clone(), left.differentiate(var));
                let u_dv = Expression::multiply((**left).clone(), right.differentiate(var));
                let numerator = Expression::subtract(v_du, u_dv);
                let denominator = Expression::power((**right).clone(), Expression::constant(2.0));
                Expression::divide(numerator, denominator)
            }
            Expression::Power(base, exponent) => {
                match &**exponent {
                    Expression::Constant(n) => {
                        // Power rule: d(x^n) = n*x^(n-1)*dx
                        let new_power = Expression::power(
                            (**base).clone(),
                            Expression::constant(n - 1.0),
                        );
                        Expression::multiply(
                            Expression::constant(*n),
                            Expression::multiply(new_power, base.differentiate(var)),
                        )
                    }
                    _ => {
                        // General case using logarithmic differentiation
                        let ln_base = Expression::ln((**base).clone());
                        let derivative = Expression::multiply(
                            (**exponent).clone(),
                            Expression::multiply(ln_base, base.differentiate(var)),
                        );
                        derivative
                    }
                }
            }
            Expression::Root(base, n) => {
                // Convert root to power and differentiate
                let power = Expression::divide(Expression::constant(1.0), (**n).clone());
                Expression::power((**base).clone(), power).differentiate(var)
            }
            Expression::Sin(expr) => {
                // d/dx sin(u) = cos(u) * du/dx
                Expression::multiply(
                    Expression::cos((**expr).clone()),
                    expr.differentiate(var)
                )
            }
            Expression::Cos(expr) => {
                // d/dx cos(u) = -sin(u) * du/dx
                Expression::multiply(
                    Expression::multiply(
                        Expression::constant(-1.0),
                        Expression::sin((**expr).clone())
                    ),
                    expr.differentiate(var)
                )
            }
            Expression::Tan(expr) => {
                // d/dx tan(u) = sec²(u) * du/dx = (1 / cos²(u)) * du/dx
                Expression::multiply(
                    Expression::divide(
                        Expression::constant(1.0),
                        Expression::power(
                            Expression::cos((**expr).clone()),
                            Expression::constant(2.0)
                        )
                    ),
                    expr.differentiate(var)
                )
            }
            Expression::Arcsin(expr) => {
                // d/dx arcsin(x) = 1/sqrt(1 - x^2)
                let one = Expression::constant(1.0);
                let two = Expression::constant(2.0);
                let inner_deriv = (**expr).clone().differentiate(var);
                let denom = Expression::power(
                    Expression::subtract(one, Expression::power((**expr).clone(), two)),
                    Expression::constant(0.5)
                );
                Expression::multiply(inner_deriv, Expression::divide(Expression::constant(1.0), denom))
            }
            Expression::Arccos(expr) => {
                // d/dx arccos(x) = -1/sqrt(1 - x^2)
                let one = Expression::constant(1.0);
                let two = Expression::constant(2.0);
                let inner_deriv = (**expr).clone().differentiate(var);
                let denom = Expression::power(
                    Expression::subtract(one, Expression::power((**expr).clone(), two)),
                    Expression::constant(0.5)
                );
                Expression::multiply(
                    inner_deriv,
                    Expression::multiply(
                        Expression::constant(-1.0),
                        Expression::divide(Expression::constant(1.0), denom)
                    )
                )
            }
            Expression::Arctan(expr) => {
                // d/dx arctan(x) = 1/(1 + x^2)
                let one = Expression::constant(1.0);
                let two = Expression::constant(2.0);
                let inner_deriv = (**expr).clone().differentiate(var);
                let denom = Expression::add(one, Expression::power((**expr).clone(), two));
                Expression::multiply(inner_deriv, Expression::divide(Expression::constant(1.0), denom))
            }
            Expression::Exp(expr) => {
                // d/dx e^u = e^u * du/dx
                Expression::multiply(
                    Expression::exp((**expr).clone()),
                    (**expr).clone().differentiate(var)
                )
            }
            Expression::Ln(expr) => {
                // d/dx ln(u) = 1/u * du/dx
                Expression::multiply(
                    Expression::divide(
                        Expression::constant(1.0),
                        (**expr).clone()
                    ),
                    (**expr).clone().differentiate(var)
                )
            }
            Expression::Log(base, expr) => {
                // d/dx log_b(u) = 1/(u * ln(b))
                let inner_deriv = (**expr).clone().differentiate(var);
                let denom = Expression::multiply(
                    (**expr).clone(),
                    Expression::ln((**base).clone())
                );
                Expression::multiply(inner_deriv, Expression::divide(Expression::constant(1.0), denom))
            }
            Expression::Sinh(expr) => {
                // d/dx sinh(u) = cosh(u) * du/dx
                Expression::multiply(
                    Expression::cosh((**expr).clone()),
                    (**expr).clone().differentiate(var)
                )
            }
            Expression::Cosh(expr) => {
                // d/dx cosh(u) = sinh(u) * du/dx
                Expression::multiply(
                    Expression::sinh((**expr).clone()),
                    (**expr).clone().differentiate(var)
                )
            }
            Expression::Tanh(expr) => {
                // d/dx tanh(u) = sech²(u) * du/dx = (1 - tanh²(u)) * du/dx
                Expression::multiply(
                    Expression::subtract(
                        Expression::constant(1.0),
                        Expression::power(
                            Expression::tanh((**expr).clone()),
                            Expression::constant(2.0)
                        )
                    ),
                    (**expr).clone().differentiate(var)
                )
            }
        }
    }

    pub fn integrate(&self, var: &str) -> Result<Expression, IntegrationError> {
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
