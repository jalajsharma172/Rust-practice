fn main(){
    let mut str:  String=String::from("Chal chal ke ");//str is the owner of HEAP1
    append_str(  & mut str);//str refering s 
    print!("{}",str);
}
fn append_str(s:  & mut String){
    s.push_str(" Dikha ");
}       