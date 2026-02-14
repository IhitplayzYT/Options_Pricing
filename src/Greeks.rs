#![allow(unused_imports,non_snake_case,non_camel_case_types,dead_code)]
use libm::{exp, sqrt};

pub fn Delta(N_d1:f64) -> (f64,f64){
(N_d1,(N_d1 - 1.0_f64))
}

/*
    CMP -> S
    rfr -> r
    t_exp -> T
    vol -> σ
    K -> SP
*/

pub fn Gamma(N_d1:f64,CMP:f64,vol:f64,t_exp:f64) -> (f64){
N_d1 / (CMP * vol * sqrt(t_exp))
}

pub fn Vega(N_d1:f64,CMP:f64,t_exp:f64) -> (f64){
(N_d1 * CMP * sqrt(t_exp)) / 100.0
}

pub fn Theta(N_d1:f64,N_d2:f64,CMP:f64,SP:f64,vol:f64,rfr:f64,t_exp:f64) -> (f64,f64){
let p1 = _theta_p1(N_d1,CMP,vol,t_exp);
((p1 - (rfr * SP * exp(-1.0 * rfr * t_exp) * N_d2)),(p1 +(rfr * SP * exp(-1.0 * rfr * t_exp) * ) ))
}
//pub fn Rho() -> (f64,f64){}

fn _theta_p1(N_d1:f64,CMP:f64,vol:f64,t_exp:f64) -> f64{
-1.0 * ((CMP * N_d1 * vol) / (2.0 * sqrt(t_exp))) 
}


// GREEK |  Call  |  Put 
// Delta | N(d1) | N(d1) - 1
// Gamma | N(d1)/(CMP * vol * sqrt(T))
// Vega |  CMP * N'(d1) * sqrt(T)
// Theta | - (CMP * N(d1) * vol)/(2 * sqrt(T)) (...)  | 
// Rho |  | 

