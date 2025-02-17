# Neuro Worker

Neuro Worker это прокси-сервер для доступа к Yandex Neuro API. Может использоваться с [neurojs](https://github.com/FOSWLY/neurojs)

## Поддержка

Neuro Worker поддерживает работу с:

- Официальным API суммаризации (получение sharing-url)

  префикс: `/th`

  `POST /th/api/sharing-url`

- Браузерным API суммаризации (создание сессии и суммаризация видео)

  префикс: `/browser`

  `POST /browser/session/create`

  `POST /browser/video-summary/generation`

- Веб API суммаризации (300) с YaHMAC или Cookies

  префикс: `/th`

  С YaHMAC:

  `POST /th/api/neuro/generation`

  С Cookies:

  `POST /th/api/generation`

  Получение существующей суммаризации по токену из ссылки (С YaHmac или Cookies)

  `POST /th/sharing`

- Самопроверка. Проверьте, работает ли сервис

  `GET /health`

## Зачем?

Вам нужен Neuro Worker, если:

1. У вас заблокированы сервера Yandex
2. Вам нужно обойти CORS

## Как запустить

Чтобы запустить свой инстанс:

### С Docker

1. Установите Docker
2. Соберите образ

```bash
docker build -t "neuro-worker" .
```

3. Запустите контейнер

```bash
docker run -p 7674:7674 neuro-worker
```

### Ручная сборка и запуск

1. Установите [Rust 1.75+](https://www.rust-lang.org/learn/get-started)

   1.1. Для linux также установите:

```bash
# ubuntu / debian
sudo apt install build-essential pkg-config
```

2. (Опционально) Запуск для разработки:

   2.1. Установите `cargo watch`:

   ```bash
   cargo install cargo-watch
   ```

   2.2. Запустите live сервер:

   ```bash
   cargo watch -x run
   ```

3. Запуск для Production:

   3.1. Соберите:

   ```bash
   cargo build --release
   ```

   3.2. Запустите файл `neuro-worker`:

   ```bash
   ./target/release/neuro-worker
   ```
