pub fn check_envs() {
    //DATABASE
    std::env::var("DATABASE_URL").expect("DATABASE_URL not found");

    //API
    std::env::var("API_URL").expect("API_URL not found");
    std::env::var("API_KEY").expect("API_KEY not found");

    //AWS
    std::env::var("AWS_BUCKET_NAME").expect("AWS_BUCKET_NAME not found");
    std::env::var("AWS_ACCESS_KEY_ID").expect("AWS_ACCESS_KEY_ID not found");
    std::env::var("AWS_SECRET_ACCESS_KEY").expect("AWS_SECRET_ACCESS_KEY not found");
    std::env::var("AWS_QUEUE_URL").expect("AWS_QUEUE_URL not found");

    //ASSEMBLY_AI
    std::env::var("ASSEMBLY_AI_API_KEY").expect("ASSEMBLY_AI_API_KEY not found");
    std::env::var("ASSEMBLY_AI_BASE_URL").expect("ASSEMBLY_AI_BASE_URL not found");
    std::env::var("ASSEMBLY_AI_WEBHOOK_ENDPOINT").expect("ASSEMBLY_AI_WEBHOOK_ENDPOINT not found");
    std::env::var("ASSEMBLY_AI_WEBHOOK_TOKEN").expect("ASSEMBLY_AI_WEBHOOK_TOKEN not found");

    //DEEPL
    std::env::var("DEEPL_BASE_URL").expect("DEEPL_BASE_URL not set");
    std::env::var("DEEPL_API_KEY").expect("DEEPL_API_KEY not set");
}
