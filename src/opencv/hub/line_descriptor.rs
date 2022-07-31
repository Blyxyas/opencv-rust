#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Binary descriptors for lines extracted from an image
//! 
//! Introduction
//! ------------
//! 
//! One of the most challenging activities in computer vision is the extraction of useful information
//! from a given image. Such information, usually comes in the form of points that preserve some kind of
//! property (for instance, they are scale-invariant) and are actually representative of input image.
//! 
//! The goal of this module is seeking a new kind of representative information inside an image and
//! providing the functionalities for its extraction and representation. In particular, differently from
//! previous methods for detection of relevant elements inside an image, lines are extracted in place of
//! points; a new class is defined ad hoc to summarize a line's properties, for reuse and plotting
//! purposes.
//! 
//! Computation of binary descriptors
//! ---------------------------------
//! 
//! To obtatin a binary descriptor representing a certain line detected from a certain octave of an
//! image, we first compute a non-binary descriptor as described in [LBD](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_LBD) . Such algorithm works on
//! lines extracted using EDLine detector, as explained in [EDL](https://docs.opencv.org/4.6.0/d0/de3/citelist.html#CITEREF_EDL) . Given a line, we consider a
//! rectangular region centered at it and called *line support region (LSR)*. Such region is divided
//! into a set of bands ![inline formula](https://latex.codecogs.com/png.latex?%5C%7BB%5F1%2C%20B%5F2%2C%20%2E%2E%2E%2C%20B%5Fm%5C%7D), whose length equals the one of line.
//! 
//! If we indicate with ![inline formula](https://latex.codecogs.com/png.latex?%5Cbf%7Bd%7D%5FL) the direction of line, the orthogonal and clockwise direction to line
//! ![inline formula](https://latex.codecogs.com/png.latex?%5Cbf%7Bd%7D%5F%7B%5Cperp%7D) can be determined; these two directions, are used to construct a reference frame
//! centered in the middle point of line. The gradients of pixels ![inline formula](https://latex.codecogs.com/png.latex?%5Cbf%7Bg%27%7D) inside LSR can be projected
//! to the newly determined frame, obtaining their local equivalent
//! ![inline formula](https://latex.codecogs.com/png.latex?%5Cbf%7Bg%27%7D%20%3D%20%28%5Cbf%7Bg%7D%5ET%20%5Ccdot%20%5Cbf%7Bd%7D%5F%7B%5Cperp%7D%2C%20%5Cbf%7Bg%7D%5ET%20%5Ccdot%20%5Cbf%7Bd%7D%5FL%29%5ET%20%5Ctriangleq%20%28%5Cbf%7Bg%27%7D%5F%7Bd%5F%7B%5Cperp%7D%7D%2C%20%5Cbf%7Bg%27%7D%5F%7Bd%5FL%7D%29%5ET).
//! 
//! Later on, a Gaussian function is applied to all LSR's pixels along ![inline formula](https://latex.codecogs.com/png.latex?%5Cbf%7Bd%7D%5F%5Cperp) direction; first,
//! we assign a global weighting coefficient ![inline formula](https://latex.codecogs.com/png.latex?f%5Fg%28i%29%20%3D%20%281%2F%5Csqrt%7B2%5Cpi%7D%5Csigma%5Fg%29e%5E%7B%2Dd%5E2%5Fi%2F2%5Csigma%5E2%5Fg%7D) to
//! *i*-th row in LSR, where ![inline formula](https://latex.codecogs.com/png.latex?d%5Fi) is the distance of *i*-th row from the center row in LSR,
//! ![inline formula](https://latex.codecogs.com/png.latex?%5Csigma%5Fg%20%3D%200%2E5%28m%20%5Ccdot%20w%20%2D%201%29) and ![inline formula](https://latex.codecogs.com/png.latex?w) is the width of bands (the same for every band). Secondly,
//! considering a band ![inline formula](https://latex.codecogs.com/png.latex?B%5Fj) and its neighbor bands ![inline formula](https://latex.codecogs.com/png.latex?B%5F%7Bj%2D1%7D%2C%20B%5F%7Bj%2B1%7D), we assign a local weighting
//! ![inline formula](https://latex.codecogs.com/png.latex?F%5Fl%28k%29%20%3D%20%281%2F%5Csqrt%7B2%5Cpi%7D%5Csigma%5Fl%29e%5E%7B%2Dd%27%5E2%5Fk%2F2%5Csigma%5Fl%5E2%7D), where ![inline formula](https://latex.codecogs.com/png.latex?d%27%5Fk) is the distance of *k*-th
//! row from the center row in ![inline formula](https://latex.codecogs.com/png.latex?B%5Fj) and ![inline formula](https://latex.codecogs.com/png.latex?%5Csigma%5Fl%20%3D%20w). Using the global and local weights, we obtain,
//! at the same time, the reduction of role played by gradients far from line and of boundary effect,
//! respectively.
//! 
//! Each band ![inline formula](https://latex.codecogs.com/png.latex?B%5Fj) in LSR has an associated *band descriptor(BD)* which is computed considering
//! previous and next band (top and bottom bands are ignored when computing descriptor for first and
//! last band). Once each band has been assignen its BD, the LBD descriptor of line is simply given by
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?LBD%20%3D%20%28BD%5F1%5ET%2C%20BD%5F2%5ET%2C%20%2E%2E%2E%20%2C%20BD%5ET%5Fm%29%5ET%2E)
//! 
//! To compute a band descriptor ![inline formula](https://latex.codecogs.com/png.latex?B%5Fj), each *k*-th row in it is considered and the gradients in such
//! row are accumulated:
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?%5Cbegin%7Bmatrix%7D%20%5Cbf%7BV1%7D%5Ek%5Fj%20%3D%20%5Clambda%20%5Csum%5Climits%5F%7B%5Cbf%7Bg%7D%27%5F%7Bd%5F%5Cperp%7D%3E0%7D%5Cbf%7Bg%7D%27%5F%7Bd%5F%5Cperp%7D%2C%20%26%20%20%5Cbf%7BV2%7D%5Ek%5Fj%20%3D%20%5Clambda%20%5Csum%5Climits%5F%7B%5Cbf%7Bg%7D%27%5F%7Bd%5F%5Cperp%7D%3C0%7D%20%2D%5Cbf%7Bg%7D%27%5F%7Bd%5F%5Cperp%7D%2C%20%5C%5C%20%5Cbf%7BV3%7D%5Ek%5Fj%20%3D%20%5Clambda%20%5Csum%5Climits%5F%7B%5Cbf%7Bg%7D%27%5F%7Bd%5FL%7D%3E0%7D%5Cbf%7Bg%7D%27%5F%7Bd%5FL%7D%2C%20%26%20%5Cbf%7BV4%7D%5Ek%5Fj%20%3D%20%5Clambda%20%5Csum%5Climits%5F%7B%5Cbf%7Bg%7D%27%5F%7Bd%5FL%7D%3C0%7D%20%2D%5Cbf%7Bg%7D%27%5F%7Bd%5FL%7D%5Cend%7Bmatrix%7D%2E)
//! 
//! with ![inline formula](https://latex.codecogs.com/png.latex?%5Clambda%20%3D%20f%5Fg%28k%29f%5Fl%28k%29).
//! 
//! By stacking previous results, we obtain the *band description matrix (BDM)*
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?BDM%5Fj%20%3D%20%5Cleft%28%5Cbegin%7Bmatrix%7D%20%5Cbf%7BV1%7D%5Fj%5E1%20%26%20%5Cbf%7BV1%7D%5Fj%5E2%20%26%20%5Cldots%20%26%20%5Cbf%7BV1%7D%5Fj%5En%20%5C%5C%20%5Cbf%7BV2%7D%5Fj%5E1%20%26%20%5Cbf%7BV2%7D%5Fj%5E2%20%26%20%5Cldots%20%26%20%5Cbf%7BV2%7D%5Fj%5En%20%5C%5C%20%5Cbf%7BV3%7D%5Fj%5E1%20%26%20%5Cbf%7BV3%7D%5Fj%5E2%20%26%20%5Cldots%20%26%20%5Cbf%7BV3%7D%5Fj%5En%20%5C%5C%20%5Cbf%7BV4%7D%5Fj%5E1%20%26%20%5Cbf%7BV4%7D%5Fj%5E2%20%26%20%5Cldots%20%26%20%5Cbf%7BV4%7D%5Fj%5En%20%5Cend%7Bmatrix%7D%20%5Cright%29%20%5Cin%20%5Cmathbb%7BR%7D%5E%7B4%5Ctimes%20n%7D%2C)
//! 
//! with ![inline formula](https://latex.codecogs.com/png.latex?n) the number of rows in band ![inline formula](https://latex.codecogs.com/png.latex?B%5Fj):
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?n%20%3D%20%5Cbegin%7Bcases%7D%202w%2C%20%26%20j%20%3D%201%7C%7Cm%3B%20%5C%5C%203w%2C%20%26%20%5Cmbox%7Belse%7D%2E%20%5Cend%7Bcases%7D)
//! 
//! Each ![inline formula](https://latex.codecogs.com/png.latex?BD%5Fj) can be obtained using the standard deviation vector ![inline formula](https://latex.codecogs.com/png.latex?S%5Fj) and mean vector ![inline formula](https://latex.codecogs.com/png.latex?M%5Fj) of
//! ![inline formula](https://latex.codecogs.com/png.latex?BDM%5FJ). Thus, finally:
//! 
//! ![block formula](https://latex.codecogs.com/png.latex?LBD%20%3D%20%28M%5F1%5ET%2C%20S%5F1%5ET%2C%20M%5F2%5ET%2C%20S%5F2%5ET%2C%20%5Cldots%2C%20M%5Fm%5ET%2C%20S%5Fm%5ET%29%5ET%20%5Cin%20%5Cmathbb%7BR%7D%5E%7B8m%7D)
//! 
//! Once the LBD has been obtained, it must be converted into a binary form. For such purpose, we
//! consider 32 possible pairs of BD inside it; each couple of BD is compared bit by bit and comparison
//! generates an 8 bit string. Concatenating 32 comparison strings, we get the 256-bit final binary
//! representation of a single LBD.
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::BinaryDescriptor_ParamsTraitConst, super::BinaryDescriptor_ParamsTrait, super::BinaryDescriptorTraitConst, super::BinaryDescriptorTrait, super::LSDDetectorTraitConst, super::LSDDetectorTrait, super::BinaryDescriptorMatcherTraitConst, super::BinaryDescriptorMatcherTrait };
}

