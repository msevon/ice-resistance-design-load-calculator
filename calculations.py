import math
from math import radians, sin

# Constants and parameters
G = 9.81  # Acceleration due to gravity (m/s^2)
RHO_W = 1025.0  # Density of seawater (kg/m^3)
RHO_I = 920.0  # Density of ice (kg/m^3)
MU_H = 0.3  # Coefficient of friction between ship's side and ice
SIGMA_B = 500000.0  # Ice bending strength (Pa)

# Function to calculate Lindqvist components
def calculate_lindqvist_components(L, B, T, phi, psi, alpha, h_ice):
    rc = 0.5 * SIGMA_B * h_ice ** 2 * (math.tan(math.radians(phi)) +
        MU_H * math.cos(math.radians(phi)) / math.cos(math.radians(psi))) / (1.0 - MU_H * math.sin(math.radians(phi)) / math.cos(math.radians(psi)))

    rb = 0.003 * SIGMA_B * B * h_ice ** 1.5 * (math.tan(math.radians(psi)) +
        MU_H * math.cos(math.radians(phi)) / (math.sin(math.radians(alpha)) * math.cos(math.radians(psi)) * (1.0 + 1.0 / math.cos(math.radians(psi)))))

    rs = (RHO_W - RHO_I) * G * h_ice * B * (T * (B + T) / (B + 2.0 * T) +
        MU_H * (0.7 * L - T / math.tan(math.radians(phi)) - B / (4.0 * math.tan(math.radians(alpha)) + T * math.cos(math.radians(phi)) * math.cos(math.radians(psi)) * (1.0 / math.sin(math.radians(phi))**2 + 1.0 / math.tan(math.radians(alpha))**2))))

    return rc, rb, rs

# Function to calculate total ice resistance
def calculate_total_ice_resistance(rc, rb, rs, L, v, h_ice):
    r_ice = (rc + rb) * (1.0 + 1.4 * v / (math.sqrt(G * h_ice))) + rs * (1.0 + 9.4 * v / (math.sqrt(G * L)))
    return r_ice

# Function to get valid numeric input
def get_valid_input(prompt):
    while True:
        try:
            value = float(input(prompt))
            if value >= 0.0:
                return value
            else:
                print("Invalid input. Please enter a valid non-negative numeric value.")
        except ValueError:
            print("Invalid input. Please enter a valid non-negative numeric value.")

# Function to calculate design ice load
def calculate_design_ice_load(length_ui, deadweight_ui, polar_class, beta_prime, alpha, gamma):
    # Step 1: Define class factors for all ice classes
    class_factors = {
        "PC1": (17.69, 68.60, 2.01, 250, 7.46),
        "PC2": (9.89, 46.80, 1.75, 210, 5.46),
        "PC3": (6.06, 21.17, 1.53, 180, 4.17),
        "PC4": (4.50, 13.48, 1.42, 130, 3.15),
        "PC5": (3.10, 9.00, 1.31, 70, 2.50),
        "PC6": (2.40, 5.49, 1.17, 40, 2.37),
        "PC7": (1.80, 4.06, 1.11, 22, 1.81),
    }

    # Update class factors based on the polar class
    if polar_class in class_factors:
        CFC, CFF, CFD, CFDIS, CFL = class_factors[polar_class]
        print(f"\nShape Factors for {polar_class}:")
        print(f"  CFC: {CFC}")
        print(f"  CFF: {CFF}")
        print(f"  CFD: {CFD}")
        print(f"  CFDIS: {CFDIS}")
        print(f"  CFL: {CFL}")
    else:
        print("\nInvalid Polar Class. Please enter a valid Polar Class. Ending calculation...")
        return

    # Step 2: Initalize the bow
    bow = {
            "name": "Bow",
            "beta_prime": beta_prime,
            "alpha": alpha,
            "gamma": gamma,
            "x": 0.25 * length_ui,
            "Dui": deadweight_ui,
        }


    # Step 3: Calculate fai, Fi, ARi, Qi, and Pi
    x = bow["x"]
    beta_prime = bow["beta_prime"]
    alpha = bow["alpha"]
    gamma = bow["gamma"]
    Dui = bow["Dui"]

    # (a) Calculate fai
    fai1 = (0.097 - 0.68 * ((x / length_ui - 0.15) ** 2)) * alpha / (beta_prime ** 0.5)
    fai2 = 1.2 * CFF / (sin(radians(beta_prime)) * CFC * Dui ** 0.64)
    fai3 = 0.60
    fai = min(fai1, fai2, fai3)

    # (b) Calculate Fi
    Fi = fai * CFC * Dui ** 0.64

    # (c) Calculate ARi
    ARi = max(7.46 * sin(radians(beta_prime)), 1.3)

    # (d) Calculate Qi
    Qi = Fi ** 0.61 * CFD / ARi ** 0.35

    # (e) Calculate Pi
    Pi = Fi ** 0.22 * CFD ** 2 * ARi ** 0.3

    bow["fai"] = fai
    bow["Fi"] = Fi
    bow["ARi"] = ARi
    bow["Qi"] = Qi
    bow["Pi"] = Pi

    # Step 4: Calculate design load patch
    b = Fi / Qi
    w = Qi / Pi
    bow["b"] = b
    bow["w"] = w

    # Step 5: Calculate design average pressure (Pavg)
    Pavg = Fi / (b * w)

    # Print results
    print(f"\nResults:")
    print(f"  fai: {bow['fai']:.2f}")
    print(f"  Fi: {bow['Fi']:.2f} MN")
    print(f"  ARi: {bow['ARi']:.2f}")
    print(f"  Qi: {bow['Qi']:.2f} MN/m")
    print(f"  Pi: {bow['Pi']:.2f} MPa")
    print(f"  Design Load Patch (b x w): {bow['b']:.2f} x {bow['w']:.2f} m")
    print(f"  Design Average Pressure (Pavg): {Pavg:.2f} MPa")



