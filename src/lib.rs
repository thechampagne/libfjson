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
use std::os::raw::c_char;
use std::ffi::CStr;
use std::ffi::CString;
use fjson::to_json;
use fjson::to_json_compact;
use fjson::to_jsonc;

#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(PartialEq)]
enum fjson_error_t {
    FJSON_ERROR_OK,
    FJSON_ERROR_RECURSION_LIMIT_EXCEEDED,
    FJSON_ERROR_UNEXPECTED_CHARACTER,
    FJSON_ERROR_UNEXPECTED_TOKEN,
    FJSON_ERROR_UNEXPECTED_EOF,
    FJSON_ERROR_WRITE,
    FJSON_ERROR_NOT_VALID_UTF8,
    FJSON_ERROR_NUL
}

#[no_mangle]
unsafe extern "C" fn fjson_to_json(input: *const c_char, err_out: *mut fjson_error_t) -> *mut c_char {
    let input_rs = match CStr::from_ptr(input).to_str() {
	Ok(str) => str,
	Err(_) => {
	    (*err_out) = fjson_error_t::FJSON_ERROR_NOT_VALID_UTF8;
	    return std::ptr::null_mut();
	}
    };
    match to_json(input_rs) {
	Ok(json_str) => match CString::new(json_str) {
	    Ok(cstr) => {
		(*err_out) = fjson_error_t::FJSON_ERROR_OK;
		cstr.into_raw()
	    },
	    Err(_) => {
		(*err_out) = fjson_error_t::FJSON_ERROR_NUL;
		std::ptr::null_mut()
	    }
	},
	Err(err) => match err {
	    fjson::error::Error::RecursionLimitExceeded => {
		(*err_out) = fjson_error_t::FJSON_ERROR_RECURSION_LIMIT_EXCEEDED;
		std::ptr::null_mut()
	    },
	    fjson::error::Error::UnexpectedCharacter(_, _) => {
		(*err_out) = fjson_error_t::FJSON_ERROR_UNEXPECTED_CHARACTER;
		std::ptr::null_mut()
	    },
	    fjson::error::Error::UnexpectedToken(_, _) => {
		(*err_out) = fjson_error_t::FJSON_ERROR_UNEXPECTED_TOKEN;
		std::ptr::null_mut()
	    },
	    fjson::error::Error::UnexpectedEOF => {
		(*err_out) = fjson_error_t::FJSON_ERROR_UNEXPECTED_EOF;
		std::ptr::null_mut()
	    },
	    fjson::error::Error::Write(_) => {
		(*err_out) = fjson_error_t::FJSON_ERROR_WRITE;
		std::ptr::null_mut()
	    },
	}
    }
}

#[no_mangle]
unsafe extern "C" fn fjson_to_json_compact(input: *const c_char, err_out: *mut fjson_error_t) -> *mut c_char {
    let input_rs = match CStr::from_ptr(input).to_str() {
	Ok(str) => str,
	Err(_) => {
	    (*err_out) = fjson_error_t::FJSON_ERROR_NOT_VALID_UTF8;
	    return std::ptr::null_mut();
	}
    };
    match to_json_compact(input_rs) {
	Ok(json_str) => match CString::new(json_str) {
	    Ok(cstr) => {
		(*err_out) = fjson_error_t::FJSON_ERROR_OK;
		cstr.into_raw()
	    },
	    Err(_) => {
		(*err_out) = fjson_error_t::FJSON_ERROR_NUL;
		std::ptr::null_mut()
	    }
	},
	Err(err) => match err {
	    fjson::error::Error::RecursionLimitExceeded => {
		(*err_out) = fjson_error_t::FJSON_ERROR_RECURSION_LIMIT_EXCEEDED;
		std::ptr::null_mut()
	    },
	    fjson::error::Error::UnexpectedCharacter(_, _) => {
		(*err_out) = fjson_error_t::FJSON_ERROR_UNEXPECTED_CHARACTER;
		std::ptr::null_mut()
	    },
	    fjson::error::Error::UnexpectedToken(_, _) => {
		(*err_out) = fjson_error_t::FJSON_ERROR_UNEXPECTED_TOKEN;
		std::ptr::null_mut()
	    },
	    fjson::error::Error::UnexpectedEOF => {
		(*err_out) = fjson_error_t::FJSON_ERROR_UNEXPECTED_EOF;
		std::ptr::null_mut()
	    },
	    fjson::error::Error::Write(_) => {
		(*err_out) = fjson_error_t::FJSON_ERROR_WRITE;
		std::ptr::null_mut()
	    },
	}
    }
}

