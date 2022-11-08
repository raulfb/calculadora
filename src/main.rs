use std::env::{args, Args};
fn main() {
    let mut args: Args = args();
    
    let primero;
    match args.nth(1) {
        Some(v)=>{
            primero=v;
        },
        None=>{
            panic!("Error de sintaxis en num1. La sintaxis adecuada es: cargo run num1 operador num2");
        }
    }
    let operador:char;
    match args.nth(0) {
        Some(v)=>{
            operador=v.chars().next().unwrap();
        },
        None=>{
            panic!("Error de sintaxis en operador. La sintaxis adecuada es: cargo run num1 operador num2");
        }
    }

    let segundo;
    match args.nth(0) {
        Some(v)=>{
            segundo=v;
        },
        None=>{
            panic!("Error de sintaxis en num2. La sintaxis adecuada es: cargo run num1 operador num2");
        }
    }

    let primer_numero = primero.parse::<f32>().unwrap();
    let segundo_numero = segundo.parse::<f32>().unwrap();
    let resultado = calculadora(operador, primer_numero, segundo_numero);

    println!("{} {} {} = {}",primer_numero,operador,segundo_numero,resultado);
}

fn calculadora(operador:char,primer_numero:f32,segundo_numero:f32) -> f32{
    match operador {
        '+' => primer_numero + segundo_numero,
        '-' => primer_numero - segundo_numero,
        '/' => primer_numero / segundo_numero,
        '*'|'x'|'X' => primer_numero * segundo_numero,
        _ => panic!("El operador no es válido")
        
    }
}
