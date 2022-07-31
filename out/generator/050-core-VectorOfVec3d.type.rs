pub type VectorOfVec3d = core::Vector<core::Vec3d>;

impl VectorOfVec3d {
	pub fn as_raw_VectorOfVec3d(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVec3d(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vec3d, *const c_void, *mut c_void,
	cv_VectorOfVec3d_new, cv_VectorOfVec3d_delete,
	cv_VectorOfVec3d_len, cv_VectorOfVec3d_is_empty,
	cv_VectorOfVec3d_capacity, cv_VectorOfVec3d_resize,
	cv_VectorOfVec3d_shrink_to_fit,
	cv_VectorOfVec3d_reserve, cv_VectorOfVec3d_remove,
	cv_VectorOfVec3d_swap, cv_VectorOfVec3d_clear,
	cv_VectorOfVec3d_get, cv_VectorOfVec3d_set,
	cv_VectorOfVec3d_push, cv_VectorOfVec3d_insert,
}
vector_copy_non_bool! { core::Vec3d, *const c_void, *mut c_void,
	cv_VectorOfVec3d_data, cv_VectorOfVec3d_data_mut, cv_VectorOfVec3d_from_slice,
	cv_VectorOfVec3d_clone,
}

unsafe impl Send for core::Vector<core::Vec3d> {}

impl core::ToInputArray for VectorOfVec3d {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfVec3d_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVec3d_input_array(self.as_raw_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfVec3d {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfVec3d_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVec3d_output_array(self.as_raw_mut_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfVec3d {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfVec3d_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVec3d_input_output_array(self.as_raw_mut_VectorOfVec3d(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfVec3d }

