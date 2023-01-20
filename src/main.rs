fn main() 
{
    // Podrão em rust todas as variáveis são imutáveis
    // mut transforma uma várias imutáveis em um variável mutável
    let mut x = 45; // 45 é to tipo i32 por padrão integer 32 bit
    let y: i64 = 45; // i64 - integer 64bit
    let _inteiro_positivo: u64 = 54; // u64 - Unsigned 64 bit
    let _float: f32 = 6.7; // f32 floating 
    let _verdade: bool = false; //boolean


    println!("O valor de x é {}", x);
    println!("O valor de y é {}", y);

    x = 60;

    println!("O valor de x é {}", x);

}
