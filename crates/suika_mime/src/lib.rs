use std::fmt;
use std::path::Path;

/// Enum representing various MIME types.
#[derive(Debug, PartialEq)]
pub enum MimeType {
    TextHtml,
    TextCss,
    ApplicationJavaScript,
    ApplicationJson,
    ApplicationXml,
    ApplicationPdf,
    ApplicationZip,
    ApplicationXTar,
    ApplicationGzip,
    ApplicationXBzip2,
    ApplicationX7zCompressed,
    ApplicationVndRar,
    ApplicationVndMicrosoftPE,
    ApplicationXMsdownload,
    ApplicationOctetStream,
    ImagePng,
    ImageJpeg,
    ImageGif,
    ImageSvgXml,
    ImageBmp,
    ImageXIcon,
    ImageTiff,
    ImageWebp,
    AudioMpeg,
    AudioWav,
    AudioOgg,
    AudioFlac,
    AudioAac,
    VideoMp4,
    VideoXM4v,
    VideoXMatroska,
    VideoWebm,
    VideoXMsVideo,
    VideoQuicktime,
    VideoXMsWmv,
    TextPlain,
    TextCsv,
    TextMarkdown,
    ApplicationRtf,
    ApplicationVndOasisOdt,
    ApplicationVndOasisOds,
    ApplicationVndOasisOdp,
    ApplicationMsword,
    ApplicationVndOpenxmlWord,
    ApplicationVndMsExcel,
    ApplicationVndOpenxmlExcel,
    ApplicationVndMsPowerpoint,
    ApplicationVndOpenxmlPowerpoint,
    ApplicationWasm, // Added WebAssembly MIME type
}

impl fmt::Display for MimeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl MimeType {
    /// Converts the `MimeType` enum to a string slice.
    pub fn as_str(&self) -> &str {
        match self {
            MimeType::TextHtml => "text/html",
            MimeType::TextCss => "text/css",
            MimeType::ApplicationJavaScript => "application/javascript",
            MimeType::ApplicationJson => "application/json",
            MimeType::ApplicationXml => "application/xml",
            MimeType::ApplicationPdf => "application/pdf",
            MimeType::ApplicationZip => "application/zip",
            MimeType::ApplicationXTar => "application/x-tar",
            MimeType::ApplicationGzip => "application/gzip",
            MimeType::ApplicationXBzip2 => "application/x-bzip2",
            MimeType::ApplicationX7zCompressed => "application/x-7z-compressed",
            MimeType::ApplicationVndRar => "application/vnd.rar",
            MimeType::ApplicationVndMicrosoftPE => "application/vnd.microsoft.portable-executable",
            MimeType::ApplicationXMsdownload => "application/x-msdownload",
            MimeType::ApplicationOctetStream => "application/octet-stream",
            MimeType::ImagePng => "image/png",
            MimeType::ImageJpeg => "image/jpeg",
            MimeType::ImageGif => "image/gif",
            MimeType::ImageSvgXml => "image/svg+xml",
            MimeType::ImageBmp => "image/bmp",
            MimeType::ImageXIcon => "image/x-icon",
            MimeType::ImageTiff => "image/tiff",
            MimeType::ImageWebp => "image/webp",
            MimeType::AudioMpeg => "audio/mpeg",
            MimeType::AudioWav => "audio/wav",
            MimeType::AudioOgg => "audio/ogg",
            MimeType::AudioFlac => "audio/flac",
            MimeType::AudioAac => "audio/aac",
            MimeType::VideoMp4 => "video/mp4",
            MimeType::VideoXM4v => "video/x-m4v",
            MimeType::VideoXMatroska => "video/x-matroska",
            MimeType::VideoWebm => "video/webm",
            MimeType::VideoXMsVideo => "video/x-msvideo",
            MimeType::VideoQuicktime => "video/quicktime",
            MimeType::VideoXMsWmv => "video/x-ms-wmv",
            MimeType::TextPlain => "text/plain",
            MimeType::TextCsv => "text/csv",
            MimeType::TextMarkdown => "text/markdown",
            MimeType::ApplicationRtf => "application/rtf",
            MimeType::ApplicationVndOasisOdt => "application/vnd.oasis.opendocument.text",
            MimeType::ApplicationVndOasisOds => "application/vnd.oasis.opendocument.spreadsheet",
            MimeType::ApplicationVndOasisOdp => "application/vnd.oasis.opendocument.presentation",
            MimeType::ApplicationMsword => "application/msword",
            MimeType::ApplicationVndOpenxmlWord => {
                "application/vnd.openxmlformats-officedocument.wordprocessingml.document"
            }
            MimeType::ApplicationVndMsExcel => "application/vnd.ms-excel",
            MimeType::ApplicationVndOpenxmlExcel => {
                "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
            }
            MimeType::ApplicationVndMsPowerpoint => "application/vnd.ms-powerpoint",
            MimeType::ApplicationVndOpenxmlPowerpoint => {
                "application/vnd.openxmlformats-officedocument.presentationml.presentation"
            }
            MimeType::ApplicationWasm => "application/wasm", // Added WebAssembly MIME type
        }
    }
}

