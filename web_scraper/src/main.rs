use thirtyfour::prelude::*;
use tokio;

/**
 * あらゆる要素をテキストで探してクリックする
 * 万能な自作ツール（関数）
 * (変更なし)
 */
async fn find_and_click_by_text(driver: &WebDriver, keyword: &str) -> WebDriverResult<()> {
    println!("「{}」というキーワードで *あらゆる要素* を探しています...", keyword);

    let xpath_selector = format!("//*[contains(text(), '{}')]", keyword);
    let element = driver.find(By::XPath(&xpath_selector)).await?;

    element.click().await?;
    println!("「{}」をクリックしました。", keyword);
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    Ok(())
}


// --- 2. メインの処理 ---
#[tokio::main]
async fn main() -> WebDriverResult<()> {
    // ブラウザ起動の準備 (ポート番号はご自身のものに合わせてください)
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:54369", caps).await?;

    // サイトを開く
    driver.get("https://ops-jg.d1-law.com/opensearch/SrMjF01/init?jctcd=8A8016811F").await?;
    println!("サイトを開きました。");
    
    // --- ↓↓↓ ここが今回の修正点 ↓↓↓ ---

    // [修正点] iframeのコードをすべて削除！
    //          代わりに、メインページのJSが読み込まれるのを 5秒 待つ
    //          (以前の 3秒 + 3秒 = 6秒 より少し短縮)
    println!("メインページのJavaScriptが読み込まれるのを5秒待ちます...");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    
    // --- ↑↑↑ 修正点ここまで ↑↑↑ ---


    // メインページを直接探す
    find_and_click_by_text(&driver, "建設").await?;
    find_and_click_by_text(&driver, "道路").await?;
    
    // --- ↓↓↓ 終了処理も修正 ↓↓↓ ---
    // [修正点] iframeから出る処理も不要なので削除
    // driver.enter_default_frame().await?; // ← 削除

    // ブラウザを閉じる
    driver.quit().await?;
    println!("ブラウザを閉じました。");

    Ok(())
}