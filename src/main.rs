use chrono::{DateTime, Local};
use std::thread;
use std::time::{self};
use sysinfo::{self, System};
mod db;

#[derive(Debug)]
struct ProcessInfo {
    name: String,
    start: DateTime<Local>,
}

fn main() {
    collect_process_statistics();
}

fn collect_process_statistics() {
    let cooldown = time::Duration::from_secs(5); // Интервал сбора статистики
    let mut sys = sysinfo::System::new_all();

    loop {
        thread::sleep(cooldown);
        sys.refresh_all();

        let current_time = Local::now();
        println!(
            "Собираем статистику процессов на {}",
            current_time.format("%d/%m/%Y %H:%M")
        );

        let processes = sys.processes();
        let mut process_stats: Vec<ProcessInfo> = Vec::new();

        for (pid, process) in processes {
            let process_info = ProcessInfo {
                name: process.name().to_string_lossy().into_owned(), // Преобразуем OsStr в String
                start: current_time, // Можно добавить логику для отслеживания времени запуска
            };
            process_stats.push(process_info);
            println!(
                "Процесс: {} (PID: {})",
                process.name().to_string_lossy(),
                pid
            );
        }

        // Здесь можно сохранить собранные данные в базу данных
        if let Ok(mut sql) = db::Sql::new() {
            for process_info in &process_stats {
                match sql.save(process_info) {
                    Ok(_) => println!("Статистика процесса {} сохранена в БД", process_info.name),
                    Err(e) => eprintln!(
                        "Ошибка при сохранении статистики процесса {}: {}",
                        process_info.name, e
                    ),
                }
            }
        }
    }
}
