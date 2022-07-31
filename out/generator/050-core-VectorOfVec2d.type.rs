pub type VectorOfVec2d = core::Vector<core::Vec2d>;

impl VectorOfVec2d {
	pub fn as_raw_VectorOfVec2d(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVec2d(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vec2d, *const c_void, *mut c_void,
	cv_VectorOfVec2d_new, cv_VectorOfVec2d_delete,
	cv_VectorOfVec2d_len, cv_VectorOfVec2d_is_empty,
	cv_VectorOfVec2d_capacity, cv_VectorOfVec2d_resize,
	cv_VectorOfVec2d_shrink_to_fit,
	cv_VectorOfVec2d_reserve, cv_VectorOfVec2d_remove,
	cv_VectorOfVec2d_swap, cv_VectorOfVec2d_clear,
	cv_VectorOfVec2d_get, cv_VectorOfVec2d_set,
	cv_VectorOfVec2d_push, cv_VectorOfVec2d_insert,
}
vector_copy_non_bool! { core::Vec2d, *const c_void, *mut c_void,
	cv_VectorOfVec2d_data, cv_VectorOfVec2d_data_mut, cv_VectorOfVec2d_from_slice,
	cv_VectorOfVec2d_clone,
}

unsafe impl Send for core::Vector<core::Vec2d> {}

impl core::ToInputArray for VectorOfVec2d {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfVec2d_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVec2d_input_array(self.as_raw_VectorOfVec2d(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfVec2d {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfVec2d_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVec2d_output_array(self.as_raw_mut_VectorOfVec2d(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfVec2d {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfVec2d_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVec2d_input_output_array(self.as_raw_mut_VectorOfVec2d(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfVec2d }

