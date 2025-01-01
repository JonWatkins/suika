pub fn get_mime_type(extension: &str) -> &str {
  match extension {
      "html" => "text/html",
      "css" => "text/css",
      "js" => "application/javascript",
      "png" => "image/png",
      "jpg" | "jpeg" => "image/jpeg",
      "gif" => "image/gif",
      "svg" => "image/svg+xml",
      "txt" => "text/plain",
      "ico" => "image/x-icon",
      "json" => "application/json",
      "xml" => "application/xml",
      "pdf" => "application/pdf",
      "zip" => "application/zip",
      "mp3" => "audio/mpeg",
      "mp4" => "video/mp4",
      "wav" => "audio/wav",
      _ => "application/octet-stream",
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_mime_type() {
      assert_eq!(get_mime_type("html"), "text/html");
      assert_eq!(get_mime_type("css"), "text/css");
      assert_eq!(get_mime_type("js"), "application/javascript");
      assert_eq!(get_mime_type("png"), "image/png");
      assert_eq!(get_mime_type("jpg"), "image/jpeg");
      assert_eq!(get_mime_type("jpeg"), "image/jpeg");
      assert_eq!(get_mime_type("gif"), "image/gif");
      assert_eq!(get_mime_type("svg"), "image/svg+xml");
      assert_eq!(get_mime_type("txt"), "text/plain");
      assert_eq!(get_mime_type("ico"), "image/x-icon");
      assert_eq!(get_mime_type("json"), "application/json");
      assert_eq!(get_mime_type("xml"), "application/xml");
      assert_eq!(get_mime_type("pdf"), "application/pdf");
      assert_eq!(get_mime_type("zip"), "application/zip");
      assert_eq!(get_mime_type("mp3"), "audio/mpeg");
      assert_eq!(get_mime_type("mp4"), "video/mp4");
      assert_eq!(get_mime_type("wav"), "audio/wav");
      assert_eq!(get_mime_type("unknown"), "application/octet-stream");
  }
}
