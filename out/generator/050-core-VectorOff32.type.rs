pub type VectorOff32 = core::Vector<f32>;

impl VectorOff32 {
	pub fn as_raw_VectorOff32(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOff32(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { f32, *const c_void, *mut c_void,
	cv_VectorOff32_new, cv_VectorOff32_delete,
	cv_VectorOff32_len, cv_VectorOff32_is_empty,
	cv_VectorOff32_capacity, cv_VectorOff32_resize,
	cv_VectorOff32_shrink_to_fit,
	cv_VectorOff32_reserve, cv_VectorOff32_remove,
	cv_VectorOff32_swap, cv_VectorOff32_clear,
	cv_VectorOff32_get, cv_VectorOff32_set,
	cv_VectorOff32_push, cv_VectorOff32_insert,
}
vector_copy_non_bool! { f32, *const c_void, *mut c_void,
	cv_VectorOff32_data, cv_VectorOff32_data_mut, cv_VectorOff32_from_slice,
	cv_VectorOff32_clone,
}

unsafe impl Send for core::Vector<f32> {}

impl core::ToInputArray for VectorOff32 {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOff32_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOff32_input_array(self.as_raw_VectorOff32(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOff32 {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOff32_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOff32_output_array(self.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOff32 {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOff32_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOff32_input_output_array(self.as_raw_mut_VectorOff32(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOff32 }

