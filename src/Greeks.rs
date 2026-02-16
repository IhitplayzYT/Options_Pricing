#![allow(unused_imports,non_snake_case,non_camel_case_types,dead_code)]
use core::f64;

use libm::{exp, sqrt,erfc,erf};

/*
    CMP -> S
    rfr -> r
    t_exp -> T
    vol -> σ
    K -> SP
*/

/* 
    d1 = (ln(CMP/SP) + (rfr + vol ^ 2/2) * T_exp) / vol * T_exp ^ 0.5
    d2 = d1 - vol * T_exp ^ 0.5
    Call = CMP * N(d1) - SP * e^(-rfr * T_exp ) * N(d2)
    Put = SP * e^(-rfr * T_exp ) * N(-d2) - CMP * N(-d1)
*/


pub fn Delta(N_d1:f64) -> (f64,f64){
(N_d1,(N_d1 - 1.0_f64))
}

pub fn phi(x:f64) -> f64{
    (-x.powi(2)/2.0).exp() / (2.0 * f64::consts::PI).sqrt()
}


pub fn Gamma(phi_d1:f64,CMP:f64,vol:f64,t_exp:f64) -> f64{
phi_d1 / (CMP * vol * sqrt(t_exp))
}

pub fn Vega(phi_d1:f64,CMP:f64,t_exp:f64) -> f64{
(phi_d1 * CMP * sqrt(t_exp)) / 100.0_f64
}

pub fn Theta(phi_d1:f64,N_d2:f64,CMP:f64,SP:f64,vol:f64,rfr:f64,t_exp:f64) -> (f64,f64) {
let p1 = _theta_p1(phi_d1,CMP,vol,t_exp);
(p1 - (rfr * SP * exp(-1.0 * rfr * t_exp) * N_d2),p1 + (rfr * SP * exp(-1.0 * rfr * t_exp) * (1.0 - N_d2)))
}

fn _theta_p1(phi_d1:f64,CMP:f64,vol:f64,t_exp:f64) -> f64{
-1.0 * ((CMP * phi_d1 * vol) / (2.0 * t_exp.sqrt())) 
}

pub fn Rho(SP:f64,T:f64,rfr:f64,N_d2:f64) -> (f64,f64){
(
SP * T * exp(-1.0 * rfr * T) * N_d2,
-1.0 * SP * T * exp(-1.0 * rfr * T) * (1.0 - N_d2)
)
}


// GREEK |  Call  |  Put 
// Delta | N(d1) | N(d1) - 1
// Gamma | N(d1)/(CMP * vol * sqrt(T))
// Vega |  CMP * N'(d1) * sqrt(T)
// Theta | - (CMP * N(d1) * vol)/(2 * sqrt(T)) - (rfr * SP * e^(-r * T) * N(d2))  |  - (CMP * N(d1) * vol)/(2 * sqrt(T)) + (rfr * SP * e^(-r * T) * N(-d2))  
// Rho |  SP * T * e^(-r * T) * N(d2) |  - SP * T * e^(-r * T) * N(-d2)

