# Suika MIME

Suika MIME is a Rust library that provides functionality for determining the
MIME type of files based on their extensions or file paths. This library is
designed to be simple, efficient, and easy to use.

## Features

- Determine MIME type from file extension.
- Determine MIME type from file path.
- Comprehensive support for common file types.
- Extensible and easy to integrate into existing projects.

## Installation

To use Suika MIME in your Rust project, add the following to your `Cargo.toml`:

```toml
[dependencies]
suika_mime = "0.1.0"
```

## Usage

### Determining MIME Type from Extension

You can determine the MIME type based on a file extension using the
`get_mime_type` function.

```rust
use suika_mime::get_mime_type;

let mime_type = get_mime_type("html");
assert_eq!(mime_type, "text/html".to_string());

let mime_type = get_mime_type("png");
assert_eq!(mime_type, "image/png".to_string());
```

### Determining MIME Type from File Path

You can determine the MIME type based on a file path using the
`get_mime_type_from_path` function.

```rust
use suika_mime::get_mime_type_from_path;

let mime_type = get_mime_type_from_path("index.html");
assert_eq!(mime_type, "text/html".to_string());

let mime_type = get_mime_type_from_path("images/photo.png");
assert_eq!(mime_type, "image/png".to_string());
```

## Supported MIME Types

Suika MIME supports a wide range of file types. Here are some of the supported
extensions and their corresponding MIME types:

- `html`, `htm`: `text/html`
- `css`: `text/css`
- `js`, `mjs`: `application/javascript`
- `json`: `application/json`
- `xml`: `application/xml`
- `pdf`: `application/pdf`
- `zip`: `application/zip`
- `tar`: `application/x-tar`
- `gz`: `application/gzip`
- `bz2`: `application/x-bzip2`
- `7z`: `application/x-7z-compressed`
- `rar`: `application/vnd.rar`
- `exe`: `application/vnd.microsoft.portable-executable`
- `msi`: `application/x-msdownload`
- `bin`, `dll`, `iso`, `dmg`: `application/octet-stream`
- `png`: `image/png`
- `jpg`, `jpeg`: `image/jpeg`
- `gif`: `image/gif`
- `svg`: `image/svg+xml`
- `bmp`: `image/bmp`
- `ico`: `image/x-icon`
- `tiff`, `tif`: `image/tiff`
- `webp`: `image/webp`
- `mp3`: `audio/mpeg`
- `wav`: `audio/wav`
- `ogg`: `audio/ogg`
- `flac`: `audio/flac`
- `aac`: `audio/aac`
- `mp4`: `video/mp4`
- `m4v`: `video/x-m4v`
- `mkv`: `video/x-matroska`
- `webm`: `video/webm`
- `avi`: `video/x-msvideo`
- `mov`: `video/quicktime`
- `wmv`: `video/x-ms-wmv`
- `txt`: `text/plain`
- `csv`: `text/csv`
- `md`: `text/markdown`
- `rtf`: `application/rtf`
- `odt`: `application/vnd.oasis.opendocument.text`
- `ods`: `application/vnd.oasis.opendocument.spreadsheet`
- `odp`: `application/vnd.oasis.opendocument.presentation`
- `doc`: `application/msword`
- `docx`:
  `application/vnd.openxmlformats-officedocument.wordprocessingml.document`
- `xls`: `application/vnd.ms-excel`
- `xlsx`: `application/vnd.openxmlformats-officedocument.spreadsheetml.sheet`
- `ppt`: `application/vnd.ms-powerpoint`
- `pptx`:
  `application/vnd.openxmlformats-officedocument.presentationml.presentation`
