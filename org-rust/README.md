# org-rust

参考 `eav/eav-rust` 的 `axum + sqlx` 分层风格重建的组织服务脚手架，接口与原 Java `auth/org` 的核心路由保持一致。

## 运行

1. 复制环境变量：

```bash
cp .env.example .env
```

2. 启动：

```bash
cargo run
```

## 核心接口

- `GET /health`
- `POST /api/adm/org/:id/children`
- `DELETE /api/adm/org/:id`
- `PUT /api/adm/org/:id`
- `GET /api/adm/org/:id`
- `GET /api/adm/org`
- `GET /api/adm/org/tree`
- `POST /api/adm/sys/extOrg/add`
- `DELETE /api/adm/sys/extOrg/delete?id=...`
- `PUT /api/adm/sys/extOrg/update`
- `GET /api/adm/sys/extOrg/list`
- `POST /api/adm/sys/extOrg/sync`

## 鉴权上下文

当前用请求头透传 JWT 上下文（用于替代 Java 版 `JWTKit`）：

- `x-org-id`
- `x-tenant-org-id`
- `x-appid`

