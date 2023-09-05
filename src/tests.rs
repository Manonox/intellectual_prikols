use std::time::Instant;

#[path = "bfs.rs"] mod bfs;


pub fn tests() {

    // Ruleset: +3, *2
    {
        let functions = &vec![
            |x: i32| -> i32 { x + 3 },
            |x: i32| -> i32 { x * 2 },
        ];

        let inverse_functions = &vec![
            |x: i32| -> i32 { x - 3 },
            |x: i32| -> i32 { x / 2 },
        ];

        println!("# Ruleset: +3, *2");

        println!("\n[NORMAL]");
        case(1, 100, 7, functions);
        case(2, 55, 6, functions);
        case(2, 100, 7, functions);
        case(1, 97, 8, functions);
        case(2, 1000, 12, functions);
        // case(2, 10000001, 30, functions1);

        println!("\n[BIDIRECTIONAL]");
        case_bi(1, 100, 7, functions, inverse_functions);
        case_bi(2, 55, 6, functions, inverse_functions);
        case_bi(2, 100, 7, functions, inverse_functions);
        case_bi(1, 97, 8, functions, inverse_functions);
        case_bi(2, 1000, 12, functions, inverse_functions);
        case_bi(2, 10000001, 30, functions, inverse_functions);

        println!("\n");
    }

    // Ruleset: +3, *2, -2
    {
        let functions = &vec![
            |x: i32| -> i32 { x + 3 },
            |x: i32| -> i32 { x * 2 },
            |x: i32| -> i32 { x - 2 },
        ];

        let inverse_functions = &vec![
            |x: i32| -> i32 { x - 3 },
            |x: i32| -> i32 { x / 2 },
            |x: i32| -> i32 { x + 2 },
        ];

        println!("# Ruleset: +3, *2, -2");
        println!("\n[NORMAL]");
        case(1, 100,	7, functions);
        case(2, 55,	6, functions);
        case(2, 100,	7, functions);
        case(1, 97,	8, functions);
        case(2, 1000,	11, functions);
        case(3, 1001,	13, functions);
        // case(2, 10000001,	30, functions);

        println!("\n[BIDIRECTIONAL]");
        case_bi(1, 100,	7, functions, inverse_functions);
        case_bi(2, 55,	6, functions, inverse_functions);
        case_bi(2, 100,	7, functions, inverse_functions);
        case_bi(1, 97,	8, functions, inverse_functions);
        case_bi(2, 1000,	11, functions, inverse_functions);
        case_bi(3, 1001,	13, functions, inverse_functions);
        case_bi(2, 10000001,	30, functions, inverse_functions);

        println!("\n");
    }
}

fn case(start: i32, target: i32, path_length: i32, functions: &Vec<fn(i32) -> i32>) {
    let now = Instant::now();
    let path = bfs::normal(start, target, &functions);
    let elapsed = now.elapsed();
    let operations = path.len() - 1;
    let symbol = if operations == path_length.try_into().unwrap() { "✅" } else { "🟥" };
    println!("\tResult: {path:?} (took: {elapsed:.2?}) [{symbol}]");
}

fn case_bi(start: i32, target: i32, path_length: i32, functions: &Vec<fn(i32) -> i32>, inverse_functions: &Vec<fn(i32) -> i32>) {
    let now = Instant::now();
    let path = bfs::bidirectional(start, target, &functions, &inverse_functions);
    let elapsed = now.elapsed();
    let operations = path.len() - 1;
    let symbol = if operations == path_length.try_into().unwrap() { "✅" } else { "🟥" };
    println!("\tResult: {path:?} (took: {elapsed:.2?}) [{symbol}]");
}
