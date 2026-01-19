## 📋 Фаза 1: Минимальный рабочий продукт (MVP)

### Цель:
**Сгенерировать из Rust работающий JavaScript, который можно подключить в HTML страницу и использовать.**

---

## 🎯 Требования MVP

### 1. **Поддерживаемые типы Rust:**
```rust
// Примитивы
i32    → number
f64    → number  
bool   → boolean
String → string

// Композиции
[T; N]       → Array<T> (фиксированный размер в Rust, обычный массив в JS)
Vec<T>       → Array<T>
struct S { } → interface S { }
```

### 2. **Структуры (только данные):**
```rust
// Rust
#[rust2ts]
struct Point {
    x: i32,
    y: i32,
    name: String,
    active: bool,
}

// → TypeScript
interface Point {
    x: number;
    y: number; 
    name: string;
    active: boolean;
}
```

### 3. **Функции (без замыканий, без async):**
```rust
#[rust2ts]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[rust2ts] 
fn process_points(points: Vec<Point>) -> i32 {
    let mut sum = 0;
    for point in points {
        sum += point.x;
    }
    sum
}
```

### 4. **Базовый контроль потока:**
- `if` / `else`
- `for` loops (только с индексами: `for i in 0..n`)
- `match` простые (без сложных паттернов)

---

## 🚫 Что НЕ поддерживаем в MVP:

1. **❌ Ссылки** (`&`, `&mut`)
2. **❌ Option/Result** (пока используем nullable в JS)
3. **❌ Итераторы** (только индексированные циклы)
4. **❌ Методы** (только free functions)
5. **❌ Traits, generics**
6. **❌ Модули/видимость** (все публичное)
7. **❌ i64/u64**
8. **❌ Указатели/unsafe**

---

## 🛠️ Архитектура проекта

### Структура:
```
rust2ts/
├── Cargo.toml
├── src/
│   ├── lib.rs              # Процедурный макрос
│   ├── generator.rs        # Генерация TypeScript
│   └── validator.rs        # Проверка совместимости
├── example/
│   └── chess/              # Пример шахматного движка
└── tests/
```

### Макрос `#[rust2ts]`:
```rust
// Пример использования
use rust2ts::rust2ts;

#[rust2ts]
pub mod chess {
    #[rust2ts]
    pub struct Board {
        squares: [[i32; 8]; 8],
    }
    
    #[rust2ts]
    pub fn evaluate(board: Board) -> i32 {
        let mut score = 0;
        for i in 0..8 {
            for j in 0..8 {
                score += board.squares[i][j];
            }
        }
        score
    }
}
```

---

## 🔄 Генерация вывода

### Команда:
```bash
cargo build --features rust2ts
# Автоматически генерирует:
# - target/rust2ts/generated.ts  # TypeScript типы
# - target/rust2ts/generated.js  # JavaScript код
```

### Пример вывода:
```typescript
// generated.ts
export interface Board {
    squares: number[][];
}

export function evaluate(board: Board): number {
    let score = 0;
    for (let i = 0; i < 8; i++) {
        for (let j = 0; j < 8; j++) {
            score += board.squares[i][j];
        }
    }
    return score;
}
```

```javascript
// generated.js (тот же код, без типов)
export function evaluate(board) {
    let score = 0;
    for (let i = 0; i < 8; i++) {
        for (let j = 0; j < 8; j++) {
            score += board.squares[i][j];
        }
    }
    return score;
}
```

---

## 🌐 Интеграция с DOM/браузером

### Подход: **Ручные биндинги**

#### Шаг 1: Rust функции для логики
```rust
#[rust2ts]
pub fn calculate_game_state(board: Board, move: Move) -> GameState {
    // Чистая логика, без DOM
    // Только вычисления
}
```

#### Шаг 2: JavaScript для DOM
```javascript
// manual-bindings.js - пишем вручную
import { calculate_game_state } from './generated.js';

export function updateGameUI(boardElement, move) {
    // 1. Получаем данные из DOM
    const board = readBoardFromDOM(boardElement);
    
    // 2. Вызываем Rust логику
    const newState = calculate_game_state(board, move);
    
    // 3. Обновляем DOM
    updateDOMWithState(boardElement, newState);
}

function readBoardFromDOM(element) {
    // Ручное чтение DOM
    return {
        squares: /* ... */
    };
}
```

### Альтернатива: **Автоматические DOM биндинги (фаза 2)**

