use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;

#[link(name = "crypt")]
extern "C" {
    fn crypt(_: *const c_char, _: *const c_char) -> *const c_char;
}

pub fn c_crypt(password: &str, salt: &str) -> String {
    unsafe {
        let hash = crypt(
            CString::new(password).unwrap().as_ptr() as *const c_char,
            CString::new(salt).unwrap().as_ptr() as *const c_char
        );
        let hash: &CStr = CStr::from_ptr(hash);
        let hash: String = hash.to_str().unwrap().to_string();
        hash
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c_crypt_test() {
        assert_eq!(
            c_crypt("hello", "$6$salty"), 
            "$6$salty$DYOO4U869mvL5FZYB26Qk2JMtJJzLLbowdeqR50BjVVFcZMvZsEu9hICfEKtXe8q6JvoMlWZiUhNDG1wRaliq.".to_string());
    }
}
