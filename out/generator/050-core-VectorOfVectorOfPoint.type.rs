pub type VectorOfVectorOfPoint = core::Vector<core::Vector<core::Point>>;

impl VectorOfVectorOfPoint {
	pub fn as_raw_VectorOfVectorOfPoint(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::Point>, *const c_void, *mut c_void,
	cv_VectorOfVectorOfPoint_new, cv_VectorOfVectorOfPoint_delete,
	cv_VectorOfVectorOfPoint_len, cv_VectorOfVectorOfPoint_is_empty,
	cv_VectorOfVectorOfPoint_capacity, cv_VectorOfVectorOfPoint_resize,
	cv_VectorOfVectorOfPoint_shrink_to_fit,
	cv_VectorOfVectorOfPoint_reserve, cv_VectorOfVectorOfPoint_remove,
	cv_VectorOfVectorOfPoint_swap, cv_VectorOfVectorOfPoint_clear,
	cv_VectorOfVectorOfPoint_get, cv_VectorOfVectorOfPoint_set,
	cv_VectorOfVectorOfPoint_push, cv_VectorOfVectorOfPoint_insert,
}
vector_non_copy_or_bool! { clone core::Vector<core::Point> }

unsafe impl Send for core::Vector<core::Vector<core::Point>> {}

impl core::ToInputArray for VectorOfVectorOfPoint {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfVectorOfPoint_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfPoint_input_array(self.as_raw_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfVectorOfPoint {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfVectorOfPoint_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfPoint_output_array(self.as_raw_mut_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfVectorOfPoint {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfVectorOfPoint_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfPoint_input_output_array(self.as_raw_mut_VectorOfVectorOfPoint(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfVectorOfPoint }

