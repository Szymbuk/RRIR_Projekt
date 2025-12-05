mod visualisation;

use::std::io;
use::ndarray;
use::std::io::Write;
use crate::visualisation::visualize;

fn main() {
    let n = read_input();
    let lower_bound:f64 = 0.0;
    let upper_bound:f64 = 2.0;
    let h = (upper_bound-lower_bound)/n as f64;
    let mut nodes_vector: Vec<f64> = Vec::new();
    for i in 0..n+1{
        nodes_vector.push(i as f64 * h);
    }
    let mut _a_array = ndarray::Array2::<f64>::zeros((n+1, n+1));
    let mut _b_vector = ndarray::Array1::<f64>::zeros(n+1);
    //println!("{:?}", nodes_vector);
    for i in  0..n{
        calculate_partial_integral_matrix(nodes_vector[i],nodes_vector[i+1],i,&mut _a_array,&mut _b_vector);
    }

    //uwzględnienie warunku robina (Brzeg x = 2)
    _a_array[[n,n]] -= 1.0;
    _b_vector[n] += 5.0;

    //uwzględnienie warunku dirichleta (Brzeg x=0)
    for i in 0..n+1{
        _a_array[[0,i]] = 0.0;
    }
    _a_array[[0,0]] = 1.0;
    _b_vector[0] = 1.0;

    //println!("{}", _a_array);

    let _u_vector = solve_gaussian_elimination(_a_array,_b_vector);

    //println!("{}", _u_vector);
    let filename = "result";
    save_to_csv(&nodes_vector,&_u_vector,filename);
    let _ = visualize(filename);
}


fn read_input() -> usize {
    println!("Podaj liczbę n:");
    loop {
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Błąd podczas odczytu. Spróbuj ponownie.");
            continue;
        }

        // Próbujemy sparsować tylko liczby
        let numbers: Vec<usize> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Musi być minimum jedna liczba, uwzględniam tylko 1
        if numbers.len() < 1 {
            println!("Proszę podać 1 liczbę");
            continue;
        }
        else{
            return numbers[0];

        }
    }
}

fn calculate_partial_integral_matrix(k0: f64, k1: f64,iteration: usize,_a_array: &mut ndarray::Array2<f64>,_b_vector: &mut ndarray::Array1<f64>){
    let points:Vec<f64> = vec![-1.0/3f64.sqrt(), 1.0/3f64.sqrt()];
    let weights = vec![1.0,1.0];
    let jacobian = (k1-k0)/2.0;
    let middle_point = (k1+k0)/2.0;

    //liczymy dla punktu -xi oraz xi
    for i in 0..2 {

        let n0 = (1.0 - points[i]) / 2.0;
        let n1 = (1.0 + points[i]) / 2.0;

        let der_n0_ksi = -0.5;
        let der_n1_ksi = 0.5;

        let der_n0_x = der_n0_ksi / jacobian;
        let der_n1_x = der_n1_ksi / jacobian;

        let x_real = middle_point + points[i] * jacobian;

        _a_array[[iteration, iteration]] += (der_n0_x * der_n0_x - n0 * n0) * jacobian * weights[i];
        _a_array[[iteration + 1, iteration]] += (der_n0_x * der_n1_x - n0 * n1) * jacobian * weights[i];
        _a_array[[iteration, iteration + 1]] += (der_n1_x * der_n0_x - n1 * n0) * jacobian * weights[i];
        _a_array[[iteration + 1, iteration + 1]] += (der_n1_x * der_n1_x - n1 * n1) * jacobian * weights[i];

        _b_vector[iteration] += x_real.sin() * n0 * weights[i] * jacobian;
        _b_vector[iteration+1]+= x_real.sin() * n1 * weights[i] * jacobian;
    }
}

fn solve_gaussian_elimination(mut a: ndarray::Array2<f64>, mut b: ndarray::Array1<f64>) -> ndarray::Array1<f64> {
    let n = b.len();
    // 1. Eliminacja w przód
    for k in 0..n {
        // Szukanie pivota (dla stabilności)
        let mut max_row = k;
        for i in k + 1..n {
            if a[[i, k]].abs() > a[[max_row, k]].abs() {
                max_row = i;
            }
        }

        // Zamiana wierszy
        if max_row != k {
            for j in k..n {
                let temp = a[[k, j]];
                a[[k, j]] = a[[max_row, j]];
                a[[max_row, j]] = temp;
            }
            let temp_b = b[k];
            b[k] = b[max_row];
            b[max_row] = temp_b;
        }

        // Zerowanie pod przekątną
        for i in k + 1..n {
            if a[[k, k]].abs() < 1e-12 {
                continue; // Unikamy dzielenia przez zero przy osobliwych macierzach
            }
            let factor = a[[i, k]] / a[[k, k]];
            b[i] -= factor * b[k];
            for j in k..n {
                a[[i, j]] -= factor * a[[k, j]];
            }
        }
    }

    // 2. Podstawianie wstecz
    let mut x = ndarray::Array1::<f64>::zeros(n);
    for i in (0..n).rev() {
        let mut sum = 0.0;
        for j in i + 1..n {
            sum += a[[i, j]] * x[j];
        }
        if a[[i, i]].abs() > 1e-12 {
            x[i] = (b[i] - sum) / a[[i, i]];
        } else {
            x[i] = 0.0; // Zabezpieczenie
        }
    }

    x
}


fn save_to_csv(x: &Vec<f64>, u: &ndarray::Array1<f64>, filename: &str) {
    let mut file = std::fs::File::create(filename).expect("Nie można utworzyć pliku");
    writeln!(file, "x,u(x)").expect("Błąd zapisu");
    for i in 0..x.len() {
        writeln!(file, "{:.6},{:.6}", x[i], u[i]).expect("Błąd zapisu");
    }
}
