
```markdown
# Python SDK

The SDK uses PyO3.

```python
import ngos

device = ngos.get_device()
tensor = ngos.tensor([1.0, 2.0, 3.0])
result = device.execute(tensor.tolist())
```

Install from `bindings/python`:

```bash
python -m pip install maturin
python -m maturin develop
```
```

