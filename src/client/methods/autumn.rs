use crate::client::Client;
use core::fmt;
use reywen_http::{driver::Method, results::DeltaError, Delta};

impl Client {
    // prerun - remove all api relevant information from http
    pub fn autumn(&self) -> Delta {
        let mut client = self.http.clone();
        client.url = self.autumn_uri.clone();
        client.remove_headers();
        client
    }
    pub async fn file_download(&self, path: impl Into<String>) -> Result<Vec<u8>, DeltaError> {
        let path = path.into();
        println!("{}", path);
        self.clone()
            .autumn()
            .request_raw(Method::GET, &path, None)
            .await
    }
    pub async fn file_upload(
        &self,
        path: impl Into<String>,
        data: Vec<u8>,
    ) -> Result<Vec<u8>, DeltaError> {
        let path = path.into();
        println!("{}", path);
        self.clone()
            .autumn()
            .request_raw(Method::POST, &path, Some(data))
            .await
    }
}

/*
/// Upload an attachment.
   pub async fn upload(
       &self,
       tag: impl Into<String>,
       filename: impl Into<String>,
       contents: impl AsyncRead + Send + Sync + 'static,
   ) -> Result<UploadData> {
       let stream = FramedRead::new(contents, BytesCodec::new());
       let body = Body::wrap_stream(stream);
       let part = Part::stream(body).file_name(filename.into());
       let form = Form::new().part("file", part);

       let response = self
           .client
           .post(format!("{}/{}", self.base_url, tag.into()))
           .multipart(form)
           .send()
           .await?;

       match response.status().as_u16() {
           200..=299 => Ok(response.json().await?),
           _ => Err(Error::Api(response.json().await?)),
       }
   } */
#[derive(Debug, Default, Clone)]
pub struct Autumn {
    pub collection: AutumnCollection,
    pub id: String,
    pub tag: String,
    pub only_string: Option<String>,
}
impl Autumn {
    pub fn from_string(input: impl Into<String>) -> Self {
        Self {
            only_string: Some(input.into()),
            ..Default::default()
        }
    }

    pub fn create(
        collection: AutumnCollection,
        id: impl Into<String>,
        tag: impl Into<String>,
    ) -> Self {
        Self {
            collection,
            id: id.into(),
            tag: tag.into(),
            only_string: None,
        }
    }
}
#[derive(Debug, Default, Clone)]
pub enum AutumnCollection {
    Attachments,
    Avatars,
    Icons,
    Banners,
    Emojis,
    #[default]
    Null,
}

impl fmt::Display for Autumn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = if let Some(only) = &self.only_string {
            only.to_owned()
        } else {
            format!("/{}/{}/{}", self.collection, self.id, self.tag)
        };
        write!(f, "{a}")
    }
}

impl fmt::Display for AutumnCollection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let a = match self {
            AutumnCollection::Attachments => "attachments",
            AutumnCollection::Avatars => "avatars",
            AutumnCollection::Icons => "icons",
            AutumnCollection::Banners => "banners",
            AutumnCollection::Emojis => "emojis",
            AutumnCollection::Null => "",
        };
        write!(f, "{a}")
    }
}

impl From<Autumn> for String {
    fn from(value: Autumn) -> Self {
        format!("{value}")
    }
}
