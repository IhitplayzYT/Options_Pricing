use std::{cell::RefCell, env, f64::consts::E, fs, io, process::exit, rc::Rc};
use libm::{erf, erfc};

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
[CMP,SP,T_Exp,rfr,vol]
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
