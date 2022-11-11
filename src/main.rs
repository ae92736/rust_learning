fn main() 
{
    let x: f32 = 256.0;
    println!("x is {}", x);
    {
        let x: i32 = x as i32 - 2;
        println!("x is {}", x);
    }
    let x: i128 = 42;
    println!("x is {}", x);
}