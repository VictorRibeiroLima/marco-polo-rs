pub fn check_envs() {
    //DATABASE
    std::env::var("DATABASE_URL").expect("DATABASE_URL not found");

    //API
    std::env::var("API_URL").expect("API_URL not found");
    std::env::var("API_KEY").expect("API_KEY not found");
    std::env::var("API_JSON_WEB_TOKEN_SECRET").expect("API_JSON_WEB_TOKEN_SECRET not found");

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

    // GOOGLE TRANSLATE API
    std::env::var("GOOGLE_TRANSLATE_API_BASE_URL").expect("GOOGLE_TRANSLATE_API_BASE_URL not set");
    std::env::var("GOOGLE_TRANSLATE_API_KEY").expect("GOOGLE_TRANSLATE_API_KEY not set");

    //VIDEO BOX
    std::env::var("VIDEO_BOX_BASE_URL").expect("VIDEO_BOX_BASE_URL not set");
    std::env::var("VIDEO_BOX_API_KEY").expect("VIDEO_BOX_API_KEY not set");

    //SMTP
    std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME not found");
    std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD not found");
    std::env::var("SMTP_HOST").expect("SMTP_FROM not found");
    std::env::var("SMTP_FROM").expect("SMTP_FROM not found");

    // HASH
    std::env::var("HASH_KEY").expect("HASH_KEY not found");
}
