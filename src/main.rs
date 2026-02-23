use sysinfo::{
    Components, Disks, Networks, System,
}; // Либа для информации про ОС
use std::thread;
use std::time::Duration; // Таймер

use std::process::Command; // Использывать команды ОС
use colored::*; // Либа для цветов

fn draw_bar(used: u64, total: u64, width: usize) -> String {
    if total == 0 { return "[] 0% (0 / 0 MB)".to_string(); }

    let percentage = used as f64 / total as f64;
    let filled_len = (percentage * width as f64).round() as usize;
    let empty_len = width - filled_len;

    // Логика цвета (как мы обсуждали ранее)
    let color_func = if percentage > 0.9 {
        |s: &str| s.red().bold()
    } else if percentage > 0.7 {
        |s: &str| s.yellow().bold()
    } else {
        |s: &str| s.green().bold()
    };

    let filled = "█".repeat(filled_len);
    let empty = "░".repeat(empty_len);

    // Собираем всё в одну строку с цифрами в конце
    format!(
        "[{}{}] {:>3.0}%  {:>5} / {:>5} MB",
        color_func(&filled),
        empty.white(),
        percentage * 100.0,
        used.to_string().bright_white(), // Используемые МБ
        total.to_string().cyan()         // Всего МБ
    )
}

#[cfg(target_os = "linux")]
fn get_gpu_name() -> String {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("lspci | grep -E 'VGA|3D' | cut -d ':' -f3")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unknown Linux GPU".into());
    output
}

#[cfg(target_os = "windows")]
fn get_gpu_name() -> String {
    // В Windows используем PowerShell (wmic)
    let output = std::process::Command::new("powershell")
        .arg("-Command")
        .arg("Get-CimInstance Win32_VideoController | Select-Object -ExpandProperty Name")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "Unknown Windows GPU".into());
    output
}

#[cfg(target_os = "macos")]
fn get_gpu_name() -> String {
    "Apple M-series GPU".to_string() // Или системный профиль
}

fn main() {
    loop {
        if cfg!(target_os = "windows") {
            // Вызываем cmd, передаем аргумент /c (выполнить и закрыться) и саму команду cls
            let _ = std::process::Command::new("cmd")
                .args(["/c", "cls"])
                .status();
        } else {
            Command::new("clear").status().unwrap();
        }

        let mut sys = System::new_all(); // Получение информации и запись в sys
        sys.refresh_all(); // Обновление данных


        println!("{}", "====== Micro-top by Hetman ======".yellow().bold()); // Стартовый текст
        // Память

        let used_mem = sys.used_memory() / 1024 / 1024; // Используеться ОЗУ
        let total_mem = sys.total_memory() / 1024 / 1024; // Всего ОЗУ
        println!("{}", draw_bar(used_mem, total_mem, 20)); // Вывод ОЗУ

        let used_swap = sys.used_swap() / 1024 / 1024; // Используеться Swap
        let total_swap = sys.total_swap() / 1024 / 1024; // Всего Swap
        println!("{}", draw_bar(used_swap, total_swap, 20));;

        println!("------------------------------------------------");


        // Информация о системе

        let kernel = System::kernel_version().unwrap_or_else(|| "unknown".into()); // Информация про версию ядра
        let os_ver = System::os_version().unwrap_or_else(|| "unknown".into()); // Информация про версию ОС
        let host   = System::host_name().unwrap_or_else(|| "localhost".into()); // Информация про имя хоста

        println!("{}: {}", "Kernel version".green().bold(), kernel.green().bold());
        println!("{}:     {}", "OS version".green().bold(), os_ver.green().bold());
        println!("{}:      {}", "Host name".green().bold(), host.green().bold());

        println!("------------------------------------------------");


        // Информация про процессор и видеокарту

        // 1. Вывод кол-ва ядер
        println!("{}: {}",
                 "CPUs numbers".magenta().bold(),
                 sys.cpus().len().to_string().magenta().bold()
        );

        // 2. Вывод названия процессора
        if let Some(cpu) = sys.cpus().first() {
            println!("{}:    {}",
                     "CPU name".magenta().bold(),
                     cpu.brand().magenta().bold()
            );
        } else {
            // Если процессор не найден
            println!("{}", "CPU not found".red().bold());
        }

        // 3. Вывод информации про видеокарту
        println!("{}:    {}",
                 "GPU(s) name".magenta().bold(),
                 get_gpu_name().magenta().bold()
        );

        println!("=========================================");


        thread::sleep(Duration::from_secs(5));
    }

}
