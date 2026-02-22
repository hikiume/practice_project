use std::path::Path;

// P は AsRef<Path> を持っている（＝パスの住所を貸し出せる）型なら何でもOK！
fn print_file_extension<P: AsRef<Path>>(path: P) {
    // path.as_ref() を呼んで、共通の「&Path（住所）」を受け取る
    let p = path.as_ref();

    // パスとして扱えるので、拡張子を取り出すような操作ができる
    if let Some(ext) = p.extension() {
        println!("拡張子は: {:?}", ext);
    } else {
        println!("拡張子は見つかりませんでした");
    }
}

fn main() {
    // 1. &str 型を渡す
    let s: &str = "test.json";
    print_file_extension(s);

    // 2. String 型を渡す
    let string_type: String = String::from("main.rs");
    print_file_extension(string_type);

    // 3. PathBuf（パス専用の型）を渡す
    let path_type = std::path::PathBuf::from("config.toml");
    print_file_extension(path_type);
}