pub const DrawLinesMatchesFlags_DEFAULT: i32 = 0;
pub const DrawLinesMatchesFlags_DRAW_OVER_OUTIMG: i32 = 1;
pub const DrawLinesMatchesFlags_NOT_DRAW_SINGLE_LINES: i32 = 2;
pub const MLN10: f64 = 2.30258509299404568402;
pub const RELATIVE_ERROR_FACTOR: f64 = 100.0;
pub type uint16 = u16;
pub type uint32 = u32;
pub type uint64 = u64;
pub type uint8 = u8;
/// ## C++ default parameters
/// * color: Scalar::all(-1)
/// * flags: DrawLinesMatchesFlags::DEFAULT
#[inline]
pub fn draw_keylines(image: &core::Mat, keylines: &core::Vector<crate::line_descriptor::KeyLine>, out_image: &mut core::Mat, color: core::Scalar, flags: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_line_descriptor_drawKeylines_const_MatR_const_vector_KeyLine_R_MatR_const_ScalarR_int(image.as_raw_Mat(), keylines.as_raw_VectorOfKeyLine(), out_image.as_raw_mut_Mat(), &color, flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

/// ## C++ default parameters
/// * match_color: Scalar::all(-1)
/// * single_line_color: Scalar::all(-1)
/// * matches_mask: std::vector<char>()
/// * flags: DrawLinesMatchesFlags::DEFAULT
#[inline]
pub fn draw_line_matches(img1: &core::Mat, keylines1: &core::Vector<crate::line_descriptor::KeyLine>, img2: &core::Mat, keylines2: &core::Vector<crate::line_descriptor::KeyLine>, matches1to2: &core::Vector<core::DMatch>, out_img: &mut core::Mat, match_color: core::Scalar, single_line_color: core::Scalar, matches_mask: &core::Vector<i8>, flags: i32) -> Result<()> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_line_descriptor_drawLineMatches_const_MatR_const_vector_KeyLine_R_const_MatR_const_vector_KeyLine_R_const_vector_DMatch_R_MatR_const_ScalarR_const_ScalarR_const_vector_char_R_int(img1.as_raw_Mat(), keylines1.as_raw_VectorOfKeyLine(), img2.as_raw_Mat(), keylines2.as_raw_VectorOfKeyLine(), matches1to2.as_raw_VectorOfDMatch(), out_img.as_raw_mut_Mat(), &match_color, &single_line_color, matches_mask.as_raw_VectorOfi8(), flags, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

pub trait BinaryDescriptorTraitConst: core::AlgorithmTraitConst {
	fn as_raw_BinaryDescriptor(&self) -> *const c_void;

	#[inline]
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_write_const_FileStorageR(self.as_raw_BinaryDescriptor(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * masks: std::vector<Mat>()
	#[inline]
	fn detect(&self, images: &core::Vector<core::Mat>, keylines: &mut core::Vector<core::Vector<crate::line_descriptor::KeyLine>>, masks: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_detect_const_const_vector_Mat_R_vector_vector_KeyLine__R_const_vector_Mat_R(self.as_raw_BinaryDescriptor(), images.as_raw_VectorOfMat(), keylines.as_raw_mut_VectorOfVectorOfKeyLine(), masks.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * return_float_descr: false
	#[inline]
	fn compute(&self, image: &core::Mat, keylines: &mut core::Vector<crate::line_descriptor::KeyLine>, descriptors: &mut core::Mat, return_float_descr: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_compute_const_const_MatR_vector_KeyLine_R_MatR_bool(self.as_raw_BinaryDescriptor(), image.as_raw_Mat(), keylines.as_raw_mut_VectorOfKeyLine(), descriptors.as_raw_mut_Mat(), return_float_descr, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * return_float_descr: false
	#[inline]
	fn compute_1(&self, images: &core::Vector<core::Mat>, keylines: &mut core::Vector<core::Vector<crate::line_descriptor::KeyLine>>, descriptors: &mut core::Vector<core::Mat>, return_float_descr: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_compute_const_const_vector_Mat_R_vector_vector_KeyLine__R_vector_Mat_R_bool(self.as_raw_BinaryDescriptor(), images.as_raw_VectorOfMat(), keylines.as_raw_mut_VectorOfVectorOfKeyLine(), descriptors.as_raw_mut_VectorOfMat(), return_float_descr, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn descriptor_size(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_descriptorSize_const(self.as_raw_BinaryDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn descriptor_type(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_descriptorType_const(self.as_raw_BinaryDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn default_norm(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_defaultNorm_const(self.as_raw_BinaryDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait BinaryDescriptorTrait: core::AlgorithmTrait + crate::line_descriptor::BinaryDescriptorTraitConst {
	fn as_raw_mut_BinaryDescriptor(&mut self) -> *mut c_void;

	#[inline]
	fn get_num_of_octaves(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_getNumOfOctaves(self.as_raw_mut_BinaryDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_num_of_octaves(&mut self, octaves: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_setNumOfOctaves_int(self.as_raw_mut_BinaryDescriptor(), octaves, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_width_of_band(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_getWidthOfBand(self.as_raw_mut_BinaryDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_width_of_band(&mut self, width: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_setWidthOfBand_int(self.as_raw_mut_BinaryDescriptor(), width, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn get_reduction_ratio(&mut self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_getReductionRatio(self.as_raw_mut_BinaryDescriptor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn set_reduction_ratio(&mut self, r_ratio: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_setReductionRatio_int(self.as_raw_mut_BinaryDescriptor(), r_ratio, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_read_const_FileNodeR(self.as_raw_mut_BinaryDescriptor(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * mask: Mat()
	#[inline]
	fn detect_1(&mut self, image: &core::Mat, keypoints: &mut core::Vector<crate::line_descriptor::KeyLine>, mask: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_detect_const_MatR_vector_KeyLine_R_const_MatR(self.as_raw_mut_BinaryDescriptor(), image.as_raw_Mat(), keypoints.as_raw_mut_VectorOfKeyLine(), mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct BinaryDescriptor {
	ptr: *mut c_void
}

opencv_type_boxed! { BinaryDescriptor }

impl Drop for BinaryDescriptor {
	fn drop(&mut self) {
		extern "C" { fn cv_BinaryDescriptor_delete(instance: *mut c_void); }
		unsafe { cv_BinaryDescriptor_delete(self.as_raw_mut_BinaryDescriptor()) };
	}
}

unsafe impl Send for BinaryDescriptor {}

impl core::AlgorithmTraitConst for BinaryDescriptor {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BinaryDescriptor {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::line_descriptor::BinaryDescriptorTraitConst for BinaryDescriptor {
	#[inline] fn as_raw_BinaryDescriptor(&self) -> *const c_void { self.as_raw() }
}

impl crate::line_descriptor::BinaryDescriptorTrait for BinaryDescriptor {
	#[inline] fn as_raw_mut_BinaryDescriptor(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BinaryDescriptor {
	/// ## C++ default parameters
	/// * parameters: BinaryDescriptor::Params()
	#[inline]
	pub fn new(parameters: &crate::line_descriptor::BinaryDescriptor_Params) -> Result<crate::line_descriptor::BinaryDescriptor> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_BinaryDescriptor_const_ParamsR(parameters.as_raw_BinaryDescriptor_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::line_descriptor::BinaryDescriptor::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn create_binary_descriptor() -> Result<core::Ptr<crate::line_descriptor::BinaryDescriptor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_createBinaryDescriptor(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::line_descriptor::BinaryDescriptor>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn create_binary_descriptor_1(mut parameters: crate::line_descriptor::BinaryDescriptor_Params) -> Result<core::Ptr<crate::line_descriptor::BinaryDescriptor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_createBinaryDescriptor_Params(parameters.as_raw_mut_BinaryDescriptor_Params(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::line_descriptor::BinaryDescriptor>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { BinaryDescriptor, core::Algorithm, cv_BinaryDescriptor_to_Algorithm }

pub trait BinaryDescriptor_ParamsTraitConst {
	fn as_raw_BinaryDescriptor_Params(&self) -> *const c_void;

	#[inline]
	fn num_of_octave_(&self) -> i32 {
		let ret = unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_getPropNumOfOctave__const(self.as_raw_BinaryDescriptor_Params()) };
		ret
	}
	
	#[inline]
	fn width_of_band_(&self) -> i32 {
		let ret = unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_getPropWidthOfBand__const(self.as_raw_BinaryDescriptor_Params()) };
		ret
	}
	
	#[inline]
	fn reduction_ratio(&self) -> i32 {
		let ret = unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_getPropReductionRatio_const(self.as_raw_BinaryDescriptor_Params()) };
		ret
	}
	
	#[inline]
	fn ksize_(&self) -> i32 {
		let ret = unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_getPropKsize__const(self.as_raw_BinaryDescriptor_Params()) };
		ret
	}
	
	#[inline]
	fn write(&self, fs: &mut core::FileStorage) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_write_const_FileStorageR(self.as_raw_BinaryDescriptor_Params(), fs.as_raw_mut_FileStorage(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait BinaryDescriptor_ParamsTrait: crate::line_descriptor::BinaryDescriptor_ParamsTraitConst {
	fn as_raw_mut_BinaryDescriptor_Params(&mut self) -> *mut c_void;

	#[inline]
	fn set_num_of_octave_(&mut self, val: i32) {
		let ret = unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_setPropNumOfOctave__int(self.as_raw_mut_BinaryDescriptor_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_width_of_band_(&mut self, val: i32) {
		let ret = unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_setPropWidthOfBand__int(self.as_raw_mut_BinaryDescriptor_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_reduction_ratio(&mut self, val: i32) {
		let ret = unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_setPropReductionRatio_int(self.as_raw_mut_BinaryDescriptor_Params(), val) };
		ret
	}
	
	#[inline]
	fn set_ksize_(&mut self, val: i32) {
		let ret = unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_setPropKsize__int(self.as_raw_mut_BinaryDescriptor_Params(), val) };
		ret
	}
	
	#[inline]
	fn read(&mut self, fn_: &core::FileNode) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_read_const_FileNodeR(self.as_raw_mut_BinaryDescriptor_Params(), fn_.as_raw_FileNode(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct BinaryDescriptor_Params {
	ptr: *mut c_void
}

opencv_type_boxed! { BinaryDescriptor_Params }

impl Drop for BinaryDescriptor_Params {
	fn drop(&mut self) {
		extern "C" { fn cv_BinaryDescriptor_Params_delete(instance: *mut c_void); }
		unsafe { cv_BinaryDescriptor_Params_delete(self.as_raw_mut_BinaryDescriptor_Params()) };
	}
}

unsafe impl Send for BinaryDescriptor_Params {}

impl crate::line_descriptor::BinaryDescriptor_ParamsTraitConst for BinaryDescriptor_Params {
	#[inline] fn as_raw_BinaryDescriptor_Params(&self) -> *const c_void { self.as_raw() }
}

impl crate::line_descriptor::BinaryDescriptor_ParamsTrait for BinaryDescriptor_Params {
	#[inline] fn as_raw_mut_BinaryDescriptor_Params(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BinaryDescriptor_Params {
	#[inline]
	pub fn default() -> Result<crate::line_descriptor::BinaryDescriptor_Params> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptor_Params_Params(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::line_descriptor::BinaryDescriptor_Params::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait BinaryDescriptorMatcherTraitConst: core::AlgorithmTraitConst {
	fn as_raw_BinaryDescriptorMatcher(&self) -> *const c_void;

	/// ## C++ default parameters
	/// * mask: Mat()
	#[inline]
	fn match_(&self, query_descriptors: &core::Mat, train_descriptors: &core::Mat, matches: &mut core::Vector<core::DMatch>, mask: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_match_const_const_MatR_const_MatR_vector_DMatch_R_const_MatR(self.as_raw_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), train_descriptors.as_raw_Mat(), matches.as_raw_mut_VectorOfDMatch(), mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * mask: Mat()
	/// * compact_result: false
	#[inline]
	fn knn_match(&self, query_descriptors: &core::Mat, train_descriptors: &core::Mat, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32, mask: &core::Mat, compact_result: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_const_MatR_const_MatR_vector_vector_DMatch__R_int_const_MatR_bool(self.as_raw_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), train_descriptors.as_raw_Mat(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, mask.as_raw_Mat(), compact_result, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * mask: Mat()
	/// * compact_result: false
	#[inline]
	fn radius_match(&self, query_descriptors: &core::Mat, train_descriptors: &core::Mat, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32, mask: &core::Mat, compact_result: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_const_MatR_const_MatR_vector_vector_DMatch__R_float_const_MatR_bool(self.as_raw_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), train_descriptors.as_raw_Mat(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, mask.as_raw_Mat(), compact_result, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait BinaryDescriptorMatcherTrait: core::AlgorithmTrait + crate::line_descriptor::BinaryDescriptorMatcherTraitConst {
	fn as_raw_mut_BinaryDescriptorMatcher(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * masks: std::vector<Mat>()
	#[inline]
	fn match_query(&mut self, query_descriptors: &core::Mat, matches: &mut core::Vector<core::DMatch>, masks: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_match_const_MatR_vector_DMatch_R_const_vector_Mat_R(self.as_raw_mut_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), matches.as_raw_mut_VectorOfDMatch(), masks.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * masks: std::vector<Mat>()
	/// * compact_result: false
	#[inline]
	fn knn_match_query(&mut self, query_descriptors: &core::Mat, matches: &mut core::Vector<core::Vector<core::DMatch>>, k: i32, masks: &core::Vector<core::Mat>, compact_result: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_knnMatch_const_MatR_vector_vector_DMatch__R_int_const_vector_Mat_R_bool(self.as_raw_mut_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), matches.as_raw_mut_VectorOfVectorOfDMatch(), k, masks.as_raw_VectorOfMat(), compact_result, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * masks: std::vector<Mat>()
	/// * compact_result: false
	#[inline]
	fn radius_match_1(&mut self, query_descriptors: &core::Mat, matches: &mut core::Vector<core::Vector<core::DMatch>>, max_distance: f32, masks: &core::Vector<core::Mat>, compact_result: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_radiusMatch_const_MatR_vector_vector_DMatch__R_float_const_vector_Mat_R_bool(self.as_raw_mut_BinaryDescriptorMatcher(), query_descriptors.as_raw_Mat(), matches.as_raw_mut_VectorOfVectorOfDMatch(), max_distance, masks.as_raw_VectorOfMat(), compact_result, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn add(&mut self, descriptors: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_add_const_vector_Mat_R(self.as_raw_mut_BinaryDescriptorMatcher(), descriptors.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn train(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_train(self.as_raw_mut_BinaryDescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	fn clear(&mut self) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_clear(self.as_raw_mut_BinaryDescriptorMatcher(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct BinaryDescriptorMatcher {
	ptr: *mut c_void
}

opencv_type_boxed! { BinaryDescriptorMatcher }

impl Drop for BinaryDescriptorMatcher {
	fn drop(&mut self) {
		extern "C" { fn cv_BinaryDescriptorMatcher_delete(instance: *mut c_void); }
		unsafe { cv_BinaryDescriptorMatcher_delete(self.as_raw_mut_BinaryDescriptorMatcher()) };
	}
}

unsafe impl Send for BinaryDescriptorMatcher {}

impl core::AlgorithmTraitConst for BinaryDescriptorMatcher {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for BinaryDescriptorMatcher {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::line_descriptor::BinaryDescriptorMatcherTraitConst for BinaryDescriptorMatcher {
	#[inline] fn as_raw_BinaryDescriptorMatcher(&self) -> *const c_void { self.as_raw() }
}

impl crate::line_descriptor::BinaryDescriptorMatcherTrait for BinaryDescriptorMatcher {
	#[inline] fn as_raw_mut_BinaryDescriptorMatcher(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl BinaryDescriptorMatcher {
	#[inline]
	pub fn create_binary_descriptor_matcher() -> Result<core::Ptr<crate::line_descriptor::BinaryDescriptorMatcher>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_createBinaryDescriptorMatcher(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::line_descriptor::BinaryDescriptorMatcher>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn default() -> Result<crate::line_descriptor::BinaryDescriptorMatcher> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_BinaryDescriptorMatcher_BinaryDescriptorMatcher(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::line_descriptor::BinaryDescriptorMatcher::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { BinaryDescriptorMatcher, core::Algorithm, cv_BinaryDescriptorMatcher_to_Algorithm }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DrawLinesMatchesFlags {
	__rust_private: [u8; 0],
}

opencv_type_simple! { crate::line_descriptor::DrawLinesMatchesFlags }

impl DrawLinesMatchesFlags {
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KeyLine {
	pub angle: f32,
	pub class_id: i32,
	pub octave: i32,
	pub pt: core::Point2f,
	pub response: f32,
	pub size: f32,
	pub start_point_x: f32,
	pub start_point_y: f32,
	pub end_point_x: f32,
	pub end_point_y: f32,
	pub s_point_in_octave_x: f32,
	pub s_point_in_octave_y: f32,
	pub e_point_in_octave_x: f32,
	pub e_point_in_octave_y: f32,
	pub line_length: f32,
	pub num_of_pixels: i32,
}

opencv_type_simple! { crate::line_descriptor::KeyLine }

impl KeyLine {
	#[inline]
	pub fn get_start_point(self) -> Result<core::Point2f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_KeyLine_getStartPoint_const(self.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn get_end_point(self) -> Result<core::Point2f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_KeyLine_getEndPoint_const(self.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn get_start_point_in_octave(self) -> Result<core::Point2f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_KeyLine_getStartPointInOctave_const(self.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn get_end_point_in_octave(self) -> Result<core::Point2f> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_KeyLine_getEndPointInOctave_const(self.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	#[inline]
	pub fn default() -> Result<crate::line_descriptor::KeyLine> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_KeyLine_KeyLine(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait LSDDetectorTraitConst: core::AlgorithmTraitConst {
	fn as_raw_LSDDetector(&self) -> *const c_void;

	/// ## C++ default parameters
	/// * masks: std::vector<Mat>()
	#[inline]
	fn detect_multiple(&self, images: &core::Vector<core::Mat>, keylines: &mut core::Vector<core::Vector<crate::line_descriptor::KeyLine>>, scale: i32, num_octaves: i32, masks: &core::Vector<core::Mat>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_LSDDetector_detect_const_const_vector_Mat_R_vector_vector_KeyLine__R_int_int_const_vector_Mat_R(self.as_raw_LSDDetector(), images.as_raw_VectorOfMat(), keylines.as_raw_mut_VectorOfVectorOfKeyLine(), scale, num_octaves, masks.as_raw_VectorOfMat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait LSDDetectorTrait: core::AlgorithmTrait + crate::line_descriptor::LSDDetectorTraitConst {
	fn as_raw_mut_LSDDetector(&mut self) -> *mut c_void;

	/// ## C++ default parameters
	/// * mask: Mat()
	#[inline]
	fn detect(&mut self, image: &core::Mat, keypoints: &mut core::Vector<crate::line_descriptor::KeyLine>, scale: i32, num_octaves: i32, mask: &core::Mat) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_LSDDetector_detect_const_MatR_vector_KeyLine_R_int_int_const_MatR(self.as_raw_mut_LSDDetector(), image.as_raw_Mat(), keypoints.as_raw_mut_VectorOfKeyLine(), scale, num_octaves, mask.as_raw_Mat(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub struct LSDDetector {
	ptr: *mut c_void
}

opencv_type_boxed! { LSDDetector }

impl Drop for LSDDetector {
	fn drop(&mut self) {
		extern "C" { fn cv_LSDDetector_delete(instance: *mut c_void); }
		unsafe { cv_LSDDetector_delete(self.as_raw_mut_LSDDetector()) };
	}
}

unsafe impl Send for LSDDetector {}

impl core::AlgorithmTraitConst for LSDDetector {
	#[inline] fn as_raw_Algorithm(&self) -> *const c_void { self.as_raw() }
}

impl core::AlgorithmTrait for LSDDetector {
	#[inline] fn as_raw_mut_Algorithm(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl crate::line_descriptor::LSDDetectorTraitConst for LSDDetector {
	#[inline] fn as_raw_LSDDetector(&self) -> *const c_void { self.as_raw() }
}

impl crate::line_descriptor::LSDDetectorTrait for LSDDetector {
	#[inline] fn as_raw_mut_LSDDetector(&mut self) -> *mut c_void { self.as_raw_mut() }
}

impl LSDDetector {
	#[inline]
	pub fn default() -> Result<crate::line_descriptor::LSDDetector> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_LSDDetector_LSDDetector(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::line_descriptor::LSDDetector::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn new(_params: crate::line_descriptor::LSDParam) -> Result<crate::line_descriptor::LSDDetector> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_LSDDetector_LSDDetector_LSDParam(_params.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { crate::line_descriptor::LSDDetector::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn create_lsd_detector() -> Result<core::Ptr<crate::line_descriptor::LSDDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_LSDDetector_createLSDDetector(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::line_descriptor::LSDDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	#[inline]
	pub fn create_lsd_detector_with_params(params: crate::line_descriptor::LSDParam) -> Result<core::Ptr<crate::line_descriptor::LSDDetector>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_LSDDetector_createLSDDetector_LSDParam(params.opencv_as_extern(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<crate::line_descriptor::LSDDetector>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

boxed_cast_base! { LSDDetector, core::Algorithm, cv_LSDDetector_to_Algorithm }

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LSDParam {
	pub scale: f64,
	pub sigma_scale: f64,
	pub quant: f64,
	pub ang_th: f64,
	pub log_eps: f64,
	pub density_th: f64,
	pub n_bins: i32,
}

opencv_type_simple! { crate::line_descriptor::LSDParam }

impl LSDParam {
	#[inline]
	pub fn default() -> Result<crate::line_descriptor::LSDParam> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_line_descriptor_LSDParam_LSDParam(ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}
