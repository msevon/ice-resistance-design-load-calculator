use std::io;

// Constants and parameters
const G: f64 = 9.81; // Acceleration due to gravity (m/s^2)
const RHO_W: f64 = 1025.0; // Density of seawater (kg/m^3)
const RHO_I: f64 = 920.0; // Density of ice (kg/m^3)
const MU_H: f64 = 0.3; // Coefficient of friction between ship's side and ice
const SIGMA_B: f64 = 500000.0; // Ice bending strength (Pa)

// Function to calculate Lindqvist components
fn calculate_lindqvist_components(L: f64, B: f64, T: f64, phi: f64, psi: f64, alpha: f64, h_ice: f64) -> (f64, f64, f64) {
    let rc = 0.5 * SIGMA_B * h_ice.powi(2) * (f64::tan(f64::to_radians(phi))
        + MU_H * f64::cos(f64::to_radians(phi)) / f64::cos(f64::to_radians(psi)))
        / (1.0 - MU_H * f64::sin(f64::to_radians(phi)) / f64::cos(f64::to_radians(psi)));

    let rb = 0.003 * SIGMA_B * B * h_ice.powf(1.5) * (f64::tan(f64::to_radians(psi))
        + MU_H * f64::cos(f64::to_radians(phi))
            / (f64::sin(f64::to_radians(alpha)) * f64::cos(f64::to_radians(psi)))
        * (1.0 + 1.0 / f64::cos(f64::to_radians(psi))));

    let rs = (RHO_W - RHO_I) * G * h_ice * B
        * (T * (B + T) / (B + 2.0 * T)
            + MU_H * (0.7 * L - T / f64::tan(f64::to_radians(phi))
                - B / (4.0 * f64::tan(f64::to_radians(alpha))
                    + T * f64::cos(f64::to_radians(phi))
                        * f64::cos(f64::to_radians(psi))
                        * (1.0 / f64::sin(f64::to_radians(phi)).powi(2)
                            + 1.0 / f64::tan(f64::to_radians(alpha)).powi(2)))));

    (rc, rb, rs)
}

// Function to calculate total ice resistance
fn calculate_total_ice_resistance(rc: f64, rb: f64, rs: f64, L: f64, v: f64, h_ice: f64) -> f64 {
    let r_ice = (rc + rb) * (1.0 + 1.4 * v / (f64::sqrt(G * h_ice))) + rs * (1.0 + 9.4 * v / (f64::sqrt(G * L)));
    r_ice
}

// Function to get valid numeric input
fn get_valid_input(prompt: &str) -> f64 {
    loop {
        let mut input = String::new();
        println!("{}", prompt);
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse() {
            Ok(value) if value >= 0.0 => return value,
            _ => println!("Invalid input. Please enter a valid non-negative numeric value."),
        }
    }
}

fn main() {
    loop {
        println!("Options:");
        println!("0: End program");
        println!("1: Calculate level ice resistance");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "0" => {
                println!("Exiting the program.");
                break; // Exit the loop and end the program
            }
            "1" => {
                let l = get_valid_input("Enter the length of the ship (m): ");
                let b = get_valid_input("Enter the breadth of the ship (m): ");
                let t = get_valid_input("Enter the draft of the ship (m): ");
                let v = get_valid_input("Enter the ship speed (kn): ");
                let phi = get_valid_input("Enter the angle (trim) in degrees: ");
                let psi = get_valid_input("Enter the angle (ship keel - direction of motion) in degrees: ");
                let alpha = get_valid_input("Enter the angle (ship side - waterline) in degrees: ");
                let h_ice = get_valid_input("Enter the ice thickness in cm: ") / 100.0;

                if h_ice > 0.0 {
                    let (rc, rb, rs) = calculate_lindqvist_components(l, b, t, phi, psi, alpha, h_ice);
                    let r_ice = calculate_total_ice_resistance(rc, rb, rs, l, v, h_ice) / 1000.0;
                    println!("Ice resistance: {:.2} kN\n", r_ice); // Provide the value for formatting
                } else {
                    println!("Ice resistance: 0 kN\n");
                }
            }
            _ => println!("Invalid choice. Please select option 0 or 1."),
        }
    }
}