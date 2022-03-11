## TicToc

A time duration profiling library.

The library comes with `default` and `localtime` feature.

### Usage

Both features enables `unstable-locales` feature from `chrono`, however `default` feature uses Utc for time calculation,
meanwhile `localtime` feature uses current timezone.

Anytime while using `localtime`, you can get Utc result by switching
between `time` and `local_time` methods once `toc` method has been called.

```TOML
...
[dependencies]
tictoc = "0.1"
...
```

...

```TOML
...
[dependencies]
tictoc = { version = "0.1", features = ["localtime"] }
...
```

### Examples



```rust
use tictoc::TicToc;

fn main(){
    let mut tt: TicToc = TicToc::new();
    // Error handling
    let tic = match tt.tic(None) {
        Ok(t) => t,
        Err(e) => {
            panic!("{e}");
            std::process::exit(1);
        }
    };
    // do somethig
    std::thread::sleep(std::time::Duration::from_secs(3));
    // unwrap the errors
    let toc = tt.toc(None).unwrap();
    println!("start_time: {tic}");
    println!("end_time: {toc}");
    // default output in milliseconds
    println!("elapsed_time: {}", tt.time(None, None).unwrap() );
}

```

Named timers

```rust
use tictoc::TicToc;

fn main(){
    let timer_name: Option<&str> = Some("mytimer");

    let mut tt: TicToc = TicToc::new();

    let tic = tt.tic(timer_name).unwrap();

    // do somethig
    std::thread::sleep(std::time::Duration::from_secs(3));

    let toc = tt.toc(timer_name).unwrap();

    println!("start_time: {tic}");
    println!("end_time: {toc}");
    // default output in milliseconds
    println!("elapsed_time: {}", tt.time(timer_name, None).unwrap() );
}

```

Multiple timers

```rust
use tictoc::TicToc;

fn main(){
    let timer_1: Option<&str> = Some("mytimer");
    let timer_2: Option<&str> = Some("mytimer2");
    let timer_3: Option<&str> = Some("mytimer3");

    let mut tt: TicToc = TicToc::new();

    let tic_1 = tt.tic(timer_1).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    // do somethig
    let tic_2 = tt.tic(timer_2).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    // do somethig
    let tic_3 = tt.tic(timer_3).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100));
    // do somethig
    let toc_1 = tt.toc(timer_1).unwrap();
    // do somethig
    let toc_2 = tt.toc(timer_2).unwrap();
    let toc_3 = tt.toc(timer_3).unwrap();

    
    println!(
        "elapsed_time {}: {}",
        timer_1.unwrap(),
        tt.time(timer_1, None).unwrap()
    );
    println!(
        "elapsed_time {}: {}",
        timer_2.unwrap(),
        tt.time(timer_2, None).unwrap()
    );
    println!(
        "elapsed_time {}: {}",
        timer_3.unwrap(),
        tt.time(timer_3, None).unwrap()
    );

}

```

Use custom unit time.

```rust
use tictoc::{TicToc, TimeUnits};

fn main(){
    let mut tt: TicToc = TicToc::new();
    let _ = tt.tic(timer_name).unwrap();
    // do somethig
    std::thread::sleep(std::time::Duration::from_secs(3));
    let _ = tt.toc(None).unwrap();

    // result in Seconds
    println!(
        "elapsed_time: {}",
        tt.time(None, Some(TimeUnits::Seconds)).unwrap()
    );
}

```

Using local time with `localtime` feature enabled.

```rust
use tictoc::{TicToc, TimeUnits};

fn main(){
    let mut tt: TicToc = TicToc::new();
    let tic = tt.tic(timer_name).unwrap();

    // do somethig
    std::thread::sleep(std::time::Duration::from_secs(3));
    let toc = tt.toc(None).unwrap();

    println!("start_time: {tic}");
    println!("end_time: {toc}");


    println!(
        "elapsed_time: {}",
        tt.local_time(None, Some(TimeUnits::Minutes)).unwrap()
    );
}

```

# License
## The MIT License
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)