pub type VectorOfVectorOfVec3f = core::Vector<core::Vector<core::Vec3f>>;

impl VectorOfVectorOfVec3f {
	pub fn as_raw_VectorOfVectorOfVec3f(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOfVec3f(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vector<core::Vec3f>, *const c_void, *mut c_void,
	cv_VectorOfVectorOfVec3f_new, cv_VectorOfVectorOfVec3f_delete,
	cv_VectorOfVectorOfVec3f_len, cv_VectorOfVectorOfVec3f_is_empty,
	cv_VectorOfVectorOfVec3f_capacity, cv_VectorOfVectorOfVec3f_resize,
	cv_VectorOfVectorOfVec3f_shrink_to_fit,
	cv_VectorOfVectorOfVec3f_reserve, cv_VectorOfVectorOfVec3f_remove,
	cv_VectorOfVectorOfVec3f_swap, cv_VectorOfVectorOfVec3f_clear,
	cv_VectorOfVectorOfVec3f_get, cv_VectorOfVectorOfVec3f_set,
	cv_VectorOfVectorOfVec3f_push, cv_VectorOfVectorOfVec3f_insert,
}
vector_non_copy_or_bool! { clone core::Vector<core::Vec3f> }

unsafe impl Send for core::Vector<core::Vector<core::Vec3f>> {}

impl core::ToInputArray for VectorOfVectorOfVec3f {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfVectorOfVec3f_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfVec3f_input_array(self.as_raw_VectorOfVectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfVectorOfVec3f {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfVectorOfVec3f_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfVec3f_output_array(self.as_raw_mut_VectorOfVectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfVectorOfVec3f {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfVectorOfVec3f_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOfVec3f_input_output_array(self.as_raw_mut_VectorOfVectorOfVec3f(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfVectorOfVec3f }

