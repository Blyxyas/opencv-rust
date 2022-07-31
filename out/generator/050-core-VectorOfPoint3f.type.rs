pub type VectorOfPoint3f = core::Vector<core::Point3f>;

impl VectorOfPoint3f {
	pub fn as_raw_VectorOfPoint3f(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfPoint3f(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Point3f, *const c_void, *mut c_void,
	cv_VectorOfPoint3f_new, cv_VectorOfPoint3f_delete,
	cv_VectorOfPoint3f_len, cv_VectorOfPoint3f_is_empty,
	cv_VectorOfPoint3f_capacity, cv_VectorOfPoint3f_resize,
	cv_VectorOfPoint3f_shrink_to_fit,
	cv_VectorOfPoint3f_reserve, cv_VectorOfPoint3f_remove,
	cv_VectorOfPoint3f_swap, cv_VectorOfPoint3f_clear,
	cv_VectorOfPoint3f_get, cv_VectorOfPoint3f_set,
	cv_VectorOfPoint3f_push, cv_VectorOfPoint3f_insert,
}
vector_copy_non_bool! { core::Point3f, *const c_void, *mut c_void,
	cv_VectorOfPoint3f_data, cv_VectorOfPoint3f_data_mut, cv_VectorOfPoint3f_from_slice,
	cv_VectorOfPoint3f_clone,
}

unsafe impl Send for core::Vector<core::Point3f> {}

impl core::ToInputArray for VectorOfPoint3f {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfPoint3f_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfPoint3f_input_array(self.as_raw_VectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfPoint3f {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfPoint3f_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfPoint3f_output_array(self.as_raw_mut_VectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfPoint3f {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfPoint3f_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfPoint3f_input_output_array(self.as_raw_mut_VectorOfPoint3f(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfPoint3f }

