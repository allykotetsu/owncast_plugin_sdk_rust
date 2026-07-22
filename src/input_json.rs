use std::error::Error;
use serde::Deserialize;
use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::describe::WasmDescribe;

pub struct InputJson<T: for<'de> Deserialize<'de>>(pub(crate) Result<T, Box<dyn Error>>);

impl<T: for<'de> Deserialize<'de>> FromWasmAbi for InputJson<T> {
    type Abi = <Vec<u8> as FromWasmAbi>::Abi;

    unsafe fn from_abi(js: Self::Abi) -> InputJson<T> {
        unsafe {
            match String::from_utf8(<Vec<u8>>::from_abi(js)) {
                Ok(string) => {
                    match serde_json::from_str(string.as_str()) {
                        Ok(t) => InputJson(Ok(t)),
                        Err(error) => InputJson(Err(Box::new(error)))
                    }
                },
                Err(from_utf8_error) => InputJson(Err(Box::new(from_utf8_error)))
            }
        }
    }
}

impl<T: for<'de> Deserialize<'de>> WasmDescribe for InputJson<T> {
    fn describe() {
        String::describe()
    }
}