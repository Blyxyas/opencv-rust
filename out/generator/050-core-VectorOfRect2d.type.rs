pub type VectorOfRect2d = core::Vector<core::Rect2d>;

impl VectorOfRect2d {
	pub fn as_raw_VectorOfRect2d(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfRect2d(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Rect2d, *const c_void, *mut c_void,
	cv_VectorOfRect2d_new, cv_VectorOfRect2d_delete,
	cv_VectorOfRect2d_len, cv_VectorOfRect2d_is_empty,
	cv_VectorOfRect2d_capacity, cv_VectorOfRect2d_resize,
	cv_VectorOfRect2d_shrink_to_fit,
	cv_VectorOfRect2d_reserve, cv_VectorOfRect2d_remove,
	cv_VectorOfRect2d_swap, cv_VectorOfRect2d_clear,
	cv_VectorOfRect2d_get, cv_VectorOfRect2d_set,
	cv_VectorOfRect2d_push, cv_VectorOfRect2d_insert,
}
vector_copy_non_bool! { core::Rect2d, *const c_void, *mut c_void,
	cv_VectorOfRect2d_data, cv_VectorOfRect2d_data_mut, cv_VectorOfRect2d_from_slice,
	cv_VectorOfRect2d_clone,
}

unsafe impl Send for core::Vector<core::Rect2d> {}

impl core::ToInputArray for VectorOfRect2d {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfRect2d_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfRect2d_input_array(self.as_raw_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfRect2d {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfRect2d_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfRect2d_output_array(self.as_raw_mut_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfRect2d {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfRect2d_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfRect2d_input_output_array(self.as_raw_mut_VectorOfRect2d(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfRect2d }

