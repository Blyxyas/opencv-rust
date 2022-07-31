pub type VectorOfPoint2d = core::Vector<core::Point2d>;

impl VectorOfPoint2d {
	pub fn as_raw_VectorOfPoint2d(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfPoint2d(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Point2d, *const c_void, *mut c_void,
	cv_VectorOfPoint2d_new, cv_VectorOfPoint2d_delete,
	cv_VectorOfPoint2d_len, cv_VectorOfPoint2d_is_empty,
	cv_VectorOfPoint2d_capacity, cv_VectorOfPoint2d_resize,
	cv_VectorOfPoint2d_shrink_to_fit,
	cv_VectorOfPoint2d_reserve, cv_VectorOfPoint2d_remove,
	cv_VectorOfPoint2d_swap, cv_VectorOfPoint2d_clear,
	cv_VectorOfPoint2d_get, cv_VectorOfPoint2d_set,
	cv_VectorOfPoint2d_push, cv_VectorOfPoint2d_insert,
}
vector_copy_non_bool! { core::Point2d, *const c_void, *mut c_void,
	cv_VectorOfPoint2d_data, cv_VectorOfPoint2d_data_mut, cv_VectorOfPoint2d_from_slice,
	cv_VectorOfPoint2d_clone,
}

unsafe impl Send for core::Vector<core::Point2d> {}

impl core::ToInputArray for VectorOfPoint2d {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfPoint2d_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfPoint2d_input_array(self.as_raw_VectorOfPoint2d(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfPoint2d {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfPoint2d_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfPoint2d_output_array(self.as_raw_mut_VectorOfPoint2d(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfPoint2d {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfPoint2d_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfPoint2d_input_output_array(self.as_raw_mut_VectorOfPoint2d(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfPoint2d }

