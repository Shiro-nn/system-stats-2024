# system‑stats 📊

[![GitHub stars](https://img.shields.io/github/stars/Shiro-nn/system-stats-2024?style=social)](https://github.com/Shiro-nn/system-stats-2024/stargazers)
[![GitHub forks](https://img.shields.io/github/forks/Shiro-nn/system-stats-2024?style=social)](https://github.com/Shiro-nn/system-stats-2024/network/members)
[![GitHub issues](https://img.shields.io/github/issues/Shiro-nn/system-stats-2024)](https://github.com/Shiro-nn/system-stats-2024/issues)
[![GitHub last commit](https://img.shields.io/github/last-commit/Shiro-nn/system-stats-2024)](https://github.com/Shiro-nn/system-stats-2024/commits)
[![License: MIT](https://img.shields.io/github/license/Shiro-nn/system-stats-2024)](LICENSE)
[![Status: Archived](https://img.shields.io/badge/status-archived-lightgrey.svg)](https://github.com/Shiro-nn/system-stats-2024)

![Repo Stats](https://github-readme-stats.vercel.app/api/pin/?username=Shiro-nn\&repo=system-stats-2024)

> **system‑stats** — кроссплатформенный Rust‑агент, который раз в минуту собирает показатели нагрузки (CPU, память, диски, сеть, аптайм, количество процессов) и сохраняет их в MongoDB. Проект переведён в архив: новые фичи не планируются, но код остаётся доступным «как есть».

---

## ✨ Возможности

| Метрика      | Что фиксируется                                                                               |
| ------------ | --------------------------------------------------------------------------------------------- |
| **CPU**      | Процент загрузки и частота каждого ядра (через [`sysinfo`](https://crates.io/crates/sysinfo)) |
| **Память**   | Общий объём, занято, свободно, кэш, swap                                                      |
| **Диски**    | Использование каждой точки монтирования, тип накопителя                                       |
| **Сеть**     | Трафик по каждому сетевому интерфейсу (вход/выход, байт)                                      |
| **Процессы** | Общее число активных процессов                                                                |
| **Общее**    | Аптайм хоста, hostname, внешний IP, пользовательская `category`                               |

Дополнительно агент:

* **Удаляет записи старше 24 часов**, чтобы база не разрасталась без контроля.
* **Шифрует строку подключения** к MongoDB в бинарнике через [`include_crypt`](https://crates.io/crates/include_crypt): никаких «голых» URI в исходниках.
* **Работает на Linux / macOS / Windows** — везде, где поддерживается `sysinfo` и официальные драйверы MongoDB.

---

## 🚀 Быстрый старт

> ⚠️ **Внимание:** проект заархивирован. Нижеописанная установка поддерживается «как есть». Pull‑request’ы всё ещё приветствуются, но мейнтейнер не гарантирует оперативного ответа.

### 1. Клонирование и сборка

```bash
git clone https://github.com/Shiro-nn/system-stats-2024.git
cd system-stats-2024
cargo build --release   # необходим Rust 1.75+
```

Бинарник появится в `target/release/system-stats` (`.exe` под Windows).

### 2. Настройка MongoDB

1. Создайте пользователя с доступом на запись в вашу базу;
2. Сформируйте URI:

   ```
   mongodb+srv://USER:PASSWORD@cluster0.mongodb.net/monitoring?retryWrites=true&w=majority
   ```
3. Зашифруйте URI командой:

   ```bash
   echo "mongodb+srv://USER:PASSWORD@..." | include_crypt-cli encrypt -k "your‑secret‑key" > src/mongodb.txt
   ```
4. Пересоберите проект — строка будет зашита в бинарник.

### 3. Запуск агента

```bash
./system-stats-2024 "MyServer / Prod"
```

*Аргументы после имени бинарника объединяются в поле `category` и помогают фильтровать хосты в дальнейшем.* Если аргументов нет, значение по умолчанию — **Unknown**.

Агент выводит лог подключения, затем раз в минуту добавляет новую запись.

---

## 📦 Формат документа в MongoDB

```json
{
  "category": "MyServer / Prod",
  "name": "my‑hostname",
  "uptime": 86400,
  "date": 1721116800000,
  "processes": 175,
  "cpus": [
    { "load": "12%", "frequency": 3292 },
    { "load": "08%", "frequency": 3292 }
  ],
  "memory": {
    "total": 16777216,
    "load": 9453312,
    "cache": 2149580,
    "used": 7239456,
    "swap_total": 0,
    "swap_free": 0,
    "swap_used": 0
  },
  "disks": [
    { "load": "43%", "usage": 50465865728, "name": "/dev/sda1 (SSD)" }
  ],
  "network": [
    { "name": "eth0", "inbount": 102400, "outbount": 51200 }
  ]
}
```

---

## 📝 Пример запроса на агрегирование

```js
// Вывести среднюю загрузку CPU по каждому хосту за последний час
db.system.aggregate([
  { $match: { date: { $gte: Date.now() - 3600 * 1000 } } },
  { $group: {
      _id: "$name",
      avgCpu: { $avg: { $toDouble: { $substr: ["$cpus.load", 0, 2] } } }
  } },
  { $sort: { avgCpu: -1 } }
]);
```

---

## 🤝 Вклад

* Форкайте репозиторий, создавайте ветку `feature/your-feature`, отправляйте PR.
* Перед пушем запустите `cargo fmt && cargo clippy -- -D warnings`.

---

## ⚖️ Лицензия

Код распространяется под лицензией **MIT**. Используйте, как считаете нужным — но помните, что проект больше **не поддерживается официально**.
