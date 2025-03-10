fn main(){// This one is Dynamic Size Data that's why HEAP is used and there are used pointer's
    //DAta ek aalag memory me store hota hai issi liye x aur y alag alag memory me store hote hai
    let str :String =String::from ("Life is unexpected in IT sector ");
    //Stack[ptr[----------------->HEAP],//len[5],capacity[5]]
    //Heap[Index,Value]=[0=>L,1=>i,2=>f,3=>e,4=> ,5=>i,6=>s,7=> ,8=>u,9=>n,10=>e,11=>x,12=>p,13=>e,14=>c,15=>t,16=>e,17=>d,18=> ,19=>i,20=>n,21=> ,22=>I,23=>T,24=> ,25=>s,26=>e,27=>c,28=>t,29=>o,30=>r,31=> ]



    // let _copy_str:String = str;//str is the owner
    //Stack1[ptr[----------------->HEAP],//len[5],capacity[5]]
    //Stack2[ptr[----------------->HEAP],//len[5],capacity[5]]
    //Heap[Index,Value]=[0=>L,1=>i,2=>f,3=>e,4=> ,5=>i,6=>s,7=> ,8=>u,9=>n,10=>e,11=>x,12=>p,13=>e,14=>c,15=>t,16=>e,17=>d,18=> ,19=>i,20=>n,21=> ,22=>I,23=>T,24=> ,25=>s,26=>e,27=>c,28=>t,29=>o,30=>r,31=> ]
         


    let _copy_str:String = str.clone();//This will Create Different Stack and Heap Memory

     
    println!("String=> {} ",str);
    println!("String=> {} ",_copy_str);

     
}
// fn main (){//Fix size data tha iisi Liye Stack HI USE hota hai
//     let x=10;
//     let y=x;
//     println!("x=>{}",x);
//     println!("y=>{}",y);
// }

// Question : Why is this so
// While Compile Clear the memory when  the scope of the variable ends
// When the scope of the variable ends, Rust automatically calls the drop function and clears the memory.
// In the case of fixed-size data, the memory is cleared from the stack.    
// In the case of dynamic-size data, the memory is cleared from the heap.
// BUT BUT when 2 -STACK1,STACK2 are pointing to the same heap memory then the memory is not cleared from the heap.
// We get ERROR , because both are pointing to the same memory location of its dynamic data called as HEAP.
