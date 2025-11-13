# Dependency Injection (DI) - Tổng hợp

## Tổng quan

Module `di` (Dependency Injection) quản lý tất cả dependencies của application, đảm bảo:
- **Single Source of Truth**: Tất cả dependencies được khởi tạo ở một nơi
- **Loose Coupling**: Components không phụ thuộc trực tiếp vào implementations
- **Testability**: Dễ dàng mock dependencies cho testing
- **Thread Safety**: Sử dụng `Arc` để chia sẻ dependencies an toàn giữa threads

## Cấu trúc

```
di/
└── mod.rs          # AppContext - DI Container
```

## AppContext - DI Container

### Định nghĩa

```rust
#[derive(Clone)]
pub struct AppContext {
    pub user_repository: Arc<dyn UserRepository>,
    pub profile_repository: Arc<dyn ProfileRepository>,
    pub redis_cache: Arc<RedisCache>,
}
```

### Dependencies được quản lý

1. **UserRepository** - Repository cho User entity
   - Implementation: `UserRepositoryImpl`
   - Sử dụng: PostgreSQL (Commands) + Cassandra (Queries)
   
2. **ProfileRepository** - Repository cho Profile entity
   - Implementation: `PostgresProfileRepository`
   - Sử dụng: PostgreSQL

3. **RedisCache** - Cache layer
   - Implementation: `RedisCache`
   - Hỗ trợ: Single instance và Cluster mode

## Lifecycle

### 1. Khởi tạo (Initialization)

```rust
impl AppContext {
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error>> {
        // 1. Tạo PostgreSQL connection pool
        let postgres_pool = create_postgresql_pool(
            config.postgresql.as_ref().ok_or("PostgreSQL config not found")?
        ).await?;

        // 2. Tạo Cassandra session
        let cassandra_session = create_cassandra_session(
            config.cassandra.as_ref().ok_or("Cassandra config not found")?
        ).await?;

        // 3. Tạo Redis cache
        let redis_config = config.redis.as_ref().ok_or("Redis config not found")?;
        let redis_cache = Arc::new(create_redis_cache(redis_config).await?);

        // 4. Tạo UserRepository với CQRS pattern
        let user_repository: Arc<dyn UserRepository> = Arc::new(
            UserRepositoryImpl::new(postgres_pool.clone(), cassandra_session.clone())
        );

        // 5. Tạo ProfileRepository
        let profile_repository: Arc<dyn ProfileRepository> = Arc::new(
            PostgresProfileRepository::new(postgres_pool)
        );

        Ok(Self {
            user_repository,
            profile_repository,
            redis_cache,
        })
    }
}
```

### 2. Sử dụng trong Server

```rust
// src/presentation/server.rs
pub async fn create_server(config: Config) -> Result<Server, ...> {
    // Khởi tạo AppContext với tất cả dependencies
    let context = Arc::new(AppContext::new(config.clone()).await?);
    
    // Inject vào router
    let app = create_router(context).await?;
    // ...
}
```

### 3. Sử dụng trong GraphQL

```rust
// src/presentation/graphql/schema.rs
pub fn create_schema(context: AppContext) -> AppSchema {
    Schema::build(Query, Mutation, EmptySubscription)
        .data(context)  // Inject AppContext vào GraphQL context
        .finish()
}

// src/presentation/graphql/resolvers.rs
async fn register(&self, ctx: &Context<'_>, ...) -> GraphQLResult<...> {
    // Lấy AppContext từ GraphQL context
    let context = ctx.data::<AppContext>()?;
    
    // Sử dụng dependencies
    let register_use_case = RegisterUseCase::new(
        context.user_repository.clone(),
        context.profile_repository.clone(),
    );
    // ...
}
```

## Dependency Graph

```
AppContext
├── UserRepository (trait)
│   └── UserRepositoryImpl
│       ├── PostgreSQLUserCommands (PostgreSQLPool)
│       └── CassandraUserQueries (CassandraSession)
│
├── ProfileRepository (trait)
│   └── PostgresProfileRepository (PostgreSQLPool)
│
└── RedisCache
    └── RedisCache (RedisConfig)
```

