#[cfg(target_arch = "wasm32")]
pub mod test {
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn pass() {
        // sanity check
        assert_eq!(1, 1);
    }

    #[wasm_bindgen_test]
    async fn post() {
        use reqwest::Client;
        let req = Client::new()
            .get("https://postman-echo.com/post")
            .body("this should be the returned body")
            .send()
            .await
            .unwrap();

        let text: String = req.text().await.unwrap();
        assert_eq!(&text, "this should be the returned body");
    }
}
