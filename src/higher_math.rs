/*
#![feature(proc_macro_hygiene)]
use inline_python::{Context, python};

pub fn solve_rational(equation: String, var: String) -> String{
    let p: Context = python! {
        from sympy.parsing.sympy_parser import parse_expr
        from sympy import Symbol, solve, Eq
        
        x = Symbol('var, real=True)

        lhs = parse_expr('equation.split('=')[0], local_dict={'var:x})
        rhs = parse_expr('equation.split('=')[1], local_dict={'var:x})
        sols = solve(Eq(lhs, rhs), x)
        solutions = str(sols)
    };
    let solution_string: String = (p.get::<String>("solutions"));
    println!("{}", solution_string);
    solution_string
}

fn poly_gcd (poly_1: String, poly_2: String, var: String) -> String {
    let var_replace = &var.to_string();
    let p: Context = python! {
        from sympy import gcd
        from sympy.abc import x
        from sympy.parsing.sympy_parser import parse_expr
        

        poly_x_1 = parse_expr('poly_1, local_dict = {'var:x})
        poly_x_2 = parse_expr('poly_2, local_dict = {'var:x})
        print(gcd(poly_x_1, poly_x_2))
        gcd_sol = str(gcd(poly_x_1, poly_x_2))
    };
    let solution_string: String = (p.get::<String>("gcd_sol"));
    solution_string.replace("x", var_replace)
}

pub fn multi_gcd( polynomials: Vec<String>, var: String) -> String {
    let var_replace = &var.to_string();

    let p: Context = python! {
        from sympy import gcd
        from sympy.abc import x
        from sympy.parsing.sympy_parser import parse_expr
        
        polys = []
        for p in 'polynomials:
            polys.append(parse_expr(p, local_dict = {'var:x}))
        
        #print(gcd(polys))
        gcd_sol = str(gcd(polys))
    };
    let solution_string: String = (p.get::<String>("gcd_sol"));
    solution_string.replace("x", var_replace).replace("**", "^")
}
*/