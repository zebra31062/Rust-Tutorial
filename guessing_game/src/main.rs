use std::io;

fn main() 
{
    let mut array:[u8;6] = [0;6];

    for i in 0..6
    {
        println!("Guess the number!");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let n = input.trim().parse::<u8>().unwrap();
        let mut num:u8;
        
        loop
        {
            num  = rand::random::<u8>();
            num = num % 49;
            num = num + n + i;
            num = num % 49;
            num = num + 1;
            if !array.iter().any(|&x| x==num){break;}
        }
        
        array[i as usize] = num;
        println!("The correct number is {}", num);
    }

    

}
