
import ngos

device = ngos.get_device()
tensor = ngos.tensor([1.0, 2.0, 3.0])
result = device.execute(tensor.tolist())

print({"device": device.name, "result": result})

