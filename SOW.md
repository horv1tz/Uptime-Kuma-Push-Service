# Statement of Work: Uptime Kuma Push Service

## 1. Введение (Introduction)

Цель этого проекта — разработать легковесный фоновый сервис, который будет периодически отправлять heartbeat-запросы на заданный URL-адрес Uptime Kuma. Это позволит отслеживать работоспособность и доступность системы, на которой запущен сервис.

## 2. Объем работ (Scope of Work)

Разработать, протестировать и задокументировать консольное приложение, выполняющее следующие функции:
- Периодически отправлять HTTP GET-запросы на URL-адрес Uptime Kuma.
- Конфигурироваться с помощью переменных окружения.
- Обеспечивать логирование своей работы.
- Корректно обрабатывать ошибки сети и завершать работу по сигналу от ОС.

## 3. Технические требования (Technical Requirements)

### 3.1. Язык программирования (Programming Language)
- Сервис должен быть написан на **Go** или **Rust**.
- Основной критерий выбора — минимальный размер конечного исполняемого файла (бинарника) при сохранении производительности и надежности.

### 3.2. Конфигурация (Configuration)
Приложение должно настраиваться через переменные окружения:
- `UPTIME_KUMA_PUSH_URL`: Полный URL для push-монитора Uptime Kuma (обязательный параметр).
- `UPTIME_KUMA_PUSH_INTERVAL_SECONDS`: Интервал между запросами в секундах. Значение по умолчанию: `60`.

### 3.3. Логирование (Logging)
- Сервис должен выводить информацию о своей работе в `stdout`.
- Формат логов: `[YYYY-MM-DD HH:MM:SS] [LEVEL] Message`.
- Уровни логов: `INFO` для успешных отправок, `ERROR` для ошибок.
- Пример:
  ```
  [2023-10-25 13:30:00] [INFO] Heartbeat sent successfully to Uptime Kuma.
  [2023-10-25 13:31:00] [ERROR] Failed to send heartbeat: <описание ошибки>.
  ```

### 3.4. Обработка ошибок (Error Handling)
- Приложение должно корректно обрабатывать ошибки HTTP-запросов (например, таймауты, недоступность сети).
- В случае ошибки необходимо записать ее в лог и повторить попытку через заданный интервал.

### 3.5. Завершение работы (Graceful Shutdown)
- Сервис должен корректно завершать свою работу при получении сигналов `SIGINT` и `SIGTERM` от операционной системы.

## 4. Пример реализации (Implementation Example)

Приведенный ниже код на Go демонстрирует базовую логику, которую необходимо расширить в соответствии с требованиями выше.

```go
package main

import (
	"log"
	"net/http"
	"os"
	"strconv"
	"time"
)

func main() {
	// Получение URL из переменной окружения
	pushURL := os.Getenv("UPTIME_KUMA_PUSH_URL")
	if pushURL == "" {
		log.Fatal("ERROR: UPTIME_KUMA_PUSH_URL environment variable is not set.")
	}

	// Получение интервала из переменной окружения
	intervalStr := os.Getenv("UPTIME_KUMA_PUSH_INTERVAL_SECONDS")
	if intervalStr == "" {
		intervalStr = "60" // Значение по умолчанию
	}
	interval, err := strconv.Atoi(intervalStr)
	if err != nil {
		log.Fatalf("ERROR: Invalid interval value: %s", intervalStr)
	}

	log.Printf("INFO: Starting Uptime Kuma push service. URL: %s, Interval: %d seconds", pushURL, interval)

	// Основной цикл
	ticker := time.NewTicker(time.Duration(interval) * time.Second)
	defer ticker.Stop()

	for range ticker.C {
		resp, err := http.Get(pushURL)
		if err != nil {
			log.Printf("ERROR: Failed to send heartbeat: %v", err)
			continue
		}
		resp.Body.Close()
		if resp.StatusCode >= 200 && resp.StatusCode < 300 {
			log.Println("INFO: Heartbeat sent successfully.")
		} else {
			log.Printf("ERROR: Received non-2xx status code: %d", resp.StatusCode)
		}
	}
}
```
*Примечание: Данный пример не включает обработку сигналов ОС для корректного завершения работы.*

## 5. Ожидаемые результаты (Deliverables)
1. Исходный код сервиса на выбранном языке (Go или Rust).
2. `README.md` файл с инструкциями по сборке и запуску.
3. (Опционально) `Dockerfile` для контейнеризации приложения.
