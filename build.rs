fn main() {
    println!("cargo::rustc-check-cfg=cfg(disable_experiment_tracking)");
}
