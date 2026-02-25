//fn main() {
    
  //  let mut result : f32 = 0.0; //int
    //let x:i32 = 5 ; //float
    //result = result + x as f32; // no implicit conversion

    //println!("{}", result);


//}

//fn main()
//{

  //  let x:i32 = 5;
    // x = 1.012; // you can't


    //let x:f32 = x as f32 + 1.012;
    //println!("{}",x)



//}


//fn main() {
    // Shadowing
    //let x = 5;
    //let x = x + 1;  // Creates a new variable
    


    // Mutation
    //let mut y = 5;
    //y = y + 1;  // Modifies the existing variable
    //println!("x: {}, y: {}", x, y);
    
   // let x = 5;
    //{
    //    let x = x + 10;
    //    println!("x:{}",x);
    //}//free will be called for you 
 //   println!("x: {}",x);
//}


    // fn say_hi(x)
    // {
    //     println!("Hi John! My favorite num {}", x);
    // }
    // fn main()
    // {
    //     say_hi(5);
    // }

fn double(x: i32) -> i32
{
    return x * 2
}

fn main()
{

    println!("Double {} equals to {}", 5, double(5));
}