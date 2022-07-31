extern "C" {
	// compute(cv::InputArray) /usr/include/opencv2/quality/qualitybrisque.hpp:33
	pub fn cv_quality_QualityBRISQUE_compute_const__InputArrayR(instance: *mut c_void, img: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
	// create(const cv::String &, const cv::String &) /usr/include/opencv2/quality/qualitybrisque.hpp:40
	pub fn cv_quality_QualityBRISQUE_create_const_StringR_const_StringR(model_file_path: *const c_char, range_file_path: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::Ptr<cv::ml::SVM> &, const cv::Mat &) /usr/include/opencv2/quality/qualitybrisque.hpp:47
	pub fn cv_quality_QualityBRISQUE_create_const_Ptr_SVM_R_const_MatR(model: *const c_void, range: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// compute(cv::InputArray, const cv::String &, const cv::String &) /usr/include/opencv2/quality/qualitybrisque.hpp:56
	pub fn cv_quality_QualityBRISQUE_compute_const__InputArrayR_const_StringR_const_StringR(img: *const c_void, model_file_path: *const c_char, range_file_path: *const c_char, ocvrs_return: *mut Result<core::Scalar>);
	// computeFeatures(cv::InputArray, cv::OutputArray) /usr/include/opencv2/quality/qualitybrisque.hpp:63
	pub fn cv_quality_QualityBRISQUE_computeFeatures_const__InputArrayR_const__OutputArrayR(img: *const c_void, features: *const c_void, ocvrs_return: *mut Result_void);
	// compute(cv::InputArray) /usr/include/opencv2/quality/qualitybase.hpp:35
	pub fn cv_quality_QualityBase_compute_const__InputArrayR(instance: *mut c_void, img: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
	// getQualityMap(cv::OutputArray) /usr/include/opencv2/quality/qualitybase.hpp:38
	pub fn cv_quality_QualityBase_getQualityMap_const_const__OutputArrayR(instance: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// clear() /usr/include/opencv2/quality/qualitybase.hpp:46
	pub fn cv_quality_QualityBase_clear(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// empty() /usr/include/opencv2/quality/qualitybase.hpp:49
	pub fn cv_quality_QualityBase_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// compute(cv::InputArray) /usr/include/opencv2/quality/qualitygmsd.hpp:28
	pub fn cv_quality_QualityGMSD_compute_const__InputArrayR(instance: *mut c_void, cmp: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
	// empty() /usr/include/opencv2/quality/qualitygmsd.hpp:31
	pub fn cv_quality_QualityGMSD_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// clear() /usr/include/opencv2/quality/qualitygmsd.hpp:34
	pub fn cv_quality_QualityGMSD_clear(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// create(cv::InputArray) /usr/include/opencv2/quality/qualitygmsd.hpp:40
	pub fn cv_quality_QualityGMSD_create_const__InputArrayR(ref_: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// compute(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/quality/qualitygmsd.hpp:49
	pub fn cv_quality_QualityGMSD_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(ref_: *const c_void, cmp: *const c_void, quality_map: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
	// compute(cv::InputArrayOfArrays) /usr/include/opencv2/quality/qualitymse.hpp:25
	pub fn cv_quality_QualityMSE_compute_const__InputArrayR(instance: *mut c_void, cmp_imgs: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
	// empty() /usr/include/opencv2/quality/qualitymse.hpp:28
	pub fn cv_quality_QualityMSE_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// clear() /usr/include/opencv2/quality/qualitymse.hpp:31
	pub fn cv_quality_QualityMSE_clear(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// create(cv::InputArray) /usr/include/opencv2/quality/qualitymse.hpp:37
	pub fn cv_quality_QualityMSE_create_const__InputArrayR(ref_: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// compute(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/quality/qualitymse.hpp:46
	pub fn cv_quality_QualityMSE_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(ref_: *const c_void, cmp: *const c_void, quality_map: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
	// create(cv::InputArray, double) /usr/include/opencv2/quality/qualitypsnr.hpp:38
	pub fn cv_quality_QualityPSNR_create_const__InputArrayR_double(ref_: *const c_void, max_pixel_value: f64, ocvrs_return: *mut Result<*mut c_void>);
	// compute(cv::InputArray) /usr/include/opencv2/quality/qualitypsnr.hpp:48
	pub fn cv_quality_QualityPSNR_compute_const__InputArrayR(instance: *mut c_void, cmp: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
	// empty() /usr/include/opencv2/quality/qualitypsnr.hpp:59
	pub fn cv_quality_QualityPSNR_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// clear() /usr/include/opencv2/quality/qualitypsnr.hpp:62
	pub fn cv_quality_QualityPSNR_clear(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// compute(cv::InputArray, cv::InputArray, cv::OutputArray, double) /usr/include/opencv2/quality/qualitypsnr.hpp:72
	pub fn cv_quality_QualityPSNR_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double(ref_: *const c_void, cmp: *const c_void, quality_map: *const c_void, max_pixel_value: f64, ocvrs_return: *mut Result<core::Scalar>);
	// getMaxPixelValue() /usr/include/opencv2/quality/qualitypsnr.hpp:81
	pub fn cv_quality_QualityPSNR_getMaxPixelValue_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setMaxPixelValue(double) /usr/include/opencv2/quality/qualitypsnr.hpp:87
	pub fn cv_quality_QualityPSNR_setMaxPixelValue_double(instance: *mut c_void, val: f64, ocvrs_return: *mut Result_void);
	// compute(cv::InputArray) /usr/include/opencv2/quality/qualityssim.hpp:27
	pub fn cv_quality_QualitySSIM_compute_const__InputArrayR(instance: *mut c_void, cmp: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
	// empty() /usr/include/opencv2/quality/qualityssim.hpp:30
	pub fn cv_quality_QualitySSIM_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// clear() /usr/include/opencv2/quality/qualityssim.hpp:33
	pub fn cv_quality_QualitySSIM_clear(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// create(cv::InputArray) /usr/include/opencv2/quality/qualityssim.hpp:39
	pub fn cv_quality_QualitySSIM_create_const__InputArrayR(ref_: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// compute(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/quality/qualityssim.hpp:48
	pub fn cv_quality_QualitySSIM_compute_const__InputArrayR_const__InputArrayR_const__OutputArrayR(ref_: *const c_void, cmp: *const c_void, quality_map: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
}
