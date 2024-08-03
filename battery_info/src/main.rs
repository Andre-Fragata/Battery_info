use std::fs;

fn read_battery_info() -> Result<(String, String, String), std::io::Error> {
    // Define o caminho para os arquivos de informações da bateria
    let capacity_path = "/sys/class/power_supply/BAT1/capacity";
    let status_path = "/sys/class/power_supply/BAT1/status";
    let chargeFull_path = "/sys/class/power_supply/BAT1/charge_full_design";

    // Lê o conteúdo dos arquivos
    let capacity = fs::read_to_string(capacity_path)?.trim().to_string();
    let status = fs::read_to_string(status_path)?.trim().to_string();
    let charge_full_design = fs::read_to_string(chargeFull_path)?.trim().to_string();

    Ok((capacity, status, charge_full_design))
}

fn main() {
    match read_battery_info() {
        Ok((capacity, status, charge_full_design)) => {
            println!("Capacidade da Bateria: {}%", capacity);
            println!("Status da Bateria: {}", status);
            println!("Design total da bateria original: {}", charge_full_design);
        },
        Err(e) => eprintln!("Erro ao ler informações da bateria: {}", e),
    }
}

