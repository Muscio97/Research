// #![feature(unboxed_closures)]
// #![feature(fn_traits)]


fn main(){    
    println!("\n\nThis a functional test script made by 0x4D3937\n");

    //Opdracht 3.2.1
        let mut values: Vec<f64> = Vec::new();
        for i in 0 .. 100 {
            values.push(i as f64);
        }
    
        //println!("=====================================");
        println!("Opdacht 3.2.1");     
        println!("\tStandaard: {}", Opdracht321::sum_of_squares(&values));

        println!("\trecursief: {}", Opdracht321::better_sum_of_squares_wrapper(&values));

        println!("\nIs het een succes? {}", Opdracht321::better_sum_of_squares_wrapper(&values) == Opdracht321::sum_of_squares(&values));
        println!("=====================================\n");
    
    //Opdracht 3.2.2
        //println!("=====================================");
        println!("Opdacht 3.2.2");
        
        println!("standaard: {}",Opdracht322::repeat("X".to_string(),3));
        println!("recursief {}",Opdracht322::better_repeat_wrapper("X".to_string(),3));

        println!("\nIs het een succes? {}", Opdracht322::repeat("X".to_string(),3) == Opdracht322::better_repeat_wrapper("X".to_string(),3));

        println!("=====================================\n");

    //Opdracht 3.2.3
        //println!("=====================================");
        println!("Opdacht 3.2.3");
        
        let mut input = Vec::new();
        for i in 0..10 {
            input.push(i);
        }
        println!("Orgineel:  {:?}", &input);
        println!("Standaard: {:?}", Opdracht323::revers(&input));
        println!("Recursief: {:?}", Opdracht323::better_revers_wrapper(&input));

        println!("\nIs het een succes? {}", Opdracht323::revers(&input) == Opdracht323::better_revers_wrapper(&input));

        println!("=====================================\n");
    //Opdreacht 3.2.4    

        //println!("=====================================");
        println!("Opdracht 3.2.4");
        println!("Standaard: {}", Opdracht324::fib_rec(15));
        println!("Standaard: {}", Opdracht324::better_fib_rec_wrapper(15));
        println!("\nIs het een succes? {}", Opdracht324::fib_rec(15) == Opdracht324::better_fib_rec_wrapper(15));
        println!("=====================================\n");
    //Opdreacht 3.3.1    

        //println!("=====================================");
        println!("Opdracht 3.3.1");
        
        let mut list = Vec::new();
        const CHANGE: isize = 10;
        const FINPUT: isize = 42;
        list.push((|i|i + CHANGE) as fn(isize) -> isize);
        list.push((|i|i - CHANGE) as fn(isize) -> isize);
        list.push((|i|i / CHANGE) as fn(isize) -> isize);
        list.push((|i|i * CHANGE) as fn(isize) -> isize);
        list.push((|i|i % CHANGE) as fn(isize) -> isize);
        list.push((|i|i ^ CHANGE) as fn(isize) -> isize);
        list.push((|i|i*i       ) as fn(isize) -> isize);
        println!("Orgineel: {}", CHANGE);
        println!("{:?}",Opdracht331::apply_everything_wrapper(&list, FINPUT)); 
        println!("=====================================\n");

    //Opdreacht 3.3.2   

        //println!("=====================================");
        println!("Opdracht 3.3.2");

        const CHANGE2: isize = 10;
        const FINPUTE: isize = 47;
        let mut list_even = Vec::new();
        
        list_even.push((|i|i + CHANGE2) as fn(isize) -> isize);
        list_even.push((|i|i - CHANGE2) as fn(isize) -> isize);
        list_even.push((|i|i / CHANGE2) as fn(isize) -> isize);
        list_even.push((|i|i * CHANGE2) as fn(isize) -> isize);
        list_even.push((|i|i % CHANGE2) as fn(isize) -> isize);
        list_even.push((|i|i ^ CHANGE2) as fn(isize) -> isize);
        list_even.push((|i|i*i        ) as fn(isize) -> isize);

        println!("Orgineel: {:?}", CHANGE2);
        println!("{:?}",Opdracht331::apply_everything_wrapper(&list_even, FINPUTE)); 
        println!("{:?}",Opdracht331::apply_everything_wrapper(&Opdracht332::filter_even_on_input_wrapper(list_even, FINPUTE), FINPUTE)); 
        println!("=====================================\n");

    //     let x = callable;
    // x();
}

