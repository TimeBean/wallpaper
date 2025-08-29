# wallpaper — автоматизация установки обоев и генерации палитры

**Кратко:** `wallpaper` — это минималистичный CLI-инструмент на Rust для установки обоев и автоматической генерации цветовой палитры (matugen → wal/wallust → swww). Сделан для Linux (Hyprland, Sway, Wayland окружения), прост в настройке и расширении.

---

## Содержание (Outline)

1. Описание
2. Особенности
3. Требования
4. Быстрая установка
5. Примеры использования
   - CLI
   - GUI
   - Режимы и опции
6. Конфигурация (пример)
7. Порядок выполнения команд и причинность
8. Debug / Troubleshooting
9. Разработка и тесты
10. CI (GitHub Actions) — пример
11. systemd user service — пример
12. Contributing
13. Changelog
14. License

---

## 1. Описание

`wallpaper` — утилита, которая берет изображение, генерирует палитру (matugen или другой генератор), применяет её через `wallust`/`wal` и ставит обои через `swww` (или любой другой бекенд). Цель — обеспечить воспроизводимый, тестируемый и безопасный pipeline для управления рабочим столом.

Проект оптимизирован под повседневное использование и интеграцию в автоматизацию (скрипты, systemd user, cron). Локально тестировалось в Ростове-на-Дону (Rostov-on-Don) на Arch Linux с Hyprland.

---

## 2. Особенности

- Явный и предсказуемый CLI (на базе `clap`).
- Безопасная обработка путей (`PathBuf`, `args_os`).
- Чёткая обработка ошибок (`anyhow`) и логирование stdout/stderr.
- Поддержка GUI выбора файла через `rfd`.
- Лёгкая интеграция в systemd user service.
- Dry-run, light/dark палитры и настраиваемый `matugen`-тип.

---

## 3. Требования

- Rust (stable) и `cargo` для сборки.
- Установленные внешние утилиты (в зависимости от конфигурации):
  - `matugen` (или ваш генератор палитры)
  - `wallust` или `wal` (pywal-совместимая утилита)
  - `swww` (swww для Wayland) или любой другой бекенд для установки обоев
  - (опционально) `rfd` — используется как зависимость библиотеки для GUI-диалогов

> Примечание: названия бинарей и их аргументы могут различаться на вашей системе. Проверьте их версии и доступные флаги.

---

## 4. Быстрая установка

### Собрать из исходников

```bash
git clone <репозиторий>
cd wallpaper
cargo build --release
# бинарник окажется в target/release/wallpaper
```

### Установка в систему (пример)

```bash
sudo install -Dm755 target/release/wallpaper /usr/local/bin/wallpaper
```

---

## 5. Примеры использования

### Простой запуск (CLI)

```bash
# Установит палитру по умолчанию и обои из path/to/image.jpg
wallpaper path/to/image.jpg

# То же, но светлая палитра
wallpaper -l path/to/image.jpg

# С явным типом matugen
wallpaper --type scheme-tonal-spot path/to/image.jpg
```

### Запуск с GUI для выбора файла

```bash
wallpaper --gui
```

### Пример вывода

```text
wallpaper v0.2 - /home/user/Pictures/wall.jpg
matugen: image /home/user/Pictures/wall.jpg --type scheme-tonal-spot
wallust: run /home/user/Pictures/wall.jpg -k --palette light
swww: img /home/user/Pictures/wall.jpg --transition-fps 60 --transition-duration 1
Done.
```

---

## 6. Конфигурация (пример)

Создайте `~/.config/wallpaper/config.toml` (опционально) для хранения дефолтных значений:

```toml
# ~/.config/wallpaper/config.toml
matugen_type = "scheme-tonal-spot"
light = false
swww_transition_fps = 60
swww_transition_duration = 1
```

Код читает конфиг при старте (если файл присутствует) и переопределяет значения из CLI.

---

## 7. Порядок выполнения команд и объяснение

Рекомендованный pipeline:

1. **matugen** — анализирует изображение и пишет палитру/JSON/файл с цветами.
2. **wallust/wal** — применяет палитру к системе/темам/декорациям.
3. **swww** — устанавливает изображение в качестве обоев и выполняет анимацию/переход.

Почему такой порядок: генерация палитры должна происходить до применения, иначе байпас цветовой схемы случится с прошлой палитрой.

---

## 8. Debug / Troubleshooting

### Частые ошибки

- `failed to spawn command` — бинарного файла нет в PATH. Проверьте `which matugen`/`which wallust`/`which swww`.
- `rfd` не запускает диалог — вы работаете в headless-сессии (нет X/Wayland). Запускайте `--gui` только в рабочей сессии.
- Некорректная кодировка путей — используйте `args_os()` или проверяйте `PathBuf`.

### Логирование

Программа выводит stdout/stderr исполняемых команд. Для более подробного логирования можно направлять в файл:

```bash
wallpaper /path/to/img.jpg 2>&1 | tee ~/wallpaper.log
```

### Проверка порядка и dry-run

Запустите с `--dry-run` (если реализовано), чтобы увидеть сформированные команды без их выполнения.

---

## 9. Разработка и тесты

### Локальная сборка

```bash
cargo build
```

### Запуск тестов

```bash
cargo test
```

### Линтер

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Как добавить тесты

- Unit tests: парсинг CLI, формирование аргументов для `Command`.
- Integration tests: мок-обёртки для `Command` (интерфейс-адаптер), чтобы не запускать реальные бинарники.

---

## 10. CI: GitHub Actions — пример

`.github/workflows/ci.yml`:

```yaml
name: CI
on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build
        run: cargo build --release
      - name: Clippy
        run: cargo clippy --all-targets --all-features -- -D warnings
      - name: Test
        run: cargo test --workspace --verbose
```

---

## 11. systemd user service — пример

Создайте `~/.config/systemd/user/wallpaper.service`:

```ini
[Unit]
Description=Wallpaper setter

[Service]
Type=simple
ExecStart=/usr/local/bin/wallpaper /home/youruser/Pictures/wall.jpg
Restart=on-failure

[Install]
WantedBy=default.target
```

Активировать:

```bash
systemctl --user daemon-reload
systemctl --user enable --now wallpaper.service
```

---

## 12. Contributing

1. Форкните репозиторий.
2. Создайте ветку feature/bugfix: `git checkout -b feature/awesome`
3. Реализуйте изменения, убедитесь, что `cargo test` проходит.
4. Откройте Pull Request с описанием изменений.

Правила кодстайла: используйте `rustfmt` и `clippy`.

---

## 13. Changelog

Следуйте Semantic Versioning. Ветка `main` содержит последние стабильные изменения.

**v0.2** — refactor: clap + anyhow, PathBuf, run_cmd_and_log, GUI selector

---

## 14. License

Укажите желаемую лицензию: `MIT`, `Apache-2.0` или другую. Пример добавления `LICENSE` файла в репо.

---

## 15. Дополнительные заметки

- Используйте `PathBuf` и `args_os` для корректной работы с международными путями.
- Проверяйте флаги конкретных утилит на целевой машине — синтаксис флагов может отличаться.
- Если планируете поддержку Windows/macOS, инкапсулируйте бекенды установки обоев за trait и реализуйте платформенные адаптеры.

---

Если нужно, могу:
- добавить шаблоны ISSUE/PR;
- подготовить `.gitattributes` и `.editorconfig`;
- добавить CI-юнит для packaging (deb/aur) и пример release workflow.