```rust
// В будущем можно будет добавить:
#[rust2ts(dom_binding = "getElementById")]
pub fn get_element_value(id: String) -> String {
    // Специальные функции, которые компилятор знает
    // как преобразовать в DOM API
}

// → Генерирует:
function get_element_value(id) {
    return document.getElementById(id).value;
}
```

---

## 📦 Публичный API библиотеки

### Cargo.toml:
```toml
[package]
name = "rust2ts"
version = "0.1.0"

[lib]
proc-macro = true

[dependencies]
syn = { version = "2.0", features = ["full", "extra-traits"] }
quote = "1.0"
proc-macro2 = "1.0"

[features]
default = []
rust2ts = []  # Включает генерацию TypeScript
```

### Использование:
```toml
# В проекте пользователя
[dependencies]
rust2ts = { path = "../rust2ts", features = ["rust2ts"] }
```

```rust
// user's src/lib.rs
use rust2ts::rust2ts;

#[rust2ts]
pub mod my_logic {
    // ...
}
```

---

## 🧪 Тестовый пример: шахматный движок

### `example/chess/src/lib.rs`:
```rust
use rust2ts::rust2ts;

#[rust2ts]
pub mod chess {
    #[rust2ts]
    #[derive(Clone, Copy)]
    pub struct Board {
        // 0 = empty, 1-6 = white pieces, -1..-6 = black pieces
        squares: [[i32; 8]; 8],
        turn: i32, // 0 = white, 1 = black
    }
    
    #[rust2ts]
    pub const PIECE_PAWN: i32 = 1;
    pub const PIECE_KNIGHT: i32 = 2;
    pub const PIECE_BISHOP: i32 = 3;
    pub const PIECE_ROOK: i32 = 4;
    pub const PIECE_QUEEN: i32 = 5;
    pub const PIECE_KING: i32 = 6;
    
    #[rust2ts]
    pub fn evaluate_material(board: Board) -> i32 {
        let mut score = 0;
        for i in 0..8 {
            for j in 0..8 {
                let piece = board.squares[i][j];
                if piece != 0 {
                    score += piece_value(piece);
                }
            }
        }
        score
    }
    
    fn piece_value(piece: i32) -> i32 {
        match piece.abs() {
            PIECE_PAWN => 1,
            PIECE_KNIGHT => 3,
            PIECE_BISHOP => 3,
            PIECE_ROOK => 5,
            PIECE_QUEEN => 9,
            PIECE_KING => 0, // король бесценен
            _ => 0,
        }
    }
    
    #[rust2ts]
    pub fn create_initial_board() -> Board {
        Board {
            squares: initial_position(),
            turn: 0,
        }
    }
    
    fn initial_position() -> [[i32; 8]; 8] {
        // Начальная позиция в шахматах
        [
            [-4, -2, -3, -5, -6, -3, -2, -4],
            [-1, -1, -1, -1, -1, -1, -1, -1],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 1, 1, 1, 1, 1, 1, 1],
            [4, 2, 3, 5, 6, 3, 2, 4],
        ]
    }
}
```

### Генерируемый TypeScript:
```typescript
// target/rust2ts/chess.ts
export interface Board {
    squares: number[][];
    turn: number;
}

export const PIECE_PAWN = 1;
export const PIECE_KNIGHT = 2;
export const PIECE_BISHOP = 3;
export const PIECE_ROOK = 4;
export const PIECE_QUEEN = 5;
export const PIECE_KING = 6;

export function evaluate_material(board: Board): number {
    let score = 0;
    for (let i = 0; i < 8; i++) {
        for (let j = 0; j < 8; j++) {
            const piece = board.squares[i][j];
            if (piece !== 0) {
                score += piece_value(piece);
            }
        }
    }
    return score;
}

function piece_value(piece: number): number {
    const absPiece = Math.abs(piece);
    switch (absPiece) {
        case PIECE_PAWN: return 1;
        case PIECE_KNIGHT: return 3;
        case PIECE_BISHOP: return 3;
        case PIECE_ROOK: return 5;
        case PIECE_QUEEN: return 9;
        case PIECE_KING: return 0;
        default: return 0;
    }
}

export function create_initial_board(): Board {
    return {
        squares: initial_position(),
        turn: 0,
    };
}

function initial_position(): number[][] {
    return [
        [-4, -2, -3, -5, -6, -3, -2, -4],
        [-1, -1, -1, -1, -1, -1, -1, -1],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 1, 1, 1, 1, 1, 1, 1],
        [4, 2, 3, 5, 6, 3, 2, 4],
    ];
}
```

