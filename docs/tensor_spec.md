
```markdown
# Tensor Specification

`NgosTensorView` is the ABI tensor descriptor. It contains:

- data pointer
- byte length
- dtype
- layout
- ownership
- device id
- rank
- fixed-size shape array

Rules:

1. `ndim` must be between 1 and 8.
2. `byte_len` must be at least `product(shape) * dtype.byte_width`.
3. Pointer ownership is explicit and must not be inferred.
4. Plugins may not retain borrowed tensor pointers after `execute` returns.
```