## Các Pattern được sử dụng

### 1. **Repository Pattern**
- Domain layer định nghĩa traits (interfaces)
- Infrastructure layer implement traits
- DI container inject implementations

### 2. **CQRS Pattern** (cho UserRepository)
- **Commands** → PostgreSQL (Write operations)
- **Queries** → Cassandra (Read operations)
- `UserRepositoryImpl` kết hợp cả hai

### 3. **Dependency Inversion Principle**
- High-level modules (Use Cases) không phụ thuộc vào low-level modules (Repositories)
- Cả hai phụ thuộc vào abstractions (traits)

### 4. **Singleton Pattern** (với Arc)
- Mỗi dependency chỉ được tạo một lần
- Chia sẻ an toàn giữa nhiều threads

## Thread Safety

Tất cả dependencies được wrap trong `Arc` để:
- **Share ownership**: Nhiều owners có thể sử dụng cùng một instance
- **Thread safe**: Có thể chia sẻ giữa async tasks
- **Clone cheap**: `Arc::clone()` chỉ tăng reference count

```rust
pub user_repository: Arc<dyn UserRepository>,  // Thread-safe
pub profile_repository: Arc<dyn ProfileRepository>,  // Thread-safe
pub redis_cache: Arc<RedisCache>,  // Thread-safe
```

## Sử dụng trong Use Cases

### Register Use Case
```rust
let register_use_case = RegisterUseCase::new(
    context.user_repository.clone(),      // Clone Arc (cheap)
    context.profile_repository.clone(),   // Clone Arc (cheap)
);
```

### Login Use Case
```rust
let login_use_case = LoginUseCase::new(
    context.user_repository.clone(),
);
```

### Logout Use Case
```rust
let logout_use_case = LogoutUseCase::new(
    context.redis_cache.clone(),
);
```

### Get Profile Use Case
```rust
let get_profile_use_case = GetProfileUseCase::new(
    context.profile_repository.clone(),
);
```

## Lợi ích

### 1. **Maintainability**
- Dễ dàng thay đổi implementations
- Tập trung quản lý dependencies ở một nơi

### 2. **Testability**
- Có thể inject mock implementations
- Dễ dàng test từng component riêng biệt

### 3. **Flexibility**
- Có thể thay đổi database implementations
- Có thể thêm/cập nhật dependencies mà không ảnh hưởng code khác

### 4. **Type Safety**
- Compile-time checking
- Không có runtime errors về missing dependencies

## Mở rộng

Để thêm dependency mới:

1. **Thêm vào AppContext:**
```rust
pub struct AppContext {
    // ... existing dependencies
    pub new_service: Arc<dyn NewService>,
}
```

2. **Khởi tạo trong `new()`:**
```rust
let new_service = Arc::new(NewServiceImpl::new(...));
```

3. **Sử dụng trong resolvers:**
```rust
let context = ctx.data::<AppContext>()?;
let use_case = SomeUseCase::new(context.new_service.clone());
```

## Best Practices

1. ✅ **Luôn sử dụng `Arc`** cho shared dependencies
2. ✅ **Clone Arc thay vì clone struct** (cheap operation)
3. ✅ **Sử dụng traits** cho abstractions
4. ✅ **Khởi tạo tất cả dependencies** trong `AppContext::new()`
5. ✅ **Error handling** khi khởi tạo dependencies
6. ✅ **Thread safety** - đảm bảo tất cả dependencies là Send + Sync

## Ví dụ hoàn chỉnh

```rust
// 1. Khởi tạo
let context = Arc::new(AppContext::new(config).await?);

// 2. Inject vào GraphQL
let schema = create_schema((*context).clone());

// 3. Sử dụng trong resolver
async fn some_mutation(ctx: &Context<'_>) -> Result<...> {
    let context = ctx.data::<AppContext>()?;
    let use_case = SomeUseCase::new(
        context.user_repository.clone(),
    );
    use_case.execute().await
}
```

