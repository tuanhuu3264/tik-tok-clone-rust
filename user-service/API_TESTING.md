# Hướng dẫn Test API

## 1. Khởi động Server

### Cách 1: Chạy trực tiếp (PowerShell)
```powershell
# Set environment variables
$env:POSTGRESQL_URL="postgresql://user:password@localhost/userdb"
$env:REDIS_URL="redis://127.0.0.1:6379"
$env:JWT_SECRET="your-secret-key-change-in-production"
$env:SERVER_PORT="3000"

# Chạy server
cargo run
```

### Cách 2: Chạy với default values
```powershell
cargo run
```

Server sẽ chạy tại: **http://localhost:3000**

## 2. Các Endpoint

- **GraphQL API**: `http://localhost:3000/graphql` (POST)
- **GraphiQL Playground**: `http://localhost:3000/graphiql` (GET) - Giao diện test trực quan

## 3. Test bằng GraphiQL Playground (Khuyên dùng)

1. Mở trình duyệt và truy cập: `http://localhost:3000/graphiql`
2. Sử dụng giao diện để test các queries và mutations

## 4. Test bằng cURL

### 4.1. Health Check Query
```bash
curl -X POST http://localhost:3000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "{ health }"
  }'
```

### 4.2. Register Mutation
```bash
curl -X POST http://localhost:3000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "mutation { register(username: \"testuser\", email: \"test@example.com\", password: \"password123\") { success message user { id username email } tokens { access_token refresh_token } } }"
  }'
```

### 4.3. Login Mutation
```bash
curl -X POST http://localhost:3000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "mutation { login(email: \"test@example.com\", password: \"password123\") { success message user { id username email } tokens { access_token refresh_token } } }"
  }'
```

### 4.4. Get Profile Query
```bash
curl -X POST http://localhost:3000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query { getProfile(userId: \"YOUR_USER_ID_HERE\") { user_id display_name bio avatar_url created_at status_id } }"
  }'
```

### 4.5. Logout Mutation
```bash
curl -X POST http://localhost:3000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "mutation { logout(accessToken: \"YOUR_ACCESS_TOKEN_HERE\") { success message } }"
  }'
```

## 5. Test bằng PowerShell (Invoke-RestMethod)

### 5.1. Health Check
```powershell
$body = @{
    query = "{ health }"
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://localhost:3000/graphql" -Method Post -Body $body -ContentType "application/json"
```

### 5.2. Register
```powershell
$body = @{
    query = "mutation { register(username: `"testuser`", email: `"test@example.com`", password: `"password123`") { success message user { id username email } tokens { access_token } } }"
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://localhost:3000/graphql" -Method Post -Body $body -ContentType "application/json"
```

### 5.3. Login
```powershell
$body = @{
    query = "mutation { login(email: `"test@example.com`", password: `"password123`") { success message user { id username email } tokens { access_token refresh_token } } }"
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://localhost:3000/graphql" -Method Post -Body $body -ContentType "application/json"
```

## 6. Test bằng Postman

1. Tạo request mới
2. Method: **POST**
3. URL: `http://localhost:3000/graphql`
4. Headers:
   - `Content-Type: application/json`
5. Body (raw JSON):
```json
{
  "query": "mutation { register(username: \"testuser\", email: \"test@example.com\", password: \"password123\") { success message user { id username email } tokens { access_token } } }"
}
```

## 7. GraphQL Queries/Mutations chi tiết

### 7.1. Health Check
```graphql
query {
  health
}
```

### 7.2. Register
```graphql
mutation {
  register(
    username: "testuser"
    email: "test@example.com"
    password: "password123"
  ) {
    success
    message
    user {
      id
      username
      email
      created_at
      updated_at
    }
    tokens {
      access_token
      refresh_token
      access_token_expires_at
      refresh_token_expires_at
    }
  }
}
```

### 7.3. Login
```graphql
mutation {
  login(
    email: "test@example.com"
    password: "password123"
  ) {
    success
    message
    user {
      id
      username
      email
      created_at
      updated_at
    }
    tokens {
      access_token
      refresh_token
      access_token_expires_at
      refresh_token_expires_at
    }
  }
}
```

### 7.4. Get Profile
```graphql
query {
  getProfile(userId: "YOUR_USER_ID_HERE") {
    user_id
    display_name
    bio
    avatar_url
    created_at
    updated_at
    status_id
  }
}
```

### 7.5. Logout
```graphql
mutation {
  logout(accessToken: "YOUR_ACCESS_TOKEN_HERE") {
    success
    message
  }
}
```

## 8. Lưu ý

1. **Database**: Đảm bảo PostgreSQL và Redis đang chạy
2. **Environment Variables**: Có thể set trong PowerShell hoặc tạo file `.env`
3. **JWT Secret**: Nên đổi `JWT_SECRET` trong production
4. **CORS**: Server đã enable CORS permissive, có thể test từ browser

## 9. Troubleshooting

### Server không start được
- Kiểm tra port 3000 có đang được sử dụng không
- Kiểm tra database connection
- Xem logs trong terminal

### Lỗi connection
- Đảm bảo PostgreSQL đang chạy: `psql -U postgres`
- Đảm bảo Redis đang chạy: `redis-cli ping`

### Lỗi authentication
- Kiểm tra JWT_SECRET đã được set
- Kiểm tra token có hợp lệ không

