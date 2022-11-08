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

#[cfg(test)]

mod tests{ 
    use super::*; 
    #[test]
    fn suma(){
        let resultado=calculadora('+',5.22,5.1);
        assert_eq!(resultado,10.32)
    }

    #[test]
    fn resta(){
        let resultado=calculadora('-',5.00,2.00);
        assert_eq!(resultado,3.00)
    }

    #[test]
    fn multiplicacion(){
        let resultado=calculadora('*',2.00,2.00);
        assert_eq!(resultado,4.00)
    }

    #[test]
    fn division(){
        let resultado=calculadora('/',2.00,2.00);
        assert_eq!(resultado,1.00)
    }

    #[test]
    #[should_panic]
    fn parametros_erroneos(){
        let resultado=calculadora('/',2.00,0.00);
        assert_eq!(resultado,0.00)
    }

}