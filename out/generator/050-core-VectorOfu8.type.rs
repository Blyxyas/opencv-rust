pub type VectorOfu8 = core::Vector<u8>;

impl VectorOfu8 {
	pub fn as_raw_VectorOfu8(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfu8(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { u8, *const c_void, *mut c_void,
	cv_VectorOfu8_new, cv_VectorOfu8_delete,
	cv_VectorOfu8_len, cv_VectorOfu8_is_empty,
	cv_VectorOfu8_capacity, cv_VectorOfu8_resize,
	cv_VectorOfu8_shrink_to_fit,
	cv_VectorOfu8_reserve, cv_VectorOfu8_remove,
	cv_VectorOfu8_swap, cv_VectorOfu8_clear,
	cv_VectorOfu8_get, cv_VectorOfu8_set,
	cv_VectorOfu8_push, cv_VectorOfu8_insert,
}
vector_copy_non_bool! { u8, *const c_void, *mut c_void,
	cv_VectorOfu8_data, cv_VectorOfu8_data_mut, cv_VectorOfu8_from_slice,
	cv_VectorOfu8_clone,
}

unsafe impl Send for core::Vector<u8> {}

impl core::ToInputArray for VectorOfu8 {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfu8_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfu8_input_array(self.as_raw_VectorOfu8(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfu8 {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfu8_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfu8_output_array(self.as_raw_mut_VectorOfu8(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfu8 {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfu8_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfu8_input_output_array(self.as_raw_mut_VectorOfu8(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfu8 }

