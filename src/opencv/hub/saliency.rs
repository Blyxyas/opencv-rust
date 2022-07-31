#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Saliency API
//! 
//! Many computer vision applications may benefit from understanding where humans focus given a scene.
//! Other than cognitively understanding the way human perceive images and scenes, finding salient
//! regions and objects in the images helps various tasks such as speeding up object detection, object
//! recognition, object tracking and content-aware image editing.
//! 
//! About the saliency, there is a rich literature but the development is very fragmented. The principal
//! purpose of this API is to give a unique interface, a unique framework for use and plug sever
//! saliency algorithms, also with very different nature and methodology, but they share the same
//! purpose, organizing algorithms into three main categories:
//! 
//! **Static Saliency**: algorithms belonging to this category, exploit different image features that
//! allow to detect salient objects in a non dynamic scenarios.
//! 
//! **Motion Saliency**: algorithms belonging to this category, are particularly focused to detect
//! salient objects over time (hence also over frame), then there is a temporal component sealing
//! cosider that allows to detect "moving" objects as salient, meaning therefore also the more general
//! sense of detection the changes in the scene.
//! 
//! **Objectness**: Objectness is usually represented as a value which reflects how likely an image
//! window covers an object of any category. Algorithms belonging to this category, avoid making
//! decisions early on, by proposing a small number of category-independent proposals, that are expected
//! to cover all objects in an image. Being able to perceive objects before identifying them is closely
//! related to bottom up visual attention (saliency).
//! 
//! ![Saliency diagram](https://docs.opencv.org/4.6.0/saliency.png)
//! 
//! To see how API works, try tracker demo:
//! <https://github.com/fpuja/opencv_contrib/blob/saliencyModuleDevelop/modules/saliency/samples/computeSaliency.cpp>
//! 
//! 
//! Note: This API has been designed with PlantUML. If you modify this API please change UML.
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::SaliencyConst, super::Saliency, super::StaticSaliencyConst, super::StaticSaliency, super::MotionSaliencyConst, super::MotionSaliency, super::ObjectnessConst, super::Objectness, super::StaticSaliencySpectralResidualTraitConst, super::StaticSaliencySpectralResidualTrait, super::StaticSaliencyFineGrainedTraitConst, super::StaticSaliencyFineGrainedTrait, super::MotionSaliencyBinWangApr2014TraitConst, super::MotionSaliencyBinWangApr2014Trait, super::ObjectnessBINGTraitConst, super::ObjectnessBINGTrait };
}

pub trait MotionSaliencyConst: crate::saliency::SaliencyConst {
	fn as_raw_MotionSaliency(&self) -> *const c_void;

}

pub trait MotionSaliency: crate::saliency::MotionSaliencyConst + crate::saliency::Saliency {
	fn as_raw_mut_MotionSaliency(&mut self) -> *mut c_void;

}

pub trait MotionSaliencyBinWangApr2014TraitConst: crate::saliency::MotionSaliencyConst {
	fn as_raw_MotionSaliencyBinWangApr2014(&self) -> *const c_void;

