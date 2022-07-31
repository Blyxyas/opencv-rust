extern "C" {
	// averageHash(cv::InputArray, cv::OutputArray) /usr/include/opencv2/img_hash/average_hash.hpp:33
	pub fn cv_img_hash_averageHash_const__InputArrayR_const__OutputArrayR(input_arr: *const c_void, output_arr: *const c_void, ocvrs_return: *mut Result_void);
	// blockMeanHash(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/img_hash/block_mean_hash.hpp:44
	pub fn cv_img_hash_blockMeanHash_const__InputArrayR_const__OutputArrayR_int(input_arr: *const c_void, output_arr: *const c_void, mode: i32, ocvrs_return: *mut Result_void);
	// colorMomentHash(cv::InputArray, cv::OutputArray) /usr/include/opencv2/img_hash/color_moment_hash.hpp:35
	pub fn cv_img_hash_colorMomentHash_const__InputArrayR_const__OutputArrayR(input_arr: *const c_void, output_arr: *const c_void, ocvrs_return: *mut Result_void);
	// marrHildrethHash(cv::InputArray, cv::OutputArray, float, float) /usr/include/opencv2/img_hash/marr_hildreth_hash.hpp:56
	pub fn cv_img_hash_marrHildrethHash_const__InputArrayR_const__OutputArrayR_float_float(input_arr: *const c_void, output_arr: *const c_void, alpha: f32, scale: f32, ocvrs_return: *mut Result_void);
	// pHash(cv::InputArray, cv::OutputArray) /usr/include/opencv2/img_hash/phash.hpp:35
	pub fn cv_img_hash_pHash_const__InputArrayR_const__OutputArrayR(input_arr: *const c_void, output_arr: *const c_void, ocvrs_return: *mut Result_void);
	// radialVarianceHash(cv::InputArray, cv::OutputArray, double, int) /usr/include/opencv2/img_hash/radial_variance_hash.hpp:48
	pub fn cv_img_hash_radialVarianceHash_const__InputArrayR_const__OutputArrayR_double_int(input_arr: *const c_void, output_arr: *const c_void, sigma: f64, num_of_angle_line: i32, ocvrs_return: *mut Result_void);
	// create() /usr/include/opencv2/img_hash/average_hash.hpp:24
	pub fn cv_img_hash_AverageHash_create(ocvrs_return: *mut Result<*mut c_void>);
	// setMode(int) /usr/include/opencv2/img_hash/block_mean_hash.hpp:32
	pub fn cv_img_hash_BlockMeanHash_setMode_int(instance: *mut c_void, mode: i32, ocvrs_return: *mut Result_void);
	// getMean() /usr/include/opencv2/img_hash/block_mean_hash.hpp:33
	pub fn cv_img_hash_BlockMeanHash_getMean_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(int) /usr/include/opencv2/img_hash/block_mean_hash.hpp:34
	pub fn cv_img_hash_BlockMeanHash_create_int(mode: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create() /usr/include/opencv2/img_hash/color_moment_hash.hpp:23
	pub fn cv_img_hash_ColorMomentHash_create(ocvrs_return: *mut Result<*mut c_void>);
	// compute(cv::InputArray, cv::OutputArray) /usr/include/opencv2/img_hash/img_hash_base.hpp:28
	pub fn cv_img_hash_ImgHashBase_compute_const__InputArrayR_const__OutputArrayR(instance: *mut c_void, input_arr: *const c_void, output_arr: *const c_void, ocvrs_return: *mut Result_void);
	// compare(cv::InputArray, cv::InputArray) /usr/include/opencv2/img_hash/img_hash_base.hpp:35
	pub fn cv_img_hash_ImgHashBase_compare_const_const__InputArrayR_const__InputArrayR(instance: *const c_void, hash_one: *const c_void, hash_two: *const c_void, ocvrs_return: *mut Result<f64>);
	// getAlpha() /usr/include/opencv2/img_hash/marr_hildreth_hash.hpp:26
	pub fn cv_img_hash_MarrHildrethHash_getAlpha_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// getScale() /usr/include/opencv2/img_hash/marr_hildreth_hash.hpp:31
	pub fn cv_img_hash_MarrHildrethHash_getScale_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// setKernelParam(float, float) /usr/include/opencv2/img_hash/marr_hildreth_hash.hpp:37
	pub fn cv_img_hash_MarrHildrethHash_setKernelParam_float_float(instance: *mut c_void, alpha: f32, scale: f32, ocvrs_return: *mut Result_void);
	// create(float, float) /usr/include/opencv2/img_hash/marr_hildreth_hash.hpp:43
	pub fn cv_img_hash_MarrHildrethHash_create_float_float(alpha: f32, scale: f32, ocvrs_return: *mut Result<*mut c_void>);
	// create() /usr/include/opencv2/img_hash/phash.hpp:25
	pub fn cv_img_hash_PHash_create(ocvrs_return: *mut Result<*mut c_void>);
	// create(double, int) /usr/include/opencv2/img_hash/radial_variance_hash.hpp:24
	pub fn cv_img_hash_RadialVarianceHash_create_double_int(sigma: f64, num_of_angle_line: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getNumOfAngleLine() /usr/include/opencv2/img_hash/radial_variance_hash.hpp:26
	pub fn cv_img_hash_RadialVarianceHash_getNumOfAngleLine_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getSigma() /usr/include/opencv2/img_hash/radial_variance_hash.hpp:27
	pub fn cv_img_hash_RadialVarianceHash_getSigma_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// setNumOfAngleLine(int) /usr/include/opencv2/img_hash/radial_variance_hash.hpp:29
	pub fn cv_img_hash_RadialVarianceHash_setNumOfAngleLine_int(instance: *mut c_void, value: i32, ocvrs_return: *mut Result_void);
	// setSigma(double) /usr/include/opencv2/img_hash/radial_variance_hash.hpp:30
	pub fn cv_img_hash_RadialVarianceHash_setSigma_double(instance: *mut c_void, value: f64, ocvrs_return: *mut Result_void);
	// getFeatures() /usr/include/opencv2/img_hash/radial_variance_hash.hpp:33
	pub fn cv_img_hash_RadialVarianceHash_getFeatures(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getHash() /usr/include/opencv2/img_hash/radial_variance_hash.hpp:34
	pub fn cv_img_hash_RadialVarianceHash_getHash(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getPixPerLine(const cv::Mat &) /usr/include/opencv2/img_hash/radial_variance_hash.hpp:35
	pub fn cv_img_hash_RadialVarianceHash_getPixPerLine_const_MatR(instance: *mut c_void, input: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getProjection() /usr/include/opencv2/img_hash/radial_variance_hash.hpp:36
	pub fn cv_img_hash_RadialVarianceHash_getProjection(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
}