#[no_mangle]
unsafe extern "C" fn fjson_to_jsonc(input: *const c_char, err_out: *mut fjson_error_t) -> *mut c_char {
    let input_rs = match CStr::from_ptr(input).to_str() {
	Ok(str) => str,
	Err(_) => {
	    (*err_out) = fjson_error_t::FJSON_ERROR_NOT_VALID_UTF8;
	    return std::ptr::null_mut();
	}
    };
    match to_jsonc(input_rs) {
	Ok(json_str) => match CString::new(json_str) {
	    Ok(cstr) => {
		(*err_out) = fjson_error_t::FJSON_ERROR_OK;
		cstr.into_raw()
	    },
	    Err(_) => {
		(*err_out) = fjson_error_t::FJSON_ERROR_NUL;
		std::ptr::null_mut()
	    }
	},
	Err(err) => match err {
	    fjson::error::Error::RecursionLimitExceeded => {
		(*err_out) = fjson_error_t::FJSON_ERROR_RECURSION_LIMIT_EXCEEDED;
		std::ptr::null_mut()
	    },
	    fjson::error::Error::UnexpectedCharacter(_, _) => {
		(*err_out) = fjson_error_t::FJSON_ERROR_UNEXPECTED_CHARACTER;
		std::ptr::null_mut()
	    },
	    fjson::error::Error::UnexpectedToken(_, _) => {
		(*err_out) = fjson_error_t::FJSON_ERROR_UNEXPECTED_TOKEN;
		std::ptr::null_mut()
	    },
	    fjson::error::Error::UnexpectedEOF => {
		(*err_out) = fjson_error_t::FJSON_ERROR_UNEXPECTED_EOF;
		std::ptr::null_mut()
	    },
	    fjson::error::Error::Write(_) => {
		(*err_out) = fjson_error_t::FJSON_ERROR_WRITE;
		std::ptr::null_mut()
	    },
	}
    }
}

mod tests {

    use crate::*;

    #[allow(non_upper_case_globals)]
    const input: &'static str = "// This is a JSON value with comments and trailing commas
{

    /* The project name is fjson */
    \"project\": \"fjson\",
    \"language\": \"Rust\",
    \"license\": [
        \"MIT\",
    ],


  // This project is public.
    \"public\": true,
}\0";

    #[test]
    fn to_jsonc() {
	unsafe {
	    let mut err: fjson_error_t = fjson_error_t::FJSON_ERROR_OK;
	    let output = fjson_to_jsonc(input.as_ptr() as *const c_char, &mut err as *mut fjson_error_t);
	    assert!(err == fjson_error_t::FJSON_ERROR_OK);
	    assert!(!output.is_null());
	}
    }

    #[test]
    fn to_json() {
	unsafe {
	    let mut err: fjson_error_t = fjson_error_t::FJSON_ERROR_OK;
	    let output = fjson_to_json(input.as_ptr() as *const c_char, &mut err as *mut fjson_error_t);
	    assert!(err == fjson_error_t::FJSON_ERROR_OK);
	    assert!(!output.is_null());
	}
    }

    #[test]
    fn to_json_compact() {
	unsafe {
	    let mut err: fjson_error_t = fjson_error_t::FJSON_ERROR_OK;
	    let output = fjson_to_json_compact(input.as_ptr() as *const c_char, &mut err as *mut fjson_error_t);
	    assert!(err == fjson_error_t::FJSON_ERROR_OK);
	    assert!(!output.is_null());
	}
    }
}
