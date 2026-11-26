use std::env;

use wallpaper;

fn main() {
    App::new().run();
}

struct App {
    dir_path: String,
}

impl App {

    const GET: &str = "get";
    const SET: &str = "set";
    const RANDOM: &str = "random";
    const HELP: &str = "help";

    pub fn new() -> Self {
        Self {
            // jsonからディレクトリパスを取得
            dir_path: String::from("C:\\Users\\norder\\Documents\\Images\\Nord\\"),
        }
    }

    pub fn run(&mut self) {
        // コマンドライン引数から受け取り
        let args: Vec<String> = env::args().collect();

        // コマンドライン引数があるのならば
        if args.len() > 1 {

            // マッチするコマンドを探索し, 実行
            match args[1].as_str() {
                Self::GET => self.get_dir_path(),
                Self::SET => {
                    if args.len() > 2 { self.set_dir_path(&args[2]) }
                },
                Self::RANDOM => self.change_random_wallpaper(),
                Self::HELP => self.help(),
                _ => {},
            }

        }

    }

    // ディレクトリパスを表示
    fn get_dir_path(&self) {
        println!("directory path: {}", self.dir_path);
    }

    // ディレクトリパスを設定
    fn set_dir_path(&mut self, dir_path: &str) {
        self.dir_path = dir_path.to_string();
    }

    fn help(&self) {
        println!("help");
    }

    // ランダムに壁紙を変更する
    fn change_random_wallpaper(&self) {
        println!("random");
        self.change_wallpaper("C:\\Users\\norder\\Documents\\Images\\Nord\\nord02.png");
    }

    // 壁紙を指定ファイルパスのものに変更する
    fn change_wallpaper(&self, file_path: &str) {
        wallpaper::set_from_path(file_path).unwrap();
    }

}