	#[inline]
	fn get_image_width(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_getImageWidth_const(self.as_raw_MotionSaliencyBinWangApr2014(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_image_height(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_getImageHeight_const(self.as_raw_MotionSaliencyBinWangApr2014(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait MotionSaliencyBinWangApr2014Trait: crate::saliency::MotionSaliency + crate::saliency::MotionSaliencyBinWangApr2014TraitConst {
	fn as_raw_mut_MotionSaliencyBinWangApr2014(&mut self) -> *mut c_void;

	#[inline]
	fn compute_saliency(&mut self, image: &dyn core::ToInputArray, saliency_map: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(saliency_map);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_computeSaliency_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_MotionSaliencyBinWangApr2014(), image.as_raw__InputArray(), saliency_map.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_imagesize(&mut self, w: i32, h: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_setImagesize_int_int(self.as_raw_mut_MotionSaliencyBinWangApr2014(), w, h, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn init(&mut self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_init(self.as_raw_mut_MotionSaliencyBinWangApr2014(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_image_width(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_setImageWidth_int(self.as_raw_mut_MotionSaliencyBinWangApr2014(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_image_height(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_setImageHeight_int(self.as_raw_mut_MotionSaliencyBinWangApr2014(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct MotionSaliencyBinWangApr2014 {
	ptr: *mut c_void
}

opencv_type_boxed! { MotionSaliencyBinWangApr2014 }

impl Drop for MotionSaliencyBinWangApr2014 {
	fn drop(&mut self) {
		extern "C" { fn cv_MotionSaliencyBinWangApr2014_delete(instance: *mut c_void); }
		unsafe { cv_MotionSaliencyBinWangApr2014_delete(self.as_raw_mut_MotionSaliencyBinWangApr2014()) };
	}
}

unsafe impl Send for MotionSaliencyBinWangApr2014 {}

impl core::AlgorithmTraitConst for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::MotionSaliencyConst for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_MotionSaliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::MotionSaliency for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_mut_MotionSaliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::SaliencyConst for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::Saliency for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::MotionSaliencyBinWangApr2014TraitConst for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_MotionSaliencyBinWangApr2014(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::MotionSaliencyBinWangApr2014Trait for MotionSaliencyBinWangApr2014 {
	#[inline] fn as_raw_mut_MotionSaliencyBinWangApr2014(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl MotionSaliencyBinWangApr2014 {
	#[inline]
	pub fn default() -> Result<crate::saliency::MotionSaliencyBinWangApr2014> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_MotionSaliencyBinWangApr2014(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::saliency::MotionSaliencyBinWangApr2014::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::saliency::MotionSaliencyBinWangApr2014>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_MotionSaliencyBinWangApr2014_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::saliency::MotionSaliencyBinWangApr2014>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { MotionSaliencyBinWangApr2014, core::Algorithm, cv_MotionSaliencyBinWangApr2014_to_Algorithm }

pub trait ObjectnessConst: crate::saliency::SaliencyConst {
	fn as_raw_Objectness(&self) -> *const c_void;

}

pub trait Objectness: crate::saliency::ObjectnessConst + crate::saliency::Saliency {
	fn as_raw_mut_Objectness(&mut self) -> *mut c_void;

}

pub trait ObjectnessBINGTraitConst: crate::saliency::ObjectnessConst {
	fn as_raw_ObjectnessBING(&self) -> *const c_void;

	#[inline]
	fn write(&self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_write_const(self.as_raw_ObjectnessBING(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_base(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_getBase_const(self.as_raw_ObjectnessBING(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_nss(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_getNSS_const(self.as_raw_ObjectnessBING(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_w(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_getW_const(self.as_raw_ObjectnessBING(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait ObjectnessBINGTrait: crate::saliency::Objectness + crate::saliency::ObjectnessBINGTraitConst {
	fn as_raw_mut_ObjectnessBING(&mut self) -> *mut c_void;

	#[inline]
	fn compute_saliency(&mut self, image: &dyn core::ToInputArray, saliency_map: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(saliency_map);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_computeSaliency_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_ObjectnessBING(), image.as_raw__InputArray(), saliency_map.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn read(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_read(self.as_raw_mut_ObjectnessBING(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn getobjectness_values(&mut self) -> Result<core::Vector<f32>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_getobjectnessValues(self.as_raw_mut_ObjectnessBING(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Vector::<f32>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	fn set_training_path(&mut self, training_path: &str) -> Result<()> {
		extern_container_arg!(training_path);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_setTrainingPath_const_StringR(self.as_raw_mut_ObjectnessBING(), training_path.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_bb_res_dir(&mut self, results_dir: &str) -> Result<()> {
		extern_container_arg!(results_dir);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_setBBResDir_const_StringR(self.as_raw_mut_ObjectnessBING(), results_dir.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_base(&mut self, val: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_setBase_double(self.as_raw_mut_ObjectnessBING(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_nss(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_setNSS_int(self.as_raw_mut_ObjectnessBING(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_w(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_setW_int(self.as_raw_mut_ObjectnessBING(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct ObjectnessBING {
	ptr: *mut c_void
}

opencv_type_boxed! { ObjectnessBING }

impl Drop for ObjectnessBING {
	fn drop(&mut self) {
		extern "C" { fn cv_ObjectnessBING_delete(instance: *mut c_void); }
		unsafe { cv_ObjectnessBING_delete(self.as_raw_mut_ObjectnessBING()) };
	}
}

unsafe impl Send for ObjectnessBING {}

impl core::AlgorithmTraitConst for ObjectnessBING {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for ObjectnessBING {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::ObjectnessConst for ObjectnessBING {
	#[inline] fn as_raw_Objectness(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::Objectness for ObjectnessBING {
	#[inline] fn as_raw_mut_Objectness(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::SaliencyConst for ObjectnessBING {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::Saliency for ObjectnessBING {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::ObjectnessBINGTraitConst for ObjectnessBING {
	#[inline] fn as_raw_ObjectnessBING(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::ObjectnessBINGTrait for ObjectnessBING {
	#[inline] fn as_raw_mut_ObjectnessBING(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl ObjectnessBING {
	#[inline]
	pub fn default() -> Result<crate::saliency::ObjectnessBING> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_ObjectnessBING(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::saliency::ObjectnessBING::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::saliency::ObjectnessBING>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_ObjectnessBING_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::saliency::ObjectnessBING>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { ObjectnessBING, core::Algorithm, cv_ObjectnessBING_to_Algorithm }

pub trait SaliencyConst: core::AlgorithmTraitConst {
	fn as_raw_Saliency(&self) -> *const c_void;

}

pub trait Saliency: core::AlgorithmTrait + crate::saliency::SaliencyConst {
	fn as_raw_mut_Saliency(&mut self) -> *mut c_void;

	#[inline]
	fn compute_saliency(&mut self, image: &dyn core::ToInputArray, saliency_map: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(saliency_map);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_Saliency_computeSaliency_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_Saliency(), image.as_raw__InputArray(), saliency_map.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait StaticSaliencyConst: crate::saliency::SaliencyConst {
	fn as_raw_StaticSaliency(&self) -> *const c_void;

}

pub trait StaticSaliency: crate::saliency::Saliency + crate::saliency::StaticSaliencyConst {
	fn as_raw_mut_StaticSaliency(&mut self) -> *mut c_void;

	#[inline]
	fn compute_binary_map(&mut self, _saliency_map: &dyn core::ToInputArray, _binary_map: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(_saliency_map);
		output_array_arg!(_binary_map);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliency_computeBinaryMap_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_StaticSaliency(), _saliency_map.as_raw__InputArray(), _binary_map.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait StaticSaliencyFineGrainedTraitConst: crate::saliency::StaticSaliencyConst {
	fn as_raw_StaticSaliencyFineGrained(&self) -> *const c_void;

}

pub trait StaticSaliencyFineGrainedTrait: crate::saliency::StaticSaliency + crate::saliency::StaticSaliencyFineGrainedTraitConst {
	fn as_raw_mut_StaticSaliencyFineGrained(&mut self) -> *mut c_void;

	#[inline]
	fn compute_saliency(&mut self, image: &dyn core::ToInputArray, saliency_map: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(saliency_map);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencyFineGrained_computeSaliency_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_StaticSaliencyFineGrained(), image.as_raw__InputArray(), saliency_map.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct StaticSaliencyFineGrained {
	ptr: *mut c_void
}

opencv_type_boxed! { StaticSaliencyFineGrained }

impl Drop for StaticSaliencyFineGrained {
	fn drop(&mut self) {
		extern "C" { fn cv_StaticSaliencyFineGrained_delete(instance: *mut c_void); }
		unsafe { cv_StaticSaliencyFineGrained_delete(self.as_raw_mut_StaticSaliencyFineGrained()) };
	}
}

unsafe impl Send for StaticSaliencyFineGrained {}

impl core::AlgorithmTraitConst for StaticSaliencyFineGrained {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for StaticSaliencyFineGrained {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::SaliencyConst for StaticSaliencyFineGrained {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::Saliency for StaticSaliencyFineGrained {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::StaticSaliencyConst for StaticSaliencyFineGrained {
	#[inline] fn as_raw_StaticSaliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::StaticSaliency for StaticSaliencyFineGrained {
	#[inline] fn as_raw_mut_StaticSaliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::StaticSaliencyFineGrainedTraitConst for StaticSaliencyFineGrained {
	#[inline] fn as_raw_StaticSaliencyFineGrained(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::StaticSaliencyFineGrainedTrait for StaticSaliencyFineGrained {
	#[inline] fn as_raw_mut_StaticSaliencyFineGrained(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl StaticSaliencyFineGrained {
	#[inline]
	pub fn default() -> Result<crate::saliency::StaticSaliencyFineGrained> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencyFineGrained_StaticSaliencyFineGrained(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::saliency::StaticSaliencyFineGrained::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::saliency::StaticSaliencyFineGrained>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencyFineGrained_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::saliency::StaticSaliencyFineGrained>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { StaticSaliencyFineGrained, core::Algorithm, cv_StaticSaliencyFineGrained_to_Algorithm }

pub trait StaticSaliencySpectralResidualTraitConst: crate::saliency::StaticSaliencyConst {
	fn as_raw_StaticSaliencySpectralResidual(&self) -> *const c_void;

	#[inline]
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_write_const_FileStorageR(self.as_raw_StaticSaliencySpectralResidual(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_image_width(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_getImageWidth_const(self.as_raw_StaticSaliencySpectralResidual(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_image_height(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_getImageHeight_const(self.as_raw_StaticSaliencySpectralResidual(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait StaticSaliencySpectralResidualTrait: crate::saliency::StaticSaliency + crate::saliency::StaticSaliencySpectralResidualTraitConst {
	fn as_raw_mut_StaticSaliencySpectralResidual(&mut self) -> *mut c_void;

	#[inline]
	fn compute_saliency(&mut self, image: &dyn core::ToInputArray, saliency_map: &mut dyn core::ToOutputArray) -> Result<bool> {
		input_array_arg!(image);
		output_array_arg!(saliency_map);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_computeSaliency_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_StaticSaliencySpectralResidual(), image.as_raw__InputArray(), saliency_map.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_read_const_FileNodeR(self.as_raw_mut_StaticSaliencySpectralResidual(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_image_width(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_setImageWidth_int(self.as_raw_mut_StaticSaliencySpectralResidual(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_image_height(&mut self, val: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_setImageHeight_int(self.as_raw_mut_StaticSaliencySpectralResidual(), val, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct StaticSaliencySpectralResidual {
	ptr: *mut c_void
}

opencv_type_boxed! { StaticSaliencySpectralResidual }

impl Drop for StaticSaliencySpectralResidual {
	fn drop(&mut self) {
		extern "C" { fn cv_StaticSaliencySpectralResidual_delete(instance: *mut c_void); }
		unsafe { cv_StaticSaliencySpectralResidual_delete(self.as_raw_mut_StaticSaliencySpectralResidual()) };
	}
}

unsafe impl Send for StaticSaliencySpectralResidual {}

impl core::AlgorithmTraitConst for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::SaliencyConst for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_Saliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::Saliency for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_mut_Saliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::StaticSaliencyConst for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_StaticSaliency(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::StaticSaliency for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_mut_StaticSaliency(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::saliency::StaticSaliencySpectralResidualTraitConst for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_StaticSaliencySpectralResidual(&self) -> *const c_void { self.as_raw() }
}

impl crate::saliency::StaticSaliencySpectralResidualTrait for StaticSaliencySpectralResidual {
	#[inline] fn as_raw_mut_StaticSaliencySpectralResidual(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl StaticSaliencySpectralResidual {
	#[inline]
	pub fn default() -> Result<crate::saliency::StaticSaliencySpectralResidual> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_StaticSaliencySpectralResidual(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::saliency::StaticSaliencySpectralResidual::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn create() -> Result<core::Ptr<crate::saliency::StaticSaliencySpectralResidual>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_saliency_StaticSaliencySpectralResidual_create(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::saliency::StaticSaliencySpectralResidual>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { StaticSaliencySpectralResidual, core::Algorithm, cv_StaticSaliencySpectralResidual_to_Algorithm }
