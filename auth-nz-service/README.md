# Auth-NZ Service - DDD Architecture

Authentication & Authorization service với DDD architecture.

## Domain Entities

- **User**: User entity với email và password hash
- **Session**: User session với token
- **Role**: User roles
- **Permission**: Permissions cho resources

## Value Objects

- Email, UserId, PasswordHash
- SessionId, Token
- RoleId, RoleName
- PermissionId, PermissionName

## Use Cases

- Register: Đăng ký user mới
- Login: Đăng nhập
- Logout: Đăng xuất

## Cấu trúc

Tương tự như user-service với các thư mục domain:
- `domain/entities/` - User, Session, Role, Permission
- `domain/value_objects/` - Các value objects
- `domain/repositories/` - Repository traits
- `domain/services/` - AuthService
- `domain/errors.rs` - Domain errors

