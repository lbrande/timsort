#![allow(unused_macros)]
macro_rules! scan_str {
    ($str:expr,$($var:ident),+) => {
        let mut str = $str.split_whitespace();
        $($var = str.next().unwrap().parse().unwrap();)+
    };
    ($str:expr,$max:expr,$($var:ident),+) => {
        let mut str = $str.splitn($max, char::is_whitespace);
        $($var = str.next().unwrap().parse().unwrap();)+
    };
    ($str:expr) => (
        $str.split_whitespace().map(|s| s.parse().unwrap())
    )
}

macro_rules! scanln {
    ($($var:ident),+) => {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        scan_str!(line, $($var),+);
    };
    ($max:expr,$($var:ident),+) => {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        scan_str!(line, $max, $($var),+);
    };
    () => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        scan_str!(line.clone())
    })
}

macro_rules! readln {
    () => ({
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.truncate(line.len() - 1);
        line
    });
}