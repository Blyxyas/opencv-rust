pub type VectorOfi32 = core::Vector<i32>;

impl VectorOfi32 {
	pub fn as_raw_VectorOfi32(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfi32(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { i32, *const c_void, *mut c_void,
	cv_VectorOfi32_new, cv_VectorOfi32_delete,
	cv_VectorOfi32_len, cv_VectorOfi32_is_empty,
	cv_VectorOfi32_capacity, cv_VectorOfi32_resize,
	cv_VectorOfi32_shrink_to_fit,
	cv_VectorOfi32_reserve, cv_VectorOfi32_remove,
	cv_VectorOfi32_swap, cv_VectorOfi32_clear,
	cv_VectorOfi32_get, cv_VectorOfi32_set,
	cv_VectorOfi32_push, cv_VectorOfi32_insert,
}
vector_copy_non_bool! { i32, *const c_void, *mut c_void,
	cv_VectorOfi32_data, cv_VectorOfi32_data_mut, cv_VectorOfi32_from_slice,
	cv_VectorOfi32_clone,
}

unsafe impl Send for core::Vector<i32> {}

impl core::ToInputArray for VectorOfi32 {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfi32_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfi32_input_array(self.as_raw_VectorOfi32(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfi32 {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfi32_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfi32_output_array(self.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfi32 {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfi32_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfi32_input_output_array(self.as_raw_mut_VectorOfi32(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfi32 }