/// Returns the MIME type for a given file extension.
///
/// This function takes a file extension as input and returns the corresponding
/// MIME type as a `String`. If the file extension is not recognized, it returns
/// `"application/octet-stream"`.
///
/// # Arguments
///
/// * `extension` - A string slice that holds the file extension.
///
/// # Examples
///
/// ```
/// use suika_mime::get_mime_type;
///
/// assert_eq!(get_mime_type("html"), "text/html".to_string());
/// assert_eq!(get_mime_type("css"), "text/css".to_string());
/// assert_eq!(get_mime_type("js"), "application/javascript".to_string());
/// assert_eq!(get_mime_type("png"), "image/png".to_string());
/// assert_eq!(get_mime_type("jpg"), "image/jpeg".to_string());
/// assert_eq!(get_mime_type("unknown"), "application/octet-stream".to_string());
/// ```
pub fn get_mime_type(extension: &str) -> String {
    match extension {
        "html" | "htm" => MimeType::TextHtml,
        "css" => MimeType::TextCss,
        "js" | "mjs" => MimeType::ApplicationJavaScript,
        "json" => MimeType::ApplicationJson,
        "xml" => MimeType::ApplicationXml,
        "pdf" => MimeType::ApplicationPdf,
        "zip" => MimeType::ApplicationZip,
        "tar" => MimeType::ApplicationXTar,
        "gz" => MimeType::ApplicationGzip,
        "bz2" => MimeType::ApplicationXBzip2,
        "7z" => MimeType::ApplicationX7zCompressed,
        "rar" => MimeType::ApplicationVndRar,
        "exe" => MimeType::ApplicationVndMicrosoftPE,
        "msi" => MimeType::ApplicationXMsdownload,
        "bin" | "dll" | "iso" | "dmg" => MimeType::ApplicationOctetStream,
        "png" => MimeType::ImagePng,
        "jpg" | "jpeg" => MimeType::ImageJpeg,
        "gif" => MimeType::ImageGif,
        "svg" => MimeType::ImageSvgXml,
        "bmp" => MimeType::ImageBmp,
        "ico" => MimeType::ImageXIcon,
        "tiff" | "tif" => MimeType::ImageTiff,
        "webp" => MimeType::ImageWebp,
        "mp3" => MimeType::AudioMpeg,
        "wav" => MimeType::AudioWav,
        "ogg" => MimeType::AudioOgg,
        "flac" => MimeType::AudioFlac,
        "aac" => MimeType::AudioAac,
        "mp4" => MimeType::VideoMp4,
        "m4v" => MimeType::VideoXM4v,
        "mkv" => MimeType::VideoXMatroska,
        "webm" => MimeType::VideoWebm,
        "avi" => MimeType::VideoXMsVideo,
        "mov" => MimeType::VideoQuicktime,
        "wmv" => MimeType::VideoXMsWmv,
        "txt" => MimeType::TextPlain,
        "csv" => MimeType::TextCsv,
        "md" => MimeType::TextMarkdown,
        "rtf" => MimeType::ApplicationRtf,
        "odt" => MimeType::ApplicationVndOasisOdt,
        "ods" => MimeType::ApplicationVndOasisOds,
        "odp" => MimeType::ApplicationVndOasisOdp,
        "doc" => MimeType::ApplicationMsword,
        "docx" => MimeType::ApplicationVndOpenxmlWord,
        "xls" => MimeType::ApplicationVndMsExcel,
        "xlsx" => MimeType::ApplicationVndOpenxmlExcel,
        "ppt" => MimeType::ApplicationVndMsPowerpoint,
        "pptx" => MimeType::ApplicationVndOpenxmlPowerpoint,
        "wasm" => MimeType::ApplicationWasm, // Added WebAssembly MIME type
        _ => MimeType::ApplicationOctetStream,
    }
    .to_string()
}

