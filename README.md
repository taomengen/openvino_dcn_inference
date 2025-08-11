# openvino_dcn_inference

功能：

1.Rust通过ORT加载ONNX模型，选择后端OpenVINO推理加速

2.Rust通过FFI调用C++

（1）静态链接libexample.a，直接编译到rust二进制文件中

objdump -t openvino_dcn_inference | grep 'example'

0000000000000000 l    df *ABS*	0000000000000000              example.cpp

l（小写L）：表示这个符号是局部符号（local），通常意味着这是来自静态链接库（libexample.a）的代码，

但符号已被内联/优化，未暴露为全局符号

df：表示该符号关联到一个调试信息段

*ABS*：绝对地址（说明代码已被静态链接到二进制中）

example.cpp：明确显示来自你的 C++ 源文件，证明静态链接成功

（2）动态链接libsum.so,ldd可以看大显示调用

ldd openvino_dcn_inference
	
    libsum.so => /home/data/openvino_dcn_inference/lib/libsum.so (0x000079d50299e000)

    libonnxruntime.so.1.16.0 (0x000079d501400000)


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
