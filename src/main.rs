use std::process;
use std::ffi::OsString;
use std::error::Error;
use ravencol::RawFrame;
use nalgebra::{DMatrix,Dynamic};

fn main() {
    
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }

}

fn run() -> Result<(), Box<dyn Error>> {

    let path = OsString::from("./datos/BostonHousing.csv");
    let datos = RawFrame::from_os_string(path)?;

    let columnas: Vec<&str> = datos.columns.iter().collect();
    let zeros: Vec<f64> = datos.columns.iter().map(|_| 0.0).collect();

    let fils = Dynamic::new(datos.records.len());
    let cols = Dynamic::new(columnas.len());

    let coldatos = datos.column_major_vector(columnas,zeros)?;

    let matriz = DMatrix::from_vec_generic(fils,cols,coldatos);

    let mat_a = matriz.columns(0, 13).into_owned();
    let mat_y = matriz.column(13).into_owned();

    //Regresión con la inversa de la matriz
    let a = mat_a.clone().insert_column(13, 1.0).into_owned();
    let b = mat_y.clone();
    let x = (a.transpose() * &a).try_inverse().unwrap() * &a.transpose() *&b;
    let coeff = x.rows(0, 13);
    let intercept = x[(13, 0)];
    println!("coeff: {}, intercept: {}", coeff, intercept);

    //Regresión con descomposición QR
    let a = mat_a.clone().insert_column(13, 1.0).into_owned();
    let b = mat_y.clone();
    let qr = a.qr();
    let (q, r) = (qr.q().transpose(), qr.r());  
    let x = r.try_inverse().unwrap() * &q * &b;
    let coeff = x.rows(0, 13);
    let intercept = x[(13, 0)];
    println!("coeff: {}, intercept: {}", coeff, intercept);

    //Regresión con descomposicion SVD
    let a = mat_a.clone().insert_column(13, 1.0).into_owned();
    let b = mat_y.clone();
    let svd = a.svd(true, true);
    let ps_inv = svd.pseudo_inverse(0.001).unwrap();
    let x = ps_inv * &b;
    let coeff = x.rows(0, 13);
    let intercept = x[(13, 0)];
    println!("coeff: {}, intercept: {}", coeff, intercept);

    //Regresión con descomposicion SVD resolviendo con nalgebra
    let a = mat_a.clone().insert_column(13, 1.0).into_owned();
    let b = mat_y.clone();
    let svd = a.svd(true, true);
    let x = svd.solve(&b,0.001).unwrap();
    let coeff = x.rows(0, 13);
    let intercept = x[(13, 0)];
    println!("coeff: {}, intercept: {}", coeff, intercept);

    Ok(())
}