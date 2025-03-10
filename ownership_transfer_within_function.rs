//With Dynamic Memory
fn main(){
    let  str = String::from(" men are from Mars");
    printstr(str);
    // println!("{}",s); // This will give error because s is moved to print function
}
fn printstr(str_one:String){
    println!("Hey hi  {}",str_one); 
}



// With STACK 
// fn main(){
//     let x=10;
//        let y=x;
//        print(y);
//        println!("x=>{}",x);
//    }
   
//    fn print(str:u32){
//        println!("Hello {}",str); 
//    }