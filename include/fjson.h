/*
 * Copyright (c) 2023 XXIV
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
#ifndef __FJSON_H__
#define __FJSON_H__

#ifdef __cplusplus
extern "C" {
#endif

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

extern char* fjson_to_json(const char* input, fjson_error_t* err_out);

extern char* fjson_to_json_compact(const char* input, fjson_error_t* err_out);

extern char* fjson_to_jsonc(const char* input, fjson_error_t* err_out);

#ifdef __cplusplus
}
#endif

#endif // __FJSON_H__
