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

pub fn Gamma(N_d1:f64,CMP:f64,vol:f64,t_exp:f64) -> f64{
N_d1 / (CMP * vol * sqrt(t_exp))
}

pub fn Vega(N_d1:f64,CMP:f64,t_exp:f64) -> f64{
(N_d1 * CMP * sqrt(t_exp)) / 100.0
}

pub fn Theta(N_d1:f64,d2:f64,CMP:f64,SP:f64,vol:f64,rfr:f64,t_exp:f64) -> (f64,f64) {
let p1 = _theta_p1(N_d1,CMP,vol,t_exp);
(p1 - (rfr * SP * exp(-1.0 * rfr * t_exp) * libm::erf(d2)),p1 + (rfr * SP * exp(-1.0 * rfr * t_exp) * libm::erfc(d2)))
}

fn _theta_p1(N_d1:f64,CMP:f64,vol:f64,t_exp:f64) -> f64{
-1.0 * ((CMP * N_d1 * vol) / (2.0 * sqrt(t_exp))) 
}

pub fn Rho(SP:f64,T:f64,rfr:f64,d2:f64) -> (f64,f64){
(
SP * T * exp(-1.0 * rfr * T) * libm::erf(d2),
-1.0 * SP * T * exp(-1.0 * rfr * T) * libm::erfc(-d2)
)
}


// GREEK |  Call  |  Put 
// Delta | N(d1) | N(d1) - 1
// Gamma | N(d1)/(CMP * vol * sqrt(T))
// Vega |  CMP * N'(d1) * sqrt(T)
// Theta | - (CMP * N(d1) * vol)/(2 * sqrt(T)) - (rfr * SP * e^(-r * T) * N(d2))  |  - (CMP * N(d1) * vol)/(2 * sqrt(T)) + (rfr * SP * e^(-r * T) * N(-d2))  
// Rho |  SP * T * e^(-r * T) * N(d2) |  - SP * T * e^(-r * T) * N(-d2)

