# Power Table

Простая консольная программа на Rust для вывода таблицы степеней.

## Установка

Поддерживается только Linux.

Скачайте файл из раздела [Releases](https://github.com/sqeezy-53/power-table/releases).

Выдайте разрешение на запуск:

```bash
chmod +x power-table
```

Запустите программу:

```bash
./power-table
```

## Сборка из исходного кода

Для сборки необходим Rust и Cargo.

```bash
git clone https://github.com/sqeezy-53/power-table.git
cd power-table
cargo build --release
./target/release/power-table
```

## Возможности

- Таблица степеней чисел от 2 до 10.
- Степени от 1 до 10.
- Автоматическое выравнивание столбцов.
- Настройка диапазонов через константы.

## Лицензия

Проект распространяется под лицензией [MIT](LICENSE).
