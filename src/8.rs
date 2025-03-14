let mut rng = rand::thread_rng();
let code = format!("let x = {}; let y = {}; x + y;", rng.gen_range(1..10), rng.gen_range(1..10));
println!("{}", code);