/// Returns the MIME type for a given file path.
///
/// This function takes a file path as input and returns the corresponding
/// MIME type as a `String`. If the file extension is not recognized or the path
/// has no extension, it returns `"application/octet-stream"`.
///
/// # Arguments
///
/// * `path` - A string slice that holds the file path.
///
/// # Examples
///
/// ```
/// use suika_mime::get_mime_type_from_path;
///
/// assert_eq!(get_mime_type_from_path("index.html"), "text/html".to_string());
/// assert_eq!(get_mime_type_from_path("style.css"), "text/css".to_string());
/// assert_eq!(get_mime_type_from_path("script.js"), "application/javascript".to_string());
/// assert_eq!(get_mime_type_from_path("image.png"), "image/png".to_string());
/// assert_eq!(get_mime_type_from_path("unknownfile"), "application/octet-stream".to_string());
/// ```
pub fn get_mime_type_from_path(path: &str) -> String {
    Path::new(path)
        .extension()
        .and_then(|ext| ext.to_str())
        .map_or("application/octet-stream".to_string(), get_mime_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_mime_type() {
        assert_eq!(get_mime_type("html"), "text/html".to_string());
        assert_eq!(get_mime_type("htm"), "text/html".to_string());
        assert_eq!(get_mime_type("css"), "text/css".to_string());
        assert_eq!(get_mime_type("js"), "application/javascript".to_string());
        assert_eq!(get_mime_type("mjs"), "application/javascript".to_string());
        assert_eq!(get_mime_type("json"), "application/json".to_string());
        assert_eq!(get_mime_type("xml"), "application/xml".to_string());
        assert_eq!(get_mime_type("pdf"), "application/pdf".to_string());
        assert_eq!(get_mime_type("zip"), "application/zip".to_string());
        assert_eq!(get_mime_type("tar"), "application/x-tar".to_string());
        assert_eq!(get_mime_type("gz"), "application/gzip".to_string());
        assert_eq!(get_mime_type("bz2"), "application/x-bzip2".to_string());
        assert_eq!(get_mime_type("7z"), "application/x-7z-compressed".to_string());
        assert_eq!(get_mime_type("rar"), "application/vnd.rar".to_string());
        assert_eq!(get_mime_type("exe"), "application/vnd.microsoft.portable-executable".to_string());
        assert_eq!(get_mime_type("msi"), "application/x-msdownload".to_string());
        assert_eq!(get_mime_type("bin"), "application/octet-stream".to_string());
        assert_eq!(get_mime_type("dll"), "application/octet-stream".to_string());
        assert_eq!(get_mime_type("iso"), "application/octet-stream".to_string());
        assert_eq!(get_mime_type("dmg"), "application/octet-stream".to_string());
        assert_eq!(get_mime_type("png"), "image/png".to_string());
        assert_eq!(get_mime_type("jpg"), "image/jpeg".to_string());
        assert_eq!(get_mime_type("jpeg"), "image/jpeg".to_string());
        assert_eq!(get_mime_type("gif"), "image/gif".to_string());
        assert_eq!(get_mime_type("svg"), "image/svg+xml".to_string());
        assert_eq!(get_mime_type("bmp"), "image/bmp".to_string());
        assert_eq!(get_mime_type("ico"), "image/x-icon".to_string());
        assert_eq!(get_mime_type("tiff"), "image/tiff".to_string());
        assert_eq!(get_mime_type("tif"), "image/tiff".to_string());
        assert_eq!(get_mime_type("webp"), "image/webp".to_string());
        assert_eq!(get_mime_type("mp3"), "audio/mpeg".to_string());
        assert_eq!(get_mime_type("wav"), "audio/wav".to_string());
        assert_eq!(get_mime_type("ogg"), "audio/ogg".to_string());
        assert_eq!(get_mime_type("flac"), "audio/flac".to_string());
        assert_eq!(get_mime_type("aac"), "audio/aac".to_string());
        assert_eq!(get_mime_type("mp4"), "video/mp4".to_string());
        assert_eq!(get_mime_type("m4v"), "video/x-m4v".to_string());
        assert_eq!(get_mime_type("mkv"), "video/x-matroska".to_string());
        assert_eq!(get_mime_type("webm"), "video/webm".to_string());
        assert_eq!(get_mime_type("avi"), "video/x-msvideo".to_string());
        assert_eq!(get_mime_type("mov"), "video/quicktime".to_string());
        assert_eq!(get_mime_type("wmv"), "video/x-ms-wmv".to_string());
        assert_eq!(get_mime_type("txt"), "text/plain".to_string());
        assert_eq!(get_mime_type("csv"), "text/csv".to_string());
        assert_eq!(get_mime_type("md"), "text/markdown".to_string());
        assert_eq!(get_mime_type("rtf"), "application/rtf".to_string());
        assert_eq!(get_mime_type("odt"), "application/vnd.oasis.opendocument.text".to_string());
        assert_eq!(get_mime_type("ods"), "application/vnd.oasis.opendocument.spreadsheet".to_string());
        assert_eq!(get_mime_type("odp"), "application/vnd.oasis.opendocument.presentation".to_string());
        assert_eq!(get_mime_type("doc"), "application/msword".to_string());
        assert_eq!(get_mime_type("docx"), "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string());
        assert_eq!(get_mime_type("xls"), "application/vnd.ms-excel".to_string());
        assert_eq!(get_mime_type("xlsx"), "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet".to_string());
        assert_eq!(get_mime_type("ppt"), "application/vnd.ms-powerpoint".to_string());
        assert_eq!(get_mime_type("pptx"), "application/vnd.openxmlformats-officedocument.presentationml.presentation".to_string());
        assert_eq!(get_mime_type("wasm"), "application/wasm".to_string()); // Test for WebAssembly MIME type
        assert_eq!(get_mime_type("unknown"), "application/octet-stream".to_string());
    }

    #[test]
    fn test_get_mime_type_from_path() {
        assert_eq!(get_mime_type_from_path("index.html"), "text/html".to_string());
        assert_eq!(get_mime_type_from_path("style.css"), "text/css".to_string());
        assert_eq!(get_mime_type_from_path("script.js"), "application/javascript".to_string());
        assert_eq!(get_mime_type_from_path("image.png"), "image/png".to_string());
        assert_eq!(get_mime_type_from_path("unknownfile"), "application/octet-stream".to_string());
        assert_eq!(get_mime_type_from_path("document.pdf"), "application/pdf".to_string());
        assert_eq!(get_mime_type_from_path("archive.tar.gz"), "application/gzip".to_string());
        assert_eq!(get_mime_type_from_path("audio.mp3"), "audio/mpeg".to_string());
        assert_eq!(get_mime_type_from_path("video.mp4"), "video/mp4".to_string());
        assert_eq!(get_mime_type_from_path("module.wasm"), "application/wasm".to_string()); // Test for WebAssembly MIME type from path
    }
}
