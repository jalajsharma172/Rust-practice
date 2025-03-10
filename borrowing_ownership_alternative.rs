fn main(){
    let str:String=String::from("Hey man");//str is the owner of HEAP1
    let len= calculate_length(&str);//Reference that refers to the value of str but does not own it.
    print!("The length of '{}' in main is {}",str,len);
}
fn calculate_length(s:&String)->usize{//because 's' does not own 'str', when 's' goes out of scope nothing happens
    let length = s.len();
    println!("The length of in calculate_length'{}' is {}",s,length);
    s.len()
}       