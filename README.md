
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
git clone git@github.com:TimeBean/wallpaper.git

или

https://github.com/TimeBean/wallpaper.git

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

