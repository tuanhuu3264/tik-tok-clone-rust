# Câu lệnh chạy User Service

## 1. Kiểm tra code (không compile)
```bash
cargo check
```

## 2. Build project
```bash
cargo build
```

## 3. Build release
```bash
cargo build --release
```

## 4. Chạy project
```bash
cargo run
```

## 5. Chạy với environment variables
```bash
# Windows PowerShell
$env:DATABASE_URL="postgresql://user:password@localhost/userdb"
$env:POSTGRESQL_URL="postgresql://user:password@localhost/userdb"
$env:CASSANDRA_URL="127.0.0.1:9042"
$env:REDIS_URL="redis://127.0.0.1:6379"
$env:JWT_SECRET="your-secret-key"
$env:SERVER_HOST="0.0.0.0"
$env:SERVER_PORT="3000"
cargo run

# Linux/Mac
export DATABASE_URL="postgresql://user:password@localhost/userdb"
export POSTGRESQL_URL="postgresql://user:password@localhost/userdb"
export CASSANDRA_URL="127.0.0.1:9042"
export REDIS_URL="redis://127.0.0.1:6379"
export JWT_SECRET="your-secret-key"
export SERVER_HOST="0.0.0.0"
export SERVER_PORT="3000"
cargo run
```

## 6. Chạy với .env file (nếu có)
```bash
# Cần thêm dotenv crate vào Cargo.toml
cargo run
```

## 7. Chạy tests
```bash
cargo test
```

## 8. Format code
```bash
cargo fmt
```

## 9. Lint code
```bash
cargo clippy
```

## 10. Clean build
```bash
cargo clean
cargo build
```

## Lưu ý quan trọng:

⚠️ **Hiện tại project có 49 lỗi compile, cần sửa trước khi chạy được!**

Các lỗi chính:
- SQLx query macros cần DATABASE_URL
- Cassandra row access cần sửa
- Missing DTOs
- Axum version compatibility

Sau khi sửa lỗi, các endpoint sẽ có tại:
- GraphQL: http://localhost:3000/graphql
- GraphiQL Playground: http://localhost:3000/graphiql

