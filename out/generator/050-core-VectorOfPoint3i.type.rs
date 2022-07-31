pub type VectorOfPoint3i = core::Vector<core::Point3i>;

impl VectorOfPoint3i {
	pub fn as_raw_VectorOfPoint3i(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfPoint3i(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Point3i, *const c_void, *mut c_void,
	cv_VectorOfPoint3i_new, cv_VectorOfPoint3i_delete,
	cv_VectorOfPoint3i_len, cv_VectorOfPoint3i_is_empty,
	cv_VectorOfPoint3i_capacity, cv_VectorOfPoint3i_resize,
	cv_VectorOfPoint3i_shrink_to_fit,
	cv_VectorOfPoint3i_reserve, cv_VectorOfPoint3i_remove,
	cv_VectorOfPoint3i_swap, cv_VectorOfPoint3i_clear,
	cv_VectorOfPoint3i_get, cv_VectorOfPoint3i_set,
	cv_VectorOfPoint3i_push, cv_VectorOfPoint3i_insert,
}
vector_copy_non_bool! { core::Point3i, *const c_void, *mut c_void,
	cv_VectorOfPoint3i_data, cv_VectorOfPoint3i_data_mut, cv_VectorOfPoint3i_from_slice,
	cv_VectorOfPoint3i_clone,
}

unsafe impl Send for core::Vector<core::Point3i> {}

impl core::ToInputArray for VectorOfPoint3i {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfPoint3i_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfPoint3i_input_array(self.as_raw_VectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfPoint3i {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfPoint3i_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfPoint3i_output_array(self.as_raw_mut_VectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfPoint3i {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfPoint3i_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfPoint3i_input_output_array(self.as_raw_mut_VectorOfPoint3i(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfPoint3i }

