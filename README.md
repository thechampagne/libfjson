# libfjson

[![](https://img.shields.io/github/v/tag/thechampagne/libfjson?label=version)](https://github.com/thechampagne/libfjson/releases/latest) [![](https://img.shields.io/github/license/thechampagne/libfjson)](https://github.com/thechampagne/libfjson/blob/main/LICENSE)

A **C** library for parsing and formatting json with C-style comments and trailing commas.

### Installation & Setup

#### 1. Clone the repository
```
git clone https://github.com/thechampagne/libfjson.git
```
#### 2. Navigate to the root
```
cd libfjson
```
#### 3. Build the project
```
cargo build
```

### API
```c
typedef enum {
    FJSON_ERROR_OK,
    FJSON_ERROR_RECURSION_LIMIT_EXCEEDED,
    FJSON_ERROR_UNEXPECTED_CHARACTER,
    FJSON_ERROR_UNEXPECTED_TOKEN,
    FJSON_ERROR_UNEXPECTED_EOF,
    FJSON_ERROR_WRITE,
    FJSON_ERROR_NOT_VALID_UTF8,
    FJSON_ERROR_NUL
} fjson_error_t;

char* fjson_to_json(const char* input, fjson_error_t* err_out);

char* fjson_to_json_compact(const char* input, fjson_error_t* err_out);

char* fjson_to_jsonc(const char* input, fjson_error_t* err_out);
```

### References
 - [fjson](https://github.com/ryanfowler/fjson)

### License

This repo is released under the [MIT](https://github.com/thechampagne/libfjson/blob/main/LICENSE).
