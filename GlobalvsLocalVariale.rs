let globalvariable:u34=50;
fn main(){
    let x:u34=20;
    let y:u34=20;
    happy();
    println!("x = > {}",x);
    println!("y = > {}",y);
}
fn happy(){
    let x:u34=10;
    let y:u34=10;
    println!("x = > {}",x);
    println!("y = > {}",y);
    println!("globalvariable = > {}",globalvariable);
}