# Catalog Service - DDD Architecture

Product catalog service với DDD architecture.

## Domain Entities

- **Product**: Product entity với name, SKU, price
- **Category**: Product categories
- **Inventory**: Product inventory với quantity tracking

## Value Objects

- ProductId, ProductName, SKU, Price, Description
- CategoryId, CategoryName
- Quantity

## Use Cases

- CreateProduct: Tạo product mới
- GetProduct: Lấy thông tin product

## Cấu trúc

Tương tự như user-service với các thư mục domain:
- `domain/entities/` - Product, Category, Inventory
- `domain/value_objects/` - Các value objects
- `domain/repositories/` - Repository traits
- `domain/services/` - CatalogService
- `domain/errors.rs` - Domain errors

