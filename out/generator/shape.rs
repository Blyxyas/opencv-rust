#![allow(
	unused_parens,
	clippy::excessive_precision,
	clippy::missing_safety_doc,
	clippy::not_unsafe_ptr_arg_deref,
	clippy::should_implement_trait,
	clippy::too_many_arguments,
	clippy::unused_unit,
)]
//! # Shape Distance and Matching
use crate::{mod_prelude::*, core, sys, types};
pub mod prelude {
	pub use { super::ShapeTransformerConst, super::ShapeTransformer, super::ThinPlateSplineShapeTransformerConst, super::ThinPlateSplineShapeTransformer, super::AffineTransformerConst, super::AffineTransformer, super::HistogramCostExtractorConst, super::HistogramCostExtractor, super::NormHistogramCostExtractorConst, super::NormHistogramCostExtractor, super::EMDHistogramCostExtractorConst, super::EMDHistogramCostExtractor, super::ChiHistogramCostExtractorConst, super::ChiHistogramCostExtractor, super::EMDL1HistogramCostExtractorConst, super::EMDL1HistogramCostExtractor, super::ShapeDistanceExtractorConst, super::ShapeDistanceExtractor, super::ShapeContextDistanceExtractorConst, super::ShapeContextDistanceExtractor, super::HausdorffDistanceExtractorConst, super::HausdorffDistanceExtractor };
}

