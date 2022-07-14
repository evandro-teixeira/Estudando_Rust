/**
 *
 */

fn main() 
{
/* 
    // primeiro exemplo: Variáveis e Mutabilidade

    let mut x = 5;
    println!("O valor de x é: {}", x);

    x = 6;
    println!("O valor de x é: {}", x);
*/
/*
    // segundo exemplo: Shadowing
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("O valor de x é: {}",x);

    let espaco = "    ";
    //let espaco = espaco.len();
    let n_espaco = espaco.len();

    println!("{}",espaco);
    println!("{}",n_espaco);
*/
/*
    // terceiro exemplo: Tipo de variaveis 
    let guess: u32 = "42".parse().expect("Não é numero!") ; // warn - help: if this is intentional, prefix it with an underscore: `_guess`
    println!("{}",guess);

    let x = 2.0;// f64
    let y: f32 = 3.50; // f32

    println!("{} | {}",x,y);
*/
/*
    // Operações numéricas
    let soma =  5 + 10;
    let diferenca  = 95.5 - 4.3;
    let produto = 4 * 30;
    let quociente = 56.7 / 32.2; 
    let resto = 43 % 5;

    println!("soma: {}",soma);
    println!("diferenca: {}",diferenca);
    println!("produto: {}",produto);
    println!("quociente: {}",quociente);
    println!("resto: {}",resto);
*/
/*
    // O tipo booleano 
    let t = true;
    let f: bool = false;
    
    // O tipo de caractere
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    println!("{} {} {}",c,z,heart_eyed_cat);
*/
/*
    // O tipo tuplaero de valores
    let tup: (i32,f32,u8) = (500,6.4,1);
    let tup_1 = (550,6.5,14); 

    let (x,y,z) = tup;

    println!("x: {} y: {} z: {}",x,y,z);

    println!("x: {} y: {} z: {}",tup.0,tup.1,tup.2);
*/

    // O tipo matriz
    let a = [1, 2, 3, 4, 5];
    let meses = ["Janeiro", "Fevereiro", "Março", "Abril", "Maio", "Junho", "Julho", "Agosto", "Setembro", "Outubro", "Novembro", "Dezembro"];

    let primeiro = a[0];
    let segundo = a[1];

    let index = 6;

    println!("{}",meses[index]);
}
