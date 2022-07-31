pub type VectorOfSize = core::Vector<core::Size>;

impl VectorOfSize {
	pub fn as_raw_VectorOfSize(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfSize(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Size, *const c_void, *mut c_void,
	cv_VectorOfSize_new, cv_VectorOfSize_delete,
	cv_VectorOfSize_len, cv_VectorOfSize_is_empty,
	cv_VectorOfSize_capacity, cv_VectorOfSize_resize,
	cv_VectorOfSize_shrink_to_fit,
	cv_VectorOfSize_reserve, cv_VectorOfSize_remove,
	cv_VectorOfSize_swap, cv_VectorOfSize_clear,
	cv_VectorOfSize_get, cv_VectorOfSize_set,
	cv_VectorOfSize_push, cv_VectorOfSize_insert,
}
vector_copy_non_bool! { core::Size, *const c_void, *mut c_void,
	cv_VectorOfSize_data, cv_VectorOfSize_data_mut, cv_VectorOfSize_from_slice,
	cv_VectorOfSize_clone,
}

unsafe impl Send for core::Vector<core::Size> {}

impl core::ToInputArray for VectorOfSize {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfSize_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfSize_input_array(self.as_raw_VectorOfSize(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfSize {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfSize_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfSize_output_array(self.as_raw_mut_VectorOfSize(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfSize {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfSize_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfSize_input_output_array(self.as_raw_mut_VectorOfSize(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfSize }

