fn main(){
    let name=String::from("Life");
    let len=get_str_len(str:name);
    println!("Length = > {}",len);   
    // let content_string = String::from("TutorialsPoint");
    // println!("length is {}",content_string.len());
}
fn get_str_len(str:String)->usize{
    // str.chars().count()
    return  str.len();
}


//     let mut hello = String::from("Hello, ");
// hello.push('w');
// hello.push_str("orld!");