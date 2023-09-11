use std::io::{self, Write};
use std::time::Instant;
use std::str::FromStr;

mod tests;
mod bfs;


fn main() -> io::Result<()> {
    let run_tests = read::<bool>("Run tests? ")?;
    if run_tests {
        tests::tests();
        return Ok(());
    }

    let start = read("start = ")?;
    let target = read("target = ")?;
    println!("------------");

    let functions = &vec![
        |x| { x + 3 },
        |x| { x * 2 },
        |x| { x - 2 },
    ];

    let inverse_functions = &vec![
        |x| { x - 3 },
        |x| { x / 2 },
        |x| { x + 2 },
    ];
    
    {
        let now = Instant::now();
        let path = bfs::bidirectional(start, target, functions, inverse_functions);
        let elapsed = now.elapsed();
        println!("[BIDIRECTIONAL] Result: {path:?} (took: {elapsed:.2?})");
    }
    
    // {
    //     let now = Instant::now();
    //     let path = bfs::normal(start, target, functions);
    //     let elapsed = now.elapsed();
    //     println!("[NORMAL] Result: {path:?} (took: {elapsed:.2?})");
    // }

    Ok(())
}


fn read<T: FromStr>(name: &str) -> io::Result<T> {
    loop {
        print!("{name}");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        input.pop(); input.pop();

        if let Ok(x) = input.parse::<T>() {
            return Result::Ok(x);
        }
    }
}
