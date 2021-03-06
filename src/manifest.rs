use futures::{Future, Stream};
use rusoto_s3::{GetObjectRequest, S3Client, S3};
use std::fs::read_to_string;

static MANIFEST_KEY: &'static str = "manifest_v2.json";

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub bucket_name: String,
    pub files: Vec<FileMetadata>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FileMetadata {
    pub key: String,
    pub hash: String,
    pub last_modified: i64,
    pub content_length: i64,
}

impl Manifest {
    pub fn from_cloud(&self, s3: &S3Client) -> Manifest {
        let request = GetObjectRequest {
            bucket: self.bucket_name.to_owned(),
            key: MANIFEST_KEY.to_owned(),
            ..Default::default()
        };

        let resp = s3.get_object(request).sync().expect("Couldn't GET object");
        let stream = resp.body.expect("Couldn't get body stream");
        let body = stream.concat2().wait().expect("Couldn't stream body");
        return serde_json::from_slice(&body)
            .expect("Couldn't serialize Manifest");
    }

    pub fn from_local(path: &String) -> Manifest {
        let bytes = read_to_string(path).expect("Reading manifest failed.");
        return serde_json::from_str(&bytes).expect("Deserializing failed");
    }
}
