# openvino_dcn_inference

功能：
1.Rust通过ORT加载ONNX模型，选择后端OpenVINO推理加速
2.Rust通过FFI调用C++

步骤：
 第一步：生成onnx模型
 cd openvino_dcn_inference/model
 python doc_onnx.py
 
 第二步：生成libsum.so
 cd  openvino_dcn_inference/lib
 g++ -shared -fPIC -o libsum.so sum.cpp
 
 第三步：设置环境变量
 export LD_LIBRARY_PATH=/home/data/openvino_dcn_inference/lib:$LD_LIBRARY_PATH
 
 第四步：运行程序
 cargo run -r
