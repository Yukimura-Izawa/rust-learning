# Rust 学習記録 👻

このリポジトリは,プログラミング言語Rustを学習した過程を記録するもの.

---

## 1. Rustの心得（哲学）

Rustを学ぶ上で最も重要だと感じた,言語の根幹をなす理念.

- **所有権 (Ownership)**: メモリ安全性を保証する核となる概念
- **借用 (Borrowing)**: データを安全に貸し借りする仕組み
- **`Option` / `Result`**: `null`を排除し、エラーハンドリングを強制する型システム

---

## 2. 開発環境の構築 (Windows)

### 1.  Rust本体のインストール
1. [公式サイト](https://rustup.rs/)から`rustup-init.exe`をダウンロードし実行する.
2. インストールを実行後,黒いターミナル画面が表示され,インストールの方法を尋ねられる.
   '1) Proceed with installation (default)` を選ぶ(キーボードの1を入力しEnter).これで標準的なインストールが始まる.
3. インストール完了後ターミナルを開きなおす.

### 2. Microsoft C++ Build Toolsをインストール 
1. Visual Studio Toolsの[公式サイト](https://visualstudio.microsoft.com/ja/visual-cpp-build-tools/)にアクセス.
   「Build Toolsのダウンロード」ボタンをクリックし,インストーラー(vs_buildtools.exeなど)をダウンロードし,実行する.
2. インストール後sign in to visual studioと出てくるが,今回のようにRustのためのビルドツールだけをインストールする場合,Microsoftアカウントでのサインインは全く必要ないためskipを選択.
   その後development settingが出てくるがここもRustには関係ないのでどれを選んでも良い.
   そこまでいけば設定が完了しGet startedとでる.
   右上の「×」で閉じても問題ない.
3. Windowsのスタートメニューを開いて「Visual Studio Installer」を起動する.
   インストーラーが起動すると,インストール済みの製品（"Visual Studio Build Tools 2022"など）が表示される.
   その横にある「変更」（Modify）ボタンをクリック.
   ワークロード選択画面になり,「C++によるデスクトップ開発」にチェックを入れ,右下の「変更」または「インストール」ボタンを押す.

### 3. cargo runの実行
1. ターミナルを開く.
   作業レポジトリに移動し,cargo runを実行する.
   ```rust
   cd ~/projects/hello_rust
   cargo run
   ```
   結果として成功すれば「Hello, world!」が表示されるはず.

---

## 3. FizzBuzz問題を通した基本構文の学習

Rustを理解するための例題として、古典的な「FizzBuzz問題」を扱う.この課題を通して,Rustの基本的な制御構文である **forループ** と **if文** を学ぶ.ファイルはalgorithms_practiceに入っている.

```rust
// 1から100までの数字を順番に表示する
// - 3の倍数: "Fizz"
// - 5の倍数: "Buzz"
// - 3と5の両方の倍数: "FizzBuzz"
fn main() { 
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("FizzBuzz");
        } else if n % 3 == 0 {
            println!("Fizz");
        } else if n % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", n);
        }
    } 
}
```

### 1. 関数の定義 (fn main() {})
   fn は function の略で,関数を定義するキーワード.
   main という名前の関数は特別で,プログラムの実行開始点になる.
   () には関数の引数を書く.今回は空なので引数なし.
   {} で囲まれた部分が関数の処理本体.

### 2. 繰り返し処理 (forループ)
   for 変数 in 範囲 {} の形で指定した範囲の要素を一つずつ取り出して処理を繰り返す.
   1..=100 は「1から100まで(100を含む)」という範囲を表す.
   補足: Rustの変数は不変（イミュータブル）が基本.forループではループの各周回で古い変数 n を破棄し,新しい変数 n を作成する「シャドーイング」という仕組みで値の更新を実現している.

### 3. 条件分岐 (if/else if/else)
   if 条件式 {} で,条件が真 (true) の場合にブロック内の処理を実行する.
   n % 3 == 0 は「nを3で割った余りが0と等しい」という条件式.
   複数の条件を試したい場合は else if 条件式 {}を、どの条件にも当てはまらなかった場合の処理は else {}を使う.
   重要: 条件は上から順番に評価されるため、n % 15 == 0 のような最も厳しい条件（FizzBuzz）を最初に持ってくる必要がある.

### 4. 画面出力 (println!マクロ)
   println!("...") は,コンソールに文字を表示する命令.
   末尾の ! はこれが通常の関数ではなく「マクロ」であることを示している.
   {} はプレースホルダと呼ばれ,後続の引数の値に置き換えられて表示される.
   補足: マクロを使うことで,println!("{}", a) や println!("{} {}", a, b) のように引数の数を柔軟に変えることができる.
   
---

## 4. web scrapingの学習まとめ
Rust言語の学習の一環として特定のウェブサイトから情報を自動で取得するweb scraping(ウェブスクレイピング)について学んだことのまとめ.

### 1. プロジェクトの方針（心）
このプログラムの基本的な考え方は「人間がブラウザを操作する手順をそのままプログラムに真似させる」こと.
実際にサイト内に訪問し,調べたい言葉を検索し,そこをクリックして中に入るようにする.その後情報を取得することを目指す.

### 2. 準備
1. `Cargo.toml` に以下のライブラリを追加
   * `thirtyfour`: ブラウザ(Chromeなど)を自動操作するためのメインライブラリ
   * `tokio`: Rustで非同期処理(`async/await`)を行うための定番ライブラリ
   具体的な操作 : ターミナルでプロジェクトフォルダ(ここでは`web_scraper`)に移動した状態で以下2つを実行.
   ```rust
   cargo add thirtyfour
   cargo add tokio --features full
   ```
2. ChromeDriver (Googleの拡張ツール)の追加
   thirtyfourライブラリがChromeブラウザを操作するためのdriver役としてChromeDriverが必要.
   - [Chrome for Testing availability](https://googlechromelabs.github.io/chrome-for-testing/)からバージョンに合った chromedriver (例: chromedriver-win64.zip) をダウンロードする.
   - ZIPを解凍し,chromedriver.exe をプロジェクトフォルダに移す.
   - ターミナルでプロジェクトに移動し,`.\chromedriver.exe`を実行.
   コマンドを実行すると,ターミナルに以下のようなメッセージが表示され,カーソルが点滅したまま待機状態になる.
   ```rust
   Starting ChromeDriver 141.0.7390.78 (....) on port 0
   ...
   ChromeDriver was started successfully on port 54369.
   ```
   この状態になれば起動成功で,このターミナルは cargo run が終わるまで閉じない.(ポート番号は毎回変わる可能性がある)
   (※今の操作自体をrustのコードに入れることも可能だが,ここではそれはしていない.)
   - chromedriver.exe などは 100MB 以上ある巨大なファイル.これを GitHub にアップロードしないように,Gitに伝える必要がある.
   プロジェクトフォルダに .gitignore というファイルを作り、以下を記述する.
   ```rust
   /target
   Cargo.lock  
   chromedriver.exe  
   ```

### 3. FASE 1 : 言葉を検索し,そこをクリックする
ここでは言葉を検索し,そこをクリックするところまで実装する.
このプログラムは,大きく分けて3つのステップで構成される.
1. 道具の準備 (use ...): thirtyfour や tokio といったブラウザ操作や非同期処理に必要な道具(ライブラリ)を読み込む.

2. 自作ツールの定義 (async fn find_and_click_by_text): 「キーワードで要素を探してクリックする」という、よく使う操作を「自作ツール（関数）」としてまとめて定義しておく.

3. メイン処理の実行 (async fn main): ブラウザ起動 → サイトを開く → 待機 → 自作ツールで"建設"を調べクリックする → 同様に新しく開いたサイト内で自作ツールで"道路"を調べクリックする → ブラウザを閉じる. この流れを実行する.
```rust
// --- 道具の準備 ---
use thirtyfour::prelude::*;
use tokio;

// --- 関数定義 ---
async fn find_and_click_by_text(driver: &WebDriver, keyword: &str) -> WebDriverResult<()> {
    // ターミナルに進捗を表示
    println!("「{}」というキーワードで *あらゆる要素* を探しています...", keyword);

    // [ポイント1] XPathを使って要素を検索
    // "//*[contains(text(), 'キーワード')]" という強力な探し方
    // これにより,<a>タグだけでなく<span>や<div>タグなども対象にできる
    let xpath_selector = format!("//*[contains(text(), '{}')]", keyword);
    
    // .find() で要素を実際に探しに行く (見つかるまで .await で待つ)
    let element = driver.find(By::XPath(&xpath_selector)).await?;

    // 見つけた要素をクリックする
    element.click().await?;
    println!("「{}」をクリックしました。", keyword);
    
    // [ポイント2] ページ遷移を待つための「スリープ」
    // クリック直後はページが読み込み中のため, 2秒間待機する
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    Ok(())
}


// --- メインの処理 ---
#[tokio::main]
async fn main() -> WebDriverResult<()> {
    // [ポイント3] ChromeDriverへの接続
    // ポート番号は、chromedriver.exe起動時に表示されたものに合わせる
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:49255", caps).await?; // ポート番号は要確認

    // サイトを開く(足立区の例規集)
    driver.get("https://ops-jg.d1-law.com/opensearch/SrMjF01/init?jctcd=8A8016811F").await?;
    println!("サイトを開きました。");
    

    // [ポイント4] JavaScriptの読み込み待機 (最重要)
    // このサイトはJavaScriptで動的にリンクを生成する(SPA)。
    // そのため、リンクが表示されるまで5秒間待機する。
    // (iframeの罠を回避した結果、この待機が成功の鍵となった)
    println!("メインページのJavaScriptが読み込まれるのを5秒待ちます...");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    
    // メインページを直接探す
    // 上で定義した自作ツール（関数）を呼び出す
    find_and_click_by_text(&driver, "建設").await?;
    find_and_click_by_text(&driver, "道路").await?;
    

    // ブラウザを閉じる
    driver.quit().await?;
    println!("ブラウザを閉じました。");

    Ok(())
}
```
