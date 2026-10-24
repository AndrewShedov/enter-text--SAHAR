[![GitHub Release](https://img.shields.io/github/v/release/AndrewShedov/enter-text--SAHAR?style=for-the-badge&logo=github&logoColor=white&labelColor=black&color=%23f3f3f3)](https://github.com/AndrewShedov/enter-text--SAHAR/releases)&nbsp;[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge&logo=5865F2&logoColor=black&labelColor=black&color=%23f3f3f3)](https://github.com/AndrewShedov/enter-text--SAHAR/blob/main/LICENSE)&nbsp;[![Members](https://img.shields.io/badge/dynamic/json?style=for-the-badge&label=&logo=discord&logoColor=white&labelColor=black&color=%23f3f3f3&query=$.approximate_member_count&url=https%3A%2F%2Fdiscord.com%2Fapi%2Finvites%2FENB7RbxVZE%3Fwith_counts%3Dtrue)](https://discord.gg/ENB7RbxVZE)

# Enter Text (SAHAR)

[![En](https://img.shields.io/badge/En-black?style=flat&logoColor=whit)](https://github.com/AndrewShedov/enter-text--SAHAR)<br>

> **Исследовательская статья:** <br>
>  [💎 Дневник разработчика CRYSTAL #3: Сравнительное исследование производительности и кибербезопасности клиент-серверного взаимодействия в технологических стеках LARS и SAHAR](https://shedov.top/ru/dnevnik-razrabotchika-crystal-3-sravnitelnoe-issledovanie-lars-i-sahar/)

Это приложение является прототипом, разработанным в рамках поиска нового технологического стека для CRYSTAL v3.0.<br>
Данный прототип позволяет **добавлять**, **отображать**, **обновлять** и **удалять текст**, демонстрируя полный цикл **CRUD**-операций с базой данных ScyllaDB.<br>
Благодаря использованию **компилируемых шаблонов Askama** и **HTMX**, приложение обеспечивает молниеносный **рендеринг на стороне сервера (SSR)** с бесшовным частичным обновлением DOM. Это гарантирует **100% индексацию поисковыми системами**, безупречную **SEO-оптимизацию** и максимальную устойчивость к уязвимостям.

<p align="center">
  <img src="https://raw.githubusercontent.com/AndrewShedov/enter-text--SAHAR/refs/heads/main/assets/gif.gif"/>
</p>

**Содержание:**<br>
1. [Структура проекта и характеристики локального ПК](#paragraph_1)
2. [Ключевые особенности](#paragraph_2)
3. [Установка и настройка](#paragraph_3)
4. [Подготовка базы данных](#paragraph_4)
5. [Запуск проекта](#paragraph_5)

<span id="paragraph_1"></span> 
### 1. Структура проекта и характеристики локального ПК 

**Строение:** <br>
[Full code](https://github.com/AndrewShedov/enter-text--SAHAR/tree/main/main) | [Cargo.toml](https://github.com/AndrewShedov/enter-text--SAHAR/blob/main/main/Cargo.toml)<br>

**Структура:** <br>
**S**cyllaDB v2026.2.6 (драйвер: v1.8.0).<br>
**A**ctix Web v4.15.0.<br>
**H**TMX v4.0.0.<br>
**A**skama v0.16.<br>
**R**ust v1.98.0.<br>

**Характеристики локального ПК:** <br>
ОС: Debian 12.<br>

<span id="paragraph_2"></span> 
### 2. Ключевые особенности: 

**2.1. Авто-схема (Auto-Schema), однострочная архитектура и константный id**<br>
При запуске приложения автоматически создается таблица <code>sahar_prototype.data</code> (если она не существует) для хранения введенного текста. Во время операции сохранения в таблице формируется строка, состоящая из трех столбцов: <code>id</code> (Первичный ключ), <code>content</code> (текстовые данные) и <code>created_at</code> (временная метка). Для введенного текста используется постоянный <code>id</code> в формате <code>UUID (11111111-1111-1111-1111-111111111111)</code>. Вместо создания множества записей система использует операцию <code>INSERT</code> как <code>"upsert"</code> (обновление существующей записи). Поскольку <code>id</code> всегда один и тот же, любая операция сохранения просто перезаписывает данные в столбце content для этой конкретной строки:

<p align="center">
  <img src="https://raw.githubusercontent.com/AndrewShedov/enter-text--SAHAR/refs/heads/main/assets/screenshot-1.png"/>
</p>
<p align="center"><strong>Скриншот 1: Отображение единственной строки в БД</strong></p>

**2.2. SSR с нулевыми накладными расходами через Askama**<br>
В отличие от традиционных шаблонизаторов, работающих в рантайме, Askama компилирует HTML-шаблоны непосредственно в код Rust во время сборки. Это обеспечивает абсолютную безопасность типов и устраняет накладные расходы на парсинг в реальном времени, мгновенно отдавая клиентам и поисковым роботам полностью отрендеренные страницы:

<p align="center">
  <img src="https://raw.githubusercontent.com/AndrewShedov/enter-text--SAHAR/refs/heads/main/assets/screenshot-2.png"/>
</p>
<p align="center"><strong>Скриншот 2: Рендеринг на стороне сервера (SSR). Просмотр исходного кода в браузере (Ctrl+U)</strong></p>

**2.3. SPA-подобная реактивность с помощью HTMX**<br>
За счет использования атрибутов `hx-post` и `hx-target` приложение выполняет частичную замену HTML вместо полной перезагрузки страницы. Бэкенд на Actix Web динамически определяет запросы от HTMX через заголовок `hx-request` и отвечает только необходимым фрагментом HTML, радикально уменьшая размер передаваемых данных.

**2.4. Работает с отключенным JavaScript**<br>
Приложение спроектировано таким образом, чтобы оставаться полностью функциональным даже в случае отключения JavaScript в браузере пользователя. Опираясь на серверный рендеринг (SSR) с шаблонами Askama и нативные HTML-элементы `<form>`, базовые операции с данными (добавление, обновление и удаление контента) плавно переходят на стандартную отправку HTML-форм с классической перезагрузкой страницы. Это обеспечивает абсолютную отказоустойчивость.

Вы можете протестировать это поведение, отключив JavaScript в своем браузере:
* **Google Chrome / Chromium:** Откройте Инструменты разработчика (`F12`), нажмите `Ctrl+Shift+P` (или `Cmd+Shift+P` на macOS) для открытия командного меню, введите `Disable JavaScript` и нажмите Enter.
* **Mozilla Firefox:** Введите `about:config` в адресной строке, примите риски, найдите параметр `javascript.enabled` и дважды кликните по нему, чтобы переключить значение на `false`.

**2.5. Усиленная архитектура кибербезопасности**<br>
Спроектировано строго в соответствии с передовыми практиками (best practices) кибербезопасности для предотвращения несанкционированного проникновения в систему и эксплуатации уязвимостей:
* **Анти-Инъекция:** 100% взаимодействий с базой данных проходят через подготавливаемые запросы (`PreparedStatement`) ScyllaDB, что полностью исключает возможность CQL-инъекций.
* **Анти-CSRF:** Кастомный middleware (промежуточное ПО) явным образом валидирует заголовок `Origin`, сверяя его с `ALLOWED_ORIGIN`, блокируя попытки подделки межсайтовых запросов (Cross-Site Request Forgery).
* **Защита от DoS:** Лимиты на размер полезной нагрузки (`FormConfig::limit(4096)`) защищают сервер от атак, направленных на исчерпание памяти.
* **Безупречная CSP и заголовки безопасности:** Применяются принудительно через middleware Actix Web. Бескомпромиссная Политика Защиты Контента (Content Security Policy) достигается полным отказом от использования директивы `'unsafe-inline'` как для скриптов (`script-src 'self'`), так и для стилей (`style-src 'self'`). Поскольку инлайн-скрипты отсутствуют, необходимость в динамической генерации `nonce` на стороне сервера полностью отпадает, что значительно снижает вычислительную нагрузку на сервер. Для соответствия этим строгим правилам внедрение инлайн-стилей по умолчанию в HTMX отключено через конфигурацию мета-тега (`<meta name="htmx-config" content='{"includeIndicatorStyles": false}'>`), успешно нейтрализуя все векторы XSS и внедрения CSS (UI Redressing):

```rust
.app_data(app_state.clone())
            // Лимит входящих данных (Защита от DoS-атак через огромные payload'ы)
            .app_data(web::FormConfig::default().limit(4096))
            // Строгие заголовки безопасности (Нулевая терпимость к 'unsafe-inline')
            .wrap(
                middleware::DefaultHeaders::new()
                    // Защита от кликджекинга (Clickjacking)
                    .add(("X-Frame-Options", "DENY"))
                    // Защита от MIME-сниффинга
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add((
                        "Content-Security-Policy",
                        // Отключение инлайн скриптов и стилей. Разрешен только собственный Origin ('self')
                        "default-src 'self'; script-src 'self'; style-src 'self';",
                    ))
                    .add((
                        "Strict-Transport-Security",
                        "max-age=31536000; includeSubDomains; preload",
                    ))
                    .add(("Referrer-Policy", "strict-origin-when-cross-origin")),
            ) 
```

**2.6. Асинхронная интеграция ScyllaDB**<br>
Высокопроизводительное асинхронное подключение через <code>scylla-rust-driver</code>. Благодаря использованию разделяемого состояния <code>Arc&lt;Session&gt;</code> и заранее подготовленных запросов (prepared statements), загружаемых при старте, сервер может обрабатывать тысячи одновременных запросов без блокировки потоков процессора.

**2.7. Информативное логирование сервера**<br>
Система выводит в консоль подробные отчеты об операциях:

<p align="center">
    <img src="https://raw.githubusercontent.com/AndrewShedov/enter-text--SAHAR/refs/heads/main/assets/screenshot-3.png" width="750" />
</p>
<p align="center"><strong>Скриншот 3: Лог готовности ScyllaDB</strong></p>

<span id="paragraph_3"></span> 
### 3. Установка и настройка

**Примечание о совместимости:** Работа данного проекта проверена на Debian 12. Разработка на Windows не рекомендуется. Для корректной работы требуется Linux-среда.

**3.1. Подготовка окружения (Debian 12 и аналоги)**<br>

Установка системных зависимостей:

```bash
sudo apt update && sudo apt install build-essential pkg-config libssl-dev -y
```

**3.2. Установка Rust**

**3.2.1. Установите Rust (устанавливает текущую стабильную версию):**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**3.2.2. Примените переменные окружения:**

```bash
source $HOME/.cargo/env
```

**3.3 Установка и настройка ScyllaDB.**

Установка производится непосредственно в систему Debian 12 с использованием официального репозитория ScyllaDB.

**3.3.1. Обновите индексы пакетов:**

```bash
sudo apt-get update
```

**3.3.2. Установите необходимые системные утилиты:**

```bash
sudo apt-get install -y apt-transport-https curl gnupg
```

**3.3.3. Добавьте официальный репозиторий ScyllaDB в систему:**

📌 Примечание: По умолчанию этот скрипт автоматически регистрирует репозиторий для последней стабильной версии ScyllaDB Open Source. Чтобы явно установить ScyllaDB v2026.2.6, используемую в этом прототипе, выполните команду с флагом версии:

```bash
curl -sSf https://get.scylladb.com/server | sudo bash -s -- --scylla-version 2026.2
```

**3.3.4. Интерактивная конфигурация и установка ScyllaDB:**

```bash
sudo scylla_setup
```

**3.3.5. Запустите службу сервера ScyllaDB:**

```bash
sudo systemctl start scylla-server
```

**3.3.6. Проверьте статус узлов кластера:**

```bash
nodetool status
```
 
<span id="paragraph_4"></span> 
### 4. Подготовка базы данных

Перед первым запуском проекта необходимо создать пространство ключей (Keyspace) в ScyllaDB.

Войдите в консоль базы данных с помощью команды:

```bash
cqlsh
```

и выполните запрос:

```bash
CREATE KEYSPACE IF NOT EXISTS sahar_prototype 
WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1};
```

<span id="paragraph_5"></span> 
### 5. Запуск проекта

**5.1. Клонирование репозитория проекта:** 

```bash
git clone https://github.com/AndrewShedov/enter-text--SAHAR && cd enter-text--SAHAR/main 
```

**5.2. Запуск проекта:**

```bash
cargo run
```

После завершения сборки приложение будет доступно по адресу:

<code>http://127.0.0.1:8080</code>  

Таблица **data** внутри пространства ключей **sahar_prototype** будет создана автоматически при первом запросе приложения к базе данных благодаря встроенной логике Авто-схемы (Auto-Schema).<br>  

По умолчанию ScyllaDB настроена на работу с адресом <code>127.0.0.1:9042</code>.<br>
Вы можете проверить адрес, введя команду в терминал:

```bash
cqlsh
```

После ввода команды должен отобразиться адрес:<br>

<code>Connected to at 127.0.0.1:9042</code>
<br>
<br>

<p align="center">
    <img src="https://raw.githubusercontent.com/AndrewShedov/enter-text--SAHAR/refs/heads/main/assets/screenshot-5.png" width="300" />
</p>
 
<div align="center">
  <a href="https://www.youtube.com/watch?v=0B5I_nPQDtw&list=RD0B5I_nPQDtw&start_radio=1&pp=ygUeYW15IGhvbGxhbmQgdHVybiBvdXQgdGhlIGxpZ2h0oAcB">
    <img src="https://img.shields.io/badge/Amy_Holland_--_Turn_Out_The_Night-ffffff?style=for-the-badge&logo=youtube&logoColor=red" alt="Amy Holland Soundtrack">
  </a>
</div>

<br>
<br>


[![SHEDOV.TOP](https://img.shields.io/badge/SHEDOV.TOP-black?style=for-the-badge)](https://shedov.top/) [![CRYSTAL](https://img.shields.io/badge/CRYSTAL-black?style=for-the-badge)](https://crystal.you/AndrewShedov) [![Discord](https://img.shields.io/badge/Discord-black?style=for-the-badge&logo=discord&color=black&logoColor=white)](https://discord.gg/ENB7RbxVZE) [![Telegram](https://img.shields.io/badge/Telegram-black?style=for-the-badge&logo=telegram&color=black&logoColor=white)](https://t.me/ShedovTop) [![X](https://img.shields.io/badge/%20-black?style=for-the-badge&logo=x&logoColor=white)](https://x.com/AndrewShedov) [![YouTube](https://img.shields.io/badge/YouTube-black?style=for-the-badge&logo=youtube)](https://www.youtube.com/@AndrewShedov) [![VK](https://img.shields.io/badge/VK-black?style=for-the-badge&logo=vk)](https://vk.com/ShedovTop) [![VK Video](https://img.shields.io/badge/VK%20Video-black?style=for-the-badge&logo=vk)](https://vkvideo.ru/@ShedovTop)