# 🚀 Micro-top

**Micro-top** — це крихітний та швидкий монітор системи, написаний на Rust. Працює там, де інші «важковаговики» пасують. Створений для моніторингу Linux та Windows (навіть у віртуальних машинах!).

---

## 🇺🇦 Українська

### Особливості
* 📊 **Візуальні індикатори**: Кольорові прогрес-бари для CPU, RAM та Swap.
* 🛠 **Кросплатформність**: Повна підтримка Linux та Windows 10/11.
* ⚡ **Легкість**: Написано на Rust, мінімальне споживання ресурсів.
* 🖥 **Підтримка заліза**: Визначення моделей CPU та GPU.

### Встановлення та запуск

#### 📦 Windows
Вам не потрібно встановлювати Rust чи Cargo:
1. Перейдіть на сторінку [Releases](https://github.com/KotanUA/Micro-top/releases).
2. Завантажте файл `Micro-top.exe`.
3. Запустіть його в терміналі (PowerShell або CMD).

#### 🐧 Arch Linux (AUR)
Встановіть через ваш улюблений AUR helper:
```bash
yay -S micro-top-git
``` 

#### Інші дистрибутиви Linux (Ubuntu, Fedora, Debian тощо)
Якщо вашого дистрибутива немає в списку, ви можете легко зібрати бінарний файл самостійно:
  1. Встановіть інструмент збірки Rust:
     ``` bash
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
  2. Клонуйте репозиторій та зберіть код
     ``` bash
     git clone https://github.com/KotanUA/Micro-top.git
     cd Micro-top
     cargo build --release sudo cp target/release/micro-top /usr/local/bin/
     ```

  P.S Це моя перша програма :)   

# 🚀 Micro-top

**Micro-top** is a tiny and fast system monitor written in Rust. It works where other "heavyweights" fail. Designed for monitoring Linux and Windows.

---

## 🇺🇸 Features
* 📊 **Visual Indicators**: Colorful progress bars for CPU, RAM, and Swap.
* 🛠 **Cross-platform**: Full support for Linux and Windows 10/11.
* ⚡ **Performance**: Written in Rust with minimal resource consumption.
* 🖥 **Hardware Support**: Detects CPU models and GPU adapters.

## Installation & Build

### 📦 Windows
You don't need to install Rust or Cargo:
1. Go to the [Releases](https://github.com/KotanUA/Micro-top/releases) page.
2. Download the `Micro-top.exe` file.
3. Run it in your terminal (PowerShell or CMD).

### 🐧 Arch Linux (AUR)
Install via your favorite AUR helper:
```bash
yay -S micro-top-git
```
### 🐧 Other Linux Distros (Ubuntu, Fedora, Debian, etc.)
If your distro is not listed, you can easily build the binary yourself:
  1. Install build tools and Rust:
  ``` bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
  2. Clone and build:
     ``` bash
     git clone https://github.com/KotanUA/Micro-top.git
     cd Micro-top
     cargo build --release
     sudo cp target/release/micro-top /usr/local/bin/ 

  P.S This is my first program :) 

  
  
   



