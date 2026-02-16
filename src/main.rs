#![allow(unused_imports,non_snake_case,non_camel_case_types,dead_code)]
mod Greeks;
use std::{collections::HashMap, f64::consts::E, io};
use libm::{erfc};

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

fn d1(cmp:f64,sp:f64,t_exp:f64,rfr:f64,vol:f64) -> f64{
let p = (cmp/sp);
let q = (rfr + (vol.powi(2)/2.0)) * t_exp;
(p.ln() + q)/ (vol * t_exp.sqrt())
}

fn d2(d1:f64,t_exp:f64,vol:f64) -> f64{
d1 - vol * t_exp.sqrt()
}

fn N(d:f64) -> f64{
0.5 * erfc(-d/2.0_f64.sqrt())
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

impl s_Greeks {
fn new(CMP:f64,SP:f64,t_exp:f64,rfr:f64,vol:f64,d1:f64,d2:f64,N_d1:f64,N_d2:f64) -> Self{

Self{
delta: Greeks::Delta(N_d1),
gamma: Greeks::Gamma(N_d1,CMP,vol,t_exp),
vega:  Greeks::Vega(N_d1, CMP, t_exp),
theta: Greeks::Theta(N_d1, d2, CMP, SP, vol, rfr, t_exp),
rho: Greeks::Rho(SP, t_exp, rfr, d2)
}
}


}


// Greeks returned in order [(Delta),(Gamma),(Vega),(Theta),(Rho)]
// Each tuple will have 2 elems the 1st is call the 2nd elem will be for put
fn greeks(CMP:f64,SP:f64,t_exp:f64,rfr:f64,vol:f64,d1:f64,d2:f64,N_d1:f64,N_d2:f64) -> HashMap<String,(f64,f64)> {
let mut greeks:HashMap<String,(f64,f64)> = HashMap::new();
greeks.insert("delta".to_string(),Greeks::Delta(N_d1));
let mut t = Greeks::Gamma(N_d1,CMP,vol,t_exp);
greeks.insert("gamma".to_string(),(t,t));
t = Greeks::Vega(N_d1, CMP, t_exp);
greeks.insert("vega".to_string(),(t,t));
greeks.insert("theta".to_string(),Greeks::Theta(N_d1, d2, CMP, SP, vol, rfr, t_exp));
greeks.insert("rho".to_string(),Greeks::Rho(SP, t_exp, rfr, d2));
return greeks;
}



// d1 = (ln(CMP/SP) + (rfr + vol ^ 2/2) * T_exp) / vol * T_exp ^ 0.5
// d2 = d1 - vol * T_exp ^ 0.5
// Call = CMP * N(d1) - SP * e^(-rfr * T_exp ) * N(d2)
// Put = SP * e^(-rfr * T_exp ) * N(-d2) - CMP * N(-d1)

fn main(){
println!("Input CMP,SP,T_exp,rfr and volatility\n");
let ret = get_args();
let ret = Black_Scholes(ret);
println!("Call => {}\nPut => {}\n",ret.0,ret.1);
}


fn Black_Scholes(ar:[f64;5]) -> (f64,f64) {
let [CMP,SP,T_exp,Rfr,Vol]  = ar;
let d1 = d1(CMP,SP,T_exp,Rfr,Vol);
let d2 = d2(d1,T_exp,Vol);
(((CMP * N(d1)) - (SP * E.powf(-Rfr * T_exp ) * N(d2))) , ((SP * E.powf(-Rfr * T_exp) * N(-d2)) - (CMP * N(-d1))))
}
