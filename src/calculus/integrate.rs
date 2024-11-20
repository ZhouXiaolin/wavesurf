use crate::Expression;
use super::IntegrationError;
use super::integration_rules::IntegrationTable;
use super::integration_state::{IntegrationState, IntegrationMethod};
use super::differentiate::Differentiate;
use lazy_static::lazy_static;

type IntegrationFn = Box<dyn Fn(&Expression, &str, &mut IntegrationState) -> Result<Expression, IntegrationError>>;

lazy_static! {
    static ref INTEGRATION_TABLE: IntegrationTable = IntegrationTable::new();
}

impl Expression {
    pub fn integrate(&self, var: &str) -> Result<Expression, IntegrationError> {
        let mut state = IntegrationState::new(5); // 默认最大深度为5
        self.integrate_with_state(var, &mut state)
    }

    pub fn integrate_with_state(&self, var: &str, state: &mut IntegrationState) -> Result<Expression, IntegrationError> {
        if state.should_prune(self) {
            return Err(IntegrationError::MaxDepthExceeded);
        }

        // 首先尝试使用积分表
        if let Some(result) = INTEGRATION_TABLE.lookup(self, var) {
            return result;
        }

        // 尝试各种积分方法
        let methods: [(IntegrationMethod, IntegrationFn); 3] = [
            (IntegrationMethod::Direct, Box::new(|expr, v, s| expr.try_direct_integration(v, s))),
            (IntegrationMethod::ByParts, Box::new(|expr, v, s| expr.try_integration_by_parts(v, s))),
            (IntegrationMethod::Substitution, Box::new(|expr, v, s| expr.try_substitution(v, s))),
        ];

        for (method, integration_fn) in methods.iter() {
            let prev_method = state.get_method();
            state.set_method(method.clone());
            
            if let Ok(result) = integration_fn(self, var, state) {
                return Ok(result);
            }
            
            state.set_method(prev_method);
        }

        Err(IntegrationError::NoMethodFound)
    }

    fn try_direct_integration(&self, var: &str, _state: &mut IntegrationState) -> Result<Expression, IntegrationError> {
        match self {
            Expression::Constant(c) => {
                Ok(Expression::multiply(
                    Expression::constant(*c),
                    Expression::variable(var),
                ))
            }
            Expression::Variable(name) => {
                if name == var {
                    Ok(Expression::divide(
                        Expression::power(
                            Expression::variable(var),
                            Expression::constant(2.0),
                        ),
                        Expression::constant(2.0),
                    ))
                } else {
                    Ok(Expression::multiply(
                        Expression::variable(name),
                        Expression::variable(var),
                    ))
                }
            }
            Expression::Add(left, right) => {
                let left_int = (**left).clone().integrate_with_state(var, _state)?;
                let right_int = (**right).clone().integrate_with_state(var, _state)?;
                Ok(Expression::add(left_int, right_int))
            }
            Expression::Subtract(left, right) => {
                let left_int = (**left).clone().integrate_with_state(var, _state)?;
                let right_int = (**right).clone().integrate_with_state(var, _state)?;
                Ok(Expression::subtract(left_int, right_int))
            }
            _ => Err(IntegrationError::NoMethodFound),
        }
    }

    fn try_integration_by_parts(&self, var: &str, state: &mut IntegrationState) -> Result<Expression, IntegrationError> {
        state.push_expression(self.clone());
        
        match self {
            Expression::Multiply(left, right) => {
                let (u, dv) = self.choose_u_dv(left, right)?;
                
                // 计算du
                let du = u.differentiate(var);
                
                // 计算v (∫dv)
                let v = dv.integrate_with_state(var, state)?;
                
                // 计算∫v·du
                let v_du = Expression::multiply(v.clone(), du);
                let v_du_int = v_du.integrate_with_state(var, state)?;
                
                // 最终结果：u·v - ∫v·du
                let result = Expression::subtract(
                    Expression::multiply(u, v),
                    v_du_int
                );
                
                state.pop_expression();
                Ok(result)
            }
            _ => {
                state.pop_expression();
                Err(IntegrationError::NoMethodFound)
            }
        }
    }

    fn try_substitution(&self, _var: &str, _state: &mut IntegrationState) -> Result<Expression, IntegrationError> {
        // TODO: 实现替换积分
        // 使用 _var 和 _state 来避免未使用变量警告
        let _ = _var;
        let _ = _state;
        Err(IntegrationError::NotImplemented)
    }

    fn choose_u_dv(&self, left: &Expression, right: &Expression) -> Result<(Expression, Expression), IntegrationError> {
        // 评估哪个部分更适合作为u
        let left_score = self.get_integration_difficulty_score(left);
        let right_score = self.get_integration_difficulty_score(right);
        
        if left_score <= right_score {
            Ok((left.clone(), right.clone()))
        } else {
            Ok((right.clone(), left.clone()))
        }
    }

    fn get_integration_difficulty_score(&self, expr: &Expression) -> i32 {
        match expr {
            Expression::Constant(_) => 1,
            Expression::Variable(_) => 2,
            Expression::Add(_, _) | Expression::Subtract(_, _) => 3,
            Expression::Multiply(_, _) => 4,
            Expression::Divide(_, _) => 5,
            Expression::Power(_, _) => 6,
            Expression::Ln(_) => 7,
            Expression::Sin(_) | Expression::Cos(_) => 4,
            Expression::Tan(_) => 5,
            Expression::Exp(_) => 4,
            _ => 10,
        }
    }
}
