
fn handle_args() {
    use std::env;
    // env::args()はコマンドライン引数をIteratorとして順番に渡してくれる関数
    // skipで最初の一つ(=実行ファイル名)を飛ばして引数だけString型として配列argsに入れる
    let args: Vec<String> = env::args().skip(1).collect();

    let str_n = "123456789";
    // Stringをchars()するとchar型のIteratorとして渡してくれるのでVectorに入れられる
    let num_n: Vec<char> = str_n.chars().collect();

    // Rustのrangeは0..nって書くらしい
    for i in 0..args.len(){
        println!("{}: {}", num_n[i], args[i]);
    }
}

fn handle_stdin() {
    use std::io;

    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();

    println!("{}", line);
}


use std::env;
mod cmds;

fn main() {

    // handle_args();
    // handle_stdin();
    
    let args: Vec<String> = env::args().skip(1).collect();
    if args[0] == "cut" {
        cmds::cut::my_cut(&args[1], args[2].trim().parse().unwrap());
    }
    
}