struct Opdracht321{}
struct Opdracht322{}
struct Opdracht323{}
struct Opdracht324{}
struct Opdracht331{}
struct Opdracht332{}
struct Parser{}






impl Opdracht321{
    fn sum_of_squares(values: &Vec<f64>) -> f64 {
        let mut result: f64 = 0.0;
        for i in values {
            result = result + i.powf(2.0);
        }
        result
    }

    fn better_sum_of_squares_wrapper(values: &Vec<f64>) -> f64 {
        return Opdracht321::better_sum_of_squares(values, 0, 0.0)
    }
    
    fn better_sum_of_squares(values: &Vec<f64>, count: usize, mut result: f64) -> f64 {
        result = result + values[count].powf(2.0);
        if count == values.len()-1{
            return result
        }
        return Opdracht321::better_sum_of_squares(&values, count+1, result);
    }
}

impl Opdracht322{
    fn repeat(item: String, times: usize) -> String{
        let mut result = String::new();
        for _i in 0 .. times{
            result = result + &item;
        }
        return result
    }

    fn better_repeat_wrapper (item: String, times: usize) -> String{
        return Opdracht322::better_repeat(item, times, String::new())
    }

    fn better_repeat (item: String, times: usize, mut result: String) -> String{
        if times == 0 {
            return result
        }
        result = item.clone() + &Opdracht322::better_repeat(item, times-1, result);
        return result
    }
}

impl Opdracht323{
    fn revers(item: &Vec<i32>) ->  Vec<i32>{
        let mut result = Vec::new();
        let mut state = true;
        let mut i = item.len();
        while state {
            i = i -1;
            result.push(item[i]);
            if i <= 0 {
                state = false;
            }
        } 
        return result
    }

    fn better_revers_wrapper(item: &Vec<i32>) ->  Vec<i32>{
        return Opdracht323::better_revers(item, Vec::new(), 0)
    }

    fn better_revers(item: &Vec<i32>, mut result: Vec<i32>, mut count: usize) ->  Vec<i32>{
        count = count + 1;
        if item.len() == result.len() {
            return result
        }
        result.push(item[item.len()-count]);
        return Opdracht323::better_revers(item, result, count)

    }
}

impl Opdracht324 {
    fn fib_rec(n: usize) -> usize {
        if n == 0 {
            return 0 
        }
        else if n == 1 {
            return 1
        }
        else {
            return Opdracht324::fib_rec(n-1) + Opdracht324::fib_rec(n-2)
        }
    }

    fn better_fib_rec_wrapper(n: usize) -> usize {
        return Opdracht324::better_fib_rec(n, 0, 1)
    }
    fn better_fib_rec(n: usize, a: usize, b: usize) -> usize {
        if n == 0 {
            return a 
        }
        else if n == 1 {
            return b
        }
        else {
            return Opdracht324::better_fib_rec(n - 1, b , a + b);

            

        }
    }
}

impl Opdracht331{

    fn apply_everything_wrapper<F: Fn(isize) -> isize>(list : &Vec<F>, integer: isize) -> Vec<isize>{
        return Opdracht331::apply_everything(list, integer, 0, Vec::new())
    }

    fn apply_everything<F: Fn(isize) -> isize>( list : &Vec<F>, integer: isize,  count: usize, mut result: Vec<isize> ) -> Vec<isize>{
        if list.len() == count {
            return result
        }
        result.push(list[count](integer));
        return Opdracht331::apply_everything(list, integer, count+1, result)
    }
}

