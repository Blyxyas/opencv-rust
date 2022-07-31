pub type VectorOfVec4f = core::Vector<core::Vec4f>;

impl VectorOfVec4f {
	pub fn as_raw_VectorOfVec4f(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVec4f(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vec4f, *const c_void, *mut c_void,
	cv_VectorOfVec4f_new, cv_VectorOfVec4f_delete,
	cv_VectorOfVec4f_len, cv_VectorOfVec4f_is_empty,
	cv_VectorOfVec4f_capacity, cv_VectorOfVec4f_resize,
	cv_VectorOfVec4f_shrink_to_fit,
	cv_VectorOfVec4f_reserve, cv_VectorOfVec4f_remove,
	cv_VectorOfVec4f_swap, cv_VectorOfVec4f_clear,
	cv_VectorOfVec4f_get, cv_VectorOfVec4f_set,
	cv_VectorOfVec4f_push, cv_VectorOfVec4f_insert,
}
vector_copy_non_bool! { core::Vec4f, *const c_void, *mut c_void,
	cv_VectorOfVec4f_data, cv_VectorOfVec4f_data_mut, cv_VectorOfVec4f_from_slice,
	cv_VectorOfVec4f_clone,
}

unsafe impl Send for core::Vector<core::Vec4f> {}

impl core::ToInputArray for VectorOfVec4f {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfVec4f_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVec4f_input_array(self.as_raw_VectorOfVec4f(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfVec4f {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfVec4f_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVec4f_output_array(self.as_raw_mut_VectorOfVec4f(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfVec4f {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfVec4f_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVec4f_input_output_array(self.as_raw_mut_VectorOfVec4f(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfVec4f }

