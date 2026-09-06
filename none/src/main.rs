fn region_for(provider: &str) -> Option<String> {
    if provider == "hetzner" {
        Some("fsn1".to_string())
    } else {
        None
    }
}

fn compute_default() -> String {
    "default-region".to_string()
}

fn main() {
    // 1. unwrap()
    // Takes the value out of Some.
    // Panics if the Option is None.
    let r = region_for("hetzner");
    let region = r.unwrap();
    println!("unwrap: {region}");

    // 2. expect()
    // Same idea as unwrap(), but with a custom panic message.
    let r = region_for("hetzner");
    let region = r.expect("hetzner must have a region");
    println!("expect: {region}");

    // 3. unwrap_or()
    // Returns the value if Some, otherwise uses the provided default.
    let r = region_for("aws");
    let region = r.unwrap_or("default".to_string());
    println!("unwrap_or: {region}");

    // 4. unwrap_or_default()
    // Returns the value if Some, otherwise String::default().
    // String::default() is "".
    let r = region_for("aws");
    let region = r.unwrap_or_default();
    println!("unwrap_or_default: '{region}'");

    // 5. unwrap_or_else()
    // Returns the value if Some.
    // Calls the closure only if None.
    let r = region_for("aws");
    let region = r.unwrap_or_else(|| compute_default());
    println!("unwrap_or_else: {region}");

    // 6. match
    // Explicitly handle both possibilities.
    let r = region_for("hetzner");

    match r {
        Some(region) => println!("match: deploy to {region}"),
        None => println!("match: no region configured"),
    }

    // 7. if let
    // Only handle the Some case.
    let r = region_for("hetzner");

    if let Some(region) = r {
        println!("if let: deploy to {region}");
    }
}