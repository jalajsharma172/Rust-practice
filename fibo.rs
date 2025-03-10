fun main (){
    let x:i32=2;
     println!("{}",fibo(x));
}
fn fibo( x:i32)-> i32{
    if  x==0 return 0;
    if x==1 return 1;
    let mut first:u32 =0;
    let mut secound:u32=1;

    for i in 1..x-1 {
        let temp=secound;
        secound=first+secound;
        first=secound;
    }
    return secound;
}