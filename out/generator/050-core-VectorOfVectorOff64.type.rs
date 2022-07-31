pub type VectorOfVectorOff64 = core::Vector<core::Vector<f64>>;

impl VectorOfVectorOff64 {
	pub fn as_raw_VectorOfVectorOff64(&self) -> *const c_void { self.as_raw() }
	pub fn as_raw_mut_VectorOfVectorOff64(&mut self) -> *mut c_void { self.as_raw_mut() }
}

vector_extern! { core::Vector<f64>, *const c_void, *mut c_void,
	cv_VectorOfVectorOff64_new, cv_VectorOfVectorOff64_delete,
	cv_VectorOfVectorOff64_len, cv_VectorOfVectorOff64_is_empty,
	cv_VectorOfVectorOff64_capacity, cv_VectorOfVectorOff64_resize,
	cv_VectorOfVectorOff64_shrink_to_fit,
	cv_VectorOfVectorOff64_reserve, cv_VectorOfVectorOff64_remove,
	cv_VectorOfVectorOff64_swap, cv_VectorOfVectorOff64_clear,
	cv_VectorOfVectorOff64_get, cv_VectorOfVectorOff64_set,
	cv_VectorOfVectorOff64_push, cv_VectorOfVectorOff64_insert,
}
vector_non_copy_or_bool! { clone core::Vector<f64> }

unsafe impl Send for core::Vector<core::Vector<f64>> {}

impl core::ToInputArray for VectorOfVectorOff64 {
	#[inline]
	fn input_array(&self) -> Result<core::_InputArray> {
		extern "C" { fn cv_VectorOfVectorOff64_input_array(instance: *const c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOff64_input_array(self.as_raw_VectorOfVectorOff64(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputArray::from_raw(ptr) } )
	}
}

impl core::ToOutputArray for VectorOfVectorOff64 {
	#[inline]
	fn output_array(&mut self) -> Result<core::_OutputArray> {
		extern "C" { fn cv_VectorOfVectorOff64_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOff64_output_array(self.as_raw_mut_VectorOfVectorOff64(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_OutputArray::from_raw(ptr) })
	}
}

impl core::ToInputOutputArray for VectorOfVectorOff64 {
	#[inline]
	fn input_output_array(&mut self) -> Result<core::_InputOutputArray> {
		extern "C" { fn cv_VectorOfVectorOff64_input_output_array(instance: *mut c_void, ocvrs_return: *mut sys::Result<*mut c_void>); }
		return_send!(via ocvrs_return);
		unsafe { cv_VectorOfVectorOff64_input_output_array(self.as_raw_mut_VectorOfVectorOff64(), ocvrs_return.as_mut_ptr()) }
		return_receive!(unsafe ocvrs_return => ret);
		ret.into_result()
			.map(|ptr| unsafe { core::_InputOutputArray::from_raw(ptr) })
	}
}

input_output_array_ref_forward! { VectorOfVectorOff64 }

