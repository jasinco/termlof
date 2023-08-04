mod term;
mod config;

fn main(){

    println!("TermLofi v1.0.0");
    let f = term::start_term();
    if f.is_err(){
        panic!("{}", f.unwrap_err())
    }
}
