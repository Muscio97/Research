fn main() {
    println!("This a functional test script made by 0x4D3937\n");

    //Opdracht 3.2.1
        let mut values: Vec<f64> = Vec::new();
        for i in 0 .. 100 {
            values.push(i as f64);
        }
    
        println!("=====================================");
        println!("Opdacht 3.2.1");     
        println!("\tStandaard: {}", Opdracht321::sum_of_squares(&values));

        println!("\trecursief: {}", Opdracht321::better_sum_of_squares(&values, 0, 0.0));

        println!("\nIs het een succes? {}", Opdracht321::better_sum_of_squares(&values, 0, 0.0) == Opdracht321::sum_of_squares(&values));
        println!("=====================================\n");
    
    //Opdracht 3.2.2
        println!("=====================================");
        println!("Opdacht 3.2.2");
        
        println!("standaard: {}",Opdracht322::repeat("X".to_string(),3));
        println!("recursief {}",Opdracht322::better_repeat("X".to_string(),3, String::new()));

        println!("\nIs het een succes? {}", Opdracht322::repeat("X".to_string(),3) == Opdracht322::better_repeat("X".to_string(),3, String::new()));

        println!("=====================================\n");

    //Opdracht 3.2.3
        println!("=====================================");
        println!("Opdacht 3.2.3");
        
        let mut input = Vec::new();
        for i in 0..10 {
            input.push(i);
        }
        
        println!("Standaard: {:?}", Opdracht323::revers(&input));
        println!("Recursief: {:?}", Opdracht323::better_revers(&input, Vec::new(), 0));

        println!("\nIs het een succes? {}", Opdracht323::revers(&input) == Opdracht323::better_revers(&input, Vec::new(), 0));

        println!("=====================================\n");

        println!("Standaard: {}", Opdracht324::fib_rec(15));
        println!("Standaard: {}", Opdracht324::better_fib_rec(15, 0, 1));

}


struct Opdracht321{}
struct Opdracht322{}
struct Opdracht323{}
struct Opdracht324{}

impl Opdracht321{
    fn sum_of_squares(values: &Vec<f64>) -> f64 {
        let mut result: f64 = 0.0;
        for i in values {
            result = result + i.powf(2.0);
        }
        result
    }

    fn better_sum_of_squares(values: &Vec<f64>, count: usize, mut result: f64) -> f64 {
        result = result + values[count].powf(2.0);
        if count == values.len()-1{
            return result
        }
        result = Opdracht321::better_sum_of_squares(&values, count+1, result);
        return result
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
