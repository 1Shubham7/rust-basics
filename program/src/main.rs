use std::env;

fn main () {
    let env = env::var("ELESTIO_ENV").unwrap_or("dev".to_string());

    let services: u32 = 12000;
    let per_vm: u32 = 1;

    let vms = vm_count(services, per_vm);

    let replicas = if env == "prod" {
        3
    } else {
        1
    };

    print_summary(&env, vms, replicas);

    for attempt in 1..=3 {
        let delay = backoff_ms(attempt);
        println!("attempt {attempt}: would wait {delay} ms");
    }
}

fn vm_count(services: u32, per_vm: u32) -> u32 {
    if per_vm == 0 {
        return 0;
    }

    return services / per_vm;
}

fn backoff_ms(attempts: u32) -> u32 {
    let base = 200;
    return base*2_u32.pow(attempts-1)
}

fn print_summary(env: &str, vms: u32, replicas: u32) {
    println!("env={env} vms={vms} replicas={replicas}");
}