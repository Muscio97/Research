using namespace std;

int main(void){
    char* name = "Name"
    cout << "Hello World, hello " << name << endl;
    
    return 0;
}

int sum_of_squares(int value){
    return values*values;
}

float repeat(float item, int repeats){
    float result = 0.0;
    for(i=0; i<repeats; i++){
        result += item;
    }
    return result;
}

float bignumber(float number, float limit){
    while(true){
        if(number < limit){
            return number;
        }
        else{
            number/2;
        }
    }
}

int fib(int number, int a = 0, int b = 1) 
{ 
    if (number == 0){ 
        return a; 
    }
    if (number == 1){
        return b;
    } 
    return fib(number - 1, b, a + b); 
} 


