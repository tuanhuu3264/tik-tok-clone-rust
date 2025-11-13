# Các lỗi cần sửa để code có thể chạy được

## 1. Lỗi Module Redis
- **File**: `src/infrastructure/persistence/redis_cache.rs`
- **Vấn đề**: Module `redis` đã tồn tại trong `redis/mod.rs`
- **Giải pháp**: File này đã đúng, chỉ cần đảm bảo `redis/mod.rs` tồn tại

## 2. Lỗi SQLx Query Macros
- **Vấn đề**: `sqlx::query!` cần DATABASE_URL hoặc offline mode
- **Giải pháp**: 
  - Option 1: Set `DATABASE_URL` environment variable
  - Option 2: Sử dụng `sqlx::query` thay vì `sqlx::query!`
  - Option 3: Chạy `cargo sqlx prepare` để tạo offline query cache

## 3. Lỗi Missing DTOs
- **File**: `src/presentation/handlers.rs`
- **Vấn đề**: `CreateUserDto`, `UserDto` không tồn tại
- **Giải pháp**: Tạo DTOs hoặc comment out handlers này

## 4. Lỗi Cassandra Row Access
- **Vấn đề**: `row.columns[0]` trả về `Option<CqlValue>`, không phải `CqlValue`
- **Giải pháp**: Cần unwrap hoặc pattern match

## 5. Lỗi Axum Version Mismatch
- **Vấn đề**: `async-graphql-axum` dùng axum 0.6, nhưng project dùng axum 0.7
- **Giải pháp**: Cần kiểm tra version compatibility hoặc downgrade axum

## 6. Lỗi GraphQL Handler
- **Vấn đề**: Handler signature không match với axum 0.7
- **Giải pháp**: Cần update handler signature

## 7. Lỗi Password Import
- **File**: `src/application/use_cases/auth/register.rs`
- **Vấn đề**: Import `Password` đã đúng nhưng có thể thiếu module export
- **Giải pháp**: Kiểm tra `domain/passwords/entities/mod.rs`

## 8. Lỗi Cassandra Query Params
- **Vấn đề**: `&[&dyn Value]` không implement `SerializeRow`
- **Giải pháp**: Cần convert sang `Vec<Value>` hoặc sử dụng tuple

## 9. Lỗi Send Trait
- **Vấn đề**: Future không Send vì `dyn Value` không Sync
- **Giải pháp**: Cần clone values trước khi await

