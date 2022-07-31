pub type VectorOfVec3i = core::Vector<core::Vec3i>;

impl VectorOfVec3i {
	pub fn as_raw_VectorOfVec3i(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVec3i(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vec3i, *const c_void, *mut c_void,
	cv_VectorOfVec3i_new, cv_VectorOfVec3i_delete,
	cv_VectorOfVec3i_len, cv_VectorOfVec3i_is_empty,
	cv_VectorOfVec3i_capacity, cv_VectorOfVec3i_resize,
	cv_VectorOfVec3i_shrink_to_fit,
	cv_VectorOfVec3i_reserve, cv_VectorOfVec3i_remove,
	cv_VectorOfVec3i_swap, cv_VectorOfVec3i_clear,
	cv_VectorOfVec3i_get, cv_VectorOfVec3i_set,
	cv_VectorOfVec3i_push, cv_VectorOfVec3i_insert,
}
vector_copy_non_bool! { core::Vec3i, *const c_void, *mut c_void,
	cv_VectorOfVec3i_data, cv_VectorOfVec3i_data_mut, cv_VectorOfVec3i_from_slice,
	cv_VectorOfVec3i_clone,
}

unsafe impl Send for core::Vector<core::Vec3i> {}

impl core::ToInputArray for VectorOfVec3i {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfVec3i_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVec3i_input_array(self.as_raw_VectorOfVec3i(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfVec3i {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfVec3i_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVec3i_output_array(self.as_raw_mut_VectorOfVec3i(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfVec3i {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfVec3i_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVec3i_input_output_array(self.as_raw_mut_VectorOfVec3i(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfVec3i }

