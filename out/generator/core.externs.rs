extern "C" {
	// Cholesky(double *, size_t, int, double *, size_t, int) /usr/include/opencv2/core/base.hpp:567
	pub fn cv_Cholesky_doubleX_size_t_int_doubleX_size_t_int(a: *mut f64, astep: size_t, m: i32, b: *mut f64, bstep: size_t, n: i32, ocvrs_return: *mut Result<bool>);
	// Cholesky(float *, size_t, int, float *, size_t, int) /usr/include/opencv2/core/base.hpp:565
	pub fn cv_Cholesky_floatX_size_t_int_floatX_size_t_int(a: *mut f32, astep: size_t, m: i32, b: *mut f32, bstep: size_t, n: i32, ocvrs_return: *mut Result<bool>);
	// LUT(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:563
	pub fn cv_LUT_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src: *const c_void, lut: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// LU(double *, size_t, int, double *, size_t, int) /usr/include/opencv2/core/base.hpp:563
	pub fn cv_LU_doubleX_size_t_int_doubleX_size_t_int(a: *mut f64, astep: size_t, m: i32, b: *mut f64, bstep: size_t, n: i32, ocvrs_return: *mut Result<i32>);
	// LU(float *, size_t, int, float *, size_t, int) /usr/include/opencv2/core/base.hpp:561
	pub fn cv_LU_floatX_size_t_int_floatX_size_t_int(a: *mut f32, astep: size_t, m: i32, b: *mut f32, bstep: size_t, n: i32, ocvrs_return: *mut Result<i32>);
	// Mahalanobis(cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/core.hpp:2083
	pub fn cv_Mahalanobis_const__InputArrayR_const__InputArrayR_const__InputArrayR(v1: *const c_void, v2: *const c_void, icovar: *const c_void, ocvrs_return: *mut Result<f64>);
	// PCABackProject(cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2063
	pub fn cv_PCABackProject_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(data: *const c_void, mean: *const c_void, eigenvectors: *const c_void, result: *const c_void, ocvrs_return: *mut Result_void);
	// PCACompute(cv::InputArray, cv::InputOutputArray, cv::OutputArray, cv::OutputArray, double) /usr/include/opencv2/core.hpp:2054
	pub fn cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_double(data: *const c_void, mean: *const c_void, eigenvectors: *const c_void, eigenvalues: *const c_void, retained_variance: f64, ocvrs_return: *mut Result_void);
	// PCACompute(cv::InputArray, cv::InputOutputArray, cv::OutputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2045
	pub fn cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int(data: *const c_void, mean: *const c_void, eigenvectors: *const c_void, eigenvalues: *const c_void, max_components: i32, ocvrs_return: *mut Result_void);
	// PCACompute(cv::InputArray, cv::InputOutputArray, cv::OutputArray, double) /usr/include/opencv2/core.hpp:2050
	pub fn cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_double(data: *const c_void, mean: *const c_void, eigenvectors: *const c_void, retained_variance: f64, ocvrs_return: *mut Result_void);
	// PCACompute(cv::InputArray, cv::InputOutputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2041
	pub fn cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_int(data: *const c_void, mean: *const c_void, eigenvectors: *const c_void, max_components: i32, ocvrs_return: *mut Result_void);
	// PCAProject(cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2059
	pub fn cv_PCAProject_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(data: *const c_void, mean: *const c_void, eigenvectors: *const c_void, result: *const c_void, ocvrs_return: *mut Result_void);
	// PSNR(cv::InputArray, cv::InputArray, double) /usr/include/opencv2/core.hpp:723
	pub fn cv_PSNR_const__InputArrayR_const__InputArrayR_double(src1: *const c_void, src2: *const c_void, r: f64, ocvrs_return: *mut Result<f64>);
	// SVBackSubst(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2070
	pub fn cv_SVBackSubst_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(w: *const c_void, u: *const c_void, vt: *const c_void, rhs: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// SVDecomp(cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2067
	pub fn cv_SVDecomp_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(src: *const c_void, w: *const c_void, u: *const c_void, vt: *const c_void, flags: i32, ocvrs_return: *mut Result_void);
	// abs(const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3768
	pub fn cv_abs_const_MatExprR(e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// abs(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3764
	pub fn cv_abs_const_MatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// absdiff(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1397
	pub fn cv_absdiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// addWeighted(cv::InputArray, double, cv::InputArray, double, double, cv::OutputArray, int) /usr/include/opencv2/core.hpp:506
	pub fn cv_addWeighted_const__InputArrayR_double_const__InputArrayR_double_double_const__OutputArrayR_int(src1: *const c_void, alpha: f64, src2: *const c_void, beta: f64, gamma: f64, dst: *const c_void, dtype: i32, ocvrs_return: *mut Result_void);
	// add(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray, int) /usr/include/opencv2/core.hpp:360
	pub fn cv_add_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int(src1: *const c_void, src2: *const c_void, dst: *const c_void, mask: *const c_void, dtype: i32, ocvrs_return: *mut Result_void);
	// batchDistance(cv::InputArray, cv::InputArray, cv::OutputArray, int, cv::OutputArray, int, int, cv::InputArray, int, bool) /usr/include/opencv2/core.hpp:730
	pub fn cv_batchDistance_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_const__OutputArrayR_int_int_const__InputArrayR_int_bool(src1: *const c_void, src2: *const c_void, dist: *const c_void, dtype: i32, nidx: *const c_void, norm_type: i32, k: i32, mask: *const c_void, update: i32, crosscheck: bool, ocvrs_return: *mut Result_void);
	// bitwise_and(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1299
	pub fn cv_bitwise_and_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// bitwise_not(cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1371
	pub fn cv_bitwise_not_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// bitwise_or(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1326
	pub fn cv_bitwise_or_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// bitwise_xor(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1354
	pub fn cv_bitwise_xor_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// borderInterpolate(int, int, int) /usr/include/opencv2/core.hpp:270
	pub fn cv_borderInterpolate_int_int_int(p: i32, len: i32, border_type: i32, ocvrs_return: *mut Result<i32>);
	// calcCovarMatrix(cv::InputArray, cv::OutputArray, cv::InputOutputArray, int, int) /usr/include/opencv2/core.hpp:2037
	pub fn cv_calcCovarMatrix_const__InputArrayR_const__OutputArrayR_const__InputOutputArrayR_int_int(samples: *const c_void, covar: *const c_void, mean: *const c_void, flags: i32, ctype: i32, ocvrs_return: *mut Result_void);
	// cartToPolar(cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray, bool) /usr/include/opencv2/core.hpp:1608
	pub fn cv_cartToPolar_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool(x: *const c_void, y: *const c_void, magnitude: *const c_void, angle: *const c_void, angle_in_degrees: bool, ocvrs_return: *mut Result_void);
	// checkHardwareSupport(int) /usr/include/opencv2/core/utility.hpp:425
	pub fn cv_checkHardwareSupport_int(feature: i32, ocvrs_return: *mut Result<bool>);
	// checkRange(cv::InputArray, bool, cv::Point *, double, double) /usr/include/opencv2/core.hpp:1659
	pub fn cv_checkRange_const__InputArrayR_bool_PointX_double_double(a: *const c_void, quiet: bool, pos: *mut core::Point, min_val: f64, max_val: f64, ocvrs_return: *mut Result<bool>);
	// compare(cv::InputArray, cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1458
	pub fn cv_compare_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(src1: *const c_void, src2: *const c_void, dst: *const c_void, cmpop: i32, ocvrs_return: *mut Result_void);
	// completeSymm(cv::InputOutputArray, bool) /usr/include/opencv2/core.hpp:1819
	pub fn cv_completeSymm_const__InputOutputArrayR_bool(m: *const c_void, lower_to_upper: bool, ocvrs_return: *mut Result_void);
	// convertFp16(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:547
	pub fn cv_convertFp16_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// convertScaleAbs(cv::InputArray, cv::OutputArray, double, double) /usr/include/opencv2/core.hpp:534
	pub fn cv_convertScaleAbs_const__InputArrayR_const__OutputArrayR_double_double(src: *const c_void, dst: *const c_void, alpha: f64, beta: f64, ocvrs_return: *mut Result_void);
	// copyMakeBorder(cv::InputArray, cv::OutputArray, int, int, int, int, int, const cv::Scalar &) /usr/include/opencv2/core.hpp:320
	pub fn cv_copyMakeBorder_const__InputArrayR_const__OutputArrayR_int_int_int_int_int_const_ScalarR(src: *const c_void, dst: *const c_void, top: i32, bottom: i32, left: i32, right: i32, border_type: i32, value: *const core::Scalar, ocvrs_return: *mut Result_void);
	// copyTo(cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1409
	pub fn cv_copyTo_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// countNonZero(cv::InputArray) /usr/include/opencv2/core.hpp:581
	pub fn cv_countNonZero_const__InputArrayR(src: *const c_void, ocvrs_return: *mut Result<i32>);
	// cubeRoot(float) /usr/include/opencv2/core/base.hpp:539
	pub fn cv_cubeRoot_float(val: f32, ocvrs_return: *mut Result<f32>);
	// createContinuous(int, int, int, cv::OutputArray) /usr/include/opencv2/core/cuda.hpp:557
	pub fn cv_cuda_createContinuous_int_int_int_const__OutputArrayR(rows: i32, cols: i32, typ: i32, arr: *const c_void, ocvrs_return: *mut Result_void);
	// deviceSupports(cv::cuda::FeatureSet) /usr/include/opencv2/core/cuda.hpp:1010
	pub fn cv_cuda_deviceSupports_FeatureSet(feature_set: core::FeatureSet, ocvrs_return: *mut Result<bool>);
	// ensureSizeIsEnough(int, int, int, cv::OutputArray) /usr/include/opencv2/core/cuda.hpp:568
	pub fn cv_cuda_ensureSizeIsEnough_int_int_int_const__OutputArrayR(rows: i32, cols: i32, typ: i32, arr: *const c_void, ocvrs_return: *mut Result_void);
	// getCudaEnabledDeviceCount() /usr/include/opencv2/core/cuda.hpp:966
	pub fn cv_cuda_getCudaEnabledDeviceCount(ocvrs_return: *mut Result<i32>);
	// getDevice() /usr/include/opencv2/core/cuda.hpp:978
	pub fn cv_cuda_getDevice(ocvrs_return: *mut Result<i32>);
	// printCudaDeviceInfo(int) /usr/include/opencv2/core/cuda.hpp:1252
	pub fn cv_cuda_printCudaDeviceInfo_int(device: i32, ocvrs_return: *mut Result_void);
	// printShortCudaDeviceInfo(int) /usr/include/opencv2/core/cuda.hpp:1253
	pub fn cv_cuda_printShortCudaDeviceInfo_int(device: i32, ocvrs_return: *mut Result_void);
	// registerPageLocked(cv::Mat &) /usr/include/opencv2/core/cuda.hpp:809
	pub fn cv_cuda_registerPageLocked_MatR(m: *mut c_void, ocvrs_return: *mut Result_void);
	// resetDevice() /usr/include/opencv2/core/cuda.hpp:985
	pub fn cv_cuda_resetDevice(ocvrs_return: *mut Result_void);
	// setBufferPoolConfig(int, size_t, int) /usr/include/opencv2/core/cuda.hpp:709
	pub fn cv_cuda_setBufferPoolConfig_int_size_t_int(device_id: i32, stack_size: size_t, stack_count: i32, ocvrs_return: *mut Result_void);
	// setBufferPoolUsage(bool) /usr/include/opencv2/core/cuda.hpp:708
	pub fn cv_cuda_setBufferPoolUsage_bool(on: bool, ocvrs_return: *mut Result_void);
	// setDevice(int) /usr/include/opencv2/core/cuda.hpp:974
	pub fn cv_cuda_setDevice_int(device: i32, ocvrs_return: *mut Result_void);
	// setGlDevice(int) /usr/include/opencv2/core/opengl.hpp:572
	pub fn cv_cuda_setGlDevice_int(device: i32, ocvrs_return: *mut Result_void);
	// unregisterPageLocked(cv::Mat &) /usr/include/opencv2/core/cuda.hpp:815
	pub fn cv_cuda_unregisterPageLocked_MatR(m: *mut c_void, ocvrs_return: *mut Result_void);
	// dct(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2272
	pub fn cv_dct_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, flags: i32, ocvrs_return: *mut Result_void);
	// depthToString(int) /usr/include/opencv2/core/check.hpp:13
	pub fn cv_depthToString_int(depth: i32, ocvrs_return: *mut Result<*mut c_void>);
	// check_failed_MatChannels(const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:85
	pub fn cv_detail_check_failed_MatChannels_const_int_const_CheckContextR(v: i32, ctx: *const c_void, ocvrs_return: *mut Result_void);
	// check_failed_MatChannels(const int, const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:75
	pub fn cv_detail_check_failed_MatChannels_const_int_const_int_const_CheckContextR(v1: i32, v2: i32, ctx: *const c_void, ocvrs_return: *mut Result_void);
	// check_failed_MatDepth(const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:83
	pub fn cv_detail_check_failed_MatDepth_const_int_const_CheckContextR(v: i32, ctx: *const c_void, ocvrs_return: *mut Result_void);
	// check_failed_MatDepth(const int, const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:73
	pub fn cv_detail_check_failed_MatDepth_const_int_const_int_const_CheckContextR(v1: i32, v2: i32, ctx: *const c_void, ocvrs_return: *mut Result_void);
	// check_failed_MatType(const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:84
	pub fn cv_detail_check_failed_MatType_const_int_const_CheckContextR(v: i32, ctx: *const c_void, ocvrs_return: *mut Result_void);
	// check_failed_MatType(const int, const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:74
	pub fn cv_detail_check_failed_MatType_const_int_const_int_const_CheckContextR(v1: i32, v2: i32, ctx: *const c_void, ocvrs_return: *mut Result_void);
	// check_failed_auto(const Size_<int>, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:81
	pub fn cv_detail_check_failed_auto_const_Size__int__const_CheckContextR(v: *const core::Size_<i32>, ctx: *const c_void, ocvrs_return: *mut Result_void);
	// check_failed_auto(const Size_<int>, const Size_<int>, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:72
	pub fn cv_detail_check_failed_auto_const_Size__int__const_Size__int__const_CheckContextR(v1: *const core::Size_<i32>, v2: *const core::Size_<i32>, ctx: *const c_void, ocvrs_return: *mut Result_void);
	// check_failed_auto(const double, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:80
	pub fn cv_detail_check_failed_auto_const_double_const_CheckContextR(v: f64, ctx: *const c_void, ocvrs_return: *mut Result_void);
	// check_failed_auto(const double, const double, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:71
	pub fn cv_detail_check_failed_auto_const_double_const_double_const_CheckContextR(v1: f64, v2: f64, ctx: *const c_void, ocvrs_return: *mut Result_void);
	// check_failed_auto(const float, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:79
	pub fn cv_detail_check_failed_auto_const_float_const_CheckContextR(v: f32, ctx: *const c_void, ocvrs_return: *mut Result_void);
	// check_failed_auto(const float, const float, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:70
	pub fn cv_detail_check_failed_auto_const_float_const_float_const_CheckContextR(v1: f32, v2: f32, ctx: *const c_void, ocvrs_return: *mut Result_void);
	// check_failed_auto(const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:77
	pub fn cv_detail_check_failed_auto_const_int_const_CheckContextR(v: i32, ctx: *const c_void, ocvrs_return: *mut Result_void);
	// check_failed_auto(const int, const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:68
	pub fn cv_detail_check_failed_auto_const_int_const_int_const_CheckContextR(v1: i32, v2: i32, ctx: *const c_void, ocvrs_return: *mut Result_void);
	// check_failed_auto(const size_t, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:78
	pub fn cv_detail_check_failed_auto_const_size_t_const_CheckContextR(v: size_t, ctx: *const c_void, ocvrs_return: *mut Result_void);
	// check_failed_auto(const size_t, const size_t, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:69
	pub fn cv_detail_check_failed_auto_const_size_t_const_size_t_const_CheckContextR(v1: size_t, v2: size_t, ctx: *const c_void, ocvrs_return: *mut Result_void);
	// check_failed_auto(const std::string &, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:82
	pub fn cv_detail_check_failed_auto_const_stringR_const_CheckContextR(v1: *const c_char, ctx: *const c_void, ocvrs_return: *mut Result_void);
	// determinant(cv::InputArray) /usr/include/opencv2/core.hpp:1851
	pub fn cv_determinant_const__InputArrayR(mtx: *const c_void, ocvrs_return: *mut Result<f64>);
	// dft(cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/core.hpp:2217
	pub fn cv_dft_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, flags: i32, nonzero_rows: i32, ocvrs_return: *mut Result_void);
	// getTypeFromD3DFORMAT(const int) /usr/include/opencv2/core/directx.hpp:178
	pub fn cv_directx_getTypeFromD3DFORMAT_const_int(i_d3_dformat: i32, ocvrs_return: *mut Result<i32>);
	// getTypeFromDXGI_FORMAT(const int) /usr/include/opencv2/core/directx.hpp:173
	pub fn cv_directx_getTypeFromDXGI_FORMAT_const_int(i_dxgi_format: i32, ocvrs_return: *mut Result<i32>);
	// divide(cv::InputArray, cv::InputArray, cv::OutputArray, double, int) /usr/include/opencv2/core.hpp:453
	pub fn cv_divide_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int(src1: *const c_void, src2: *const c_void, dst: *const c_void, scale: f64, dtype: i32, ocvrs_return: *mut Result_void);
	// divide(double, cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:457
	pub fn cv_divide_double_const__InputArrayR_const__OutputArrayR_int(scale: f64, src2: *const c_void, dst: *const c_void, dtype: i32, ocvrs_return: *mut Result_void);
	// eigenNonSymmetric(cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2010
	pub fn cv_eigenNonSymmetric_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src: *const c_void, eigenvalues: *const c_void, eigenvectors: *const c_void, ocvrs_return: *mut Result_void);
	// eigen(cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1993
	pub fn cv_eigen_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(src: *const c_void, eigenvalues: *const c_void, eigenvectors: *const c_void, ocvrs_return: *mut Result<bool>);
	// error(const cv::Exception &) /usr/include/opencv2/core.hpp:155
	pub fn cv_error_const_ExceptionR(exc: *const c_void, ocvrs_return: *mut Result_void);
	// error(int, const cv::String &, const char *, const char *, int) /usr/include/opencv2/core/base.hpp:298
	pub fn cv_error_int_const_StringR_const_charX_const_charX_int(_code: i32, _err: *const c_char, _func: *const c_char, _file: *const c_char, _line: i32, ocvrs_return: *mut Result_void);
	// exp(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1552
	pub fn cv_exp_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// extractChannel(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1061
	pub fn cv_extractChannel_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, coi: i32, ocvrs_return: *mut Result_void);
	// fastAtan2(float, float) /usr/include/opencv2/core/base.hpp:558
	pub fn cv_fastAtan2_float_float(y: f32, x: f32, ocvrs_return: *mut Result<f32>);
	// findNonZero(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:609
	pub fn cv_findNonZero_const__InputArrayR_const__OutputArrayR(src: *const c_void, idx: *const c_void, ocvrs_return: *mut Result_void);
	// flip(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1103
	pub fn cv_flip_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, flip_code: i32, ocvrs_return: *mut Result_void);
	// gemm(cv::InputArray, cv::InputArray, double, cv::InputArray, double, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1696
	pub fn cv_gemm_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_double_const__OutputArrayR_int(src1: *const c_void, src2: *const c_void, alpha: f64, src3: *const c_void, beta: f64, dst: *const c_void, flags: i32, ocvrs_return: *mut Result_void);
	// getBuildInformation() /usr/include/opencv2/core/utility.hpp:242
	pub fn cv_getBuildInformation(ocvrs_return: *mut Result<*mut c_void>);
	// getCPUFeaturesLine() /usr/include/opencv2/core/utility.hpp:443
	pub fn cv_getCPUFeaturesLine(ocvrs_return: *mut Result<*mut c_void>);
	// getCPUTickCount() /usr/include/opencv2/core/utility.hpp:415
	pub fn cv_getCPUTickCount(ocvrs_return: *mut Result<i64>);
	// getElemSize(int) /usr/include/opencv2/core/utility.hpp:568
	pub fn cv_getElemSize_int(typ: i32, ocvrs_return: *mut Result<size_t>);
	// getHardwareFeatureName(int) /usr/include/opencv2/core/utility.hpp:431
	pub fn cv_getHardwareFeatureName_int(feature: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getLogLevel() /usr/include/opencv2/core/bindings_utils.hpp:292
	pub fn cv_getLogLevel(ocvrs_return: *mut Result<i32>);
	// getNumThreads() /usr/include/opencv2/core/utility.hpp:218
	pub fn cv_getNumThreads(ocvrs_return: *mut Result<i32>);
	// getNumberOfCPUs() /usr/include/opencv2/core/utility.hpp:447
	pub fn cv_getNumberOfCPUs(ocvrs_return: *mut Result<i32>);
	// getOptimalDFTSize(int) /usr/include/opencv2/core.hpp:2325
	pub fn cv_getOptimalDFTSize_int(vecsize: i32, ocvrs_return: *mut Result<i32>);
	// getThreadNum() /usr/include/opencv2/core/utility.hpp:234
	pub fn cv_getThreadNum(ocvrs_return: *mut Result<i32>);
	// getTickCount() /usr/include/opencv2/core/utility.hpp:268
	pub fn cv_getTickCount(ocvrs_return: *mut Result<i64>);
	// getTickFrequency() /usr/include/opencv2/core/utility.hpp:281
	pub fn cv_getTickFrequency(ocvrs_return: *mut Result<f64>);
	// getVersionMajor() /usr/include/opencv2/core/utility.hpp:253
	pub fn cv_getVersionMajor() -> i32;
	// getVersionMinor() /usr/include/opencv2/core/utility.hpp:256
	pub fn cv_getVersionMinor() -> i32;
	// getVersionRevision() /usr/include/opencv2/core/utility.hpp:259
	pub fn cv_getVersionRevision() -> i32;
	// getVersionString() /usr/include/opencv2/core/utility.hpp:250
	pub fn cv_getVersionString(ocvrs_return: *mut Result<*mut c_void>);
	// glob(cv::String, std::vector<String> &, bool) /usr/include/opencv2/core/utility.hpp:180
	pub fn cv_glob_String_vector_String_R_bool(pattern: *mut c_char, result: *mut c_void, recursive: bool, ocvrs_return: *mut Result_void);
	// haveOpenVX() /usr/include/opencv2/core/ovx.hpp:19
	pub fn cv_haveOpenVX(ocvrs_return: *mut Result<bool>);
	// hconcat(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1189
	pub fn cv_hconcat_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// hconcat(cv::InputArrayOfArrays, cv::OutputArray) /usr/include/opencv2/core.hpp:1208
	pub fn cv_hconcat_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// idct(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2282
	pub fn cv_idct_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, flags: i32, ocvrs_return: *mut Result_void);
	// idft(cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/core.hpp:2231
	pub fn cv_idft_const__InputArrayR_const__OutputArrayR_int_int(src: *const c_void, dst: *const c_void, flags: i32, nonzero_rows: i32, ocvrs_return: *mut Result_void);
	// inRange(cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1429
	pub fn cv_inRange_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src: *const c_void, lowerb: *const c_void, upperb: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// insertChannel(cv::InputArray, cv::InputOutputArray, int) /usr/include/opencv2/core.hpp:1069
	pub fn cv_insertChannel_const__InputArrayR_const__InputOutputArrayR_int(src: *const c_void, dst: *const c_void, coi: i32, ocvrs_return: *mut Result_void);
	// getFlags() /usr/include/opencv2/core/utils/instrumentation.hpp:117
	pub fn cv_instr_getFlags(ocvrs_return: *mut Result<core::FLAGS>);
	// resetTrace() /usr/include/opencv2/core/utils/instrumentation.hpp:106
	pub fn cv_instr_resetTrace(ocvrs_return: *mut Result_void);
	// setFlags(cv::instr::FLAGS) /usr/include/opencv2/core/utils/instrumentation.hpp:115
	pub fn cv_instr_setFlags_FLAGS(mode_flags: core::FLAGS, ocvrs_return: *mut Result_void);
	// setUseInstrumentation(bool) /usr/include/opencv2/core/utils/instrumentation.hpp:105
	pub fn cv_instr_setUseInstrumentation_bool(flag: bool, ocvrs_return: *mut Result_void);
	// useInstrumentation() /usr/include/opencv2/core/utils/instrumentation.hpp:104
	pub fn cv_instr_useInstrumentation(ocvrs_return: *mut Result<bool>);
	// invert(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1887
	pub fn cv_invert_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, flags: i32, ocvrs_return: *mut Result<f64>);
	// getIppErrorLocation() /usr/include/opencv2/core/base.hpp:635
	pub fn cv_ipp_getIppErrorLocation(ocvrs_return: *mut Result<*mut c_void>);
	// getIppFeatures() /usr/include/opencv2/core/base.hpp:631
	pub fn cv_ipp_getIppFeatures(ocvrs_return: *mut Result<u64>);
	// getIppStatus() /usr/include/opencv2/core/base.hpp:634
	pub fn cv_ipp_getIppStatus(ocvrs_return: *mut Result<i32>);
	// getIppVersion() /usr/include/opencv2/core/base.hpp:638
	pub fn cv_ipp_getIppVersion(ocvrs_return: *mut Result<*mut c_void>);
	// setIppStatus(int, const char *const, const char *const, int) /usr/include/opencv2/core/base.hpp:632
	pub fn cv_ipp_setIppStatus_int_const_charX_const_charX_int(status: i32, funcname: *const c_char, filename: *const c_char, line: i32, ocvrs_return: *mut Result_void);
	// setUseIPP_NotExact(bool) /usr/include/opencv2/core/base.hpp:643
	pub fn cv_ipp_setUseIPP_NotExact_bool(flag: bool, ocvrs_return: *mut Result_void);
	// setUseIPP(bool) /usr/include/opencv2/core/base.hpp:637
	pub fn cv_ipp_setUseIPP_bool(flag: bool, ocvrs_return: *mut Result_void);
	// useIPP() /usr/include/opencv2/core/base.hpp:636
	pub fn cv_ipp_useIPP(ocvrs_return: *mut Result<bool>);
	// useIPP_NotExact() /usr/include/opencv2/core/base.hpp:642
	pub fn cv_ipp_useIPP_NotExact(ocvrs_return: *mut Result<bool>);
	// kmeans(cv::InputArray, int, cv::InputOutputArray, cv::TermCriteria, int, int, cv::OutputArray) /usr/include/opencv2/core.hpp:3054
	pub fn cv_kmeans_const__InputArrayR_int_const__InputOutputArrayR_TermCriteria_int_int_const__OutputArrayR(data: *const c_void, k: i32, best_labels: *const c_void, criteria: *const core::TermCriteria, attempts: i32, flags: i32, centers: *const c_void, ocvrs_return: *mut Result<f64>);
	// log(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1565
	pub fn cv_log_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// magnitude(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1642
	pub fn cv_magnitude_const__InputArrayR_const__InputArrayR_const__OutputArrayR(x: *const c_void, y: *const c_void, magnitude: *const c_void, ocvrs_return: *mut Result_void);
	// max(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3743
	pub fn cv_max_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// max(const cv::Mat &, const cv::Mat &, cv::Mat &) /usr/include/opencv2/core.hpp:1496
	pub fn cv_max_const_MatR_const_MatR_MatR(src1: *const c_void, src2: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result_void);
	// max(const cv::Mat &, double) /usr/include/opencv2/core/mat.hpp:3744
	pub fn cv_max_const_MatR_double(a: *const c_void, s: f64, ocvrs_return: *mut Result<*mut c_void>);
	// max(const cv::UMat &, const cv::UMat &, cv::UMat &) /usr/include/opencv2/core.hpp:1500
	pub fn cv_max_const_UMatR_const_UMatR_UMatR(src1: *const c_void, src2: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result_void);
	// max(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1492
	pub fn cv_max_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// max(double, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3745
	pub fn cv_max_double_const_MatR(s: f64, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// meanStdDev(cv::InputArray, cv::OutputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:644
	pub fn cv_meanStdDev_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR(src: *const c_void, mean: *const c_void, stddev: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// mean(cv::InputArray, cv::InputArray) /usr/include/opencv2/core.hpp:622
	pub fn cv_mean_const__InputArrayR_const__InputArrayR(src: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
	// merge(cv::InputArrayOfArrays, cv::OutputArray) /usr/include/opencv2/core.hpp:953
	pub fn cv_merge_const__InputArrayR_const__OutputArrayR(mv: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// minMaxIdx(cv::InputArray, double *, double *, int *, int *, cv::InputArray) /usr/include/opencv2/core.hpp:885
	pub fn cv_minMaxIdx_const__InputArrayR_doubleX_doubleX_intX_intX_const__InputArrayR(src: *const c_void, min_val: *mut f64, max_val: *mut f64, min_idx: *mut i32, max_idx: *mut i32, mask: *const c_void, ocvrs_return: *mut Result_void);
	// minMaxLoc(const cv::SparseMat &, double *, double *, int *, int *) /usr/include/opencv2/core.hpp:897
	pub fn cv_minMaxLoc_const_SparseMatR_doubleX_doubleX_intX_intX(a: *const c_void, min_val: *mut f64, max_val: *mut f64, min_idx: *mut i32, max_idx: *mut i32, ocvrs_return: *mut Result_void);
	// minMaxLoc(cv::InputArray, double *, double *, cv::Point *, cv::Point *, cv::InputArray) /usr/include/opencv2/core.hpp:824
	pub fn cv_minMaxLoc_const__InputArrayR_doubleX_doubleX_PointX_PointX_const__InputArrayR(src: *const c_void, min_val: *mut f64, max_val: *mut f64, min_loc: *mut core::Point, max_loc: *mut core::Point, mask: *const c_void, ocvrs_return: *mut Result_void);
	// min(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3735
	pub fn cv_min_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// min(const cv::Mat &, const cv::Mat &, cv::Mat &) /usr/include/opencv2/core.hpp:1475
	pub fn cv_min_const_MatR_const_MatR_MatR(src1: *const c_void, src2: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result_void);
	// min(const cv::Mat &, double) /usr/include/opencv2/core/mat.hpp:3736
	pub fn cv_min_const_MatR_double(a: *const c_void, s: f64, ocvrs_return: *mut Result<*mut c_void>);
	// min(const cv::UMat &, const cv::UMat &, cv::UMat &) /usr/include/opencv2/core.hpp:1479
	pub fn cv_min_const_UMatR_const_UMatR_UMatR(src1: *const c_void, src2: *const c_void, dst: *mut c_void, ocvrs_return: *mut Result_void);
	// min(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1471
	pub fn cv_min_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// min(double, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3737
	pub fn cv_min_double_const_MatR(s: f64, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// mixChannels(cv::InputArrayOfArrays, cv::InputOutputArrayOfArrays, const int *, size_t) /usr/include/opencv2/core.hpp:1036
	pub fn cv_mixChannels_const__InputArrayR_const__InputOutputArrayR_const_intX_size_t(src: *const c_void, dst: *const c_void, from_to: *const i32, npairs: size_t, ocvrs_return: *mut Result_void);
	// mixChannels(cv::InputArrayOfArrays, cv::InputOutputArrayOfArrays, const std::vector<int> &) /usr/include/opencv2/core.hpp:1052
	pub fn cv_mixChannels_const__InputArrayR_const__InputOutputArrayR_const_vector_int_R(src: *const c_void, dst: *const c_void, from_to: *const c_void, ocvrs_return: *mut Result_void);
	// mulSpectrums(cv::InputArray, cv::InputArray, cv::OutputArray, int, bool) /usr/include/opencv2/core.hpp:2301
	pub fn cv_mulSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_bool(a: *const c_void, b: *const c_void, c: *const c_void, flags: i32, conj_b: bool, ocvrs_return: *mut Result_void);
	// mulTransposed(cv::InputArray, cv::OutputArray, bool, cv::InputArray, double, int) /usr/include/opencv2/core.hpp:1727
	pub fn cv_mulTransposed_const__InputArrayR_const__OutputArrayR_bool_const__InputArrayR_double_int(src: *const c_void, dst: *const c_void, a_ta: bool, delta: *const c_void, scale: f64, dtype: i32, ocvrs_return: *mut Result_void);
	// multiply(cv::InputArray, cv::InputArray, cv::OutputArray, double, int) /usr/include/opencv2/core.hpp:425
	pub fn cv_multiply_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int(src1: *const c_void, src2: *const c_void, dst: *const c_void, scale: f64, dtype: i32, ocvrs_return: *mut Result_void);
	// noArray() /usr/include/opencv2/core/mat.hpp:448
	pub fn cv_noArray() -> *mut c_void;
	// norm(const cv::SparseMat &, int) /usr/include/opencv2/core.hpp:702
	pub fn cv_norm_const_SparseMatR_int(src: *const c_void, norm_type: i32, ocvrs_return: *mut Result<f64>);
	// norm(cv::InputArray, cv::InputArray, int, cv::InputArray) /usr/include/opencv2/core.hpp:696
	pub fn cv_norm_const__InputArrayR_const__InputArrayR_int_const__InputArrayR(src1: *const c_void, src2: *const c_void, norm_type: i32, mask: *const c_void, ocvrs_return: *mut Result<f64>);
	// norm(cv::InputArray, int, cv::InputArray) /usr/include/opencv2/core.hpp:683
	pub fn cv_norm_const__InputArrayR_int_const__InputArrayR(src1: *const c_void, norm_type: i32, mask: *const c_void, ocvrs_return: *mut Result<f64>);
	// normalize(const cv::SparseMat &, cv::SparseMat &, double, int) /usr/include/opencv2/core.hpp:804
	pub fn cv_normalize_const_SparseMatR_SparseMatR_double_int(src: *const c_void, dst: *mut c_void, alpha: f64, norm_type: i32, ocvrs_return: *mut Result_void);
	// normalize(cv::InputArray, cv::InputOutputArray, double, double, int, int, cv::InputArray) /usr/include/opencv2/core.hpp:794
	pub fn cv_normalize_const__InputArrayR_const__InputOutputArrayR_double_double_int_int_const__InputArrayR(src: *const c_void, dst: *const c_void, alpha: f64, beta: f64, norm_type: i32, dtype: i32, mask: *const c_void, ocvrs_return: *mut Result_void);
	// attachContext(const cv::String &, void *, void *, void *) /usr/include/opencv2/core/ocl.hpp:361
	pub fn cv_ocl_attachContext_const_StringR_voidX_voidX_voidX(platform_name: *const c_char, platform_id: *mut c_void, context: *mut c_void, device_id: *mut c_void, ocvrs_return: *mut Result_void);
	// buildOptionsAddMatrixDescription(cv::String &, const cv::String &, cv::InputArray) /usr/include/opencv2/core/ocl.hpp:737
	pub fn cv_ocl_buildOptionsAddMatrixDescription_StringR_const_StringR_const__InputArrayR(build_options: *mut *mut c_void, name: *const c_char, _m: *const c_void, ocvrs_return: *mut Result_void);
	// checkOptimalVectorWidth(const int *, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::ocl::OclVectorStrategy) /usr/include/opencv2/core/ocl.hpp:726
	pub fn cv_ocl_checkOptimalVectorWidth_const_intX_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_OclVectorStrategy(vector_widths: *const i32, src1: *const c_void, src2: *const c_void, src3: *const c_void, src4: *const c_void, src5: *const c_void, src6: *const c_void, src7: *const c_void, src8: *const c_void, src9: *const c_void, strat: core::OclVectorStrategy, ocvrs_return: *mut Result<i32>);
	// convertFromBuffer(void *, size_t, int, int, int, cv::UMat &) /usr/include/opencv2/core/ocl.hpp:375
	pub fn cv_ocl_convertFromBuffer_voidX_size_t_int_int_int_UMatR(cl_mem_buffer: *mut c_void, step: size_t, rows: i32, cols: i32, typ: i32, dst: *mut c_void, ocvrs_return: *mut Result_void);
	// convertFromImage(void *, cv::UMat &) /usr/include/opencv2/core/ocl.hpp:384
	pub fn cv_ocl_convertFromImage_voidX_UMatR(cl_mem_image: *mut c_void, dst: *mut c_void, ocvrs_return: *mut Result_void);
	// convertTypeStr(int, int, int, char *) /usr/include/opencv2/core/ocl.hpp:700
	pub fn cv_ocl_convertTypeStr_int_int_int_charX(sdepth: i32, ddepth: i32, cn: i32, buf: *mut *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// finish() /usr/include/opencv2/core/ocl.hpp:59
	pub fn cv_ocl_finish(ocvrs_return: *mut Result_void);
	// getOpenCLErrorString(int) /usr/include/opencv2/core/ocl.hpp:704
	pub fn cv_ocl_getOpenCLErrorString_int(error_code: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getPlatfomsInfo(std::vector<PlatformInfo> &) /usr/include/opencv2/core/ocl.hpp:706
	pub fn cv_ocl_getPlatfomsInfo_vector_PlatformInfo_R(platform_info: *mut c_void, ocvrs_return: *mut Result_void);
	// haveAmdBlas() /usr/include/opencv2/core/ocl.hpp:56
	pub fn cv_ocl_haveAmdBlas(ocvrs_return: *mut Result<bool>);
	// haveAmdFft() /usr/include/opencv2/core/ocl.hpp:57
	pub fn cv_ocl_haveAmdFft(ocvrs_return: *mut Result<bool>);
	// haveOpenCL() /usr/include/opencv2/core/ocl.hpp:54
	pub fn cv_ocl_haveOpenCL(ocvrs_return: *mut Result<bool>);
	// haveSVM() /usr/include/opencv2/core/ocl.hpp:61
	pub fn cv_ocl_haveSVM(ocvrs_return: *mut Result<bool>);
	// kernelToStr(cv::InputArray, int, const char *) /usr/include/opencv2/core/ocl.hpp:705
	pub fn cv_ocl_kernelToStr_const__InputArrayR_int_const_charX(_kernel: *const c_void, ddepth: i32, name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// memopTypeToStr(int) /usr/include/opencv2/core/ocl.hpp:702
	pub fn cv_ocl_memopTypeToStr_int(t: i32, ocvrs_return: *mut Result<*mut c_void>);
	// predictOptimalVectorWidthMax(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/core/ocl.hpp:733
	pub fn cv_ocl_predictOptimalVectorWidthMax_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(src1: *const c_void, src2: *const c_void, src3: *const c_void, src4: *const c_void, src5: *const c_void, src6: *const c_void, src7: *const c_void, src8: *const c_void, src9: *const c_void, ocvrs_return: *mut Result<i32>);
	// predictOptimalVectorWidth(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::ocl::OclVectorStrategy) /usr/include/opencv2/core/ocl.hpp:721
	pub fn cv_ocl_predictOptimalVectorWidth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_OclVectorStrategy(src1: *const c_void, src2: *const c_void, src3: *const c_void, src4: *const c_void, src5: *const c_void, src6: *const c_void, src7: *const c_void, src8: *const c_void, src9: *const c_void, strat: core::OclVectorStrategy, ocvrs_return: *mut Result<i32>);
	// setUseOpenCL(bool) /usr/include/opencv2/core/ocl.hpp:58
	pub fn cv_ocl_setUseOpenCL_bool(flag: bool, ocvrs_return: *mut Result_void);
	// typeToStr(int) /usr/include/opencv2/core/ocl.hpp:701
	pub fn cv_ocl_typeToStr_int(t: i32, ocvrs_return: *mut Result<*mut c_void>);
	// useOpenCL() /usr/include/opencv2/core/ocl.hpp:55
	pub fn cv_ocl_useOpenCL(ocvrs_return: *mut Result<bool>);
	// vecopTypeToStr(int) /usr/include/opencv2/core/ocl.hpp:703
	pub fn cv_ocl_vecopTypeToStr_int(t: i32, ocvrs_return: *mut Result<*mut c_void>);
	// convertFromGLTexture2D(const cv::ogl::Texture2D &, cv::OutputArray) /usr/include/opencv2/core/opengl.hpp:538
	pub fn cv_ogl_convertFromGLTexture2D_const_Texture2DR_const__OutputArrayR(texture: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// convertToGLTexture2D(cv::InputArray, cv::ogl::Texture2D &) /usr/include/opencv2/core/opengl.hpp:532
	pub fn cv_ogl_convertToGLTexture2D_const__InputArrayR_Texture2DR(src: *const c_void, texture: *mut c_void, ocvrs_return: *mut Result_void);
	// mapGLBuffer(const cv::ogl::Buffer &, cv::AccessFlag) /usr/include/opencv2/core/opengl.hpp:551
	pub fn cv_ogl_mapGLBuffer_const_BufferR_AccessFlag(buffer: *const c_void, access_flags: core::AccessFlag, ocvrs_return: *mut Result<*mut c_void>);
	// initializeContextFromGL() /usr/include/opencv2/core/opengl.hpp:524
	pub fn cv_ogl_ocl_initializeContextFromGL(ocvrs_return: *mut Result<*mut c_void>);
	// render(const cv::ogl::Arrays &, cv::InputArray, int, cv::Scalar) /usr/include/opencv2/core/opengl.hpp:513
	pub fn cv_ogl_render_const_ArraysR_const__InputArrayR_int_Scalar(arr: *const c_void, indices: *const c_void, mode: i32, color: *const core::Scalar, ocvrs_return: *mut Result_void);
	// render(const cv::ogl::Arrays &, int, cv::Scalar) /usr/include/opencv2/core/opengl.hpp:505
	pub fn cv_ogl_render_const_ArraysR_int_Scalar(arr: *const c_void, mode: i32, color: *const core::Scalar, ocvrs_return: *mut Result_void);
	// render(const cv::ogl::Texture2D &, Rect_<double>, Rect_<double>) /usr/include/opencv2/core/opengl.hpp:496
	pub fn cv_ogl_render_const_Texture2DR_Rect__double__Rect__double_(tex: *const c_void, wnd_rect: *const core::Rect_<f64>, tex_rect: *const core::Rect_<f64>, ocvrs_return: *mut Result_void);
	// unmapGLBuffer(cv::UMat &) /usr/include/opencv2/core/opengl.hpp:559
	pub fn cv_ogl_unmapGLBuffer_UMatR(u: *mut c_void, ocvrs_return: *mut Result_void);
	// operator+(const cv::MatExpr &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3613
	pub fn cv_operatorA_const_MatExprR_const_MatExprR(e1: *const c_void, e2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator+(const cv::MatExpr &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3609
	pub fn cv_operatorA_const_MatExprR_const_MatR(e: *const c_void, m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator+(const cv::MatExpr &, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:3611
	pub fn cv_operatorA_const_MatExprR_const_ScalarR(e: *const c_void, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// operator+(const cv::Mat &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3610
	pub fn cv_operatorA_const_MatR_const_MatExprR(m: *const c_void, e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator+(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3606
	pub fn cv_operatorA_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator+(const cv::Mat &, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:3607
	pub fn cv_operatorA_const_MatR_const_ScalarR(a: *const c_void, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// operator+(const cv::Scalar &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3612
	pub fn cv_operatorA_const_ScalarR_const_MatExprR(s: *const core::Scalar, e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator+(const cv::Scalar &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3608
	pub fn cv_operatorA_const_ScalarR_const_MatR(s: *const core::Scalar, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator/(const cv::MatExpr &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3655
	pub fn cv_operatorD_const_MatExprR_const_MatExprR(e1: *const c_void, e2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator/(const cv::MatExpr &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3651
	pub fn cv_operatorD_const_MatExprR_const_MatR(e: *const c_void, m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator/(const cv::MatExpr &, double) /usr/include/opencv2/core/mat.hpp:3653
	pub fn cv_operatorD_const_MatExprR_double(e: *const c_void, s: f64, ocvrs_return: *mut Result<*mut c_void>);
	// operator/(const cv::Mat &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3652
	pub fn cv_operatorD_const_MatR_const_MatExprR(m: *const c_void, e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator/(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3648
	pub fn cv_operatorD_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator/(const cv::Mat &, double) /usr/include/opencv2/core/mat.hpp:3649
	pub fn cv_operatorD_const_MatR_double(a: *const c_void, s: f64, ocvrs_return: *mut Result<*mut c_void>);
	// operator/(double, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3654
	pub fn cv_operatorD_double_const_MatExprR(s: f64, e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator/(double, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3650
	pub fn cv_operatorD_double_const_MatR(s: f64, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator==(const cv::FileNodeIterator &, const cv::FileNodeIterator &) /usr/include/opencv2/core/persistence.hpp:1331
	pub fn cv_operatorEQ_const_FileNodeIteratorR_const_FileNodeIteratorR(it1: *const c_void, it2: *const c_void, ocvrs_return: *mut Result<bool>);
	// operator==(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3677
	pub fn cv_operatorEQ_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator==(const cv::Mat &, double) /usr/include/opencv2/core/mat.hpp:3678
	pub fn cv_operatorEQ_const_MatR_double(a: *const c_void, s: f64, ocvrs_return: *mut Result<*mut c_void>);
	// operator==(double, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3679
	pub fn cv_operatorEQ_double_const_MatR(s: f64, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator-(const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3633
	pub fn cv_operatorS_const_MatExprR(e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator-(const cv::MatExpr &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3626
	pub fn cv_operatorS_const_MatExprR_const_MatExprR(e1: *const c_void, e2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator-(const cv::MatExpr &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3622
	pub fn cv_operatorS_const_MatExprR_const_MatR(e: *const c_void, m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator-(const cv::MatExpr &, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:3624
	pub fn cv_operatorS_const_MatExprR_const_ScalarR(e: *const c_void, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// operator-(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3632
	pub fn cv_operatorS_const_MatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator-(const cv::Mat &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3623
	pub fn cv_operatorS_const_MatR_const_MatExprR(m: *const c_void, e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator-(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3619
	pub fn cv_operatorS_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator-(const cv::Mat &, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:3620
	pub fn cv_operatorS_const_MatR_const_ScalarR(a: *const c_void, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// operator-(const cv::Scalar &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3625
	pub fn cv_operatorS_const_ScalarR_const_MatExprR(s: *const core::Scalar, e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator-(const cv::Scalar &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3621
	pub fn cv_operatorS_const_ScalarR_const_MatR(s: *const core::Scalar, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator*(const cv::MatExpr &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3642
	pub fn cv_operatorX_const_MatExprR_const_MatExprR(e1: *const c_void, e2: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator*(const cv::MatExpr &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3638
	pub fn cv_operatorX_const_MatExprR_const_MatR(e: *const c_void, m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator*(const cv::MatExpr &, double) /usr/include/opencv2/core/mat.hpp:3640
	pub fn cv_operatorX_const_MatExprR_double(e: *const c_void, s: f64, ocvrs_return: *mut Result<*mut c_void>);
	// operator*(const cv::Mat &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3639
	pub fn cv_operatorX_const_MatR_const_MatExprR(m: *const c_void, e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator*(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3635
	pub fn cv_operatorX_const_MatR_const_MatR(a: *const c_void, b: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator*(const cv::Mat &, double) /usr/include/opencv2/core/mat.hpp:3636
	pub fn cv_operatorX_const_MatR_double(a: *const c_void, s: f64, ocvrs_return: *mut Result<*mut c_void>);
	// operator*(double, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3641
	pub fn cv_operatorX_double_const_MatExprR(s: f64, e: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator*(double, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3637
	pub fn cv_operatorX_double_const_MatR(s: f64, a: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// parallel_for_(const cv::Range &, const cv::ParallelLoopBody &, double) /usr/include/opencv2/core/utility.hpp:587
	pub fn cv_parallel_for__const_RangeR_const_ParallelLoopBodyR_double(range: *const c_void, body: *const c_void, nstripes: f64, ocvrs_return: *mut Result_void);
	// patchNaNs(cv::InputOutputArray, double) /usr/include/opencv2/core.hpp:1666
	pub fn cv_patchNaNs_const__InputOutputArrayR_double(a: *const c_void, val: f64, ocvrs_return: *mut Result_void);
	// perspectiveTransform(cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1803
	pub fn cv_perspectiveTransform_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, m: *const c_void, ocvrs_return: *mut Result_void);
	// phase(cv::InputArray, cv::InputArray, cv::OutputArray, bool) /usr/include/opencv2/core.hpp:1628
	pub fn cv_phase_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool(x: *const c_void, y: *const c_void, angle: *const c_void, angle_in_degrees: bool, ocvrs_return: *mut Result_void);
	// polarToCart(cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray, bool) /usr/include/opencv2/core.hpp:1587
	pub fn cv_polarToCart_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool(magnitude: *const c_void, angle: *const c_void, x: *const c_void, y: *const c_void, angle_in_degrees: bool, ocvrs_return: *mut Result_void);
	// pow(cv::InputArray, double, cv::OutputArray) /usr/include/opencv2/core.hpp:1536
	pub fn cv_pow_const__InputArrayR_double_const__OutputArrayR(src: *const c_void, power: f64, dst: *const c_void, ocvrs_return: *mut Result_void);
	// randShuffle(cv::InputOutputArray, double, cv::RNG *) /usr/include/opencv2/core.hpp:2382
	pub fn cv_randShuffle_const__InputOutputArrayR_double_RNGX(dst: *const c_void, iter_factor: f64, rng: *mut c_void, ocvrs_return: *mut Result_void);
	// randn(cv::InputOutputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/core.hpp:2369
	pub fn cv_randn_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(dst: *const c_void, mean: *const c_void, stddev: *const c_void, ocvrs_return: *mut Result_void);
	// randu(cv::InputOutputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/core.hpp:2356
	pub fn cv_randu_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(dst: *const c_void, low: *const c_void, high: *const c_void, ocvrs_return: *mut Result_void);
	// read(const cv::FileNode &, cv::DMatch &, const cv::DMatch &) /usr/include/opencv2/core/persistence.hpp:734
	pub fn cv_read_const_FileNodeR_DMatchR_const_DMatchR(node: *const c_void, value: *mut core::DMatch, default_value: *const core::DMatch, ocvrs_return: *mut Result_void);
	// read(const cv::FileNode &, cv::KeyPoint &, const cv::KeyPoint &) /usr/include/opencv2/core/persistence.hpp:733
	pub fn cv_read_const_FileNodeR_KeyPointR_const_KeyPointR(node: *const c_void, value: *mut core::KeyPoint, default_value: *const core::KeyPoint, ocvrs_return: *mut Result_void);
	// read(const cv::FileNode &, cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/persistence.hpp:727
	pub fn cv_read_const_FileNodeR_MatR_const_MatR(node: *const c_void, mat: *mut c_void, default_mat: *const c_void, ocvrs_return: *mut Result_void);
	// read(const cv::FileNode &, cv::SparseMat &, const cv::SparseMat &) /usr/include/opencv2/core/persistence.hpp:728
	pub fn cv_read_const_FileNodeR_SparseMatR_const_SparseMatR(node: *const c_void, mat: *mut c_void, default_mat: *const c_void, ocvrs_return: *mut Result_void);
	// read(const cv::FileNode &, double &, double) /usr/include/opencv2/core/persistence.hpp:725
	pub fn cv_read_const_FileNodeR_doubleR_double(node: *const c_void, value: *mut f64, default_value: f64, ocvrs_return: *mut Result_void);
	// read(const cv::FileNode &, float &, float) /usr/include/opencv2/core/persistence.hpp:724
	pub fn cv_read_const_FileNodeR_floatR_float(node: *const c_void, value: *mut f32, default_value: f32, ocvrs_return: *mut Result_void);
	// read(const cv::FileNode &, int &, int) /usr/include/opencv2/core/persistence.hpp:723
	pub fn cv_read_const_FileNodeR_intR_int(node: *const c_void, value: *mut i32, default_value: i32, ocvrs_return: *mut Result_void);
	// read(const cv::FileNode &, std::string &, const std::string &) /usr/include/opencv2/core/persistence.hpp:726
	pub fn cv_read_const_FileNodeR_stringR_const_stringR(node: *const c_void, value: *mut *mut c_void, default_value: *const c_char, ocvrs_return: *mut Result_void);
	// read(const cv::FileNode &, std::vector<DMatch> &) /usr/include/opencv2/core/persistence.hpp:731
	pub fn cv_read_const_FileNodeR_vector_DMatch_R(node: *const c_void, matches: *mut c_void, ocvrs_return: *mut Result_void);
	// read(const cv::FileNode &, std::vector<KeyPoint> &) /usr/include/opencv2/core/persistence.hpp:730
	pub fn cv_read_const_FileNodeR_vector_KeyPoint_R(node: *const c_void, keypoints: *mut c_void, ocvrs_return: *mut Result_void);
	// reduceArgMax(cv::InputArray, cv::OutputArray, int, bool) /usr/include/opencv2/core.hpp:860
	pub fn cv_reduceArgMax_const__InputArrayR_const__OutputArrayR_int_bool(src: *const c_void, dst: *const c_void, axis: i32, last_index: bool, ocvrs_return: *mut Result_void);
	// reduceArgMin(cv::InputArray, cv::OutputArray, int, bool) /usr/include/opencv2/core.hpp:843
	pub fn cv_reduceArgMin_const__InputArrayR_const__OutputArrayR_int_bool(src: *const c_void, dst: *const c_void, axis: i32, last_index: bool, ocvrs_return: *mut Result_void);
	// reduce(cv::InputArray, cv::OutputArray, int, int, int) /usr/include/opencv2/core.hpp:924
	pub fn cv_reduce_const__InputArrayR_const__OutputArrayR_int_int_int(src: *const c_void, dst: *const c_void, dim: i32, rtype: i32, dtype: i32, ocvrs_return: *mut Result_void);
	// repeat(const cv::Mat &, int, int) /usr/include/opencv2/core.hpp:1145
	pub fn cv_repeat_const_MatR_int_int(src: *const c_void, ny: i32, nx: i32, ocvrs_return: *mut Result<*mut c_void>);
	// repeat(cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/core.hpp:1136
	pub fn cv_repeat_const__InputArrayR_int_int_const__OutputArrayR(src: *const c_void, ny: i32, nx: i32, dst: *const c_void, ocvrs_return: *mut Result_void);
	// rotate(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1121
	pub fn cv_rotate_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, rotate_code: i32, ocvrs_return: *mut Result_void);
	// addSamplesDataSearchPath(const cv::String &) /usr/include/opencv2/core/utility.hpp:1200
	pub fn cv_samples_addSamplesDataSearchPath_const_StringR(path: *const c_char, ocvrs_return: *mut Result_void);
	// addSamplesDataSearchSubDirectory(const cv::String &) /usr/include/opencv2/core/utility.hpp:1209
	pub fn cv_samples_addSamplesDataSearchSubDirectory_const_StringR(subdir: *const c_char, ocvrs_return: *mut Result_void);
	// findFileOrKeep(const cv::String &, bool) /usr/include/opencv2/core/utility.hpp:1183
	pub fn cv_samples_findFileOrKeep_const_StringR_bool(relative_path: *const c_char, silent_mode: bool, ocvrs_return: *mut Result<*mut c_void>);
	// findFile(const cv::String &, bool, bool) /usr/include/opencv2/core/utility.hpp:1181
	pub fn cv_samples_findFile_const_StringR_bool_bool(relative_path: *const c_char, required: bool, silent_mode: bool, ocvrs_return: *mut Result<*mut c_void>);
	// scaleAdd(cv::InputArray, double, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:478
	pub fn cv_scaleAdd_const__InputArrayR_double_const__InputArrayR_const__OutputArrayR(src1: *const c_void, alpha: f64, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// setBreakOnError(bool) /usr/include/opencv2/core/utility.hpp:160
	pub fn cv_setBreakOnError_bool(flag: bool, ocvrs_return: *mut Result<bool>);
	// setIdentity(cv::InputOutputArray, const cv::Scalar &) /usr/include/opencv2/core.hpp:1836
	pub fn cv_setIdentity_const__InputOutputArrayR_const_ScalarR(mtx: *const c_void, s: *const core::Scalar, ocvrs_return: *mut Result_void);
	// setLogLevel(int) /usr/include/opencv2/core/bindings_utils.hpp:285
	pub fn cv_setLogLevel_int(level: i32, ocvrs_return: *mut Result<i32>);
	// setNumThreads(int) /usr/include/opencv2/core/utility.hpp:200
	pub fn cv_setNumThreads_int(nthreads: i32, ocvrs_return: *mut Result_void);
	// setRNGSeed(int) /usr/include/opencv2/core.hpp:2344
	pub fn cv_setRNGSeed_int(seed: i32, ocvrs_return: *mut Result_void);
	// setUseOpenVX(bool) /usr/include/opencv2/core/ovx.hpp:25
	pub fn cv_setUseOpenVX_bool(flag: bool, ocvrs_return: *mut Result_void);
	// setUseOptimized(bool) /usr/include/opencv2/core/utility.hpp:560
	pub fn cv_setUseOptimized_bool(onoff: bool, ocvrs_return: *mut Result_void);
	// solveCubic(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1962
	pub fn cv_solveCubic_const__InputArrayR_const__OutputArrayR(coeffs: *const c_void, roots: *const c_void, ocvrs_return: *mut Result<i32>);
	// solveLP(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core/optim.hpp:296
	pub fn cv_solveLP_const__InputArrayR_const__InputArrayR_const__OutputArrayR(func: *const c_void, constr: *const c_void, z: *const c_void, ocvrs_return: *mut Result<i32>);
	// solvePoly(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1972
	pub fn cv_solvePoly_const__InputArrayR_const__OutputArrayR_int(coeffs: *const c_void, roots: *const c_void, max_iters: i32, ocvrs_return: *mut Result<f64>);
	// solve(cv::InputArray, cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1911
	pub fn cv_solve_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(src1: *const c_void, src2: *const c_void, dst: *const c_void, flags: i32, ocvrs_return: *mut Result<bool>);
	// sortIdx(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1947
	pub fn cv_sortIdx_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, flags: i32, ocvrs_return: *mut Result_void);
	// sort(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1927
	pub fn cv_sort_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, dst: *const c_void, flags: i32, ocvrs_return: *mut Result_void);
	// split(const cv::Mat &, cv::Mat *) /usr/include/opencv2/core.hpp:970
	pub fn cv_split_const_MatR_MatX(src: *const c_void, mvbegin: *mut c_void, ocvrs_return: *mut Result_void);
	// split(cv::InputArray, cv::OutputArrayOfArrays) /usr/include/opencv2/core.hpp:976
	pub fn cv_split_const__InputArrayR_const__OutputArrayR(m: *const c_void, mv: *const c_void, ocvrs_return: *mut Result_void);
	// sqrt(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1511
	pub fn cv_sqrt_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// subtract(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray, int) /usr/include/opencv2/core.hpp:400
	pub fn cv_subtract_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int(src1: *const c_void, src2: *const c_void, dst: *const c_void, mask: *const c_void, dtype: i32, ocvrs_return: *mut Result_void);
	// sum(cv::InputArray) /usr/include/opencv2/core.hpp:572
	pub fn cv_sum_const__InputArrayR(src: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
	// swap(cv::Mat &, cv::Mat &) /usr/include/opencv2/core.hpp:240
	pub fn cv_swap_MatR_MatR(a: *mut c_void, b: *mut c_void, ocvrs_return: *mut Result_void);
	// swap(cv::UMat &, cv::UMat &) /usr/include/opencv2/core.hpp:242
	pub fn cv_swap_UMatR_UMatR(a: *mut c_void, b: *mut c_void, ocvrs_return: *mut Result_void);
	// tempfile(const char *) /usr/include/opencv2/core/utility.hpp:179
	pub fn cv_tempfile_const_charX(suffix: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// theRNG() /usr/include/opencv2/core.hpp:2336
	pub fn cv_theRNG(ocvrs_return: *mut Result<*mut c_void>);
	// trace(cv::InputArray) /usr/include/opencv2/core.hpp:1860
	pub fn cv_trace_const__InputArrayR(mtx: *const c_void, ocvrs_return: *mut Result<core::Scalar>);
	// transform(cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1776
	pub fn cv_transform_const__InputArrayR_const__OutputArrayR_const__InputArrayR(src: *const c_void, dst: *const c_void, m: *const c_void, ocvrs_return: *mut Result_void);
	// transposeND(cv::InputArray, const std::vector<int> &, cv::OutputArray) /usr/include/opencv2/core.hpp:1750
	pub fn cv_transposeND_const__InputArrayR_const_vector_int_R_const__OutputArrayR(src: *const c_void, order: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// transpose(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1740
	pub fn cv_transpose_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// typeToString(int) /usr/include/opencv2/core/check.hpp:16
	pub fn cv_typeToString_int(typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// useOpenVX() /usr/include/opencv2/core/ovx.hpp:22
	pub fn cv_useOpenVX(ocvrs_return: *mut Result<bool>);
	// useOptimized() /usr/include/opencv2/core/utility.hpp:566
	pub fn cv_useOptimized(ocvrs_return: *mut Result<bool>);
	// dumpBool(bool) /usr/include/opencv2/core/bindings_utils.hpp:27
	pub fn cv_utils_dumpBool_bool(argument: bool, ocvrs_return: *mut Result<*mut c_void>);
	// dumpCString(const char *) /usr/include/opencv2/core/bindings_utils.hpp:59
	pub fn cv_utils_dumpCString_const_charX(argument: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// dumpDouble(double) /usr/include/opencv2/core/bindings_utils.hpp:53
	pub fn cv_utils_dumpDouble_double(argument: f64, ocvrs_return: *mut Result<*mut c_void>);
	// dumpFloat(float) /usr/include/opencv2/core/bindings_utils.hpp:47
	pub fn cv_utils_dumpFloat_float(argument: f32, ocvrs_return: *mut Result<*mut c_void>);
	// dumpInputArrayOfArrays(cv::InputArrayOfArrays) /usr/include/opencv2/core/bindings_utils.hpp:20
	pub fn cv_utils_dumpInputArrayOfArrays_const__InputArrayR(argument: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// dumpInputArray(cv::InputArray) /usr/include/opencv2/core/bindings_utils.hpp:18
	pub fn cv_utils_dumpInputArray_const__InputArrayR(argument: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// dumpInputOutputArrayOfArrays(cv::InputOutputArrayOfArrays) /usr/include/opencv2/core/bindings_utils.hpp:24
	pub fn cv_utils_dumpInputOutputArrayOfArrays_const__InputOutputArrayR(argument: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// dumpInputOutputArray(cv::InputOutputArray) /usr/include/opencv2/core/bindings_utils.hpp:22
	pub fn cv_utils_dumpInputOutputArray_const__InputOutputArrayR(argument: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// dumpInt(int) /usr/include/opencv2/core/bindings_utils.hpp:33
	pub fn cv_utils_dumpInt_int(argument: i32, ocvrs_return: *mut Result<*mut c_void>);
	// dumpRange(const cv::Range &) /usr/include/opencv2/core/bindings_utils.hpp:122
	pub fn cv_utils_dumpRange_const_RangeR(argument: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// dumpRect(const cv::Rect &) /usr/include/opencv2/core/bindings_utils.hpp:85
	pub fn cv_utils_dumpRect_const_RectR(argument: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
	// dumpRotatedRect(const cv::RotatedRect &) /usr/include/opencv2/core/bindings_utils.hpp:99
	pub fn cv_utils_dumpRotatedRect_const_RotatedRectR(argument: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// dumpSizeT(size_t) /usr/include/opencv2/core/bindings_utils.hpp:39
	pub fn cv_utils_dumpSizeT_size_t(argument: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// dumpString(const cv::String &) /usr/include/opencv2/core/bindings_utils.hpp:65
	pub fn cv_utils_dumpString_const_StringR(argument: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// dumpTermCriteria(const cv::TermCriteria &) /usr/include/opencv2/core/bindings_utils.hpp:92
	pub fn cv_utils_dumpTermCriteria_const_TermCriteriaR(argument: *const core::TermCriteria, ocvrs_return: *mut Result<*mut c_void>);
	// dumpVectorOfDouble(const std::vector<double> &) /usr/include/opencv2/core/bindings_utils.hpp:148
	pub fn cv_utils_dumpVectorOfDouble_const_vector_double_R(vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// dumpVectorOfInt(const std::vector<int> &) /usr/include/opencv2/core/bindings_utils.hpp:146
	pub fn cv_utils_dumpVectorOfInt_const_vector_int_R(vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// dumpVectorOfRect(const std::vector<Rect> &) /usr/include/opencv2/core/bindings_utils.hpp:150
	pub fn cv_utils_dumpVectorOfRect_const_vector_Rect_R(vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getCacheDirectoryForDownloads() /usr/include/opencv2/core/bindings_utils.hpp:276
	pub fn cv_utils_fs_getCacheDirectoryForDownloads(ocvrs_return: *mut Result<*mut c_void>);
	// generateVectorOfInt(size_t, std::vector<int> &) /usr/include/opencv2/core/bindings_utils.hpp:166
	pub fn cv_utils_generateVectorOfInt_size_t_vector_int_R(len: size_t, vec: *mut c_void, ocvrs_return: *mut Result_void);
	// generateVectorOfMat(size_t, int, int, int, std::vector<Mat> &) /usr/include/opencv2/core/bindings_utils.hpp:179
	pub fn cv_utils_generateVectorOfMat_size_t_int_int_int_vector_Mat_R(len: size_t, rows: i32, cols: i32, dtype: i32, vec: *mut c_void, ocvrs_return: *mut Result_void);
	// generateVectorOfRect(size_t, std::vector<Rect> &) /usr/include/opencv2/core/bindings_utils.hpp:153
	pub fn cv_utils_generateVectorOfRect_size_t_vector_Rect_R(len: size_t, vec: *mut c_void, ocvrs_return: *mut Result_void);
	// getThreadID() /usr/include/opencv2/core/utility.hpp:1216
	pub fn cv_utils_getThreadID(ocvrs_return: *mut Result<i32>);
	// getLogLevel() /usr/include/opencv2/core/utils/logger.hpp:27
	pub fn cv_utils_logging_getLogLevel(ocvrs_return: *mut Result<core::LogLevel>);
	// getLogTagLevel(const char *) /usr/include/opencv2/core/utils/logger.hpp:33
	pub fn cv_utils_logging_getLogTagLevel_const_charX(tag: *const c_char, ocvrs_return: *mut Result<core::LogLevel>);
	// getGlobalLogTag() /usr/include/opencv2/core/utils/logger.hpp:38
	pub fn cv_utils_logging_internal_getGlobalLogTag(ocvrs_return: *mut Result<*mut c_void>);
	// writeLogMessageEx(cv::utils::logging::LogLevel, const char *, const char *, int, const char *, const char *) /usr/include/opencv2/core/utils/logger.hpp:44
	pub fn cv_utils_logging_internal_writeLogMessageEx_LogLevel_const_charX_const_charX_int_const_charX_const_charX(log_level: core::LogLevel, tag: *const c_char, file: *const c_char, line: i32, func: *const c_char, message: *const c_char, ocvrs_return: *mut Result_void);
	// writeLogMessage(cv::utils::logging::LogLevel, const char *) /usr/include/opencv2/core/utils/logger.hpp:41
	pub fn cv_utils_logging_internal_writeLogMessage_LogLevel_const_charX(log_level: core::LogLevel, message: *const c_char, ocvrs_return: *mut Result_void);
	// registerLogTag(cv::utils::logging::LogTag *) /usr/include/opencv2/core/utils/logger.hpp:29
	pub fn cv_utils_logging_registerLogTag_LogTagX(plogtag: *mut c_void, ocvrs_return: *mut Result_void);
	// setLogLevel(cv::utils::logging::LogLevel) /usr/include/opencv2/core/utils/logger.hpp:25
	pub fn cv_utils_logging_setLogLevel_LogLevel(log_level: core::LogLevel, ocvrs_return: *mut Result<core::LogLevel>);
	// setLogTagLevel(const char *, cv::utils::logging::LogLevel) /usr/include/opencv2/core/utils/logger.hpp:31
	pub fn cv_utils_logging_setLogTagLevel_const_charX_LogLevel(tag: *const c_char, level: core::LogLevel, ocvrs_return: *mut Result_void);
	// testEchoBooleanFunction(bool) /usr/include/opencv2/core/bindings_utils.hpp:223
	pub fn cv_utils_nested_testEchoBooleanFunction_bool(flag: bool, ocvrs_return: *mut Result<bool>);
	// testAsyncArray(cv::InputArray) /usr/include/opencv2/core/bindings_utils.hpp:200
	pub fn cv_utils_testAsyncArray_const__InputArrayR(argument: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// testAsyncException() /usr/include/opencv2/core/bindings_utils.hpp:208
	pub fn cv_utils_testAsyncException(ocvrs_return: *mut Result<*mut c_void>);
	// testOverloadResolution(const cv::Rect &) /usr/include/opencv2/core/bindings_utils.hpp:78
	pub fn cv_utils_testOverloadResolution_const_RectR(rect: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
	// testOverloadResolution(int, const cv::Point &) /usr/include/opencv2/core/bindings_utils.hpp:71
	pub fn cv_utils_testOverloadResolution_int_const_PointR(value: i32, point: *const core::Point, ocvrs_return: *mut Result<*mut c_void>);
	// testOverwriteNativeMethod(int) /usr/include/opencv2/core/bindings_utils.hpp:135
	pub fn cv_utils_testOverwriteNativeMethod_int(argument: i32, ocvrs_return: *mut Result<i32>);
	// testRaiseGeneralException() /usr/include/opencv2/core/bindings_utils.hpp:194
	pub fn cv_utils_testRaiseGeneralException(ocvrs_return: *mut Result_void);
	// testReservedKeywordConversion(int, int, int) /usr/include/opencv2/core/bindings_utils.hpp:141
	pub fn cv_utils_testReservedKeywordConversion_int_int_int(positional_argument: i32, lambda: i32, from: i32, ocvrs_return: *mut Result<*mut c_void>);
	// testRotatedRectVector(float, float, float, float, float) /usr/include/opencv2/core/bindings_utils.hpp:113
	pub fn cv_utils_testRotatedRectVector_float_float_float_float_float(x: f32, y: f32, w: f32, h: f32, angle: f32, ocvrs_return: *mut Result<*mut c_void>);
	// testRotatedRect(float, float, float, float, float) /usr/include/opencv2/core/bindings_utils.hpp:107
	pub fn cv_utils_testRotatedRect_float_float_float_float_float(x: f32, y: f32, w: f32, h: f32, angle: f32, ocvrs_return: *mut Result<*mut c_void>);
	// convertFromVASurface(VADisplay, VASurfaceID, cv::Size, cv::OutputArray) /usr/include/opencv2/core/va_intel.hpp:69
	pub fn cv_va_intel_convertFromVASurface_VADisplay_VASurfaceID_Size_const__OutputArrayR(display: *mut c_void, surface: core::va_surface_id, size: *const core::Size, dst: *const c_void, ocvrs_return: *mut Result_void);
	// convertToVASurface(VADisplay, cv::InputArray, VASurfaceID, cv::Size) /usr/include/opencv2/core/va_intel.hpp:61
	pub fn cv_va_intel_convertToVASurface_VADisplay_const__InputArrayR_VASurfaceID_Size(display: *mut c_void, src: *const c_void, surface: core::va_surface_id, size: *const core::Size, ocvrs_return: *mut Result_void);
	// initializeContextFromVA(VADisplay, bool) /usr/include/opencv2/core/va_intel.hpp:51
	pub fn cv_va_intel_ocl_initializeContextFromVA_VADisplay_bool(display: *mut c_void, try_interop: bool, ocvrs_return: *mut Result<*mut c_void>);
	// vconcat(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1254
	pub fn cv_vconcat_const__InputArrayR_const__InputArrayR_const__OutputArrayR(src1: *const c_void, src2: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// vconcat(cv::InputArrayOfArrays, cv::OutputArray) /usr/include/opencv2/core.hpp:1272
	pub fn cv_vconcat_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// writeScalar(cv::FileStorage &, const cv::String &) /usr/include/opencv2/core/persistence.hpp:716
	pub fn cv_writeScalar_FileStorageR_const_StringR(fs: *mut c_void, value: *const c_char, ocvrs_return: *mut Result_void);
	// writeScalar(cv::FileStorage &, double) /usr/include/opencv2/core/persistence.hpp:715
	pub fn cv_writeScalar_FileStorageR_double(fs: *mut c_void, value: f64, ocvrs_return: *mut Result_void);
	// writeScalar(cv::FileStorage &, float) /usr/include/opencv2/core/persistence.hpp:714
	pub fn cv_writeScalar_FileStorageR_float(fs: *mut c_void, value: f32, ocvrs_return: *mut Result_void);
	// writeScalar(cv::FileStorage &, int) /usr/include/opencv2/core/persistence.hpp:713
	pub fn cv_writeScalar_FileStorageR_int(fs: *mut c_void, value: i32, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &, const cv::String &, const cv::Mat &) /usr/include/opencv2/core/persistence.hpp:706
	pub fn cv_write_FileStorageR_const_StringR_const_MatR(fs: *mut c_void, name: *const c_char, value: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &, const cv::String &, const cv::SparseMat &) /usr/include/opencv2/core/persistence.hpp:707
	pub fn cv_write_FileStorageR_const_StringR_const_SparseMatR(fs: *mut c_void, name: *const c_char, value: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &, const cv::String &, const cv::String &) /usr/include/opencv2/core/persistence.hpp:705
	pub fn cv_write_FileStorageR_const_StringR_const_StringR(fs: *mut c_void, name: *const c_char, value: *const c_char, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &, const cv::String &, const std::vector<DMatch> &) /usr/include/opencv2/core/persistence.hpp:710
	pub fn cv_write_FileStorageR_const_StringR_const_vector_DMatch_R(fs: *mut c_void, name: *const c_char, value: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &, const cv::String &, const std::vector<KeyPoint> &) /usr/include/opencv2/core/persistence.hpp:709
	pub fn cv_write_FileStorageR_const_StringR_const_vector_KeyPoint_R(fs: *mut c_void, name: *const c_char, value: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &, const cv::String &, double) /usr/include/opencv2/core/persistence.hpp:704
	pub fn cv_write_FileStorageR_const_StringR_double(fs: *mut c_void, name: *const c_char, value: f64, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &, const cv::String &, float) /usr/include/opencv2/core/persistence.hpp:703
	pub fn cv_write_FileStorageR_const_StringR_float(fs: *mut c_void, name: *const c_char, value: f32, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &, const cv::String &, int) /usr/include/opencv2/core/persistence.hpp:702
	pub fn cv_write_FileStorageR_const_StringR_int(fs: *mut c_void, name: *const c_char, value: i32, ocvrs_return: *mut Result_void);
	// Algorithm() /usr/include/opencv2/core.hpp:3136
	pub fn cv_Algorithm_Algorithm(ocvrs_return: *mut Result<*mut c_void>);
	// clear() /usr/include/opencv2/core.hpp:3141
	pub fn cv_Algorithm_clear(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/core.hpp:3145
	pub fn cv_Algorithm_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// write(const Ptr<cv::FileStorage> &, const cv::String &) /usr/include/opencv2/core.hpp:3150
	pub fn cv_Algorithm_write_const_const_Ptr_FileStorage_R_const_StringR(instance: *const c_void, fs: *const c_void, name: *const c_char, ocvrs_return: *mut Result_void);
	// read(const cv::FileNode &) /usr/include/opencv2/core.hpp:3154
	pub fn cv_Algorithm_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result_void);
	// empty() /usr/include/opencv2/core.hpp:3158
	pub fn cv_Algorithm_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// save(const cv::String &) /usr/include/opencv2/core.hpp:3222
	pub fn cv_Algorithm_save_const_const_StringR(instance: *const c_void, filename: *const c_char, ocvrs_return: *mut Result_void);
	// getDefaultName() /usr/include/opencv2/core.hpp:3226
	pub fn cv_Algorithm_getDefaultName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// AsyncArray() /usr/include/opencv2/core/async.hpp:35
	pub fn cv_AsyncArray_AsyncArray() -> *mut c_void;
	// AsyncArray(const cv::AsyncArray &) /usr/include/opencv2/core/async.hpp:36
	pub fn cv_AsyncArray_AsyncArray_const_AsyncArrayR(o: *const c_void) -> *mut c_void;
	// release() /usr/include/opencv2/core/async.hpp:38
	pub fn cv_AsyncArray_release(instance: *mut c_void);
	// get(cv::OutputArray) /usr/include/opencv2/core/async.hpp:50
	pub fn cv_AsyncArray_get_const_const__OutputArrayR(instance: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// get(cv::OutputArray, int64) /usr/include/opencv2/core/async.hpp:60
	pub fn cv_AsyncArray_get_const_const__OutputArrayR_int64_t(instance: *const c_void, dst: *const c_void, timeout_ns: i64, ocvrs_return: *mut Result<bool>);
	// get(cv::OutputArray, double) /usr/include/opencv2/core/async.hpp:63
	pub fn cv_AsyncArray_get_const_const__OutputArrayR_double(instance: *const c_void, dst: *const c_void, timeout_ns: f64, ocvrs_return: *mut Result<bool>);
	// wait_for(int64) /usr/include/opencv2/core/async.hpp:65
	pub fn cv_AsyncArray_wait_for_const_int64_t(instance: *const c_void, timeout_ns: i64, ocvrs_return: *mut Result<bool>);
	// wait_for(double) /usr/include/opencv2/core/async.hpp:68
	pub fn cv_AsyncArray_wait_for_const_double(instance: *const c_void, timeout_ns: f64, ocvrs_return: *mut Result<bool>);
	// valid() /usr/include/opencv2/core/async.hpp:70
	pub fn cv_AsyncArray_valid_const(instance: *const c_void) -> bool;
	// AsyncArray(cv::AsyncArray &&) /usr/include/opencv2/core/async.hpp:73
	pub fn cv_AsyncArray_AsyncArray_AsyncArrayR(o: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// AsyncPromise() /usr/include/opencv2/core/detail/async_promise.hpp:26
	pub fn cv_AsyncPromise_AsyncPromise() -> *mut c_void;
	// AsyncPromise(const cv::AsyncPromise &) /usr/include/opencv2/core/detail/async_promise.hpp:27
	pub fn cv_AsyncPromise_AsyncPromise_const_AsyncPromiseR(o: *const c_void) -> *mut c_void;
	// release() /usr/include/opencv2/core/detail/async_promise.hpp:29
	pub fn cv_AsyncPromise_release(instance: *mut c_void);
	// getArrayResult() /usr/include/opencv2/core/detail/async_promise.hpp:34
	pub fn cv_AsyncPromise_getArrayResult(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setValue(cv::InputArray) /usr/include/opencv2/core/detail/async_promise.hpp:39
	pub fn cv_AsyncPromise_setValue_const__InputArrayR(instance: *mut c_void, value: *const c_void, ocvrs_return: *mut Result_void);
	// setException(const cv::Exception &) /usr/include/opencv2/core/detail/async_promise.hpp:53
	pub fn cv_AsyncPromise_setException_const_ExceptionR(instance: *mut c_void, exception: *const c_void, ocvrs_return: *mut Result_void);
	// AsyncPromise(cv::AsyncPromise &&) /usr/include/opencv2/core/detail/async_promise.hpp:56
	pub fn cv_AsyncPromise_AsyncPromise_AsyncPromiseR(o: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _getImpl() /usr/include/opencv2/core/detail/async_promise.hpp:63
	pub fn cv_AsyncPromise__getImpl_const(instance: *const c_void) -> *mut c_void;
	// CommandLineParser(int, const char *const *, const cv::String &) /usr/include/opencv2/core/utility.hpp:829
	pub fn cv_CommandLineParser_CommandLineParser_int_const_charXX_const_StringR(argc: i32, argv: *const *const c_char, keys: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// CommandLineParser(const cv::CommandLineParser &) /usr/include/opencv2/core/utility.hpp:832
	pub fn cv_CommandLineParser_CommandLineParser_const_CommandLineParserR(parser: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getPathToApplication() /usr/include/opencv2/core/utility.hpp:850
	pub fn cv_CommandLineParser_getPathToApplication_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// has(const cv::String &) /usr/include/opencv2/core/utility.hpp:927
	pub fn cv_CommandLineParser_has_const_const_StringR(instance: *const c_void, name: *const c_char, ocvrs_return: *mut Result<bool>);
	// check() /usr/include/opencv2/core/utility.hpp:934
	pub fn cv_CommandLineParser_check_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// about(const cv::String &) /usr/include/opencv2/core/utility.hpp:940
	pub fn cv_CommandLineParser_about_const_StringR(instance: *mut c_void, message: *const c_char, ocvrs_return: *mut Result_void);
	// printMessage() /usr/include/opencv2/core/utility.hpp:948
	pub fn cv_CommandLineParser_printMessage_const(instance: *const c_void, ocvrs_return: *mut Result_void);
	// printErrors() /usr/include/opencv2/core/utility.hpp:954
	pub fn cv_CommandLineParser_printErrors_const(instance: *const c_void, ocvrs_return: *mut Result_void);
	// create(const Ptr<MinProblemSolver::Function> &, cv::TermCriteria) /usr/include/opencv2/core/optim.hpp:252
	pub fn cv_ConjGradSolver_create_const_Ptr_Function_R_TermCriteria(f: *const c_void, termcrit: *const core::TermCriteria, ocvrs_return: *mut Result<*mut c_void>);
	// DMatch() /usr/include/opencv2/core/types.hpp:833
	pub fn cv_DMatch_DMatch(ocvrs_return: *mut Result<core::DMatch>);
	// DMatch(int, int, float) /usr/include/opencv2/core/types.hpp:834
	pub fn cv_DMatch_DMatch_int_int_float(_query_idx: i32, _train_idx: i32, _distance: f32, ocvrs_return: *mut Result<core::DMatch>);
	// DMatch(int, int, int, float) /usr/include/opencv2/core/types.hpp:835
	pub fn cv_DMatch_DMatch_int_int_int_float(_query_idx: i32, _train_idx: i32, _img_idx: i32, _distance: f32, ocvrs_return: *mut Result<core::DMatch>);
	// getInitStep(cv::OutputArray) /usr/include/opencv2/core/optim.hpp:164
	pub fn cv_DownhillSolver_getInitStep_const_const__OutputArrayR(instance: *const c_void, step: *const c_void, ocvrs_return: *mut Result_void);
	// setInitStep(cv::InputArray) /usr/include/opencv2/core/optim.hpp:180
	pub fn cv_DownhillSolver_setInitStep_const__InputArrayR(instance: *mut c_void, step: *const c_void, ocvrs_return: *mut Result_void);
	// create(const Ptr<MinProblemSolver::Function> &, cv::InputArray, cv::TermCriteria) /usr/include/opencv2/core/optim.hpp:198
	pub fn cv_DownhillSolver_create_const_Ptr_Function_R_const__InputArrayR_TermCriteria(f: *const c_void, init_step: *const c_void, termcrit: *const core::TermCriteria, ocvrs_return: *mut Result<*mut c_void>);
	// msg /usr/include/opencv2/core.hpp:138
	pub fn cv_Exception_getPropMsg_const(instance: *const c_void) -> *mut c_void;
	// msg /usr/include/opencv2/core.hpp:138
	pub fn cv_Exception_setPropMsg_String(instance: *mut c_void, val: *mut c_char);
	// code /usr/include/opencv2/core.hpp:140
	pub fn cv_Exception_getPropCode_const(instance: *const c_void) -> i32;
	// code /usr/include/opencv2/core.hpp:140
	pub fn cv_Exception_setPropCode_int(instance: *mut c_void, val: i32);
	// err /usr/include/opencv2/core.hpp:141
	pub fn cv_Exception_getPropErr_const(instance: *const c_void) -> *mut c_void;
	// err /usr/include/opencv2/core.hpp:141
	pub fn cv_Exception_setPropErr_String(instance: *mut c_void, val: *mut c_char);
	// func /usr/include/opencv2/core.hpp:142
	pub fn cv_Exception_getPropFunc_const(instance: *const c_void) -> *mut c_void;
	// func /usr/include/opencv2/core.hpp:142
	pub fn cv_Exception_setPropFunc_String(instance: *mut c_void, val: *mut c_char);
	// file /usr/include/opencv2/core.hpp:143
	pub fn cv_Exception_getPropFile_const(instance: *const c_void) -> *mut c_void;
	// file /usr/include/opencv2/core.hpp:143
	pub fn cv_Exception_setPropFile_String(instance: *mut c_void, val: *mut c_char);
	// line /usr/include/opencv2/core.hpp:144
	pub fn cv_Exception_getPropLine_const(instance: *const c_void) -> i32;
	// line /usr/include/opencv2/core.hpp:144
	pub fn cv_Exception_setPropLine_int(instance: *mut c_void, val: i32);
	// Exception() /usr/include/opencv2/core.hpp:124
	pub fn cv_Exception_Exception(ocvrs_return: *mut Result<*mut c_void>);
	// Exception(int, const cv::String &, const cv::String &, const cv::String &, int) /usr/include/opencv2/core.hpp:129
	pub fn cv_Exception_Exception_int_const_StringR_const_StringR_const_StringR_int(_code: i32, _err: *const c_char, _func: *const c_char, _file: *const c_char, _line: i32, ocvrs_return: *mut Result<*mut c_void>);
	// what() /usr/include/opencv2/core.hpp:135
	pub fn cv_Exception_what_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// formatMessage() /usr/include/opencv2/core.hpp:136
	pub fn cv_Exception_formatMessage(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// blockIdx /usr/include/opencv2/core/persistence.hpp:623
	pub fn cv_FileNode_getPropBlockIdx_const(instance: *const c_void) -> size_t;
	// blockIdx /usr/include/opencv2/core/persistence.hpp:623
	pub fn cv_FileNode_setPropBlockIdx_size_t(instance: *mut c_void, val: size_t);
	// ofs /usr/include/opencv2/core/persistence.hpp:624
	pub fn cv_FileNode_getPropOfs_const(instance: *const c_void) -> size_t;
	// ofs /usr/include/opencv2/core/persistence.hpp:624
	pub fn cv_FileNode_setPropOfs_size_t(instance: *mut c_void, val: size_t);
	// FileNode() /usr/include/opencv2/core/persistence.hpp:508
	pub fn cv_FileNode_FileNode(ocvrs_return: *mut Result<*mut c_void>);
	// FileNode(const cv::FileStorage *, size_t, size_t) /usr/include/opencv2/core/persistence.hpp:517
	pub fn cv_FileNode_FileNode_const_FileStorageX_size_t_size_t(fs: *const c_void, block_idx: size_t, ofs: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// FileNode(const cv::FileNode &) /usr/include/opencv2/core/persistence.hpp:522
	pub fn cv_FileNode_FileNode_const_FileNodeR(node: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator[](const cv::String &) /usr/include/opencv2/core/persistence.hpp:530
	pub fn cv_FileNode_operator___const_const_StringR(instance: *const c_void, nodename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// operator[](const char *) /usr/include/opencv2/core/persistence.hpp:535
	pub fn cv_FileNode_operator___const_const_charX(instance: *const c_void, nodename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// operator[](int) /usr/include/opencv2/core/persistence.hpp:540
	pub fn cv_FileNode_operator___const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<*mut c_void>);
	// keys() /usr/include/opencv2/core/persistence.hpp:545
	pub fn cv_FileNode_keys_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// type() /usr/include/opencv2/core/persistence.hpp:550
	pub fn cv_FileNode_type_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// empty() /usr/include/opencv2/core/persistence.hpp:553
	pub fn cv_FileNode_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isNone() /usr/include/opencv2/core/persistence.hpp:555
	pub fn cv_FileNode_isNone_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isSeq() /usr/include/opencv2/core/persistence.hpp:557
	pub fn cv_FileNode_isSeq_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isMap() /usr/include/opencv2/core/persistence.hpp:559
	pub fn cv_FileNode_isMap_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isInt() /usr/include/opencv2/core/persistence.hpp:561
	pub fn cv_FileNode_isInt_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isReal() /usr/include/opencv2/core/persistence.hpp:563
	pub fn cv_FileNode_isReal_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isString() /usr/include/opencv2/core/persistence.hpp:565
	pub fn cv_FileNode_isString_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isNamed() /usr/include/opencv2/core/persistence.hpp:567
	pub fn cv_FileNode_isNamed_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// name() /usr/include/opencv2/core/persistence.hpp:569
	pub fn cv_FileNode_name_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// size() /usr/include/opencv2/core/persistence.hpp:571
	pub fn cv_FileNode_size_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// rawSize() /usr/include/opencv2/core/persistence.hpp:573
	pub fn cv_FileNode_rawSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// operator int() /usr/include/opencv2/core/persistence.hpp:575
	pub fn cv_FileNode_operator_int_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// operator float() /usr/include/opencv2/core/persistence.hpp:577
	pub fn cv_FileNode_operator_float_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// operator double() /usr/include/opencv2/core/persistence.hpp:579
	pub fn cv_FileNode_operator_double_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// operator basic_string() /usr/include/opencv2/core/persistence.hpp:581
	pub fn cv_FileNode_operator_std_string_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// isMap(int) /usr/include/opencv2/core/persistence.hpp:583
	pub fn cv_FileNode_isMap_int(flags: i32, ocvrs_return: *mut Result<bool>);
	// isSeq(int) /usr/include/opencv2/core/persistence.hpp:584
	pub fn cv_FileNode_isSeq_int(flags: i32, ocvrs_return: *mut Result<bool>);
	// isCollection(int) /usr/include/opencv2/core/persistence.hpp:585
	pub fn cv_FileNode_isCollection_int(flags: i32, ocvrs_return: *mut Result<bool>);
	// isEmptyCollection(int) /usr/include/opencv2/core/persistence.hpp:586
	pub fn cv_FileNode_isEmptyCollection_int(flags: i32, ocvrs_return: *mut Result<bool>);
	// isFlow(int) /usr/include/opencv2/core/persistence.hpp:587
	pub fn cv_FileNode_isFlow_int(flags: i32, ocvrs_return: *mut Result<bool>);
	// ptr() /usr/include/opencv2/core/persistence.hpp:589
	pub fn cv_FileNode_ptr(instance: *mut c_void, ocvrs_return: *mut Result<*mut u8>);
	// ptr() /usr/include/opencv2/core/persistence.hpp:590
	pub fn cv_FileNode_ptr_const(instance: *const c_void, ocvrs_return: *mut Result<*const u8>);
	// begin() /usr/include/opencv2/core/persistence.hpp:593
	pub fn cv_FileNode_begin_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// end() /usr/include/opencv2/core/persistence.hpp:595
	pub fn cv_FileNode_end_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// readRaw(const cv::String &, void *, size_t) /usr/include/opencv2/core/persistence.hpp:605
	pub fn cv_FileNode_readRaw_const_const_StringR_voidX_size_t(instance: *const c_void, fmt: *const c_char, vec: *mut c_void, len: size_t, ocvrs_return: *mut Result_void);
	// setValue(int, const void *, int) /usr/include/opencv2/core/persistence.hpp:610
	pub fn cv_FileNode_setValue_int_const_voidX_int(instance: *mut c_void, typ: i32, value: *const c_void, len: i32, ocvrs_return: *mut Result_void);
	// real() /usr/include/opencv2/core/persistence.hpp:613
	pub fn cv_FileNode_real_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// string() /usr/include/opencv2/core/persistence.hpp:615
	pub fn cv_FileNode_string_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// mat() /usr/include/opencv2/core/persistence.hpp:617
	pub fn cv_FileNode_mat_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// FileNodeIterator() /usr/include/opencv2/core/persistence.hpp:641
	pub fn cv_FileNodeIterator_FileNodeIterator(ocvrs_return: *mut Result<*mut c_void>);
	// FileNodeIterator(const cv::FileNode &, bool) /usr/include/opencv2/core/persistence.hpp:651
	pub fn cv_FileNodeIterator_FileNodeIterator_const_FileNodeR_bool(node: *const c_void, seek_end: bool, ocvrs_return: *mut Result<*mut c_void>);
	// FileNodeIterator(const cv::FileNodeIterator &) /usr/include/opencv2/core/persistence.hpp:656
	pub fn cv_FileNodeIterator_FileNodeIterator_const_FileNodeIteratorR(it: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator*() /usr/include/opencv2/core/persistence.hpp:661
	pub fn cv_FileNodeIterator_operatorX_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator++() /usr/include/opencv2/core/persistence.hpp:664
	pub fn cv_FileNodeIterator_operatorAA(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// readRaw(const cv::String &, void *, size_t) /usr/include/opencv2/core/persistence.hpp:678
	pub fn cv_FileNodeIterator_readRaw_const_StringR_voidX_size_t(instance: *mut c_void, fmt: *const c_char, vec: *mut c_void, len: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// remaining() /usr/include/opencv2/core/persistence.hpp:682
	pub fn cv_FileNodeIterator_remaining_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// equalTo(const cv::FileNodeIterator &) /usr/include/opencv2/core/persistence.hpp:684
	pub fn cv_FileNodeIterator_equalTo_const_const_FileNodeIteratorR(instance: *const c_void, it: *const c_void, ocvrs_return: *mut Result<bool>);
	// state /usr/include/opencv2/core/persistence.hpp:462
	pub fn cv_FileStorage_getPropState_const(instance: *const c_void) -> i32;
	// state /usr/include/opencv2/core/persistence.hpp:462
	pub fn cv_FileStorage_setPropState_int(instance: *mut c_void, val: i32);
	// elname /usr/include/opencv2/core/persistence.hpp:463
	pub fn cv_FileStorage_getPropElname_const(instance: *const c_void) -> *mut c_void;
	// elname /usr/include/opencv2/core/persistence.hpp:463
	pub fn cv_FileStorage_setPropElname_string(instance: *mut c_void, val: *mut c_char);
	// FileStorage() /usr/include/opencv2/core/persistence.hpp:336
	pub fn cv_FileStorage_FileStorage(ocvrs_return: *mut Result<*mut c_void>);
	// FileStorage(const cv::String &, int, const cv::String &) /usr/include/opencv2/core/persistence.hpp:341
	pub fn cv_FileStorage_FileStorage_const_StringR_int_const_StringR(filename: *const c_char, flags: i32, encoding: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// open(const cv::String &, int, const cv::String &) /usr/include/opencv2/core/persistence.hpp:361
	pub fn cv_FileStorage_open_const_StringR_int_const_StringR(instance: *mut c_void, filename: *const c_char, flags: i32, encoding: *const c_char, ocvrs_return: *mut Result<bool>);
	// isOpened() /usr/include/opencv2/core/persistence.hpp:368
	pub fn cv_FileStorage_isOpened_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// release() /usr/include/opencv2/core/persistence.hpp:374
	pub fn cv_FileStorage_release(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// releaseAndGetString() /usr/include/opencv2/core/persistence.hpp:381
	pub fn cv_FileStorage_releaseAndGetString(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getFirstTopLevelNode() /usr/include/opencv2/core/persistence.hpp:386
	pub fn cv_FileStorage_getFirstTopLevelNode_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// root(int) /usr/include/opencv2/core/persistence.hpp:393
	pub fn cv_FileStorage_root_const_int(instance: *const c_void, streamidx: i32, ocvrs_return: *mut Result<*mut c_void>);
	// operator[](const cv::String &) /usr/include/opencv2/core/persistence.hpp:399
	pub fn cv_FileStorage_operator___const_const_StringR(instance: *const c_void, nodename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// operator[](const char *) /usr/include/opencv2/core/persistence.hpp:402
	pub fn cv_FileStorage_operator___const_const_charX(instance: *const c_void, nodename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// write(const cv::String &, int) /usr/include/opencv2/core/persistence.hpp:409
	pub fn cv_FileStorage_write_const_StringR_int(instance: *mut c_void, name: *const c_char, val: i32, ocvrs_return: *mut Result_void);
	// write(const cv::String &, double) /usr/include/opencv2/core/persistence.hpp:411
	pub fn cv_FileStorage_write_const_StringR_double(instance: *mut c_void, name: *const c_char, val: f64, ocvrs_return: *mut Result_void);
	// write(const cv::String &, const cv::String &) /usr/include/opencv2/core/persistence.hpp:413
	pub fn cv_FileStorage_write_const_StringR_const_StringR(instance: *mut c_void, name: *const c_char, val: *const c_char, ocvrs_return: *mut Result_void);
	// write(const cv::String &, const cv::Mat &) /usr/include/opencv2/core/persistence.hpp:415
	pub fn cv_FileStorage_write_const_StringR_const_MatR(instance: *mut c_void, name: *const c_char, val: *const c_void, ocvrs_return: *mut Result_void);
	// write(const cv::String &, const std::vector<String> &) /usr/include/opencv2/core/persistence.hpp:417
	pub fn cv_FileStorage_write_const_StringR_const_vector_String_R(instance: *mut c_void, name: *const c_char, val: *const c_void, ocvrs_return: *mut Result_void);
	// writeRaw(const cv::String &, const void *, size_t) /usr/include/opencv2/core/persistence.hpp:427
	pub fn cv_FileStorage_writeRaw_const_StringR_const_voidX_size_t(instance: *mut c_void, fmt: *const c_char, vec: *const c_void, len: size_t, ocvrs_return: *mut Result_void);
	// writeComment(const cv::String &, bool) /usr/include/opencv2/core/persistence.hpp:437
	pub fn cv_FileStorage_writeComment_const_StringR_bool(instance: *mut c_void, comment: *const c_char, append: bool, ocvrs_return: *mut Result_void);
	// startWriteStruct(const cv::String &, int, const cv::String &) /usr/include/opencv2/core/persistence.hpp:445
	pub fn cv_FileStorage_startWriteStruct_const_StringR_int_const_StringR(instance: *mut c_void, name: *const c_char, flags: i32, type_name: *const c_char, ocvrs_return: *mut Result_void);
	// endWriteStruct() /usr/include/opencv2/core/persistence.hpp:449
	pub fn cv_FileStorage_endWriteStruct(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// getDefaultObjectName(const cv::String &) /usr/include/opencv2/core/persistence.hpp:455
	pub fn cv_FileStorage_getDefaultObjectName_const_StringR(filename: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// getFormat() /usr/include/opencv2/core/persistence.hpp:460
	pub fn cv_FileStorage_getFormat_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// next() /usr/include/opencv2/core.hpp:3069
	pub fn cv_Formatted_next(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// reset() /usr/include/opencv2/core.hpp:3070
	pub fn cv_Formatted_reset(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// format(const cv::Mat &) /usr/include/opencv2/core.hpp:3089
	pub fn cv_Formatter_format_const_const_MatR(instance: *const c_void, mtx: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// set16fPrecision(int) /usr/include/opencv2/core.hpp:3091
	pub fn cv_Formatter_set16fPrecision_int(instance: *mut c_void, p: i32, ocvrs_return: *mut Result_void);
	// set32fPrecision(int) /usr/include/opencv2/core.hpp:3092
	pub fn cv_Formatter_set32fPrecision_int(instance: *mut c_void, p: i32, ocvrs_return: *mut Result_void);
	// set64fPrecision(int) /usr/include/opencv2/core.hpp:3093
	pub fn cv_Formatter_set64fPrecision_int(instance: *mut c_void, p: i32, ocvrs_return: *mut Result_void);
	// setMultiline(bool) /usr/include/opencv2/core.hpp:3094
	pub fn cv_Formatter_setMultiline_bool(instance: *mut c_void, ml: bool, ocvrs_return: *mut Result_void);
	// get(Formatter::FormatType) /usr/include/opencv2/core.hpp:3096
	pub fn cv_Formatter_get_FormatType(fmt: core::Formatter_FormatType, ocvrs_return: *mut Result<*mut c_void>);
	// KeyPoint() /usr/include/opencv2/core/types.hpp:737
	pub fn cv_KeyPoint_KeyPoint(ocvrs_return: *mut Result<core::KeyPoint>);
	// KeyPoint(cv::Point2f, float, float, float, int, int) /usr/include/opencv2/core/types.hpp:746
	pub fn cv_KeyPoint_KeyPoint_Point2f_float_float_float_int_int(pt: *const core::Point2f, size: f32, angle: f32, response: f32, octave: i32, class_id: i32, ocvrs_return: *mut Result<core::KeyPoint>);
	// KeyPoint(float, float, float, float, float, int, int) /usr/include/opencv2/core/types.hpp:756
	pub fn cv_KeyPoint_KeyPoint_float_float_float_float_float_int_int(x: f32, y: f32, size: f32, angle: f32, response: f32, octave: i32, class_id: i32, ocvrs_return: *mut Result<core::KeyPoint>);
	// hash() /usr/include/opencv2/core/types.hpp:758
	pub fn cv_KeyPoint_hash_const(instance: *const core::KeyPoint, ocvrs_return: *mut Result<size_t>);
	// convert(const std::vector<KeyPoint> &, std::vector<Point2f> &, const std::vector<int> &) /usr/include/opencv2/core/types.hpp:769
	pub fn cv_KeyPoint_convert_const_vector_KeyPoint_R_vector_Point2f_R_const_vector_int_R(keypoints: *const c_void, points2f: *mut c_void, keypoint_indexes: *const c_void, ocvrs_return: *mut Result_void);
	// convert(const std::vector<Point2f> &, std::vector<KeyPoint> &, float, float, int, int) /usr/include/opencv2/core/types.hpp:780
	pub fn cv_KeyPoint_convert_const_vector_Point2f_R_vector_KeyPoint_R_float_float_int_int(points2f: *const c_void, keypoints: *mut c_void, size: f32, response: f32, octave: i32, class_id: i32, ocvrs_return: *mut Result_void);
	// overlap(const cv::KeyPoint &, const cv::KeyPoint &) /usr/include/opencv2/core/types.hpp:791
	pub fn cv_KeyPoint_overlap_const_KeyPointR_const_KeyPointR(kp1: *const core::KeyPoint, kp2: *const core::KeyPoint, ocvrs_return: *mut Result<f32>);
	// LDA(int) /usr/include/opencv2/core.hpp:2611
	pub fn cv_LDA_LDA_int(num_components: i32, ocvrs_return: *mut Result<*mut c_void>);
	// LDA(cv::InputArrayOfArrays, cv::InputArray, int) /usr/include/opencv2/core.hpp:2618
	pub fn cv_LDA_LDA_const__InputArrayR_const__InputArrayR_int(src: *const c_void, labels: *const c_void, num_components: i32, ocvrs_return: *mut Result<*mut c_void>);
	// save(const cv::String &) /usr/include/opencv2/core.hpp:2622
	pub fn cv_LDA_save_const_const_StringR(instance: *const c_void, filename: *const c_char, ocvrs_return: *mut Result_void);
	// load(const cv::String &) /usr/include/opencv2/core.hpp:2626
	pub fn cv_LDA_load_const_StringR(instance: *mut c_void, filename: *const c_char, ocvrs_return: *mut Result_void);
	// save(cv::FileStorage &) /usr/include/opencv2/core.hpp:2630
	pub fn cv_LDA_save_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// load(const cv::FileStorage &) /usr/include/opencv2/core.hpp:2634
	pub fn cv_LDA_load_const_FileStorageR(instance: *mut c_void, node: *const c_void, ocvrs_return: *mut Result_void);
	// compute(cv::InputArrayOfArrays, cv::InputArray) /usr/include/opencv2/core.hpp:2642
	pub fn cv_LDA_compute_const__InputArrayR_const__InputArrayR(instance: *mut c_void, src: *const c_void, labels: *const c_void, ocvrs_return: *mut Result_void);
	// project(cv::InputArray) /usr/include/opencv2/core.hpp:2647
	pub fn cv_LDA_project_const__InputArrayR(instance: *mut c_void, src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// reconstruct(cv::InputArray) /usr/include/opencv2/core.hpp:2652
	pub fn cv_LDA_reconstruct_const__InputArrayR(instance: *mut c_void, src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// eigenvectors() /usr/include/opencv2/core.hpp:2656
	pub fn cv_LDA_eigenvectors_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// eigenvalues() /usr/include/opencv2/core.hpp:2660
	pub fn cv_LDA_eigenvalues_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// subspaceProject(cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/core.hpp:2662
	pub fn cv_LDA_subspaceProject_const__InputArrayR_const__InputArrayR_const__InputArrayR(w: *const c_void, mean: *const c_void, src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// subspaceReconstruct(cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/core.hpp:2663
	pub fn cv_LDA_subspaceReconstruct_const__InputArrayR_const__InputArrayR_const__InputArrayR(w: *const c_void, mean: *const c_void, src: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// flags /usr/include/opencv2/core/mat.hpp:2112
	pub fn cv_Mat_getPropFlags_const(instance: *const c_void) -> i32;
	// flags /usr/include/opencv2/core/mat.hpp:2112
	pub fn cv_Mat_setPropFlags_int(instance: *mut c_void, val: i32);
	// dims /usr/include/opencv2/core/mat.hpp:2114
	pub fn cv_Mat_getPropDims_const(instance: *const c_void) -> i32;
	// dims /usr/include/opencv2/core/mat.hpp:2114
	pub fn cv_Mat_setPropDims_int(instance: *mut c_void, val: i32);
	// rows /usr/include/opencv2/core/mat.hpp:2116
	pub fn cv_Mat_getPropRows_const(instance: *const c_void) -> i32;
	// rows /usr/include/opencv2/core/mat.hpp:2116
	pub fn cv_Mat_setPropRows_int(instance: *mut c_void, val: i32);
	// cols /usr/include/opencv2/core/mat.hpp:2116
	pub fn cv_Mat_getPropCols_const(instance: *const c_void) -> i32;
	// cols /usr/include/opencv2/core/mat.hpp:2116
	pub fn cv_Mat_setPropCols_int(instance: *mut c_void, val: i32);
	// data /usr/include/opencv2/core/mat.hpp:2118
	pub fn cv_Mat_getPropData(instance: *mut c_void) -> *mut u8;
	// data /usr/include/opencv2/core/mat.hpp:2118
	pub fn cv_Mat_setPropData_unsigned_charX(instance: *mut c_void, val: *mut u8);
	// datastart /usr/include/opencv2/core/mat.hpp:2121
	pub fn cv_Mat_getPropDatastart_const(instance: *const c_void) -> *const u8;
	// dataend /usr/include/opencv2/core/mat.hpp:2122
	pub fn cv_Mat_getPropDataend_const(instance: *const c_void) -> *const u8;
	// datalimit /usr/include/opencv2/core/mat.hpp:2123
	pub fn cv_Mat_getPropDatalimit_const(instance: *const c_void) -> *const u8;
	// u /usr/include/opencv2/core/mat.hpp:2136
	pub fn cv_Mat_getPropU(instance: *mut c_void) -> *mut c_void;
	// u /usr/include/opencv2/core/mat.hpp:2136
	pub fn cv_Mat_setPropU_UMatDataX(instance: *mut c_void, val: *mut c_void);
	// size /usr/include/opencv2/core/mat.hpp:2138
	pub fn cv_Mat_getPropSize_const(instance: *const c_void) -> *mut c_void;
	// step /usr/include/opencv2/core/mat.hpp:2139
	pub fn cv_Mat_getPropStep_const(instance: *const c_void) -> *mut c_void;
	// Mat() /usr/include/opencv2/core/mat.hpp:819
	pub fn cv_Mat_Mat() -> *mut c_void;
	// Mat(int, int, int) /usr/include/opencv2/core/mat.hpp:827
	pub fn cv_Mat_Mat_int_int_int(rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// Mat(cv::Size, int) /usr/include/opencv2/core/mat.hpp:835
	pub fn cv_Mat_Mat_Size_int(size: *const core::Size, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// Mat(int, int, int, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:846
	pub fn cv_Mat_Mat_int_int_int_const_ScalarR(rows: i32, cols: i32, typ: i32, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// Mat(cv::Size, int, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:857
	pub fn cv_Mat_Mat_Size_int_const_ScalarR(size: *const core::Size, typ: i32, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// Mat(int, const int *, int) /usr/include/opencv2/core/mat.hpp:865
	pub fn cv_Mat_Mat_int_const_intX_int(ndims: i32, sizes: *const i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// Mat(const std::vector<int> &, int) /usr/include/opencv2/core/mat.hpp:872
	pub fn cv_Mat_Mat_const_vector_int_R_int(sizes: *const c_void, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// Mat(int, const int *, int, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:883
	pub fn cv_Mat_Mat_int_const_intX_int_const_ScalarR(ndims: i32, sizes: *const i32, typ: i32, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// Mat(const std::vector<int> &, int, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:893
	pub fn cv_Mat_Mat_const_vector_int_R_int_const_ScalarR(sizes: *const c_void, typ: i32, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// Mat(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:903
	pub fn cv_Mat_Mat_const_MatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Mat(int, int, int, void *, size_t) /usr/include/opencv2/core/mat.hpp:919
	pub fn cv_Mat_Mat_int_int_int_voidX_size_t(rows: i32, cols: i32, typ: i32, data: *mut c_void, step: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// Mat(cv::Size, int, void *, size_t) /usr/include/opencv2/core/mat.hpp:935
	pub fn cv_Mat_Mat_Size_int_voidX_size_t(size: *const core::Size, typ: i32, data: *mut c_void, step: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// Mat(int, const int *, int, void *, const size_t *) /usr/include/opencv2/core/mat.hpp:950
	pub fn cv_Mat_Mat_int_const_intX_int_voidX_const_size_tX(ndims: i32, sizes: *const i32, typ: i32, data: *mut c_void, steps: *const size_t, ocvrs_return: *mut Result<*mut c_void>);
	// Mat(const std::vector<int> &, int, void *, const size_t *) /usr/include/opencv2/core/mat.hpp:964
	pub fn cv_Mat_Mat_const_vector_int_R_int_voidX_const_size_tX(sizes: *const c_void, typ: i32, data: *mut c_void, steps: *const size_t, ocvrs_return: *mut Result<*mut c_void>);
	// Mat(const cv::Mat &, const cv::Range &, const cv::Range &) /usr/include/opencv2/core/mat.hpp:976
	pub fn cv_Mat_Mat_const_MatR_const_RangeR_const_RangeR(m: *const c_void, row_range: *const c_void, col_range: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Mat(const cv::Mat &, const cv::Rect &) /usr/include/opencv2/core/mat.hpp:986
	pub fn cv_Mat_Mat_const_MatR_const_RectR(m: *const c_void, roi: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
	// Mat(const cv::Mat &, const std::vector<Range> &) /usr/include/opencv2/core/mat.hpp:1006
	pub fn cv_Mat_Mat_const_MatR_const_vector_Range_R(m: *const c_void, ranges: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Mat(const cuda::GpuMat &) /usr/include/opencv2/core/mat.hpp:1060
	pub fn cv_Mat_Mat_const_GpuMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getUMat(cv::AccessFlag, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:1085
	pub fn cv_Mat_getUMat_const_AccessFlag_UMatUsageFlags(instance: *const c_void, access_flags: core::AccessFlag, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
	// row(int) /usr/include/opencv2/core/mat.hpp:1120
	pub fn cv_Mat_row_const_int(instance: *const c_void, y: i32, ocvrs_return: *mut Result<*mut c_void>);
	// col(int) /usr/include/opencv2/core/mat.hpp:1129
	pub fn cv_Mat_col_const_int(instance: *const c_void, x: i32, ocvrs_return: *mut Result<*mut c_void>);
	// rowRange(int, int) /usr/include/opencv2/core/mat.hpp:1138
	pub fn cv_Mat_rowRange_const_int_int(instance: *const c_void, startrow: i32, endrow: i32, ocvrs_return: *mut Result<*mut c_void>);
	// rowRange(const cv::Range &) /usr/include/opencv2/core/mat.hpp:1143
	pub fn cv_Mat_rowRange_const_const_RangeR(instance: *const c_void, r: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// colRange(int, int) /usr/include/opencv2/core/mat.hpp:1152
	pub fn cv_Mat_colRange_const_int_int(instance: *const c_void, startcol: i32, endcol: i32, ocvrs_return: *mut Result<*mut c_void>);
	// colRange(const cv::Range &) /usr/include/opencv2/core/mat.hpp:1157
	pub fn cv_Mat_colRange_const_const_RangeR(instance: *const c_void, r: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// diag(int) /usr/include/opencv2/core/mat.hpp:1193
	pub fn cv_Mat_diag_const_int(instance: *const c_void, d: i32, ocvrs_return: *mut Result<*mut c_void>);
	// diag(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:1200
	pub fn cv_Mat_diag_const_MatR(d: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// clone() /usr/include/opencv2/core/mat.hpp:1207
	pub fn cv_Mat_clone_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// copyTo(cv::OutputArray) /usr/include/opencv2/core/mat.hpp:1224
	pub fn cv_Mat_copyTo_const_const__OutputArrayR(instance: *const c_void, m: *const c_void, ocvrs_return: *mut Result_void);
	// copyTo(cv::OutputArray, cv::InputArray) /usr/include/opencv2/core/mat.hpp:1232
	pub fn cv_Mat_copyTo_const_const__OutputArrayR_const__InputArrayR(instance: *const c_void, m: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// convertTo(cv::OutputArray, int, double, double) /usr/include/opencv2/core/mat.hpp:1247
	pub fn cv_Mat_convertTo_const_const__OutputArrayR_int_double_double(instance: *const c_void, m: *const c_void, rtype: i32, alpha: f64, beta: f64, ocvrs_return: *mut Result_void);
	// assignTo(cv::Mat &, int) /usr/include/opencv2/core/mat.hpp:1255
	pub fn cv_Mat_assignTo_const_MatR_int(instance: *const c_void, m: *mut c_void, typ: i32, ocvrs_return: *mut Result_void);
	// setTo(cv::InputArray, cv::InputArray) /usr/include/opencv2/core/mat.hpp:1269
	pub fn cv_Mat_setTo_const__InputArrayR_const__InputArrayR(instance: *mut c_void, value: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// reshape(int, int) /usr/include/opencv2/core/mat.hpp:1295
	pub fn cv_Mat_reshape_const_int_int(instance: *const c_void, cn: i32, rows: i32, ocvrs_return: *mut Result<*mut c_void>);
	// reshape(int, int, const int *) /usr/include/opencv2/core/mat.hpp:1298
	pub fn cv_Mat_reshape_const_int_int_const_intX(instance: *const c_void, cn: i32, newndims: i32, newsz: *const i32, ocvrs_return: *mut Result<*mut c_void>);
	// reshape(int, const std::vector<int> &) /usr/include/opencv2/core/mat.hpp:1301
	pub fn cv_Mat_reshape_const_int_const_vector_int_R(instance: *const c_void, cn: i32, newshape: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// t() /usr/include/opencv2/core/mat.hpp:1313
	pub fn cv_Mat_t_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// inv(int) /usr/include/opencv2/core/mat.hpp:1322
	pub fn cv_Mat_inv_const_int(instance: *const c_void, method: i32, ocvrs_return: *mut Result<*mut c_void>);
	// mul(cv::InputArray, double) /usr/include/opencv2/core/mat.hpp:1336
	pub fn cv_Mat_mul_const_const__InputArrayR_double(instance: *const c_void, m: *const c_void, scale: f64, ocvrs_return: *mut Result<*mut c_void>);
	// cross(cv::InputArray) /usr/include/opencv2/core/mat.hpp:1345
	pub fn cv_Mat_cross_const_const__InputArrayR(instance: *const c_void, m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// dot(cv::InputArray) /usr/include/opencv2/core/mat.hpp:1355
	pub fn cv_Mat_dot_const_const__InputArrayR(instance: *const c_void, m: *const c_void, ocvrs_return: *mut Result<f64>);
	// zeros(int, int, int) /usr/include/opencv2/core/mat.hpp:1371
	pub fn cv_Mat_zeros_int_int_int(rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// zeros(cv::Size, int) /usr/include/opencv2/core/mat.hpp:1377
	pub fn cv_Mat_zeros_Size_int(size: *const core::Size, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// zeros(int, const int *, int) /usr/include/opencv2/core/mat.hpp:1384
	pub fn cv_Mat_zeros_int_const_intX_int(ndims: i32, sz: *const i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// ones(int, int, int) /usr/include/opencv2/core/mat.hpp:1402
	pub fn cv_Mat_ones_int_int_int(rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// ones(cv::Size, int) /usr/include/opencv2/core/mat.hpp:1408
	pub fn cv_Mat_ones_Size_int(size: *const core::Size, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// ones(int, const int *, int) /usr/include/opencv2/core/mat.hpp:1415
	pub fn cv_Mat_ones_int_const_intX_int(ndims: i32, sz: *const i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// eye(int, int, int) /usr/include/opencv2/core/mat.hpp:1431
	pub fn cv_Mat_eye_int_int_int(rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// eye(cv::Size, int) /usr/include/opencv2/core/mat.hpp:1437
	pub fn cv_Mat_eye_Size_int(size: *const core::Size, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, int, int) /usr/include/opencv2/core/mat.hpp:1472
	pub fn cv_Mat_create_int_int_int(instance: *mut c_void, rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result_void);
	// create(cv::Size, int) /usr/include/opencv2/core/mat.hpp:1478
	pub fn cv_Mat_create_Size_int(instance: *mut c_void, size: *const core::Size, typ: i32, ocvrs_return: *mut Result_void);
	// create(int, const int *, int) /usr/include/opencv2/core/mat.hpp:1485
	pub fn cv_Mat_create_int_const_intX_int(instance: *mut c_void, ndims: i32, sizes: *const i32, typ: i32, ocvrs_return: *mut Result_void);
	// create(const std::vector<int> &, int) /usr/include/opencv2/core/mat.hpp:1491
	pub fn cv_Mat_create_const_vector_int_R_int(instance: *mut c_void, sizes: *const c_void, typ: i32, ocvrs_return: *mut Result_void);
	// addref() /usr/include/opencv2/core/mat.hpp:1502
	pub fn cv_Mat_addref(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// release() /usr/include/opencv2/core/mat.hpp:1517
	pub fn cv_Mat_release(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// deallocate() /usr/include/opencv2/core/mat.hpp:1520
	pub fn cv_Mat_deallocate(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// reserve(size_t) /usr/include/opencv2/core/mat.hpp:1531
	pub fn cv_Mat_reserve_size_t(instance: *mut c_void, sz: size_t, ocvrs_return: *mut Result_void);
	// reserveBuffer(size_t) /usr/include/opencv2/core/mat.hpp:1539
	pub fn cv_Mat_reserveBuffer_size_t(instance: *mut c_void, sz: size_t, ocvrs_return: *mut Result_void);
	// resize(size_t) /usr/include/opencv2/core/mat.hpp:1548
	pub fn cv_Mat_resize_size_t(instance: *mut c_void, sz: size_t, ocvrs_return: *mut Result_void);
	// resize(size_t, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:1554
	pub fn cv_Mat_resize_size_t_const_ScalarR(instance: *mut c_void, sz: size_t, s: *const core::Scalar, ocvrs_return: *mut Result_void);
	// push_back(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:1581
	pub fn cv_Mat_push_back_const_MatR(instance: *mut c_void, m: *const c_void, ocvrs_return: *mut Result_void);
	// pop_back(size_t) /usr/include/opencv2/core/mat.hpp:1589
	pub fn cv_Mat_pop_back_size_t(instance: *mut c_void, nelems: size_t, ocvrs_return: *mut Result_void);
	// locateROI(cv::Size &, cv::Point &) /usr/include/opencv2/core/mat.hpp:1602
	pub fn cv_Mat_locateROI_const_SizeR_PointR(instance: *const c_void, whole_size: *mut core::Size, ofs: *mut core::Point, ocvrs_return: *mut Result_void);
	// adjustROI(int, int, int, int) /usr/include/opencv2/core/mat.hpp:1631
	pub fn cv_Mat_adjustROI_int_int_int_int(instance: *mut c_void, dtop: i32, dbottom: i32, dleft: i32, dright: i32, ocvrs_return: *mut Result<*mut c_void>);
	// isContinuous() /usr/include/opencv2/core/mat.hpp:1741
	pub fn cv_Mat_isContinuous_const(instance: *const c_void) -> bool;
	// isSubmatrix() /usr/include/opencv2/core/mat.hpp:1744
	pub fn cv_Mat_isSubmatrix_const(instance: *const c_void) -> bool;
	// elemSize() /usr/include/opencv2/core/mat.hpp:1751
	pub fn cv_Mat_elemSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// elemSize1() /usr/include/opencv2/core/mat.hpp:1758
	pub fn cv_Mat_elemSize1_const(instance: *const c_void) -> size_t;
	// type() /usr/include/opencv2/core/mat.hpp:1765
	pub fn cv_Mat_type_const(instance: *const c_void) -> i32;
	// depth() /usr/include/opencv2/core/mat.hpp:1780
	pub fn cv_Mat_depth_const(instance: *const c_void) -> i32;
	// channels() /usr/include/opencv2/core/mat.hpp:1786
	pub fn cv_Mat_channels_const(instance: *const c_void) -> i32;
	// step1(int) /usr/include/opencv2/core/mat.hpp:1793
	pub fn cv_Mat_step1_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<size_t>);
	// empty() /usr/include/opencv2/core/mat.hpp:1800
	pub fn cv_Mat_empty_const(instance: *const c_void) -> bool;
	// total() /usr/include/opencv2/core/mat.hpp:1807
	pub fn cv_Mat_total_const(instance: *const c_void) -> size_t;
	// total(int, int) /usr/include/opencv2/core/mat.hpp:1813
	pub fn cv_Mat_total_const_int_int(instance: *const c_void, start_dim: i32, end_dim: i32, ocvrs_return: *mut Result<size_t>);
	// checkVector(int, int, bool) /usr/include/opencv2/core/mat.hpp:1836
	pub fn cv_Mat_checkVector_const_int_int_bool(instance: *const c_void, elem_channels: i32, depth: i32, require_continuous: bool, ocvrs_return: *mut Result<i32>);
	// ptr(int) /usr/include/opencv2/core/mat.hpp:1844
	pub fn cv_Mat_ptr_int(instance: *mut c_void, i0: i32, ocvrs_return: *mut Result<*mut u8>);
	// ptr(int) /usr/include/opencv2/core/mat.hpp:1846
	pub fn cv_Mat_ptr_const_int(instance: *const c_void, i0: i32, ocvrs_return: *mut Result<*const u8>);
	// ptr(int, int) /usr/include/opencv2/core/mat.hpp:1852
	pub fn cv_Mat_ptr_int_int(instance: *mut c_void, row: i32, col: i32, ocvrs_return: *mut Result<*mut u8>);
	// ptr(int, int) /usr/include/opencv2/core/mat.hpp:1857
	pub fn cv_Mat_ptr_const_int_int(instance: *const c_void, row: i32, col: i32, ocvrs_return: *mut Result<*const u8>);
	// ptr(int, int, int) /usr/include/opencv2/core/mat.hpp:1860
	pub fn cv_Mat_ptr_int_int_int(instance: *mut c_void, i0: i32, i1: i32, i2: i32, ocvrs_return: *mut Result<*mut u8>);
	// ptr(int, int, int) /usr/include/opencv2/core/mat.hpp:1862
	pub fn cv_Mat_ptr_const_int_int_int(instance: *const c_void, i0: i32, i1: i32, i2: i32, ocvrs_return: *mut Result<*const u8>);
	// ptr(const int *) /usr/include/opencv2/core/mat.hpp:1865
	pub fn cv_Mat_ptr_const_intX(instance: *mut c_void, idx: *const i32, ocvrs_return: *mut Result<*mut u8>);
	// ptr(const int *) /usr/include/opencv2/core/mat.hpp:1867
	pub fn cv_Mat_ptr_const_const_intX(instance: *const c_void, idx: *const i32, ocvrs_return: *mut Result<*const u8>);
	// Mat(cv::Mat &&) /usr/include/opencv2/core/mat.hpp:2100
	pub fn cv_Mat_Mat_MatR(m: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// updateContinuityFlag() /usr/include/opencv2/core/mat.hpp:2133
	pub fn cv_Mat_updateContinuityFlag(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// m /usr/include/opencv2/core/mat.hpp:3112
	pub fn cv_MatConstIterator_getPropM_const(instance: *const c_void) -> *mut c_void;
	// elemSize /usr/include/opencv2/core/mat.hpp:3113
	pub fn cv_MatConstIterator_getPropElemSize_const(instance: *const c_void) -> size_t;
	// elemSize /usr/include/opencv2/core/mat.hpp:3113
	pub fn cv_MatConstIterator_setPropElemSize_size_t(instance: *mut c_void, val: size_t);
	// ptr /usr/include/opencv2/core/mat.hpp:3114
	pub fn cv_MatConstIterator_getPropPtr_const(instance: *const c_void) -> *const u8;
	// sliceStart /usr/include/opencv2/core/mat.hpp:3115
	pub fn cv_MatConstIterator_getPropSliceStart_const(instance: *const c_void) -> *const u8;
	// sliceEnd /usr/include/opencv2/core/mat.hpp:3116
	pub fn cv_MatConstIterator_getPropSliceEnd_const(instance: *const c_void) -> *const u8;
	// MatConstIterator() /usr/include/opencv2/core/mat.hpp:3072
	pub fn cv_MatConstIterator_MatConstIterator(ocvrs_return: *mut Result<*mut c_void>);
	// MatConstIterator(const cv::Mat *) /usr/include/opencv2/core/mat.hpp:3074
	pub fn cv_MatConstIterator_MatConstIterator_const_MatX(_m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// MatConstIterator(const cv::Mat *, int, int) /usr/include/opencv2/core/mat.hpp:3076
	pub fn cv_MatConstIterator_MatConstIterator_const_MatX_int_int(_m: *const c_void, _row: i32, _col: i32, ocvrs_return: *mut Result<*mut c_void>);
	// MatConstIterator(const cv::Mat *, cv::Point) /usr/include/opencv2/core/mat.hpp:3078
	pub fn cv_MatConstIterator_MatConstIterator_const_MatX_Point(_m: *const c_void, _pt: *const core::Point, ocvrs_return: *mut Result<*mut c_void>);
	// MatConstIterator(const cv::MatConstIterator &) /usr/include/opencv2/core/mat.hpp:3082
	pub fn cv_MatConstIterator_MatConstIterator_const_MatConstIteratorR(it: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator*() /usr/include/opencv2/core/mat.hpp:3087
	pub fn cv_MatConstIterator_operatorX_const(instance: *const c_void, ocvrs_return: *mut Result<*const u8>);
	// operator[](ptrdiff_t) /usr/include/opencv2/core/mat.hpp:3089
	pub fn cv_MatConstIterator_operator___const_ptrdiff_t(instance: *const c_void, i: ptrdiff_t, ocvrs_return: *mut Result<*const u8>);
	// operator--() /usr/include/opencv2/core/mat.hpp:3096
	pub fn cv_MatConstIterator_operatorSS(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator++() /usr/include/opencv2/core/mat.hpp:3100
	pub fn cv_MatConstIterator_operatorAA(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// pos() /usr/include/opencv2/core/mat.hpp:3104
	pub fn cv_MatConstIterator_pos_const(instance: *const c_void, ocvrs_return: *mut Result<core::Point>);
	// pos(int *) /usr/include/opencv2/core/mat.hpp:3106
	pub fn cv_MatConstIterator_pos_const_intX(instance: *const c_void, _idx: *mut i32, ocvrs_return: *mut Result_void);
	// lpos() /usr/include/opencv2/core/mat.hpp:3108
	pub fn cv_MatConstIterator_lpos_const(instance: *const c_void, ocvrs_return: *mut Result<ptrdiff_t>);
	// seek(ptrdiff_t, bool) /usr/include/opencv2/core/mat.hpp:3109
	pub fn cv_MatConstIterator_seek_ptrdiff_t_bool(instance: *mut c_void, ofs: ptrdiff_t, relative: bool, ocvrs_return: *mut Result_void);
	// seek(const int *, bool) /usr/include/opencv2/core/mat.hpp:3110
	pub fn cv_MatConstIterator_seek_const_intX_bool(instance: *mut c_void, _idx: *const i32, relative: bool, ocvrs_return: *mut Result_void);
	// flags /usr/include/opencv2/core/mat.hpp:3595
	pub fn cv_MatExpr_getPropFlags_const(instance: *const c_void) -> i32;
	// flags /usr/include/opencv2/core/mat.hpp:3595
	pub fn cv_MatExpr_setPropFlags_int(instance: *mut c_void, val: i32);
	// a /usr/include/opencv2/core/mat.hpp:3597
	pub fn cv_MatExpr_getPropA_const(instance: *const c_void) -> *mut c_void;
	// a /usr/include/opencv2/core/mat.hpp:3597
	pub fn cv_MatExpr_setPropA_Mat(instance: *mut c_void, val: *mut c_void);
	// b /usr/include/opencv2/core/mat.hpp:3597
	pub fn cv_MatExpr_getPropB_const(instance: *const c_void) -> *mut c_void;
	// b /usr/include/opencv2/core/mat.hpp:3597
	pub fn cv_MatExpr_setPropB_Mat(instance: *mut c_void, val: *mut c_void);
	// c /usr/include/opencv2/core/mat.hpp:3597
	pub fn cv_MatExpr_getPropC_const(instance: *const c_void) -> *mut c_void;
	// c /usr/include/opencv2/core/mat.hpp:3597
	pub fn cv_MatExpr_setPropC_Mat(instance: *mut c_void, val: *mut c_void);
	// alpha /usr/include/opencv2/core/mat.hpp:3598
	pub fn cv_MatExpr_getPropAlpha_const(instance: *const c_void) -> f64;
	// alpha /usr/include/opencv2/core/mat.hpp:3598
	pub fn cv_MatExpr_setPropAlpha_double(instance: *mut c_void, val: f64);
	// beta /usr/include/opencv2/core/mat.hpp:3598
	pub fn cv_MatExpr_getPropBeta_const(instance: *const c_void) -> f64;
	// beta /usr/include/opencv2/core/mat.hpp:3598
	pub fn cv_MatExpr_setPropBeta_double(instance: *mut c_void, val: f64);
	// s /usr/include/opencv2/core/mat.hpp:3599
	pub fn cv_MatExpr_getPropS_const(instance: *const c_void, ocvrs_return: *mut core::Scalar);
	// s /usr/include/opencv2/core/mat.hpp:3599
	pub fn cv_MatExpr_setPropS_Scalar(instance: *mut c_void, val: *const core::Scalar);
	// MatExpr() /usr/include/opencv2/core/mat.hpp:3566
	pub fn cv_MatExpr_MatExpr(ocvrs_return: *mut Result<*mut c_void>);
	// MatExpr(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3567
	pub fn cv_MatExpr_MatExpr_const_MatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// MatExpr(const cv::MatOp *, int, const cv::Mat &, const cv::Mat &, const cv::Mat &, double, double, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:3569
	pub fn cv_MatExpr_MatExpr_const_MatOpX_int_const_MatR_const_MatR_const_MatR_double_double_const_ScalarR(_op: *const c_void, _flags: i32, _a: *const c_void, _b: *const c_void, _c: *const c_void, _alpha: f64, _beta: f64, _s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// operator Mat() /usr/include/opencv2/core/mat.hpp:3572
	pub fn cv_MatExpr_operator_cv_Mat_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// size() /usr/include/opencv2/core/mat.hpp:3575
	pub fn cv_MatExpr_size_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// type() /usr/include/opencv2/core/mat.hpp:3576
	pub fn cv_MatExpr_type_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// row(int) /usr/include/opencv2/core/mat.hpp:3578
	pub fn cv_MatExpr_row_const_int(instance: *const c_void, y: i32, ocvrs_return: *mut Result<*mut c_void>);
	// col(int) /usr/include/opencv2/core/mat.hpp:3579
	pub fn cv_MatExpr_col_const_int(instance: *const c_void, x: i32, ocvrs_return: *mut Result<*mut c_void>);
	// diag(int) /usr/include/opencv2/core/mat.hpp:3580
	pub fn cv_MatExpr_diag_const_int(instance: *const c_void, d: i32, ocvrs_return: *mut Result<*mut c_void>);
	// t() /usr/include/opencv2/core/mat.hpp:3584
	pub fn cv_MatExpr_t_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// inv(int) /usr/include/opencv2/core/mat.hpp:3585
	pub fn cv_MatExpr_inv_const_int(instance: *const c_void, method: i32, ocvrs_return: *mut Result<*mut c_void>);
	// mul(const cv::MatExpr &, double) /usr/include/opencv2/core/mat.hpp:3586
	pub fn cv_MatExpr_mul_const_const_MatExprR_double(instance: *const c_void, e: *const c_void, scale: f64, ocvrs_return: *mut Result<*mut c_void>);
	// mul(const cv::Mat &, double) /usr/include/opencv2/core/mat.hpp:3587
	pub fn cv_MatExpr_mul_const_const_MatR_double(instance: *const c_void, m: *const c_void, scale: f64, ocvrs_return: *mut Result<*mut c_void>);
	// cross(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3589
	pub fn cv_MatExpr_cross_const_const_MatR(instance: *const c_void, m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// dot(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3590
	pub fn cv_MatExpr_dot_const_const_MatR(instance: *const c_void, m: *const c_void, ocvrs_return: *mut Result<f64>);
	// swap(cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3592
	pub fn cv_MatExpr_swap_MatExprR(instance: *mut c_void, b: *mut c_void, ocvrs_return: *mut Result_void);
	// elementWise(const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3482
	pub fn cv_MatOp_elementWise_const_const_MatExprR(instance: *const c_void, expr: *const c_void, ocvrs_return: *mut Result<bool>);
	// assign(const cv::MatExpr &, cv::Mat &, int) /usr/include/opencv2/core/mat.hpp:3483
	pub fn cv_MatOp_assign_const_const_MatExprR_MatR_int(instance: *const c_void, expr: *const c_void, m: *mut c_void, typ: i32, ocvrs_return: *mut Result_void);
	// roi(const cv::MatExpr &, const cv::Range &, const cv::Range &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3484
	pub fn cv_MatOp_roi_const_const_MatExprR_const_RangeR_const_RangeR_MatExprR(instance: *const c_void, expr: *const c_void, row_range: *const c_void, col_range: *const c_void, res: *mut c_void, ocvrs_return: *mut Result_void);
	// diag(const cv::MatExpr &, int, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3486
	pub fn cv_MatOp_diag_const_const_MatExprR_int_MatExprR(instance: *const c_void, expr: *const c_void, d: i32, res: *mut c_void, ocvrs_return: *mut Result_void);
	// augAssignAdd(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3487
	pub fn cv_MatOp_augAssignAdd_const_const_MatExprR_MatR(instance: *const c_void, expr: *const c_void, m: *mut c_void, ocvrs_return: *mut Result_void);
	// augAssignSubtract(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3488
	pub fn cv_MatOp_augAssignSubtract_const_const_MatExprR_MatR(instance: *const c_void, expr: *const c_void, m: *mut c_void, ocvrs_return: *mut Result_void);
	// augAssignMultiply(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3489
	pub fn cv_MatOp_augAssignMultiply_const_const_MatExprR_MatR(instance: *const c_void, expr: *const c_void, m: *mut c_void, ocvrs_return: *mut Result_void);
	// augAssignDivide(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3490
	pub fn cv_MatOp_augAssignDivide_const_const_MatExprR_MatR(instance: *const c_void, expr: *const c_void, m: *mut c_void, ocvrs_return: *mut Result_void);
	// augAssignAnd(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3491
	pub fn cv_MatOp_augAssignAnd_const_const_MatExprR_MatR(instance: *const c_void, expr: *const c_void, m: *mut c_void, ocvrs_return: *mut Result_void);
	// augAssignOr(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3492
	pub fn cv_MatOp_augAssignOr_const_const_MatExprR_MatR(instance: *const c_void, expr: *const c_void, m: *mut c_void, ocvrs_return: *mut Result_void);
	// augAssignXor(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3493
	pub fn cv_MatOp_augAssignXor_const_const_MatExprR_MatR(instance: *const c_void, expr: *const c_void, m: *mut c_void, ocvrs_return: *mut Result_void);
	// add(const cv::MatExpr &, const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3495
	pub fn cv_MatOp_add_const_const_MatExprR_const_MatExprR_MatExprR(instance: *const c_void, expr1: *const c_void, expr2: *const c_void, res: *mut c_void, ocvrs_return: *mut Result_void);
	// add(const cv::MatExpr &, const cv::Scalar &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3496
	pub fn cv_MatOp_add_const_const_MatExprR_const_ScalarR_MatExprR(instance: *const c_void, expr1: *const c_void, s: *const core::Scalar, res: *mut c_void, ocvrs_return: *mut Result_void);
	// subtract(const cv::MatExpr &, const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3498
	pub fn cv_MatOp_subtract_const_const_MatExprR_const_MatExprR_MatExprR(instance: *const c_void, expr1: *const c_void, expr2: *const c_void, res: *mut c_void, ocvrs_return: *mut Result_void);
	// subtract(const cv::Scalar &, const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3499
	pub fn cv_MatOp_subtract_const_const_ScalarR_const_MatExprR_MatExprR(instance: *const c_void, s: *const core::Scalar, expr: *const c_void, res: *mut c_void, ocvrs_return: *mut Result_void);
	// multiply(const cv::MatExpr &, const cv::MatExpr &, cv::MatExpr &, double) /usr/include/opencv2/core/mat.hpp:3501
	pub fn cv_MatOp_multiply_const_const_MatExprR_const_MatExprR_MatExprR_double(instance: *const c_void, expr1: *const c_void, expr2: *const c_void, res: *mut c_void, scale: f64, ocvrs_return: *mut Result_void);
	// multiply(const cv::MatExpr &, double, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3502
	pub fn cv_MatOp_multiply_const_const_MatExprR_double_MatExprR(instance: *const c_void, expr1: *const c_void, s: f64, res: *mut c_void, ocvrs_return: *mut Result_void);
	// divide(const cv::MatExpr &, const cv::MatExpr &, cv::MatExpr &, double) /usr/include/opencv2/core/mat.hpp:3504
	pub fn cv_MatOp_divide_const_const_MatExprR_const_MatExprR_MatExprR_double(instance: *const c_void, expr1: *const c_void, expr2: *const c_void, res: *mut c_void, scale: f64, ocvrs_return: *mut Result_void);
	// divide(double, const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3505
	pub fn cv_MatOp_divide_const_double_const_MatExprR_MatExprR(instance: *const c_void, s: f64, expr: *const c_void, res: *mut c_void, ocvrs_return: *mut Result_void);
	// abs(const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3507
	pub fn cv_MatOp_abs_const_const_MatExprR_MatExprR(instance: *const c_void, expr: *const c_void, res: *mut c_void, ocvrs_return: *mut Result_void);
	// transpose(const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3509
	pub fn cv_MatOp_transpose_const_const_MatExprR_MatExprR(instance: *const c_void, expr: *const c_void, res: *mut c_void, ocvrs_return: *mut Result_void);
	// matmul(const cv::MatExpr &, const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3510
	pub fn cv_MatOp_matmul_const_const_MatExprR_const_MatExprR_MatExprR(instance: *const c_void, expr1: *const c_void, expr2: *const c_void, res: *mut c_void, ocvrs_return: *mut Result_void);
	// invert(const cv::MatExpr &, int, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3511
	pub fn cv_MatOp_invert_const_const_MatExprR_int_MatExprR(instance: *const c_void, expr: *const c_void, method: i32, res: *mut c_void, ocvrs_return: *mut Result_void);
	// size(const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3513
	pub fn cv_MatOp_size_const_const_MatExprR(instance: *const c_void, expr: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// type(const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3514
	pub fn cv_MatOp_type_const_const_MatExprR(instance: *const c_void, expr: *const c_void, ocvrs_return: *mut Result<i32>);
	// p /usr/include/opencv2/core/mat.hpp:597
	pub fn cv_MatSize_getPropP(instance: *mut c_void) -> *mut i32;
	// p /usr/include/opencv2/core/mat.hpp:597
	pub fn cv_MatSize_setPropP_intX(instance: *mut c_void, val: *mut i32);
	// MatSize(int *) /usr/include/opencv2/core/mat.hpp:588
	pub fn cv_MatSize_MatSize_intX(_p: *mut i32) -> *mut c_void;
	// dims() /usr/include/opencv2/core/mat.hpp:589
	pub fn cv_MatSize_dims_const(instance: *const c_void) -> i32;
	// operator[](int) /usr/include/opencv2/core/mat.hpp:591
	pub fn cv_MatSize_operator___const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<i32>);
	// operator[](int) /usr/include/opencv2/core/mat.hpp:592
	pub fn cv_MatSize_operator___int(instance: *mut c_void, i: i32, ocvrs_return: *mut Result<i32>);
	// operator const int *() /usr/include/opencv2/core/mat.hpp:593
	pub fn cv_MatSize_operator_const_intX_const(instance: *const c_void) -> *const i32;
	// operator==(const cv::MatSize &) /usr/include/opencv2/core/mat.hpp:594
	pub fn cv_MatSize_operatorEQ_const_const_MatSizeR(instance: *const c_void, sz: *const c_void) -> bool;
	// p /usr/include/opencv2/core/mat.hpp:609
	pub fn cv_MatStep_getPropP(instance: *mut c_void) -> *mut size_t;
	// p /usr/include/opencv2/core/mat.hpp:609
	pub fn cv_MatStep_setPropP_size_tX(instance: *mut c_void, val: *mut size_t);
	// buf /usr/include/opencv2/core/mat.hpp:610
	pub fn cv_MatStep_getPropBuf(instance: *mut c_void) -> *mut [size_t; 2];
	// MatStep() /usr/include/opencv2/core/mat.hpp:602
	pub fn cv_MatStep_MatStep() -> *mut c_void;
	// MatStep(size_t) /usr/include/opencv2/core/mat.hpp:603
	pub fn cv_MatStep_MatStep_size_t(s: size_t) -> *mut c_void;
	// operator[](int) /usr/include/opencv2/core/mat.hpp:604
	pub fn cv_MatStep_operator___const_int(instance: *const c_void, i: i32) -> size_t;
	// operator[](int) /usr/include/opencv2/core/mat.hpp:605
	pub fn cv_MatStep_operator___int(instance: *mut c_void, i: i32) -> size_t;
	// operator unsigned long() /usr/include/opencv2/core/mat.hpp:606
	pub fn cv_MatStep_operator_size_t_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// Matx_AddOp() /usr/include/opencv2/core/matx.hpp:68
	pub fn cv_Matx_AddOp_Matx_AddOp(ocvrs_return: *mut Result<*mut c_void>);
	// Matx_AddOp(const cv::Matx_AddOp &) /usr/include/opencv2/core/matx.hpp:68
	pub fn cv_Matx_AddOp_Matx_AddOp_const_Matx_AddOpR(unnamed: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Matx_DivOp() /usr/include/opencv2/core/matx.hpp:72
	pub fn cv_Matx_DivOp_Matx_DivOp(ocvrs_return: *mut Result<*mut c_void>);
	// Matx_DivOp(const cv::Matx_DivOp &) /usr/include/opencv2/core/matx.hpp:72
	pub fn cv_Matx_DivOp_Matx_DivOp_const_Matx_DivOpR(unnamed: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Matx_MatMulOp() /usr/include/opencv2/core/matx.hpp:73
	pub fn cv_Matx_MatMulOp_Matx_MatMulOp(ocvrs_return: *mut Result<*mut c_void>);
	// Matx_MatMulOp(const cv::Matx_MatMulOp &) /usr/include/opencv2/core/matx.hpp:73
	pub fn cv_Matx_MatMulOp_Matx_MatMulOp_const_Matx_MatMulOpR(unnamed: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Matx_MulOp() /usr/include/opencv2/core/matx.hpp:71
	pub fn cv_Matx_MulOp_Matx_MulOp(ocvrs_return: *mut Result<*mut c_void>);
	// Matx_MulOp(const cv::Matx_MulOp &) /usr/include/opencv2/core/matx.hpp:71
	pub fn cv_Matx_MulOp_Matx_MulOp_const_Matx_MulOpR(unnamed: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Matx_ScaleOp() /usr/include/opencv2/core/matx.hpp:70
	pub fn cv_Matx_ScaleOp_Matx_ScaleOp(ocvrs_return: *mut Result<*mut c_void>);
	// Matx_ScaleOp(const cv::Matx_ScaleOp &) /usr/include/opencv2/core/matx.hpp:70
	pub fn cv_Matx_ScaleOp_Matx_ScaleOp_const_Matx_ScaleOpR(unnamed: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Matx_SubOp() /usr/include/opencv2/core/matx.hpp:69
	pub fn cv_Matx_SubOp_Matx_SubOp(ocvrs_return: *mut Result<*mut c_void>);
	// Matx_SubOp(const cv::Matx_SubOp &) /usr/include/opencv2/core/matx.hpp:69
	pub fn cv_Matx_SubOp_Matx_SubOp_const_Matx_SubOpR(unnamed: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Matx_TOp() /usr/include/opencv2/core/matx.hpp:74
	pub fn cv_Matx_TOp_Matx_TOp(ocvrs_return: *mut Result<*mut c_void>);
	// Matx_TOp(const cv::Matx_TOp &) /usr/include/opencv2/core/matx.hpp:74
	pub fn cv_Matx_TOp_Matx_TOp_const_Matx_TOpR(unnamed: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getFunction() /usr/include/opencv2/core/optim.hpp:81
	pub fn cv_MinProblemSolver_getFunction_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setFunction(const Ptr<cv::MinProblemSolver::Function> &) /usr/include/opencv2/core/optim.hpp:89
	pub fn cv_MinProblemSolver_setFunction_const_Ptr_Function_R(instance: *mut c_void, f: *const c_void, ocvrs_return: *mut Result_void);
	// getTermCriteria() /usr/include/opencv2/core/optim.hpp:95
	pub fn cv_MinProblemSolver_getTermCriteria_const(instance: *const c_void, ocvrs_return: *mut Result<core::TermCriteria>);
	// setTermCriteria(const cv::TermCriteria &) /usr/include/opencv2/core/optim.hpp:108
	pub fn cv_MinProblemSolver_setTermCriteria_const_TermCriteriaR(instance: *mut c_void, termcrit: *const core::TermCriteria, ocvrs_return: *mut Result_void);
	// minimize(cv::InputOutputArray) /usr/include/opencv2/core/optim.hpp:122
	pub fn cv_MinProblemSolver_minimize_const__InputOutputArrayR(instance: *mut c_void, x: *const c_void, ocvrs_return: *mut Result<f64>);
	// getDims() /usr/include/opencv2/core/optim.hpp:67
	pub fn cv_MinProblemSolver_Function_getDims_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getGradientEps() /usr/include/opencv2/core/optim.hpp:68
	pub fn cv_MinProblemSolver_Function_getGradientEps_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// calc(const double *) /usr/include/opencv2/core/optim.hpp:69
	pub fn cv_MinProblemSolver_Function_calc_const_const_doubleX(instance: *const c_void, x: *const f64, ocvrs_return: *mut Result<f64>);
	// getGradient(const double *, double *) /usr/include/opencv2/core/optim.hpp:70
	pub fn cv_MinProblemSolver_Function_getGradient_const_doubleX_doubleX(instance: *mut c_void, x: *const f64, grad: *mut f64, ocvrs_return: *mut Result_void);
	// Moments() /usr/include/opencv2/core/types.hpp:952
	pub fn cv_Moments_Moments(ocvrs_return: *mut Result<core::Moments>);
	// Moments(double, double, double, double, double, double, double, double, double, double) /usr/include/opencv2/core/types.hpp:954
	pub fn cv_Moments_Moments_double_double_double_double_double_double_double_double_double_double(m00: f64, m10: f64, m01: f64, m20: f64, m11: f64, m02: f64, m30: f64, m21: f64, m12: f64, m03: f64, ocvrs_return: *mut Result<core::Moments>);
	// eigenvectors /usr/include/opencv2/core.hpp:2588
	pub fn cv_PCA_getPropEigenvectors_const(instance: *const c_void) -> *mut c_void;
	// eigenvectors /usr/include/opencv2/core.hpp:2588
	pub fn cv_PCA_setPropEigenvectors_Mat(instance: *mut c_void, val: *mut c_void);
	// eigenvalues /usr/include/opencv2/core.hpp:2589
	pub fn cv_PCA_getPropEigenvalues_const(instance: *const c_void) -> *mut c_void;
	// eigenvalues /usr/include/opencv2/core.hpp:2589
	pub fn cv_PCA_setPropEigenvalues_Mat(instance: *mut c_void, val: *mut c_void);
	// mean /usr/include/opencv2/core.hpp:2590
	pub fn cv_PCA_getPropMean_const(instance: *const c_void) -> *mut c_void;
	// mean /usr/include/opencv2/core.hpp:2590
	pub fn cv_PCA_setPropMean_Mat(instance: *mut c_void, val: *mut c_void);
	// PCA() /usr/include/opencv2/core.hpp:2462
	pub fn cv_PCA_PCA(ocvrs_return: *mut Result<*mut c_void>);
	// PCA(cv::InputArray, cv::InputArray, int, int) /usr/include/opencv2/core.hpp:2473
	pub fn cv_PCA_PCA_const__InputArrayR_const__InputArrayR_int_int(data: *const c_void, mean: *const c_void, flags: i32, max_components: i32, ocvrs_return: *mut Result<*mut c_void>);
	// PCA(cv::InputArray, cv::InputArray, int, double) /usr/include/opencv2/core.hpp:2485
	pub fn cv_PCA_PCA_const__InputArrayR_const__InputArrayR_int_double(data: *const c_void, mean: *const c_void, flags: i32, retained_variance: f64, ocvrs_return: *mut Result<*mut c_void>);
	// project(cv::InputArray) /usr/include/opencv2/core.hpp:2536
	pub fn cv_PCA_project_const_const__InputArrayR(instance: *const c_void, vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// project(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2550
	pub fn cv_PCA_project_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, vec: *const c_void, result: *const c_void, ocvrs_return: *mut Result_void);
	// backProject(cv::InputArray) /usr/include/opencv2/core.hpp:2565
	pub fn cv_PCA_backProject_const_const__InputArrayR(instance: *const c_void, vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// backProject(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2574
	pub fn cv_PCA_backProject_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, vec: *const c_void, result: *const c_void, ocvrs_return: *mut Result_void);
	// write(cv::FileStorage &) /usr/include/opencv2/core.hpp:2580
	pub fn cv_PCA_write_const_FileStorageR(instance: *const c_void, fs: *mut c_void, ocvrs_return: *mut Result_void);
	// read(const cv::FileNode &) /usr/include/opencv2/core.hpp:2586
	pub fn cv_PCA_read_const_FileNodeR(instance: *mut c_void, fn_: *const c_void, ocvrs_return: *mut Result_void);
	// state /usr/include/opencv2/core.hpp:2972
	pub fn cv_RNG_getPropState_const(instance: *const c_void) -> u64;
	// state /usr/include/opencv2/core.hpp:2972
	pub fn cv_RNG_setPropState_uint64_t(instance: *mut c_void, val: u64);
	// RNG() /usr/include/opencv2/core.hpp:2840
	pub fn cv_RNG_RNG(ocvrs_return: *mut Result<*mut c_void>);
	// RNG(uint64) /usr/include/opencv2/core.hpp:2844
	pub fn cv_RNG_RNG_uint64_t(state: u64, ocvrs_return: *mut Result<*mut c_void>);
	// next() /usr/include/opencv2/core.hpp:2847
	pub fn cv_RNG_next(instance: *mut c_void, ocvrs_return: *mut Result<u32>);
	// operator unsigned char() /usr/include/opencv2/core.hpp:2855
	pub fn cv_RNG_operator_unsigned_char(instance: *mut c_void, ocvrs_return: *mut Result<u8>);
	// operator signed char() /usr/include/opencv2/core.hpp:2857
	pub fn cv_RNG_operator_signed_char(instance: *mut c_void, ocvrs_return: *mut Result<i8>);
	// operator unsigned short() /usr/include/opencv2/core.hpp:2859
	pub fn cv_RNG_operator_unsigned_short(instance: *mut c_void, ocvrs_return: *mut Result<u16>);
	// operator short() /usr/include/opencv2/core.hpp:2861
	pub fn cv_RNG_operator_short(instance: *mut c_void, ocvrs_return: *mut Result<i16>);
	// operator unsigned int() /usr/include/opencv2/core.hpp:2863
	pub fn cv_RNG_operator_unsigned_int(instance: *mut c_void, ocvrs_return: *mut Result<u32>);
	// operator int() /usr/include/opencv2/core.hpp:2865
	pub fn cv_RNG_operator_int(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// operator float() /usr/include/opencv2/core.hpp:2867
	pub fn cv_RNG_operator_float(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
	// operator double() /usr/include/opencv2/core.hpp:2869
	pub fn cv_RNG_operator_double(instance: *mut c_void, ocvrs_return: *mut Result<f64>);
	// uniform(int, int) /usr/include/opencv2/core.hpp:2920
	pub fn cv_RNG_uniform_int_int(instance: *mut c_void, a: i32, b: i32, ocvrs_return: *mut Result<i32>);
	// uniform(float, float) /usr/include/opencv2/core.hpp:2922
	pub fn cv_RNG_uniform_float_float(instance: *mut c_void, a: f32, b: f32, ocvrs_return: *mut Result<f32>);
	// uniform(double, double) /usr/include/opencv2/core.hpp:2924
	pub fn cv_RNG_uniform_double_double(instance: *mut c_void, a: f64, b: f64, ocvrs_return: *mut Result<f64>);
	// fill(cv::InputOutputArray, int, cv::InputArray, cv::InputArray, bool) /usr/include/opencv2/core.hpp:2960
	pub fn cv_RNG_fill_const__InputOutputArrayR_int_const__InputArrayR_const__InputArrayR_bool(instance: *mut c_void, mat: *const c_void, dist_type: i32, a: *const c_void, b: *const c_void, saturate_range: bool, ocvrs_return: *mut Result_void);
	// gaussian(double) /usr/include/opencv2/core.hpp:2970
	pub fn cv_RNG_gaussian_double(instance: *mut c_void, sigma: f64, ocvrs_return: *mut Result<f64>);
	// operator==(const cv::RNG &) /usr/include/opencv2/core.hpp:2974
	pub fn cv_RNG_operatorEQ_const_const_RNGR(instance: *const c_void, other: *const c_void, ocvrs_return: *mut Result<bool>);
	// RNG_MT19937() /usr/include/opencv2/core.hpp:2985
	pub fn cv_RNG_MT19937_RNG_MT19937(ocvrs_return: *mut Result<*mut c_void>);
	// RNG_MT19937(unsigned int) /usr/include/opencv2/core.hpp:2986
	pub fn cv_RNG_MT19937_RNG_MT19937_unsigned_int(s: u32, ocvrs_return: *mut Result<*mut c_void>);
	// seed(unsigned int) /usr/include/opencv2/core.hpp:2987
	pub fn cv_RNG_MT19937_seed_unsigned_int(instance: *mut c_void, s: u32, ocvrs_return: *mut Result_void);
	// next() /usr/include/opencv2/core.hpp:2989
	pub fn cv_RNG_MT19937_next(instance: *mut c_void, ocvrs_return: *mut Result<u32>);
	// operator int() /usr/include/opencv2/core.hpp:2991
	pub fn cv_RNG_MT19937_operator_int(instance: *mut c_void, ocvrs_return: *mut Result<i32>);
	// operator unsigned int() /usr/include/opencv2/core.hpp:2992
	pub fn cv_RNG_MT19937_operator_unsigned_int(instance: *mut c_void, ocvrs_return: *mut Result<u32>);
	// operator float() /usr/include/opencv2/core.hpp:2993
	pub fn cv_RNG_MT19937_operator_float(instance: *mut c_void, ocvrs_return: *mut Result<f32>);
	// operator double() /usr/include/opencv2/core.hpp:2994
	pub fn cv_RNG_MT19937_operator_double(instance: *mut c_void, ocvrs_return: *mut Result<f64>);
	// uniform(int, int) /usr/include/opencv2/core.hpp:3000
	pub fn cv_RNG_MT19937_uniform_int_int(instance: *mut c_void, a: i32, b: i32, ocvrs_return: *mut Result<i32>);
	// uniform(float, float) /usr/include/opencv2/core.hpp:3002
	pub fn cv_RNG_MT19937_uniform_float_float(instance: *mut c_void, a: f32, b: f32, ocvrs_return: *mut Result<f32>);
	// uniform(double, double) /usr/include/opencv2/core.hpp:3004
	pub fn cv_RNG_MT19937_uniform_double_double(instance: *mut c_void, a: f64, b: f64, ocvrs_return: *mut Result<f64>);
	// start /usr/include/opencv2/core/types.hpp:620
	pub fn cv_Range_getPropStart_const(instance: *const c_void) -> i32;
	// start /usr/include/opencv2/core/types.hpp:620
	pub fn cv_Range_setPropStart_int(instance: *mut c_void, val: i32);
	// end /usr/include/opencv2/core/types.hpp:620
	pub fn cv_Range_getPropEnd_const(instance: *const c_void) -> i32;
	// end /usr/include/opencv2/core/types.hpp:620
	pub fn cv_Range_setPropEnd_int(instance: *mut c_void, val: i32);
	// Range() /usr/include/opencv2/core/types.hpp:614
	pub fn cv_Range_Range(ocvrs_return: *mut Result<*mut c_void>);
	// Range(int, int) /usr/include/opencv2/core/types.hpp:615
	pub fn cv_Range_Range_int_int(_start: i32, _end: i32, ocvrs_return: *mut Result<*mut c_void>);
	// size() /usr/include/opencv2/core/types.hpp:616
	pub fn cv_Range_size_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// empty() /usr/include/opencv2/core/types.hpp:617
	pub fn cv_Range_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// all() /usr/include/opencv2/core/types.hpp:618
	pub fn cv_Range_all(ocvrs_return: *mut Result<*mut c_void>);
	// center /usr/include/opencv2/core/types.hpp:552
	pub fn cv_RotatedRect_getPropCenter_const(instance: *const c_void, ocvrs_return: *mut core::Point2f);
	// center /usr/include/opencv2/core/types.hpp:552
	pub fn cv_RotatedRect_setPropCenter_Point2f(instance: *mut c_void, val: *const core::Point2f);
	// size /usr/include/opencv2/core/types.hpp:554
	pub fn cv_RotatedRect_getPropSize_const(instance: *const c_void, ocvrs_return: *mut core::Size2f);
	// size /usr/include/opencv2/core/types.hpp:554
	pub fn cv_RotatedRect_setPropSize_Size2f(instance: *mut c_void, val: *const core::Size2f);
	// angle /usr/include/opencv2/core/types.hpp:556
	pub fn cv_RotatedRect_getPropAngle_const(instance: *const c_void) -> f32;
	// angle /usr/include/opencv2/core/types.hpp:556
	pub fn cv_RotatedRect_setPropAngle_float(instance: *mut c_void, val: f32);
	// RotatedRect() /usr/include/opencv2/core/types.hpp:529
	pub fn cv_RotatedRect_RotatedRect(ocvrs_return: *mut Result<*mut c_void>);
	// RotatedRect(const cv::Point2f &, const cv::Size2f &, float) /usr/include/opencv2/core/types.hpp:536
	pub fn cv_RotatedRect_RotatedRect_const_Point2fR_const_Size2fR_float(center: *const core::Point2f, size: *const core::Size2f, angle: f32, ocvrs_return: *mut Result<*mut c_void>);
	// RotatedRect(const cv::Point2f &, const cv::Point2f &, const cv::Point2f &) /usr/include/opencv2/core/types.hpp:541
	pub fn cv_RotatedRect_RotatedRect_const_Point2fR_const_Point2fR_const_Point2fR(point1: *const core::Point2f, point2: *const core::Point2f, point3: *const core::Point2f, ocvrs_return: *mut Result<*mut c_void>);
	// points(cv::Point2f *) /usr/include/opencv2/core/types.hpp:546
	pub fn cv_RotatedRect_points_const_Point2fX(instance: *const c_void, pts: *mut core::Point2f, ocvrs_return: *mut Result_void);
	// boundingRect() /usr/include/opencv2/core/types.hpp:548
	pub fn cv_RotatedRect_boundingRect_const(instance: *const c_void, ocvrs_return: *mut Result<core::Rect>);
	// boundingRect2f() /usr/include/opencv2/core/types.hpp:550
	pub fn cv_RotatedRect_boundingRect2f_const(instance: *const c_void, ocvrs_return: *mut Result<core::Rect_<f32>>);
	// u /usr/include/opencv2/core.hpp:2810
	pub fn cv_SVD_getPropU_const(instance: *const c_void) -> *mut c_void;
	// u /usr/include/opencv2/core.hpp:2810
	pub fn cv_SVD_setPropU_Mat(instance: *mut c_void, val: *mut c_void);
	// w /usr/include/opencv2/core.hpp:2810
	pub fn cv_SVD_getPropW_const(instance: *const c_void) -> *mut c_void;
	// w /usr/include/opencv2/core.hpp:2810
	pub fn cv_SVD_setPropW_Mat(instance: *mut c_void, val: *mut c_void);
	// vt /usr/include/opencv2/core.hpp:2810
	pub fn cv_SVD_getPropVt_const(instance: *const c_void) -> *mut c_void;
	// vt /usr/include/opencv2/core.hpp:2810
	pub fn cv_SVD_setPropVt_Mat(instance: *mut c_void, val: *mut c_void);
	// SVD() /usr/include/opencv2/core.hpp:2706
	pub fn cv_SVD_SVD(ocvrs_return: *mut Result<*mut c_void>);
	// SVD(cv::InputArray, int) /usr/include/opencv2/core.hpp:2713
	pub fn cv_SVD_SVD_const__InputArrayR_int(src: *const c_void, flags: i32, ocvrs_return: *mut Result<*mut c_void>);
	// compute(cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2745
	pub fn cv_SVD_compute_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(src: *const c_void, w: *const c_void, u: *const c_void, vt: *const c_void, flags: i32, ocvrs_return: *mut Result_void);
	// compute(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2754
	pub fn cv_SVD_compute_const__InputArrayR_const__OutputArrayR_int(src: *const c_void, w: *const c_void, flags: i32, ocvrs_return: *mut Result_void);
	// backSubst(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2758
	pub fn cv_SVD_backSubst_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(w: *const c_void, u: *const c_void, vt: *const c_void, rhs: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// solveZ(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2772
	pub fn cv_SVD_solveZ_const__InputArrayR_const__OutputArrayR(src: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// backSubst(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2796
	pub fn cv_SVD_backSubst_const_const__InputArrayR_const__OutputArrayR(instance: *const c_void, rhs: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// flags /usr/include/opencv2/core/mat.hpp:2971
	pub fn cv_SparseMat_getPropFlags_const(instance: *const c_void) -> i32;
	// flags /usr/include/opencv2/core/mat.hpp:2971
	pub fn cv_SparseMat_setPropFlags_int(instance: *mut c_void, val: i32);
	// hdr /usr/include/opencv2/core/mat.hpp:2972
	pub fn cv_SparseMat_getPropHdr(instance: *mut c_void) -> *mut c_void;
	// hdr /usr/include/opencv2/core/mat.hpp:2972
	pub fn cv_SparseMat_setPropHdr_HdrX(instance: *mut c_void, val: *mut c_void);
	// SparseMat() /usr/include/opencv2/core/mat.hpp:2749
	pub fn cv_SparseMat_SparseMat(ocvrs_return: *mut Result<*mut c_void>);
	// SparseMat(int, const int *, int) /usr/include/opencv2/core/mat.hpp:2756
	pub fn cv_SparseMat_SparseMat_int_const_intX_int(dims: i32, _sizes: *const i32, _type: i32, ocvrs_return: *mut Result<*mut c_void>);
	// SparseMat(const cv::SparseMat &) /usr/include/opencv2/core/mat.hpp:2762
	pub fn cv_SparseMat_SparseMat_const_SparseMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// SparseMat(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:2768
	pub fn cv_SparseMat_SparseMat_const_MatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// clone() /usr/include/opencv2/core/mat.hpp:2779
	pub fn cv_SparseMat_clone_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// copyTo(cv::SparseMat &) /usr/include/opencv2/core/mat.hpp:2782
	pub fn cv_SparseMat_copyTo_const_SparseMatR(instance: *const c_void, m: *mut c_void, ocvrs_return: *mut Result_void);
	// copyTo(cv::Mat &) /usr/include/opencv2/core/mat.hpp:2784
	pub fn cv_SparseMat_copyTo_const_MatR(instance: *const c_void, m: *mut c_void, ocvrs_return: *mut Result_void);
	// convertTo(cv::SparseMat &, int, double) /usr/include/opencv2/core/mat.hpp:2786
	pub fn cv_SparseMat_convertTo_const_SparseMatR_int_double(instance: *const c_void, m: *mut c_void, rtype: i32, alpha: f64, ocvrs_return: *mut Result_void);
	// convertTo(cv::Mat &, int, double, double) /usr/include/opencv2/core/mat.hpp:2797
	pub fn cv_SparseMat_convertTo_const_MatR_int_double_double(instance: *const c_void, m: *mut c_void, rtype: i32, alpha: f64, beta: f64, ocvrs_return: *mut Result_void);
	// assignTo(cv::SparseMat &, int) /usr/include/opencv2/core/mat.hpp:2800
	pub fn cv_SparseMat_assignTo_const_SparseMatR_int(instance: *const c_void, m: *mut c_void, typ: i32, ocvrs_return: *mut Result_void);
	// create(int, const int *, int) /usr/include/opencv2/core/mat.hpp:2808
	pub fn cv_SparseMat_create_int_const_intX_int(instance: *mut c_void, dims: i32, _sizes: *const i32, _type: i32, ocvrs_return: *mut Result_void);
	// clear() /usr/include/opencv2/core/mat.hpp:2810
	pub fn cv_SparseMat_clear(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// addref() /usr/include/opencv2/core/mat.hpp:2812
	pub fn cv_SparseMat_addref(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// release() /usr/include/opencv2/core/mat.hpp:2814
	pub fn cv_SparseMat_release(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// elemSize() /usr/include/opencv2/core/mat.hpp:2819
	pub fn cv_SparseMat_elemSize_const(instance: *const c_void) -> size_t;
	// elemSize1() /usr/include/opencv2/core/mat.hpp:2821
	pub fn cv_SparseMat_elemSize1_const(instance: *const c_void) -> size_t;
	// type() /usr/include/opencv2/core/mat.hpp:2824
	pub fn cv_SparseMat_type_const(instance: *const c_void) -> i32;
	// depth() /usr/include/opencv2/core/mat.hpp:2826
	pub fn cv_SparseMat_depth_const(instance: *const c_void) -> i32;
	// channels() /usr/include/opencv2/core/mat.hpp:2828
	pub fn cv_SparseMat_channels_const(instance: *const c_void) -> i32;
	// size() /usr/include/opencv2/core/mat.hpp:2831
	pub fn cv_SparseMat_size_const(instance: *const c_void, ocvrs_return: *mut Result<*const i32>);
	// size(int) /usr/include/opencv2/core/mat.hpp:2833
	pub fn cv_SparseMat_size_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<i32>);
	// dims() /usr/include/opencv2/core/mat.hpp:2835
	pub fn cv_SparseMat_dims_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// nzcount() /usr/include/opencv2/core/mat.hpp:2837
	pub fn cv_SparseMat_nzcount_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// hash(int) /usr/include/opencv2/core/mat.hpp:2840
	pub fn cv_SparseMat_hash_const_int(instance: *const c_void, i0: i32, ocvrs_return: *mut Result<size_t>);
	// hash(int, int) /usr/include/opencv2/core/mat.hpp:2842
	pub fn cv_SparseMat_hash_const_int_int(instance: *const c_void, i0: i32, i1: i32, ocvrs_return: *mut Result<size_t>);
	// hash(int, int, int) /usr/include/opencv2/core/mat.hpp:2844
	pub fn cv_SparseMat_hash_const_int_int_int(instance: *const c_void, i0: i32, i1: i32, i2: i32, ocvrs_return: *mut Result<size_t>);
	// hash(const int *) /usr/include/opencv2/core/mat.hpp:2846
	pub fn cv_SparseMat_hash_const_const_intX(instance: *const c_void, idx: *const i32, ocvrs_return: *mut Result<size_t>);
	// ptr(int, bool, size_t *) /usr/include/opencv2/core/mat.hpp:2860
	pub fn cv_SparseMat_ptr_int_bool_size_tX(instance: *mut c_void, i0: i32, create_missing: bool, hashval: *mut size_t, ocvrs_return: *mut Result<*mut u8>);
	// ptr(int, int, bool, size_t *) /usr/include/opencv2/core/mat.hpp:2862
	pub fn cv_SparseMat_ptr_int_int_bool_size_tX(instance: *mut c_void, i0: i32, i1: i32, create_missing: bool, hashval: *mut size_t, ocvrs_return: *mut Result<*mut u8>);
	// ptr(int, int, int, bool, size_t *) /usr/include/opencv2/core/mat.hpp:2864
	pub fn cv_SparseMat_ptr_int_int_int_bool_size_tX(instance: *mut c_void, i0: i32, i1: i32, i2: i32, create_missing: bool, hashval: *mut size_t, ocvrs_return: *mut Result<*mut u8>);
	// ptr(const int *, bool, size_t *) /usr/include/opencv2/core/mat.hpp:2866
	pub fn cv_SparseMat_ptr_const_intX_bool_size_tX(instance: *mut c_void, idx: *const i32, create_missing: bool, hashval: *mut size_t, ocvrs_return: *mut Result<*mut u8>);
	// erase(int, int, size_t *) /usr/include/opencv2/core/mat.hpp:2927
	pub fn cv_SparseMat_erase_int_int_size_tX(instance: *mut c_void, i0: i32, i1: i32, hashval: *mut size_t, ocvrs_return: *mut Result_void);
	// erase(int, int, int, size_t *) /usr/include/opencv2/core/mat.hpp:2929
	pub fn cv_SparseMat_erase_int_int_int_size_tX(instance: *mut c_void, i0: i32, i1: i32, i2: i32, hashval: *mut size_t, ocvrs_return: *mut Result_void);
	// erase(const int *, size_t *) /usr/include/opencv2/core/mat.hpp:2931
	pub fn cv_SparseMat_erase_const_intX_size_tX(instance: *mut c_void, idx: *const i32, hashval: *mut size_t, ocvrs_return: *mut Result_void);
	// begin() /usr/include/opencv2/core/mat.hpp:2938
	pub fn cv_SparseMat_begin(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// begin() /usr/include/opencv2/core/mat.hpp:2942
	pub fn cv_SparseMat_begin_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// end() /usr/include/opencv2/core/mat.hpp:2950
	pub fn cv_SparseMat_end(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// end() /usr/include/opencv2/core/mat.hpp:2952
	pub fn cv_SparseMat_end_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// node(size_t) /usr/include/opencv2/core/mat.hpp:2964
	pub fn cv_SparseMat_node_size_t(instance: *mut c_void, nidx: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// node(size_t) /usr/include/opencv2/core/mat.hpp:2965
	pub fn cv_SparseMat_node_const_size_t(instance: *const c_void, nidx: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// newNode(const int *, size_t) /usr/include/opencv2/core/mat.hpp:2967
	pub fn cv_SparseMat_newNode_const_intX_size_t(instance: *mut c_void, idx: *const i32, hashval: size_t, ocvrs_return: *mut Result<*mut u8>);
	// removeNode(size_t, size_t, size_t) /usr/include/opencv2/core/mat.hpp:2968
	pub fn cv_SparseMat_removeNode_size_t_size_t_size_t(instance: *mut c_void, hidx: size_t, nidx: size_t, previdx: size_t, ocvrs_return: *mut Result_void);
	// resizeHashTab(size_t) /usr/include/opencv2/core/mat.hpp:2969
	pub fn cv_SparseMat_resizeHashTab_size_t(instance: *mut c_void, newsize: size_t, ocvrs_return: *mut Result_void);
	// refcount /usr/include/opencv2/core/mat.hpp:2725
	pub fn cv_SparseMat_Hdr_getPropRefcount_const(instance: *const c_void) -> i32;
	// refcount /usr/include/opencv2/core/mat.hpp:2725
	pub fn cv_SparseMat_Hdr_setPropRefcount_int(instance: *mut c_void, val: i32);
	// dims /usr/include/opencv2/core/mat.hpp:2726
	pub fn cv_SparseMat_Hdr_getPropDims_const(instance: *const c_void) -> i32;
	// dims /usr/include/opencv2/core/mat.hpp:2726
	pub fn cv_SparseMat_Hdr_setPropDims_int(instance: *mut c_void, val: i32);
	// valueOffset /usr/include/opencv2/core/mat.hpp:2727
	pub fn cv_SparseMat_Hdr_getPropValueOffset_const(instance: *const c_void) -> i32;
	// valueOffset /usr/include/opencv2/core/mat.hpp:2727
	pub fn cv_SparseMat_Hdr_setPropValueOffset_int(instance: *mut c_void, val: i32);
	// nodeSize /usr/include/opencv2/core/mat.hpp:2728
	pub fn cv_SparseMat_Hdr_getPropNodeSize_const(instance: *const c_void) -> size_t;
	// nodeSize /usr/include/opencv2/core/mat.hpp:2728
	pub fn cv_SparseMat_Hdr_setPropNodeSize_size_t(instance: *mut c_void, val: size_t);
	// nodeCount /usr/include/opencv2/core/mat.hpp:2729
	pub fn cv_SparseMat_Hdr_getPropNodeCount_const(instance: *const c_void) -> size_t;
	// nodeCount /usr/include/opencv2/core/mat.hpp:2729
	pub fn cv_SparseMat_Hdr_setPropNodeCount_size_t(instance: *mut c_void, val: size_t);
	// freeList /usr/include/opencv2/core/mat.hpp:2730
	pub fn cv_SparseMat_Hdr_getPropFreeList_const(instance: *const c_void) -> size_t;
	// freeList /usr/include/opencv2/core/mat.hpp:2730
	pub fn cv_SparseMat_Hdr_setPropFreeList_size_t(instance: *mut c_void, val: size_t);
	// pool /usr/include/opencv2/core/mat.hpp:2731
	pub fn cv_SparseMat_Hdr_getPropPool_const(instance: *const c_void) -> *mut c_void;
	// pool /usr/include/opencv2/core/mat.hpp:2731
	pub fn cv_SparseMat_Hdr_setPropPool_vector_unsigned_char_(instance: *mut c_void, val: *mut c_void);
	// hashtab /usr/include/opencv2/core/mat.hpp:2732
	pub fn cv_SparseMat_Hdr_getPropHashtab_const(instance: *const c_void) -> *mut c_void;
	// hashtab /usr/include/opencv2/core/mat.hpp:2732
	pub fn cv_SparseMat_Hdr_setPropHashtab_vector_size_t_(instance: *mut c_void, val: *mut c_void);
	// size /usr/include/opencv2/core/mat.hpp:2733
	pub fn cv_SparseMat_Hdr_getPropSize(instance: *mut c_void) -> *mut [i32; 32];
	// Hdr(int, const int *, int) /usr/include/opencv2/core/mat.hpp:2723
	pub fn cv_SparseMat_Hdr_Hdr_int_const_intX_int(_dims: i32, _sizes: *const i32, _type: i32, ocvrs_return: *mut Result<*mut c_void>);
	// clear() /usr/include/opencv2/core/mat.hpp:2724
	pub fn cv_SparseMat_Hdr_clear(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// hashval /usr/include/opencv2/core/mat.hpp:2740
	pub fn cv_SparseMat_Node_getPropHashval_const(instance: *const c_void) -> size_t;
	// hashval /usr/include/opencv2/core/mat.hpp:2740
	pub fn cv_SparseMat_Node_setPropHashval_size_t(instance: *mut c_void, val: size_t);
	// next /usr/include/opencv2/core/mat.hpp:2742
	pub fn cv_SparseMat_Node_getPropNext_const(instance: *const c_void) -> size_t;
	// next /usr/include/opencv2/core/mat.hpp:2742
	pub fn cv_SparseMat_Node_setPropNext_size_t(instance: *mut c_void, val: size_t);
	// idx /usr/include/opencv2/core/mat.hpp:2744
	pub fn cv_SparseMat_Node_getPropIdx(instance: *mut c_void) -> *mut [i32; 32];
	// m /usr/include/opencv2/core/mat.hpp:3267
	pub fn cv_SparseMatConstIterator_getPropM_const(instance: *const c_void) -> *mut c_void;
	// hashidx /usr/include/opencv2/core/mat.hpp:3268
	pub fn cv_SparseMatConstIterator_getPropHashidx_const(instance: *const c_void) -> size_t;
	// hashidx /usr/include/opencv2/core/mat.hpp:3268
	pub fn cv_SparseMatConstIterator_setPropHashidx_size_t(instance: *mut c_void, val: size_t);
	// ptr /usr/include/opencv2/core/mat.hpp:3269
	pub fn cv_SparseMatConstIterator_getPropPtr(instance: *mut c_void) -> *mut u8;
	// ptr /usr/include/opencv2/core/mat.hpp:3269
	pub fn cv_SparseMatConstIterator_setPropPtr_unsigned_charX(instance: *mut c_void, val: *mut u8);
	// SparseMatConstIterator() /usr/include/opencv2/core/mat.hpp:3241
	pub fn cv_SparseMatConstIterator_SparseMatConstIterator(ocvrs_return: *mut Result<*mut c_void>);
	// SparseMatConstIterator(const cv::SparseMat *) /usr/include/opencv2/core/mat.hpp:3243
	pub fn cv_SparseMatConstIterator_SparseMatConstIterator_const_SparseMatX(_m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// SparseMatConstIterator(const cv::SparseMatConstIterator &) /usr/include/opencv2/core/mat.hpp:3245
	pub fn cv_SparseMatConstIterator_SparseMatConstIterator_const_SparseMatConstIteratorR(it: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// node() /usr/include/opencv2/core/mat.hpp:3253
	pub fn cv_SparseMatConstIterator_node_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	#[cfg(not(target_os = "windows"))]
	// operator--() /usr/include/opencv2/core/mat.hpp:3256
	pub fn cv_SparseMatConstIterator_operatorSS(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator++() /usr/include/opencv2/core/mat.hpp:3260
	pub fn cv_SparseMatConstIterator_operatorAA(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// seekEnd() /usr/include/opencv2/core/mat.hpp:3265
	pub fn cv_SparseMatConstIterator_seekEnd(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// SparseMatIterator() /usr/include/opencv2/core/mat.hpp:3285
	pub fn cv_SparseMatIterator_SparseMatIterator(ocvrs_return: *mut Result<*mut c_void>);
	// SparseMatIterator(cv::SparseMat *) /usr/include/opencv2/core/mat.hpp:3287
	pub fn cv_SparseMatIterator_SparseMatIterator_SparseMatX(_m: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// SparseMatIterator(const cv::SparseMatIterator &) /usr/include/opencv2/core/mat.hpp:3291
	pub fn cv_SparseMatIterator_SparseMatIterator_const_SparseMatIteratorR(it: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// node() /usr/include/opencv2/core/mat.hpp:3298
	pub fn cv_SparseMatIterator_node_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator++() /usr/include/opencv2/core/mat.hpp:3301
	pub fn cv_SparseMatIterator_operatorAA(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// cleanup() /usr/include/opencv2/core/utils/tls.hpp:49
	pub fn cv_TLSDataContainer_cleanup(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// TermCriteria() /usr/include/opencv2/core/types.hpp:888
	pub fn cv_TermCriteria_TermCriteria(ocvrs_return: *mut Result<core::TermCriteria>);
	// TermCriteria(int, int, double) /usr/include/opencv2/core/types.hpp:894
	pub fn cv_TermCriteria_TermCriteria_int_int_double(typ: i32, max_count: i32, epsilon: f64, ocvrs_return: *mut Result<core::TermCriteria>);
	// isValid() /usr/include/opencv2/core/types.hpp:896
	pub fn cv_TermCriteria_isValid_const(instance: *const core::TermCriteria, ocvrs_return: *mut Result<bool>);
	// TickMeter() /usr/include/opencv2/core/utility.hpp:298
	pub fn cv_TickMeter_TickMeter(ocvrs_return: *mut Result<*mut c_void>);
	// start() /usr/include/opencv2/core/utility.hpp:304
	pub fn cv_TickMeter_start(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// stop() /usr/include/opencv2/core/utility.hpp:310
	pub fn cv_TickMeter_stop(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// getTimeTicks() /usr/include/opencv2/core/utility.hpp:321
	pub fn cv_TickMeter_getTimeTicks_const(instance: *const c_void, ocvrs_return: *mut Result<i64>);
	// getTimeMicro() /usr/include/opencv2/core/utility.hpp:327
	pub fn cv_TickMeter_getTimeMicro_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// getTimeMilli() /usr/include/opencv2/core/utility.hpp:333
	pub fn cv_TickMeter_getTimeMilli_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// getTimeSec() /usr/include/opencv2/core/utility.hpp:339
	pub fn cv_TickMeter_getTimeSec_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// getCounter() /usr/include/opencv2/core/utility.hpp:345
	pub fn cv_TickMeter_getCounter_const(instance: *const c_void, ocvrs_return: *mut Result<i64>);
	// getFPS() /usr/include/opencv2/core/utility.hpp:351
	pub fn cv_TickMeter_getFPS_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// getAvgTimeSec() /usr/include/opencv2/core/utility.hpp:360
	pub fn cv_TickMeter_getAvgTimeSec_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// getAvgTimeMilli() /usr/include/opencv2/core/utility.hpp:368
	pub fn cv_TickMeter_getAvgTimeMilli_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// reset() /usr/include/opencv2/core/utility.hpp:374
	pub fn cv_TickMeter_reset(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// flags /usr/include/opencv2/core/mat.hpp:2592
	pub fn cv_UMat_getPropFlags_const(instance: *const c_void) -> i32;
	// flags /usr/include/opencv2/core/mat.hpp:2592
	pub fn cv_UMat_setPropFlags_int(instance: *mut c_void, val: i32);
	// dims /usr/include/opencv2/core/mat.hpp:2595
	pub fn cv_UMat_getPropDims_const(instance: *const c_void) -> i32;
	// dims /usr/include/opencv2/core/mat.hpp:2595
	pub fn cv_UMat_setPropDims_int(instance: *mut c_void, val: i32);
	// rows /usr/include/opencv2/core/mat.hpp:2598
	pub fn cv_UMat_getPropRows_const(instance: *const c_void) -> i32;
	// rows /usr/include/opencv2/core/mat.hpp:2598
	pub fn cv_UMat_setPropRows_int(instance: *mut c_void, val: i32);
	// cols /usr/include/opencv2/core/mat.hpp:2601
	pub fn cv_UMat_getPropCols_const(instance: *const c_void) -> i32;
	// cols /usr/include/opencv2/core/mat.hpp:2601
	pub fn cv_UMat_setPropCols_int(instance: *mut c_void, val: i32);
	// usageFlags /usr/include/opencv2/core/mat.hpp:2607
	pub fn cv_UMat_getPropUsageFlags_const(instance: *const c_void, ocvrs_return: *mut core::UMatUsageFlags);
	// usageFlags /usr/include/opencv2/core/mat.hpp:2607
	pub fn cv_UMat_setPropUsageFlags_UMatUsageFlags(instance: *mut c_void, val: core::UMatUsageFlags);
	// u /usr/include/opencv2/core/mat.hpp:2616
	pub fn cv_UMat_getPropU(instance: *mut c_void) -> *mut c_void;
	// u /usr/include/opencv2/core/mat.hpp:2616
	pub fn cv_UMat_setPropU_UMatDataX(instance: *mut c_void, val: *mut c_void);
	// offset /usr/include/opencv2/core/mat.hpp:2619
	pub fn cv_UMat_getPropOffset_const(instance: *const c_void) -> size_t;
	// offset /usr/include/opencv2/core/mat.hpp:2619
	pub fn cv_UMat_setPropOffset_size_t(instance: *mut c_void, val: size_t);
	// size /usr/include/opencv2/core/mat.hpp:2622
	pub fn cv_UMat_getPropSize_const(instance: *const c_void) -> *mut c_void;
	// step /usr/include/opencv2/core/mat.hpp:2625
	pub fn cv_UMat_getPropStep_const(instance: *const c_void) -> *mut c_void;
	// UMat(cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2414
	pub fn cv_UMat_UMat_UMatUsageFlags(usage_flags: core::UMatUsageFlags) -> *mut c_void;
	// UMat(int, int, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2417
	pub fn cv_UMat_UMat_int_int_int_UMatUsageFlags(rows: i32, cols: i32, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
	// UMat(cv::Size, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2418
	pub fn cv_UMat_UMat_Size_int_UMatUsageFlags(size: *const core::Size, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
	// UMat(int, int, int, const cv::Scalar &, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2420
	pub fn cv_UMat_UMat_int_int_int_const_ScalarR_UMatUsageFlags(rows: i32, cols: i32, typ: i32, s: *const core::Scalar, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
	// UMat(cv::Size, int, const cv::Scalar &, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2421
	pub fn cv_UMat_UMat_Size_int_const_ScalarR_UMatUsageFlags(size: *const core::Size, typ: i32, s: *const core::Scalar, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
	// UMat(int, const int *, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2424
	pub fn cv_UMat_UMat_int_const_intX_int_UMatUsageFlags(ndims: i32, sizes: *const i32, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
	// UMat(int, const int *, int, const cv::Scalar &, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2425
	pub fn cv_UMat_UMat_int_const_intX_int_const_ScalarR_UMatUsageFlags(ndims: i32, sizes: *const i32, typ: i32, s: *const core::Scalar, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
	// UMat(const cv::UMat &) /usr/include/opencv2/core/mat.hpp:2428
	pub fn cv_UMat_UMat_const_UMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// UMat(const cv::UMat &, const cv::Range &, const cv::Range &) /usr/include/opencv2/core/mat.hpp:2431
	pub fn cv_UMat_UMat_const_UMatR_const_RangeR_const_RangeR(m: *const c_void, row_range: *const c_void, col_range: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// UMat(const cv::UMat &, const cv::Rect &) /usr/include/opencv2/core/mat.hpp:2432
	pub fn cv_UMat_UMat_const_UMatR_const_RectR(m: *const c_void, roi: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
	// UMat(const cv::UMat &, const std::vector<Range> &) /usr/include/opencv2/core/mat.hpp:2434
	pub fn cv_UMat_UMat_const_UMatR_const_vector_Range_R(m: *const c_void, ranges: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getMat(cv::AccessFlag) /usr/include/opencv2/core/mat.hpp:2445
	pub fn cv_UMat_getMat_const_AccessFlag(instance: *const c_void, flags: core::AccessFlag, ocvrs_return: *mut Result<*mut c_void>);
	// row(int) /usr/include/opencv2/core/mat.hpp:2448
	pub fn cv_UMat_row_const_int(instance: *const c_void, y: i32, ocvrs_return: *mut Result<*mut c_void>);
	// col(int) /usr/include/opencv2/core/mat.hpp:2450
	pub fn cv_UMat_col_const_int(instance: *const c_void, x: i32, ocvrs_return: *mut Result<*mut c_void>);
	// rowRange(int, int) /usr/include/opencv2/core/mat.hpp:2452
	pub fn cv_UMat_rowRange_const_int_int(instance: *const c_void, startrow: i32, endrow: i32, ocvrs_return: *mut Result<*mut c_void>);
	// rowRange(const cv::Range &) /usr/include/opencv2/core/mat.hpp:2453
	pub fn cv_UMat_rowRange_const_const_RangeR(instance: *const c_void, r: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// colRange(int, int) /usr/include/opencv2/core/mat.hpp:2455
	pub fn cv_UMat_colRange_const_int_int(instance: *const c_void, startcol: i32, endcol: i32, ocvrs_return: *mut Result<*mut c_void>);
	// colRange(const cv::Range &) /usr/include/opencv2/core/mat.hpp:2456
	pub fn cv_UMat_colRange_const_const_RangeR(instance: *const c_void, r: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// diag(int) /usr/include/opencv2/core/mat.hpp:2461
	pub fn cv_UMat_diag_const_int(instance: *const c_void, d: i32, ocvrs_return: *mut Result<*mut c_void>);
	// diag(const cv::UMat &, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2463
	pub fn cv_UMat_diag_const_UMatR_UMatUsageFlags(d: *const c_void, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
	// diag(const cv::UMat &) /usr/include/opencv2/core/mat.hpp:2464
	pub fn cv_UMat_diag_const_UMatR(d: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// clone() /usr/include/opencv2/core/mat.hpp:2467
	pub fn cv_UMat_clone_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// copyTo(cv::OutputArray) /usr/include/opencv2/core/mat.hpp:2470
	pub fn cv_UMat_copyTo_const_const__OutputArrayR(instance: *const c_void, m: *const c_void, ocvrs_return: *mut Result_void);
	// copyTo(cv::OutputArray, cv::InputArray) /usr/include/opencv2/core/mat.hpp:2472
	pub fn cv_UMat_copyTo_const_const__OutputArrayR_const__InputArrayR(instance: *const c_void, m: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// convertTo(cv::OutputArray, int, double, double) /usr/include/opencv2/core/mat.hpp:2474
	pub fn cv_UMat_convertTo_const_const__OutputArrayR_int_double_double(instance: *const c_void, m: *const c_void, rtype: i32, alpha: f64, beta: f64, ocvrs_return: *mut Result_void);
	// assignTo(cv::UMat &, int) /usr/include/opencv2/core/mat.hpp:2476
	pub fn cv_UMat_assignTo_const_UMatR_int(instance: *const c_void, m: *mut c_void, typ: i32, ocvrs_return: *mut Result_void);
	// setTo(cv::InputArray, cv::InputArray) /usr/include/opencv2/core/mat.hpp:2481
	pub fn cv_UMat_setTo_const__InputArrayR_const__InputArrayR(instance: *mut c_void, value: *const c_void, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// reshape(int, int) /usr/include/opencv2/core/mat.hpp:2484
	pub fn cv_UMat_reshape_const_int_int(instance: *const c_void, cn: i32, rows: i32, ocvrs_return: *mut Result<*mut c_void>);
	// reshape(int, int, const int *) /usr/include/opencv2/core/mat.hpp:2485
	pub fn cv_UMat_reshape_const_int_int_const_intX(instance: *const c_void, cn: i32, newndims: i32, newsz: *const i32, ocvrs_return: *mut Result<*mut c_void>);
	// t() /usr/include/opencv2/core/mat.hpp:2488
	pub fn cv_UMat_t_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// inv(int) /usr/include/opencv2/core/mat.hpp:2490
	pub fn cv_UMat_inv_const_int(instance: *const c_void, method: i32, ocvrs_return: *mut Result<*mut c_void>);
	// mul(cv::InputArray, double) /usr/include/opencv2/core/mat.hpp:2492
	pub fn cv_UMat_mul_const_const__InputArrayR_double(instance: *const c_void, m: *const c_void, scale: f64, ocvrs_return: *mut Result<*mut c_void>);
	// dot(cv::InputArray) /usr/include/opencv2/core/mat.hpp:2495
	pub fn cv_UMat_dot_const_const__InputArrayR(instance: *const c_void, m: *const c_void, ocvrs_return: *mut Result<f64>);
	// zeros(int, int, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2498
	pub fn cv_UMat_zeros_int_int_int_UMatUsageFlags(rows: i32, cols: i32, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
	// zeros(cv::Size, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2499
	pub fn cv_UMat_zeros_Size_int_UMatUsageFlags(size: *const core::Size, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
	// zeros(int, const int *, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2500
	pub fn cv_UMat_zeros_int_const_intX_int_UMatUsageFlags(ndims: i32, sz: *const i32, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
	// zeros(int, int, int) /usr/include/opencv2/core/mat.hpp:2501
	pub fn cv_UMat_zeros_int_int_int(rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// zeros(cv::Size, int) /usr/include/opencv2/core/mat.hpp:2502
	pub fn cv_UMat_zeros_Size_int(size: *const core::Size, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// zeros(int, const int *, int) /usr/include/opencv2/core/mat.hpp:2503
	pub fn cv_UMat_zeros_int_const_intX_int(ndims: i32, sz: *const i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// ones(int, int, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2504
	pub fn cv_UMat_ones_int_int_int_UMatUsageFlags(rows: i32, cols: i32, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
	// ones(cv::Size, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2505
	pub fn cv_UMat_ones_Size_int_UMatUsageFlags(size: *const core::Size, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
	// ones(int, const int *, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2506
	pub fn cv_UMat_ones_int_const_intX_int_UMatUsageFlags(ndims: i32, sz: *const i32, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
	// ones(int, int, int) /usr/include/opencv2/core/mat.hpp:2507
	pub fn cv_UMat_ones_int_int_int(rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// ones(cv::Size, int) /usr/include/opencv2/core/mat.hpp:2508
	pub fn cv_UMat_ones_Size_int(size: *const core::Size, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// ones(int, const int *, int) /usr/include/opencv2/core/mat.hpp:2509
	pub fn cv_UMat_ones_int_const_intX_int(ndims: i32, sz: *const i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// eye(int, int, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2510
	pub fn cv_UMat_eye_int_int_int_UMatUsageFlags(rows: i32, cols: i32, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
	// eye(cv::Size, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2511
	pub fn cv_UMat_eye_Size_int_UMatUsageFlags(size: *const core::Size, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result<*mut c_void>);
	// eye(int, int, int) /usr/include/opencv2/core/mat.hpp:2512
	pub fn cv_UMat_eye_int_int_int(rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// eye(cv::Size, int) /usr/include/opencv2/core/mat.hpp:2513
	pub fn cv_UMat_eye_Size_int(size: *const core::Size, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, int, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2517
	pub fn cv_UMat_create_int_int_int_UMatUsageFlags(instance: *mut c_void, rows: i32, cols: i32, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result_void);
	// create(cv::Size, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2518
	pub fn cv_UMat_create_Size_int_UMatUsageFlags(instance: *mut c_void, size: *const core::Size, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result_void);
	// create(int, const int *, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2519
	pub fn cv_UMat_create_int_const_intX_int_UMatUsageFlags(instance: *mut c_void, ndims: i32, sizes: *const i32, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result_void);
	// create(const std::vector<int> &, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2520
	pub fn cv_UMat_create_const_vector_int_R_int_UMatUsageFlags(instance: *mut c_void, sizes: *const c_void, typ: i32, usage_flags: core::UMatUsageFlags, ocvrs_return: *mut Result_void);
	// addref() /usr/include/opencv2/core/mat.hpp:2523
	pub fn cv_UMat_addref(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// release() /usr/include/opencv2/core/mat.hpp:2526
	pub fn cv_UMat_release(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// deallocate() /usr/include/opencv2/core/mat.hpp:2529
	pub fn cv_UMat_deallocate(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// locateROI(cv::Size &, cv::Point &) /usr/include/opencv2/core/mat.hpp:2534
	pub fn cv_UMat_locateROI_const_SizeR_PointR(instance: *const c_void, whole_size: *mut core::Size, ofs: *mut core::Point, ocvrs_return: *mut Result_void);
	// adjustROI(int, int, int, int) /usr/include/opencv2/core/mat.hpp:2536
	pub fn cv_UMat_adjustROI_int_int_int_int(instance: *mut c_void, dtop: i32, dbottom: i32, dleft: i32, dright: i32, ocvrs_return: *mut Result<*mut c_void>);
	// isContinuous() /usr/include/opencv2/core/mat.hpp:2547
	pub fn cv_UMat_isContinuous_const(instance: *const c_void) -> bool;
	// isSubmatrix() /usr/include/opencv2/core/mat.hpp:2550
	pub fn cv_UMat_isSubmatrix_const(instance: *const c_void) -> bool;
	// elemSize() /usr/include/opencv2/core/mat.hpp:2554
	pub fn cv_UMat_elemSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// elemSize1() /usr/include/opencv2/core/mat.hpp:2556
	pub fn cv_UMat_elemSize1_const(instance: *const c_void) -> size_t;
	// type() /usr/include/opencv2/core/mat.hpp:2558
	pub fn cv_UMat_type_const(instance: *const c_void) -> i32;
	// depth() /usr/include/opencv2/core/mat.hpp:2560
	pub fn cv_UMat_depth_const(instance: *const c_void) -> i32;
	// channels() /usr/include/opencv2/core/mat.hpp:2562
	pub fn cv_UMat_channels_const(instance: *const c_void) -> i32;
	// step1(int) /usr/include/opencv2/core/mat.hpp:2564
	pub fn cv_UMat_step1_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<size_t>);
	// empty() /usr/include/opencv2/core/mat.hpp:2566
	pub fn cv_UMat_empty_const(instance: *const c_void) -> bool;
	// total() /usr/include/opencv2/core/mat.hpp:2568
	pub fn cv_UMat_total_const(instance: *const c_void) -> size_t;
	// checkVector(int, int, bool) /usr/include/opencv2/core/mat.hpp:2571
	pub fn cv_UMat_checkVector_const_int_int_bool(instance: *const c_void, elem_channels: i32, depth: i32, require_continuous: bool, ocvrs_return: *mut Result<i32>);
	// UMat(cv::UMat &&) /usr/include/opencv2/core/mat.hpp:2573
	pub fn cv_UMat_UMat_UMatR(m: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// handle(cv::AccessFlag) /usr/include/opencv2/core/mat.hpp:2580
	pub fn cv_UMat_handle_const_AccessFlag(instance: *const c_void, access_flags: core::AccessFlag, ocvrs_return: *mut Result<*mut c_void>);
	// ndoffset(size_t *) /usr/include/opencv2/core/mat.hpp:2581
	pub fn cv_UMat_ndoffset_const_size_tX(instance: *const c_void, ofs: *mut size_t, ocvrs_return: *mut Result_void);
	// updateContinuityFlag() /usr/include/opencv2/core/mat.hpp:2613
	pub fn cv_UMat_updateContinuityFlag(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// urefcount /usr/include/opencv2/core/mat.hpp:569
	pub fn cv_UMatData_getPropUrefcount_const(instance: *const c_void) -> i32;
	// urefcount /usr/include/opencv2/core/mat.hpp:569
	pub fn cv_UMatData_setPropUrefcount_int(instance: *mut c_void, val: i32);
	// refcount /usr/include/opencv2/core/mat.hpp:570
	pub fn cv_UMatData_getPropRefcount_const(instance: *const c_void) -> i32;
	// refcount /usr/include/opencv2/core/mat.hpp:570
	pub fn cv_UMatData_setPropRefcount_int(instance: *mut c_void, val: i32);
	// data /usr/include/opencv2/core/mat.hpp:571
	pub fn cv_UMatData_getPropData(instance: *mut c_void) -> *mut u8;
	// data /usr/include/opencv2/core/mat.hpp:571
	pub fn cv_UMatData_setPropData_unsigned_charX(instance: *mut c_void, val: *mut u8);
	// origdata /usr/include/opencv2/core/mat.hpp:572
	pub fn cv_UMatData_getPropOrigdata(instance: *mut c_void) -> *mut u8;
	// origdata /usr/include/opencv2/core/mat.hpp:572
	pub fn cv_UMatData_setPropOrigdata_unsigned_charX(instance: *mut c_void, val: *mut u8);
	// size /usr/include/opencv2/core/mat.hpp:573
	pub fn cv_UMatData_getPropSize_const(instance: *const c_void) -> size_t;
	// size /usr/include/opencv2/core/mat.hpp:573
	pub fn cv_UMatData_setPropSize_size_t(instance: *mut c_void, val: size_t);
	// flags /usr/include/opencv2/core/mat.hpp:575
	pub fn cv_UMatData_getPropFlags_const(instance: *const c_void, ocvrs_return: *mut core::UMatData_MemoryFlag);
	// flags /usr/include/opencv2/core/mat.hpp:575
	pub fn cv_UMatData_setPropFlags_MemoryFlag(instance: *mut c_void, val: core::UMatData_MemoryFlag);
	// handle /usr/include/opencv2/core/mat.hpp:576
	pub fn cv_UMatData_getPropHandle(instance: *mut c_void) -> *mut c_void;
	// handle /usr/include/opencv2/core/mat.hpp:576
	pub fn cv_UMatData_setPropHandle_voidX(instance: *mut c_void, val: *mut c_void);
	// userdata /usr/include/opencv2/core/mat.hpp:577
	pub fn cv_UMatData_getPropUserdata(instance: *mut c_void) -> *mut c_void;
	// userdata /usr/include/opencv2/core/mat.hpp:577
	pub fn cv_UMatData_setPropUserdata_voidX(instance: *mut c_void, val: *mut c_void);
	// allocatorFlags_ /usr/include/opencv2/core/mat.hpp:578
	pub fn cv_UMatData_getPropAllocatorFlags__const(instance: *const c_void) -> i32;
	// allocatorFlags_ /usr/include/opencv2/core/mat.hpp:578
	pub fn cv_UMatData_setPropAllocatorFlags__int(instance: *mut c_void, val: i32);
	// mapcount /usr/include/opencv2/core/mat.hpp:579
	pub fn cv_UMatData_getPropMapcount_const(instance: *const c_void) -> i32;
	// mapcount /usr/include/opencv2/core/mat.hpp:579
	pub fn cv_UMatData_setPropMapcount_int(instance: *mut c_void, val: i32);
	// originalUMatData /usr/include/opencv2/core/mat.hpp:580
	pub fn cv_UMatData_getPropOriginalUMatData(instance: *mut c_void) -> *mut c_void;
	// originalUMatData /usr/include/opencv2/core/mat.hpp:580
	pub fn cv_UMatData_setPropOriginalUMatData_UMatDataX(instance: *mut c_void, val: *mut c_void);
	// lock() /usr/include/opencv2/core/mat.hpp:554
	pub fn cv_UMatData_lock(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// unlock() /usr/include/opencv2/core/mat.hpp:555
	pub fn cv_UMatData_unlock(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// hostCopyObsolete() /usr/include/opencv2/core/mat.hpp:557
	pub fn cv_UMatData_hostCopyObsolete_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// deviceCopyObsolete() /usr/include/opencv2/core/mat.hpp:558
	pub fn cv_UMatData_deviceCopyObsolete_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// deviceMemMapped() /usr/include/opencv2/core/mat.hpp:559
	pub fn cv_UMatData_deviceMemMapped_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// copyOnMap() /usr/include/opencv2/core/mat.hpp:560
	pub fn cv_UMatData_copyOnMap_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// tempUMat() /usr/include/opencv2/core/mat.hpp:561
	pub fn cv_UMatData_tempUMat_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// tempCopiedUMat() /usr/include/opencv2/core/mat.hpp:562
	pub fn cv_UMatData_tempCopiedUMat_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// markHostCopyObsolete(bool) /usr/include/opencv2/core/mat.hpp:563
	pub fn cv_UMatData_markHostCopyObsolete_bool(instance: *mut c_void, flag: bool, ocvrs_return: *mut Result_void);
	// markDeviceCopyObsolete(bool) /usr/include/opencv2/core/mat.hpp:564
	pub fn cv_UMatData_markDeviceCopyObsolete_bool(instance: *mut c_void, flag: bool, ocvrs_return: *mut Result_void);
	// markDeviceMemMapped(bool) /usr/include/opencv2/core/mat.hpp:565
	pub fn cv_UMatData_markDeviceMemMapped_bool(instance: *mut c_void, flag: bool, ocvrs_return: *mut Result_void);
	// _InputArray() /usr/include/opencv2/core/mat.hpp:189
	pub fn cv__InputArray__InputArray(ocvrs_return: *mut Result<*mut c_void>);
	// _InputArray(int, void *) /usr/include/opencv2/core/mat.hpp:190
	pub fn cv__InputArray__InputArray_int_voidX(_flags: i32, _obj: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputArray(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:191
	pub fn cv__InputArray__InputArray_const_MatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputArray(const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:192
	pub fn cv__InputArray__InputArray_const_MatExprR(expr: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputArray(const std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:193
	pub fn cv__InputArray__InputArray_const_vector_Mat_R(vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputArray(const std::vector<bool> &) /usr/include/opencv2/core/mat.hpp:196
	pub fn cv__InputArray__InputArray_const_vector_bool_R(vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputArray(const double &) /usr/include/opencv2/core/mat.hpp:202
	pub fn cv__InputArray__InputArray_const_doubleR(val: *const f64, ocvrs_return: *mut Result<*mut c_void>);
	// _InputArray(const cuda::GpuMat &) /usr/include/opencv2/core/mat.hpp:203
	pub fn cv__InputArray__InputArray_const_GpuMatR(d_mat: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputArray(const std::vector<cuda::GpuMat> &) /usr/include/opencv2/core/mat.hpp:204
	pub fn cv__InputArray__InputArray_const_vector_GpuMat_R(d_mat_array: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputArray(const ogl::Buffer &) /usr/include/opencv2/core/mat.hpp:205
	pub fn cv__InputArray__InputArray_const_BufferR(buf: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputArray(const cuda::HostMem &) /usr/include/opencv2/core/mat.hpp:206
	pub fn cv__InputArray__InputArray_const_HostMemR(cuda_mem: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputArray(const cv::UMat &) /usr/include/opencv2/core/mat.hpp:208
	pub fn cv__InputArray__InputArray_const_UMatR(um: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputArray(const std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:209
	pub fn cv__InputArray__InputArray_const_vector_UMat_R(umv: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getMat(int) /usr/include/opencv2/core/mat.hpp:217
	pub fn cv__InputArray_getMat_const_int(instance: *const c_void, idx: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getMat_(int) /usr/include/opencv2/core/mat.hpp:218
	pub fn cv__InputArray_getMat__const_int(instance: *const c_void, idx: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getUMat(int) /usr/include/opencv2/core/mat.hpp:219
	pub fn cv__InputArray_getUMat_const_int(instance: *const c_void, idx: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getMatVector(std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:220
	pub fn cv__InputArray_getMatVector_const_vector_Mat_R(instance: *const c_void, mv: *mut c_void, ocvrs_return: *mut Result_void);
	// getUMatVector(std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:221
	pub fn cv__InputArray_getUMatVector_const_vector_UMat_R(instance: *const c_void, umv: *mut c_void, ocvrs_return: *mut Result_void);
	// getGpuMatVector(std::vector<cuda::GpuMat> &) /usr/include/opencv2/core/mat.hpp:222
	pub fn cv__InputArray_getGpuMatVector_const_vector_GpuMat_R(instance: *const c_void, gpumv: *mut c_void, ocvrs_return: *mut Result_void);
	// getGpuMat() /usr/include/opencv2/core/mat.hpp:223
	pub fn cv__InputArray_getGpuMat_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getOGlBuffer() /usr/include/opencv2/core/mat.hpp:224
	pub fn cv__InputArray_getOGlBuffer_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getFlags() /usr/include/opencv2/core/mat.hpp:226
	pub fn cv__InputArray_getFlags_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getObj() /usr/include/opencv2/core/mat.hpp:227
	pub fn cv__InputArray_getObj_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getSz() /usr/include/opencv2/core/mat.hpp:228
	pub fn cv__InputArray_getSz_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// kind() /usr/include/opencv2/core/mat.hpp:230
	pub fn cv__InputArray_kind_const(instance: *const c_void, ocvrs_return: *mut Result<core::_InputArray_KindFlag>);
	// dims(int) /usr/include/opencv2/core/mat.hpp:231
	pub fn cv__InputArray_dims_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<i32>);
	// cols(int) /usr/include/opencv2/core/mat.hpp:232
	pub fn cv__InputArray_cols_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<i32>);
	// rows(int) /usr/include/opencv2/core/mat.hpp:233
	pub fn cv__InputArray_rows_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<i32>);
	// size(int) /usr/include/opencv2/core/mat.hpp:234
	pub fn cv__InputArray_size_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<core::Size>);
	// sizend(int *, int) /usr/include/opencv2/core/mat.hpp:235
	pub fn cv__InputArray_sizend_const_intX_int(instance: *const c_void, sz: *mut i32, i: i32, ocvrs_return: *mut Result<i32>);
	// sameSize(const cv::_InputArray &) /usr/include/opencv2/core/mat.hpp:236
	pub fn cv__InputArray_sameSize_const_const__InputArrayR(instance: *const c_void, arr: *const c_void, ocvrs_return: *mut Result<bool>);
	// total(int) /usr/include/opencv2/core/mat.hpp:237
	pub fn cv__InputArray_total_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<size_t>);
	// type(int) /usr/include/opencv2/core/mat.hpp:238
	pub fn cv__InputArray_type_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<i32>);
	// depth(int) /usr/include/opencv2/core/mat.hpp:239
	pub fn cv__InputArray_depth_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<i32>);
	// channels(int) /usr/include/opencv2/core/mat.hpp:240
	pub fn cv__InputArray_channels_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<i32>);
	// isContinuous(int) /usr/include/opencv2/core/mat.hpp:241
	pub fn cv__InputArray_isContinuous_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<bool>);
	// isSubmatrix(int) /usr/include/opencv2/core/mat.hpp:242
	pub fn cv__InputArray_isSubmatrix_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<bool>);
	// empty() /usr/include/opencv2/core/mat.hpp:243
	pub fn cv__InputArray_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// copyTo(const cv::_OutputArray &) /usr/include/opencv2/core/mat.hpp:244
	pub fn cv__InputArray_copyTo_const_const__OutputArrayR(instance: *const c_void, arr: *const c_void, ocvrs_return: *mut Result_void);
	// copyTo(const cv::_OutputArray &, const cv::_InputArray &) /usr/include/opencv2/core/mat.hpp:245
	pub fn cv__InputArray_copyTo_const_const__OutputArrayR_const__InputArrayR(instance: *const c_void, arr: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// offset(int) /usr/include/opencv2/core/mat.hpp:246
	pub fn cv__InputArray_offset_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<size_t>);
	// step(int) /usr/include/opencv2/core/mat.hpp:247
	pub fn cv__InputArray_step_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<size_t>);
	// isMat() /usr/include/opencv2/core/mat.hpp:248
	pub fn cv__InputArray_isMat_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isUMat() /usr/include/opencv2/core/mat.hpp:249
	pub fn cv__InputArray_isUMat_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isMatVector() /usr/include/opencv2/core/mat.hpp:250
	pub fn cv__InputArray_isMatVector_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isUMatVector() /usr/include/opencv2/core/mat.hpp:251
	pub fn cv__InputArray_isUMatVector_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isMatx() /usr/include/opencv2/core/mat.hpp:252
	pub fn cv__InputArray_isMatx_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isVector() /usr/include/opencv2/core/mat.hpp:253
	pub fn cv__InputArray_isVector_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isGpuMat() /usr/include/opencv2/core/mat.hpp:254
	pub fn cv__InputArray_isGpuMat_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isGpuMatVector() /usr/include/opencv2/core/mat.hpp:255
	pub fn cv__InputArray_isGpuMatVector_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// _InputOutputArray() /usr/include/opencv2/core/mat.hpp:388
	pub fn cv__InputOutputArray__InputOutputArray(ocvrs_return: *mut Result<*mut c_void>);
	// _InputOutputArray(int, void *) /usr/include/opencv2/core/mat.hpp:389
	pub fn cv__InputOutputArray__InputOutputArray_int_voidX(_flags: i32, _obj: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputOutputArray(cv::Mat &) /usr/include/opencv2/core/mat.hpp:390
	pub fn cv__InputOutputArray__InputOutputArray_MatR(m: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputOutputArray(std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:391
	pub fn cv__InputOutputArray__InputOutputArray_vector_Mat_R(vec: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputOutputArray(cuda::GpuMat &) /usr/include/opencv2/core/mat.hpp:392
	pub fn cv__InputOutputArray__InputOutputArray_GpuMatR(d_mat: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputOutputArray(ogl::Buffer &) /usr/include/opencv2/core/mat.hpp:393
	pub fn cv__InputOutputArray__InputOutputArray_BufferR(buf: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputOutputArray(cuda::HostMem &) /usr/include/opencv2/core/mat.hpp:394
	pub fn cv__InputOutputArray__InputOutputArray_HostMemR(cuda_mem: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputOutputArray(cv::UMat &) /usr/include/opencv2/core/mat.hpp:403
	pub fn cv__InputOutputArray__InputOutputArray_UMatR(m: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputOutputArray(std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:404
	pub fn cv__InputOutputArray__InputOutputArray_vector_UMat_R(vec: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputOutputArray(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:406
	pub fn cv__InputOutputArray__InputOutputArray_const_MatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputOutputArray(const std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:407
	pub fn cv__InputOutputArray__InputOutputArray_const_vector_Mat_R(vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputOutputArray(const cuda::GpuMat &) /usr/include/opencv2/core/mat.hpp:408
	pub fn cv__InputOutputArray__InputOutputArray_const_GpuMatR(d_mat: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputOutputArray(const std::vector<cuda::GpuMat> &) /usr/include/opencv2/core/mat.hpp:409
	pub fn cv__InputOutputArray__InputOutputArray_const_vector_GpuMat_R(d_mat: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputOutputArray(const ogl::Buffer &) /usr/include/opencv2/core/mat.hpp:410
	pub fn cv__InputOutputArray__InputOutputArray_const_BufferR(buf: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputOutputArray(const cuda::HostMem &) /usr/include/opencv2/core/mat.hpp:411
	pub fn cv__InputOutputArray__InputOutputArray_const_HostMemR(cuda_mem: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputOutputArray(const cv::UMat &) /usr/include/opencv2/core/mat.hpp:419
	pub fn cv__InputOutputArray__InputOutputArray_const_UMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _InputOutputArray(const std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:420
	pub fn cv__InputOutputArray__InputOutputArray_const_vector_UMat_R(vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _OutputArray() /usr/include/opencv2/core/mat.hpp:313
	pub fn cv__OutputArray__OutputArray(ocvrs_return: *mut Result<*mut c_void>);
	// _OutputArray(int, void *) /usr/include/opencv2/core/mat.hpp:314
	pub fn cv__OutputArray__OutputArray_int_voidX(_flags: i32, _obj: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _OutputArray(cv::Mat &) /usr/include/opencv2/core/mat.hpp:315
	pub fn cv__OutputArray__OutputArray_MatR(m: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _OutputArray(std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:316
	pub fn cv__OutputArray__OutputArray_vector_Mat_R(vec: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _OutputArray(cuda::GpuMat &) /usr/include/opencv2/core/mat.hpp:317
	pub fn cv__OutputArray__OutputArray_GpuMatR(d_mat: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _OutputArray(std::vector<cuda::GpuMat> &) /usr/include/opencv2/core/mat.hpp:318
	pub fn cv__OutputArray__OutputArray_vector_GpuMat_R(d_mat: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _OutputArray(ogl::Buffer &) /usr/include/opencv2/core/mat.hpp:319
	pub fn cv__OutputArray__OutputArray_BufferR(buf: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _OutputArray(cuda::HostMem &) /usr/include/opencv2/core/mat.hpp:320
	pub fn cv__OutputArray__OutputArray_HostMemR(cuda_mem: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _OutputArray(cv::UMat &) /usr/include/opencv2/core/mat.hpp:330
	pub fn cv__OutputArray__OutputArray_UMatR(m: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _OutputArray(std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:331
	pub fn cv__OutputArray__OutputArray_vector_UMat_R(vec: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _OutputArray(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:333
	pub fn cv__OutputArray__OutputArray_const_MatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _OutputArray(const std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:334
	pub fn cv__OutputArray__OutputArray_const_vector_Mat_R(vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _OutputArray(const cuda::GpuMat &) /usr/include/opencv2/core/mat.hpp:335
	pub fn cv__OutputArray__OutputArray_const_GpuMatR(d_mat: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _OutputArray(const ogl::Buffer &) /usr/include/opencv2/core/mat.hpp:337
	pub fn cv__OutputArray__OutputArray_const_BufferR(buf: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _OutputArray(const cuda::HostMem &) /usr/include/opencv2/core/mat.hpp:338
	pub fn cv__OutputArray__OutputArray_const_HostMemR(cuda_mem: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _OutputArray(const cv::UMat &) /usr/include/opencv2/core/mat.hpp:346
	pub fn cv__OutputArray__OutputArray_const_UMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// _OutputArray(const std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:347
	pub fn cv__OutputArray__OutputArray_const_vector_UMat_R(vec: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// fixedSize() /usr/include/opencv2/core/mat.hpp:357
	pub fn cv__OutputArray_fixedSize_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// fixedType() /usr/include/opencv2/core/mat.hpp:358
	pub fn cv__OutputArray_fixedType_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// needed() /usr/include/opencv2/core/mat.hpp:359
	pub fn cv__OutputArray_needed_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// getMatRef(int) /usr/include/opencv2/core/mat.hpp:360
	pub fn cv__OutputArray_getMatRef_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getUMatRef(int) /usr/include/opencv2/core/mat.hpp:361
	pub fn cv__OutputArray_getUMatRef_const_int(instance: *const c_void, i: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getGpuMatRef() /usr/include/opencv2/core/mat.hpp:362
	pub fn cv__OutputArray_getGpuMatRef_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getGpuMatVecRef() /usr/include/opencv2/core/mat.hpp:363
	pub fn cv__OutputArray_getGpuMatVecRef_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getOGlBufferRef() /usr/include/opencv2/core/mat.hpp:364
	pub fn cv__OutputArray_getOGlBufferRef_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getHostMemRef() /usr/include/opencv2/core/mat.hpp:365
	pub fn cv__OutputArray_getHostMemRef_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(cv::Size, int, int, bool, _OutputArray::DepthMask) /usr/include/opencv2/core/mat.hpp:366
	pub fn cv__OutputArray_create_const_Size_int_int_bool_DepthMask(instance: *const c_void, sz: *const core::Size, typ: i32, i: i32, allow_transposed: bool, fixed_depth_mask: core::_OutputArray_DepthMask, ocvrs_return: *mut Result_void);
	// create(int, int, int, int, bool, _OutputArray::DepthMask) /usr/include/opencv2/core/mat.hpp:367
	pub fn cv__OutputArray_create_const_int_int_int_int_bool_DepthMask(instance: *const c_void, rows: i32, cols: i32, typ: i32, i: i32, allow_transposed: bool, fixed_depth_mask: core::_OutputArray_DepthMask, ocvrs_return: *mut Result_void);
	// create(int, const int *, int, int, bool, _OutputArray::DepthMask) /usr/include/opencv2/core/mat.hpp:368
	pub fn cv__OutputArray_create_const_int_const_intX_int_int_bool_DepthMask(instance: *const c_void, dims: i32, size: *const i32, typ: i32, i: i32, allow_transposed: bool, fixed_depth_mask: core::_OutputArray_DepthMask, ocvrs_return: *mut Result_void);
	// createSameSize(const cv::_InputArray &, int) /usr/include/opencv2/core/mat.hpp:369
	pub fn cv__OutputArray_createSameSize_const_const__InputArrayR_int(instance: *const c_void, arr: *const c_void, mtype: i32, ocvrs_return: *mut Result_void);
	// release() /usr/include/opencv2/core/mat.hpp:370
	pub fn cv__OutputArray_release_const(instance: *const c_void, ocvrs_return: *mut Result_void);
	// clear() /usr/include/opencv2/core/mat.hpp:371
	pub fn cv__OutputArray_clear_const(instance: *const c_void, ocvrs_return: *mut Result_void);
	// setTo(const cv::_InputArray &, const cv::_InputArray &) /usr/include/opencv2/core/mat.hpp:372
	pub fn cv__OutputArray_setTo_const_const__InputArrayR_const__InputArrayR(instance: *const c_void, value: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// assign(const cv::UMat &) /usr/include/opencv2/core/mat.hpp:374
	pub fn cv__OutputArray_assign_const_const_UMatR(instance: *const c_void, u: *const c_void, ocvrs_return: *mut Result_void);
	// assign(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:375
	pub fn cv__OutputArray_assign_const_const_MatR(instance: *const c_void, m: *const c_void, ocvrs_return: *mut Result_void);
	// assign(const std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:377
	pub fn cv__OutputArray_assign_const_const_vector_UMat_R(instance: *const c_void, v: *const c_void, ocvrs_return: *mut Result_void);
	// assign(const std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:378
	pub fn cv__OutputArray_assign_const_const_vector_Mat_R(instance: *const c_void, v: *const c_void, ocvrs_return: *mut Result_void);
	// move(cv::UMat &) /usr/include/opencv2/core/mat.hpp:380
	pub fn cv__OutputArray_move_const_UMatR(instance: *const c_void, u: *mut c_void, ocvrs_return: *mut Result_void);
	// move(cv::Mat &) /usr/include/opencv2/core/mat.hpp:381
	pub fn cv__OutputArray_move_const_MatR(instance: *const c_void, m: *mut c_void, ocvrs_return: *mut Result_void);
	// BufferPool(cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:692
	pub fn cv_cuda_BufferPool_BufferPool_StreamR(stream: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getBuffer(int, int, int) /usr/include/opencv2/core/cuda.hpp:695
	pub fn cv_cuda_BufferPool_getBuffer_int_int_int(instance: *mut c_void, rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getBuffer(cv::Size, int) /usr/include/opencv2/core/cuda.hpp:698
	pub fn cv_cuda_BufferPool_getBuffer_Size_int(instance: *mut c_void, size: *const core::Size, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// getAllocator() /usr/include/opencv2/core/cuda.hpp:701
	pub fn cv_cuda_BufferPool_getAllocator_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// DeviceInfo() /usr/include/opencv2/core/cuda.hpp:1049
	pub fn cv_cuda_DeviceInfo_DeviceInfo(ocvrs_return: *mut Result<*mut c_void>);
	// DeviceInfo(int) /usr/include/opencv2/core/cuda.hpp:1058
	pub fn cv_cuda_DeviceInfo_DeviceInfo_int(device_id: i32, ocvrs_return: *mut Result<*mut c_void>);
	// deviceID() /usr/include/opencv2/core/cuda.hpp:1062
	pub fn cv_cuda_DeviceInfo_deviceID_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// name() /usr/include/opencv2/core/cuda.hpp:1065
	pub fn cv_cuda_DeviceInfo_name_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// totalGlobalMem() /usr/include/opencv2/core/cuda.hpp:1068
	pub fn cv_cuda_DeviceInfo_totalGlobalMem_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// sharedMemPerBlock() /usr/include/opencv2/core/cuda.hpp:1071
	pub fn cv_cuda_DeviceInfo_sharedMemPerBlock_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// regsPerBlock() /usr/include/opencv2/core/cuda.hpp:1074
	pub fn cv_cuda_DeviceInfo_regsPerBlock_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// warpSize() /usr/include/opencv2/core/cuda.hpp:1077
	pub fn cv_cuda_DeviceInfo_warpSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// memPitch() /usr/include/opencv2/core/cuda.hpp:1080
	pub fn cv_cuda_DeviceInfo_memPitch_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// maxThreadsPerBlock() /usr/include/opencv2/core/cuda.hpp:1083
	pub fn cv_cuda_DeviceInfo_maxThreadsPerBlock_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// maxThreadsDim() /usr/include/opencv2/core/cuda.hpp:1086
	pub fn cv_cuda_DeviceInfo_maxThreadsDim_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec3i>);
	// maxGridSize() /usr/include/opencv2/core/cuda.hpp:1089
	pub fn cv_cuda_DeviceInfo_maxGridSize_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec3i>);
	// clockRate() /usr/include/opencv2/core/cuda.hpp:1092
	pub fn cv_cuda_DeviceInfo_clockRate_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// totalConstMem() /usr/include/opencv2/core/cuda.hpp:1095
	pub fn cv_cuda_DeviceInfo_totalConstMem_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// majorVersion() /usr/include/opencv2/core/cuda.hpp:1098
	pub fn cv_cuda_DeviceInfo_majorVersion_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// minorVersion() /usr/include/opencv2/core/cuda.hpp:1101
	pub fn cv_cuda_DeviceInfo_minorVersion_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// textureAlignment() /usr/include/opencv2/core/cuda.hpp:1104
	pub fn cv_cuda_DeviceInfo_textureAlignment_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// texturePitchAlignment() /usr/include/opencv2/core/cuda.hpp:1107
	pub fn cv_cuda_DeviceInfo_texturePitchAlignment_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// multiProcessorCount() /usr/include/opencv2/core/cuda.hpp:1110
	pub fn cv_cuda_DeviceInfo_multiProcessorCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// kernelExecTimeoutEnabled() /usr/include/opencv2/core/cuda.hpp:1113
	pub fn cv_cuda_DeviceInfo_kernelExecTimeoutEnabled_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// integrated() /usr/include/opencv2/core/cuda.hpp:1116
	pub fn cv_cuda_DeviceInfo_integrated_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// canMapHostMemory() /usr/include/opencv2/core/cuda.hpp:1119
	pub fn cv_cuda_DeviceInfo_canMapHostMemory_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// computeMode() /usr/include/opencv2/core/cuda.hpp:1130
	pub fn cv_cuda_DeviceInfo_computeMode_const(instance: *const c_void, ocvrs_return: *mut Result<core::DeviceInfo_ComputeMode>);
	// maxTexture1D() /usr/include/opencv2/core/cuda.hpp:1133
	pub fn cv_cuda_DeviceInfo_maxTexture1D_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// maxTexture1DMipmap() /usr/include/opencv2/core/cuda.hpp:1136
	pub fn cv_cuda_DeviceInfo_maxTexture1DMipmap_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// maxTexture1DLinear() /usr/include/opencv2/core/cuda.hpp:1139
	pub fn cv_cuda_DeviceInfo_maxTexture1DLinear_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// maxTexture2D() /usr/include/opencv2/core/cuda.hpp:1142
	pub fn cv_cuda_DeviceInfo_maxTexture2D_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2i>);
	// maxTexture2DMipmap() /usr/include/opencv2/core/cuda.hpp:1145
	pub fn cv_cuda_DeviceInfo_maxTexture2DMipmap_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2i>);
	// maxTexture2DLinear() /usr/include/opencv2/core/cuda.hpp:1148
	pub fn cv_cuda_DeviceInfo_maxTexture2DLinear_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec3i>);
	// maxTexture2DGather() /usr/include/opencv2/core/cuda.hpp:1151
	pub fn cv_cuda_DeviceInfo_maxTexture2DGather_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2i>);
	// maxTexture3D() /usr/include/opencv2/core/cuda.hpp:1154
	pub fn cv_cuda_DeviceInfo_maxTexture3D_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec3i>);
	// maxTextureCubemap() /usr/include/opencv2/core/cuda.hpp:1157
	pub fn cv_cuda_DeviceInfo_maxTextureCubemap_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// maxTexture1DLayered() /usr/include/opencv2/core/cuda.hpp:1160
	pub fn cv_cuda_DeviceInfo_maxTexture1DLayered_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2i>);
	// maxTexture2DLayered() /usr/include/opencv2/core/cuda.hpp:1163
	pub fn cv_cuda_DeviceInfo_maxTexture2DLayered_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec3i>);
	// maxTextureCubemapLayered() /usr/include/opencv2/core/cuda.hpp:1166
	pub fn cv_cuda_DeviceInfo_maxTextureCubemapLayered_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2i>);
	// maxSurface1D() /usr/include/opencv2/core/cuda.hpp:1169
	pub fn cv_cuda_DeviceInfo_maxSurface1D_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// maxSurface2D() /usr/include/opencv2/core/cuda.hpp:1172
	pub fn cv_cuda_DeviceInfo_maxSurface2D_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2i>);
	// maxSurface3D() /usr/include/opencv2/core/cuda.hpp:1175
	pub fn cv_cuda_DeviceInfo_maxSurface3D_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec3i>);
	// maxSurface1DLayered() /usr/include/opencv2/core/cuda.hpp:1178
	pub fn cv_cuda_DeviceInfo_maxSurface1DLayered_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2i>);
	// maxSurface2DLayered() /usr/include/opencv2/core/cuda.hpp:1181
	pub fn cv_cuda_DeviceInfo_maxSurface2DLayered_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec3i>);
	// maxSurfaceCubemap() /usr/include/opencv2/core/cuda.hpp:1184
	pub fn cv_cuda_DeviceInfo_maxSurfaceCubemap_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// maxSurfaceCubemapLayered() /usr/include/opencv2/core/cuda.hpp:1187
	pub fn cv_cuda_DeviceInfo_maxSurfaceCubemapLayered_const(instance: *const c_void, ocvrs_return: *mut Result<core::Vec2i>);
	// surfaceAlignment() /usr/include/opencv2/core/cuda.hpp:1190
	pub fn cv_cuda_DeviceInfo_surfaceAlignment_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// concurrentKernels() /usr/include/opencv2/core/cuda.hpp:1193
	pub fn cv_cuda_DeviceInfo_concurrentKernels_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// ECCEnabled() /usr/include/opencv2/core/cuda.hpp:1196
	pub fn cv_cuda_DeviceInfo_ECCEnabled_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// pciBusID() /usr/include/opencv2/core/cuda.hpp:1199
	pub fn cv_cuda_DeviceInfo_pciBusID_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// pciDeviceID() /usr/include/opencv2/core/cuda.hpp:1202
	pub fn cv_cuda_DeviceInfo_pciDeviceID_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// pciDomainID() /usr/include/opencv2/core/cuda.hpp:1205
	pub fn cv_cuda_DeviceInfo_pciDomainID_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// tccDriver() /usr/include/opencv2/core/cuda.hpp:1208
	pub fn cv_cuda_DeviceInfo_tccDriver_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// asyncEngineCount() /usr/include/opencv2/core/cuda.hpp:1211
	pub fn cv_cuda_DeviceInfo_asyncEngineCount_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// unifiedAddressing() /usr/include/opencv2/core/cuda.hpp:1214
	pub fn cv_cuda_DeviceInfo_unifiedAddressing_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// memoryClockRate() /usr/include/opencv2/core/cuda.hpp:1217
	pub fn cv_cuda_DeviceInfo_memoryClockRate_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// memoryBusWidth() /usr/include/opencv2/core/cuda.hpp:1220
	pub fn cv_cuda_DeviceInfo_memoryBusWidth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// l2CacheSize() /usr/include/opencv2/core/cuda.hpp:1223
	pub fn cv_cuda_DeviceInfo_l2CacheSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// maxThreadsPerMultiProcessor() /usr/include/opencv2/core/cuda.hpp:1226
	pub fn cv_cuda_DeviceInfo_maxThreadsPerMultiProcessor_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// queryMemory(size_t &, size_t &) /usr/include/opencv2/core/cuda.hpp:1229
	pub fn cv_cuda_DeviceInfo_queryMemory_const_size_tR_size_tR(instance: *const c_void, total_memory: *mut size_t, free_memory: *mut size_t, ocvrs_return: *mut Result_void);
	// freeMemory() /usr/include/opencv2/core/cuda.hpp:1230
	pub fn cv_cuda_DeviceInfo_freeMemory_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// totalMemory() /usr/include/opencv2/core/cuda.hpp:1231
	pub fn cv_cuda_DeviceInfo_totalMemory_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// supports(cv::cuda::FeatureSet) /usr/include/opencv2/core/cuda.hpp:1239
	pub fn cv_cuda_DeviceInfo_supports_const_FeatureSet(instance: *const c_void, feature_set: core::FeatureSet, ocvrs_return: *mut Result<bool>);
	// isCompatible() /usr/include/opencv2/core/cuda.hpp:1246
	pub fn cv_cuda_DeviceInfo_isCompatible_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// Event(const Event::CreateFlags) /usr/include/opencv2/core/cuda.hpp:927
	pub fn cv_cuda_Event_Event_const_CreateFlags(flags: core::Event_CreateFlags, ocvrs_return: *mut Result<*mut c_void>);
	// record(cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:930
	pub fn cv_cuda_Event_record_StreamR(instance: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result_void);
	// queryIfComplete() /usr/include/opencv2/core/cuda.hpp:933
	pub fn cv_cuda_Event_queryIfComplete_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// waitForCompletion() /usr/include/opencv2/core/cuda.hpp:936
	pub fn cv_cuda_Event_waitForCompletion(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// elapsedTime(const cv::cuda::Event &, const cv::cuda::Event &) /usr/include/opencv2/core/cuda.hpp:939
	pub fn cv_cuda_Event_elapsedTime_const_EventR_const_EventR(start: *const c_void, end: *const c_void, ocvrs_return: *mut Result<f32>);
	// data /usr/include/opencv2/core/cuda.hpp:354
	pub fn cv_cuda_GpuData_getPropData(instance: *mut c_void) -> *mut u8;
	// data /usr/include/opencv2/core/cuda.hpp:354
	pub fn cv_cuda_GpuData_setPropData_unsigned_charX(instance: *mut c_void, val: *mut u8);
	// size /usr/include/opencv2/core/cuda.hpp:355
	pub fn cv_cuda_GpuData_getPropSize_const(instance: *const c_void) -> size_t;
	// size /usr/include/opencv2/core/cuda.hpp:355
	pub fn cv_cuda_GpuData_setPropSize_size_t(instance: *mut c_void, val: size_t);
	// GpuData(size_t) /usr/include/opencv2/core/cuda.hpp:345
	pub fn cv_cuda_GpuData_GpuData_size_t(_size: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// flags /usr/include/opencv2/core/cuda.hpp:320
	pub fn cv_cuda_GpuMat_getPropFlags_const(instance: *const c_void) -> i32;
	// flags /usr/include/opencv2/core/cuda.hpp:320
	pub fn cv_cuda_GpuMat_setPropFlags_int(instance: *mut c_void, val: i32);
	// rows /usr/include/opencv2/core/cuda.hpp:323
	pub fn cv_cuda_GpuMat_getPropRows_const(instance: *const c_void) -> i32;
	// rows /usr/include/opencv2/core/cuda.hpp:323
	pub fn cv_cuda_GpuMat_setPropRows_int(instance: *mut c_void, val: i32);
	// cols /usr/include/opencv2/core/cuda.hpp:323
	pub fn cv_cuda_GpuMat_getPropCols_const(instance: *const c_void) -> i32;
	// cols /usr/include/opencv2/core/cuda.hpp:323
	pub fn cv_cuda_GpuMat_setPropCols_int(instance: *mut c_void, val: i32);
	// step /usr/include/opencv2/core/cuda.hpp:326
	pub fn cv_cuda_GpuMat_getPropStep_const(instance: *const c_void) -> size_t;
	// step /usr/include/opencv2/core/cuda.hpp:326
	pub fn cv_cuda_GpuMat_setPropStep_size_t(instance: *mut c_void, val: size_t);
	// data /usr/include/opencv2/core/cuda.hpp:329
	pub fn cv_cuda_GpuMat_getPropData(instance: *mut c_void) -> *mut u8;
	// data /usr/include/opencv2/core/cuda.hpp:329
	pub fn cv_cuda_GpuMat_setPropData_unsigned_charX(instance: *mut c_void, val: *mut u8);
	// refcount /usr/include/opencv2/core/cuda.hpp:333
	pub fn cv_cuda_GpuMat_getPropRefcount(instance: *mut c_void) -> *mut i32;
	// refcount /usr/include/opencv2/core/cuda.hpp:333
	pub fn cv_cuda_GpuMat_setPropRefcount_intX(instance: *mut c_void, val: *mut i32);
	// datastart /usr/include/opencv2/core/cuda.hpp:336
	pub fn cv_cuda_GpuMat_getPropDatastart(instance: *mut c_void) -> *mut u8;
	// datastart /usr/include/opencv2/core/cuda.hpp:336
	pub fn cv_cuda_GpuMat_setPropDatastart_unsigned_charX(instance: *mut c_void, val: *mut u8);
	// dataend /usr/include/opencv2/core/cuda.hpp:337
	pub fn cv_cuda_GpuMat_getPropDataend_const(instance: *const c_void) -> *const u8;
	// allocator /usr/include/opencv2/core/cuda.hpp:340
	pub fn cv_cuda_GpuMat_getPropAllocator(instance: *mut c_void) -> *mut c_void;
	// allocator /usr/include/opencv2/core/cuda.hpp:340
	pub fn cv_cuda_GpuMat_setPropAllocator_AllocatorX(instance: *mut c_void, val: *mut c_void);
	// defaultAllocator() /usr/include/opencv2/core/cuda.hpp:119
	pub fn cv_cuda_GpuMat_defaultAllocator(ocvrs_return: *mut Result<*mut c_void>);
	// setDefaultAllocator(GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:120
	pub fn cv_cuda_GpuMat_setDefaultAllocator_AllocatorX(allocator: *mut c_void, ocvrs_return: *mut Result_void);
	// GpuMat(GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:123
	pub fn cv_cuda_GpuMat_GpuMat_AllocatorX(allocator: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// GpuMat(int, int, int, GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:126
	pub fn cv_cuda_GpuMat_GpuMat_int_int_int_AllocatorX(rows: i32, cols: i32, typ: i32, allocator: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// GpuMat(cv::Size, int, GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:127
	pub fn cv_cuda_GpuMat_GpuMat_Size_int_AllocatorX(size: *const core::Size, typ: i32, allocator: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// GpuMat(int, int, int, cv::Scalar, GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:130
	pub fn cv_cuda_GpuMat_GpuMat_int_int_int_Scalar_AllocatorX(rows: i32, cols: i32, typ: i32, s: *const core::Scalar, allocator: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// GpuMat(cv::Size, int, cv::Scalar, GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:131
	pub fn cv_cuda_GpuMat_GpuMat_Size_int_Scalar_AllocatorX(size: *const core::Size, typ: i32, s: *const core::Scalar, allocator: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// GpuMat(const cv::cuda::GpuMat &) /usr/include/opencv2/core/cuda.hpp:134
	pub fn cv_cuda_GpuMat_GpuMat_const_GpuMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// GpuMat(int, int, int, void *, size_t) /usr/include/opencv2/core/cuda.hpp:137
	pub fn cv_cuda_GpuMat_GpuMat_int_int_int_voidX_size_t(rows: i32, cols: i32, typ: i32, data: *mut c_void, step: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// GpuMat(cv::Size, int, void *, size_t) /usr/include/opencv2/core/cuda.hpp:138
	pub fn cv_cuda_GpuMat_GpuMat_Size_int_voidX_size_t(size: *const core::Size, typ: i32, data: *mut c_void, step: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// GpuMat(const cv::cuda::GpuMat &, cv::Range, cv::Range) /usr/include/opencv2/core/cuda.hpp:141
	pub fn cv_cuda_GpuMat_GpuMat_const_GpuMatR_Range_Range(m: *const c_void, row_range: *mut c_void, col_range: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// GpuMat(const cv::cuda::GpuMat &, cv::Rect) /usr/include/opencv2/core/cuda.hpp:142
	pub fn cv_cuda_GpuMat_GpuMat_const_GpuMatR_Rect(m: *const c_void, roi: *const core::Rect, ocvrs_return: *mut Result<*mut c_void>);
	// GpuMat(cv::InputArray, GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:145
	pub fn cv_cuda_GpuMat_GpuMat_const__InputArrayR_AllocatorX(arr: *const c_void, allocator: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, int, int) /usr/include/opencv2/core/cuda.hpp:154
	pub fn cv_cuda_GpuMat_create_int_int_int(instance: *mut c_void, rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result_void);
	// create(cv::Size, int) /usr/include/opencv2/core/cuda.hpp:155
	pub fn cv_cuda_GpuMat_create_Size_int(instance: *mut c_void, size: *const core::Size, typ: i32, ocvrs_return: *mut Result_void);
	// release() /usr/include/opencv2/core/cuda.hpp:158
	pub fn cv_cuda_GpuMat_release(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// swap(cv::cuda::GpuMat &) /usr/include/opencv2/core/cuda.hpp:161
	pub fn cv_cuda_GpuMat_swap_GpuMatR(instance: *mut c_void, mat: *mut c_void, ocvrs_return: *mut Result_void);
	// upload(cv::InputArray) /usr/include/opencv2/core/cuda.hpp:168
	pub fn cv_cuda_GpuMat_upload_const__InputArrayR(instance: *mut c_void, arr: *const c_void, ocvrs_return: *mut Result_void);
	// upload(cv::InputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:178
	pub fn cv_cuda_GpuMat_upload_const__InputArrayR_StreamR(instance: *mut c_void, arr: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result_void);
	// download(cv::OutputArray) /usr/include/opencv2/core/cuda.hpp:185
	pub fn cv_cuda_GpuMat_download_const_const__OutputArrayR(instance: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// download(cv::OutputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:195
	pub fn cv_cuda_GpuMat_download_const_const__OutputArrayR_StreamR(instance: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result_void);
	// clone() /usr/include/opencv2/core/cuda.hpp:198
	pub fn cv_cuda_GpuMat_clone_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// copyTo(cv::OutputArray) /usr/include/opencv2/core/cuda.hpp:201
	pub fn cv_cuda_GpuMat_copyTo_const_const__OutputArrayR(instance: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// copyTo(cv::OutputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:204
	pub fn cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_StreamR(instance: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result_void);
	// copyTo(cv::OutputArray, cv::InputArray) /usr/include/opencv2/core/cuda.hpp:207
	pub fn cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_const__InputArrayR(instance: *const c_void, dst: *const c_void, mask: *const c_void, ocvrs_return: *mut Result_void);
	// copyTo(cv::OutputArray, cv::InputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:210
	pub fn cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_const__InputArrayR_StreamR(instance: *const c_void, dst: *const c_void, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result_void);
	// setTo(cv::Scalar) /usr/include/opencv2/core/cuda.hpp:213
	pub fn cv_cuda_GpuMat_setTo_Scalar(instance: *mut c_void, s: *const core::Scalar, ocvrs_return: *mut Result<*mut c_void>);
	// setTo(cv::Scalar, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:216
	pub fn cv_cuda_GpuMat_setTo_Scalar_StreamR(instance: *mut c_void, s: *const core::Scalar, stream: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setTo(cv::Scalar, cv::InputArray) /usr/include/opencv2/core/cuda.hpp:219
	pub fn cv_cuda_GpuMat_setTo_Scalar_const__InputArrayR(instance: *mut c_void, s: *const core::Scalar, mask: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// setTo(cv::Scalar, cv::InputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:222
	pub fn cv_cuda_GpuMat_setTo_Scalar_const__InputArrayR_StreamR(instance: *mut c_void, s: *const core::Scalar, mask: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// convertTo(cv::OutputArray, int) /usr/include/opencv2/core/cuda.hpp:225
	pub fn cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int(instance: *const c_void, dst: *const c_void, rtype: i32, ocvrs_return: *mut Result_void);
	// convertTo(cv::OutputArray, int, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:228
	pub fn cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_StreamR(instance: *const c_void, dst: *const c_void, rtype: i32, stream: *mut c_void, ocvrs_return: *mut Result_void);
	// convertTo(cv::OutputArray, int, double, double) /usr/include/opencv2/core/cuda.hpp:231
	pub fn cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_double(instance: *const c_void, dst: *const c_void, rtype: i32, alpha: f64, beta: f64, ocvrs_return: *mut Result_void);
	// convertTo(cv::OutputArray, int, double, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:234
	pub fn cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_StreamR(instance: *const c_void, dst: *const c_void, rtype: i32, alpha: f64, stream: *mut c_void, ocvrs_return: *mut Result_void);
	// convertTo(cv::OutputArray, int, double, double, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:237
	pub fn cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_double_StreamR(instance: *const c_void, dst: *const c_void, rtype: i32, alpha: f64, beta: f64, stream: *mut c_void, ocvrs_return: *mut Result_void);
	// assignTo(cv::cuda::GpuMat &, int) /usr/include/opencv2/core/cuda.hpp:239
	pub fn cv_cuda_GpuMat_assignTo_const_GpuMatR_int(instance: *const c_void, m: *mut c_void, typ: i32, ocvrs_return: *mut Result_void);
	// ptr(int) /usr/include/opencv2/core/cuda.hpp:242
	pub fn cv_cuda_GpuMat_ptr_int(instance: *mut c_void, y: i32, ocvrs_return: *mut Result<*mut u8>);
	// ptr(int) /usr/include/opencv2/core/cuda.hpp:243
	pub fn cv_cuda_GpuMat_ptr_const_int(instance: *const c_void, y: i32, ocvrs_return: *mut Result<*const u8>);
	// row(int) /usr/include/opencv2/core/cuda.hpp:253
	pub fn cv_cuda_GpuMat_row_const_int(instance: *const c_void, y: i32, ocvrs_return: *mut Result<*mut c_void>);
	// col(int) /usr/include/opencv2/core/cuda.hpp:256
	pub fn cv_cuda_GpuMat_col_const_int(instance: *const c_void, x: i32, ocvrs_return: *mut Result<*mut c_void>);
	// rowRange(int, int) /usr/include/opencv2/core/cuda.hpp:259
	pub fn cv_cuda_GpuMat_rowRange_const_int_int(instance: *const c_void, startrow: i32, endrow: i32, ocvrs_return: *mut Result<*mut c_void>);
	// rowRange(cv::Range) /usr/include/opencv2/core/cuda.hpp:260
	pub fn cv_cuda_GpuMat_rowRange_const_Range(instance: *const c_void, r: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// colRange(int, int) /usr/include/opencv2/core/cuda.hpp:263
	pub fn cv_cuda_GpuMat_colRange_const_int_int(instance: *const c_void, startcol: i32, endcol: i32, ocvrs_return: *mut Result<*mut c_void>);
	// colRange(cv::Range) /usr/include/opencv2/core/cuda.hpp:264
	pub fn cv_cuda_GpuMat_colRange_const_Range(instance: *const c_void, r: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// reshape(int, int) /usr/include/opencv2/core/cuda.hpp:272
	pub fn cv_cuda_GpuMat_reshape_const_int_int(instance: *const c_void, cn: i32, rows: i32, ocvrs_return: *mut Result<*mut c_void>);
	// locateROI(cv::Size &, cv::Point &) /usr/include/opencv2/core/cuda.hpp:275
	pub fn cv_cuda_GpuMat_locateROI_const_SizeR_PointR(instance: *const c_void, whole_size: *mut core::Size, ofs: *mut core::Point, ocvrs_return: *mut Result_void);
	// adjustROI(int, int, int, int) /usr/include/opencv2/core/cuda.hpp:278
	pub fn cv_cuda_GpuMat_adjustROI_int_int_int_int(instance: *mut c_void, dtop: i32, dbottom: i32, dleft: i32, dright: i32, ocvrs_return: *mut Result<*mut c_void>);
	// isContinuous() /usr/include/opencv2/core/cuda.hpp:282
	pub fn cv_cuda_GpuMat_isContinuous_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// elemSize() /usr/include/opencv2/core/cuda.hpp:285
	pub fn cv_cuda_GpuMat_elemSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// elemSize1() /usr/include/opencv2/core/cuda.hpp:288
	pub fn cv_cuda_GpuMat_elemSize1_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// type() /usr/include/opencv2/core/cuda.hpp:291
	pub fn cv_cuda_GpuMat_type_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// depth() /usr/include/opencv2/core/cuda.hpp:294
	pub fn cv_cuda_GpuMat_depth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// channels() /usr/include/opencv2/core/cuda.hpp:297
	pub fn cv_cuda_GpuMat_channels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// step1() /usr/include/opencv2/core/cuda.hpp:300
	pub fn cv_cuda_GpuMat_step1_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// size() /usr/include/opencv2/core/cuda.hpp:303
	pub fn cv_cuda_GpuMat_size_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// empty() /usr/include/opencv2/core/cuda.hpp:306
	pub fn cv_cuda_GpuMat_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// cudaPtr() /usr/include/opencv2/core/cuda.hpp:309
	pub fn cv_cuda_GpuMat_cudaPtr_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// updateContinuityFlag() /usr/include/opencv2/core/cuda.hpp:312
	pub fn cv_cuda_GpuMat_updateContinuityFlag(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// allocate(cv::cuda::GpuMat *, int, int, size_t) /usr/include/opencv2/core/cuda.hpp:114
	pub fn cv_cuda_GpuMat_Allocator_allocate_GpuMatX_int_int_size_t(instance: *mut c_void, mat: *mut c_void, rows: i32, cols: i32, elem_size: size_t, ocvrs_return: *mut Result<bool>);
	// free(cv::cuda::GpuMat *) /usr/include/opencv2/core/cuda.hpp:115
	pub fn cv_cuda_GpuMat_Allocator_free_GpuMatX(instance: *mut c_void, mat: *mut c_void, ocvrs_return: *mut Result_void);
	// flags /usr/include/opencv2/core/cuda.hpp:513
	pub fn cv_cuda_GpuMatND_getPropFlags_const(instance: *const c_void) -> i32;
	// flags /usr/include/opencv2/core/cuda.hpp:513
	pub fn cv_cuda_GpuMatND_setPropFlags_int(instance: *mut c_void, val: i32);
	// dims /usr/include/opencv2/core/cuda.hpp:516
	pub fn cv_cuda_GpuMatND_getPropDims_const(instance: *const c_void) -> i32;
	// dims /usr/include/opencv2/core/cuda.hpp:516
	pub fn cv_cuda_GpuMatND_setPropDims_int(instance: *mut c_void, val: i32);
	// size /usr/include/opencv2/core/cuda.hpp:519
	pub fn cv_cuda_GpuMatND_getPropSize_const(instance: *const c_void) -> *mut c_void;
	// size /usr/include/opencv2/core/cuda.hpp:519
	pub fn cv_cuda_GpuMatND_setPropSize_SizeArray(instance: *mut c_void, val: *mut c_void);
	// step /usr/include/opencv2/core/cuda.hpp:524
	pub fn cv_cuda_GpuMatND_getPropStep_const(instance: *const c_void) -> *mut c_void;
	// step /usr/include/opencv2/core/cuda.hpp:524
	pub fn cv_cuda_GpuMatND_setPropStep_StepArray(instance: *mut c_void, val: *mut c_void);
	// GpuMatND() /usr/include/opencv2/core/cuda.hpp:369
	pub fn cv_cuda_GpuMatND_GpuMatND(ocvrs_return: *mut Result<*mut c_void>);
	// GpuMatND(cv::cuda::GpuMatND::SizeArray, int) /usr/include/opencv2/core/cuda.hpp:376
	pub fn cv_cuda_GpuMatND_GpuMatND_SizeArray_int(size: *mut c_void, typ: i32, ocvrs_return: *mut Result<*mut c_void>);
	// GpuMatND(cv::cuda::GpuMatND::SizeArray, int, void *, cv::cuda::GpuMatND::StepArray) /usr/include/opencv2/core/cuda.hpp:390
	pub fn cv_cuda_GpuMatND_GpuMatND_SizeArray_int_voidX_StepArray(size: *mut c_void, typ: i32, data: *mut c_void, step: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(cv::cuda::GpuMatND::SizeArray, int) /usr/include/opencv2/core/cuda.hpp:399
	pub fn cv_cuda_GpuMatND_create_SizeArray_int(instance: *mut c_void, size: *mut c_void, typ: i32, ocvrs_return: *mut Result_void);
	// release() /usr/include/opencv2/core/cuda.hpp:401
	pub fn cv_cuda_GpuMatND_release(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// swap(cv::cuda::GpuMatND &) /usr/include/opencv2/core/cuda.hpp:403
	pub fn cv_cuda_GpuMatND_swap_GpuMatNDR(instance: *mut c_void, m: *mut c_void);
	// clone() /usr/include/opencv2/core/cuda.hpp:410
	pub fn cv_cuda_GpuMatND_clone_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// clone(cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:415
	pub fn cv_cuda_GpuMatND_clone_const_StreamR(instance: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// createGpuMatHeader(cv::cuda::GpuMatND::IndexArray, cv::Range, cv::Range) /usr/include/opencv2/core/cuda.hpp:429
	pub fn cv_cuda_GpuMatND_createGpuMatHeader_const_IndexArray_Range_Range(instance: *const c_void, idx: *mut c_void, row_range: *mut c_void, col_range: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// createGpuMatHeader() /usr/include/opencv2/core/cuda.hpp:437
	pub fn cv_cuda_GpuMatND_createGpuMatHeader_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// operator GpuMat() /usr/include/opencv2/core/cuda.hpp:450
	pub fn cv_cuda_GpuMatND_operator_cv_cuda_GpuMat_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// GpuMatND(const cv::cuda::GpuMatND &) /usr/include/opencv2/core/cuda.hpp:452
	pub fn cv_cuda_GpuMatND_GpuMatND_const_GpuMatNDR(unnamed: *const c_void) -> *mut c_void;
	// GpuMatND(cv::cuda::GpuMatND &&) /usr/include/opencv2/core/cuda.hpp:459
	pub fn cv_cuda_GpuMatND_GpuMatND_GpuMatNDR(unnamed: *mut c_void) -> *mut c_void;
	// upload(cv::InputArray) /usr/include/opencv2/core/cuda.hpp:466
	pub fn cv_cuda_GpuMatND_upload_const__InputArrayR(instance: *mut c_void, src: *const c_void, ocvrs_return: *mut Result_void);
	// upload(cv::InputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:467
	pub fn cv_cuda_GpuMatND_upload_const__InputArrayR_StreamR(instance: *mut c_void, src: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result_void);
	// download(cv::OutputArray) /usr/include/opencv2/core/cuda.hpp:468
	pub fn cv_cuda_GpuMatND_download_const_const__OutputArrayR(instance: *const c_void, dst: *const c_void, ocvrs_return: *mut Result_void);
	// download(cv::OutputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:469
	pub fn cv_cuda_GpuMatND_download_const_const__OutputArrayR_StreamR(instance: *const c_void, dst: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result_void);
	// isContinuous() /usr/include/opencv2/core/cuda.hpp:473
	pub fn cv_cuda_GpuMatND_isContinuous_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isSubmatrix() /usr/include/opencv2/core/cuda.hpp:476
	pub fn cv_cuda_GpuMatND_isSubmatrix_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// elemSize() /usr/include/opencv2/core/cuda.hpp:479
	pub fn cv_cuda_GpuMatND_elemSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// elemSize1() /usr/include/opencv2/core/cuda.hpp:482
	pub fn cv_cuda_GpuMatND_elemSize1_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// empty() /usr/include/opencv2/core/cuda.hpp:485
	pub fn cv_cuda_GpuMatND_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// external() /usr/include/opencv2/core/cuda.hpp:488
	pub fn cv_cuda_GpuMatND_external_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// getDevicePtr() /usr/include/opencv2/core/cuda.hpp:491
	pub fn cv_cuda_GpuMatND_getDevicePtr_const(instance: *const c_void, ocvrs_return: *mut Result<*mut u8>);
	// total() /usr/include/opencv2/core/cuda.hpp:494
	pub fn cv_cuda_GpuMatND_total_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// totalMemSize() /usr/include/opencv2/core/cuda.hpp:497
	pub fn cv_cuda_GpuMatND_totalMemSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// type() /usr/include/opencv2/core/cuda.hpp:500
	pub fn cv_cuda_GpuMatND_type_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// flags /usr/include/opencv2/core/cuda.hpp:792
	pub fn cv_cuda_HostMem_getPropFlags_const(instance: *const c_void) -> i32;
	// flags /usr/include/opencv2/core/cuda.hpp:792
	pub fn cv_cuda_HostMem_setPropFlags_int(instance: *mut c_void, val: i32);
	// rows /usr/include/opencv2/core/cuda.hpp:793
	pub fn cv_cuda_HostMem_getPropRows_const(instance: *const c_void) -> i32;
	// rows /usr/include/opencv2/core/cuda.hpp:793
	pub fn cv_cuda_HostMem_setPropRows_int(instance: *mut c_void, val: i32);
	// cols /usr/include/opencv2/core/cuda.hpp:793
	pub fn cv_cuda_HostMem_getPropCols_const(instance: *const c_void) -> i32;
	// cols /usr/include/opencv2/core/cuda.hpp:793
	pub fn cv_cuda_HostMem_setPropCols_int(instance: *mut c_void, val: i32);
	// step /usr/include/opencv2/core/cuda.hpp:794
	pub fn cv_cuda_HostMem_getPropStep_const(instance: *const c_void) -> size_t;
	// step /usr/include/opencv2/core/cuda.hpp:794
	pub fn cv_cuda_HostMem_setPropStep_size_t(instance: *mut c_void, val: size_t);
	// data /usr/include/opencv2/core/cuda.hpp:796
	pub fn cv_cuda_HostMem_getPropData(instance: *mut c_void) -> *mut u8;
	// data /usr/include/opencv2/core/cuda.hpp:796
	pub fn cv_cuda_HostMem_setPropData_unsigned_charX(instance: *mut c_void, val: *mut u8);
	// refcount /usr/include/opencv2/core/cuda.hpp:797
	pub fn cv_cuda_HostMem_getPropRefcount(instance: *mut c_void) -> *mut i32;
	// refcount /usr/include/opencv2/core/cuda.hpp:797
	pub fn cv_cuda_HostMem_setPropRefcount_intX(instance: *mut c_void, val: *mut i32);
	// datastart /usr/include/opencv2/core/cuda.hpp:799
	pub fn cv_cuda_HostMem_getPropDatastart(instance: *mut c_void) -> *mut u8;
	// datastart /usr/include/opencv2/core/cuda.hpp:799
	pub fn cv_cuda_HostMem_setPropDatastart_unsigned_charX(instance: *mut c_void, val: *mut u8);
	// dataend /usr/include/opencv2/core/cuda.hpp:800
	pub fn cv_cuda_HostMem_getPropDataend_const(instance: *const c_void) -> *const u8;
	// alloc_type /usr/include/opencv2/core/cuda.hpp:802
	pub fn cv_cuda_HostMem_getPropAlloc_type_const(instance: *const c_void, ocvrs_return: *mut core::HostMem_AllocType);
	// alloc_type /usr/include/opencv2/core/cuda.hpp:802
	pub fn cv_cuda_HostMem_setPropAlloc_type_AllocType(instance: *mut c_void, val: core::HostMem_AllocType);
	// HostMem(HostMem::AllocType) /usr/include/opencv2/core/cuda.hpp:737
	pub fn cv_cuda_HostMem_HostMem_AllocType(alloc_type: core::HostMem_AllocType, ocvrs_return: *mut Result<*mut c_void>);
	// HostMem(const cv::cuda::HostMem &) /usr/include/opencv2/core/cuda.hpp:739
	pub fn cv_cuda_HostMem_HostMem_const_HostMemR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// HostMem(int, int, int, HostMem::AllocType) /usr/include/opencv2/core/cuda.hpp:741
	pub fn cv_cuda_HostMem_HostMem_int_int_int_AllocType(rows: i32, cols: i32, typ: i32, alloc_type: core::HostMem_AllocType, ocvrs_return: *mut Result<*mut c_void>);
	// HostMem(cv::Size, int, HostMem::AllocType) /usr/include/opencv2/core/cuda.hpp:742
	pub fn cv_cuda_HostMem_HostMem_Size_int_AllocType(size: *const core::Size, typ: i32, alloc_type: core::HostMem_AllocType, ocvrs_return: *mut Result<*mut c_void>);
	// HostMem(cv::InputArray, HostMem::AllocType) /usr/include/opencv2/core/cuda.hpp:745
	pub fn cv_cuda_HostMem_HostMem_const__InputArrayR_AllocType(arr: *const c_void, alloc_type: core::HostMem_AllocType, ocvrs_return: *mut Result<*mut c_void>);
	// swap(cv::cuda::HostMem &) /usr/include/opencv2/core/cuda.hpp:752
	pub fn cv_cuda_HostMem_swap_HostMemR(instance: *mut c_void, b: *mut c_void, ocvrs_return: *mut Result_void);
	// clone() /usr/include/opencv2/core/cuda.hpp:755
	pub fn cv_cuda_HostMem_clone_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, int, int) /usr/include/opencv2/core/cuda.hpp:758
	pub fn cv_cuda_HostMem_create_int_int_int(instance: *mut c_void, rows: i32, cols: i32, typ: i32, ocvrs_return: *mut Result_void);
	// create(cv::Size, int) /usr/include/opencv2/core/cuda.hpp:759
	pub fn cv_cuda_HostMem_create_Size_int(instance: *mut c_void, size: *const core::Size, typ: i32, ocvrs_return: *mut Result_void);
	// reshape(int, int) /usr/include/opencv2/core/cuda.hpp:763
	pub fn cv_cuda_HostMem_reshape_const_int_int(instance: *const c_void, cn: i32, rows: i32, ocvrs_return: *mut Result<*mut c_void>);
	// release() /usr/include/opencv2/core/cuda.hpp:766
	pub fn cv_cuda_HostMem_release(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// createMatHeader() /usr/include/opencv2/core/cuda.hpp:769
	pub fn cv_cuda_HostMem_createMatHeader_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// createGpuMatHeader() /usr/include/opencv2/core/cuda.hpp:778
	pub fn cv_cuda_HostMem_createGpuMatHeader_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// isContinuous() /usr/include/opencv2/core/cuda.hpp:781
	pub fn cv_cuda_HostMem_isContinuous_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// elemSize() /usr/include/opencv2/core/cuda.hpp:782
	pub fn cv_cuda_HostMem_elemSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// elemSize1() /usr/include/opencv2/core/cuda.hpp:783
	pub fn cv_cuda_HostMem_elemSize1_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// type() /usr/include/opencv2/core/cuda.hpp:784
	pub fn cv_cuda_HostMem_type_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// depth() /usr/include/opencv2/core/cuda.hpp:785
	pub fn cv_cuda_HostMem_depth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// channels() /usr/include/opencv2/core/cuda.hpp:786
	pub fn cv_cuda_HostMem_channels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// step1() /usr/include/opencv2/core/cuda.hpp:787
	pub fn cv_cuda_HostMem_step1_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// size() /usr/include/opencv2/core/cuda.hpp:788
	pub fn cv_cuda_HostMem_size_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// empty() /usr/include/opencv2/core/cuda.hpp:789
	pub fn cv_cuda_HostMem_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// Stream() /usr/include/opencv2/core/cuda.hpp:857
	pub fn cv_cuda_Stream_Stream(ocvrs_return: *mut Result<*mut c_void>);
	// Stream(const Ptr<GpuMat::Allocator> &) /usr/include/opencv2/core/cuda.hpp:860
	pub fn cv_cuda_Stream_Stream_const_Ptr_Allocator_R(allocator: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Stream(const size_t) /usr/include/opencv2/core/cuda.hpp:872
	pub fn cv_cuda_Stream_Stream_const_size_t(cuda_flags: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// queryIfComplete() /usr/include/opencv2/core/cuda.hpp:876
	pub fn cv_cuda_Stream_queryIfComplete_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// waitForCompletion() /usr/include/opencv2/core/cuda.hpp:880
	pub fn cv_cuda_Stream_waitForCompletion(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// waitEvent(const cv::cuda::Event &) /usr/include/opencv2/core/cuda.hpp:884
	pub fn cv_cuda_Stream_waitEvent_const_EventR(instance: *mut c_void, event: *const c_void, ocvrs_return: *mut Result_void);
	// enqueueHostCallback(cv::cuda::Stream::StreamCallback, void *) /usr/include/opencv2/core/cuda.hpp:894
	pub fn cv_cuda_Stream_enqueueHostCallback_StreamCallback_voidX(instance: *mut c_void, callback: Option<unsafe extern "C" fn(i32, *mut c_void) -> ()>, user_data: *mut c_void, ocvrs_return: *mut Result_void);
	// Null() /usr/include/opencv2/core/cuda.hpp:897
	pub fn cv_cuda_Stream_Null(ocvrs_return: *mut Result<*mut c_void>);
	// cudaPtr() /usr/include/opencv2/core/cuda.hpp:903
	pub fn cv_cuda_Stream_cudaPtr_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// builtWith(cv::cuda::FeatureSet) /usr/include/opencv2/core/cuda.hpp:1025
	pub fn cv_cuda_TargetArchs_builtWith_FeatureSet(feature_set: core::FeatureSet, ocvrs_return: *mut Result<bool>);
	// has(int, int) /usr/include/opencv2/core/cuda.hpp:1033
	pub fn cv_cuda_TargetArchs_has_int_int(major: i32, minor: i32, ocvrs_return: *mut Result<bool>);
	// hasPtx(int, int) /usr/include/opencv2/core/cuda.hpp:1034
	pub fn cv_cuda_TargetArchs_hasPtx_int_int(major: i32, minor: i32, ocvrs_return: *mut Result<bool>);
	// hasBin(int, int) /usr/include/opencv2/core/cuda.hpp:1035
	pub fn cv_cuda_TargetArchs_hasBin_int_int(major: i32, minor: i32, ocvrs_return: *mut Result<bool>);
	// hasEqualOrLessPtx(int, int) /usr/include/opencv2/core/cuda.hpp:1037
	pub fn cv_cuda_TargetArchs_hasEqualOrLessPtx_int_int(major: i32, minor: i32, ocvrs_return: *mut Result<bool>);
	// hasEqualOrGreater(int, int) /usr/include/opencv2/core/cuda.hpp:1038
	pub fn cv_cuda_TargetArchs_hasEqualOrGreater_int_int(major: i32, minor: i32, ocvrs_return: *mut Result<bool>);
	// hasEqualOrGreaterPtx(int, int) /usr/include/opencv2/core/cuda.hpp:1039
	pub fn cv_cuda_TargetArchs_hasEqualOrGreaterPtx_int_int(major: i32, minor: i32, ocvrs_return: *mut Result<bool>);
	// hasEqualOrGreaterBin(int, int) /usr/include/opencv2/core/cuda.hpp:1040
	pub fn cv_cuda_TargetArchs_hasEqualOrGreaterBin_int_int(major: i32, minor: i32, ocvrs_return: *mut Result<bool>);
	// func /usr/include/opencv2/core/check.hpp:40
	pub fn cv_detail_CheckContext_getPropFunc_const(instance: *const c_void) -> *mut c_void;
	// file /usr/include/opencv2/core/check.hpp:41
	pub fn cv_detail_CheckContext_getPropFile_const(instance: *const c_void) -> *mut c_void;
	// line /usr/include/opencv2/core/check.hpp:42
	pub fn cv_detail_CheckContext_getPropLine_const(instance: *const c_void) -> i32;
	// line /usr/include/opencv2/core/check.hpp:42
	pub fn cv_detail_CheckContext_setPropLine_int(instance: *mut c_void, val: i32);
	// testOp /usr/include/opencv2/core/check.hpp:43
	pub fn cv_detail_CheckContext_getPropTestOp_const(instance: *const c_void, ocvrs_return: *mut core::Detail_TestOp);
	// testOp /usr/include/opencv2/core/check.hpp:43
	pub fn cv_detail_CheckContext_setPropTestOp_TestOp(instance: *mut c_void, val: core::Detail_TestOp);
	// message /usr/include/opencv2/core/check.hpp:44
	pub fn cv_detail_CheckContext_getPropMessage_const(instance: *const c_void) -> *mut c_void;
	// p1_str /usr/include/opencv2/core/check.hpp:45
	pub fn cv_detail_CheckContext_getPropP1_str_const(instance: *const c_void) -> *mut c_void;
	// p2_str /usr/include/opencv2/core/check.hpp:46
	pub fn cv_detail_CheckContext_getPropP2_str_const(instance: *const c_void) -> *mut c_void;
	// m_funName /usr/include/opencv2/core/utils/instrumentation.hpp:77
	pub fn cv_instr_NodeData_getPropM_funName_const(instance: *const c_void) -> *mut c_void;
	// m_funName /usr/include/opencv2/core/utils/instrumentation.hpp:77
	pub fn cv_instr_NodeData_setPropM_funName_String(instance: *mut c_void, val: *mut c_char);
	// m_instrType /usr/include/opencv2/core/utils/instrumentation.hpp:78
	pub fn cv_instr_NodeData_getPropM_instrType_const(instance: *const c_void, ocvrs_return: *mut core::TYPE);
	// m_instrType /usr/include/opencv2/core/utils/instrumentation.hpp:78
	pub fn cv_instr_NodeData_setPropM_instrType_TYPE(instance: *mut c_void, val: core::TYPE);
	// m_implType /usr/include/opencv2/core/utils/instrumentation.hpp:79
	pub fn cv_instr_NodeData_getPropM_implType_const(instance: *const c_void, ocvrs_return: *mut core::IMPL);
	// m_implType /usr/include/opencv2/core/utils/instrumentation.hpp:79
	pub fn cv_instr_NodeData_setPropM_implType_IMPL(instance: *mut c_void, val: core::IMPL);
	// m_fileName /usr/include/opencv2/core/utils/instrumentation.hpp:80
	pub fn cv_instr_NodeData_getPropM_fileName_const(instance: *const c_void) -> *mut c_void;
	// m_lineNum /usr/include/opencv2/core/utils/instrumentation.hpp:81
	pub fn cv_instr_NodeData_getPropM_lineNum_const(instance: *const c_void) -> i32;
	// m_lineNum /usr/include/opencv2/core/utils/instrumentation.hpp:81
	pub fn cv_instr_NodeData_setPropM_lineNum_int(instance: *mut c_void, val: i32);
	// m_retAddress /usr/include/opencv2/core/utils/instrumentation.hpp:82
	pub fn cv_instr_NodeData_getPropM_retAddress(instance: *mut c_void) -> *mut c_void;
	// m_retAddress /usr/include/opencv2/core/utils/instrumentation.hpp:82
	pub fn cv_instr_NodeData_setPropM_retAddress_voidX(instance: *mut c_void, val: *mut c_void);
	// m_alwaysExpand /usr/include/opencv2/core/utils/instrumentation.hpp:83
	pub fn cv_instr_NodeData_getPropM_alwaysExpand_const(instance: *const c_void) -> bool;
	// m_alwaysExpand /usr/include/opencv2/core/utils/instrumentation.hpp:83
	pub fn cv_instr_NodeData_setPropM_alwaysExpand_bool(instance: *mut c_void, val: bool);
	// m_funError /usr/include/opencv2/core/utils/instrumentation.hpp:84
	pub fn cv_instr_NodeData_getPropM_funError_const(instance: *const c_void) -> bool;
	// m_funError /usr/include/opencv2/core/utils/instrumentation.hpp:84
	pub fn cv_instr_NodeData_setPropM_funError_bool(instance: *mut c_void, val: bool);
	// m_counter /usr/include/opencv2/core/utils/instrumentation.hpp:86
	pub fn cv_instr_NodeData_getPropM_counter_const(instance: *const c_void) -> i32;
	// m_counter /usr/include/opencv2/core/utils/instrumentation.hpp:86
	pub fn cv_instr_NodeData_setPropM_counter_int(instance: *mut c_void, val: i32);
	// m_ticksTotal /usr/include/opencv2/core/utils/instrumentation.hpp:87
	pub fn cv_instr_NodeData_getPropM_ticksTotal_const(instance: *const c_void) -> u64;
	// m_ticksTotal /usr/include/opencv2/core/utils/instrumentation.hpp:87
	pub fn cv_instr_NodeData_setPropM_ticksTotal_uint64_t(instance: *mut c_void, val: u64);
	// m_threads /usr/include/opencv2/core/utils/instrumentation.hpp:89
	pub fn cv_instr_NodeData_getPropM_threads_const(instance: *const c_void) -> i32;
	// m_threads /usr/include/opencv2/core/utils/instrumentation.hpp:89
	pub fn cv_instr_NodeData_setPropM_threads_int(instance: *mut c_void, val: i32);
	// NodeData(const char *, const char *, int, void *, bool, cv::instr::TYPE, cv::instr::IMPL) /usr/include/opencv2/core/utils/instrumentation.hpp:72
	pub fn cv_instr_NodeData_NodeData_const_charX_const_charX_int_voidX_bool_TYPE_IMPL(fun_name: *const c_char, file_name: *const c_char, line_num: i32, ret_address: *mut c_void, always_expand: bool, instr_type: core::TYPE, impl_type: core::IMPL, ocvrs_return: *mut Result<*mut c_void>);
	// NodeData(cv::instr::NodeData &) /usr/include/opencv2/core/utils/instrumentation.hpp:73
	pub fn cv_instr_NodeData_NodeData_NodeDataR(ref_: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getTotalMs() /usr/include/opencv2/core/utils/instrumentation.hpp:92
	pub fn cv_instr_NodeData_getTotalMs_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// getMeanMs() /usr/include/opencv2/core/utils/instrumentation.hpp:93
	pub fn cv_instr_NodeData_getMeanMs_const(instance: *const c_void, ocvrs_return: *mut Result<f64>);
	// WriteStructContext(cv::FileStorage &, const cv::String &, int, const cv::String &) /usr/include/opencv2/core/persistence.hpp:813
	pub fn cv_internal_WriteStructContext_WriteStructContext_FileStorageR_const_StringR_int_const_StringR(_fs: *mut c_void, name: *const c_char, flags: i32, type_name: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// Context() /usr/include/opencv2/core/ocl.hpp:256
	pub fn cv_ocl_Context_Context() -> *mut c_void;
	// Context(int) /usr/include/opencv2/core/ocl.hpp:257
	pub fn cv_ocl_Context_Context_int(dtype: i32, ocvrs_return: *mut Result<*mut c_void>);
	// Context(const cv::ocl::Context &) /usr/include/opencv2/core/ocl.hpp:259
	pub fn cv_ocl_Context_Context_const_ContextR(c: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Context(cv::ocl::Context &&) /usr/include/opencv2/core/ocl.hpp:261
	pub fn cv_ocl_Context_Context_ContextR(c: *mut c_void) -> *mut c_void;
	// create() /usr/include/opencv2/core/ocl.hpp:265
	pub fn cv_ocl_Context_create(instance: *mut c_void, ocvrs_return: *mut Result<bool>);
	// create(int) /usr/include/opencv2/core/ocl.hpp:267
	pub fn cv_ocl_Context_create_int(instance: *mut c_void, dtype: i32, ocvrs_return: *mut Result<bool>);
	// ndevices() /usr/include/opencv2/core/ocl.hpp:269
	pub fn cv_ocl_Context_ndevices_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// device(size_t) /usr/include/opencv2/core/ocl.hpp:270
	pub fn cv_ocl_Context_device_const_size_t(instance: *const c_void, idx: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// getProg(const cv::ocl::ProgramSource &, const cv::String &, cv::String &) /usr/include/opencv2/core/ocl.hpp:271
	pub fn cv_ocl_Context_getProg_const_ProgramSourceR_const_StringR_StringR(instance: *mut c_void, prog: *const c_void, buildopt: *const c_char, errmsg: *mut *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// unloadProg(cv::ocl::Program &) /usr/include/opencv2/core/ocl.hpp:273
	pub fn cv_ocl_Context_unloadProg_ProgramR(instance: *mut c_void, prog: *mut c_void, ocvrs_return: *mut Result_void);
	// getDefault(bool) /usr/include/opencv2/core/ocl.hpp:280
	pub fn cv_ocl_Context_getDefault_bool(initialize: bool, ocvrs_return: *mut Result<*mut c_void>);
	// ptr() /usr/include/opencv2/core/ocl.hpp:284
	pub fn cv_ocl_Context_ptr_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getOpenCLContextProperty(int) /usr/include/opencv2/core/ocl.hpp:291
	pub fn cv_ocl_Context_getOpenCLContextProperty_const_int(instance: *const c_void, property_id: i32, ocvrs_return: *mut Result<*mut c_void>);
	// useSVM() /usr/include/opencv2/core/ocl.hpp:293
	pub fn cv_ocl_Context_useSVM_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setUseSVM(bool) /usr/include/opencv2/core/ocl.hpp:294
	pub fn cv_ocl_Context_setUseSVM_bool(instance: *mut c_void, enabled: bool, ocvrs_return: *mut Result_void);
	// fromHandle(void *) /usr/include/opencv2/core/ocl.hpp:299
	pub fn cv_ocl_Context_fromHandle_voidX(context: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// fromDevice(const ocl::Device &) /usr/include/opencv2/core/ocl.hpp:300
	pub fn cv_ocl_Context_fromDevice_const_DeviceR(device: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const std::string &) /usr/include/opencv2/core/ocl.hpp:301
	pub fn cv_ocl_Context_create_const_stringR(configuration: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// release() /usr/include/opencv2/core/ocl.hpp:303
	pub fn cv_ocl_Context_release(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// empty() /usr/include/opencv2/core/ocl.hpp:322
	pub fn cv_ocl_Context_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// Device() /usr/include/opencv2/core/ocl.hpp:75
	pub fn cv_ocl_Device_Device() -> *mut c_void;
	// Device(void *) /usr/include/opencv2/core/ocl.hpp:76
	pub fn cv_ocl_Device_Device_voidX(d: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Device(const cv::ocl::Device &) /usr/include/opencv2/core/ocl.hpp:77
	pub fn cv_ocl_Device_Device_const_DeviceR(d: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Device(cv::ocl::Device &&) /usr/include/opencv2/core/ocl.hpp:79
	pub fn cv_ocl_Device_Device_DeviceR(d: *mut c_void) -> *mut c_void;
	// set(void *) /usr/include/opencv2/core/ocl.hpp:83
	pub fn cv_ocl_Device_set_voidX(instance: *mut c_void, d: *mut c_void, ocvrs_return: *mut Result_void);
	// name() /usr/include/opencv2/core/ocl.hpp:96
	pub fn cv_ocl_Device_name_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// extensions() /usr/include/opencv2/core/ocl.hpp:97
	pub fn cv_ocl_Device_extensions_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// isExtensionSupported(const cv::String &) /usr/include/opencv2/core/ocl.hpp:98
	pub fn cv_ocl_Device_isExtensionSupported_const_const_StringR(instance: *const c_void, extension_name: *const c_char, ocvrs_return: *mut Result<bool>);
	// version() /usr/include/opencv2/core/ocl.hpp:99
	pub fn cv_ocl_Device_version_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// vendorName() /usr/include/opencv2/core/ocl.hpp:100
	pub fn cv_ocl_Device_vendorName_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// OpenCL_C_Version() /usr/include/opencv2/core/ocl.hpp:101
	pub fn cv_ocl_Device_OpenCL_C_Version_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// OpenCLVersion() /usr/include/opencv2/core/ocl.hpp:102
	pub fn cv_ocl_Device_OpenCLVersion_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// deviceVersionMajor() /usr/include/opencv2/core/ocl.hpp:103
	pub fn cv_ocl_Device_deviceVersionMajor_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// deviceVersionMinor() /usr/include/opencv2/core/ocl.hpp:104
	pub fn cv_ocl_Device_deviceVersionMinor_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// driverVersion() /usr/include/opencv2/core/ocl.hpp:105
	pub fn cv_ocl_Device_driverVersion_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// ptr() /usr/include/opencv2/core/ocl.hpp:106
	pub fn cv_ocl_Device_ptr_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// type() /usr/include/opencv2/core/ocl.hpp:108
	pub fn cv_ocl_Device_type_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// addressBits() /usr/include/opencv2/core/ocl.hpp:110
	pub fn cv_ocl_Device_addressBits_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// available() /usr/include/opencv2/core/ocl.hpp:111
	pub fn cv_ocl_Device_available_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// compilerAvailable() /usr/include/opencv2/core/ocl.hpp:112
	pub fn cv_ocl_Device_compilerAvailable_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// linkerAvailable() /usr/include/opencv2/core/ocl.hpp:113
	pub fn cv_ocl_Device_linkerAvailable_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// doubleFPConfig() /usr/include/opencv2/core/ocl.hpp:126
	pub fn cv_ocl_Device_doubleFPConfig_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// singleFPConfig() /usr/include/opencv2/core/ocl.hpp:127
	pub fn cv_ocl_Device_singleFPConfig_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// halfFPConfig() /usr/include/opencv2/core/ocl.hpp:128
	pub fn cv_ocl_Device_halfFPConfig_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// endianLittle() /usr/include/opencv2/core/ocl.hpp:130
	pub fn cv_ocl_Device_endianLittle_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// errorCorrectionSupport() /usr/include/opencv2/core/ocl.hpp:131
	pub fn cv_ocl_Device_errorCorrectionSupport_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// executionCapabilities() /usr/include/opencv2/core/ocl.hpp:138
	pub fn cv_ocl_Device_executionCapabilities_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// globalMemCacheSize() /usr/include/opencv2/core/ocl.hpp:140
	pub fn cv_ocl_Device_globalMemCacheSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// globalMemCacheType() /usr/include/opencv2/core/ocl.hpp:148
	pub fn cv_ocl_Device_globalMemCacheType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// globalMemCacheLineSize() /usr/include/opencv2/core/ocl.hpp:149
	pub fn cv_ocl_Device_globalMemCacheLineSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// globalMemSize() /usr/include/opencv2/core/ocl.hpp:150
	pub fn cv_ocl_Device_globalMemSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// localMemSize() /usr/include/opencv2/core/ocl.hpp:152
	pub fn cv_ocl_Device_localMemSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// localMemType() /usr/include/opencv2/core/ocl.hpp:159
	pub fn cv_ocl_Device_localMemType_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// hostUnifiedMemory() /usr/include/opencv2/core/ocl.hpp:160
	pub fn cv_ocl_Device_hostUnifiedMemory_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// imageSupport() /usr/include/opencv2/core/ocl.hpp:162
	pub fn cv_ocl_Device_imageSupport_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// imageFromBufferSupport() /usr/include/opencv2/core/ocl.hpp:164
	pub fn cv_ocl_Device_imageFromBufferSupport_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// imagePitchAlignment() /usr/include/opencv2/core/ocl.hpp:165
	pub fn cv_ocl_Device_imagePitchAlignment_const(instance: *const c_void, ocvrs_return: *mut Result<u32>);
	// imageBaseAddressAlignment() /usr/include/opencv2/core/ocl.hpp:166
	pub fn cv_ocl_Device_imageBaseAddressAlignment_const(instance: *const c_void, ocvrs_return: *mut Result<u32>);
	// intelSubgroupsSupport() /usr/include/opencv2/core/ocl.hpp:169
	pub fn cv_ocl_Device_intelSubgroupsSupport_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// image2DMaxWidth() /usr/include/opencv2/core/ocl.hpp:171
	pub fn cv_ocl_Device_image2DMaxWidth_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// image2DMaxHeight() /usr/include/opencv2/core/ocl.hpp:172
	pub fn cv_ocl_Device_image2DMaxHeight_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// image3DMaxWidth() /usr/include/opencv2/core/ocl.hpp:174
	pub fn cv_ocl_Device_image3DMaxWidth_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// image3DMaxHeight() /usr/include/opencv2/core/ocl.hpp:175
	pub fn cv_ocl_Device_image3DMaxHeight_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// image3DMaxDepth() /usr/include/opencv2/core/ocl.hpp:176
	pub fn cv_ocl_Device_image3DMaxDepth_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// imageMaxBufferSize() /usr/include/opencv2/core/ocl.hpp:178
	pub fn cv_ocl_Device_imageMaxBufferSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// imageMaxArraySize() /usr/include/opencv2/core/ocl.hpp:179
	pub fn cv_ocl_Device_imageMaxArraySize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// vendorID() /usr/include/opencv2/core/ocl.hpp:188
	pub fn cv_ocl_Device_vendorID_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// isAMD() /usr/include/opencv2/core/ocl.hpp:193
	pub fn cv_ocl_Device_isAMD_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isIntel() /usr/include/opencv2/core/ocl.hpp:194
	pub fn cv_ocl_Device_isIntel_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// isNVidia() /usr/include/opencv2/core/ocl.hpp:195
	pub fn cv_ocl_Device_isNVidia_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// maxClockFrequency() /usr/include/opencv2/core/ocl.hpp:197
	pub fn cv_ocl_Device_maxClockFrequency_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// maxComputeUnits() /usr/include/opencv2/core/ocl.hpp:198
	pub fn cv_ocl_Device_maxComputeUnits_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// maxConstantArgs() /usr/include/opencv2/core/ocl.hpp:199
	pub fn cv_ocl_Device_maxConstantArgs_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// maxConstantBufferSize() /usr/include/opencv2/core/ocl.hpp:200
	pub fn cv_ocl_Device_maxConstantBufferSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// maxMemAllocSize() /usr/include/opencv2/core/ocl.hpp:202
	pub fn cv_ocl_Device_maxMemAllocSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// maxParameterSize() /usr/include/opencv2/core/ocl.hpp:203
	pub fn cv_ocl_Device_maxParameterSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// maxReadImageArgs() /usr/include/opencv2/core/ocl.hpp:205
	pub fn cv_ocl_Device_maxReadImageArgs_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// maxWriteImageArgs() /usr/include/opencv2/core/ocl.hpp:206
	pub fn cv_ocl_Device_maxWriteImageArgs_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// maxSamplers() /usr/include/opencv2/core/ocl.hpp:207
	pub fn cv_ocl_Device_maxSamplers_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// maxWorkGroupSize() /usr/include/opencv2/core/ocl.hpp:209
	pub fn cv_ocl_Device_maxWorkGroupSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// maxWorkItemDims() /usr/include/opencv2/core/ocl.hpp:210
	pub fn cv_ocl_Device_maxWorkItemDims_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// maxWorkItemSizes(size_t *) /usr/include/opencv2/core/ocl.hpp:211
	pub fn cv_ocl_Device_maxWorkItemSizes_const_size_tX(instance: *const c_void, unnamed: *mut size_t, ocvrs_return: *mut Result_void);
	// memBaseAddrAlign() /usr/include/opencv2/core/ocl.hpp:213
	pub fn cv_ocl_Device_memBaseAddrAlign_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// nativeVectorWidthChar() /usr/include/opencv2/core/ocl.hpp:215
	pub fn cv_ocl_Device_nativeVectorWidthChar_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// nativeVectorWidthShort() /usr/include/opencv2/core/ocl.hpp:216
	pub fn cv_ocl_Device_nativeVectorWidthShort_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// nativeVectorWidthInt() /usr/include/opencv2/core/ocl.hpp:217
	pub fn cv_ocl_Device_nativeVectorWidthInt_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// nativeVectorWidthLong() /usr/include/opencv2/core/ocl.hpp:218
	pub fn cv_ocl_Device_nativeVectorWidthLong_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// nativeVectorWidthFloat() /usr/include/opencv2/core/ocl.hpp:219
	pub fn cv_ocl_Device_nativeVectorWidthFloat_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// nativeVectorWidthDouble() /usr/include/opencv2/core/ocl.hpp:220
	pub fn cv_ocl_Device_nativeVectorWidthDouble_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// nativeVectorWidthHalf() /usr/include/opencv2/core/ocl.hpp:221
	pub fn cv_ocl_Device_nativeVectorWidthHalf_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// preferredVectorWidthChar() /usr/include/opencv2/core/ocl.hpp:223
	pub fn cv_ocl_Device_preferredVectorWidthChar_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// preferredVectorWidthShort() /usr/include/opencv2/core/ocl.hpp:224
	pub fn cv_ocl_Device_preferredVectorWidthShort_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// preferredVectorWidthInt() /usr/include/opencv2/core/ocl.hpp:225
	pub fn cv_ocl_Device_preferredVectorWidthInt_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// preferredVectorWidthLong() /usr/include/opencv2/core/ocl.hpp:226
	pub fn cv_ocl_Device_preferredVectorWidthLong_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// preferredVectorWidthFloat() /usr/include/opencv2/core/ocl.hpp:227
	pub fn cv_ocl_Device_preferredVectorWidthFloat_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// preferredVectorWidthDouble() /usr/include/opencv2/core/ocl.hpp:228
	pub fn cv_ocl_Device_preferredVectorWidthDouble_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// preferredVectorWidthHalf() /usr/include/opencv2/core/ocl.hpp:229
	pub fn cv_ocl_Device_preferredVectorWidthHalf_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// printfBufferSize() /usr/include/opencv2/core/ocl.hpp:231
	pub fn cv_ocl_Device_printfBufferSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// profilingTimerResolution() /usr/include/opencv2/core/ocl.hpp:232
	pub fn cv_ocl_Device_profilingTimerResolution_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// getDefault() /usr/include/opencv2/core/ocl.hpp:234
	pub fn cv_ocl_Device_getDefault(ocvrs_return: *mut Result<*mut c_void>);
	// fromHandle(void *) /usr/include/opencv2/core/ocl.hpp:243
	pub fn cv_ocl_Device_fromHandle_voidX(d: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// empty() /usr/include/opencv2/core/ocl.hpp:247
	pub fn cv_ocl_Device_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// Image2D() /usr/include/opencv2/core/ocl.hpp:742
	pub fn cv_ocl_Image2D_Image2D() -> *mut c_void;
	// Image2D(const cv::UMat &, bool, bool) /usr/include/opencv2/core/ocl.hpp:750
	pub fn cv_ocl_Image2D_Image2D_const_UMatR_bool_bool(src: *const c_void, norm: bool, alias: bool, ocvrs_return: *mut Result<*mut c_void>);
	// Image2D(const cv::ocl::Image2D &) /usr/include/opencv2/core/ocl.hpp:751
	pub fn cv_ocl_Image2D_Image2D_const_Image2DR(i: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Image2D(cv::ocl::Image2D &&) /usr/include/opencv2/core/ocl.hpp:755
	pub fn cv_ocl_Image2D_Image2D_Image2DR(unnamed: *mut c_void) -> *mut c_void;
	// canCreateAlias(const cv::UMat &) /usr/include/opencv2/core/ocl.hpp:761
	pub fn cv_ocl_Image2D_canCreateAlias_const_UMatR(u: *const c_void, ocvrs_return: *mut Result<bool>);
	// isFormatSupported(int, int, bool) /usr/include/opencv2/core/ocl.hpp:765
	pub fn cv_ocl_Image2D_isFormatSupported_int_int_bool(depth: i32, cn: i32, norm: bool, ocvrs_return: *mut Result<bool>);
	// ptr() /usr/include/opencv2/core/ocl.hpp:767
	pub fn cv_ocl_Image2D_ptr_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Kernel() /usr/include/opencv2/core/ocl.hpp:459
	pub fn cv_ocl_Kernel_Kernel() -> *mut c_void;
	// Kernel(const char *, const cv::ocl::Program &) /usr/include/opencv2/core/ocl.hpp:460
	pub fn cv_ocl_Kernel_Kernel_const_charX_const_ProgramR(kname: *const c_char, prog: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Kernel(const char *, const cv::ocl::ProgramSource &, const cv::String &, cv::String *) /usr/include/opencv2/core/ocl.hpp:461
	pub fn cv_ocl_Kernel_Kernel_const_charX_const_ProgramSourceR_const_StringR_StringX(kname: *const c_char, prog: *const c_void, buildopts: *const c_char, errmsg: *mut *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Kernel(const cv::ocl::Kernel &) /usr/include/opencv2/core/ocl.hpp:464
	pub fn cv_ocl_Kernel_Kernel_const_KernelR(k: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Kernel(cv::ocl::Kernel &&) /usr/include/opencv2/core/ocl.hpp:466
	pub fn cv_ocl_Kernel_Kernel_KernelR(k: *mut c_void) -> *mut c_void;
	// empty() /usr/include/opencv2/core/ocl.hpp:469
	pub fn cv_ocl_Kernel_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// create(const char *, const cv::ocl::Program &) /usr/include/opencv2/core/ocl.hpp:470
	pub fn cv_ocl_Kernel_create_const_charX_const_ProgramR(instance: *mut c_void, kname: *const c_char, prog: *const c_void, ocvrs_return: *mut Result<bool>);
	// create(const char *, const cv::ocl::ProgramSource &, const cv::String &, cv::String *) /usr/include/opencv2/core/ocl.hpp:471
	pub fn cv_ocl_Kernel_create_const_charX_const_ProgramSourceR_const_StringR_StringX(instance: *mut c_void, kname: *const c_char, prog: *const c_void, buildopts: *const c_char, errmsg: *mut *mut c_void, ocvrs_return: *mut Result<bool>);
	// set(int, const void *, size_t) /usr/include/opencv2/core/ocl.hpp:474
	pub fn cv_ocl_Kernel_set_int_const_voidX_size_t(instance: *mut c_void, i: i32, value: *const c_void, sz: size_t, ocvrs_return: *mut Result<i32>);
	// set(int, const cv::ocl::Image2D &) /usr/include/opencv2/core/ocl.hpp:475
	pub fn cv_ocl_Kernel_set_int_const_Image2DR(instance: *mut c_void, i: i32, image_2d: *const c_void, ocvrs_return: *mut Result<i32>);
	// set(int, const cv::UMat &) /usr/include/opencv2/core/ocl.hpp:476
	pub fn cv_ocl_Kernel_set_int_const_UMatR(instance: *mut c_void, i: i32, m: *const c_void, ocvrs_return: *mut Result<i32>);
	// set(int, const cv::ocl::KernelArg &) /usr/include/opencv2/core/ocl.hpp:477
	pub fn cv_ocl_Kernel_set_int_const_KernelArgR(instance: *mut c_void, i: i32, arg: *const c_void, ocvrs_return: *mut Result<i32>);
	// run(int, size_t *, size_t *, bool, const cv::ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:515
	pub fn cv_ocl_Kernel_run_int_size_tX_size_tX_bool_const_QueueR(instance: *mut c_void, dims: i32, globalsize: *mut size_t, localsize: *mut size_t, sync: bool, q: *const c_void, ocvrs_return: *mut Result<bool>);
	// run_(int, size_t *, size_t *, bool, const cv::ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:526
	pub fn cv_ocl_Kernel_run__int_size_tX_size_tX_bool_const_QueueR(instance: *mut c_void, dims: i32, globalsize: *mut size_t, localsize: *mut size_t, sync: bool, q: *const c_void, ocvrs_return: *mut Result<bool>);
	// runTask(bool, const cv::ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:528
	pub fn cv_ocl_Kernel_runTask_bool_const_QueueR(instance: *mut c_void, sync: bool, q: *const c_void, ocvrs_return: *mut Result<bool>);
	// runProfiling(int, size_t *, size_t *, const cv::ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:535
	pub fn cv_ocl_Kernel_runProfiling_int_size_tX_size_tX_const_QueueR(instance: *mut c_void, dims: i32, globalsize: *mut size_t, localsize: *mut size_t, q: *const c_void, ocvrs_return: *mut Result<i64>);
	// workGroupSize() /usr/include/opencv2/core/ocl.hpp:537
	pub fn cv_ocl_Kernel_workGroupSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// preferedWorkGroupSizeMultiple() /usr/include/opencv2/core/ocl.hpp:538
	pub fn cv_ocl_Kernel_preferedWorkGroupSizeMultiple_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// compileWorkGroupSize(size_t *) /usr/include/opencv2/core/ocl.hpp:539
	pub fn cv_ocl_Kernel_compileWorkGroupSize_const_size_tX(instance: *const c_void, wsz: *mut size_t, ocvrs_return: *mut Result<bool>);
	// localMemSize() /usr/include/opencv2/core/ocl.hpp:540
	pub fn cv_ocl_Kernel_localMemSize_const(instance: *const c_void, ocvrs_return: *mut Result<size_t>);
	// ptr() /usr/include/opencv2/core/ocl.hpp:542
	pub fn cv_ocl_Kernel_ptr_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// flags /usr/include/opencv2/core/ocl.hpp:448
	pub fn cv_ocl_KernelArg_getPropFlags_const(instance: *const c_void) -> i32;
	// flags /usr/include/opencv2/core/ocl.hpp:448
	pub fn cv_ocl_KernelArg_setPropFlags_int(instance: *mut c_void, val: i32);
	// m /usr/include/opencv2/core/ocl.hpp:449
	pub fn cv_ocl_KernelArg_getPropM(instance: *mut c_void) -> *mut c_void;
	// m /usr/include/opencv2/core/ocl.hpp:449
	pub fn cv_ocl_KernelArg_setPropM_UMatX(instance: *mut c_void, val: *mut c_void);
	// obj /usr/include/opencv2/core/ocl.hpp:450
	pub fn cv_ocl_KernelArg_getPropObj_const(instance: *const c_void) -> *const c_void;
	// sz /usr/include/opencv2/core/ocl.hpp:451
	pub fn cv_ocl_KernelArg_getPropSz_const(instance: *const c_void) -> size_t;
	// sz /usr/include/opencv2/core/ocl.hpp:451
	pub fn cv_ocl_KernelArg_setPropSz_size_t(instance: *mut c_void, val: size_t);
	// wscale /usr/include/opencv2/core/ocl.hpp:452
	pub fn cv_ocl_KernelArg_getPropWscale_const(instance: *const c_void) -> i32;
	// wscale /usr/include/opencv2/core/ocl.hpp:452
	pub fn cv_ocl_KernelArg_setPropWscale_int(instance: *mut c_void, val: i32);
	// iwscale /usr/include/opencv2/core/ocl.hpp:452
	pub fn cv_ocl_KernelArg_getPropIwscale_const(instance: *const c_void) -> i32;
	// iwscale /usr/include/opencv2/core/ocl.hpp:452
	pub fn cv_ocl_KernelArg_setPropIwscale_int(instance: *mut c_void, val: i32);
	// KernelArg(int, cv::UMat *, int, int, const void *, size_t) /usr/include/opencv2/core/ocl.hpp:421
	pub fn cv_ocl_KernelArg_KernelArg_int_UMatX_int_int_const_voidX_size_t(_flags: i32, _m: *mut c_void, wscale: i32, iwscale: i32, _obj: *const c_void, _sz: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// KernelArg() /usr/include/opencv2/core/ocl.hpp:422
	pub fn cv_ocl_KernelArg_KernelArg() -> *mut c_void;
	// Local(size_t) /usr/include/opencv2/core/ocl.hpp:424
	pub fn cv_ocl_KernelArg_Local_size_t(local_mem_size: size_t, ocvrs_return: *mut Result<*mut c_void>);
	// PtrWriteOnly(const cv::UMat &) /usr/include/opencv2/core/ocl.hpp:426
	pub fn cv_ocl_KernelArg_PtrWriteOnly_const_UMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// PtrReadOnly(const cv::UMat &) /usr/include/opencv2/core/ocl.hpp:428
	pub fn cv_ocl_KernelArg_PtrReadOnly_const_UMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// PtrReadWrite(const cv::UMat &) /usr/include/opencv2/core/ocl.hpp:430
	pub fn cv_ocl_KernelArg_PtrReadWrite_const_UMatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// ReadWrite(const cv::UMat &, int, int) /usr/include/opencv2/core/ocl.hpp:432
	pub fn cv_ocl_KernelArg_ReadWrite_const_UMatR_int_int(m: *const c_void, wscale: i32, iwscale: i32, ocvrs_return: *mut Result<*mut c_void>);
	// ReadWriteNoSize(const cv::UMat &, int, int) /usr/include/opencv2/core/ocl.hpp:434
	pub fn cv_ocl_KernelArg_ReadWriteNoSize_const_UMatR_int_int(m: *const c_void, wscale: i32, iwscale: i32, ocvrs_return: *mut Result<*mut c_void>);
	// ReadOnly(const cv::UMat &, int, int) /usr/include/opencv2/core/ocl.hpp:436
	pub fn cv_ocl_KernelArg_ReadOnly_const_UMatR_int_int(m: *const c_void, wscale: i32, iwscale: i32, ocvrs_return: *mut Result<*mut c_void>);
	// WriteOnly(const cv::UMat &, int, int) /usr/include/opencv2/core/ocl.hpp:438
	pub fn cv_ocl_KernelArg_WriteOnly_const_UMatR_int_int(m: *const c_void, wscale: i32, iwscale: i32, ocvrs_return: *mut Result<*mut c_void>);
	// ReadOnlyNoSize(const cv::UMat &, int, int) /usr/include/opencv2/core/ocl.hpp:440
	pub fn cv_ocl_KernelArg_ReadOnlyNoSize_const_UMatR_int_int(m: *const c_void, wscale: i32, iwscale: i32, ocvrs_return: *mut Result<*mut c_void>);
	// WriteOnlyNoSize(const cv::UMat &, int, int) /usr/include/opencv2/core/ocl.hpp:442
	pub fn cv_ocl_KernelArg_WriteOnlyNoSize_const_UMatR_int_int(m: *const c_void, wscale: i32, iwscale: i32, ocvrs_return: *mut Result<*mut c_void>);
	// Constant(const cv::Mat &) /usr/include/opencv2/core/ocl.hpp:444
	pub fn cv_ocl_KernelArg_Constant_const_MatR(m: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// OpenCLExecutionContext() /usr/include/opencv2/core/ocl.hpp:798
	pub fn cv_ocl_OpenCLExecutionContext_OpenCLExecutionContext() -> *mut c_void;
	// OpenCLExecutionContext(const cv::ocl::OpenCLExecutionContext &) /usr/include/opencv2/core/ocl.hpp:801
	pub fn cv_ocl_OpenCLExecutionContext_OpenCLExecutionContext_const_OpenCLExecutionContextR(unnamed: *const c_void) -> *mut c_void;
	// OpenCLExecutionContext(cv::ocl::OpenCLExecutionContext &&) /usr/include/opencv2/core/ocl.hpp:802
	pub fn cv_ocl_OpenCLExecutionContext_OpenCLExecutionContext_OpenCLExecutionContextR(unnamed: *mut c_void) -> *mut c_void;
	// getContext() /usr/include/opencv2/core/ocl.hpp:808
	pub fn cv_ocl_OpenCLExecutionContext_getContext_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getDevice() /usr/include/opencv2/core/ocl.hpp:810
	pub fn cv_ocl_OpenCLExecutionContext_getDevice_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getQueue() /usr/include/opencv2/core/ocl.hpp:814
	pub fn cv_ocl_OpenCLExecutionContext_getQueue_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// useOpenCL() /usr/include/opencv2/core/ocl.hpp:816
	pub fn cv_ocl_OpenCLExecutionContext_useOpenCL_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// setUseOpenCL(bool) /usr/include/opencv2/core/ocl.hpp:817
	pub fn cv_ocl_OpenCLExecutionContext_setUseOpenCL_bool(instance: *mut c_void, flag: bool, ocvrs_return: *mut Result_void);
	// getCurrent() /usr/include/opencv2/core/ocl.hpp:825
	pub fn cv_ocl_OpenCLExecutionContext_getCurrent(ocvrs_return: *mut Result<*mut c_void>);
	// getCurrentRef() /usr/include/opencv2/core/ocl.hpp:828
	pub fn cv_ocl_OpenCLExecutionContext_getCurrentRef(ocvrs_return: *mut Result<*mut c_void>);
	// bind() /usr/include/opencv2/core/ocl.hpp:836
	pub fn cv_ocl_OpenCLExecutionContext_bind_const(instance: *const c_void, ocvrs_return: *mut Result_void);
	// cloneWithNewQueue(const ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:842
	pub fn cv_ocl_OpenCLExecutionContext_cloneWithNewQueue_const_const_QueueR(instance: *const c_void, q: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// cloneWithNewQueue() /usr/include/opencv2/core/ocl.hpp:844
	pub fn cv_ocl_OpenCLExecutionContext_cloneWithNewQueue_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const std::string &, void *, void *, void *) /usr/include/opencv2/core/ocl.hpp:860
	pub fn cv_ocl_OpenCLExecutionContext_create_const_stringR_voidX_voidX_voidX(platform_name: *const c_char, platform_id: *mut c_void, context: *mut c_void, device_id: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::ocl::Context &, const cv::ocl::Device &, const ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:868
	pub fn cv_ocl_OpenCLExecutionContext_create_const_ContextR_const_DeviceR_const_QueueR(context: *const c_void, device: *const c_void, queue: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// create(const cv::ocl::Context &, const cv::ocl::Device &) /usr/include/opencv2/core/ocl.hpp:870
	pub fn cv_ocl_OpenCLExecutionContext_create_const_ContextR_const_DeviceR(context: *const c_void, device: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// empty() /usr/include/opencv2/core/ocl.hpp:873
	pub fn cv_ocl_OpenCLExecutionContext_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// release() /usr/include/opencv2/core/ocl.hpp:874
	pub fn cv_ocl_OpenCLExecutionContext_release(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// Platform() /usr/include/opencv2/core/ocl.hpp:332
	pub fn cv_ocl_Platform_Platform() -> *mut c_void;
	// Platform(const cv::ocl::Platform &) /usr/include/opencv2/core/ocl.hpp:334
	pub fn cv_ocl_Platform_Platform_const_PlatformR(p: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Platform(cv::ocl::Platform &&) /usr/include/opencv2/core/ocl.hpp:336
	pub fn cv_ocl_Platform_Platform_PlatformR(p: *mut c_void) -> *mut c_void;
	// ptr() /usr/include/opencv2/core/ocl.hpp:339
	pub fn cv_ocl_Platform_ptr_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getDefault() /usr/include/opencv2/core/ocl.hpp:342
	pub fn cv_ocl_Platform_getDefault(ocvrs_return: *mut Result<*mut c_void>);
	// empty() /usr/include/opencv2/core/ocl.hpp:346
	pub fn cv_ocl_Platform_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// PlatformInfo() /usr/include/opencv2/core/ocl.hpp:671
	pub fn cv_ocl_PlatformInfo_PlatformInfo() -> *mut c_void;
	// PlatformInfo(void *) /usr/include/opencv2/core/ocl.hpp:675
	pub fn cv_ocl_PlatformInfo_PlatformInfo_voidX(id: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// PlatformInfo(const cv::ocl::PlatformInfo &) /usr/include/opencv2/core/ocl.hpp:678
	pub fn cv_ocl_PlatformInfo_PlatformInfo_const_PlatformInfoR(i: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// PlatformInfo(cv::ocl::PlatformInfo &&) /usr/include/opencv2/core/ocl.hpp:680
	pub fn cv_ocl_PlatformInfo_PlatformInfo_PlatformInfoR(i: *mut c_void) -> *mut c_void;
	// name() /usr/include/opencv2/core/ocl.hpp:683
	pub fn cv_ocl_PlatformInfo_name_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// vendor() /usr/include/opencv2/core/ocl.hpp:684
	pub fn cv_ocl_PlatformInfo_vendor_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// version() /usr/include/opencv2/core/ocl.hpp:687
	pub fn cv_ocl_PlatformInfo_version_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// versionMajor() /usr/include/opencv2/core/ocl.hpp:688
	pub fn cv_ocl_PlatformInfo_versionMajor_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// versionMinor() /usr/include/opencv2/core/ocl.hpp:689
	pub fn cv_ocl_PlatformInfo_versionMinor_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// deviceNumber() /usr/include/opencv2/core/ocl.hpp:691
	pub fn cv_ocl_PlatformInfo_deviceNumber_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getDevice(cv::ocl::Device &, int) /usr/include/opencv2/core/ocl.hpp:692
	pub fn cv_ocl_PlatformInfo_getDevice_const_DeviceR_int(instance: *const c_void, device: *mut c_void, d: i32, ocvrs_return: *mut Result_void);
	// empty() /usr/include/opencv2/core/ocl.hpp:695
	pub fn cv_ocl_PlatformInfo_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// Program() /usr/include/opencv2/core/ocl.hpp:552
	pub fn cv_ocl_Program_Program() -> *mut c_void;
	// Program(const cv::ocl::ProgramSource &, const cv::String &, cv::String &) /usr/include/opencv2/core/ocl.hpp:553
	pub fn cv_ocl_Program_Program_const_ProgramSourceR_const_StringR_StringR(src: *const c_void, buildflags: *const c_char, errmsg: *mut *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Program(const cv::ocl::Program &) /usr/include/opencv2/core/ocl.hpp:555
	pub fn cv_ocl_Program_Program_const_ProgramR(prog: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Program(cv::ocl::Program &&) /usr/include/opencv2/core/ocl.hpp:557
	pub fn cv_ocl_Program_Program_ProgramR(prog: *mut c_void) -> *mut c_void;
	// create(const cv::ocl::ProgramSource &, const cv::String &, cv::String &) /usr/include/opencv2/core/ocl.hpp:561
	pub fn cv_ocl_Program_create_const_ProgramSourceR_const_StringR_StringR(instance: *mut c_void, src: *const c_void, buildflags: *const c_char, errmsg: *mut *mut c_void, ocvrs_return: *mut Result<bool>);
	// ptr() /usr/include/opencv2/core/ocl.hpp:564
	pub fn cv_ocl_Program_ptr_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getBinary(std::vector<char> &) /usr/include/opencv2/core/ocl.hpp:575
	pub fn cv_ocl_Program_getBinary_const_vector_char_R(instance: *const c_void, binary: *mut c_void, ocvrs_return: *mut Result_void);
	// empty() /usr/include/opencv2/core/ocl.hpp:579
	pub fn cv_ocl_Program_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// read(const cv::String &, const cv::String &) /usr/include/opencv2/core/ocl.hpp:585
	pub fn cv_ocl_Program_read_const_StringR_const_StringR(instance: *mut c_void, buf: *const c_char, buildflags: *const c_char, ocvrs_return: *mut Result<bool>);
	// write(cv::String &) /usr/include/opencv2/core/ocl.hpp:586
	pub fn cv_ocl_Program_write_const_StringR(instance: *const c_void, buf: *mut *mut c_void, ocvrs_return: *mut Result<bool>);
	// source() /usr/include/opencv2/core/ocl.hpp:587
	pub fn cv_ocl_Program_source_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getPrefix() /usr/include/opencv2/core/ocl.hpp:588
	pub fn cv_ocl_Program_getPrefix_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getPrefix(const cv::String &) /usr/include/opencv2/core/ocl.hpp:589
	pub fn cv_ocl_Program_getPrefix_const_StringR(buildflags: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// ProgramSource() /usr/include/opencv2/core/ocl.hpp:599
	pub fn cv_ocl_ProgramSource_ProgramSource() -> *mut c_void;
	// ProgramSource(const cv::String &, const cv::String &, const cv::String &, const cv::String &) /usr/include/opencv2/core/ocl.hpp:600
	pub fn cv_ocl_ProgramSource_ProgramSource_const_StringR_const_StringR_const_StringR_const_StringR(module: *const c_char, name: *const c_char, code_str: *const c_char, code_hash: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// ProgramSource(const cv::String &) /usr/include/opencv2/core/ocl.hpp:601
	pub fn cv_ocl_ProgramSource_ProgramSource_const_StringR(prog: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// ProgramSource(const cv::ocl::ProgramSource &) /usr/include/opencv2/core/ocl.hpp:604
	pub fn cv_ocl_ProgramSource_ProgramSource_const_ProgramSourceR(prog: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// ProgramSource(cv::ocl::ProgramSource &&) /usr/include/opencv2/core/ocl.hpp:606
	pub fn cv_ocl_ProgramSource_ProgramSource_ProgramSourceR(prog: *mut c_void) -> *mut c_void;
	// source() /usr/include/opencv2/core/ocl.hpp:609
	pub fn cv_ocl_ProgramSource_source_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// hash() /usr/include/opencv2/core/ocl.hpp:610
	pub fn cv_ocl_ProgramSource_hash_const(instance: *const c_void, ocvrs_return: *mut Result<core::ProgramSource_hash_t>);
	// fromBinary(const cv::String &, const cv::String &, const unsigned char *, const size_t, const cv::String &) /usr/include/opencv2/core/ocl.hpp:627
	pub fn cv_ocl_ProgramSource_fromBinary_const_StringR_const_StringR_const_unsigned_charX_const_size_t_const_StringR(module: *const c_char, name: *const c_char, binary: *const u8, size: size_t, build_options: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// fromSPIR(const cv::String &, const cv::String &, const unsigned char *, const size_t, const cv::String &) /usr/include/opencv2/core/ocl.hpp:652
	pub fn cv_ocl_ProgramSource_fromSPIR_const_StringR_const_StringR_const_unsigned_charX_const_size_t_const_StringR(module: *const c_char, name: *const c_char, binary: *const u8, size: size_t, build_options: *const c_char, ocvrs_return: *mut Result<*mut c_void>);
	// empty() /usr/include/opencv2/core/ocl.hpp:663
	pub fn cv_ocl_ProgramSource_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// Queue() /usr/include/opencv2/core/ocl.hpp:393
	pub fn cv_ocl_Queue_Queue() -> *mut c_void;
	// Queue(const cv::ocl::Context &, const cv::ocl::Device &) /usr/include/opencv2/core/ocl.hpp:394
	pub fn cv_ocl_Queue_Queue_const_ContextR_const_DeviceR(c: *const c_void, d: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Queue(const cv::ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:396
	pub fn cv_ocl_Queue_Queue_const_QueueR(q: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// Queue(cv::ocl::Queue &&) /usr/include/opencv2/core/ocl.hpp:398
	pub fn cv_ocl_Queue_Queue_QueueR(q: *mut c_void) -> *mut c_void;
	// create(const cv::ocl::Context &, const cv::ocl::Device &) /usr/include/opencv2/core/ocl.hpp:401
	pub fn cv_ocl_Queue_create_const_ContextR_const_DeviceR(instance: *mut c_void, c: *const c_void, d: *const c_void, ocvrs_return: *mut Result<bool>);
	// finish() /usr/include/opencv2/core/ocl.hpp:402
	pub fn cv_ocl_Queue_finish(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// ptr() /usr/include/opencv2/core/ocl.hpp:403
	pub fn cv_ocl_Queue_ptr_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// getDefault() /usr/include/opencv2/core/ocl.hpp:404
	pub fn cv_ocl_Queue_getDefault(ocvrs_return: *mut Result<*mut c_void>);
	// getProfilingQueue() /usr/include/opencv2/core/ocl.hpp:407
	pub fn cv_ocl_Queue_getProfilingQueue_const(instance: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// empty() /usr/include/opencv2/core/ocl.hpp:411
	pub fn cv_ocl_Queue_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// Timer(const cv::ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:776
	pub fn cv_ocl_Timer_Timer_const_QueueR(q: *const c_void, ocvrs_return: *mut Result<*mut c_void>);
	// start() /usr/include/opencv2/core/ocl.hpp:778
	pub fn cv_ocl_Timer_start(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// stop() /usr/include/opencv2/core/ocl.hpp:779
	pub fn cv_ocl_Timer_stop(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// durationNS() /usr/include/opencv2/core/ocl.hpp:781
	pub fn cv_ocl_Timer_durationNS_const(instance: *const c_void, ocvrs_return: *mut Result<u64>);
	// Arrays() /usr/include/opencv2/core/opengl.hpp:411
	pub fn cv_ogl_Arrays_Arrays(ocvrs_return: *mut Result<*mut c_void>);
	// setVertexArray(cv::InputArray) /usr/include/opencv2/core/opengl.hpp:416
	pub fn cv_ogl_Arrays_setVertexArray_const__InputArrayR(instance: *mut c_void, vertex: *const c_void, ocvrs_return: *mut Result_void);
	// resetVertexArray() /usr/include/opencv2/core/opengl.hpp:420
	pub fn cv_ogl_Arrays_resetVertexArray(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// setColorArray(cv::InputArray) /usr/include/opencv2/core/opengl.hpp:425
	pub fn cv_ogl_Arrays_setColorArray_const__InputArrayR(instance: *mut c_void, color: *const c_void, ocvrs_return: *mut Result_void);
	// resetColorArray() /usr/include/opencv2/core/opengl.hpp:429
	pub fn cv_ogl_Arrays_resetColorArray(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// setNormalArray(cv::InputArray) /usr/include/opencv2/core/opengl.hpp:434
	pub fn cv_ogl_Arrays_setNormalArray_const__InputArrayR(instance: *mut c_void, normal: *const c_void, ocvrs_return: *mut Result_void);
	// resetNormalArray() /usr/include/opencv2/core/opengl.hpp:438
	pub fn cv_ogl_Arrays_resetNormalArray(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// setTexCoordArray(cv::InputArray) /usr/include/opencv2/core/opengl.hpp:443
	pub fn cv_ogl_Arrays_setTexCoordArray_const__InputArrayR(instance: *mut c_void, tex_coord: *const c_void, ocvrs_return: *mut Result_void);
	// resetTexCoordArray() /usr/include/opencv2/core/opengl.hpp:447
	pub fn cv_ogl_Arrays_resetTexCoordArray(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// release() /usr/include/opencv2/core/opengl.hpp:451
	pub fn cv_ogl_Arrays_release(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// setAutoRelease(bool) /usr/include/opencv2/core/opengl.hpp:456
	pub fn cv_ogl_Arrays_setAutoRelease_bool(instance: *mut c_void, flag: bool, ocvrs_return: *mut Result_void);
	// bind() /usr/include/opencv2/core/opengl.hpp:460
	pub fn cv_ogl_Arrays_bind_const(instance: *const c_void, ocvrs_return: *mut Result_void);
	// size() /usr/include/opencv2/core/opengl.hpp:464
	pub fn cv_ogl_Arrays_size_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// empty() /usr/include/opencv2/core/opengl.hpp:465
	pub fn cv_ogl_Arrays_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// Buffer() /usr/include/opencv2/core/opengl.hpp:104
	pub fn cv_ogl_Buffer_Buffer(ocvrs_return: *mut Result<*mut c_void>);
	// Buffer(int, int, int, unsigned int, bool) /usr/include/opencv2/core/opengl.hpp:113
	pub fn cv_ogl_Buffer_Buffer_int_int_int_unsigned_int_bool(arows: i32, acols: i32, atype: i32, abuf_id: u32, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
	// Buffer(cv::Size, int, unsigned int, bool) /usr/include/opencv2/core/opengl.hpp:121
	pub fn cv_ogl_Buffer_Buffer_Size_int_unsigned_int_bool(asize: *const core::Size, atype: i32, abuf_id: u32, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
	// Buffer(int, int, int, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:130
	pub fn cv_ogl_Buffer_Buffer_int_int_int_Target_bool(arows: i32, acols: i32, atype: i32, target: core::Buffer_Target, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
	// Buffer(cv::Size, int, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:138
	pub fn cv_ogl_Buffer_Buffer_Size_int_Target_bool(asize: *const core::Size, atype: i32, target: core::Buffer_Target, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
	// Buffer(cv::InputArray, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:145
	pub fn cv_ogl_Buffer_Buffer_const__InputArrayR_Target_bool(arr: *const c_void, target: core::Buffer_Target, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, int, int, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:155
	pub fn cv_ogl_Buffer_create_int_int_int_Target_bool(instance: *mut c_void, arows: i32, acols: i32, atype: i32, target: core::Buffer_Target, auto_release: bool, ocvrs_return: *mut Result_void);
	// create(cv::Size, int, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:163
	pub fn cv_ogl_Buffer_create_Size_int_Target_bool(instance: *mut c_void, asize: *const core::Size, atype: i32, target: core::Buffer_Target, auto_release: bool, ocvrs_return: *mut Result_void);
	// release() /usr/include/opencv2/core/opengl.hpp:169
	pub fn cv_ogl_Buffer_release(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// setAutoRelease(bool) /usr/include/opencv2/core/opengl.hpp:180
	pub fn cv_ogl_Buffer_setAutoRelease_bool(instance: *mut c_void, flag: bool, ocvrs_return: *mut Result_void);
	// copyFrom(cv::InputArray, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:187
	pub fn cv_ogl_Buffer_copyFrom_const__InputArrayR_Target_bool(instance: *mut c_void, arr: *const c_void, target: core::Buffer_Target, auto_release: bool, ocvrs_return: *mut Result_void);
	// copyFrom(cv::InputArray, cuda::Stream &, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:190
	pub fn cv_ogl_Buffer_copyFrom_const__InputArrayR_StreamR_Target_bool(instance: *mut c_void, arr: *const c_void, stream: *mut c_void, target: core::Buffer_Target, auto_release: bool, ocvrs_return: *mut Result_void);
	// copyTo(cv::OutputArray) /usr/include/opencv2/core/opengl.hpp:197
	pub fn cv_ogl_Buffer_copyTo_const_const__OutputArrayR(instance: *const c_void, arr: *const c_void, ocvrs_return: *mut Result_void);
	// copyTo(cv::OutputArray, cuda::Stream &) /usr/include/opencv2/core/opengl.hpp:200
	pub fn cv_ogl_Buffer_copyTo_const_const__OutputArrayR_StreamR(instance: *const c_void, arr: *const c_void, stream: *mut c_void, ocvrs_return: *mut Result_void);
	// clone(cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:207
	pub fn cv_ogl_Buffer_clone_const_Target_bool(instance: *const c_void, target: core::Buffer_Target, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
	// bind(cv::ogl::Buffer::Target) /usr/include/opencv2/core/opengl.hpp:213
	pub fn cv_ogl_Buffer_bind_const_Target(instance: *const c_void, target: core::Buffer_Target, ocvrs_return: *mut Result_void);
	// unbind(cv::ogl::Buffer::Target) /usr/include/opencv2/core/opengl.hpp:219
	pub fn cv_ogl_Buffer_unbind_Target(target: core::Buffer_Target, ocvrs_return: *mut Result_void);
	// mapHost(cv::ogl::Buffer::Access) /usr/include/opencv2/core/opengl.hpp:236
	pub fn cv_ogl_Buffer_mapHost_Access(instance: *mut c_void, access: core::Buffer_Access, ocvrs_return: *mut Result<*mut c_void>);
	// unmapHost() /usr/include/opencv2/core/opengl.hpp:240
	pub fn cv_ogl_Buffer_unmapHost(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// mapDevice() /usr/include/opencv2/core/opengl.hpp:243
	pub fn cv_ogl_Buffer_mapDevice(instance: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// unmapDevice() /usr/include/opencv2/core/opengl.hpp:244
	pub fn cv_ogl_Buffer_unmapDevice(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// mapDevice(cuda::Stream &) /usr/include/opencv2/core/opengl.hpp:252
	pub fn cv_ogl_Buffer_mapDevice_StreamR(instance: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result<*mut c_void>);
	// unmapDevice(cuda::Stream &) /usr/include/opencv2/core/opengl.hpp:256
	pub fn cv_ogl_Buffer_unmapDevice_StreamR(instance: *mut c_void, stream: *mut c_void, ocvrs_return: *mut Result_void);
	// rows() /usr/include/opencv2/core/opengl.hpp:258
	pub fn cv_ogl_Buffer_rows_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// cols() /usr/include/opencv2/core/opengl.hpp:259
	pub fn cv_ogl_Buffer_cols_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// size() /usr/include/opencv2/core/opengl.hpp:260
	pub fn cv_ogl_Buffer_size_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// empty() /usr/include/opencv2/core/opengl.hpp:261
	pub fn cv_ogl_Buffer_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// type() /usr/include/opencv2/core/opengl.hpp:263
	pub fn cv_ogl_Buffer_type_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// depth() /usr/include/opencv2/core/opengl.hpp:264
	pub fn cv_ogl_Buffer_depth_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// channels() /usr/include/opencv2/core/opengl.hpp:265
	pub fn cv_ogl_Buffer_channels_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// elemSize() /usr/include/opencv2/core/opengl.hpp:266
	pub fn cv_ogl_Buffer_elemSize_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// elemSize1() /usr/include/opencv2/core/opengl.hpp:267
	pub fn cv_ogl_Buffer_elemSize1_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// bufId() /usr/include/opencv2/core/opengl.hpp:270
	pub fn cv_ogl_Buffer_bufId_const(instance: *const c_void, ocvrs_return: *mut Result<u32>);
	// Texture2D() /usr/include/opencv2/core/opengl.hpp:301
	pub fn cv_ogl_Texture2D_Texture2D(ocvrs_return: *mut Result<*mut c_void>);
	// Texture2D(int, int, cv::ogl::Texture2D::Format, unsigned int, bool) /usr/include/opencv2/core/opengl.hpp:304
	pub fn cv_ogl_Texture2D_Texture2D_int_int_Format_unsigned_int_bool(arows: i32, acols: i32, aformat: core::Texture2D_Format, atex_id: u32, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
	// Texture2D(cv::Size, cv::ogl::Texture2D::Format, unsigned int, bool) /usr/include/opencv2/core/opengl.hpp:307
	pub fn cv_ogl_Texture2D_Texture2D_Size_Format_unsigned_int_bool(asize: *const core::Size, aformat: core::Texture2D_Format, atex_id: u32, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
	// Texture2D(int, int, cv::ogl::Texture2D::Format, bool) /usr/include/opencv2/core/opengl.hpp:315
	pub fn cv_ogl_Texture2D_Texture2D_int_int_Format_bool(arows: i32, acols: i32, aformat: core::Texture2D_Format, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
	// Texture2D(cv::Size, cv::ogl::Texture2D::Format, bool) /usr/include/opencv2/core/opengl.hpp:322
	pub fn cv_ogl_Texture2D_Texture2D_Size_Format_bool(asize: *const core::Size, aformat: core::Texture2D_Format, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
	// Texture2D(cv::InputArray, bool) /usr/include/opencv2/core/opengl.hpp:328
	pub fn cv_ogl_Texture2D_Texture2D_const__InputArrayR_bool(arr: *const c_void, auto_release: bool, ocvrs_return: *mut Result<*mut c_void>);
	// create(int, int, cv::ogl::Texture2D::Format, bool) /usr/include/opencv2/core/opengl.hpp:337
	pub fn cv_ogl_Texture2D_create_int_int_Format_bool(instance: *mut c_void, arows: i32, acols: i32, aformat: core::Texture2D_Format, auto_release: bool, ocvrs_return: *mut Result_void);
	// create(cv::Size, cv::ogl::Texture2D::Format, bool) /usr/include/opencv2/core/opengl.hpp:343
	pub fn cv_ogl_Texture2D_create_Size_Format_bool(instance: *mut c_void, asize: *const core::Size, aformat: core::Texture2D_Format, auto_release: bool, ocvrs_return: *mut Result_void);
	// release() /usr/include/opencv2/core/opengl.hpp:349
	pub fn cv_ogl_Texture2D_release(instance: *mut c_void, ocvrs_return: *mut Result_void);
	// setAutoRelease(bool) /usr/include/opencv2/core/opengl.hpp:361
	pub fn cv_ogl_Texture2D_setAutoRelease_bool(instance: *mut c_void, flag: bool, ocvrs_return: *mut Result_void);
	// copyFrom(cv::InputArray, bool) /usr/include/opencv2/core/opengl.hpp:368
	pub fn cv_ogl_Texture2D_copyFrom_const__InputArrayR_bool(instance: *mut c_void, arr: *const c_void, auto_release: bool, ocvrs_return: *mut Result_void);
	// copyTo(cv::OutputArray, int, bool) /usr/include/opencv2/core/opengl.hpp:377
	pub fn cv_ogl_Texture2D_copyTo_const_const__OutputArrayR_int_bool(instance: *const c_void, arr: *const c_void, ddepth: i32, auto_release: bool, ocvrs_return: *mut Result_void);
	// bind() /usr/include/opencv2/core/opengl.hpp:381
	pub fn cv_ogl_Texture2D_bind_const(instance: *const c_void, ocvrs_return: *mut Result_void);
	// rows() /usr/include/opencv2/core/opengl.hpp:383
	pub fn cv_ogl_Texture2D_rows_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// cols() /usr/include/opencv2/core/opengl.hpp:384
	pub fn cv_ogl_Texture2D_cols_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// size() /usr/include/opencv2/core/opengl.hpp:385
	pub fn cv_ogl_Texture2D_size_const(instance: *const c_void, ocvrs_return: *mut Result<core::Size>);
	// empty() /usr/include/opencv2/core/opengl.hpp:386
	pub fn cv_ogl_Texture2D_empty_const(instance: *const c_void, ocvrs_return: *mut Result<bool>);
	// format() /usr/include/opencv2/core/opengl.hpp:388
	pub fn cv_ogl_Texture2D_format_const(instance: *const c_void, ocvrs_return: *mut Result<core::Texture2D_Format>);
	// texId() /usr/include/opencv2/core/opengl.hpp:391
	pub fn cv_ogl_Texture2D_texId_const(instance: *const c_void, ocvrs_return: *mut Result<u32>);
	// name /usr/include/opencv2/core/utils/logtag.hpp:17
	pub fn cv_utils_logging_LogTag_getPropName_const(instance: *const c_void) -> *mut c_void;
	// level /usr/include/opencv2/core/utils/logtag.hpp:18
	pub fn cv_utils_logging_LogTag_getPropLevel_const(instance: *const c_void, ocvrs_return: *mut core::LogLevel);
	// level /usr/include/opencv2/core/utils/logtag.hpp:18
	pub fn cv_utils_logging_LogTag_setPropLevel_LogLevel(instance: *mut c_void, val: core::LogLevel);
	// LogTag(const char *, cv::utils::logging::LogLevel) /usr/include/opencv2/core/utils/logtag.hpp:20
	pub fn cv_utils_logging_LogTag_LogTag_const_charX_LogLevel(_name: *const c_char, _level: core::LogLevel, ocvrs_return: *mut Result<*mut c_void>);
	// OriginalClassName(const OriginalClassName::Params &) /usr/include/opencv2/core/bindings_utils.hpp:242
	pub fn cv_utils_nested_OriginalClassName_OriginalClassName_const_ParamsR(params: *const core::OriginalClassName_Params, ocvrs_return: *mut Result<*mut c_void>);
	// getIntParam() /usr/include/opencv2/core/bindings_utils.hpp:247
	pub fn cv_utils_nested_OriginalClassName_getIntParam_const(instance: *const c_void, ocvrs_return: *mut Result<i32>);
	// getFloatParam() /usr/include/opencv2/core/bindings_utils.hpp:252
	pub fn cv_utils_nested_OriginalClassName_getFloatParam_const(instance: *const c_void, ocvrs_return: *mut Result<f32>);
	// originalName() /usr/include/opencv2/core/bindings_utils.hpp:257
	pub fn cv_utils_nested_OriginalClassName_originalName(ocvrs_return: *mut Result<*mut c_void>);
	// create(const OriginalClassName::Params &) /usr/include/opencv2/core/bindings_utils.hpp:263
	pub fn cv_utils_nested_OriginalClassName_create_const_ParamsR(params: *const core::OriginalClassName_Params, ocvrs_return: *mut Result<*mut c_void>);
	// Params(int, float) /usr/include/opencv2/core/bindings_utils.hpp:235
	pub fn cv_utils_nested_OriginalClassName_Params_Params_int_float(int_param: i32, float_param: f32, ocvrs_return: *mut Result<core::OriginalClassName_Params>);
}
