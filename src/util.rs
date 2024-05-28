/// 获取是否是GPU
pub fn get_cpu_or_gpu() -> String {
    let str = if cfg!(feature = "cuda") {
        "GPU"
    } else {
        "CPU"
    };
    str.to_string()
}