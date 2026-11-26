use std::env;
use std::fs;
use std::path::{PathBuf, absolute};
use std::io;
use std::error::Error;

use wallpaper;
use rand;
use serde_json::{Result, Value};

fn main() {
    let mut app = App::new();
    app.run();
}

struct App {
    dir_path: PathBuf,
}

impl App {

    // コマンドの定数
    const GET: &str = "get";
    const SET: &str = "set";
    const RANDOM: &str = "random";
    const HELP: &str = "help";

    pub fn new() -> Self {
        Self {
            // jsonからディレクトリパスを取得
            dir_path: PathBuf::from("C:\\Users\\norder\\Documents\\Images\\Nord\\"),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        // コマンドライン引数から受け取り
        let args: Vec<String> = env::args().collect();

        // コマンドライン引数があるのならば
        if args.len() > 1 {

            // マッチするコマンドを探索し, 実行
            match args[1].as_str() {
                Self::GET => self.get_dir_path(),
                Self::SET => {
                    if args.len() > 2 { self.set_dir_path(&args[2]); }
                },
                Self::RANDOM => self.change_random_wallpaper()?,
                Self::HELP => self.help(),
                _ => {},
            }

        }

        Ok(())

    }

    // ディレクトリパスを表示
    fn get_dir_path(&self) {
        println!("directory path: {:?}", self.dir_path);
    }

    // ディレクトリパスを設定
    fn set_dir_path(&mut self, dir_path: &str) -> Result<(), Box<dyn Error>> {
        let path = PathBuf::from(dir_path);

        if !path.is_dir() {
            return Err("Directory Is Not Found.".into());
        }

        self.dir_path = path;

        Ok(())
    }

    // ヘルプの表示
    fn help(&self) {
        println!("──────────────────────────────────────────── HELP ────────────────────────────────────────────");
        println!("wallpaper get                           対象ディレクトリパスを取得します.");
        println!("wallpaper set <directory path>          対象ディレクトリパスを設定します.");
        println!("wallpaper random                        対象ディレクトリ内からランダムに壁紙を設定します.");
        println!("wallpaper help                          ヘルプを表示します.");
        println!("──────────────────────────────────────────────────────────────────────────────────────────────");
    }

    // ランダムに壁紙を変更する
    fn change_random_wallpaper(&self) -> Result<(), Box<dyn Error>> {

        // ファイルパスを格納するベクトル
        let mut image_path_list: Vec<PathBuf> = Vec::new();

        // ファイル取得
        let read_dir = fs::read_dir(&self.dir_path)?;
        for entry in read_dir {
            let dir = entry?;
            image_path_list.push(dir.path());
        }

        // 画像ファイル以外を削除
        image_path_list.retain(|path| {
            matches!(
                path.extension().and_then(|ext| ext.to_str()),
                Some("png") | Some("jpg") | Some("jpeg")
            )
        });

        if image_path_list.len() == 0 {
            return Err("Image File Is Not Found.".into());
        }

        // 乱数生成
        let random_num: usize = rand::random_range(0..image_path_list.len());

        // ランダムで選択したパスを渡して壁紙変更
        let abs_path = absolute(&image_path_list[random_num])?;
        self.change_wallpaper(abs_path)?;

        Ok(())

    }

    // 壁紙を指定ファイルパスのものに変更する
    fn change_wallpaper(&self, file_path: PathBuf) -> Result<(), Box<dyn Error>> {
        let path_text = file_path.to_str().unwrap();
        wallpaper::set_from_path(path_text)?;
        Ok(())
    }

}
