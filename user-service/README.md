# User Service - DDD Architecture với Rust

## Cấu trúc DDD (Domain-Driven Design)

Dự án này được tổ chức theo kiến trúc DDD với 4 layers chính:

```
user-service/
├── src/
│   ├── domain/              # Domain Layer - Core business logic
│   │   ├── entities/        # Domain entities (User, etc.)
│   │   ├── value_objects/   # Value objects (Email, Username, UserId)
│   │   ├── repositories/    # Repository traits (interfaces)
│   │   ├── services/        # Domain services
│   │   └── errors.rs        # Domain errors
│   │
│   ├── application/         # Application Layer - Use cases
│   │   ├── use_cases/       # Application use cases
│   │   ├── dto/             # Data Transfer Objects
│   │   └── errors.rs        # Application errors
│   │
│   ├── infrastructure/      # Infrastructure Layer - External concerns
│   │   ├── config/          # Configuration
│   │   ├── persistence/     # Database, cache connections
│   │   └── repositories/    # Repository implementations
│   │
│   ├── presentation/        # Presentation Layer - HTTP/API
│   │   ├── handlers/        # HTTP handlers
│   │   ├── routes/          # API routes
│   │   └── server.rs        # Server setup
│   │
│   └── main.rs              # Application entry point
│
└── Cargo.toml
```

## Các thư mục Domain cần có

### 1. **domain/entities/**
Chứa các domain entities - đối tượng nghiệp vụ chính:
- `user.rs` - User entity với business logic
- Entities có identity và lifecycle
- Chứa business rules và validation logic

### 2. **domain/value_objects/**
Chứa các value objects - đối tượng không có identity:
- `email.rs` - Email value object với validation
- `username.rs` - Username value object
- `user_id.rs` - UserId value object
- Value objects là immutable và tự validate

### 3. **domain/repositories/**
Chứa repository traits (interfaces):
- `user_repository.rs` - Trait định nghĩa các operations
- Không có implementation ở đây (implementation ở infrastructure)
- Định nghĩa contract cho persistence

### 4. **domain/services/**
Chứa domain services - logic không thuộc về một entity cụ thể:
- `user_service.rs` - Business logic phức tạp
- Xử lý các business rules liên quan đến nhiều entities

### 5. **domain/errors.rs**
Domain-specific errors:
- `DomainError` enum
- Các lỗi nghiệp vụ cụ thể

## Dependency Rules

```
Presentation → Application → Domain
Infrastructure → Domain
Application → Infrastructure (chỉ để inject repositories)
```

**Quan trọng:**
- Domain layer KHÔNG phụ thuộc vào bất kỳ layer nào khác
- Application layer phụ thuộc vào Domain
- Infrastructure implement Domain interfaces
- Presentation gọi Application use cases

## Cách sử dụng

1. **Tạo migration database:**
```sql
CREATE TABLE users (
    id UUID PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);
```

2. **Chạy service:**
```bash
cargo run
```

3. **Environment variables:**
```bash
export DATABASE_URL="postgresql://user:password@localhost/userdb"
export SERVER_HOST="0.0.0.0"
export SERVER_PORT="3000"
```

## Best Practices

1. **Domain Layer:**
   - Không có external dependencies
   - Pure business logic
   - Entities và Value Objects tự validate

2. **Application Layer:**
   - Orchestrate use cases
   - Transform between DTOs và Domain objects
   - Handle application-level errors

3. **Infrastructure Layer:**
   - Implement Domain interfaces
   - Handle external concerns (DB, cache, messaging)
   - Configuration management

4. **Presentation Layer:**
   - HTTP handlers
   - Request/Response transformation
   - Error handling và status codes

