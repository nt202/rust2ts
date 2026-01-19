#!/bin/bash
# scripts/check_generation.sh
set -e

echo "=== Checking rust2ts Generation ==="

# Чистим предыдущие сборки
echo "Cleaning..."
cargo clean

# Собираем
echo "Building..."
cargo build --release

# Запускаем тесты
echo "Running tests..."
cargo test --release

# Ищем сгенерированные файлы
echo ""
echo "Looking for generated files..."

OUT_DIR=$(find target -name "rust2ts" -type d | head -1)
if [ -n "$OUT_DIR" ]; then
    echo "Found output directory: $OUT_DIR"
    
    if [ -f "$OUT_DIR/generated.ts" ]; then
        echo ""
        echo "=== Generated TypeScript ==="
        cat "$OUT_DIR/generated.ts"
        echo "=== End of TypeScript ==="
    else
        echo "No generated.ts found"
    fi
    
    if [ -f "$OUT_DIR/INFO.txt" ]; then
        echo ""
        cat "$OUT_DIR/INFO.txt"
    fi
else
    echo "No output directory found"
fi

echo ""
echo "=== Check Complete ==="
