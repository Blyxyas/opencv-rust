pub type VectorOfPoint = core::Vector<core::Point>;

impl VectorOfPoint {
	pub fn as_raw_VectorOfPoint(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfPoint(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Point, *const c_void, *mut c_void,
	cv_VectorOfPoint_new, cv_VectorOfPoint_delete,
	cv_VectorOfPoint_len, cv_VectorOfPoint_is_empty,
	cv_VectorOfPoint_capacity, cv_VectorOfPoint_resize,
	cv_VectorOfPoint_shrink_to_fit,
	cv_VectorOfPoint_reserve, cv_VectorOfPoint_remove,
	cv_VectorOfPoint_swap, cv_VectorOfPoint_clear,
	cv_VectorOfPoint_get, cv_VectorOfPoint_set,
	cv_VectorOfPoint_push, cv_VectorOfPoint_insert,
}
vector_copy_non_bool! { core::Point, *const c_void, *mut c_void,
	cv_VectorOfPoint_data, cv_VectorOfPoint_data_mut, cv_VectorOfPoint_from_slice,
	cv_VectorOfPoint_clone,
}

unsafe impl Send for core::Vector<core::Point> {}

impl core::ToInputArray for VectorOfPoint {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfPoint_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfPoint_input_array(self.as_raw_VectorOfPoint(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfPoint {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfPoint_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfPoint_output_array(self.as_raw_mut_VectorOfPoint(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfPoint {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfPoint_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfPoint_input_output_array(self.as_raw_mut_VectorOfPoint(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfPoint }

