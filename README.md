# wallpaper — automation for setting wallpapers and generating palettes

---

## English — Short summary

**Short:** `wallpaper` is a minimalist CLI tool written in Rust for setting desktop wallpapers and automatically generating a matching color palette (swww → wallust → matugen). Made for Linux (Hyprland, Sway, Wayland environments). Simple to configure and extend.

---

## 1. Requirements

- Rust (stable) and `cargo` to build.
- External utilities installed (depending on your configuration):
  - `matugen` (or your palette generator)
  - `wallust` (pywal-compatible utility)
  - `swww` (swww for Wayland) or any other backend to set wallpapers
  - (optional) `rfd` — used as a dependency library for GUI file dialogs

> Note: binary names and their arguments may differ on your system. Check versions and available flags.

---

## 2. Quick install — Build from source

```bash
git clone git@github.com:TimeBean/wallpaper.git

# or

git clone https://github.com/TimeBean/wallpaper.git

cd wallpaper
cargo build --release
# the binary will be in target/release/wallpaper
```

### Install system-wide (example)

```bash
sudo install -Dm755 target/release/wallpaper /usr/local/bin/wallpaper
```

---

## 3. Usage examples — Simple CLI run

```bash
# Set default palette and wallpaper from path/to/image.jpg
wallpaper path/to/image.jpg

# Same, but light palette
wallpaper -l path/to/image.jpg

# With explicit matugen type
wallpaper --type scheme-tonal-spot path/to/image.jpg
```

### GUI file chooser

```bash
wallpaper --gui
```

### Example output

```text
wallpaper v0.2 - /home/user/Pictures/wall.jpg
matugen: image /home/user/Pictures/wall.jpg --type scheme-tonal-spot
wallust: run /home/user/Pictures/wall.jpg -k --palette light
swww: img /home/user/Pictures/wall.jpg --transition-fps 60 --transition-duration 1
Done.
```

# wallpaper — автоматизация установки обоев и генерации палитры

---

## Русский — Кратко

**Кратко:** `wallpaper` — это минималистичный CLI-инструмент на Rust для установки обоев и автоматической генерации цветовой палитры (swww → wallust → matugen). Сделан для Linux (окружения Hyprland, Sway, Wayland), прост в настройке и расширении.

---

## 1. Требования

- Rust (stable) и `cargo` для сборки.
- Установленные внешние утилиты (в зависимости от конфигурации):
  - `matugen` (или ваш генератор палитры)
  - `wallust` (утилита, совместимая с pywal)
  - `swww` (swww для Wayland) или любой другой бекенд для установки обоев
  - (опционально) `rfd` — используется как зависимость библиотеки для GUI-диалогов

> Примечание: названия бинарей и их аргументы могут различаться на вашей системе. Проверьте их версии и доступные флаги.

---

## 2. Быстрая установка — Собрать из исходников

```bash
git clone git@github.com:TimeBean/wallpaper.git

или

git clone https://github.com/TimeBean/wallpaper.git

cd wallpaper
cargo build --release
# бинарник окажется в target/release/wallpaper
```

### Установка в систему (пример)

```bash
sudo install -Dm755 target/release/wallpaper /usr/local/bin/wallpaper
```

---

## 3. Примеры использования — Простой запуск (CLI)

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

