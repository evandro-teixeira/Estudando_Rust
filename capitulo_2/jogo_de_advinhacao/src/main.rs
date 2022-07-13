/**
 *
 */

extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
 
fn main() 
{
    println!("Advinhe o número!");

    // Obtem o numero aleatorio
    let numero_secreto = rand::thread_rng().gen_range(1, 101);

    loop 
    {
        println!("Digite o seu palpite.");

        let mut palpite = String::new();

        io::stdin().read_line(&mut palpite)
            .expect("Falha ao ler entrada");

        // Converte o valor presente na string para um numero interiro
        let palpite: u32 = match palpite.trim().parse() 
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você disse: {}", palpite);

        // compara o numero aleatorio com palpite
        match palpite.cmp(&numero_secreto) 
        {
            Ordering::Less => println!("Muito baixo!"),
            Ordering::Greater => println!("Muito alto!"),
            Ordering::Equal => 
            {
                println!("Você acertou!");
                break;
            }
        }
    }
}