---

## 🌐 Пример HTML страницы

### `example/chess/www/index.html`:
```html
<!DOCTYPE html>
<html>
<head>
    <title>Chess Engine Test</title>
    <style>
        .board { display: grid; grid-template-columns: repeat(8, 50px); }
        .square { width: 50px; height: 50px; border: 1px solid black; }
        .white { background: #eee; }
        .black { background: #888; }
        .piece { font-size: 40px; text-align: center; }
    </style>
</head>
<body>
    <h1>Chess Engine on rust2ts</h1>
    
    <div id="board" class="board"></div>
    
    <div>
        <p>Material score: <span id="score">0</span></p>
        <button onclick="updateScore()">Evaluate Position</button>
    </div>
    
    <script type="module">
        import { create_initial_board, evaluate_material } from './chess.js';
        
        const board = create_initial_board();
        
        function updateScore() {
            const score = evaluate_material(board);
            document.getElementById('score').textContent = score;
        }
        
        // Простой рендеринг доски
        function renderBoard() {
            const boardElement = document.getElementById('board');
            boardElement.innerHTML = '';
            
            for (let i = 0; i < 8; i++) {
                for (let j = 0; j < 8; j++) {
                    const square = document.createElement('div');
                    square.className = `square ${(i + j) % 2 === 0 ? 'white' : 'black'}`;
                    
                    const piece = board.squares[i][j];
                    if (piece !== 0) {
                        const pieceElement = document.createElement('div');
                        pieceElement.className = 'piece';
                        pieceElement.textContent = getPieceSymbol(piece);
                        square.appendChild(pieceElement);
                    }
                    
                    boardElement.appendChild(square);
                }
            }
        }
        
        function getPieceSymbol(piece) {
            const symbols = {
                1: '♙', -1: '♟',
                2: '♘', -2: '♞',
                3: '♗', -3: '♝',
                4: '♖', -4: '♜',
                5: '♕', -5: '♛',
                6: '♔', -6: '♚',
            };
            return symbols[piece] || '';
        }
        
        renderBoard();
        updateScore();
    </script>
</body>
</html>
```

---

## 📝 План реализации по неделям

### **Неделя 1: Ядро**
- [ ] Проектная структура
- [ ] Процедурный макрос `#[rust2ts]` (заглушка)
- [ ] Парсинг структур и функций
- [ ] Генерация TypeScript interfaces

### **Неделя 2: Типы и функции**
- [ ] Поддержка примитивных типов (i32, f64, bool, String)
- [ ] Поддержка массивов и Vec
- [ ] Генерация простых функций
- [ ] Базовые тесты

### **Неделя 3: Контроль потока**
- [ ] Преобразование `if`/`else`
- [ ] Преобразование `for` циклов (индексированных)
- [ ] Простые `match` выражения
- [ ] Пример с шахматным движком

### **Неделя 4: Интеграция**
- [ ] Автоматическая генерация при сборке
- [ ] Пример с HTML страницей
- [ ] Документация
- [ ] Публикация на crates.io

---

## 🎯 Критерии успеха MVP

1. **✅ Компилируется**: `cargo build` работает
2. **✅ Генерирует TypeScript**: Создает .ts файлы
3. **✅ Генерирует JavaScript**: Создает .js файлы  
4. **✅ Работает в браузере**: Можно подключить в HTML
5. **✅ Пример работает**: Шахматный движок показывает оценку позиции

---

## 🚀 Первые шаги прямо сейчас:

### 1. Создать проект:
```bash
cargo new rust2ts --lib
cd rust2ts
```

### 2. Настроить Cargo.toml:
```toml
[lib]
proc-macro = true

[dependencies]
syn = { version = "2.0", features = ["full", "extra-traits"] }
quote = "1.0"
proc-macro2 = "1.0"
```

### 3. Написать макрос-заглушку:
```rust
// src/lib.rs
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn rust2ts(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Пока просто возвращаем оригинальный код
    item
}
```

---

## 💡 Философия разработки:

1. **Сначала работает, потом оптимизируем**
2. **Один feature за раз**
3. **Тесты с самого начала**
4. **Примеры — лучшая документация**
