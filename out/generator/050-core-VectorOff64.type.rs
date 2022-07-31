pub type VectorOff64 = core::Vector<f64>;

impl VectorOff64 {
	pub fn as_raw_VectorOff64(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOff64(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { f64, *const c_void, *mut c_void,
	cv_VectorOff64_new, cv_VectorOff64_delete,
	cv_VectorOff64_len, cv_VectorOff64_is_empty,
	cv_VectorOff64_capacity, cv_VectorOff64_resize,
	cv_VectorOff64_shrink_to_fit,
	cv_VectorOff64_reserve, cv_VectorOff64_remove,
	cv_VectorOff64_swap, cv_VectorOff64_clear,
	cv_VectorOff64_get, cv_VectorOff64_set,
	cv_VectorOff64_push, cv_VectorOff64_insert,
}
vector_copy_non_bool! { f64, *const c_void, *mut c_void,
	cv_VectorOff64_data, cv_VectorOff64_data_mut, cv_VectorOff64_from_slice,
	cv_VectorOff64_clone,
}

unsafe impl Send for core::Vector<f64> {}

impl core::ToInputArray for VectorOff64 {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOff64_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOff64_input_array(self.as_raw_VectorOff64(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOff64 {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOff64_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOff64_output_array(self.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOff64 {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOff64_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOff64_input_output_array(self.as_raw_mut_VectorOff64(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOff64 }

