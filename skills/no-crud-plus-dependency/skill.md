---
description: Remove crud-plus/crud-core dependency and replace with plat-common
---

# Remove crud-plus Dependency Skill

Remove `com.jfeat:crud-plus` dependency from Spring Boot projects and replace with `com.xinzhi.plat:plat-common`.

## Background

The `crud-plus` library provides common CRUD utilities but creates tight coupling. The `plat-common` library provides equivalent utilities with better maintainability.

## Common Replacements

| crud-plus | plat-common |
|-----------|-------------|
| `com.jfeat.crud.plus.service.dao.Query` | Use MyBatis-Plus directly |
| `com.jfeat.crud.plus.util.TreeUtil` | `com.xinzhi.plat.common.util.TreeUtil` |
| `com.jfeat.crud.plus.util.MiscUtils` | `com.xinzhi.plat.common.util.MiscUtil` |

## Step-by-Step Removal

### 1. Remove Dependency from pom.xml

Remove the dependency and version property:

```xml
<!-- Remove this property -->
<crud-plus.version>21.0.0</crud-plus.version>

<!-- Remove this dependency -->
<dependency>
    <groupId>com.jfeat</groupId>
    <artifactId>crud-plus</artifactId>
    <version>${crud-plus.version}</version>
</dependency>
```

### 2. Add plat-common Dependency

```xml
<dependency>
    <groupId>com.xinzhi.plat</groupId>
    <artifactId>plat-common</artifactId>
</dependency>
```

### 3. Update Imports

Find and replace imports:

```bash
# Find all crud-plus imports
grep -r "import.*jfeat\.crud" src/

# Replace imports based on your usage
sed -i 's/import com\.jfeat\.crud\.plus\.util\.TreeUtil/import com.xinzhi.plat.common.util.TreeUtil/g' $(find src -name "*.java")
```

### 4. Update Documentation

Update README.md and other documentation to reference plat-common instead of crud-plus.

### 5. Update @MapperScan

Remove crud-plus DAO package from @MapperScan:

```java
@MapperScan({
    "com.jfeat.**.persistence.dao",
    "com.jfeat.**.domain.dao"
    // Remove: "com.jfeat.crud.plus.service.dao"
})
```

## Verification

```bash
# 1. Check dependency tree (most important - verify no crud-plus dependency)
mvn dependency:tree | grep crud-plus
# Expected: no output (crud-plus should not appear)

# 2. Check for remaining crud-plus imports in source code
grep -r "jfeat\.crud" src/ || echo "All crud-plus imports removed"

# 3. Build project to verify no compilation errors
mvn clean compile

# 4. Full package build
mvn clean package -DskipTests
```

## Common Issues

### Issue: plat-common class not found

**Error**: `package com.xinzhi.plat.common.util does not exist`

**Solution**: Verify plat-common dependency is added and version is compatible with plat-parent.

### Issue: Method signature mismatch

**Error**: Method has different signature than expected

**Solution**: Check plat-common API documentation for the correct method signatures.

### Issue: @MapperScan references crud-plus

**Error**: Bean not found for crud-plus DAO

**Solution**: Remove `"com.jfeat.crud.plus.service.dao"` from @MapperScan annotation.

## Example Migration

### Before (with crud-plus)

```xml
<properties>
    <crud-plus.version>21.0.0</crud-plus.version>
</properties>

<dependency>
    <groupId>com.jfeat</groupId>
    <artifactId>crud-plus</artifactId>
    <version>${crud-plus.version}</version>
</dependency>
```

### After (with plat-common)

```xml
<dependency>
    <groupId>com.xinzhi.plat</groupId>
    <artifactId>plat-common</artifactId>
</dependency>
```

## Related Files

- `pom.xml` - Maven dependencies
- `src/main/java/com/jfeat/AmApplication.java` - @MapperScan configuration
- `README.md` - Documentation
