fn main(){
    let str:String=String::from("Hey man");//str is the owner of HEAP1

    let (str2,length)=calculate_length2(str); //Transfering Ownership, Then Returning Ownerwhip back to str2

    println!("The length of '{}' in main is {}",str2,length);
    // println!("The length of '{}' in main is {}",str,length);//Now, dosn't exist 

        
    // let length=calculate_length(str); //Ownership transfer [ This DATA_TYPE is the owner of HEAP1  ]
    // println!("The length of '{}' is {}",str,length);

}
   

fn calculate_length2(str:String)->(String,usize){
    let length = str.len();
    println!("The length of in calculate_length2'{}' is {}",str,length);
    (str,length)
    // (str,str.len())//why i got error on this line on retering Directly
}       


fn calculate_length(str:String)->usize{
    let length = str.len();
    println!("The length of in calculate_length'{}' is {}",str,length);
    str.len()
}    