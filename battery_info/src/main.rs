use std::fs;

fn read_battery_info() -> Result<(String, String), std::io::Error> {
    // Define o caminho para os arquivos de informações da bateria
    let capacity_path = "/sys/class/power_supply/BAT1/capacity";
    let status_path = "/sys/class/power_supply/BAT1/status";

    // Lê o conteúdo dos arquivos
    let capacity = fs::read_to_string(capacity_path)?.trim().to_string();
    let status = fs::read_to_string(status_path)?.trim().to_string();

    Ok((capacity, status))
}

fn main() {
    match read_battery_info() {
        Ok((capacity, status)) => {
            println!("Capacidade da Bateria: {}%", capacity);
            println!("Status da Bateria: {}", status);
        },
        Err(e) => eprintln!("Erro ao ler informações da bateria: {}", e),
    }
}

