pub type VectorOfPoint2f = core::Vector<core::Point2f>;

impl VectorOfPoint2f {
	pub fn as_raw_VectorOfPoint2f(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfPoint2f(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Point2f, *const c_void, *mut c_void,
	cv_VectorOfPoint2f_new, cv_VectorOfPoint2f_delete,
	cv_VectorOfPoint2f_len, cv_VectorOfPoint2f_is_empty,
	cv_VectorOfPoint2f_capacity, cv_VectorOfPoint2f_resize,
	cv_VectorOfPoint2f_shrink_to_fit,
	cv_VectorOfPoint2f_reserve, cv_VectorOfPoint2f_remove,
	cv_VectorOfPoint2f_swap, cv_VectorOfPoint2f_clear,
	cv_VectorOfPoint2f_get, cv_VectorOfPoint2f_set,
	cv_VectorOfPoint2f_push, cv_VectorOfPoint2f_insert,
}
vector_copy_non_bool! { core::Point2f, *const c_void, *mut c_void,
	cv_VectorOfPoint2f_data, cv_VectorOfPoint2f_data_mut, cv_VectorOfPoint2f_from_slice,
	cv_VectorOfPoint2f_clone,
}

unsafe impl Send for core::Vector<core::Point2f> {}

impl core::ToInputArray for VectorOfPoint2f {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfPoint2f_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfPoint2f_input_array(self.as_raw_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfPoint2f {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfPoint2f_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfPoint2f_output_array(self.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfPoint2f {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfPoint2f_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfPoint2f_input_output_array(self.as_raw_mut_VectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfPoint2f }

