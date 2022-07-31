pub type VectorOfVec2f = core::Vector<core::Vec2f>;

impl VectorOfVec2f {
	pub fn as_raw_VectorOfVec2f(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVec2f(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vec2f, *const c_void, *mut c_void,
	cv_VectorOfVec2f_new, cv_VectorOfVec2f_delete,
	cv_VectorOfVec2f_len, cv_VectorOfVec2f_is_empty,
	cv_VectorOfVec2f_capacity, cv_VectorOfVec2f_resize,
	cv_VectorOfVec2f_shrink_to_fit,
	cv_VectorOfVec2f_reserve, cv_VectorOfVec2f_remove,
	cv_VectorOfVec2f_swap, cv_VectorOfVec2f_clear,
	cv_VectorOfVec2f_get, cv_VectorOfVec2f_set,
	cv_VectorOfVec2f_push, cv_VectorOfVec2f_insert,
}
vector_copy_non_bool! { core::Vec2f, *const c_void, *mut c_void,
	cv_VectorOfVec2f_data, cv_VectorOfVec2f_data_mut, cv_VectorOfVec2f_from_slice,
	cv_VectorOfVec2f_clone,
}

unsafe impl Send for core::Vector<core::Vec2f> {}

impl core::ToInputArray for VectorOfVec2f {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfVec2f_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVec2f_input_array(self.as_raw_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfVec2f {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfVec2f_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVec2f_output_array(self.as_raw_mut_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfVec2f {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfVec2f_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVec2f_input_output_array(self.as_raw_mut_VectorOfVec2f(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfVec2f }

