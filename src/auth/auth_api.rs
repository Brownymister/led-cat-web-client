use reqwest::Client;

pub async fn show_auth_num() -> Result<(), std::io::Error> {
    let client = Client::new();
    let res = client.get("http://192.168.1.120:8080/api/show_auth_num").send().await;
    // handle res with error messages
    match res {
        Ok(_) => {}
        Err(e) => {
            if e.is_timeout() {
                return Err(std::io::Error::new(std::io::ErrorKind::TimedOut, "Timeout"));
            }
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Other"));
        }
    }
    return Ok(());
}
