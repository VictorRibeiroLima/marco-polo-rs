use google_youtube3::Delegate;

#[derive(Debug)]
pub struct UploadDelegator {
    upload_url: Option<String>,
    chunk_size: u64,
}

impl UploadDelegator {
    pub fn new(chunk_size: u64) -> Self {
        Self {
            upload_url: None,
            chunk_size,
        }
    }
}

impl Delegate for UploadDelegator {
    fn upload_url(&mut self) -> Option<String> {
        return self.upload_url.clone();
    }

    fn store_upload_url(&mut self, url: Option<&str>) {
        println!("store_upload_url: {:?}", url);
        self.upload_url = match url {
            Some(url) => Some(url.to_string()),
            None => None,
        }
    }

    fn chunk_size(&mut self) -> u64 {
        return self.chunk_size;
    }
}
