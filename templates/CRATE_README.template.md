# <module-name>

> Copy this template to `<module-path>/README.md` and fill in all sections.

---

## Purpose

<One paragraph explaining what this module does and why it exists. What problem does it solve? Who uses it?>

---

## Modules

| Module | Description |
|--------|-------------|
| `<module_name>` | <One-line description> |
| `<module_name>` | <One-line description> |
| `<module_name>` | <One-line description> |

---

## Public API

### Key Types

```
// Brief description
pub struct <TypeName> {
    // ...
}
```

### Key Functions

```
// Brief description
//
// Arguments:
// * `param` - <description>
//
// Returns: <description>
//
// Errors: <when and why this fails>
pub fn <function_name>(param: Type) -> Result<Output, Error> { ... }
```

### Key Traits

```
// Brief description of the trait's purpose
pub trait <TraitName> {
    fn <method>(&self) -> Type;
}
```

---

## Usage

```
// Example usage
let instance = TypeName::new(config);
let result = function_name(&instance)?;
```

---

## Tests

```bash
# Run all tests for this module
<test command>

# Run a specific test
<specific test command>

# Run with output
<verbose test command>
```

### Test Coverage

| Area | Tests | Notes |
|------|-------|-------|
| <area> | <count> | <notes> |
| <area> | <count> | <notes> |

---

## Dependencies

| Dependency | Why |
|------------|-----|
| `<dep>` | <reason this dependency is used> |
| `<dep>` | <reason this dependency is used> |

### Internal Dependencies

| Module | Relationship |
|--------|-------------|
| `<module>` | <how this module uses the dependency> |

---

## Configuration

<Any configuration options, environment variables, or feature flags that affect this module's behavior.>

---

## Architecture Notes

<Brief description of the module's internal architecture, key design decisions, or patterns used. Link to ADRs if applicable.>
