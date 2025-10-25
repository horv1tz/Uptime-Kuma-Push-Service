# Uptime Kuma Push Service

Это легковесный фоновый сервис, который периодически отправляет heartbeat-запросы на заданный URL-адрес Uptime Kuma. Это позволяет отслеживать работоспособность и доступность системы, на которой запущен сервис.

## Технические требования

- **Язык**: Rust
- **Конфигурация**: Переменные окружения
- **Логирование**: stdout

## Конфигурация

Сервис настраивается с помощью следующих переменных окружения:

- `UPTIME_KUMA_PUSH_URL`: (Обязательно) Полный URL для push-монитора Uptime Kuma.
- `UPTIME_KUMA_PUSH_INTERVAL_SECONDS`: (Опционально) Интервал между запросами в секундах. По умолчанию: `60`.

## Сборка

Для сборки проекта вам понадобится установленный Rust.

1.  Клонируйте репозиторий:
    ```sh
    git clone <repository_url>
    cd uptime-kuma-push
    ```

2.  Соберите проект в release-режиме:
    ```sh
    cargo build --release
    ```

    Исполняемый файл будет находиться в `target/release/uptime-kuma-push`.

### Сборка с использованием Nix

Если у вас установлен Nix, вы можете использовать `shell.nix` для создания изолированного окружения для сборки.

1.  Войдите в Nix shell:
    ```sh
    nix-shell
    ```

2.  Внутри Nix shell, соберите проект:
    ```sh
    cargo build --release
    ```

## Запуск

Для запуска сервиса установите переменные окружения и запустите исполняемый файл.

### Linux / macOS
```sh
export UPTIME_KUMA_PUSH_URL="http://your-uptime-kuma-instance/api/push/your-push-token"
export UPTIME_KUMA_PUSH_INTERVAL_SECONDS="30"
./target/release/uptime-kuma-push
```

### Windows
```cmd
set UPTIME_KUMA_PUSH_URL="http://your-uptime-kuma-instance/api/push/your-push-token"
set UPTIME_KUMA_PUSH_INTERVAL_SECONDS="30"
.\target\release\uptime-kuma-push.exe
```

Сервис начнет отправлять heartbeat-запросы и выводить логи в консоль. Для остановки сервиса используйте `Ctrl+C`.
