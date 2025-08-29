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

## 3. Usage examples — Detailed CLI usage

### Basic usage with file path

```bash
# Set default palette and wallpaper from path/to/image.jpg
wallpaper path/to/image.jpg

# Same, but with light palette
wallpaper -l path/to/image.jpg

# With explicit matugen scheme type
wallpaper --type scheme-tonal-spot path/to/image.jpg

# Light palette with custom scheme type
wallpaper -l --type scheme-monochrome path/to/image.jpg
```

### GUI file chooser

```bash
# Open graphical file picker (mutually exclusive with path)
wallpaper --gui

# GUI with light palette
wallpaper -l --gui

# GUI with custom scheme type
wallpaper --gui --type scheme-expressive
```

### Available matugen scheme types

- `scheme-content` (default content-based scheme)
- `scheme-expressive` (expressive, vibrant colors)
- `scheme-fidelity` (close to source image colors)
- `scheme-fruit-salad` (playful, diverse colors)
- `scheme-monochrome` (monochromatic scheme)
- `scheme-neutral` (muted, neutral colors)
- `scheme-rainbow` (rainbow-like colors)
- `scheme-tonal-spot` (default, balanced tonal scheme)

### Error cases and validation

```bash
# ERROR: Cannot use both --gui and path simultaneously
wallpaper --gui /path/to/image.jpg
# error: the argument '--gui' cannot be used with '[PATH]'

# ERROR: Must provide either --gui or path
wallpaper
# Error: Either --gui or a path must be provided

# ERROR: Path validation
wallpaper /nonexistent/file.jpg
# Error: Path does not exist: /nonexistent/file.jpg
```

### Example output

```text
wallpaper v0.4.1 - fix local and repo mispush - /home/user/Pictures/wall.jpg
Running: swww img /home/user/Pictures/wall.jpg --transition-type any --transition-fps 60 --transition-duration 1
Running: matugen image /home/user/Pictures/wall.jpg --type scheme-tonal-spot
Running: wallust run /home/user/Pictures/wall.jpg -k
[I] image: wall.jpg
[I] image parser: Using FastResize backend parser
[I] threshold: Not defined, using best default thresholds.
[I] colorspace: Using Lch colorspace variation
[I] scheme palette: Using Dark palette
[I] contrast: Doing extra calculations to ensure a good contrast
[I] sequences: Setting terminal colors.
[I] templates: Writing templates..
[I] cache: Saving scheme to cache.

E N J O Y   T H E   P A L E T T E !
Done.
```

---

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

# или

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

## 3. Примеры использования — Подробное использование CLI

### Основное использование с путем к файлу

```bash
# Установит палитру по умолчанию и обои из path/to/image.jpg
wallpaper path/to/image.jpg

# То же, но со светлой палитрой
wallpaper -l path/to/image.jpg

# С явным типом схемы matugen
wallpaper --type scheme-tonal-spot path/to/image.jpg

# Светлая палитра с пользовательским типом схемы
wallpaper -l --type scheme-monochrome path/to/image.jpg
```

### Запуск с GUI для выбора файла

```bash
# Открыть графический выборщик файлов (взаимоисключающе с путем)
wallpaper --gui

# GUI со светлой палитрой
wallpaper -l --gui

# GUI с пользовательским типом схемы
wallpaper --gui --type scheme-expressive
```

### Доступные типы схем matugen

- `scheme-content` (схема по содержанию по умолчанию)
- `scheme-expressive` (выразительные, яркие цвета)
- `scheme-fidelity` (близкие к исходным цветам изображения)
- `scheme-fruit-salad` (игривые, разнообразные цвета)
- `scheme-monochrome` (монохромная схема)
- `scheme-neutral` (приглушенные, нейтральные цвета)
- `scheme-rainbow` (радужные цвета)
- `scheme-tonal-spot` (по умолчанию, сбалансированная тональная схема)

### Случаи ошибок и валидация

```bash
# ОШИБКА: Нельзя использовать --gui и путь одновременно
wallpaper --gui /path/to/image.jpg
# error: the argument '--gui' cannot be used with '[PATH]'

# ОШИБКА: Необходимо указать либо --gui, либо путь
wallpaper
# Error: Either --gui or a path must be provided

# ОШИБКА: Валидация пути
wallpaper /nonexistent/file.jpg
# Error: Path does not exist: /nonexistent/file.jpg
```

### Пример вывода

```text
wallpaper v0.4.1 - fix local and repo mispush - /home/user/Pictures/wall.jpg
Running: swww img /home/user/Pictures/wall.jpg --transition-type any --transition-fps 60 --transition-duration 1
Running: matugen image /home/user/Pictures/wall.jpg --type scheme-tonal-spot
Running: wallust run /home/user/Pictures/wall.jpg -k
[I] image: wall.jpg
[I] image parser: Using FastResize backend parser
[I] threshold: Not defined, using best default thresholds.
[I] colorspace: Using Lch colorspace variation
[I] scheme palette: Using Dark palette
[I] contrast: Doing extra calculations to ensure a good contrast
[I] sequences: Setting terminal colors.
[I] templates: Writing templates..
[I] cache: Saving scheme to cache.

E N J O Y   T H E   P A L E T T E !
Done.
```

---