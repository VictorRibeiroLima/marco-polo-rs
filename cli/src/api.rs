use crate::API_URL;

#[derive(serde::Serialize, Debug)]
struct Login {
    email: String,
    password: String,
}

pub async fn login(login: &str, password: &str) -> Result<(), ()> {
    let login = Login {
        email: login.to_string(),
        password: password.to_string(),
    };

    let client = reqwest::Client::new();

    let url = format!("{}/user/login", API_URL);

    let res = client
        .post(url)
        .json(&login)
        .send()
        .await
        .expect("Failed to login");

    if !res.status().is_success() {
        eprintln!("Failed to login");
        return Err(());
    }

    Ok(())
}
