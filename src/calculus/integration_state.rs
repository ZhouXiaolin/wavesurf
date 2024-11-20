use crate::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum IntegrationMethod {
    Direct,
    ByParts,
    Substitution,
    TrigonometricSubstitution,
    RationalFunction,
}

#[derive(Debug, Clone)]
pub struct IntegrationState {
    depth: usize,
    max_depth: usize,
    visited_expressions: Vec<Expression>,
    current_method: IntegrationMethod,
}

impl IntegrationState {
    pub fn new(max_depth: usize) -> Self {
        IntegrationState {
            depth: 0,
            max_depth,
            visited_expressions: Vec::new(),
            current_method: IntegrationMethod::Direct,
        }
    }

    pub fn should_prune(&self, expr: &Expression) -> bool {
        if self.depth >= self.max_depth {
            return true;
        }
        self.visited_expressions.iter().any(|visited| self.is_similar(visited, expr))
    }

    fn is_similar(&self, expr1: &Expression, expr2: &Expression) -> bool {
        // 基本相等检查
        if expr1 == expr2 {
            return true;
        }

        // 结构相似性检查
        match (expr1, expr2) {
            // 检查乘法交换律
            (Expression::Multiply(a1, b1), Expression::Multiply(a2, b2)) => {
                (self.is_similar(a1, a2) && self.is_similar(b1, b2)) ||
                (self.is_similar(a1, b2) && self.is_similar(b1, a2))
            }
            // 检查加法交换律
            (Expression::Add(a1, b1), Expression::Add(a2, b2)) => {
                (self.is_similar(a1, a2) && self.is_similar(b1, b2)) ||
                (self.is_similar(a1, b2) && self.is_similar(b1, a2))
            }
            // 检查减法（考虑可能的重排）
            (Expression::Subtract(a1, b1), Expression::Subtract(a2, b2)) => {
                self.is_similar(a1, a2) && self.is_similar(b1, b2)
            }
            // 检查函数调用
            (Expression::Sin(a1), Expression::Sin(a2)) |
            (Expression::Cos(a1), Expression::Cos(a2)) |
            (Expression::Tan(a1), Expression::Tan(a2)) |
            (Expression::Exp(a1), Expression::Exp(a2)) |
            (Expression::Ln(a1), Expression::Ln(a2)) => {
                self.is_similar(a1, a2)
            }
            // 其他情况认为不相似
            _ => false,
        }
    }

    pub fn push_expression(&mut self, expr: Expression) {
        self.visited_expressions.push(expr);
        self.depth += 1;
    }

    pub fn pop_expression(&mut self) {
        self.visited_expressions.pop();
        if self.depth > 0 {
            self.depth -= 1;
        }
    }

    pub fn set_method(&mut self, method: IntegrationMethod) {
        self.current_method = method;
    }

    pub fn get_method(&self) -> IntegrationMethod {
        self.current_method.clone()
    }
}
