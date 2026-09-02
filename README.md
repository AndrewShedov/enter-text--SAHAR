[![GitHub Release](https://img.shields.io/github/v/release/AndrewShedov/enter-text--SAHAR?style=for-the-badge&logo=github&logoColor=white&labelColor=black&color=%23f3f3f3)](https://github.com/AndrewShedov/enter-text--SAHAR/releases)&nbsp;[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg?style=for-the-badge&logo=5865F2&logoColor=black&labelColor=black&color=%23f3f3f3)](https://github.com/AndrewShedov/enter-text--SAHAR/blob/main/LICENSE)&nbsp;[![Members](https://img.shields.io/badge/dynamic/json?style=for-the-badge&label=&logo=discord&logoColor=white&labelColor=black&color=%23f3f3f3&query=$.approximate_member_count&url=https%3A%2F%2Fdiscord.com%2Fapi%2Finvites%2FENB7RbxVZE%3Fwith_counts%3Dtrue)](https://discord.gg/ENB7RbxVZE)

# Enter Text (SAHAR)

This application is a prototype developed as part of the search for a new technology stack for CRYSTAL v3.0.<br>
This prototype allows you to **add**, **display**, **update**, and **delete text**, demonstrating a full **CRUD** operation cycle with the ScyllaDB database.<br>
By utilizing **Askama compile-time templates** and **HTMX**, the application delivers lightning-fast **Server-Side Rendering (SSR)** with seamless partial DOM updates. This guarantees **100% search engine indexing**, flawless **SEO performance**, and maximum resistance to vulnerabilities.

<p align="center">
  <img src="https://raw.githubusercontent.com/AndrewShedov/enter-text--SAHAR/refs/heads/main/assets/gif.gif"/>
</p>

**Contents:**<br>
1. [Project structure and local PC specifications](#paragraph_1)
2. [Key Features](#paragraph_2)
3. [Installation & Setup](#paragraph_3)
4. [Database Preparation](#paragraph_4)
5. [Launching the project](#paragraph_5)

<span id="paragraph_1"></span> 
### 1. Project structure and local PC specifications 

**Composition:** <br>
[Full code](https://github.com/AndrewShedov/enter-text--SAHAR/tree/main/main) | [Cargo.toml](https://github.com/AndrewShedov/enter-text--SAHAR/blob/main/main/Cargo.toml)<br>

**Structure:** <br>
**S**cyllaDB v2026.2.6 (driver: v1.8.0).<br>
**A**ctix Web v4.15.0.<br>
**H**TMX v2.0.10.<br>
**A**skama v0.16.<br>
**R**ust v1.98.0.<br>

**Local PC Specifications:** <br>
OS: Debian 12.<br>

<span id="paragraph_2"></span> 
### 2. Key Features: 

**2.1. Auto-Schema, Single-Row Architecture, and Constant-id**<br>
Upon application startup, the <code>sahar_prototype.data</code> table is automatically created (if it does not exist) to store the entered text. During the save operation, a row is formed in the table, consisting of three columns: <code>id</code> (Primary Key), <code>content</code> (text data), and <code>created_at</code> (timestamp). A constant <code>id</code> of <code>UUID format (11111111-1111-1111-1111-111111111111)</code> is used for the entered text. Instead of creating multiple entries, the system uses the <code>INSERT</code> operation as an <code>"upsert"</code> (updating an existing record). Since the <code>id</code> is always the same, any save operation simply overwrites the data in the content column for this specific row:

<p align="center">
  <img src="https://raw.githubusercontent.com/AndrewShedov/enter-text--SAHAR/refs/heads/main/assets/screenshot-1.png"/>
</p>
<p align="center"><strong>Screenshot 1: Single Row View</strong></p>

**2.2. Zero-Overhead SSR via Askama**<br>
Unlike traditional runtime template engines, Askama compiles HTML templates directly into Rust code during the build process. This provides absolute type safety and eliminates runtime parsing overhead, serving fully rendered pages to clients and web crawlers instantly:

<p align="center">
  <img src="https://raw.githubusercontent.com/AndrewShedov/enter-text--SAHAR/refs/heads/main/assets/screenshot-2.png"/>
</p>
<p align="center"><strong>Screenshot 2: Server-Side Rendered (SSR). View source code in a browser (Ctrl+U)</strong></p>

**2.3. SPA-like Reactivity with HTMX**<br>
By leveraging `hx-post` and `hx-target` attributes, the application performs partial HTML swaps instead of full page reloads. The Actix Web backend dynamically detects HTMX requests via the `hx-request` header and responds only with the necessary HTML fragment, drastically reducing payload sizes.

**2.4. Works with JavaScript Disabled**<br>
The application is designed to remain fully functional even if JavaScript is disabled in the user's browser. Leveraging server-side rendering with Askama templates and native HTML `<form>` elements, basic data operations (adding, updating, and deleting content) seamlessly fall back to native HTML form submissions with standard page reloads. This ensures absolute fault tolerance.

You can test this behavior by disabling JavaScript in your browser:
* **Google Chrome / Chromium:** Open Developer Tools (`F12`), press `Ctrl+Shift+P` (or `Cmd+Shift+P` on macOS) to open the Command Menu, type `Disable JavaScript`, and press Enter.
* **Mozilla Firefox:** Type `about:config` in the address bar, accept the risk, search for the `javascript.enabled` preference, and double-click it to toggle the value to `false`.

**2.5. Hardened Cybersecurity Architecture**<br>
Designed strictly according to cybersecurity best practices to prevent unauthorized system penetration and exploitation:
* **Anti-Injection:** 100% of database interactions run through ScyllaDB `PreparedStatement`s, completely eliminating the possibility of CQL injections.
* **Anti-CSRF:** Custom middleware explicitly validates the `Origin` header against `ALLOWED_ORIGIN`, blocking Cross-Site Request Forgery attempts.
* **DoS Protection:** Payload limits (`FormConfig::limit(4096)`) protect the server from memory exhaustion attacks.
* **Strict Security Headers:** Native Actix Web middleware enforces `X-Frame-Options: DENY`, `X-Content-Type-Options: nosniff`, `X-XSS-Protection`, strict `Content-Security-Policy`, and `HSTS`. 

**2.6. Asynchronous ScyllaDB Integration**<br>
High-performance asynchronous connection via <code>scylla-rust-driver</code>. Thanks to the use of a shared <code>Arc&lt;Session&gt;</code> and pre-compiled statements loaded at startup, the server can handle thousands of concurrent requests without blocking CPU threads.

**2.7. Informative Server Logging**<br>
The system outputs informative operation reports to the console:

<p align="center">
    <img src="https://raw.githubusercontent.com/AndrewShedov/enter-text--SAHAR/refs/heads/main/assets/screenshot-3.png" width="750" />
</p>
<p align="center"><strong>Screenshot 3: ScyllaDB readiness log</strong></p>

<span id="paragraph_3"></span> 
### 3. Installation & Setup

**Compatibility Note:** This project is verified to work on Debian 12. Development on Windows is not recommended. A Linux-based environment is required for correct operation.

**3.1. Environment Preparation (Debian 12 and similar)**<br>

Installing system dependencies:

```bash
sudo apt update && sudo apt install build-essential pkg-config libssl-dev -y
```

**3.2. Installing Rust**

**3.2.1. Install Rust (installs the current stable version):**

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**3.2.2. Configure environment variables:**

```bash
source $HOME/.cargo/env
```

**3.3 ScyllaDB Installation and Configuration.**

Installation is performed directly on the Debian 12 system using the official ScyllaDB repository.

**3.3.1. Update package indexes:**

```bash
sudo apt-get update
```

**3.3.2. Install necessary system utilities:**

```bash
sudo apt-get install -y apt-transport-https curl gnupg
```

**3.3.3. Add the official ScyllaDB repository to the system:**

📌 Note: By default, this script automatically registers the repository for the latest stable version of ScyllaDB Open Source. To explicitly install ScyllaDB v2026.2.6 used in this prototype, run the command with the version flag:

```bash
curl -sSf https://get.scylladb.com/server | sudo bash -s -- --scylla-version 2026.2
```

**3.3.4. Interactive configuration and ScyllaDB installation:**

```bash
sudo scylla_setup
```

**3.3.5. Start the ScyllaDB server service:**

```bash
sudo systemctl start scylla-server
```

**3.3.6. Check the status of cluster nodes:**

```bash
nodetool status
```
 
<span id="paragraph_4"></span> 
### 4. Database Preparation

Before the first project launch, you must create a Keyspace in ScyllaDB.

Enter the database console using the:

```bash
cqlsh
```

and execute the query:

```bash
CREATE KEYSPACE IF NOT EXISTS sahar_prototype 
WITH replication = {'class': 'SimpleStrategy', 'replication_factor': 1};
```

<span id="paragraph_5"></span> 
### 5. Launching the project

**5.1. Cloning the project repository:** 

```bash
git clone https://github.com/AndrewShedov/enter-text--SAHAR && cd enter-text--SAHAR/main 
```

**5.2. Launch the project:**

```bash
cargo run
```

Once the build is complete, the application will be available at:

<code>http://127.0.0.1:8080</code>  

The **data** table inside the **sahar_prototype**  keyspace will be created automatically upon the application's first request to the database, enabled by the built-in Auto-Schema logic.<br>  

By default, ScyllaDB is configured to work with the address <code>127.0.0.1:9042</code>.<br>
You can verify the address by entering the command into the terminal:

```bash
cqlsh
```

After entering the command, the address should be displayed:<br>

<code>Connected to at 127.0.0.1:9042</code>
<br>
<br>
>SAHAR is the Russian word 'САХАР' ([IPA](https://en.wikipedia.org/wiki/Help:IPA/Russian): [ˈsaxər]), meaning 'sugar'.
<br>
<br>

[![SHEDOV.TOP](https://img.shields.io/badge/SHEDOV.TOP-black?style=for-the-badge)](https://shedov.top/) [![CRYSTAL](https://img.shields.io/badge/CRYSTAL-black?style=for-the-badge)](https://crystal.you/AndrewShedov) [![Discord](https://img.shields.io/badge/Discord-black?style=for-the-badge&logo=discord&color=black&logoColor=white)](https://discord.gg/ENB7RbxVZE) [![Telegram](https://img.shields.io/badge/Telegram-black?style=for-the-badge&logo=telegram&color=black&logoColor=white)](https://t.me/ShedovTop) [![X](https://img.shields.io/badge/%20-black?style=for-the-badge&logo=x&logoColor=white)](https://x.com/AndrewShedov) [![VK](https://img.shields.io/badge/VK-black?style=for-the-badge&logo=vk)](https://vk.com/ShedovTop) [![VK Video](https://img.shields.io/badge/VK%20Video-black?style=for-the-badge&logo=vk)](https://vkvideo.ru/@ShedovTop) [![YouTube](https://img.shields.io/badge/YouTube-black?style=for-the-badge&logo=youtube)](https://www.youtube.com/@AndrewShedov)