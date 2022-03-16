use business_core::business_core::BusinessCore;

fn main() {
    let core = BusinessCore::new();
    core.initialize();
    println!("Is finished !");
}