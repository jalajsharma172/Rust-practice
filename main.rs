//Use Scope to avoid mutable and immutable reference at the same time
fn main(){
    let mut str =String::from("Hey Bhai");
    {
        let w1=& mut str;//w1 will not be in scope
        println!("w1= {} ",w1);//w1 will not be in scope
    }
    let r1=&str;
    let r2=&str;
    println!("r1= {} and r2={}",r1,r2);
}