impl Opdracht332{

    fn filter_even_on_input_wrapper<F: Fn(isize) -> isize>( list : Vec<F>, integer: isize) -> Vec<F>{
        return Opdracht332::filter_even_on_input(list, integer, 0, Vec::new())
    }

    fn filter_even_on_input<F: Fn(isize) -> isize>( mut list : Vec<F>, integer: isize, mut  count: usize, mut result: Vec<F> ) -> Vec<F>{
        if list.len() == count {
            return result
        }
        
        if list[count](integer) % 2 ==  0 {
            result.push(list.remove(count));
        }else{
            count = count + 1;
        }

        return Opdracht332::filter_even_on_input(list, integer, count, result)        
    }
}

// struct callable;

// impl Fn<()> for callable {
//     extern "rust-call" fn call(&self, _args: ()) {
//         println!("Call (Fn) for callable");
//     }
// }

// impl FnMut<()> for callable {
//     extern "rust-call" fn call_mut(&mut self, _args: ()) {
//         println!("Call (FnMut) for callable");
//     }
// }

// impl FnOnce<()> for callable {
//     type Output = ();

//     extern "rust-call" fn call_once(self, _args: ()) {
//         println!("Call (FnOnce) for callable");
//     }
// }

impl Parser{

    /*
        Wegens de complexiteit van de parser is het niet gelukt om deze werkent te krijgen binnen Rust,
        dit komt door mijn gebrek aan functionele kennis en de gebruikte libs in python.
    */

    // fn callable(f: Vec<&str>) -> i32{
    //     0
    // } 
    //fn callable(f: Vec<&str>) -> Vec<(str,str)>{
    //     vec
    // } 
    //fn callable(f: Vec<&str>) -> Vec<&str>{
    //     f
    // } 
    //fn callable<A>(f: Vec<&str>) -> Vec<(str, A)>{
    //     0
    // }
    // fn parse_character(match_: str) -> Vec<(str, str)>{
    //     fn parse(input: Vec<&str>, match_:str) -> Vec<(str, str)>{
    //         if input.len() == 0 {
    //             return []
    //         }
    //         else{
    //             let head: Option<&str> = input.first();
    //             let tail: &[str] = &input[1..];
    //             if head == match_{
    //                 return Vec::new((head,tail))
    //             }else{
    //                 return []
    //             }
    //         }
    //         return parse
    //     }
    // }

    // fn join<T>(f: T) -> T {
    //     f
    // }
    // fn pure<A>(rv: A) -> Vec<(A, str)>{
    //     fn parse<A>(input: Vec<&str>, rv:A) -> Vec<(A, str)>{
    //         return Vec::new((rv, input))
    //     }
    // }
    // fn then<A,B,C>(f : callable((A,B),C), pa : callable(Vec<&str>, Vec<(A, str)>), pb : callable(Vec<&str>, Vec<(B, str)>)) -> callable(Vec<&str>, Vec<(C, str)>){
    //     fn parse(){

    //     }
    // }
    // fn either<A>(pa : callable(Vec<&str>, Vec<(A, str)>), pb : callable(Vec<&str>, Vec<(A, str)>)) -> callable(Vec<&str>, Vec<(A, str)>){
    //     fn parse(){

    //     }
    // }
    // fn all<A>(f : callable((A,A),A), ps : callable(Vec<&str>, Vec<(A, str)>)) -> callable(Vec<&str>, Vec<(A, str)>){

    // }
    // fn any<A>(ps : callable(Vec<&str>, Vec<(A, str)>)) -> Vec<callable(Vec<&str>, Vec<(A, str)>)>){

    // }
    // fn repeat<A>(p : callable(Vec<&str>, Vec<(A, str)>)) -> Vec<callable(Vec<&str>, Vec<(A, str)>)>){

    //     fn parse(){

    //     }
    // }
}