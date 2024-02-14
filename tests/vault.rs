use cdbootstrap::vault::*;
use std::env;

#[tokio::test]
async fn print_secret_works() {
    let tenant = env::var("TENANT").unwrap_or("none".to_string());
    let keyvault_url = env::var("KEYVAULT_URL").unwrap_or("none".to_string());
    let spn = env::var("SPN").unwrap_or("none".to_string());
    let secret_name = env::var("SECRET_NAME").unwrap_or("none".to_string());

    let azure = AzureVault::new(&tenant, &keyvault_url, &spn);
    AzureVault::print_secret_from_vault(&azure, &secret_name).await;
}
