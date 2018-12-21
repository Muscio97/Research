

fn main(){
    let name = "Name";
    println!("Hello World, hello {}", name)
}


fn sum_of_squares(values: i32) -> i32 {
    values*values
}

fn repeat(item: f32, repeats: i32) -> f32{
    let mut result: f32 = 0.0;
    for _i in 0 .. repeats{
        result = result + item;
    }
    result
}

fn bignumer(number: f32, limit: f32) -> f32{
    loop{
        if number < limit{
            return number
        }
        else{
            number/2;
        }
    }
}

fn fib_wrapper(number: usize) -> usize {
    return fib(number, 0, 1)
}
fn fib(number: usize, a: usize, b: usize) -> usize {
    if number == 0 {
        return a 
    }
    if number == 1 {
        return b
    }
    return fib(number - 1, b , a + b);
}