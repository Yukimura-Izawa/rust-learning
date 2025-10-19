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