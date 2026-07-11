use serde::Serialize;
use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
use wasm_bindgen::describe::WasmDescribe;

pub struct OutputJson<T: Serialize>(pub(crate) T);

impl<T: Serialize> IntoWasmAbi for OutputJson<T> {
    type Abi = <Vec<u8> as FromWasmAbi>::Abi;

    fn into_abi(self) -> Self::Abi {
        let OutputJson(value) = self;
        match serde_json::to_string(&value) {
            Ok(string) => string.into_abi(),
            Err(error) => error.to_string().into_abi()
        }
    }
}

impl<T: Serialize> WasmDescribe for OutputJson<T> {
    fn describe() {
        todo!()
    }
}

// TODO implement FromResidual

// TODO work on this. call serde deser functions on T for to and from abi