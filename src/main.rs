use wavesurf::ExpressionParser;

fn test_expression(expr: &str) {
    println!("\nTesting expression: {}", expr);
    
    let mut parser = ExpressionParser::new(expr);
    match parser.parse() {
        Ok(expr) => {
            println!("Parsed: {}", expr);
            let simplified = expr.simplify();
            println!("Simplified: {}", simplified);
            let derivative = simplified.differentiate("x").simplify();
            println!("Derivative: {}", derivative);
            match simplified.integrate("x") {
                Ok(integral) => println!("Integral: {}", integral.simplify()),
                Err(e) => println!("Integration error: {}", e),
            }
        }
        Err(e) => println!("Parse error: {}", e),
    }
}

fn main() {
    let test_expressions = vec![
        // 基本代数表达式
        "x^2 + 2*x + 1",
        "x^2 + x^2",
        "0 + x",
        "1 * x",
        "x^0",
        "x^1",
        "(x^2 * x^3) / (2*x)",
        
        // 三角函数
        "sin(x)",
        "cos(x)",
        "sin(x)^2 + cos(x)^2",
        "sin(2*x)",
        
        // 指数和对数
        "e^x",
        "ln(x)",
        "e^(2*x)",
        
        // 复合函数
        "sin(x^2)",
        "e^(sin(x))",
        "ln(cos(x))",
        
        // 分数和有理函数
        "(x^2 + 1)/(x - 1)",
        "1/(x^2 + 1)",
        
        // 复杂表达式
        "sin(x)*cos(x) + e^x/x",
        "(x^3 + 3*x^2 + 3*x + 1)/(x + 1)",
        "sin(x)*e^x + ln(x)*cos(x)",
    ];

    for expr in test_expressions {
        test_expression(expr);
    }
}
