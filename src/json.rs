use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi};
use wasm_bindgen::describe::WasmDescribe;

pub struct Json<T>(pub(crate) T);

impl<T> IntoWasmAbi for Json<T> {
    type Abi = ();

    fn into_abi(self) -> Self::Abi {
        todo!()
    }
}

impl<T> FromWasmAbi for Json<T> {
    type Abi = ();

    unsafe fn from_abi(_: Self::Abi) -> Self {
        todo!()
    }
}

impl<T> WasmDescribe for Json<T> {
    fn describe() {
        todo!()
    }
}