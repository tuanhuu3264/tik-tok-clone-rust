# Material Service - DDD Architecture

Material management service với DDD architecture.

## Domain Entities

- **Material**: Material entity với name, code, unit
- **MaterialType**: Material types
- **Supplier**: Material suppliers

## Value Objects

- MaterialId, MaterialName, MaterialCode, Unit
- MaterialTypeId, MaterialTypeName
- SupplierId, SupplierName, Email

## Use Cases

- CreateMaterial: Tạo material mới
- GetMaterial: Lấy thông tin material

## Cấu trúc

Tương tự như user-service với các thư mục domain:
- `domain/entities/` - Material, MaterialType, Supplier
- `domain/value_objects/` - Các value objects
- `domain/repositories/` - Repository traits
- `domain/services/` - MaterialService
- `domain/errors.rs` - Domain errors

