mod term;


fn main(){
    println!("TermLofi v1.0.0");
    let f = term::start_term();
    if f.is_err(){
        println!("{:?}", f.unwrap_err())
    }
}