// EMDL1(cv::InputArray, cv::InputArray) /usr/include/opencv2/shape/emdL1.hpp:66
#[inline]
pub fn emdl1(signature1: &dyn core::ToInputArray, signature2: &dyn core::ToInputArray) -> Result<f32> {
	input_array_arg!(signature1);
	input_array_arg!(signature2);
	return_send!(via ocvrs_return);
	unsafe { sys::cv_EMDL1_const__InputArrayR_const__InputArrayR(signature1.as_raw__InputArray(), signature2.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	Ok(ret)
}

// createAffineTransformer(bool) /usr/include/opencv2/shape/shape_transformer.hpp:127
#[inline]
pub fn create_affine_transformer(full_affine: bool) -> Result<core::Ptr<dyn crate::shape::AffineTransformer>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createAffineTransformer_bool(full_affine, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::shape::AffineTransformer>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * n_dummies: 25
/// * default_cost: 0.2f
// createChiHistogramCostExtractor(int, float) /usr/include/opencv2/shape/hist_cost.hpp:98
#[inline]
pub fn create_chi_histogram_cost_extractor(n_dummies: i32, default_cost: f32) -> Result<core::Ptr<dyn crate::shape::HistogramCostExtractor>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createChiHistogramCostExtractor_int_float(n_dummies, default_cost, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::shape::HistogramCostExtractor>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * flag: DIST_L2
/// * n_dummies: 25
/// * default_cost: 0.2f
// createEMDHistogramCostExtractor(int, int, float) /usr/include/opencv2/shape/hist_cost.hpp:91
#[inline]
pub fn create_emd_histogram_cost_extractor(flag: i32, n_dummies: i32, default_cost: f32) -> Result<core::Ptr<dyn crate::shape::HistogramCostExtractor>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createEMDHistogramCostExtractor_int_int_float(flag, n_dummies, default_cost, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::shape::HistogramCostExtractor>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * n_dummies: 25
/// * default_cost: 0.2f
// createEMDL1HistogramCostExtractor(int, float) /usr/include/opencv2/shape/hist_cost.hpp:106
#[inline]
pub fn create_emdl1_histogram_cost_extractor(n_dummies: i32, default_cost: f32) -> Result<core::Ptr<dyn crate::shape::HistogramCostExtractor>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createEMDL1HistogramCostExtractor_int_float(n_dummies, default_cost, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::shape::HistogramCostExtractor>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * distance_flag: cv::NORM_L2
/// * rank_prop: 0.6f
// createHausdorffDistanceExtractor(int, float) /usr/include/opencv2/shape/shape_distance.hpp:222
#[inline]
pub fn create_hausdorff_distance_extractor(distance_flag: i32, rank_prop: f32) -> Result<core::Ptr<dyn crate::shape::HausdorffDistanceExtractor>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createHausdorffDistanceExtractor_int_float(distance_flag, rank_prop, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::shape::HausdorffDistanceExtractor>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * flag: DIST_L2
/// * n_dummies: 25
/// * default_cost: 0.2f
// createNormHistogramCostExtractor(int, int, float) /usr/include/opencv2/shape/hist_cost.hpp:79
#[inline]
pub fn create_norm_histogram_cost_extractor(flag: i32, n_dummies: i32, default_cost: f32) -> Result<core::Ptr<dyn crate::shape::HistogramCostExtractor>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createNormHistogramCostExtractor_int_int_float(flag, n_dummies, default_cost, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::shape::HistogramCostExtractor>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * n_angular_bins: 12
/// * n_radial_bins: 4
/// * inner_radius: 0.2f
/// * outer_radius: 2
/// * iterations: 3
/// * comparer: createChiHistogramCostExtractor()
/// * transformer: createThinPlateSplineShapeTransformer()
// createShapeContextDistanceExtractor(int, int, float, float, int, const Ptr<cv::HistogramCostExtractor> &, const Ptr<cv::ShapeTransformer> &) /usr/include/opencv2/shape/shape_distance.hpp:187
#[inline]
pub fn create_shape_context_distance_extractor(n_angular_bins: i32, n_radial_bins: i32, inner_radius: f32, outer_radius: f32, iterations: i32, comparer: &core::Ptr<dyn crate::shape::HistogramCostExtractor>, transformer: &core::Ptr<dyn crate::shape::ShapeTransformer>) -> Result<core::Ptr<dyn crate::shape::ShapeContextDistanceExtractor>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createShapeContextDistanceExtractor_int_int_float_float_int_const_Ptr_HistogramCostExtractor_R_const_Ptr_ShapeTransformer_R(n_angular_bins, n_radial_bins, inner_radius, outer_radius, iterations, comparer.as_raw_PtrOfHistogramCostExtractor(), transformer.as_raw_PtrOfShapeTransformer(), ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::shape::ShapeContextDistanceExtractor>::opencv_from_extern(ret) };
	Ok(ret)
}

/// ## C++ default parameters
/// * regularization_parameter: 0
// createThinPlateSplineShapeTransformer(double) /usr/include/opencv2/shape/shape_transformer.hpp:112
#[inline]
pub fn create_thin_plate_spline_shape_transformer(regularization_parameter: f64) -> Result<core::Ptr<dyn crate::shape::ThinPlateSplineShapeTransformer>> {
	return_send!(via ocvrs_return);
	unsafe { sys::cv_createThinPlateSplineShapeTransformer_double(regularization_parameter, ocvrs_return.as_mut_ptr()) };
	return_receive!(unsafe ocvrs_return => ret);
	let ret = ret.into_result()?;
	let ret = unsafe { core::Ptr::<dyn crate::shape::ThinPlateSplineShapeTransformer>::opencv_from_extern(ret) };
	Ok(ret)
}

// AffineTransformer /usr/include/opencv2/shape/shape_transformer.hpp:119
pub trait AffineTransformerConst: crate::shape::ShapeTransformerConst {
	fn as_raw_AffineTransformer(&self) -> *const c_void;

	// getFullAffine() /usr/include/opencv2/shape/shape_transformer.hpp:123
	#[inline]
	fn get_full_affine(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AffineTransformer_getFullAffine_const(self.as_raw_AffineTransformer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait AffineTransformer: crate::shape::AffineTransformerConst + crate::shape::ShapeTransformer {
	fn as_raw_mut_AffineTransformer(&mut self) -> *mut c_void;

	// setFullAffine(bool) /usr/include/opencv2/shape/shape_transformer.hpp:122
	#[inline]
	fn set_full_affine(&mut self, full_affine: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_AffineTransformer_setFullAffine_bool(self.as_raw_mut_AffineTransformer(), full_affine, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// ChiHistogramCostExtractor /usr/include/opencv2/shape/hist_cost.hpp:95
pub trait ChiHistogramCostExtractorConst: crate::shape::HistogramCostExtractorConst {
	fn as_raw_ChiHistogramCostExtractor(&self) -> *const c_void;

}

pub trait ChiHistogramCostExtractor: crate::shape::ChiHistogramCostExtractorConst + crate::shape::HistogramCostExtractor {
	fn as_raw_mut_ChiHistogramCostExtractor(&mut self) -> *mut c_void;

}

// EMDHistogramCostExtractor /usr/include/opencv2/shape/hist_cost.hpp:83
pub trait EMDHistogramCostExtractorConst: crate::shape::HistogramCostExtractorConst {
	fn as_raw_EMDHistogramCostExtractor(&self) -> *const c_void;

	// getNormFlag() /usr/include/opencv2/shape/hist_cost.hpp:87
	#[inline]
	fn get_norm_flag(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_EMDHistogramCostExtractor_getNormFlag_const(self.as_raw_EMDHistogramCostExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait EMDHistogramCostExtractor: crate::shape::EMDHistogramCostExtractorConst + crate::shape::HistogramCostExtractor {
	fn as_raw_mut_EMDHistogramCostExtractor(&mut self) -> *mut c_void;

	// setNormFlag(int) /usr/include/opencv2/shape/hist_cost.hpp:86
	#[inline]
	fn set_norm_flag(&mut self, flag: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_EMDHistogramCostExtractor_setNormFlag_int(self.as_raw_mut_EMDHistogramCostExtractor(), flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// EMDL1HistogramCostExtractor /usr/include/opencv2/shape/hist_cost.hpp:102
pub trait EMDL1HistogramCostExtractorConst: crate::shape::HistogramCostExtractorConst {
	fn as_raw_EMDL1HistogramCostExtractor(&self) -> *const c_void;

}

pub trait EMDL1HistogramCostExtractor: crate::shape::EMDL1HistogramCostExtractorConst + crate::shape::HistogramCostExtractor {
	fn as_raw_mut_EMDL1HistogramCostExtractor(&mut self) -> *mut c_void;

}

// HausdorffDistanceExtractor /usr/include/opencv2/shape/shape_distance.hpp:200
pub trait HausdorffDistanceExtractorConst: crate::shape::ShapeDistanceExtractorConst {
	fn as_raw_HausdorffDistanceExtractor(&self) -> *const c_void;

	// getDistanceFlag() /usr/include/opencv2/shape/shape_distance.hpp:209
	#[inline]
	fn get_distance_flag(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HausdorffDistanceExtractor_getDistanceFlag_const(self.as_raw_HausdorffDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getRankProportion() /usr/include/opencv2/shape/shape_distance.hpp:218
	#[inline]
	fn get_rank_proportion(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HausdorffDistanceExtractor_getRankProportion_const(self.as_raw_HausdorffDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait HausdorffDistanceExtractor: crate::shape::HausdorffDistanceExtractorConst + crate::shape::ShapeDistanceExtractor {
	fn as_raw_mut_HausdorffDistanceExtractor(&mut self) -> *mut c_void;

	// setDistanceFlag(int) /usr/include/opencv2/shape/shape_distance.hpp:208
	#[inline]
	fn set_distance_flag(&mut self, distance_flag: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HausdorffDistanceExtractor_setDistanceFlag_int(self.as_raw_mut_HausdorffDistanceExtractor(), distance_flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setRankProportion(float) /usr/include/opencv2/shape/shape_distance.hpp:217
	#[inline]
	fn set_rank_proportion(&mut self, rank_proportion: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HausdorffDistanceExtractor_setRankProportion_float(self.as_raw_mut_HausdorffDistanceExtractor(), rank_proportion, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// HistogramCostExtractor /usr/include/opencv2/shape/hist_cost.hpp:57
pub trait HistogramCostExtractorConst: core::AlgorithmTraitConst {
	fn as_raw_HistogramCostExtractor(&self) -> *const c_void;

	// getNDummies() /usr/include/opencv2/shape/hist_cost.hpp:63
	#[inline]
	fn get_n_dummies(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HistogramCostExtractor_getNDummies_const(self.as_raw_HistogramCostExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getDefaultCost() /usr/include/opencv2/shape/hist_cost.hpp:66
	#[inline]
	fn get_default_cost(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HistogramCostExtractor_getDefaultCost_const(self.as_raw_HistogramCostExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait HistogramCostExtractor: core::AlgorithmTrait + crate::shape::HistogramCostExtractorConst {
	fn as_raw_mut_HistogramCostExtractor(&mut self) -> *mut c_void;

	// buildCostMatrix(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/shape/hist_cost.hpp:60
	#[inline]
	fn build_cost_matrix(&mut self, descriptors1: &dyn core::ToInputArray, descriptors2: &dyn core::ToInputArray, cost_matrix: &mut dyn core::ToOutputArray) -> Result<()> {
		input_array_arg!(descriptors1);
		input_array_arg!(descriptors2);
		output_array_arg!(cost_matrix);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HistogramCostExtractor_buildCostMatrix_const__InputArrayR_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_HistogramCostExtractor(), descriptors1.as_raw__InputArray(), descriptors2.as_raw__InputArray(), cost_matrix.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setNDummies(int) /usr/include/opencv2/shape/hist_cost.hpp:62
	#[inline]
	fn set_n_dummies(&mut self, n_dummies: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HistogramCostExtractor_setNDummies_int(self.as_raw_mut_HistogramCostExtractor(), n_dummies, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setDefaultCost(float) /usr/include/opencv2/shape/hist_cost.hpp:65
	#[inline]
	fn set_default_cost(&mut self, default_cost: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_HistogramCostExtractor_setDefaultCost_float(self.as_raw_mut_HistogramCostExtractor(), default_cost, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// NormHistogramCostExtractor /usr/include/opencv2/shape/hist_cost.hpp:71
pub trait NormHistogramCostExtractorConst: crate::shape::HistogramCostExtractorConst {
	fn as_raw_NormHistogramCostExtractor(&self) -> *const c_void;

	// getNormFlag() /usr/include/opencv2/shape/hist_cost.hpp:75
	#[inline]
	fn get_norm_flag(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_NormHistogramCostExtractor_getNormFlag_const(self.as_raw_NormHistogramCostExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait NormHistogramCostExtractor: crate::shape::HistogramCostExtractor + crate::shape::NormHistogramCostExtractorConst {
	fn as_raw_mut_NormHistogramCostExtractor(&mut self) -> *mut c_void;

	// setNormFlag(int) /usr/include/opencv2/shape/hist_cost.hpp:74
	#[inline]
	fn set_norm_flag(&mut self, flag: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_NormHistogramCostExtractor_setNormFlag_int(self.as_raw_mut_NormHistogramCostExtractor(), flag, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// ShapeContextDistanceExtractor /usr/include/opencv2/shape/shape_distance.hpp:81
pub trait ShapeContextDistanceExtractorConst: crate::shape::ShapeDistanceExtractorConst {
	fn as_raw_ShapeContextDistanceExtractor(&self) -> *const c_void;

	// getAngularBins() /usr/include/opencv2/shape/shape_distance.hpp:90
	#[inline]
	fn get_angular_bins(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getAngularBins_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getRadialBins() /usr/include/opencv2/shape/shape_distance.hpp:98
	#[inline]
	fn get_radial_bins(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getRadialBins_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getInnerRadius() /usr/include/opencv2/shape/shape_distance.hpp:105
	#[inline]
	fn get_inner_radius(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getInnerRadius_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getOuterRadius() /usr/include/opencv2/shape/shape_distance.hpp:112
	#[inline]
	fn get_outer_radius(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getOuterRadius_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getRotationInvariant() /usr/include/opencv2/shape/shape_distance.hpp:115
	#[inline]
	fn get_rotation_invariant(&self) -> Result<bool> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getRotationInvariant_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getShapeContextWeight() /usr/include/opencv2/shape/shape_distance.hpp:125
	#[inline]
	fn get_shape_context_weight(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getShapeContextWeight_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getImageAppearanceWeight() /usr/include/opencv2/shape/shape_distance.hpp:137
	#[inline]
	fn get_image_appearance_weight(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getImageAppearanceWeight_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getBendingEnergyWeight() /usr/include/opencv2/shape/shape_distance.hpp:147
	#[inline]
	fn get_bending_energy_weight(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getBendingEnergyWeight_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getImages(cv::OutputArray, cv::OutputArray) /usr/include/opencv2/shape/shape_distance.hpp:156
	#[inline]
	fn get_images(&self, image1: &mut dyn core::ToOutputArray, image2: &mut dyn core::ToOutputArray) -> Result<()> {
		output_array_arg!(image1);
		output_array_arg!(image2);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getImages_const_const__OutputArrayR_const__OutputArrayR(self.as_raw_ShapeContextDistanceExtractor(), image1.as_raw__OutputArray(), image2.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getIterations() /usr/include/opencv2/shape/shape_distance.hpp:159
	#[inline]
	fn get_iterations(&self) -> Result<i32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getIterations_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getCostExtractor() /usr/include/opencv2/shape/shape_distance.hpp:167
	#[inline]
	fn get_cost_extractor(&self) -> Result<core::Ptr<dyn crate::shape::HistogramCostExtractor>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getCostExtractor_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::shape::HistogramCostExtractor>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
	// getStdDev() /usr/include/opencv2/shape/shape_distance.hpp:174
	#[inline]
	fn get_std_dev(&self) -> Result<f32> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getStdDev_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// getTransformAlgorithm() /usr/include/opencv2/shape/shape_distance.hpp:182
	#[inline]
	fn get_transform_algorithm(&self) -> Result<core::Ptr<dyn crate::shape::ShapeTransformer>> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_getTransformAlgorithm_const(self.as_raw_ShapeContextDistanceExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		let ret = unsafe { core::Ptr::<dyn crate::shape::ShapeTransformer>::opencv_from_extern(ret) };
		Ok(ret)
	}
	
}

pub trait ShapeContextDistanceExtractor: crate::shape::ShapeContextDistanceExtractorConst + crate::shape::ShapeDistanceExtractor {
	fn as_raw_mut_ShapeContextDistanceExtractor(&mut self) -> *mut c_void;

	// setAngularBins(int) /usr/include/opencv2/shape/shape_distance.hpp:89
	#[inline]
	fn set_angular_bins(&mut self, n_angular_bins: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setAngularBins_int(self.as_raw_mut_ShapeContextDistanceExtractor(), n_angular_bins, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setRadialBins(int) /usr/include/opencv2/shape/shape_distance.hpp:97
	#[inline]
	fn set_radial_bins(&mut self, n_radial_bins: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setRadialBins_int(self.as_raw_mut_ShapeContextDistanceExtractor(), n_radial_bins, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setInnerRadius(float) /usr/include/opencv2/shape/shape_distance.hpp:104
	#[inline]
	fn set_inner_radius(&mut self, inner_radius: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setInnerRadius_float(self.as_raw_mut_ShapeContextDistanceExtractor(), inner_radius, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setOuterRadius(float) /usr/include/opencv2/shape/shape_distance.hpp:111
	#[inline]
	fn set_outer_radius(&mut self, outer_radius: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setOuterRadius_float(self.as_raw_mut_ShapeContextDistanceExtractor(), outer_radius, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setRotationInvariant(bool) /usr/include/opencv2/shape/shape_distance.hpp:114
	#[inline]
	fn set_rotation_invariant(&mut self, rotation_invariant: bool) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setRotationInvariant_bool(self.as_raw_mut_ShapeContextDistanceExtractor(), rotation_invariant, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setShapeContextWeight(float) /usr/include/opencv2/shape/shape_distance.hpp:124
	#[inline]
	fn set_shape_context_weight(&mut self, shape_context_weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setShapeContextWeight_float(self.as_raw_mut_ShapeContextDistanceExtractor(), shape_context_weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setImageAppearanceWeight(float) /usr/include/opencv2/shape/shape_distance.hpp:136
	#[inline]
	fn set_image_appearance_weight(&mut self, image_appearance_weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setImageAppearanceWeight_float(self.as_raw_mut_ShapeContextDistanceExtractor(), image_appearance_weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setBendingEnergyWeight(float) /usr/include/opencv2/shape/shape_distance.hpp:146
	#[inline]
	fn set_bending_energy_weight(&mut self, bending_energy_weight: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setBendingEnergyWeight_float(self.as_raw_mut_ShapeContextDistanceExtractor(), bending_energy_weight, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setImages(cv::InputArray, cv::InputArray) /usr/include/opencv2/shape/shape_distance.hpp:155
	#[inline]
	fn set_images(&mut self, image1: &dyn core::ToInputArray, image2: &dyn core::ToInputArray) -> Result<()> {
		input_array_arg!(image1);
		input_array_arg!(image2);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setImages_const__InputArrayR_const__InputArrayR(self.as_raw_mut_ShapeContextDistanceExtractor(), image1.as_raw__InputArray(), image2.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setIterations(int) /usr/include/opencv2/shape/shape_distance.hpp:158
	#[inline]
	fn set_iterations(&mut self, iterations: i32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setIterations_int(self.as_raw_mut_ShapeContextDistanceExtractor(), iterations, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setCostExtractor(Ptr<cv::HistogramCostExtractor>) /usr/include/opencv2/shape/shape_distance.hpp:166
	#[inline]
	fn set_cost_extractor(&mut self, mut comparer: core::Ptr<dyn crate::shape::HistogramCostExtractor>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setCostExtractor_Ptr_HistogramCostExtractor_(self.as_raw_mut_ShapeContextDistanceExtractor(), comparer.as_raw_mut_PtrOfHistogramCostExtractor(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setStdDev(float) /usr/include/opencv2/shape/shape_distance.hpp:173
	#[inline]
	fn set_std_dev(&mut self, sigma: f32) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setStdDev_float(self.as_raw_mut_ShapeContextDistanceExtractor(), sigma, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	// setTransformAlgorithm(Ptr<cv::ShapeTransformer>) /usr/include/opencv2/shape/shape_distance.hpp:181
	#[inline]
	fn set_transform_algorithm(&mut self, mut transformer: core::Ptr<dyn crate::shape::ShapeTransformer>) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeContextDistanceExtractor_setTransformAlgorithm_Ptr_ShapeTransformer_(self.as_raw_mut_ShapeContextDistanceExtractor(), transformer.as_raw_mut_PtrOfShapeTransformer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// ShapeDistanceExtractor /usr/include/opencv2/shape/shape_distance.hpp:61
pub trait ShapeDistanceExtractorConst: core::AlgorithmTraitConst {
	fn as_raw_ShapeDistanceExtractor(&self) -> *const c_void;

}

pub trait ShapeDistanceExtractor: core::AlgorithmTrait + crate::shape::ShapeDistanceExtractorConst {
	fn as_raw_mut_ShapeDistanceExtractor(&mut self) -> *mut c_void;

	// computeDistance(cv::InputArray, cv::InputArray) /usr/include/opencv2/shape/shape_distance.hpp:69
	#[inline]
	fn compute_distance(&mut self, contour1: &dyn core::ToInputArray, contour2: &dyn core::ToInputArray) -> Result<f32> {
		input_array_arg!(contour1);
		input_array_arg!(contour2);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeDistanceExtractor_computeDistance_const__InputArrayR_const__InputArrayR(self.as_raw_mut_ShapeDistanceExtractor(), contour1.as_raw__InputArray(), contour2.as_raw__InputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// ShapeTransformer /usr/include/opencv2/shape/shape_transformer.hpp:58
pub trait ShapeTransformerConst: core::AlgorithmTraitConst {
	fn as_raw_ShapeTransformer(&self) -> *const c_void;

	/// ## C++ default parameters
	/// * flags: INTER_LINEAR
	/// * border_mode: BORDER_CONSTANT
	/// * border_value: Scalar()
	// warpImage(cv::InputArray, cv::OutputArray, int, int, const cv::Scalar &) /usr/include/opencv2/shape/shape_transformer.hpp:85
	#[inline]
	fn warp_image(&self, transforming_image: &dyn core::ToInputArray, output: &mut dyn core::ToOutputArray, flags: i32, border_mode: i32, border_value: core::Scalar) -> Result<()> {
		input_array_arg!(transforming_image);
		output_array_arg!(output);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeTransformer_warpImage_const_const__InputArrayR_const__OutputArrayR_int_int_const_ScalarR(self.as_raw_ShapeTransformer(), transforming_image.as_raw__InputArray(), output.as_raw__OutputArray(), flags, border_mode, &border_value, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait ShapeTransformer: core::AlgorithmTrait + crate::shape::ShapeTransformerConst {
	fn as_raw_mut_ShapeTransformer(&mut self) -> *mut c_void;

	// estimateTransformation(cv::InputArray, cv::InputArray, std::vector<DMatch> &) /usr/include/opencv2/shape/shape_transformer.hpp:67
	#[inline]
	fn estimate_transformation(&mut self, transforming_shape: &dyn core::ToInputArray, target_shape: &dyn core::ToInputArray, matches: &mut core::Vector<core::DMatch>) -> Result<()> {
		input_array_arg!(transforming_shape);
		input_array_arg!(target_shape);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeTransformer_estimateTransformation_const__InputArrayR_const__InputArrayR_vector_DMatch_R(self.as_raw_mut_ShapeTransformer(), transforming_shape.as_raw__InputArray(), target_shape.as_raw__InputArray(), matches.as_raw_mut_VectorOfDMatch(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
	/// ## C++ default parameters
	/// * output: noArray()
	// applyTransformation(cv::InputArray, cv::OutputArray) /usr/include/opencv2/shape/shape_transformer.hpp:75
	#[inline]
	fn apply_transformation(&mut self, input: &dyn core::ToInputArray, output: &mut dyn core::ToOutputArray) -> Result<f32> {
		input_array_arg!(input);
		output_array_arg!(output);
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ShapeTransformer_applyTransformation_const__InputArrayR_const__OutputArrayR(self.as_raw_mut_ShapeTransformer(), input.as_raw__InputArray(), output.as_raw__OutputArray(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

// ThinPlateSplineShapeTransformer /usr/include/opencv2/shape/shape_transformer.hpp:98
pub trait ThinPlateSplineShapeTransformerConst: crate::shape::ShapeTransformerConst {
	fn as_raw_ThinPlateSplineShapeTransformer(&self) -> *const c_void;

	// getRegularizationParameter() /usr/include/opencv2/shape/shape_transformer.hpp:107
	#[inline]
	fn get_regularization_parameter(&self) -> Result<f64> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ThinPlateSplineShapeTransformer_getRegularizationParameter_const(self.as_raw_ThinPlateSplineShapeTransformer(), ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}

pub trait ThinPlateSplineShapeTransformer: crate::shape::ShapeTransformer + crate::shape::ThinPlateSplineShapeTransformerConst {
	fn as_raw_mut_ThinPlateSplineShapeTransformer(&mut self) -> *mut c_void;

	// setRegularizationParameter(double) /usr/include/opencv2/shape/shape_transformer.hpp:106
	#[inline]
	fn set_regularization_parameter(&mut self, beta: f64) -> Result<()> {
		return_send!(via ocvrs_return);
		unsafe { sys::cv_ThinPlateSplineShapeTransformer_setRegularizationParameter_double(self.as_raw_mut_ThinPlateSplineShapeTransformer(), beta, ocvrs_return.as_mut_ptr()) };
		return_receive!(unsafe ocvrs_return => ret);
		let ret = ret.into_result()?;
		Ok(ret)
	}
	
}
