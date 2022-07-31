pub type VectorOfVectorOfPoint2f = core::Vector<core::Vector<core::Point2f>>;

impl VectorOfVectorOfPoint2f {
	pub fn as_raw_VectorOfVectorOfPoint2f(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfPoint2f(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::Point2f>, *const c_void, *mut c_void,
	cv_VectorOfVectorOfPoint2f_new, cv_VectorOfVectorOfPoint2f_delete,
	cv_VectorOfVectorOfPoint2f_len, cv_VectorOfVectorOfPoint2f_is_empty,
	cv_VectorOfVectorOfPoint2f_capacity, cv_VectorOfVectorOfPoint2f_resize,
	cv_VectorOfVectorOfPoint2f_shrink_to_fit,
	cv_VectorOfVectorOfPoint2f_reserve, cv_VectorOfVectorOfPoint2f_remove,
	cv_VectorOfVectorOfPoint2f_swap, cv_VectorOfVectorOfPoint2f_clear,
	cv_VectorOfVectorOfPoint2f_get, cv_VectorOfVectorOfPoint2f_set,
	cv_VectorOfVectorOfPoint2f_push, cv_VectorOfVectorOfPoint2f_insert,
}
vector_non_copy_or_bool! { clone core::Vector<core::Point2f> }

unsafe impl Send for core::Vector<core::Vector<core::Point2f>> {}

impl core::ToInputArray for VectorOfVectorOfPoint2f {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfVectorOfPoint2f_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfPoint2f_input_array(self.as_raw_VectorOfVectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfVectorOfPoint2f {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfVectorOfPoint2f_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfPoint2f_output_array(self.as_raw_mut_VectorOfVectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfVectorOfPoint2f {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfVectorOfPoint2f_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfPoint2f_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint2f(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfVectorOfPoint2f }

