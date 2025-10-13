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
   cd ~/projects/hello_rust
   cargo run
   結果として成功すれば「Hello, world!」が表示されるはず.

---

``
## 3. Hello, world! プログラムの理解

src/main.rs`の中身の簡単な解説。

```rust
fn main() {
    // ここがプログラムの開始地点
    println!("Hello, world!"); // 文字列を画面に表示するマクロ
}
``