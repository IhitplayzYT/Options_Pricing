#![allow(unused_imports,non_snake_case,non_camel_case_types,dead_code)]
mod Greeks;
use std::{collections::HashMap, f64::consts::E, io};
use libm::{erfc};
use std::fmt::Display;
use std::fmt;

use crate::Greeks::phi;

fn get_args() -> [f64;5]{
let mut buff = String::from("");
let msg = "Invalid!";
io::stdin().read_line(& mut buff).expect(msg);
let CMP = buff.trim().parse().expect("Input Current Market Price(CMP)\n");
buff.clear();
io::stdin().read_line(& mut buff).expect(msg);
let SP = buff.trim().parse().expect("Input Strike Price(SP)\n");
buff.clear();
io::stdin().read_line(& mut buff).expect(msg);
let T_Exp = buff.trim().parse().expect("Input Expiration date\n");
buff.clear();
io::stdin().read_line(& mut buff).expect(msg);
let rfr = buff.trim().parse().expect("Input Risk-Free-Return rate\n");
buff.clear();
io::stdin().read_line(& mut buff).expect(msg);
let vol = buff.trim().parse().expect("Input Volatility\n");
buff.clear();
return [CMP,SP,T_Exp,rfr,vol];
}



/*
    CMP -> S
    rfr -> r
    t_exp -> T
    vol -> σ
    K -> SP
*/

struct s_Greeks{
delta: (f64,f64),
gamma: f64,
vega: f64,
theta: (f64,f64),
rho: (f64,f64),
}

struct API{
call_price:f64,
put_price:f64,
Greeks:s_Greeks
}

pub fn calc_call_put(CMP:f64,SP:f64,T_exp:f64,Rfr:f64,d1:f64,d2:f64) -> (f64,f64){
(((CMP * N(d1)) - (SP * E.powf(-Rfr * T_exp ) * N(d2))) , ((SP * E.powf(-Rfr * T_exp) * N(-d2)) - (CMP * N(-d1))))
}

pub fn Black_Scholes(ar:[f64;5]) -> (f64,f64) {
let [CMP,SP,T_exp,Rfr,Vol]  = ar;
let d1 = D1(CMP,SP,T_exp,Rfr,Vol);
let d2 = D2(d1,T_exp,Vol);
(((CMP * N(d1)) - (SP * E.powf(-Rfr * T_exp ) * N(d2))) , ((SP * E.powf(-Rfr * T_exp) * N(-d2)) - (CMP * N(-d1))))
}

impl API {
pub fn new(CMP:f64,SP:f64,T_exp:f64,Rfr:f64,Vol:f64) -> Self{
let d1 = D1(CMP, SP, T_exp, Rfr, Vol);
let d2 = D2(d1, T_exp, Vol);
let (call_price,put_price) = calc_call_put(CMP, SP, T_exp, Rfr, d1, d2);
let (N_d1,N_d2,phi_d1) = (N(d1),N(d2),phi(d1));

Self{
    call_price,
    put_price,
    Greeks: s_Greeks::new(CMP,SP,T_exp,Rfr,Vol,N_d1,N_d2,phi_d1),
}
}
}

impl Display for API{
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"----Black Scholes----\n\x1b[92mCALL\x1b[0m: {:.4}\n\x1b[91mPUT\x1b[0m : {:.4}\nGreeks:\t   \x1b[92mCall\x1b[0m       \x1b[91mPut\x1b[0m\n   Delta: \x1b[92m{:.4}\x1b[0m    \x1b[91m{:.4}\x1b[0m\n   Gamma:       \x1b[93m{:.4}\x1b[0m   \n   Vega :       \x1b[93m{:.4}\x1b[0m   \n   Theta: \x1b[92m{:.4}\x1b[0m    \x1b[91m{:.4}\x1b[0m\n   Rho  : \x1b[92m{:.4}\x1b[0m    \x1b[91m{:.4}\x1b[0m\n---------------------\n",
        self.call_price,self.put_price,self.Greeks.delta.0,self.Greeks.delta.1,self.Greeks.gamma,self.Greeks.vega,self.Greeks.theta.0,self.Greeks.theta.1,self.Greeks.rho.0,self.Greeks.rho.1)
    }   
}


impl s_Greeks {
pub fn new(CMP:f64,SP:f64,t_exp:f64,rfr:f64,vol:f64,N_d1:f64,N_d2:f64,phi_d1:f64) -> Self{
Self{
delta: Greeks::Delta(N_d1),
gamma: Greeks::Gamma(phi_d1,CMP,vol,t_exp),
vega:  Greeks::Vega(phi_d1, CMP, t_exp),
theta: Greeks::Theta(phi_d1, N_d2, CMP, SP, vol, rfr, t_exp),
rho: Greeks::Rho(SP, t_exp, rfr, N_d2)
}
}


}


// Greeks returned as a map
// Each tuple will have 2 elems the 1st is call the 2nd elem will be for put
fn greeks(CMP:f64,SP:f64,t_exp:f64,rfr:f64,vol:f64,d2:f64,N_d1:f64,phi_d1:f64) -> HashMap<String,(f64,f64)> {
let mut greeks:HashMap<String,(f64,f64)> = HashMap::new();
greeks.insert("delta".to_string(),Greeks::Delta(N_d1));
let mut t = Greeks::Gamma(phi_d1,CMP,vol,t_exp);
greeks.insert("gamma".to_string(),(t,t));
t = Greeks::Vega(phi_d1, CMP, t_exp);
greeks.insert("vega".to_string(),(t,t));
greeks.insert("theta".to_string(),Greeks::Theta(N_d1, d2, CMP, SP, vol, rfr, t_exp));
greeks.insert("rho".to_string(),Greeks::Rho(SP, t_exp, rfr, d2));
return greeks;
}

fn D1(cmp:f64,sp:f64,t_exp:f64,rfr:f64,vol:f64) -> f64{
let p = cmp/sp;
let q = (rfr + (vol.powi(2)/2.0)) * t_exp;
(p.ln() + q)/ (vol * t_exp.sqrt())
}

fn D2(d1:f64,t_exp:f64,vol:f64) -> f64{
d1 - vol * t_exp.sqrt()
}

fn N(d:f64) -> f64{
0.5 * erfc(-d/2.0_f64.sqrt())
}

fn main(){
let args:Vec<String> = std::env::args().collect();
let [CMP,SP,T_exp,Rfr,Vol];
println!("{}",args.len());
if args.len() != 6 {
println!("Input CMP,SP,T_exp,rfr and volatility\n");
[CMP,SP,T_exp,Rfr,Vol] = get_args();
}
else{
[CMP,SP,T_exp,Rfr,Vol] = [
args[1].parse().expect("Invalid float"),
args[2].parse().expect("Invalid float"),
args[3].parse().expect("Invalid float"),
args[4].parse().expect("Invalid float"),
args[5].parse().expect("Invalid float"),
];
}
let ret = API::new(CMP, SP, T_exp, Rfr, Vol);
println!("{ret}");
}



