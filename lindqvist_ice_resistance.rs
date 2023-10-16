use std::io;

// Constants and parameters
const G: f64 = 9.81; // Acceleration due to gravity (m/s^2)
const RHO_W: f64 = 1025.0; // Density of seawater (kg/m^3)
const RHO_I: f64 = 920.0; // Density of ice (kg/m^3)
const MU_H: f64 = 0.3; // Coefficient of friction between ship's side and ice
const SIGMA_B: f64 = 500000.0; // Ice bending strength (Pa)

fn calculate_lindqvist_components(L: f64, B: f64, T: f64, phi: f64, psi: f64, alpha: f64, h_ice: f64) -> (f64, f64, f64) {
    let rc = 0.5 * SIGMA_B * h_ice.powi(2) * (f64::tan(f64::to_radians(phi)) + MU_H * f64::cos(f64::to_radians(phi)) / f64::cos(f64::to_radians(psi)))
        / (1.0 - MU_H * f64::sin(f64::to_radians(phi)) / f64::cos(f64::to_radians(psi)));
    let rb = 0.003 * SIGMA_B * B * h_ice.powf(1.5) * (f64::tan(f64::to_radians(psi))
        + MU_H * f64::cos(f64::to_radians(phi)) / (f64::sin(f64::to_radians(alpha)) * f64::cos(f64::to_radians(psi)))
        * (1.0 + 1.0 / f64::cos(f64::to_radians(psi))));
    let rs = (RHO_W - RHO_I) * G * h_ice * B * ((T * (B + T)) / (B + 2.0 * T)
        + MU_H * (0.7 * L - T / f64::tan(f64::to_radians(phi))
        - B / (4.0 * f64::tan(f64::to_radians(alpha))
        + T * f64::cos(f64::to_radians(phi)) * f64::cos(f64::to_radians(psi))
        * (1.0 / f64::sin(f64::to_radians(phi)).powi(2)
        + 1.0 / f64::tan(f64::to_radians(alpha)).powi(2)))));
    (rc, rb, rs)
}

fn calculate_total_ice_resistance(rc: f64, rb: f64, rs: f64, L: f64, v: f64, h_ice: f64) -> f64 {
    let r_ice = (rc + rb) * (1.0 + 1.4 * v / (f64::sqrt(G * h_ice))) + rs * (1.0 + 9.4 * v / (f64::sqrt(G * L)));
    r_ice
}

fn main() {
    let mut input = String::new();

    println!("Enter the length of the ship (m): ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let l: f64 = input.trim().parse().expect("Invalid input");

    input.clear();

    println!("Enter the breadth of the ship (m): ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let b: f64 = input.trim().parse().expect("Invalid input");

    input.clear();

    println!("Enter the draft of the ship (m): ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let t: f64 = input.trim().parse().expect("Invalid input");

    input.clear();

    println!("Enter the ship speed (kn): ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let v: f64 = input.trim().parse().expect("Invalid input");

    input.clear();

    println!("Enter the angle (trim) in degrees: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let phi: f64 = input.trim().parse().expect("Invalid input");

    input.clear();

    println!("Enter the angle (ship keel - direction of motion) in degrees: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let psi: f64 = input.trim().parse().expect("Invalid input");

    input.clear();

    println!("Enter the angle (ship side - waterline) in degrees: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let alpha: f64 = input.trim().parse().expect("Invalid input");

    input.clear();

    println!("Enter the ice thickness in cm: ");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let h_ice: f64 = (input.trim().parse::<f64>().expect("Invalid input")) / 100.0; // Convert ice thickness from cm to meters

    if h_ice > 0.0 {
        // Calculate Lindqvist components and total ice resistance
        let (rc, rb, rs) = calculate_lindqvist_components(l, b, t, phi, psi, alpha, h_ice);
        let r_ice = calculate_total_ice_resistance(rc, rb, rs, l, v, h_ice) / 1000.0; // kN

        // Print the results
        println!("Ice resistance: {:.2} kN", r_ice);
    } else {
        println!("Ice resistance: 0 kN");
    }
}
