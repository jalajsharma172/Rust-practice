
// fn main(){
//     let str =String::from("Hey Bhai");
//     let r1=&str;
//     let r2=&str;
//     println!("r1= {} and r2={}",r1,r2);
//     let w1=&str;
//     println!("{}",w1);
// }

fn main(){
    let mut str =String::from("Hey Bhai");
    let r1=&str;
    let r2=&str;
    let w1=&str;
    println!("r1= {} and r2={}",r1,r2,w1);
    // Got ERROR because we can't have mutable and immutable reference at the same time
}