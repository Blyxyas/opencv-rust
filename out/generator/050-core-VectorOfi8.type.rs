pub type VectorOfi8 = core::Vector<i8>;

impl VectorOfi8 {
	pub fn as_raw_VectorOfi8(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfi8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { i8, *const c_void, *mut c_void,
	cv_VectorOfi8_new, cv_VectorOfi8_delete,
	cv_VectorOfi8_len, cv_VectorOfi8_is_empty,
	cv_VectorOfi8_capacity, cv_VectorOfi8_resize,
	cv_VectorOfi8_shrink_to_fit,
	cv_VectorOfi8_reserve, cv_VectorOfi8_remove,
	cv_VectorOfi8_swap, cv_VectorOfi8_clear,
	cv_VectorOfi8_get, cv_VectorOfi8_set,
	cv_VectorOfi8_push, cv_VectorOfi8_insert,
}
vector_copy_non_bool! { i8, *const c_void, *mut c_void,
	cv_VectorOfi8_data, cv_VectorOfi8_data_mut, cv_VectorOfi8_from_slice,
	cv_VectorOfi8_clone,
}

unsafe impl Send for core::Vector<i8> {}

impl core::ToInputArray for VectorOfi8 {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfi8_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfi8_input_array(self.as_raw_VectorOfi8(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfi8 {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfi8_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfi8_output_array(self.as_raw_mut_VectorOfi8(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfi8 {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfi8_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfi8_input_output_array(self.as_raw_mut_VectorOfi8(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfi8 }

