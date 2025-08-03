#!/usr/bin/env python3
# -*- coding: utf-8 -*-
# 训练模型 → 导出ONNX → 加载ONNX → 验证推理
# DCN实现：包含交叉网络和深度网络，确保模型保存和加载的结构一致
import torch
import torch.nn as nn
import onnxruntime as ort
import numpy as np

class DCN(nn.Module):
    def __init__(self, input_dim, cross_layers=3, hidden_units=[64, 32]):
        super(DCN, self).__init__()
        self.input_dim = input_dim
        self.cross_layers = cross_layers
        
        # 交叉网络部分
        self.cross_net = nn.ModuleList([
            nn.Linear(input_dim, input_dim) for _ in range(cross_layers)
        ])
        
        # 深度网络部分
        deep_layers = []
        units = [input_dim] + hidden_units
        for in_dim, out_dim in zip(units[:-1], units[1:]):
            deep_layers.extend([nn.Linear(in_dim, out_dim), nn.ReLU()])
        self.deep_net = nn.Sequential(*deep_layers)
        
        # 输出层
        self.output_layer = nn.Linear(input_dim + hidden_units[-1], 1)
        
    def forward(self, x):
        # 交叉网络
        x0 = x.clone()
        for layer in self.cross_net:
            x = x0 * layer(x) + x
        
        # 深度网络
        deep_out = self.deep_net(x0)
        
        # 组合输出
        combined = torch.cat([x, deep_out], dim=1)
        return self.output_layer(combined)

# 1. 创建并训练模型
model = DCN(input_dim=3, cross_layers=3, hidden_units=[64, 32])
optimizer = torch.optim.Adam(model.parameters(), lr=0.001)
print(model)  # 打印模型结构

# 训练循环（示例）
for epoch in range(100):
    inputs = torch.randn(10, 3)
    targets = torch.randn(10, 1)
    
    optimizer.zero_grad()
    outputs = model(inputs)
    loss = outputs.sum()
    loss.backward()
    optimizer.step()

    if epoch % 10 == 0:
        print(f"Epoch {epoch}, Loss: {loss.item():.4f}")

# 2. 导出ONNX
torch.onnx.export(
    model,
    torch.randn(1, 3),
    "model.onnx",
    input_names=["input"],
    output_names=["output"],
    dynamic_axes={"input": {0: "batch_size"}, "output": {0: "batch_size"}},
    opset_version=12
)

# 3. 加载ONNX格式模型
sess = ort.InferenceSession("model.onnx", providers=["CPUExecutionProvider"])
input_data = np.random.randn(5, 3).astype(np.float32)
outputs = sess.run(["output"], {"input": input_data})
print("ONNX输出形状:", outputs[0].shape)  # 应为 (5, 1)