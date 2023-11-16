use std::io;

fn main() {
    println!("Programa Fizz,Buzz!");
    println!("Digite un número indice:");
    let mut puntero = String::new();
    io::stdin().read_line(&mut puntero).expect("Error al leer la entrada");
    let puntero: i32 = puntero.trim().parse().expect("Por favor ingrese un número válido");

    println!("Digite un número finalizador:");
    let mut finalizador = String::new();
    io::stdin().read_line(&mut finalizador).expect("Error al leer la entrada");
    let finalizador: i32 = finalizador.trim().parse().expect("Por favor ingrese un número válido");

    println!("Los números entre {} y {} son:", puntero, finalizador);
    for i in puntero..=finalizador {
        if i % 3 == 0 && i % 5 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", i);
        }
    }
    io::stdin().read_line(&mut String::new()).unwrap();
}


