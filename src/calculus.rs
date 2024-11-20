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
                let left_int = left.integrate(var)?;
                let right_int = right.integrate(var)?;
                Ok(Expression::add(left_int, right_int))
            }
            Expression::Subtract(left, right) => {
                // ∫(f - g) dx = ∫f dx - ∫g dx
                let left_int = left.integrate(var)?;
                let right_int = right.integrate(var)?;
                Ok(Expression::subtract(left_int, right_int))
            }
            Expression::Multiply(left, right) => {
                match (&**left, &**right) {
                    (Expression::Constant(c), expr) | (expr, Expression::Constant(c)) => {
                        // ∫(c*f) dx = c*∫f dx
                        let int = expr.integrate(var)?;
                        Ok(Expression::multiply(Expression::constant(*c), int))
                    }
                    (Expression::Variable(name), Expression::Variable(name2)) if name == name2 => {
                        // ∫x² dx = x³/3
                        if name == var {
                            Ok(Expression::divide(
                                Expression::power(
                                    Expression::variable(var),
                                    Expression::constant(3.0),
                                ),
                                Expression::constant(3.0),
                            ))
                        } else {
                            Err(IntegrationError("Cannot integrate product of different variables".to_string()))
                        }
                    }
                    _ => {
                        Err(IntegrationError("Integration of general products not implemented".to_string()))
                    }
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
            _ => Err(IntegrationError("Integration not implemented for this expression type".to_string())),
        }
    }
}
