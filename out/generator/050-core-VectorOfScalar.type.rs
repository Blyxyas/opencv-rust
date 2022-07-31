pub type VectorOfScalar = core::Vector<core::Scalar>;

impl VectorOfScalar {
	pub fn as_raw_VectorOfScalar(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfScalar(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Scalar, *const c_void, *mut c_void,
	cv_VectorOfScalar_new, cv_VectorOfScalar_delete,
	cv_VectorOfScalar_len, cv_VectorOfScalar_is_empty,
	cv_VectorOfScalar_capacity, cv_VectorOfScalar_resize,
	cv_VectorOfScalar_shrink_to_fit,
	cv_VectorOfScalar_reserve, cv_VectorOfScalar_remove,
	cv_VectorOfScalar_swap, cv_VectorOfScalar_clear,
	cv_VectorOfScalar_get, cv_VectorOfScalar_set,
	cv_VectorOfScalar_push, cv_VectorOfScalar_insert,
}
vector_copy_non_bool! { core::Scalar, *const c_void, *mut c_void,
	cv_VectorOfScalar_data, cv_VectorOfScalar_data_mut, cv_VectorOfScalar_from_slice,
	cv_VectorOfScalar_clone,
}

unsafe impl Send for core::Vector<core::Scalar> {}

impl core::ToInputArray for VectorOfScalar {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfScalar_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfScalar_input_array(self.as_raw_VectorOfScalar(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfScalar {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfScalar_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfScalar_output_array(self.as_raw_mut_VectorOfScalar(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfScalar {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfScalar_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfScalar_input_output_array(self.as_raw_mut_VectorOfScalar(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfScalar }

