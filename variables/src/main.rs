fn main() {
  let mut  x: i32= 5;
  println!("The value of x is : {}",x);
   let x:&str ="six";
  println!("The value of x is :{}",x);

  const SUBSCRIBER_COUNT:u32=100_000;

  let f:u8 = 255;

  // Compound types 

  let tup : (&str,i32)  = ("Let's get rusty !", 100_000);
  let (channel,subcount )= tup;
  println!("Channel is {}",channel);
  println!("The value of tup is : {}",tup.1);

  // Arrays are fixed size in rust
  let error_codes= [200,400,212];
  let  not_found = error_codes[1];
  let sum = my_function(20,30);
  println!("The sum of 20 and 30 is : {}",sum);

  // Control Flow 
  let number  = 5;
  if(number < 10){
    println!("first condition is true");
  }
  else if (number >40){
    println!("second condition is true");
  }
  else{
    println!("third condition is true");
  }

  let condition:bool = true;
  let num = if condition{5} else {20};
  println!("The value of num is : {}",num);

  let mut counter = 0 ;
  loop{
    counter+=1;

    println!("loop");
    if(counter ==10){

      break;

    }

  }
}

fn iterator(){
  let arr = [1,12,5,8,8,7];

  for element in arr.iter(){
    println!(" element is {}",element);
  }
}


fn my_function(x:i32,y:i32)->i32{
  println!("ANother function ,");
  let sum = x+y;
  x+ y 
}
