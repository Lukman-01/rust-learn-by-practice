/* Annotate `r` and `x` as above, and explain why this code fails to compile, in the lifetime aspect. */

fn main() {  
    {
        let r;                
                              
        {                     
            let x = 5;        
            r = &x;    
            
            println!("r: {}", r);       
        }                           
    }                         
}