if __name__ == "__main__":
    while True:
        print("\nOptions:")
        print("0: End program")
        print("1: Calculate level ice resistance")
        print("2: Calculate design ice load at bow")

        choice = input()
        
        # End program
        if choice == "0":
            print("Exiting the program.")
            break  # Exit the loop and end the program
        
        # Level ice resistance
        elif choice == "1":
            l = get_valid_input("\nEnter the length of the ship (m): ")
            b = get_valid_input("Enter the breadth of the ship (m): ")
            t = get_valid_input("Enter the draft of the ship (m): ")
            v = get_valid_input("Enter the ship speed (kn): ")
            phi = get_valid_input("Enter the angle (trim) in degrees: ")
            psi = get_valid_input("Enter the angle (ship keel - direction of motion) in degrees: ")
            alpha = get_valid_input("Enter the angle (ship side - waterline) in degrees: ")
            h_ice = get_valid_input("Enter the ice thickness in cm: ") / 100.0

            if h_ice > 0.0:
                rc, rb, rs = calculate_lindqvist_components(l, b, t, phi, psi, alpha, h_ice)
                r_ice = calculate_total_ice_resistance(rc, rb, rs, l, v, h_ice) / 1000.0
                print(f"Ice resistance: {r_ice:.2f} kN\n")
            else:
                print("Ice resistance: 0 kN\n")
        
        # Design ice load
        elif choice == "2":
            length_ui = get_valid_input("\nEnter the ship's upper ice waterline length (Lui) in meters: ")
            deadweight_ui = get_valid_input("Enter the ship's deadweight (Dui in kt) at Upper Ice Waterline (UIWL): ")
            
            while True:
                polar_class = input("Enter the Polar Class (PC1, PC2, PC3, PC4, PC5, PC6, or PC7): ")
                if polar_class in {"PC1", "PC2", "PC3", "PC4", "PC5", "PC6", "PC7"}:
                    break
                print("Not acceptable Polar Class. Polar Class should be between PC1 - PC7")
            
            beta_prime = get_valid_input("Enter the normal frame angle at upper ice waterline for the Bow (β' in degrees):")
            alpha =  get_valid_input("Enter the upper ice waterline angle for the Bow (α in degrees): ")
            gamma = get_valid_input("Enter the buttock angle at upper ice waterline for the Bow (γ in degrees): ")
            calculate_design_ice_load(length_ui, deadweight_ui, polar_class, beta_prime, alpha, gamma)
        else:
            print("Invalid choice. Please select option 0, 1, or 2.")
