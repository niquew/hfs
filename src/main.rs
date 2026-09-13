use std::sync::{Arc, Mutex};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{copy_bidirectional};

// Структура для мониторинга нашего гидротического прокси
struct ProxyStats {
    total_connections: u32,
    active_connections: u32,
}

#[tokio::main]
async fn main() {
    println!("--- THFS: The Hydrotic Forwarding System ---");

    // 1. Список серверов, куда мы будем перенаправлять трафик.
    // Заворачиваем в Arc<Mutex>, чтобы список можно было менять на лету из других тасок.
    let backends = Arc::new(Mutex::new(vec![
        "127.0.0.1:8001".to_string(),
        "127.0.0.1:8002".to_string(),
    ]));

    // 2. Общая статистика, которую будут обновлять все Tokio-таски при подключении клиентов
    let stats = Arc::new(Mutex::new(ProxyStats {
        total_connections: 0,
        active_connections: 0,
    }));

    // Индекс для алгоритма балансировки (Round Robin)
    let current_backend_index = Arc::new(Mutex::new(0));

    // Запускаем прокси на порту 8080
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("[THFS] Прокси-сервер успешно запущен на порту :8080");

    // Бесконечный цикл приема клиентов
    loop {
        // Ждем подключения нового клиента
        let (mut client_stream, client_addr) = match listener.accept().await {
            Ok(val) => val,
            Err(_) => continue,
        };

        // Клонируем наши Arc-указатели для новой Tokio-таски
        let backends_clone = Arc::clone(&backends);
        let stats_clone = Arc::clone(&stats);
        let index_clone = Arc::clone(&current_backend_index);

        // Обрабатываем каждого клиента асинхронно в отдельной таске
        tokio::spawn(async move {
            // Обновляем статистику: заходим в Mutex
            {
                let mut s = stats_clone.lock().unwrap();
                s.total_connections += 1;
                s.active_connections += 1;
                println!(
                    "[THFS] Новый клиент: {}. Всего подключений: {}, Активных: {}", 
                    client_addr, s.total_connections, s.active_connections
                );
            }

            // Выбираем бэкенд для перенаправления (Round Robin)
            let target_backend = {
                let targets = backends_clone.lock().unwrap();
                let mut idx = index_clone.lock().unwrap();
                
                if targets.is_empty() {
                    println!("[THFS] Ошибка: Список серверов для перенаправления пуст!");
                    return;
                }
                
                let target = targets[*idx].clone();
                // Сдвигаем индекс на следующий сервер
                *idx = (*idx + 1) % targets.len();
                target
            };

            println!("[THFS] Перенаправляем {} -> {}", client_addr, target_backend);

            // Пытаемся подключиться к выбранному бэкенду
            if let Ok(mut backend_stream) = TcpStream::connect(&target_backend).await {
                // Магия Tokio: связываем клиента и сервер «трубой» в обе стороны.
                // Данные будут течь туда-сюда автоматически.
                let _ = copy_bidirectional(&mut client_stream, &mut backend_stream).await;
            } else {
                println!("[THFS] Ошибка подключения к бэкенду: {}", target_backend);
            }

            // Клиент отключился — уменьшаем счетчик активных соединений
            {
                let mut s = stats_clone.lock().unwrap();
                s.active_connections -= 1;
                println!("[THFS] Клиент {} отключился. Активных: {}", client_addr, s.active_connections);
            }
        });
    }
}
