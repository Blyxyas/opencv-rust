pub type VectorOfBarcodeType = core::Vector<crate::barcode::BarcodeType>;

impl VectorOfBarcodeType {
	pub fn as_raw_VectorOfBarcodeType(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfBarcodeType(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { crate::barcode::BarcodeType, *const c_void, *mut c_void,
	cv_VectorOfBarcodeType_new, cv_VectorOfBarcodeType_delete,
	cv_VectorOfBarcodeType_len, cv_VectorOfBarcodeType_is_empty,
	cv_VectorOfBarcodeType_capacity, cv_VectorOfBarcodeType_resize,
	cv_VectorOfBarcodeType_shrink_to_fit,
	cv_VectorOfBarcodeType_reserve, cv_VectorOfBarcodeType_remove,
	cv_VectorOfBarcodeType_swap, cv_VectorOfBarcodeType_clear,
	cv_VectorOfBarcodeType_get, cv_VectorOfBarcodeType_set,
	cv_VectorOfBarcodeType_push, cv_VectorOfBarcodeType_insert,
}
vector_copy_non_bool! { crate::barcode::BarcodeType, *const c_void, *mut c_void,
	cv_VectorOfBarcodeType_data, cv_VectorOfBarcodeType_data_mut, cv_VectorOfBarcodeType_from_slice,
	cv_VectorOfBarcodeType_clone,
}

unsafe impl Send for core::Vector<crate::barcode::BarcodeType> {}

