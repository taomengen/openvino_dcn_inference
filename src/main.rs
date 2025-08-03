use ort::{
    Environment, SessionBuilder, GraphOptimizationLevel, Value,
    execution_providers::{OpenVINOExecutionProviderOptions}
};
use std::sync::Arc;
use std::path::PathBuf;
use ndarray::{Array, IxDyn, CowArray};

#[repr(C)]
pub struct Calculator;

extern "C" {
    fn calculator_new() -> *mut Calculator;
    fn calculator_add(calc: *mut Calculator, a: i32, b: i32) -> i32;
    fn calculator_delete(calc: *mut Calculator);
}

// 在代码中指定动态库名称（不带 `lib` 前缀和 `.so` 后缀）
#[link(name = "sum", kind = "dylib")]
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn call_cplusplus () -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let calc = calculator_new();
        let sum = calculator_add(calc, 2, 3);
        println!("2 + 3 = {}", sum);
        calculator_delete(calc);
    }

    Ok(())
}

fn calc_sum () -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        let result = add(10, 2);
        println!("10 + 2 = {}", result);
    }

    Ok(())
}

fn run_unsafe () -> Result<(), Box<dyn std::error::Error>> {
    let raw_ptr: *const i32 = &42 as *const i32;
    // 直接解引用原始指针
    // println!("{}", *raw_ptr); 
    unsafe {
        // 明确告知编译器："我知道风险，我来负责"
        println!("{}", *raw_ptr); 
    }
    Ok(())
}

fn inference () -> Result<(), Box<dyn std::error::Error>> {
    // 初始化环境
    let environment = Arc::new(Environment::builder()
        .with_name("onnx_inference")
        .build()?);

    // 创建OpenVINO执行提供者配置
    let openvino_options = OpenVINOExecutionProviderOptions::default();
    // 强制使用CPU
    std::env::set_var("OV_PLUGIN_PRIORITY", "CPU");

    // 创建会话（禁用训练优化）
    //let path = "/home/mengen.tao/taomengen/openvino_dcn_inference/model.onnx";
    let path = PathBuf::from("./model/model.onnx");
    let session = SessionBuilder::new(&environment)?
        .with_optimization_level(GraphOptimizationLevel::Disable)?  // Eval模式
        .with_execution_providers([ort::ExecutionProvider::OpenVINO(openvino_options)])?
        //.with_model_from_file("model.onnx")?;
        .with_model_from_file(path)?;


    // 准备输入数据
    // 模型期望形状
    let input_info = &session.inputs[0];
    println!("模型期望形状: {:?}", input_info.dimensions().collect::<Vec<_>>());
    
    // 根据模型期望形状准备输入数据
    // 匹配模型的 [None, 3] 形状
    let input_shape = IxDyn(&[1, 3]);
    // 创建 1x3 的零值数组
    let input_array: Array<f32, _> = Array::zeros(input_shape); 
    println!("实际输入形状: {:?}", input_array.shape());

    let input_cow = CowArray::from(input_array);
    let input_tensor = Value::from_array(session.allocator(), &input_cow)?;

    // 模型推理
    let outputs = session.run(vec![input_tensor])?;

    // 处理输出
    for output in outputs {
        println!("输出张量: {:?}", output.try_extract::<f32>()?);
    }
    Ok(())

}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let _ = call_cplusplus();
    let _ = calc_sum();
    let _ = run_unsafe();
    let _ = inference();
    
    Ok(())
}