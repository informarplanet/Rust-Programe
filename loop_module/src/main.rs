fn main() {

    fn whilenum(){
        let mut i=5;
        while i<6{
            println!("The number is {}",i);
            i-=1;

            if i==0{
                println!("done");
                break;
            }
        }
        
        
    }
    
    let mut i=1;
    loop{
        println!("The number is {}",i);
        i+=1;
        if i==4{
            break;
        }
       
    }
    
    whilenum()
}


