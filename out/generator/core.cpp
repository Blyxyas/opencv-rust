#include "core.hpp"
#include "core_types.hpp"

extern "C" {
	// Cholesky(double *, size_t, int, double *, size_t, int) /usr/include/opencv2/core/base.hpp:567
	void cv_Cholesky_doubleX_size_t_int_doubleX_size_t_int(double* A, size_t astep, int m, double* b, size_t bstep, int n, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::Cholesky(A, astep, m, b, bstep, n);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// Cholesky(float *, size_t, int, float *, size_t, int) /usr/include/opencv2/core/base.hpp:565
	void cv_Cholesky_floatX_size_t_int_floatX_size_t_int(float* A, size_t astep, int m, float* b, size_t bstep, int n, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::Cholesky(A, astep, m, b, bstep, n);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// LUT(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:563
	void cv_LUT_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_InputArray* lut, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::LUT(*src, *lut, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// LU(double *, size_t, int, double *, size_t, int) /usr/include/opencv2/core/base.hpp:563
	void cv_LU_doubleX_size_t_int_doubleX_size_t_int(double* A, size_t astep, int m, double* b, size_t bstep, int n, Result<int>* ocvrs_return) {
		try {
			int ret = cv::LU(A, astep, m, b, bstep, n);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// LU(float *, size_t, int, float *, size_t, int) /usr/include/opencv2/core/base.hpp:561
	void cv_LU_floatX_size_t_int_floatX_size_t_int(float* A, size_t astep, int m, float* b, size_t bstep, int n, Result<int>* ocvrs_return) {
		try {
			int ret = cv::LU(A, astep, m, b, bstep, n);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// Mahalanobis(cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/core.hpp:2083
	void cv_Mahalanobis_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* v1, const cv::_InputArray* v2, const cv::_InputArray* icovar, Result<double>* ocvrs_return) {
		try {
			double ret = cv::Mahalanobis(*v1, *v2, *icovar);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// PCABackProject(cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2063
	void cv_PCABackProject_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* data, const cv::_InputArray* mean, const cv::_InputArray* eigenvectors, const cv::_OutputArray* result, Result_void* ocvrs_return) {
		try {
			cv::PCABackProject(*data, *mean, *eigenvectors, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// PCACompute(cv::InputArray, cv::InputOutputArray, cv::OutputArray, cv::OutputArray, double) /usr/include/opencv2/core.hpp:2054
	void cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_double(const cv::_InputArray* data, const cv::_InputOutputArray* mean, const cv::_OutputArray* eigenvectors, const cv::_OutputArray* eigenvalues, double retainedVariance, Result_void* ocvrs_return) {
		try {
			cv::PCACompute(*data, *mean, *eigenvectors, *eigenvalues, retainedVariance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// PCACompute(cv::InputArray, cv::InputOutputArray, cv::OutputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2045
	void cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_const__OutputArrayR_int(const cv::_InputArray* data, const cv::_InputOutputArray* mean, const cv::_OutputArray* eigenvectors, const cv::_OutputArray* eigenvalues, int maxComponents, Result_void* ocvrs_return) {
		try {
			cv::PCACompute(*data, *mean, *eigenvectors, *eigenvalues, maxComponents);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// PCACompute(cv::InputArray, cv::InputOutputArray, cv::OutputArray, double) /usr/include/opencv2/core.hpp:2050
	void cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_double(const cv::_InputArray* data, const cv::_InputOutputArray* mean, const cv::_OutputArray* eigenvectors, double retainedVariance, Result_void* ocvrs_return) {
		try {
			cv::PCACompute(*data, *mean, *eigenvectors, retainedVariance);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// PCACompute(cv::InputArray, cv::InputOutputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2041
	void cv_PCACompute_const__InputArrayR_const__InputOutputArrayR_const__OutputArrayR_int(const cv::_InputArray* data, const cv::_InputOutputArray* mean, const cv::_OutputArray* eigenvectors, int maxComponents, Result_void* ocvrs_return) {
		try {
			cv::PCACompute(*data, *mean, *eigenvectors, maxComponents);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// PCAProject(cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2059
	void cv_PCAProject_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* data, const cv::_InputArray* mean, const cv::_InputArray* eigenvectors, const cv::_OutputArray* result, Result_void* ocvrs_return) {
		try {
			cv::PCAProject(*data, *mean, *eigenvectors, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// PSNR(cv::InputArray, cv::InputArray, double) /usr/include/opencv2/core.hpp:723
	void cv_PSNR_const__InputArrayR_const__InputArrayR_double(const cv::_InputArray* src1, const cv::_InputArray* src2, double R, Result<double>* ocvrs_return) {
		try {
			double ret = cv::PSNR(*src1, *src2, R);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// SVBackSubst(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2070
	void cv_SVBackSubst_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* w, const cv::_InputArray* u, const cv::_InputArray* vt, const cv::_InputArray* rhs, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::SVBackSubst(*w, *u, *vt, *rhs, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// SVDecomp(cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2067
	void cv_SVDecomp_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* w, const cv::_OutputArray* u, const cv::_OutputArray* vt, int flags, Result_void* ocvrs_return) {
		try {
			cv::SVDecomp(*src, *w, *u, *vt, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// abs(const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3768
	void cv_abs_const_MatExprR(const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::abs(*e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// abs(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3764
	void cv_abs_const_MatR(const cv::Mat* m, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::abs(*m);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// absdiff(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1397
	void cv_absdiff_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::absdiff(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// addWeighted(cv::InputArray, double, cv::InputArray, double, double, cv::OutputArray, int) /usr/include/opencv2/core.hpp:506
	void cv_addWeighted_const__InputArrayR_double_const__InputArrayR_double_double_const__OutputArrayR_int(const cv::_InputArray* src1, double alpha, const cv::_InputArray* src2, double beta, double gamma, const cv::_OutputArray* dst, int dtype, Result_void* ocvrs_return) {
		try {
			cv::addWeighted(*src1, alpha, *src2, beta, gamma, *dst, dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// add(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray, int) /usr/include/opencv2/core.hpp:360
	void cv_add_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, int dtype, Result_void* ocvrs_return) {
		try {
			cv::add(*src1, *src2, *dst, *mask, dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// batchDistance(cv::InputArray, cv::InputArray, cv::OutputArray, int, cv::OutputArray, int, int, cv::InputArray, int, bool) /usr/include/opencv2/core.hpp:730
	void cv_batchDistance_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_const__OutputArrayR_int_int_const__InputArrayR_int_bool(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dist, int dtype, const cv::_OutputArray* nidx, int normType, int K, const cv::_InputArray* mask, int update, bool crosscheck, Result_void* ocvrs_return) {
		try {
			cv::batchDistance(*src1, *src2, *dist, dtype, *nidx, normType, K, *mask, update, crosscheck);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// bitwise_and(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1299
	void cv_bitwise_and_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::bitwise_and(*src1, *src2, *dst, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// bitwise_not(cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1371
	void cv_bitwise_not_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::bitwise_not(*src, *dst, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// bitwise_or(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1326
	void cv_bitwise_or_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::bitwise_or(*src1, *src2, *dst, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// bitwise_xor(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1354
	void cv_bitwise_xor_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::bitwise_xor(*src1, *src2, *dst, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// borderInterpolate(int, int, int) /usr/include/opencv2/core.hpp:270
	void cv_borderInterpolate_int_int_int(int p, int len, int borderType, Result<int>* ocvrs_return) {
		try {
			int ret = cv::borderInterpolate(p, len, borderType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// calcCovarMatrix(cv::InputArray, cv::OutputArray, cv::InputOutputArray, int, int) /usr/include/opencv2/core.hpp:2037
	void cv_calcCovarMatrix_const__InputArrayR_const__OutputArrayR_const__InputOutputArrayR_int_int(const cv::_InputArray* samples, const cv::_OutputArray* covar, const cv::_InputOutputArray* mean, int flags, int ctype, Result_void* ocvrs_return) {
		try {
			cv::calcCovarMatrix(*samples, *covar, *mean, flags, ctype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// cartToPolar(cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray, bool) /usr/include/opencv2/core.hpp:1608
	void cv_cartToPolar_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* magnitude, const cv::_OutputArray* angle, bool angleInDegrees, Result_void* ocvrs_return) {
		try {
			cv::cartToPolar(*x, *y, *magnitude, *angle, angleInDegrees);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// checkHardwareSupport(int) /usr/include/opencv2/core/utility.hpp:425
	void cv_checkHardwareSupport_int(int feature, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::checkHardwareSupport(feature);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// checkRange(cv::InputArray, bool, cv::Point *, double, double) /usr/include/opencv2/core.hpp:1659
	void cv_checkRange_const__InputArrayR_bool_PointX_double_double(const cv::_InputArray* a, bool quiet, cv::Point* pos, double minVal, double maxVal, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::checkRange(*a, quiet, pos, minVal, maxVal);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// compare(cv::InputArray, cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1458
	void cv_compare_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int cmpop, Result_void* ocvrs_return) {
		try {
			cv::compare(*src1, *src2, *dst, cmpop);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// completeSymm(cv::InputOutputArray, bool) /usr/include/opencv2/core.hpp:1819
	void cv_completeSymm_const__InputOutputArrayR_bool(const cv::_InputOutputArray* m, bool lowerToUpper, Result_void* ocvrs_return) {
		try {
			cv::completeSymm(*m, lowerToUpper);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// convertFp16(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:547
	void cv_convertFp16_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::convertFp16(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// convertScaleAbs(cv::InputArray, cv::OutputArray, double, double) /usr/include/opencv2/core.hpp:534
	void cv_convertScaleAbs_const__InputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* src, const cv::_OutputArray* dst, double alpha, double beta, Result_void* ocvrs_return) {
		try {
			cv::convertScaleAbs(*src, *dst, alpha, beta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// copyMakeBorder(cv::InputArray, cv::OutputArray, int, int, int, int, int, const cv::Scalar &) /usr/include/opencv2/core.hpp:320
	void cv_copyMakeBorder_const__InputArrayR_const__OutputArrayR_int_int_int_int_int_const_ScalarR(const cv::_InputArray* src, const cv::_OutputArray* dst, int top, int bottom, int left, int right, int borderType, const cv::Scalar* value, Result_void* ocvrs_return) {
		try {
			cv::copyMakeBorder(*src, *dst, top, bottom, left, right, borderType, *value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// copyTo(cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1409
	void cv_copyTo_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::copyTo(*src, *dst, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// countNonZero(cv::InputArray) /usr/include/opencv2/core.hpp:581
	void cv_countNonZero_const__InputArrayR(const cv::_InputArray* src, Result<int>* ocvrs_return) {
		try {
			int ret = cv::countNonZero(*src);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// cubeRoot(float) /usr/include/opencv2/core/base.hpp:539
	void cv_cubeRoot_float(float val, Result<float>* ocvrs_return) {
		try {
			float ret = cv::cubeRoot(val);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// createContinuous(int, int, int, cv::OutputArray) /usr/include/opencv2/core/cuda.hpp:557
	void cv_cuda_createContinuous_int_int_int_const__OutputArrayR(int rows, int cols, int type, const cv::_OutputArray* arr, Result_void* ocvrs_return) {
		try {
			cv::cuda::createContinuous(rows, cols, type, *arr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// deviceSupports(cv::cuda::FeatureSet) /usr/include/opencv2/core/cuda.hpp:1010
	void cv_cuda_deviceSupports_FeatureSet(cv::cuda::FeatureSet feature_set, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::cuda::deviceSupports(feature_set);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// ensureSizeIsEnough(int, int, int, cv::OutputArray) /usr/include/opencv2/core/cuda.hpp:568
	void cv_cuda_ensureSizeIsEnough_int_int_int_const__OutputArrayR(int rows, int cols, int type, const cv::_OutputArray* arr, Result_void* ocvrs_return) {
		try {
			cv::cuda::ensureSizeIsEnough(rows, cols, type, *arr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getCudaEnabledDeviceCount() /usr/include/opencv2/core/cuda.hpp:966
	void cv_cuda_getCudaEnabledDeviceCount(Result<int>* ocvrs_return) {
		try {
			int ret = cv::cuda::getCudaEnabledDeviceCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getDevice() /usr/include/opencv2/core/cuda.hpp:978
	void cv_cuda_getDevice(Result<int>* ocvrs_return) {
		try {
			int ret = cv::cuda::getDevice();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// printCudaDeviceInfo(int) /usr/include/opencv2/core/cuda.hpp:1252
	void cv_cuda_printCudaDeviceInfo_int(int device, Result_void* ocvrs_return) {
		try {
			cv::cuda::printCudaDeviceInfo(device);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// printShortCudaDeviceInfo(int) /usr/include/opencv2/core/cuda.hpp:1253
	void cv_cuda_printShortCudaDeviceInfo_int(int device, Result_void* ocvrs_return) {
		try {
			cv::cuda::printShortCudaDeviceInfo(device);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// registerPageLocked(cv::Mat &) /usr/include/opencv2/core/cuda.hpp:809
	void cv_cuda_registerPageLocked_MatR(cv::Mat* m, Result_void* ocvrs_return) {
		try {
			cv::cuda::registerPageLocked(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// resetDevice() /usr/include/opencv2/core/cuda.hpp:985
	void cv_cuda_resetDevice(Result_void* ocvrs_return) {
		try {
			cv::cuda::resetDevice();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setBufferPoolConfig(int, size_t, int) /usr/include/opencv2/core/cuda.hpp:709
	void cv_cuda_setBufferPoolConfig_int_size_t_int(int deviceId, size_t stackSize, int stackCount, Result_void* ocvrs_return) {
		try {
			cv::cuda::setBufferPoolConfig(deviceId, stackSize, stackCount);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setBufferPoolUsage(bool) /usr/include/opencv2/core/cuda.hpp:708
	void cv_cuda_setBufferPoolUsage_bool(bool on, Result_void* ocvrs_return) {
		try {
			cv::cuda::setBufferPoolUsage(on);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setDevice(int) /usr/include/opencv2/core/cuda.hpp:974
	void cv_cuda_setDevice_int(int device, Result_void* ocvrs_return) {
		try {
			cv::cuda::setDevice(device);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setGlDevice(int) /usr/include/opencv2/core/opengl.hpp:572
	void cv_cuda_setGlDevice_int(int device, Result_void* ocvrs_return) {
		try {
			cv::cuda::setGlDevice(device);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// unregisterPageLocked(cv::Mat &) /usr/include/opencv2/core/cuda.hpp:815
	void cv_cuda_unregisterPageLocked_MatR(cv::Mat* m, Result_void* ocvrs_return) {
		try {
			cv::cuda::unregisterPageLocked(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dct(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2272
	void cv_dct_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags, Result_void* ocvrs_return) {
		try {
			cv::dct(*src, *dst, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// depthToString(int) /usr/include/opencv2/core/check.hpp:13
	void cv_depthToString_int(int depth, Result<void*>* ocvrs_return) {
		try {
			const char* ret = cv::depthToString(depth);
			Ok(ocvrs_create_string(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// check_failed_MatChannels(const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:85
	void cv_detail_check_failed_MatChannels_const_int_const_CheckContextR(const int v, const cv::detail::CheckContext* ctx, Result_void* ocvrs_return) {
		try {
			cv::detail::check_failed_MatChannels(v, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// check_failed_MatChannels(const int, const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:75
	void cv_detail_check_failed_MatChannels_const_int_const_int_const_CheckContextR(const int v1, const int v2, const cv::detail::CheckContext* ctx, Result_void* ocvrs_return) {
		try {
			cv::detail::check_failed_MatChannels(v1, v2, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// check_failed_MatDepth(const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:83
	void cv_detail_check_failed_MatDepth_const_int_const_CheckContextR(const int v, const cv::detail::CheckContext* ctx, Result_void* ocvrs_return) {
		try {
			cv::detail::check_failed_MatDepth(v, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// check_failed_MatDepth(const int, const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:73
	void cv_detail_check_failed_MatDepth_const_int_const_int_const_CheckContextR(const int v1, const int v2, const cv::detail::CheckContext* ctx, Result_void* ocvrs_return) {
		try {
			cv::detail::check_failed_MatDepth(v1, v2, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// check_failed_MatType(const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:84
	void cv_detail_check_failed_MatType_const_int_const_CheckContextR(const int v, const cv::detail::CheckContext* ctx, Result_void* ocvrs_return) {
		try {
			cv::detail::check_failed_MatType(v, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// check_failed_MatType(const int, const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:74
	void cv_detail_check_failed_MatType_const_int_const_int_const_CheckContextR(const int v1, const int v2, const cv::detail::CheckContext* ctx, Result_void* ocvrs_return) {
		try {
			cv::detail::check_failed_MatType(v1, v2, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// check_failed_auto(const Size_<int>, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:81
	void cv_detail_check_failed_auto_const_Size__int__const_CheckContextR(const cv::Size_<int>* v, const cv::detail::CheckContext* ctx, Result_void* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(*v, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// check_failed_auto(const Size_<int>, const Size_<int>, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:72
	void cv_detail_check_failed_auto_const_Size__int__const_Size__int__const_CheckContextR(const cv::Size_<int>* v1, const cv::Size_<int>* v2, const cv::detail::CheckContext* ctx, Result_void* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(*v1, *v2, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// check_failed_auto(const double, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:80
	void cv_detail_check_failed_auto_const_double_const_CheckContextR(const double v, const cv::detail::CheckContext* ctx, Result_void* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(v, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// check_failed_auto(const double, const double, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:71
	void cv_detail_check_failed_auto_const_double_const_double_const_CheckContextR(const double v1, const double v2, const cv::detail::CheckContext* ctx, Result_void* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(v1, v2, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// check_failed_auto(const float, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:79
	void cv_detail_check_failed_auto_const_float_const_CheckContextR(const float v, const cv::detail::CheckContext* ctx, Result_void* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(v, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// check_failed_auto(const float, const float, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:70
	void cv_detail_check_failed_auto_const_float_const_float_const_CheckContextR(const float v1, const float v2, const cv::detail::CheckContext* ctx, Result_void* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(v1, v2, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// check_failed_auto(const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:77
	void cv_detail_check_failed_auto_const_int_const_CheckContextR(const int v, const cv::detail::CheckContext* ctx, Result_void* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(v, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// check_failed_auto(const int, const int, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:68
	void cv_detail_check_failed_auto_const_int_const_int_const_CheckContextR(const int v1, const int v2, const cv::detail::CheckContext* ctx, Result_void* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(v1, v2, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// check_failed_auto(const size_t, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:78
	void cv_detail_check_failed_auto_const_size_t_const_CheckContextR(const size_t v, const cv::detail::CheckContext* ctx, Result_void* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(v, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// check_failed_auto(const size_t, const size_t, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:69
	void cv_detail_check_failed_auto_const_size_t_const_size_t_const_CheckContextR(const size_t v1, const size_t v2, const cv::detail::CheckContext* ctx, Result_void* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(v1, v2, *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// check_failed_auto(const std::string &, const cv::detail::CheckContext &) /usr/include/opencv2/core/check.hpp:82
	void cv_detail_check_failed_auto_const_stringR_const_CheckContextR(const char* v1, const cv::detail::CheckContext* ctx, Result_void* ocvrs_return) {
		try {
			cv::detail::check_failed_auto(std::string(v1), *ctx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// determinant(cv::InputArray) /usr/include/opencv2/core.hpp:1851
	void cv_determinant_const__InputArrayR(const cv::_InputArray* mtx, Result<double>* ocvrs_return) {
		try {
			double ret = cv::determinant(*mtx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// dft(cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/core.hpp:2217
	void cv_dft_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags, int nonzeroRows, Result_void* ocvrs_return) {
		try {
			cv::dft(*src, *dst, flags, nonzeroRows);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTypeFromD3DFORMAT(const int) /usr/include/opencv2/core/directx.hpp:178
	void cv_directx_getTypeFromD3DFORMAT_const_int(const int iD3DFORMAT, Result<int>* ocvrs_return) {
		try {
			int ret = cv::directx::getTypeFromD3DFORMAT(iD3DFORMAT);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getTypeFromDXGI_FORMAT(const int) /usr/include/opencv2/core/directx.hpp:173
	void cv_directx_getTypeFromDXGI_FORMAT_const_int(const int iDXGI_FORMAT, Result<int>* ocvrs_return) {
		try {
			int ret = cv::directx::getTypeFromDXGI_FORMAT(iDXGI_FORMAT);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// divide(cv::InputArray, cv::InputArray, cv::OutputArray, double, int) /usr/include/opencv2/core.hpp:453
	void cv_divide_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, double scale, int dtype, Result_void* ocvrs_return) {
		try {
			cv::divide(*src1, *src2, *dst, scale, dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// divide(double, cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:457
	void cv_divide_double_const__InputArrayR_const__OutputArrayR_int(double scale, const cv::_InputArray* src2, const cv::_OutputArray* dst, int dtype, Result_void* ocvrs_return) {
		try {
			cv::divide(scale, *src2, *dst, dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// eigenNonSymmetric(cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2010
	void cv_eigenNonSymmetric_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* eigenvalues, const cv::_OutputArray* eigenvectors, Result_void* ocvrs_return) {
		try {
			cv::eigenNonSymmetric(*src, *eigenvalues, *eigenvectors);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// eigen(cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1993
	void cv_eigen_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* eigenvalues, const cv::_OutputArray* eigenvectors, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::eigen(*src, *eigenvalues, *eigenvectors);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// error(const cv::Exception &) /usr/include/opencv2/core.hpp:155
	void cv_error_const_ExceptionR(const cv::Exception* exc, Result_void* ocvrs_return) {
		try {
			cv::error(*exc);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// error(int, const cv::String &, const char *, const char *, int) /usr/include/opencv2/core/base.hpp:298
	void cv_error_int_const_StringR_const_charX_const_charX_int(int _code, const char* _err, const char* _func, const char* _file, int _line, Result_void* ocvrs_return) {
		try {
			cv::error(_code, std::string(_err), _func, _file, _line);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// exp(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1552
	void cv_exp_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::exp(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// extractChannel(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1061
	void cv_extractChannel_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int coi, Result_void* ocvrs_return) {
		try {
			cv::extractChannel(*src, *dst, coi);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// fastAtan2(float, float) /usr/include/opencv2/core/base.hpp:558
	void cv_fastAtan2_float_float(float y, float x, Result<float>* ocvrs_return) {
		try {
			float ret = cv::fastAtan2(y, x);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// findNonZero(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:609
	void cv_findNonZero_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* idx, Result_void* ocvrs_return) {
		try {
			cv::findNonZero(*src, *idx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// flip(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1103
	void cv_flip_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flipCode, Result_void* ocvrs_return) {
		try {
			cv::flip(*src, *dst, flipCode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// gemm(cv::InputArray, cv::InputArray, double, cv::InputArray, double, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1696
	void cv_gemm_const__InputArrayR_const__InputArrayR_double_const__InputArrayR_double_const__OutputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, double alpha, const cv::_InputArray* src3, double beta, const cv::_OutputArray* dst, int flags, Result_void* ocvrs_return) {
		try {
			cv::gemm(*src1, *src2, alpha, *src3, beta, *dst, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getBuildInformation() /usr/include/opencv2/core/utility.hpp:242
	void cv_getBuildInformation(Result<void*>* ocvrs_return) {
		try {
			const cv::String ret = cv::getBuildInformation();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// getCPUFeaturesLine() /usr/include/opencv2/core/utility.hpp:443
	void cv_getCPUFeaturesLine(Result<void*>* ocvrs_return) {
		try {
			std::string ret = cv::getCPUFeaturesLine();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// getCPUTickCount() /usr/include/opencv2/core/utility.hpp:415
	void cv_getCPUTickCount(Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = cv::getCPUTickCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int64_t>))
	}
	
	// getElemSize(int) /usr/include/opencv2/core/utility.hpp:568
	void cv_getElemSize_int(int type, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = cv::getElemSize(type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// getHardwareFeatureName(int) /usr/include/opencv2/core/utility.hpp:431
	void cv_getHardwareFeatureName_int(int feature, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::getHardwareFeatureName(feature);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// getLogLevel() /usr/include/opencv2/core/bindings_utils.hpp:292
	void cv_getLogLevel(Result<int>* ocvrs_return) {
		try {
			int ret = cv::getLogLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getNumThreads() /usr/include/opencv2/core/utility.hpp:218
	void cv_getNumThreads(Result<int>* ocvrs_return) {
		try {
			int ret = cv::getNumThreads();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getNumberOfCPUs() /usr/include/opencv2/core/utility.hpp:447
	void cv_getNumberOfCPUs(Result<int>* ocvrs_return) {
		try {
			int ret = cv::getNumberOfCPUs();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getOptimalDFTSize(int) /usr/include/opencv2/core.hpp:2325
	void cv_getOptimalDFTSize_int(int vecsize, Result<int>* ocvrs_return) {
		try {
			int ret = cv::getOptimalDFTSize(vecsize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getThreadNum() /usr/include/opencv2/core/utility.hpp:234
	void cv_getThreadNum(Result<int>* ocvrs_return) {
		try {
			int ret = cv::getThreadNum();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getTickCount() /usr/include/opencv2/core/utility.hpp:268
	void cv_getTickCount(Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = cv::getTickCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int64_t>))
	}
	
	// getTickFrequency() /usr/include/opencv2/core/utility.hpp:281
	void cv_getTickFrequency(Result<double>* ocvrs_return) {
		try {
			double ret = cv::getTickFrequency();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// getVersionMajor() /usr/include/opencv2/core/utility.hpp:253
	int cv_getVersionMajor() {
			int ret = cv::getVersionMajor();
			return ret;
	}
	
	// getVersionMinor() /usr/include/opencv2/core/utility.hpp:256
	int cv_getVersionMinor() {
			int ret = cv::getVersionMinor();
			return ret;
	}
	
	// getVersionRevision() /usr/include/opencv2/core/utility.hpp:259
	int cv_getVersionRevision() {
			int ret = cv::getVersionRevision();
			return ret;
	}
	
	// getVersionString() /usr/include/opencv2/core/utility.hpp:250
	void cv_getVersionString(Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::getVersionString();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// glob(cv::String, std::vector<String> &, bool) /usr/include/opencv2/core/utility.hpp:180
	void cv_glob_String_vector_String_R_bool(char* pattern, std::vector<cv::String>* result, bool recursive, Result_void* ocvrs_return) {
		try {
			cv::glob(std::string(pattern), *result, recursive);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// haveOpenVX() /usr/include/opencv2/core/ovx.hpp:19
	void cv_haveOpenVX(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::haveOpenVX();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// hconcat(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1189
	void cv_hconcat_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::hconcat(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// hconcat(cv::InputArrayOfArrays, cv::OutputArray) /usr/include/opencv2/core.hpp:1208
	void cv_hconcat_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::hconcat(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// idct(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2282
	void cv_idct_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags, Result_void* ocvrs_return) {
		try {
			cv::idct(*src, *dst, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// idft(cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/core.hpp:2231
	void cv_idft_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags, int nonzeroRows, Result_void* ocvrs_return) {
		try {
			cv::idft(*src, *dst, flags, nonzeroRows);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// inRange(cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1429
	void cv_inRange_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_InputArray* lowerb, const cv::_InputArray* upperb, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::inRange(*src, *lowerb, *upperb, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// insertChannel(cv::InputArray, cv::InputOutputArray, int) /usr/include/opencv2/core.hpp:1069
	void cv_insertChannel_const__InputArrayR_const__InputOutputArrayR_int(const cv::_InputArray* src, const cv::_InputOutputArray* dst, int coi, Result_void* ocvrs_return) {
		try {
			cv::insertChannel(*src, *dst, coi);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getFlags() /usr/include/opencv2/core/utils/instrumentation.hpp:117
	void cv_instr_getFlags(Result<cv::instr::FLAGS>* ocvrs_return) {
		try {
			cv::instr::FLAGS ret = cv::instr::getFlags();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::instr::FLAGS>))
	}
	
	// resetTrace() /usr/include/opencv2/core/utils/instrumentation.hpp:106
	void cv_instr_resetTrace(Result_void* ocvrs_return) {
		try {
			cv::instr::resetTrace();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setFlags(cv::instr::FLAGS) /usr/include/opencv2/core/utils/instrumentation.hpp:115
	void cv_instr_setFlags_FLAGS(cv::instr::FLAGS modeFlags, Result_void* ocvrs_return) {
		try {
			cv::instr::setFlags(modeFlags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setUseInstrumentation(bool) /usr/include/opencv2/core/utils/instrumentation.hpp:105
	void cv_instr_setUseInstrumentation_bool(bool flag, Result_void* ocvrs_return) {
		try {
			cv::instr::setUseInstrumentation(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// useInstrumentation() /usr/include/opencv2/core/utils/instrumentation.hpp:104
	void cv_instr_useInstrumentation(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::instr::useInstrumentation();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// invert(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1887
	void cv_invert_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags, Result<double>* ocvrs_return) {
		try {
			double ret = cv::invert(*src, *dst, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// getIppErrorLocation() /usr/include/opencv2/core/base.hpp:635
	void cv_ipp_getIppErrorLocation(Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::ipp::getIppErrorLocation();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// getIppFeatures() /usr/include/opencv2/core/base.hpp:631
	void cv_ipp_getIppFeatures(Result<unsigned long long>* ocvrs_return) {
		try {
			unsigned long long ret = cv::ipp::getIppFeatures();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned long long>))
	}
	
	// getIppStatus() /usr/include/opencv2/core/base.hpp:634
	void cv_ipp_getIppStatus(Result<int>* ocvrs_return) {
		try {
			int ret = cv::ipp::getIppStatus();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getIppVersion() /usr/include/opencv2/core/base.hpp:638
	void cv_ipp_getIppVersion(Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::ipp::getIppVersion();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// setIppStatus(int, const char *const, const char *const, int) /usr/include/opencv2/core/base.hpp:632
	void cv_ipp_setIppStatus_int_const_charX_const_charX_int(int status, const char* funcname, const char* filename, int line, Result_void* ocvrs_return) {
		try {
			cv::ipp::setIppStatus(status, funcname, filename, line);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setUseIPP_NotExact(bool) /usr/include/opencv2/core/base.hpp:643
	void cv_ipp_setUseIPP_NotExact_bool(bool flag, Result_void* ocvrs_return) {
		try {
			cv::ipp::setUseIPP_NotExact(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setUseIPP(bool) /usr/include/opencv2/core/base.hpp:637
	void cv_ipp_setUseIPP_bool(bool flag, Result_void* ocvrs_return) {
		try {
			cv::ipp::setUseIPP(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// useIPP() /usr/include/opencv2/core/base.hpp:636
	void cv_ipp_useIPP(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ipp::useIPP();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// useIPP_NotExact() /usr/include/opencv2/core/base.hpp:642
	void cv_ipp_useIPP_NotExact(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ipp::useIPP_NotExact();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// kmeans(cv::InputArray, int, cv::InputOutputArray, cv::TermCriteria, int, int, cv::OutputArray) /usr/include/opencv2/core.hpp:3054
	void cv_kmeans_const__InputArrayR_int_const__InputOutputArrayR_TermCriteria_int_int_const__OutputArrayR(const cv::_InputArray* data, int K, const cv::_InputOutputArray* bestLabels, cv::TermCriteria* criteria, int attempts, int flags, const cv::_OutputArray* centers, Result<double>* ocvrs_return) {
		try {
			double ret = cv::kmeans(*data, K, *bestLabels, *criteria, attempts, flags, *centers);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// log(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1565
	void cv_log_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::log(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// magnitude(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1642
	void cv_magnitude_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* magnitude, Result_void* ocvrs_return) {
		try {
			cv::magnitude(*x, *y, *magnitude);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// max(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3743
	void cv_max_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::max(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// max(const cv::Mat &, const cv::Mat &, cv::Mat &) /usr/include/opencv2/core.hpp:1496
	void cv_max_const_MatR_const_MatR_MatR(const cv::Mat* src1, const cv::Mat* src2, cv::Mat* dst, Result_void* ocvrs_return) {
		try {
			cv::max(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// max(const cv::Mat &, double) /usr/include/opencv2/core/mat.hpp:3744
	void cv_max_const_MatR_double(const cv::Mat* a, double s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::max(*a, s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// max(const cv::UMat &, const cv::UMat &, cv::UMat &) /usr/include/opencv2/core.hpp:1500
	void cv_max_const_UMatR_const_UMatR_UMatR(const cv::UMat* src1, const cv::UMat* src2, cv::UMat* dst, Result_void* ocvrs_return) {
		try {
			cv::max(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// max(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1492
	void cv_max_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::max(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// max(double, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3745
	void cv_max_double_const_MatR(double s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::max(s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// meanStdDev(cv::InputArray, cv::OutputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:644
	void cv_meanStdDev_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* mean, const cv::_OutputArray* stddev, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::meanStdDev(*src, *mean, *stddev, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mean(cv::InputArray, cv::InputArray) /usr/include/opencv2/core.hpp:622
	void cv_mean_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_InputArray* mask, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::mean(*src, *mask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	// merge(cv::InputArrayOfArrays, cv::OutputArray) /usr/include/opencv2/core.hpp:953
	void cv_merge_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* mv, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::merge(*mv, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// minMaxIdx(cv::InputArray, double *, double *, int *, int *, cv::InputArray) /usr/include/opencv2/core.hpp:885
	void cv_minMaxIdx_const__InputArrayR_doubleX_doubleX_intX_intX_const__InputArrayR(const cv::_InputArray* src, double* minVal, double* maxVal, int* minIdx, int* maxIdx, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::minMaxIdx(*src, minVal, maxVal, minIdx, maxIdx, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// minMaxLoc(const cv::SparseMat &, double *, double *, int *, int *) /usr/include/opencv2/core.hpp:897
	void cv_minMaxLoc_const_SparseMatR_doubleX_doubleX_intX_intX(const cv::SparseMat* a, double* minVal, double* maxVal, int* minIdx, int* maxIdx, Result_void* ocvrs_return) {
		try {
			cv::minMaxLoc(*a, minVal, maxVal, minIdx, maxIdx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// minMaxLoc(cv::InputArray, double *, double *, cv::Point *, cv::Point *, cv::InputArray) /usr/include/opencv2/core.hpp:824
	void cv_minMaxLoc_const__InputArrayR_doubleX_doubleX_PointX_PointX_const__InputArrayR(const cv::_InputArray* src, double* minVal, double* maxVal, cv::Point* minLoc, cv::Point* maxLoc, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::minMaxLoc(*src, minVal, maxVal, minLoc, maxLoc, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// min(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3735
	void cv_min_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::min(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// min(const cv::Mat &, const cv::Mat &, cv::Mat &) /usr/include/opencv2/core.hpp:1475
	void cv_min_const_MatR_const_MatR_MatR(const cv::Mat* src1, const cv::Mat* src2, cv::Mat* dst, Result_void* ocvrs_return) {
		try {
			cv::min(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// min(const cv::Mat &, double) /usr/include/opencv2/core/mat.hpp:3736
	void cv_min_const_MatR_double(const cv::Mat* a, double s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::min(*a, s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// min(const cv::UMat &, const cv::UMat &, cv::UMat &) /usr/include/opencv2/core.hpp:1479
	void cv_min_const_UMatR_const_UMatR_UMatR(const cv::UMat* src1, const cv::UMat* src2, cv::UMat* dst, Result_void* ocvrs_return) {
		try {
			cv::min(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// min(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1471
	void cv_min_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::min(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// min(double, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3737
	void cv_min_double_const_MatR(double s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::min(s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// mixChannels(cv::InputArrayOfArrays, cv::InputOutputArrayOfArrays, const int *, size_t) /usr/include/opencv2/core.hpp:1036
	void cv_mixChannels_const__InputArrayR_const__InputOutputArrayR_const_intX_size_t(const cv::_InputArray* src, const cv::_InputOutputArray* dst, const int* fromTo, size_t npairs, Result_void* ocvrs_return) {
		try {
			cv::mixChannels(*src, *dst, fromTo, npairs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mixChannels(cv::InputArrayOfArrays, cv::InputOutputArrayOfArrays, const std::vector<int> &) /usr/include/opencv2/core.hpp:1052
	void cv_mixChannels_const__InputArrayR_const__InputOutputArrayR_const_vector_int_R(const cv::_InputArray* src, const cv::_InputOutputArray* dst, const std::vector<int>* fromTo, Result_void* ocvrs_return) {
		try {
			cv::mixChannels(*src, *dst, *fromTo);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mulSpectrums(cv::InputArray, cv::InputArray, cv::OutputArray, int, bool) /usr/include/opencv2/core.hpp:2301
	void cv_mulSpectrums_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_bool(const cv::_InputArray* a, const cv::_InputArray* b, const cv::_OutputArray* c, int flags, bool conjB, Result_void* ocvrs_return) {
		try {
			cv::mulSpectrums(*a, *b, *c, flags, conjB);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mulTransposed(cv::InputArray, cv::OutputArray, bool, cv::InputArray, double, int) /usr/include/opencv2/core.hpp:1727
	void cv_mulTransposed_const__InputArrayR_const__OutputArrayR_bool_const__InputArrayR_double_int(const cv::_InputArray* src, const cv::_OutputArray* dst, bool aTa, const cv::_InputArray* delta, double scale, int dtype, Result_void* ocvrs_return) {
		try {
			cv::mulTransposed(*src, *dst, aTa, *delta, scale, dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// multiply(cv::InputArray, cv::InputArray, cv::OutputArray, double, int) /usr/include/opencv2/core.hpp:425
	void cv_multiply_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, double scale, int dtype, Result_void* ocvrs_return) {
		try {
			cv::multiply(*src1, *src2, *dst, scale, dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// noArray() /usr/include/opencv2/core/mat.hpp:448
	const cv::_InputOutputArray* cv_noArray() {
			const cv::_InputOutputArray ret = cv::noArray();
			return new const cv::_InputOutputArray(ret);
	}
	
	// norm(const cv::SparseMat &, int) /usr/include/opencv2/core.hpp:702
	void cv_norm_const_SparseMatR_int(const cv::SparseMat* src, int normType, Result<double>* ocvrs_return) {
		try {
			double ret = cv::norm(*src, normType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// norm(cv::InputArray, cv::InputArray, int, cv::InputArray) /usr/include/opencv2/core.hpp:696
	void cv_norm_const__InputArrayR_const__InputArrayR_int_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, int normType, const cv::_InputArray* mask, Result<double>* ocvrs_return) {
		try {
			double ret = cv::norm(*src1, *src2, normType, *mask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// norm(cv::InputArray, int, cv::InputArray) /usr/include/opencv2/core.hpp:683
	void cv_norm_const__InputArrayR_int_const__InputArrayR(const cv::_InputArray* src1, int normType, const cv::_InputArray* mask, Result<double>* ocvrs_return) {
		try {
			double ret = cv::norm(*src1, normType, *mask);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// normalize(const cv::SparseMat &, cv::SparseMat &, double, int) /usr/include/opencv2/core.hpp:804
	void cv_normalize_const_SparseMatR_SparseMatR_double_int(const cv::SparseMat* src, cv::SparseMat* dst, double alpha, int normType, Result_void* ocvrs_return) {
		try {
			cv::normalize(*src, *dst, alpha, normType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// normalize(cv::InputArray, cv::InputOutputArray, double, double, int, int, cv::InputArray) /usr/include/opencv2/core.hpp:794
	void cv_normalize_const__InputArrayR_const__InputOutputArrayR_double_double_int_int_const__InputArrayR(const cv::_InputArray* src, const cv::_InputOutputArray* dst, double alpha, double beta, int norm_type, int dtype, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::normalize(*src, *dst, alpha, beta, norm_type, dtype, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// attachContext(const cv::String &, void *, void *, void *) /usr/include/opencv2/core/ocl.hpp:361
	void cv_ocl_attachContext_const_StringR_voidX_voidX_voidX(const char* platformName, void* platformID, void* context, void* deviceID, Result_void* ocvrs_return) {
		try {
			cv::ocl::attachContext(std::string(platformName), platformID, context, deviceID);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// buildOptionsAddMatrixDescription(cv::String &, const cv::String &, cv::InputArray) /usr/include/opencv2/core/ocl.hpp:737
	void cv_ocl_buildOptionsAddMatrixDescription_StringR_const_StringR_const__InputArrayR(void** buildOptions, const char* name, const cv::_InputArray* _m, Result_void* ocvrs_return) {
		try {
			std::string buildOptions_out;
			cv::ocl::buildOptionsAddMatrixDescription(buildOptions_out, std::string(name), *_m);
			*buildOptions = ocvrs_create_string(buildOptions_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// checkOptimalVectorWidth(const int *, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::ocl::OclVectorStrategy) /usr/include/opencv2/core/ocl.hpp:726
	void cv_ocl_checkOptimalVectorWidth_const_intX_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_OclVectorStrategy(const int* vectorWidths, const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputArray* src3, const cv::_InputArray* src4, const cv::_InputArray* src5, const cv::_InputArray* src6, const cv::_InputArray* src7, const cv::_InputArray* src8, const cv::_InputArray* src9, cv::ocl::OclVectorStrategy strat, Result<int>* ocvrs_return) {
		try {
			int ret = cv::ocl::checkOptimalVectorWidth(vectorWidths, *src1, *src2, *src3, *src4, *src5, *src6, *src7, *src8, *src9, strat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// convertFromBuffer(void *, size_t, int, int, int, cv::UMat &) /usr/include/opencv2/core/ocl.hpp:375
	void cv_ocl_convertFromBuffer_voidX_size_t_int_int_int_UMatR(void* cl_mem_buffer, size_t step, int rows, int cols, int type, cv::UMat* dst, Result_void* ocvrs_return) {
		try {
			cv::ocl::convertFromBuffer(cl_mem_buffer, step, rows, cols, type, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// convertFromImage(void *, cv::UMat &) /usr/include/opencv2/core/ocl.hpp:384
	void cv_ocl_convertFromImage_voidX_UMatR(void* cl_mem_image, cv::UMat* dst, Result_void* ocvrs_return) {
		try {
			cv::ocl::convertFromImage(cl_mem_image, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// convertTypeStr(int, int, int, char *) /usr/include/opencv2/core/ocl.hpp:700
	void cv_ocl_convertTypeStr_int_int_int_charX(int sdepth, int ddepth, int cn, void** buf, Result<void*>* ocvrs_return) {
		try {
			char* buf_out = new char[1024]();
			const char* ret = cv::ocl::convertTypeStr(sdepth, ddepth, cn, buf_out);
			*buf = ocvrs_create_string(buf_out);
			void* f_ret = ocvrs_create_string(ret);
			delete[] buf_out;
			Ok(f_ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// finish() /usr/include/opencv2/core/ocl.hpp:59
	void cv_ocl_finish(Result_void* ocvrs_return) {
		try {
			cv::ocl::finish();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getOpenCLErrorString(int) /usr/include/opencv2/core/ocl.hpp:704
	void cv_ocl_getOpenCLErrorString_int(int errorCode, Result<void*>* ocvrs_return) {
		try {
			const char* ret = cv::ocl::getOpenCLErrorString(errorCode);
			Ok(ocvrs_create_string(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// getPlatfomsInfo(std::vector<PlatformInfo> &) /usr/include/opencv2/core/ocl.hpp:706
	void cv_ocl_getPlatfomsInfo_vector_PlatformInfo_R(std::vector<cv::ocl::PlatformInfo>* platform_info, Result_void* ocvrs_return) {
		try {
			cv::ocl::getPlatfomsInfo(*platform_info);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// haveAmdBlas() /usr/include/opencv2/core/ocl.hpp:56
	void cv_ocl_haveAmdBlas(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ocl::haveAmdBlas();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// haveAmdFft() /usr/include/opencv2/core/ocl.hpp:57
	void cv_ocl_haveAmdFft(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ocl::haveAmdFft();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// haveOpenCL() /usr/include/opencv2/core/ocl.hpp:54
	void cv_ocl_haveOpenCL(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ocl::haveOpenCL();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// haveSVM() /usr/include/opencv2/core/ocl.hpp:61
	void cv_ocl_haveSVM(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ocl::haveSVM();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// kernelToStr(cv::InputArray, int, const char *) /usr/include/opencv2/core/ocl.hpp:705
	void cv_ocl_kernelToStr_const__InputArrayR_int_const_charX(const cv::_InputArray* _kernel, int ddepth, const char* name, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::ocl::kernelToStr(*_kernel, ddepth, name);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// memopTypeToStr(int) /usr/include/opencv2/core/ocl.hpp:702
	void cv_ocl_memopTypeToStr_int(int t, Result<void*>* ocvrs_return) {
		try {
			const char* ret = cv::ocl::memopTypeToStr(t);
			Ok(ocvrs_create_string(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// predictOptimalVectorWidthMax(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/core/ocl.hpp:733
	void cv_ocl_predictOptimalVectorWidthMax_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputArray* src3, const cv::_InputArray* src4, const cv::_InputArray* src5, const cv::_InputArray* src6, const cv::_InputArray* src7, const cv::_InputArray* src8, const cv::_InputArray* src9, Result<int>* ocvrs_return) {
		try {
			int ret = cv::ocl::predictOptimalVectorWidthMax(*src1, *src2, *src3, *src4, *src5, *src6, *src7, *src8, *src9);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// predictOptimalVectorWidth(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::ocl::OclVectorStrategy) /usr/include/opencv2/core/ocl.hpp:721
	void cv_ocl_predictOptimalVectorWidth_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_OclVectorStrategy(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_InputArray* src3, const cv::_InputArray* src4, const cv::_InputArray* src5, const cv::_InputArray* src6, const cv::_InputArray* src7, const cv::_InputArray* src8, const cv::_InputArray* src9, cv::ocl::OclVectorStrategy strat, Result<int>* ocvrs_return) {
		try {
			int ret = cv::ocl::predictOptimalVectorWidth(*src1, *src2, *src3, *src4, *src5, *src6, *src7, *src8, *src9, strat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setUseOpenCL(bool) /usr/include/opencv2/core/ocl.hpp:58
	void cv_ocl_setUseOpenCL_bool(bool flag, Result_void* ocvrs_return) {
		try {
			cv::ocl::setUseOpenCL(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// typeToStr(int) /usr/include/opencv2/core/ocl.hpp:701
	void cv_ocl_typeToStr_int(int t, Result<void*>* ocvrs_return) {
		try {
			const char* ret = cv::ocl::typeToStr(t);
			Ok(ocvrs_create_string(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// useOpenCL() /usr/include/opencv2/core/ocl.hpp:55
	void cv_ocl_useOpenCL(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ocl::useOpenCL();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// vecopTypeToStr(int) /usr/include/opencv2/core/ocl.hpp:703
	void cv_ocl_vecopTypeToStr_int(int t, Result<void*>* ocvrs_return) {
		try {
			const char* ret = cv::ocl::vecopTypeToStr(t);
			Ok(ocvrs_create_string(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// convertFromGLTexture2D(const cv::ogl::Texture2D &, cv::OutputArray) /usr/include/opencv2/core/opengl.hpp:538
	void cv_ogl_convertFromGLTexture2D_const_Texture2DR_const__OutputArrayR(const cv::ogl::Texture2D* texture, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::ogl::convertFromGLTexture2D(*texture, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// convertToGLTexture2D(cv::InputArray, cv::ogl::Texture2D &) /usr/include/opencv2/core/opengl.hpp:532
	void cv_ogl_convertToGLTexture2D_const__InputArrayR_Texture2DR(const cv::_InputArray* src, cv::ogl::Texture2D* texture, Result_void* ocvrs_return) {
		try {
			cv::ogl::convertToGLTexture2D(*src, *texture);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mapGLBuffer(const cv::ogl::Buffer &, cv::AccessFlag) /usr/include/opencv2/core/opengl.hpp:551
	void cv_ogl_mapGLBuffer_const_BufferR_AccessFlag(const cv::ogl::Buffer* buffer, cv::AccessFlag accessFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::ogl::mapGLBuffer(*buffer, accessFlags);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// initializeContextFromGL() /usr/include/opencv2/core/opengl.hpp:524
	void cv_ogl_ocl_initializeContextFromGL(Result<cv::ocl::Context*>* ocvrs_return) {
		try {
			cv::ocl::Context ret = cv::ogl::ocl::initializeContextFromGL();
			Ok(new cv::ocl::Context(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Context*>))
	}
	
	// render(const cv::ogl::Arrays &, cv::InputArray, int, cv::Scalar) /usr/include/opencv2/core/opengl.hpp:513
	void cv_ogl_render_const_ArraysR_const__InputArrayR_int_Scalar(const cv::ogl::Arrays* arr, const cv::_InputArray* indices, int mode, cv::Scalar* color, Result_void* ocvrs_return) {
		try {
			cv::ogl::render(*arr, *indices, mode, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// render(const cv::ogl::Arrays &, int, cv::Scalar) /usr/include/opencv2/core/opengl.hpp:505
	void cv_ogl_render_const_ArraysR_int_Scalar(const cv::ogl::Arrays* arr, int mode, cv::Scalar* color, Result_void* ocvrs_return) {
		try {
			cv::ogl::render(*arr, mode, *color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// render(const cv::ogl::Texture2D &, Rect_<double>, Rect_<double>) /usr/include/opencv2/core/opengl.hpp:496
	void cv_ogl_render_const_Texture2DR_Rect__double__Rect__double_(const cv::ogl::Texture2D* tex, cv::Rect_<double>* wndRect, cv::Rect_<double>* texRect, Result_void* ocvrs_return) {
		try {
			cv::ogl::render(*tex, *wndRect, *texRect);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// unmapGLBuffer(cv::UMat &) /usr/include/opencv2/core/opengl.hpp:559
	void cv_ogl_unmapGLBuffer_UMatR(cv::UMat* u, Result_void* ocvrs_return) {
		try {
			cv::ogl::unmapGLBuffer(*u);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// operator+(const cv::MatExpr &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3613
	void cv_operatorA_const_MatExprR_const_MatExprR(const cv::MatExpr* e1, const cv::MatExpr* e2, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator+(*e1, *e2);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator+(const cv::MatExpr &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3609
	void cv_operatorA_const_MatExprR_const_MatR(const cv::MatExpr* e, const cv::Mat* m, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator+(*e, *m);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator+(const cv::MatExpr &, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:3611
	void cv_operatorA_const_MatExprR_const_ScalarR(const cv::MatExpr* e, const cv::Scalar* s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator+(*e, *s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator+(const cv::Mat &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3610
	void cv_operatorA_const_MatR_const_MatExprR(const cv::Mat* m, const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator+(*m, *e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator+(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3606
	void cv_operatorA_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator+(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator+(const cv::Mat &, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:3607
	void cv_operatorA_const_MatR_const_ScalarR(const cv::Mat* a, const cv::Scalar* s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator+(*a, *s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator+(const cv::Scalar &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3612
	void cv_operatorA_const_ScalarR_const_MatExprR(const cv::Scalar* s, const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator+(*s, *e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator+(const cv::Scalar &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3608
	void cv_operatorA_const_ScalarR_const_MatR(const cv::Scalar* s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator+(*s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator/(const cv::MatExpr &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3655
	void cv_operatorD_const_MatExprR_const_MatExprR(const cv::MatExpr* e1, const cv::MatExpr* e2, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator/(*e1, *e2);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator/(const cv::MatExpr &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3651
	void cv_operatorD_const_MatExprR_const_MatR(const cv::MatExpr* e, const cv::Mat* m, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator/(*e, *m);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator/(const cv::MatExpr &, double) /usr/include/opencv2/core/mat.hpp:3653
	void cv_operatorD_const_MatExprR_double(const cv::MatExpr* e, double s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator/(*e, s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator/(const cv::Mat &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3652
	void cv_operatorD_const_MatR_const_MatExprR(const cv::Mat* m, const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator/(*m, *e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator/(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3648
	void cv_operatorD_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator/(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator/(const cv::Mat &, double) /usr/include/opencv2/core/mat.hpp:3649
	void cv_operatorD_const_MatR_double(const cv::Mat* a, double s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator/(*a, s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator/(double, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3654
	void cv_operatorD_double_const_MatExprR(double s, const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator/(s, *e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator/(double, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3650
	void cv_operatorD_double_const_MatR(double s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator/(s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator==(const cv::FileNodeIterator &, const cv::FileNodeIterator &) /usr/include/opencv2/core/persistence.hpp:1331
	void cv_operatorEQ_const_FileNodeIteratorR_const_FileNodeIteratorR(const cv::FileNodeIterator* it1, const cv::FileNodeIterator* it2, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::operator==(*it1, *it2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// operator==(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3677
	void cv_operatorEQ_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator==(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator==(const cv::Mat &, double) /usr/include/opencv2/core/mat.hpp:3678
	void cv_operatorEQ_const_MatR_double(const cv::Mat* a, double s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator==(*a, s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator==(double, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3679
	void cv_operatorEQ_double_const_MatR(double s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator==(s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator-(const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3633
	void cv_operatorS_const_MatExprR(const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator-(*e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator-(const cv::MatExpr &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3626
	void cv_operatorS_const_MatExprR_const_MatExprR(const cv::MatExpr* e1, const cv::MatExpr* e2, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator-(*e1, *e2);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator-(const cv::MatExpr &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3622
	void cv_operatorS_const_MatExprR_const_MatR(const cv::MatExpr* e, const cv::Mat* m, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator-(*e, *m);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator-(const cv::MatExpr &, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:3624
	void cv_operatorS_const_MatExprR_const_ScalarR(const cv::MatExpr* e, const cv::Scalar* s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator-(*e, *s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator-(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3632
	void cv_operatorS_const_MatR(const cv::Mat* m, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator-(*m);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator-(const cv::Mat &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3623
	void cv_operatorS_const_MatR_const_MatExprR(const cv::Mat* m, const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator-(*m, *e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator-(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3619
	void cv_operatorS_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator-(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator-(const cv::Mat &, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:3620
	void cv_operatorS_const_MatR_const_ScalarR(const cv::Mat* a, const cv::Scalar* s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator-(*a, *s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator-(const cv::Scalar &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3625
	void cv_operatorS_const_ScalarR_const_MatExprR(const cv::Scalar* s, const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator-(*s, *e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator-(const cv::Scalar &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3621
	void cv_operatorS_const_ScalarR_const_MatR(const cv::Scalar* s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator-(*s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator*(const cv::MatExpr &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3642
	void cv_operatorX_const_MatExprR_const_MatExprR(const cv::MatExpr* e1, const cv::MatExpr* e2, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator*(*e1, *e2);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator*(const cv::MatExpr &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3638
	void cv_operatorX_const_MatExprR_const_MatR(const cv::MatExpr* e, const cv::Mat* m, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator*(*e, *m);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator*(const cv::MatExpr &, double) /usr/include/opencv2/core/mat.hpp:3640
	void cv_operatorX_const_MatExprR_double(const cv::MatExpr* e, double s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator*(*e, s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator*(const cv::Mat &, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3639
	void cv_operatorX_const_MatR_const_MatExprR(const cv::Mat* m, const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator*(*m, *e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator*(const cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3635
	void cv_operatorX_const_MatR_const_MatR(const cv::Mat* a, const cv::Mat* b, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator*(*a, *b);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator*(const cv::Mat &, double) /usr/include/opencv2/core/mat.hpp:3636
	void cv_operatorX_const_MatR_double(const cv::Mat* a, double s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator*(*a, s);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator*(double, const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3641
	void cv_operatorX_double_const_MatExprR(double s, const cv::MatExpr* e, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator*(s, *e);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator*(double, const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3637
	void cv_operatorX_double_const_MatR(double s, const cv::Mat* a, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::operator*(s, *a);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// parallel_for_(const cv::Range &, const cv::ParallelLoopBody &, double) /usr/include/opencv2/core/utility.hpp:587
	void cv_parallel_for__const_RangeR_const_ParallelLoopBodyR_double(const cv::Range* range, const cv::ParallelLoopBody* body, double nstripes, Result_void* ocvrs_return) {
		try {
			cv::parallel_for_(*range, *body, nstripes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// patchNaNs(cv::InputOutputArray, double) /usr/include/opencv2/core.hpp:1666
	void cv_patchNaNs_const__InputOutputArrayR_double(const cv::_InputOutputArray* a, double val, Result_void* ocvrs_return) {
		try {
			cv::patchNaNs(*a, val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// perspectiveTransform(cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1803
	void cv_perspectiveTransform_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* m, Result_void* ocvrs_return) {
		try {
			cv::perspectiveTransform(*src, *dst, *m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// phase(cv::InputArray, cv::InputArray, cv::OutputArray, bool) /usr/include/opencv2/core.hpp:1628
	void cv_phase_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool(const cv::_InputArray* x, const cv::_InputArray* y, const cv::_OutputArray* angle, bool angleInDegrees, Result_void* ocvrs_return) {
		try {
			cv::phase(*x, *y, *angle, angleInDegrees);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// polarToCart(cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray, bool) /usr/include/opencv2/core.hpp:1587
	void cv_polarToCart_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_bool(const cv::_InputArray* magnitude, const cv::_InputArray* angle, const cv::_OutputArray* x, const cv::_OutputArray* y, bool angleInDegrees, Result_void* ocvrs_return) {
		try {
			cv::polarToCart(*magnitude, *angle, *x, *y, angleInDegrees);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// pow(cv::InputArray, double, cv::OutputArray) /usr/include/opencv2/core.hpp:1536
	void cv_pow_const__InputArrayR_double_const__OutputArrayR(const cv::_InputArray* src, double power, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::pow(*src, power, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// randShuffle(cv::InputOutputArray, double, cv::RNG *) /usr/include/opencv2/core.hpp:2382
	void cv_randShuffle_const__InputOutputArrayR_double_RNGX(const cv::_InputOutputArray* dst, double iterFactor, cv::RNG* rng, Result_void* ocvrs_return) {
		try {
			cv::randShuffle(*dst, iterFactor, rng);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// randn(cv::InputOutputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/core.hpp:2369
	void cv_randn_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputOutputArray* dst, const cv::_InputArray* mean, const cv::_InputArray* stddev, Result_void* ocvrs_return) {
		try {
			cv::randn(*dst, *mean, *stddev);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// randu(cv::InputOutputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/core.hpp:2356
	void cv_randu_const__InputOutputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputOutputArray* dst, const cv::_InputArray* low, const cv::_InputArray* high, Result_void* ocvrs_return) {
		try {
			cv::randu(*dst, *low, *high);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// read(const cv::FileNode &, cv::DMatch &, const cv::DMatch &) /usr/include/opencv2/core/persistence.hpp:734
	void cv_read_const_FileNodeR_DMatchR_const_DMatchR(const cv::FileNode* node, cv::DMatch* value, const cv::DMatch* default_value, Result_void* ocvrs_return) {
		try {
			cv::read(*node, *value, *default_value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// read(const cv::FileNode &, cv::KeyPoint &, const cv::KeyPoint &) /usr/include/opencv2/core/persistence.hpp:733
	void cv_read_const_FileNodeR_KeyPointR_const_KeyPointR(const cv::FileNode* node, cv::KeyPoint* value, const cv::KeyPoint* default_value, Result_void* ocvrs_return) {
		try {
			cv::read(*node, *value, *default_value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// read(const cv::FileNode &, cv::Mat &, const cv::Mat &) /usr/include/opencv2/core/persistence.hpp:727
	void cv_read_const_FileNodeR_MatR_const_MatR(const cv::FileNode* node, cv::Mat* mat, const cv::Mat* default_mat, Result_void* ocvrs_return) {
		try {
			cv::read(*node, *mat, *default_mat);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// read(const cv::FileNode &, cv::SparseMat &, const cv::SparseMat &) /usr/include/opencv2/core/persistence.hpp:728
	void cv_read_const_FileNodeR_SparseMatR_const_SparseMatR(const cv::FileNode* node, cv::SparseMat* mat, const cv::SparseMat* default_mat, Result_void* ocvrs_return) {
		try {
			cv::read(*node, *mat, *default_mat);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// read(const cv::FileNode &, double &, double) /usr/include/opencv2/core/persistence.hpp:725
	void cv_read_const_FileNodeR_doubleR_double(const cv::FileNode* node, double* value, double default_value, Result_void* ocvrs_return) {
		try {
			cv::read(*node, *value, default_value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// read(const cv::FileNode &, float &, float) /usr/include/opencv2/core/persistence.hpp:724
	void cv_read_const_FileNodeR_floatR_float(const cv::FileNode* node, float* value, float default_value, Result_void* ocvrs_return) {
		try {
			cv::read(*node, *value, default_value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// read(const cv::FileNode &, int &, int) /usr/include/opencv2/core/persistence.hpp:723
	void cv_read_const_FileNodeR_intR_int(const cv::FileNode* node, int* value, int default_value, Result_void* ocvrs_return) {
		try {
			cv::read(*node, *value, default_value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// read(const cv::FileNode &, std::string &, const std::string &) /usr/include/opencv2/core/persistence.hpp:726
	void cv_read_const_FileNodeR_stringR_const_stringR(const cv::FileNode* node, void** value, const char* default_value, Result_void* ocvrs_return) {
		try {
			std::string value_out;
			cv::read(*node, value_out, std::string(default_value));
			*value = ocvrs_create_string(value_out.c_str());
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// read(const cv::FileNode &, std::vector<DMatch> &) /usr/include/opencv2/core/persistence.hpp:731
	void cv_read_const_FileNodeR_vector_DMatch_R(const cv::FileNode* node, std::vector<cv::DMatch>* matches, Result_void* ocvrs_return) {
		try {
			cv::read(*node, *matches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// read(const cv::FileNode &, std::vector<KeyPoint> &) /usr/include/opencv2/core/persistence.hpp:730
	void cv_read_const_FileNodeR_vector_KeyPoint_R(const cv::FileNode* node, std::vector<cv::KeyPoint>* keypoints, Result_void* ocvrs_return) {
		try {
			cv::read(*node, *keypoints);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// reduceArgMax(cv::InputArray, cv::OutputArray, int, bool) /usr/include/opencv2/core.hpp:860
	void cv_reduceArgMax_const__InputArrayR_const__OutputArrayR_int_bool(const cv::_InputArray* src, const cv::_OutputArray* dst, int axis, bool lastIndex, Result_void* ocvrs_return) {
		try {
			cv::reduceArgMax(*src, *dst, axis, lastIndex);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// reduceArgMin(cv::InputArray, cv::OutputArray, int, bool) /usr/include/opencv2/core.hpp:843
	void cv_reduceArgMin_const__InputArrayR_const__OutputArrayR_int_bool(const cv::_InputArray* src, const cv::_OutputArray* dst, int axis, bool lastIndex, Result_void* ocvrs_return) {
		try {
			cv::reduceArgMin(*src, *dst, axis, lastIndex);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// reduce(cv::InputArray, cv::OutputArray, int, int, int) /usr/include/opencv2/core.hpp:924
	void cv_reduce_const__InputArrayR_const__OutputArrayR_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int dim, int rtype, int dtype, Result_void* ocvrs_return) {
		try {
			cv::reduce(*src, *dst, dim, rtype, dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// repeat(const cv::Mat &, int, int) /usr/include/opencv2/core.hpp:1145
	void cv_repeat_const_MatR_int_int(const cv::Mat* src, int ny, int nx, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::repeat(*src, ny, nx);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// repeat(cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/core.hpp:1136
	void cv_repeat_const__InputArrayR_int_int_const__OutputArrayR(const cv::_InputArray* src, int ny, int nx, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::repeat(*src, ny, nx, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// rotate(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1121
	void cv_rotate_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int rotateCode, Result_void* ocvrs_return) {
		try {
			cv::rotate(*src, *dst, rotateCode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// addSamplesDataSearchPath(const cv::String &) /usr/include/opencv2/core/utility.hpp:1200
	void cv_samples_addSamplesDataSearchPath_const_StringR(const char* path, Result_void* ocvrs_return) {
		try {
			cv::samples::addSamplesDataSearchPath(std::string(path));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// addSamplesDataSearchSubDirectory(const cv::String &) /usr/include/opencv2/core/utility.hpp:1209
	void cv_samples_addSamplesDataSearchSubDirectory_const_StringR(const char* subdir, Result_void* ocvrs_return) {
		try {
			cv::samples::addSamplesDataSearchSubDirectory(std::string(subdir));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// findFileOrKeep(const cv::String &, bool) /usr/include/opencv2/core/utility.hpp:1183
	void cv_samples_findFileOrKeep_const_StringR_bool(const char* relative_path, bool silentMode, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::samples::findFileOrKeep(std::string(relative_path), silentMode);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// findFile(const cv::String &, bool, bool) /usr/include/opencv2/core/utility.hpp:1181
	void cv_samples_findFile_const_StringR_bool_bool(const char* relative_path, bool required, bool silentMode, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::samples::findFile(std::string(relative_path), required, silentMode);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// scaleAdd(cv::InputArray, double, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:478
	void cv_scaleAdd_const__InputArrayR_double_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, double alpha, const cv::_InputArray* src2, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::scaleAdd(*src1, alpha, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setBreakOnError(bool) /usr/include/opencv2/core/utility.hpp:160
	void cv_setBreakOnError_bool(bool flag, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::setBreakOnError(flag);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setIdentity(cv::InputOutputArray, const cv::Scalar &) /usr/include/opencv2/core.hpp:1836
	void cv_setIdentity_const__InputOutputArrayR_const_ScalarR(const cv::_InputOutputArray* mtx, const cv::Scalar* s, Result_void* ocvrs_return) {
		try {
			cv::setIdentity(*mtx, *s);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setLogLevel(int) /usr/include/opencv2/core/bindings_utils.hpp:285
	void cv_setLogLevel_int(int level, Result<int>* ocvrs_return) {
		try {
			int ret = cv::setLogLevel(level);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setNumThreads(int) /usr/include/opencv2/core/utility.hpp:200
	void cv_setNumThreads_int(int nthreads, Result_void* ocvrs_return) {
		try {
			cv::setNumThreads(nthreads);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setRNGSeed(int) /usr/include/opencv2/core.hpp:2344
	void cv_setRNGSeed_int(int seed, Result_void* ocvrs_return) {
		try {
			cv::setRNGSeed(seed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setUseOpenVX(bool) /usr/include/opencv2/core/ovx.hpp:25
	void cv_setUseOpenVX_bool(bool flag, Result_void* ocvrs_return) {
		try {
			cv::setUseOpenVX(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setUseOptimized(bool) /usr/include/opencv2/core/utility.hpp:560
	void cv_setUseOptimized_bool(bool onoff, Result_void* ocvrs_return) {
		try {
			cv::setUseOptimized(onoff);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// solveCubic(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1962
	void cv_solveCubic_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* coeffs, const cv::_OutputArray* roots, Result<int>* ocvrs_return) {
		try {
			int ret = cv::solveCubic(*coeffs, *roots);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// solveLP(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core/optim.hpp:296
	void cv_solveLP_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* Func, const cv::_InputArray* Constr, const cv::_OutputArray* z, Result<int>* ocvrs_return) {
		try {
			int ret = cv::solveLP(*Func, *Constr, *z);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// solvePoly(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1972
	void cv_solvePoly_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* coeffs, const cv::_OutputArray* roots, int maxIters, Result<double>* ocvrs_return) {
		try {
			double ret = cv::solvePoly(*coeffs, *roots, maxIters);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// solve(cv::InputArray, cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1911
	void cv_solve_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::solve(*src1, *src2, *dst, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// sortIdx(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1947
	void cv_sortIdx_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags, Result_void* ocvrs_return) {
		try {
			cv::sortIdx(*src, *dst, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// sort(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:1927
	void cv_sort_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int flags, Result_void* ocvrs_return) {
		try {
			cv::sort(*src, *dst, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// split(const cv::Mat &, cv::Mat *) /usr/include/opencv2/core.hpp:970
	void cv_split_const_MatR_MatX(const cv::Mat* src, cv::Mat* mvbegin, Result_void* ocvrs_return) {
		try {
			cv::split(*src, mvbegin);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// split(cv::InputArray, cv::OutputArrayOfArrays) /usr/include/opencv2/core.hpp:976
	void cv_split_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* m, const cv::_OutputArray* mv, Result_void* ocvrs_return) {
		try {
			cv::split(*m, *mv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// sqrt(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1511
	void cv_sqrt_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::sqrt(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// subtract(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray, int) /usr/include/opencv2/core.hpp:400
	void cv_subtract_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_int(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, const cv::_InputArray* mask, int dtype, Result_void* ocvrs_return) {
		try {
			cv::subtract(*src1, *src2, *dst, *mask, dtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// sum(cv::InputArray) /usr/include/opencv2/core.hpp:572
	void cv_sum_const__InputArrayR(const cv::_InputArray* src, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::sum(*src);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	// swap(cv::Mat &, cv::Mat &) /usr/include/opencv2/core.hpp:240
	void cv_swap_MatR_MatR(cv::Mat* a, cv::Mat* b, Result_void* ocvrs_return) {
		try {
			cv::swap(*a, *b);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// swap(cv::UMat &, cv::UMat &) /usr/include/opencv2/core.hpp:242
	void cv_swap_UMatR_UMatR(cv::UMat* a, cv::UMat* b, Result_void* ocvrs_return) {
		try {
			cv::swap(*a, *b);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// tempfile(const char *) /usr/include/opencv2/core/utility.hpp:179
	void cv_tempfile_const_charX(const char* suffix, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::tempfile(suffix);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// theRNG() /usr/include/opencv2/core.hpp:2336
	void cv_theRNG(Result<cv::RNG*>* ocvrs_return) {
		try {
			cv::RNG ret = cv::theRNG();
			Ok(new cv::RNG(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RNG*>))
	}
	
	// trace(cv::InputArray) /usr/include/opencv2/core.hpp:1860
	void cv_trace_const__InputArrayR(const cv::_InputArray* mtx, Result<cv::Scalar>* ocvrs_return) {
		try {
			cv::Scalar ret = cv::trace(*mtx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Scalar>))
	}
	
	// transform(cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/core.hpp:1776
	void cv_transform_const__InputArrayR_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* m, Result_void* ocvrs_return) {
		try {
			cv::transform(*src, *dst, *m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// transposeND(cv::InputArray, const std::vector<int> &, cv::OutputArray) /usr/include/opencv2/core.hpp:1750
	void cv_transposeND_const__InputArrayR_const_vector_int_R_const__OutputArrayR(const cv::_InputArray* src, const std::vector<int>* order, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::transposeND(*src, *order, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// transpose(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1740
	void cv_transpose_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::transpose(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// typeToString(int) /usr/include/opencv2/core/check.hpp:16
	void cv_typeToString_int(int type, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::typeToString(type);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// useOpenVX() /usr/include/opencv2/core/ovx.hpp:22
	void cv_useOpenVX(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::useOpenVX();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// useOptimized() /usr/include/opencv2/core/utility.hpp:566
	void cv_useOptimized(Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::useOptimized();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// dumpBool(bool) /usr/include/opencv2/core/bindings_utils.hpp:27
	void cv_utils_dumpBool_bool(bool argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpBool(argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// dumpCString(const char *) /usr/include/opencv2/core/bindings_utils.hpp:59
	void cv_utils_dumpCString_const_charX(const char* argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpCString(argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// dumpDouble(double) /usr/include/opencv2/core/bindings_utils.hpp:53
	void cv_utils_dumpDouble_double(double argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpDouble(argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// dumpFloat(float) /usr/include/opencv2/core/bindings_utils.hpp:47
	void cv_utils_dumpFloat_float(float argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpFloat(argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// dumpInputArrayOfArrays(cv::InputArrayOfArrays) /usr/include/opencv2/core/bindings_utils.hpp:20
	void cv_utils_dumpInputArrayOfArrays_const__InputArrayR(const cv::_InputArray* argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpInputArrayOfArrays(*argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// dumpInputArray(cv::InputArray) /usr/include/opencv2/core/bindings_utils.hpp:18
	void cv_utils_dumpInputArray_const__InputArrayR(const cv::_InputArray* argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpInputArray(*argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// dumpInputOutputArrayOfArrays(cv::InputOutputArrayOfArrays) /usr/include/opencv2/core/bindings_utils.hpp:24
	void cv_utils_dumpInputOutputArrayOfArrays_const__InputOutputArrayR(const cv::_InputOutputArray* argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpInputOutputArrayOfArrays(*argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// dumpInputOutputArray(cv::InputOutputArray) /usr/include/opencv2/core/bindings_utils.hpp:22
	void cv_utils_dumpInputOutputArray_const__InputOutputArrayR(const cv::_InputOutputArray* argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpInputOutputArray(*argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// dumpInt(int) /usr/include/opencv2/core/bindings_utils.hpp:33
	void cv_utils_dumpInt_int(int argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpInt(argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// dumpRange(const cv::Range &) /usr/include/opencv2/core/bindings_utils.hpp:122
	void cv_utils_dumpRange_const_RangeR(const cv::Range* argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpRange(*argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// dumpRect(const cv::Rect &) /usr/include/opencv2/core/bindings_utils.hpp:85
	void cv_utils_dumpRect_const_RectR(const cv::Rect* argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpRect(*argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// dumpRotatedRect(const cv::RotatedRect &) /usr/include/opencv2/core/bindings_utils.hpp:99
	void cv_utils_dumpRotatedRect_const_RotatedRectR(const cv::RotatedRect* argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpRotatedRect(*argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// dumpSizeT(size_t) /usr/include/opencv2/core/bindings_utils.hpp:39
	void cv_utils_dumpSizeT_size_t(size_t argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpSizeT(argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// dumpString(const cv::String &) /usr/include/opencv2/core/bindings_utils.hpp:65
	void cv_utils_dumpString_const_StringR(const char* argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpString(std::string(argument));
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// dumpTermCriteria(const cv::TermCriteria &) /usr/include/opencv2/core/bindings_utils.hpp:92
	void cv_utils_dumpTermCriteria_const_TermCriteriaR(const cv::TermCriteria* argument, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpTermCriteria(*argument);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// dumpVectorOfDouble(const std::vector<double> &) /usr/include/opencv2/core/bindings_utils.hpp:148
	void cv_utils_dumpVectorOfDouble_const_vector_double_R(const std::vector<double>* vec, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpVectorOfDouble(*vec);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// dumpVectorOfInt(const std::vector<int> &) /usr/include/opencv2/core/bindings_utils.hpp:146
	void cv_utils_dumpVectorOfInt_const_vector_int_R(const std::vector<int>* vec, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpVectorOfInt(*vec);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// dumpVectorOfRect(const std::vector<Rect> &) /usr/include/opencv2/core/bindings_utils.hpp:150
	void cv_utils_dumpVectorOfRect_const_vector_Rect_R(const std::vector<cv::Rect>* vec, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::dumpVectorOfRect(*vec);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// getCacheDirectoryForDownloads() /usr/include/opencv2/core/bindings_utils.hpp:276
	void cv_utils_fs_getCacheDirectoryForDownloads(Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::fs::getCacheDirectoryForDownloads();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// generateVectorOfInt(size_t, std::vector<int> &) /usr/include/opencv2/core/bindings_utils.hpp:166
	void cv_utils_generateVectorOfInt_size_t_vector_int_R(size_t len, std::vector<int>* vec, Result_void* ocvrs_return) {
		try {
			cv::utils::generateVectorOfInt(len, *vec);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// generateVectorOfMat(size_t, int, int, int, std::vector<Mat> &) /usr/include/opencv2/core/bindings_utils.hpp:179
	void cv_utils_generateVectorOfMat_size_t_int_int_int_vector_Mat_R(size_t len, int rows, int cols, int dtype, std::vector<cv::Mat>* vec, Result_void* ocvrs_return) {
		try {
			cv::utils::generateVectorOfMat(len, rows, cols, dtype, *vec);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// generateVectorOfRect(size_t, std::vector<Rect> &) /usr/include/opencv2/core/bindings_utils.hpp:153
	void cv_utils_generateVectorOfRect_size_t_vector_Rect_R(size_t len, std::vector<cv::Rect>* vec, Result_void* ocvrs_return) {
		try {
			cv::utils::generateVectorOfRect(len, *vec);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getThreadID() /usr/include/opencv2/core/utility.hpp:1216
	void cv_utils_getThreadID(Result<int>* ocvrs_return) {
		try {
			int ret = cv::utils::getThreadID();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getLogLevel() /usr/include/opencv2/core/utils/logger.hpp:27
	void cv_utils_logging_getLogLevel(Result<cv::utils::logging::LogLevel>* ocvrs_return) {
		try {
			cv::utils::logging::LogLevel ret = cv::utils::logging::getLogLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::utils::logging::LogLevel>))
	}
	
	// getLogTagLevel(const char *) /usr/include/opencv2/core/utils/logger.hpp:33
	void cv_utils_logging_getLogTagLevel_const_charX(const char* tag, Result<cv::utils::logging::LogLevel>* ocvrs_return) {
		try {
			cv::utils::logging::LogLevel ret = cv::utils::logging::getLogTagLevel(tag);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::utils::logging::LogLevel>))
	}
	
	// getGlobalLogTag() /usr/include/opencv2/core/utils/logger.hpp:38
	void cv_utils_logging_internal_getGlobalLogTag(Result<cv::utils::logging::LogTag**>* ocvrs_return) {
		try {
			cv::utils::logging::LogTag* ret = cv::utils::logging::internal::getGlobalLogTag();
			Ok(new cv::utils::logging::LogTag*(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::utils::logging::LogTag**>))
	}
	
	// writeLogMessageEx(cv::utils::logging::LogLevel, const char *, const char *, int, const char *, const char *) /usr/include/opencv2/core/utils/logger.hpp:44
	void cv_utils_logging_internal_writeLogMessageEx_LogLevel_const_charX_const_charX_int_const_charX_const_charX(cv::utils::logging::LogLevel logLevel, const char* tag, const char* file, int line, const char* func, const char* message, Result_void* ocvrs_return) {
		try {
			cv::utils::logging::internal::writeLogMessageEx(logLevel, tag, file, line, func, message);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// writeLogMessage(cv::utils::logging::LogLevel, const char *) /usr/include/opencv2/core/utils/logger.hpp:41
	void cv_utils_logging_internal_writeLogMessage_LogLevel_const_charX(cv::utils::logging::LogLevel logLevel, const char* message, Result_void* ocvrs_return) {
		try {
			cv::utils::logging::internal::writeLogMessage(logLevel, message);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// registerLogTag(cv::utils::logging::LogTag *) /usr/include/opencv2/core/utils/logger.hpp:29
	void cv_utils_logging_registerLogTag_LogTagX(cv::utils::logging::LogTag* plogtag, Result_void* ocvrs_return) {
		try {
			cv::utils::logging::registerLogTag(plogtag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setLogLevel(cv::utils::logging::LogLevel) /usr/include/opencv2/core/utils/logger.hpp:25
	void cv_utils_logging_setLogLevel_LogLevel(cv::utils::logging::LogLevel logLevel, Result<cv::utils::logging::LogLevel>* ocvrs_return) {
		try {
			cv::utils::logging::LogLevel ret = cv::utils::logging::setLogLevel(logLevel);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::utils::logging::LogLevel>))
	}
	
	// setLogTagLevel(const char *, cv::utils::logging::LogLevel) /usr/include/opencv2/core/utils/logger.hpp:31
	void cv_utils_logging_setLogTagLevel_const_charX_LogLevel(const char* tag, cv::utils::logging::LogLevel level, Result_void* ocvrs_return) {
		try {
			cv::utils::logging::setLogTagLevel(tag, level);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// testEchoBooleanFunction(bool) /usr/include/opencv2/core/bindings_utils.hpp:223
	void cv_utils_nested_testEchoBooleanFunction_bool(bool flag, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::utils::nested::testEchoBooleanFunction(flag);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// testAsyncArray(cv::InputArray) /usr/include/opencv2/core/bindings_utils.hpp:200
	void cv_utils_testAsyncArray_const__InputArrayR(const cv::_InputArray* argument, Result<cv::AsyncArray*>* ocvrs_return) {
		try {
			cv::AsyncArray ret = cv::utils::testAsyncArray(*argument);
			Ok(new cv::AsyncArray(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::AsyncArray*>))
	}
	
	// testAsyncException() /usr/include/opencv2/core/bindings_utils.hpp:208
	void cv_utils_testAsyncException(Result<cv::AsyncArray*>* ocvrs_return) {
		try {
			cv::AsyncArray ret = cv::utils::testAsyncException();
			Ok(new cv::AsyncArray(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::AsyncArray*>))
	}
	
	// testOverloadResolution(const cv::Rect &) /usr/include/opencv2/core/bindings_utils.hpp:78
	void cv_utils_testOverloadResolution_const_RectR(const cv::Rect* rect, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::testOverloadResolution(*rect);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// testOverloadResolution(int, const cv::Point &) /usr/include/opencv2/core/bindings_utils.hpp:71
	void cv_utils_testOverloadResolution_int_const_PointR(int value, const cv::Point* point, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::testOverloadResolution(value, *point);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// testOverwriteNativeMethod(int) /usr/include/opencv2/core/bindings_utils.hpp:135
	void cv_utils_testOverwriteNativeMethod_int(int argument, Result<int>* ocvrs_return) {
		try {
			int ret = cv::utils::testOverwriteNativeMethod(argument);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// testRaiseGeneralException() /usr/include/opencv2/core/bindings_utils.hpp:194
	void cv_utils_testRaiseGeneralException(Result_void* ocvrs_return) {
		try {
			cv::utils::testRaiseGeneralException();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// testReservedKeywordConversion(int, int, int) /usr/include/opencv2/core/bindings_utils.hpp:141
	void cv_utils_testReservedKeywordConversion_int_int_int(int positional_argument, int lambda, int from, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::utils::testReservedKeywordConversion(positional_argument, lambda, from);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// testRotatedRectVector(float, float, float, float, float) /usr/include/opencv2/core/bindings_utils.hpp:113
	void cv_utils_testRotatedRectVector_float_float_float_float_float(float x, float y, float w, float h, float angle, Result<std::vector<cv::RotatedRect>*>* ocvrs_return) {
		try {
			std::vector<cv::RotatedRect> ret = cv::utils::testRotatedRectVector(x, y, w, h, angle);
			Ok(new std::vector<cv::RotatedRect>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::RotatedRect>*>))
	}
	
	// testRotatedRect(float, float, float, float, float) /usr/include/opencv2/core/bindings_utils.hpp:107
	void cv_utils_testRotatedRect_float_float_float_float_float(float x, float y, float w, float h, float angle, Result<cv::RotatedRect*>* ocvrs_return) {
		try {
			cv::RotatedRect ret = cv::utils::testRotatedRect(x, y, w, h, angle);
			Ok(new cv::RotatedRect(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RotatedRect*>))
	}
	
	// convertFromVASurface(VADisplay, VASurfaceID, cv::Size, cv::OutputArray) /usr/include/opencv2/core/va_intel.hpp:69
	void cv_va_intel_convertFromVASurface_VADisplay_VASurfaceID_Size_const__OutputArrayR(VADisplay display, VASurfaceID surface, cv::Size* size, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::va_intel::convertFromVASurface(display, surface, *size, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// convertToVASurface(VADisplay, cv::InputArray, VASurfaceID, cv::Size) /usr/include/opencv2/core/va_intel.hpp:61
	void cv_va_intel_convertToVASurface_VADisplay_const__InputArrayR_VASurfaceID_Size(VADisplay display, const cv::_InputArray* src, VASurfaceID surface, cv::Size* size, Result_void* ocvrs_return) {
		try {
			cv::va_intel::convertToVASurface(display, *src, surface, *size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// initializeContextFromVA(VADisplay, bool) /usr/include/opencv2/core/va_intel.hpp:51
	void cv_va_intel_ocl_initializeContextFromVA_VADisplay_bool(VADisplay display, bool tryInterop, Result<cv::ocl::Context*>* ocvrs_return) {
		try {
			cv::ocl::Context ret = cv::va_intel::ocl::initializeContextFromVA(display, tryInterop);
			Ok(new cv::ocl::Context(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Context*>))
	}
	
	// vconcat(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:1254
	void cv_vconcat_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::vconcat(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// vconcat(cv::InputArrayOfArrays, cv::OutputArray) /usr/include/opencv2/core.hpp:1272
	void cv_vconcat_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::vconcat(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// writeScalar(cv::FileStorage &, const cv::String &) /usr/include/opencv2/core/persistence.hpp:716
	void cv_writeScalar_FileStorageR_const_StringR(cv::FileStorage* fs, const char* value, Result_void* ocvrs_return) {
		try {
			cv::writeScalar(*fs, std::string(value));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// writeScalar(cv::FileStorage &, double) /usr/include/opencv2/core/persistence.hpp:715
	void cv_writeScalar_FileStorageR_double(cv::FileStorage* fs, double value, Result_void* ocvrs_return) {
		try {
			cv::writeScalar(*fs, value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// writeScalar(cv::FileStorage &, float) /usr/include/opencv2/core/persistence.hpp:714
	void cv_writeScalar_FileStorageR_float(cv::FileStorage* fs, float value, Result_void* ocvrs_return) {
		try {
			cv::writeScalar(*fs, value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// writeScalar(cv::FileStorage &, int) /usr/include/opencv2/core/persistence.hpp:713
	void cv_writeScalar_FileStorageR_int(cv::FileStorage* fs, int value, Result_void* ocvrs_return) {
		try {
			cv::writeScalar(*fs, value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &, const cv::String &, const cv::Mat &) /usr/include/opencv2/core/persistence.hpp:706
	void cv_write_FileStorageR_const_StringR_const_MatR(cv::FileStorage* fs, const char* name, const cv::Mat* value, Result_void* ocvrs_return) {
		try {
			cv::write(*fs, std::string(name), *value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &, const cv::String &, const cv::SparseMat &) /usr/include/opencv2/core/persistence.hpp:707
	void cv_write_FileStorageR_const_StringR_const_SparseMatR(cv::FileStorage* fs, const char* name, const cv::SparseMat* value, Result_void* ocvrs_return) {
		try {
			cv::write(*fs, std::string(name), *value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &, const cv::String &, const cv::String &) /usr/include/opencv2/core/persistence.hpp:705
	void cv_write_FileStorageR_const_StringR_const_StringR(cv::FileStorage* fs, const char* name, const char* value, Result_void* ocvrs_return) {
		try {
			cv::write(*fs, std::string(name), std::string(value));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &, const cv::String &, const std::vector<DMatch> &) /usr/include/opencv2/core/persistence.hpp:710
	void cv_write_FileStorageR_const_StringR_const_vector_DMatch_R(cv::FileStorage* fs, const char* name, const std::vector<cv::DMatch>* value, Result_void* ocvrs_return) {
		try {
			cv::write(*fs, std::string(name), *value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &, const cv::String &, const std::vector<KeyPoint> &) /usr/include/opencv2/core/persistence.hpp:709
	void cv_write_FileStorageR_const_StringR_const_vector_KeyPoint_R(cv::FileStorage* fs, const char* name, const std::vector<cv::KeyPoint>* value, Result_void* ocvrs_return) {
		try {
			cv::write(*fs, std::string(name), *value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &, const cv::String &, double) /usr/include/opencv2/core/persistence.hpp:704
	void cv_write_FileStorageR_const_StringR_double(cv::FileStorage* fs, const char* name, double value, Result_void* ocvrs_return) {
		try {
			cv::write(*fs, std::string(name), value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &, const cv::String &, float) /usr/include/opencv2/core/persistence.hpp:703
	void cv_write_FileStorageR_const_StringR_float(cv::FileStorage* fs, const char* name, float value, Result_void* ocvrs_return) {
		try {
			cv::write(*fs, std::string(name), value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &, const cv::String &, int) /usr/include/opencv2/core/persistence.hpp:702
	void cv_write_FileStorageR_const_StringR_int(cv::FileStorage* fs, const char* name, int value, Result_void* ocvrs_return) {
		try {
			cv::write(*fs, std::string(name), value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Algorithm_delete(cv::Algorithm* instance) {
		delete instance;
	}
	// Algorithm() /usr/include/opencv2/core.hpp:3136
	void cv_Algorithm_Algorithm(Result<cv::Algorithm*>* ocvrs_return) {
		try {
			cv::Algorithm* ret = new cv::Algorithm();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Algorithm*>))
	}
	
	// clear() /usr/include/opencv2/core.hpp:3141
	void cv_Algorithm_clear(cv::Algorithm* instance, Result_void* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &) /usr/include/opencv2/core.hpp:3145
	void cv_Algorithm_write_const_FileStorageR(const cv::Algorithm* instance, cv::FileStorage* fs, Result_void* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(const Ptr<cv::FileStorage> &, const cv::String &) /usr/include/opencv2/core.hpp:3150
	void cv_Algorithm_write_const_const_Ptr_FileStorage_R_const_StringR(const cv::Algorithm* instance, const cv::Ptr<cv::FileStorage>* fs, const char* name, Result_void* ocvrs_return) {
		try {
			instance->write(*fs, std::string(name));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// read(const cv::FileNode &) /usr/include/opencv2/core.hpp:3154
	void cv_Algorithm_read_const_FileNodeR(cv::Algorithm* instance, const cv::FileNode* fn, Result_void* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// empty() /usr/include/opencv2/core.hpp:3158
	void cv_Algorithm_empty_const(const cv::Algorithm* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// save(const cv::String &) /usr/include/opencv2/core.hpp:3222
	void cv_Algorithm_save_const_const_StringR(const cv::Algorithm* instance, const char* filename, Result_void* ocvrs_return) {
		try {
			instance->save(std::string(filename));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDefaultName() /usr/include/opencv2/core.hpp:3226
	void cv_Algorithm_getDefaultName_const(const cv::Algorithm* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getDefaultName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	void cv_AsyncArray_delete(cv::AsyncArray* instance) {
		delete instance;
	}
	// AsyncArray() /usr/include/opencv2/core/async.hpp:35
	cv::AsyncArray* cv_AsyncArray_AsyncArray() {
			cv::AsyncArray* ret = new cv::AsyncArray();
			return ret;
	}
	
	// AsyncArray(const cv::AsyncArray &) /usr/include/opencv2/core/async.hpp:36
	cv::AsyncArray* cv_AsyncArray_AsyncArray_const_AsyncArrayR(const cv::AsyncArray* o) {
			cv::AsyncArray* ret = new cv::AsyncArray(*o);
			return ret;
	}
	
	// release() /usr/include/opencv2/core/async.hpp:38
	void cv_AsyncArray_release(cv::AsyncArray* instance) {
			instance->release();
	}
	
	// get(cv::OutputArray) /usr/include/opencv2/core/async.hpp:50
	void cv_AsyncArray_get_const_const__OutputArrayR(const cv::AsyncArray* instance, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			instance->get(*dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// get(cv::OutputArray, int64) /usr/include/opencv2/core/async.hpp:60
	void cv_AsyncArray_get_const_const__OutputArrayR_int64_t(const cv::AsyncArray* instance, const cv::_OutputArray* dst, int64_t timeoutNs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->get(*dst, timeoutNs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// get(cv::OutputArray, double) /usr/include/opencv2/core/async.hpp:63
	void cv_AsyncArray_get_const_const__OutputArrayR_double(const cv::AsyncArray* instance, const cv::_OutputArray* dst, double timeoutNs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->get(*dst, timeoutNs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// wait_for(int64) /usr/include/opencv2/core/async.hpp:65
	void cv_AsyncArray_wait_for_const_int64_t(const cv::AsyncArray* instance, int64_t timeoutNs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->wait_for(timeoutNs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// wait_for(double) /usr/include/opencv2/core/async.hpp:68
	void cv_AsyncArray_wait_for_const_double(const cv::AsyncArray* instance, double timeoutNs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->wait_for(timeoutNs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// valid() /usr/include/opencv2/core/async.hpp:70
	bool cv_AsyncArray_valid_const(const cv::AsyncArray* instance) {
			bool ret = instance->valid();
			return ret;
	}
	
	// AsyncArray(cv::AsyncArray &&) /usr/include/opencv2/core/async.hpp:73
	void cv_AsyncArray_AsyncArray_AsyncArrayR(cv::AsyncArray* o, Result<cv::AsyncArray*>* ocvrs_return) {
		try {
			cv::AsyncArray* ret = new cv::AsyncArray(*o);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::AsyncArray*>))
	}
	
	void cv_AsyncPromise_delete(cv::AsyncPromise* instance) {
		delete instance;
	}
	// AsyncPromise() /usr/include/opencv2/core/detail/async_promise.hpp:26
	cv::AsyncPromise* cv_AsyncPromise_AsyncPromise() {
			cv::AsyncPromise* ret = new cv::AsyncPromise();
			return ret;
	}
	
	// AsyncPromise(const cv::AsyncPromise &) /usr/include/opencv2/core/detail/async_promise.hpp:27
	cv::AsyncPromise* cv_AsyncPromise_AsyncPromise_const_AsyncPromiseR(const cv::AsyncPromise* o) {
			cv::AsyncPromise* ret = new cv::AsyncPromise(*o);
			return ret;
	}
	
	// release() /usr/include/opencv2/core/detail/async_promise.hpp:29
	void cv_AsyncPromise_release(cv::AsyncPromise* instance) {
			instance->release();
	}
	
	// getArrayResult() /usr/include/opencv2/core/detail/async_promise.hpp:34
	void cv_AsyncPromise_getArrayResult(cv::AsyncPromise* instance, Result<cv::AsyncArray*>* ocvrs_return) {
		try {
			cv::AsyncArray ret = instance->getArrayResult();
			Ok(new cv::AsyncArray(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::AsyncArray*>))
	}
	
	// setValue(cv::InputArray) /usr/include/opencv2/core/detail/async_promise.hpp:39
	void cv_AsyncPromise_setValue_const__InputArrayR(cv::AsyncPromise* instance, const cv::_InputArray* value, Result_void* ocvrs_return) {
		try {
			instance->setValue(*value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setException(const cv::Exception &) /usr/include/opencv2/core/detail/async_promise.hpp:53
	void cv_AsyncPromise_setException_const_ExceptionR(cv::AsyncPromise* instance, const cv::Exception* exception, Result_void* ocvrs_return) {
		try {
			instance->setException(*exception);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// AsyncPromise(cv::AsyncPromise &&) /usr/include/opencv2/core/detail/async_promise.hpp:56
	void cv_AsyncPromise_AsyncPromise_AsyncPromiseR(cv::AsyncPromise* o, Result<cv::AsyncPromise*>* ocvrs_return) {
		try {
			cv::AsyncPromise* ret = new cv::AsyncPromise(*o);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::AsyncPromise*>))
	}
	
	// _getImpl() /usr/include/opencv2/core/detail/async_promise.hpp:63
	void* cv_AsyncPromise__getImpl_const(const cv::AsyncPromise* instance) {
			void* ret = instance->_getImpl();
			return ret;
	}
	
	void cv_CommandLineParser_delete(cv::CommandLineParser* instance) {
		delete instance;
	}
	// CommandLineParser(int, const char *const *, const cv::String &) /usr/include/opencv2/core/utility.hpp:829
	void cv_CommandLineParser_CommandLineParser_int_const_charXX_const_StringR(int argc, const char** argv, const char* keys, Result<cv::CommandLineParser*>* ocvrs_return) {
		try {
			cv::CommandLineParser* ret = new cv::CommandLineParser(argc, argv, std::string(keys));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::CommandLineParser*>))
	}
	
	// CommandLineParser(const cv::CommandLineParser &) /usr/include/opencv2/core/utility.hpp:832
	void cv_CommandLineParser_CommandLineParser_const_CommandLineParserR(const cv::CommandLineParser* parser, Result<cv::CommandLineParser*>* ocvrs_return) {
		try {
			cv::CommandLineParser* ret = new cv::CommandLineParser(*parser);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::CommandLineParser*>))
	}
	
	// getPathToApplication() /usr/include/opencv2/core/utility.hpp:850
	void cv_CommandLineParser_getPathToApplication_const(const cv::CommandLineParser* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getPathToApplication();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// has(const cv::String &) /usr/include/opencv2/core/utility.hpp:927
	void cv_CommandLineParser_has_const_const_StringR(const cv::CommandLineParser* instance, const char* name, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->has(std::string(name));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// check() /usr/include/opencv2/core/utility.hpp:934
	void cv_CommandLineParser_check_const(const cv::CommandLineParser* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->check();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// about(const cv::String &) /usr/include/opencv2/core/utility.hpp:940
	void cv_CommandLineParser_about_const_StringR(cv::CommandLineParser* instance, const char* message, Result_void* ocvrs_return) {
		try {
			instance->about(std::string(message));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// printMessage() /usr/include/opencv2/core/utility.hpp:948
	void cv_CommandLineParser_printMessage_const(const cv::CommandLineParser* instance, Result_void* ocvrs_return) {
		try {
			instance->printMessage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// printErrors() /usr/include/opencv2/core/utility.hpp:954
	void cv_CommandLineParser_printErrors_const(const cv::CommandLineParser* instance, Result_void* ocvrs_return) {
		try {
			instance->printErrors();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(const Ptr<MinProblemSolver::Function> &, cv::TermCriteria) /usr/include/opencv2/core/optim.hpp:252
	void cv_ConjGradSolver_create_const_Ptr_Function_R_TermCriteria(const cv::Ptr<cv::MinProblemSolver::Function>* f, cv::TermCriteria* termcrit, Result<cv::Ptr<cv::ConjGradSolver>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ConjGradSolver> ret = cv::ConjGradSolver::create(*f, *termcrit);
			Ok(new cv::Ptr<cv::ConjGradSolver>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ConjGradSolver>*>))
	}
	
	// DMatch() /usr/include/opencv2/core/types.hpp:833
	void cv_DMatch_DMatch(Result<cv::DMatch>* ocvrs_return) {
		try {
			cv::DMatch ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::DMatch>))
	}
	
	// DMatch(int, int, float) /usr/include/opencv2/core/types.hpp:834
	void cv_DMatch_DMatch_int_int_float(int _queryIdx, int _trainIdx, float _distance, Result<cv::DMatch>* ocvrs_return) {
		try {
			cv::DMatch ret(_queryIdx, _trainIdx, _distance);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::DMatch>))
	}
	
	// DMatch(int, int, int, float) /usr/include/opencv2/core/types.hpp:835
	void cv_DMatch_DMatch_int_int_int_float(int _queryIdx, int _trainIdx, int _imgIdx, float _distance, Result<cv::DMatch>* ocvrs_return) {
		try {
			cv::DMatch ret(_queryIdx, _trainIdx, _imgIdx, _distance);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::DMatch>))
	}
	
	// getInitStep(cv::OutputArray) /usr/include/opencv2/core/optim.hpp:164
	void cv_DownhillSolver_getInitStep_const_const__OutputArrayR(const cv::DownhillSolver* instance, const cv::_OutputArray* step, Result_void* ocvrs_return) {
		try {
			instance->getInitStep(*step);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setInitStep(cv::InputArray) /usr/include/opencv2/core/optim.hpp:180
	void cv_DownhillSolver_setInitStep_const__InputArrayR(cv::DownhillSolver* instance, const cv::_InputArray* step, Result_void* ocvrs_return) {
		try {
			instance->setInitStep(*step);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(const Ptr<MinProblemSolver::Function> &, cv::InputArray, cv::TermCriteria) /usr/include/opencv2/core/optim.hpp:198
	void cv_DownhillSolver_create_const_Ptr_Function_R_const__InputArrayR_TermCriteria(const cv::Ptr<cv::MinProblemSolver::Function>* f, const cv::_InputArray* initStep, cv::TermCriteria* termcrit, Result<cv::Ptr<cv::DownhillSolver>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::DownhillSolver> ret = cv::DownhillSolver::create(*f, *initStep, *termcrit);
			Ok(new cv::Ptr<cv::DownhillSolver>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::DownhillSolver>*>))
	}
	
	// msg /usr/include/opencv2/core.hpp:138
	void* cv_Exception_getPropMsg_const(const cv::Exception* instance) {
			cv::String ret = instance->msg;
			return ocvrs_create_string(ret.c_str());
	}
	
	// msg /usr/include/opencv2/core.hpp:138
	void cv_Exception_setPropMsg_String(cv::Exception* instance, char* val) {
			instance->msg = std::string(val);
	}
	
	// code /usr/include/opencv2/core.hpp:140
	int cv_Exception_getPropCode_const(const cv::Exception* instance) {
			int ret = instance->code;
			return ret;
	}
	
	// code /usr/include/opencv2/core.hpp:140
	void cv_Exception_setPropCode_int(cv::Exception* instance, int val) {
			instance->code = val;
	}
	
	// err /usr/include/opencv2/core.hpp:141
	void* cv_Exception_getPropErr_const(const cv::Exception* instance) {
			cv::String ret = instance->err;
			return ocvrs_create_string(ret.c_str());
	}
	
	// err /usr/include/opencv2/core.hpp:141
	void cv_Exception_setPropErr_String(cv::Exception* instance, char* val) {
			instance->err = std::string(val);
	}
	
	// func /usr/include/opencv2/core.hpp:142
	void* cv_Exception_getPropFunc_const(const cv::Exception* instance) {
			cv::String ret = instance->func;
			return ocvrs_create_string(ret.c_str());
	}
	
	// func /usr/include/opencv2/core.hpp:142
	void cv_Exception_setPropFunc_String(cv::Exception* instance, char* val) {
			instance->func = std::string(val);
	}
	
	// file /usr/include/opencv2/core.hpp:143
	void* cv_Exception_getPropFile_const(const cv::Exception* instance) {
			cv::String ret = instance->file;
			return ocvrs_create_string(ret.c_str());
	}
	
	// file /usr/include/opencv2/core.hpp:143
	void cv_Exception_setPropFile_String(cv::Exception* instance, char* val) {
			instance->file = std::string(val);
	}
	
	// line /usr/include/opencv2/core.hpp:144
	int cv_Exception_getPropLine_const(const cv::Exception* instance) {
			int ret = instance->line;
			return ret;
	}
	
	// line /usr/include/opencv2/core.hpp:144
	void cv_Exception_setPropLine_int(cv::Exception* instance, int val) {
			instance->line = val;
	}
	
	void cv_Exception_delete(cv::Exception* instance) {
		delete instance;
	}
	// Exception() /usr/include/opencv2/core.hpp:124
	void cv_Exception_Exception(Result<cv::Exception*>* ocvrs_return) {
		try {
			cv::Exception* ret = new cv::Exception();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Exception*>))
	}
	
	// Exception(int, const cv::String &, const cv::String &, const cv::String &, int) /usr/include/opencv2/core.hpp:129
	void cv_Exception_Exception_int_const_StringR_const_StringR_const_StringR_int(int _code, const char* _err, const char* _func, const char* _file, int _line, Result<cv::Exception*>* ocvrs_return) {
		try {
			cv::Exception* ret = new cv::Exception(_code, std::string(_err), std::string(_func), std::string(_file), _line);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Exception*>))
	}
	
	// what() /usr/include/opencv2/core.hpp:135
	void cv_Exception_what_const(const cv::Exception* instance, Result<void*>* ocvrs_return) {
		try {
			const char* ret = instance->what();
			Ok(ocvrs_create_string(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// formatMessage() /usr/include/opencv2/core.hpp:136
	void cv_Exception_formatMessage(cv::Exception* instance, Result_void* ocvrs_return) {
		try {
			instance->formatMessage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// blockIdx /usr/include/opencv2/core/persistence.hpp:623
	size_t cv_FileNode_getPropBlockIdx_const(const cv::FileNode* instance) {
			size_t ret = instance->blockIdx;
			return ret;
	}
	
	// blockIdx /usr/include/opencv2/core/persistence.hpp:623
	void cv_FileNode_setPropBlockIdx_size_t(cv::FileNode* instance, size_t val) {
			instance->blockIdx = val;
	}
	
	// ofs /usr/include/opencv2/core/persistence.hpp:624
	size_t cv_FileNode_getPropOfs_const(const cv::FileNode* instance) {
			size_t ret = instance->ofs;
			return ret;
	}
	
	// ofs /usr/include/opencv2/core/persistence.hpp:624
	void cv_FileNode_setPropOfs_size_t(cv::FileNode* instance, size_t val) {
			instance->ofs = val;
	}
	
	void cv_FileNode_delete(cv::FileNode* instance) {
		delete instance;
	}
	// FileNode() /usr/include/opencv2/core/persistence.hpp:508
	void cv_FileNode_FileNode(Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode* ret = new cv::FileNode();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	// FileNode(const cv::FileStorage *, size_t, size_t) /usr/include/opencv2/core/persistence.hpp:517
	void cv_FileNode_FileNode_const_FileStorageX_size_t_size_t(const cv::FileStorage* fs, size_t blockIdx, size_t ofs, Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode* ret = new cv::FileNode(fs, blockIdx, ofs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	// FileNode(const cv::FileNode &) /usr/include/opencv2/core/persistence.hpp:522
	void cv_FileNode_FileNode_const_FileNodeR(const cv::FileNode* node, Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode* ret = new cv::FileNode(*node);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	// operator[](const cv::String &) /usr/include/opencv2/core/persistence.hpp:530
	void cv_FileNode_operator___const_const_StringR(const cv::FileNode* instance, const char* nodename, Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode ret = instance->operator[](std::string(nodename));
			Ok(new cv::FileNode(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	// operator[](const char *) /usr/include/opencv2/core/persistence.hpp:535
	void cv_FileNode_operator___const_const_charX(const cv::FileNode* instance, const char* nodename, Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode ret = instance->operator[](nodename);
			Ok(new cv::FileNode(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	// operator[](int) /usr/include/opencv2/core/persistence.hpp:540
	void cv_FileNode_operator___const_int(const cv::FileNode* instance, int i, Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode ret = instance->operator[](i);
			Ok(new cv::FileNode(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	// keys() /usr/include/opencv2/core/persistence.hpp:545
	void cv_FileNode_keys_const(const cv::FileNode* instance, Result<std::vector<cv::String>*>* ocvrs_return) {
		try {
			std::vector<cv::String> ret = instance->keys();
			Ok(new std::vector<cv::String>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::String>*>))
	}
	
	// type() /usr/include/opencv2/core/persistence.hpp:550
	void cv_FileNode_type_const(const cv::FileNode* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// empty() /usr/include/opencv2/core/persistence.hpp:553
	void cv_FileNode_empty_const(const cv::FileNode* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isNone() /usr/include/opencv2/core/persistence.hpp:555
	void cv_FileNode_isNone_const(const cv::FileNode* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isNone();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isSeq() /usr/include/opencv2/core/persistence.hpp:557
	void cv_FileNode_isSeq_const(const cv::FileNode* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isSeq();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isMap() /usr/include/opencv2/core/persistence.hpp:559
	void cv_FileNode_isMap_const(const cv::FileNode* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isMap();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isInt() /usr/include/opencv2/core/persistence.hpp:561
	void cv_FileNode_isInt_const(const cv::FileNode* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isInt();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isReal() /usr/include/opencv2/core/persistence.hpp:563
	void cv_FileNode_isReal_const(const cv::FileNode* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isReal();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isString() /usr/include/opencv2/core/persistence.hpp:565
	void cv_FileNode_isString_const(const cv::FileNode* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isString();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isNamed() /usr/include/opencv2/core/persistence.hpp:567
	void cv_FileNode_isNamed_const(const cv::FileNode* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isNamed();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// name() /usr/include/opencv2/core/persistence.hpp:569
	void cv_FileNode_name_const(const cv::FileNode* instance, Result<void*>* ocvrs_return) {
		try {
			std::string ret = instance->name();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// size() /usr/include/opencv2/core/persistence.hpp:571
	void cv_FileNode_size_const(const cv::FileNode* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// rawSize() /usr/include/opencv2/core/persistence.hpp:573
	void cv_FileNode_rawSize_const(const cv::FileNode* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->rawSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// operator int() /usr/include/opencv2/core/persistence.hpp:575
	void cv_FileNode_operator_int_const(const cv::FileNode* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->operator int();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// operator float() /usr/include/opencv2/core/persistence.hpp:577
	void cv_FileNode_operator_float_const(const cv::FileNode* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->operator float();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// operator double() /usr/include/opencv2/core/persistence.hpp:579
	void cv_FileNode_operator_double_const(const cv::FileNode* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->operator double();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// operator basic_string() /usr/include/opencv2/core/persistence.hpp:581
	void cv_FileNode_operator_std_string_const(const cv::FileNode* instance, Result<void*>* ocvrs_return) {
		try {
			std::string ret = instance->operator std::string();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// isMap(int) /usr/include/opencv2/core/persistence.hpp:583
	void cv_FileNode_isMap_int(int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::FileNode::isMap(flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isSeq(int) /usr/include/opencv2/core/persistence.hpp:584
	void cv_FileNode_isSeq_int(int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::FileNode::isSeq(flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isCollection(int) /usr/include/opencv2/core/persistence.hpp:585
	void cv_FileNode_isCollection_int(int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::FileNode::isCollection(flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isEmptyCollection(int) /usr/include/opencv2/core/persistence.hpp:586
	void cv_FileNode_isEmptyCollection_int(int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::FileNode::isEmptyCollection(flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isFlow(int) /usr/include/opencv2/core/persistence.hpp:587
	void cv_FileNode_isFlow_int(int flags, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::FileNode::isFlow(flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// ptr() /usr/include/opencv2/core/persistence.hpp:589
	void cv_FileNode_ptr(cv::FileNode* instance, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	// ptr() /usr/include/opencv2/core/persistence.hpp:590
	void cv_FileNode_ptr_const(const cv::FileNode* instance, Result<const unsigned char*>* ocvrs_return) {
		try {
			const unsigned char* ret = instance->ptr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	// begin() /usr/include/opencv2/core/persistence.hpp:593
	void cv_FileNode_begin_const(const cv::FileNode* instance, Result<cv::FileNodeIterator*>* ocvrs_return) {
		try {
			cv::FileNodeIterator ret = instance->begin();
			Ok(new cv::FileNodeIterator(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNodeIterator*>))
	}
	
	// end() /usr/include/opencv2/core/persistence.hpp:595
	void cv_FileNode_end_const(const cv::FileNode* instance, Result<cv::FileNodeIterator*>* ocvrs_return) {
		try {
			cv::FileNodeIterator ret = instance->end();
			Ok(new cv::FileNodeIterator(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNodeIterator*>))
	}
	
	// readRaw(const cv::String &, void *, size_t) /usr/include/opencv2/core/persistence.hpp:605
	void cv_FileNode_readRaw_const_const_StringR_voidX_size_t(const cv::FileNode* instance, const char* fmt, void* vec, size_t len, Result_void* ocvrs_return) {
		try {
			instance->readRaw(std::string(fmt), vec, len);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setValue(int, const void *, int) /usr/include/opencv2/core/persistence.hpp:610
	void cv_FileNode_setValue_int_const_voidX_int(cv::FileNode* instance, int type, const void* value, int len, Result_void* ocvrs_return) {
		try {
			instance->setValue(type, value, len);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// real() /usr/include/opencv2/core/persistence.hpp:613
	void cv_FileNode_real_const(const cv::FileNode* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->real();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// string() /usr/include/opencv2/core/persistence.hpp:615
	void cv_FileNode_string_const(const cv::FileNode* instance, Result<void*>* ocvrs_return) {
		try {
			std::string ret = instance->string();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// mat() /usr/include/opencv2/core/persistence.hpp:617
	void cv_FileNode_mat_const(const cv::FileNode* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->mat();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	void cv_FileNodeIterator_delete(cv::FileNodeIterator* instance) {
		delete instance;
	}
	// FileNodeIterator() /usr/include/opencv2/core/persistence.hpp:641
	void cv_FileNodeIterator_FileNodeIterator(Result<cv::FileNodeIterator*>* ocvrs_return) {
		try {
			cv::FileNodeIterator* ret = new cv::FileNodeIterator();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNodeIterator*>))
	}
	
	// FileNodeIterator(const cv::FileNode &, bool) /usr/include/opencv2/core/persistence.hpp:651
	void cv_FileNodeIterator_FileNodeIterator_const_FileNodeR_bool(const cv::FileNode* node, bool seekEnd, Result<cv::FileNodeIterator*>* ocvrs_return) {
		try {
			cv::FileNodeIterator* ret = new cv::FileNodeIterator(*node, seekEnd);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNodeIterator*>))
	}
	
	// FileNodeIterator(const cv::FileNodeIterator &) /usr/include/opencv2/core/persistence.hpp:656
	void cv_FileNodeIterator_FileNodeIterator_const_FileNodeIteratorR(const cv::FileNodeIterator* it, Result<cv::FileNodeIterator*>* ocvrs_return) {
		try {
			cv::FileNodeIterator* ret = new cv::FileNodeIterator(*it);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNodeIterator*>))
	}
	
	// operator*() /usr/include/opencv2/core/persistence.hpp:661
	void cv_FileNodeIterator_operatorX_const(const cv::FileNodeIterator* instance, Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode ret = instance->operator*();
			Ok(new cv::FileNode(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	// operator++() /usr/include/opencv2/core/persistence.hpp:664
	void cv_FileNodeIterator_operatorAA(cv::FileNodeIterator* instance, Result<cv::FileNodeIterator*>* ocvrs_return) {
		try {
			cv::FileNodeIterator ret = instance->operator++();
			Ok(new cv::FileNodeIterator(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNodeIterator*>))
	}
	
	// readRaw(const cv::String &, void *, size_t) /usr/include/opencv2/core/persistence.hpp:678
	void cv_FileNodeIterator_readRaw_const_StringR_voidX_size_t(cv::FileNodeIterator* instance, const char* fmt, void* vec, size_t len, Result<cv::FileNodeIterator*>* ocvrs_return) {
		try {
			cv::FileNodeIterator ret = instance->readRaw(std::string(fmt), vec, len);
			Ok(new cv::FileNodeIterator(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNodeIterator*>))
	}
	
	// remaining() /usr/include/opencv2/core/persistence.hpp:682
	void cv_FileNodeIterator_remaining_const(const cv::FileNodeIterator* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->remaining();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// equalTo(const cv::FileNodeIterator &) /usr/include/opencv2/core/persistence.hpp:684
	void cv_FileNodeIterator_equalTo_const_const_FileNodeIteratorR(const cv::FileNodeIterator* instance, const cv::FileNodeIterator* it, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->equalTo(*it);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// state /usr/include/opencv2/core/persistence.hpp:462
	int cv_FileStorage_getPropState_const(const cv::FileStorage* instance) {
			int ret = instance->state;
			return ret;
	}
	
	// state /usr/include/opencv2/core/persistence.hpp:462
	void cv_FileStorage_setPropState_int(cv::FileStorage* instance, int val) {
			instance->state = val;
	}
	
	// elname /usr/include/opencv2/core/persistence.hpp:463
	void* cv_FileStorage_getPropElname_const(const cv::FileStorage* instance) {
			std::string ret = instance->elname;
			return ocvrs_create_string(ret.c_str());
	}
	
	// elname /usr/include/opencv2/core/persistence.hpp:463
	void cv_FileStorage_setPropElname_string(cv::FileStorage* instance, char* val) {
			instance->elname = std::string(val);
	}
	
	void cv_FileStorage_delete(cv::FileStorage* instance) {
		delete instance;
	}
	// FileStorage() /usr/include/opencv2/core/persistence.hpp:336
	void cv_FileStorage_FileStorage(Result<cv::FileStorage*>* ocvrs_return) {
		try {
			cv::FileStorage* ret = new cv::FileStorage();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileStorage*>))
	}
	
	// FileStorage(const cv::String &, int, const cv::String &) /usr/include/opencv2/core/persistence.hpp:341
	void cv_FileStorage_FileStorage_const_StringR_int_const_StringR(const char* filename, int flags, const char* encoding, Result<cv::FileStorage*>* ocvrs_return) {
		try {
			cv::FileStorage* ret = new cv::FileStorage(std::string(filename), flags, std::string(encoding));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileStorage*>))
	}
	
	// open(const cv::String &, int, const cv::String &) /usr/include/opencv2/core/persistence.hpp:361
	void cv_FileStorage_open_const_StringR_int_const_StringR(cv::FileStorage* instance, const char* filename, int flags, const char* encoding, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->open(std::string(filename), flags, std::string(encoding));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isOpened() /usr/include/opencv2/core/persistence.hpp:368
	void cv_FileStorage_isOpened_const(const cv::FileStorage* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isOpened();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// release() /usr/include/opencv2/core/persistence.hpp:374
	void cv_FileStorage_release(cv::FileStorage* instance, Result_void* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// releaseAndGetString() /usr/include/opencv2/core/persistence.hpp:381
	void cv_FileStorage_releaseAndGetString(cv::FileStorage* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->releaseAndGetString();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// getFirstTopLevelNode() /usr/include/opencv2/core/persistence.hpp:386
	void cv_FileStorage_getFirstTopLevelNode_const(const cv::FileStorage* instance, Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode ret = instance->getFirstTopLevelNode();
			Ok(new cv::FileNode(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	// root(int) /usr/include/opencv2/core/persistence.hpp:393
	void cv_FileStorage_root_const_int(const cv::FileStorage* instance, int streamidx, Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode ret = instance->root(streamidx);
			Ok(new cv::FileNode(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	// operator[](const cv::String &) /usr/include/opencv2/core/persistence.hpp:399
	void cv_FileStorage_operator___const_const_StringR(const cv::FileStorage* instance, const char* nodename, Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode ret = instance->operator[](std::string(nodename));
			Ok(new cv::FileNode(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	// operator[](const char *) /usr/include/opencv2/core/persistence.hpp:402
	void cv_FileStorage_operator___const_const_charX(const cv::FileStorage* instance, const char* nodename, Result<cv::FileNode*>* ocvrs_return) {
		try {
			cv::FileNode ret = instance->operator[](nodename);
			Ok(new cv::FileNode(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::FileNode*>))
	}
	
	// write(const cv::String &, int) /usr/include/opencv2/core/persistence.hpp:409
	void cv_FileStorage_write_const_StringR_int(cv::FileStorage* instance, const char* name, int val, Result_void* ocvrs_return) {
		try {
			instance->write(std::string(name), val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(const cv::String &, double) /usr/include/opencv2/core/persistence.hpp:411
	void cv_FileStorage_write_const_StringR_double(cv::FileStorage* instance, const char* name, double val, Result_void* ocvrs_return) {
		try {
			instance->write(std::string(name), val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(const cv::String &, const cv::String &) /usr/include/opencv2/core/persistence.hpp:413
	void cv_FileStorage_write_const_StringR_const_StringR(cv::FileStorage* instance, const char* name, const char* val, Result_void* ocvrs_return) {
		try {
			instance->write(std::string(name), std::string(val));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(const cv::String &, const cv::Mat &) /usr/include/opencv2/core/persistence.hpp:415
	void cv_FileStorage_write_const_StringR_const_MatR(cv::FileStorage* instance, const char* name, const cv::Mat* val, Result_void* ocvrs_return) {
		try {
			instance->write(std::string(name), *val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(const cv::String &, const std::vector<String> &) /usr/include/opencv2/core/persistence.hpp:417
	void cv_FileStorage_write_const_StringR_const_vector_String_R(cv::FileStorage* instance, const char* name, const std::vector<cv::String>* val, Result_void* ocvrs_return) {
		try {
			instance->write(std::string(name), *val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// writeRaw(const cv::String &, const void *, size_t) /usr/include/opencv2/core/persistence.hpp:427
	void cv_FileStorage_writeRaw_const_StringR_const_voidX_size_t(cv::FileStorage* instance, const char* fmt, const void* vec, size_t len, Result_void* ocvrs_return) {
		try {
			instance->writeRaw(std::string(fmt), vec, len);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// writeComment(const cv::String &, bool) /usr/include/opencv2/core/persistence.hpp:437
	void cv_FileStorage_writeComment_const_StringR_bool(cv::FileStorage* instance, const char* comment, bool append, Result_void* ocvrs_return) {
		try {
			instance->writeComment(std::string(comment), append);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// startWriteStruct(const cv::String &, int, const cv::String &) /usr/include/opencv2/core/persistence.hpp:445
	void cv_FileStorage_startWriteStruct_const_StringR_int_const_StringR(cv::FileStorage* instance, const char* name, int flags, const char* typeName, Result_void* ocvrs_return) {
		try {
			instance->startWriteStruct(std::string(name), flags, std::string(typeName));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// endWriteStruct() /usr/include/opencv2/core/persistence.hpp:449
	void cv_FileStorage_endWriteStruct(cv::FileStorage* instance, Result_void* ocvrs_return) {
		try {
			instance->endWriteStruct();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDefaultObjectName(const cv::String &) /usr/include/opencv2/core/persistence.hpp:455
	void cv_FileStorage_getDefaultObjectName_const_StringR(const char* filename, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::FileStorage::getDefaultObjectName(std::string(filename));
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// getFormat() /usr/include/opencv2/core/persistence.hpp:460
	void cv_FileStorage_getFormat_const(const cv::FileStorage* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFormat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// next() /usr/include/opencv2/core.hpp:3069
	void cv_Formatted_next(cv::Formatted* instance, Result<void*>* ocvrs_return) {
		try {
			const char* ret = instance->next();
			Ok(ocvrs_create_string(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// reset() /usr/include/opencv2/core.hpp:3070
	void cv_Formatted_reset(cv::Formatted* instance, Result_void* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// format(const cv::Mat &) /usr/include/opencv2/core.hpp:3089
	void cv_Formatter_format_const_const_MatR(const cv::Formatter* instance, const cv::Mat* mtx, Result<cv::Ptr<cv::Formatted>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Formatted> ret = instance->format(*mtx);
			Ok(new cv::Ptr<cv::Formatted>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::Formatted>*>))
	}
	
	// set16fPrecision(int) /usr/include/opencv2/core.hpp:3091
	void cv_Formatter_set16fPrecision_int(cv::Formatter* instance, int p, Result_void* ocvrs_return) {
		try {
			instance->set16fPrecision(p);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// set32fPrecision(int) /usr/include/opencv2/core.hpp:3092
	void cv_Formatter_set32fPrecision_int(cv::Formatter* instance, int p, Result_void* ocvrs_return) {
		try {
			instance->set32fPrecision(p);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// set64fPrecision(int) /usr/include/opencv2/core.hpp:3093
	void cv_Formatter_set64fPrecision_int(cv::Formatter* instance, int p, Result_void* ocvrs_return) {
		try {
			instance->set64fPrecision(p);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setMultiline(bool) /usr/include/opencv2/core.hpp:3094
	void cv_Formatter_setMultiline_bool(cv::Formatter* instance, bool ml, Result_void* ocvrs_return) {
		try {
			instance->setMultiline(ml);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// get(Formatter::FormatType) /usr/include/opencv2/core.hpp:3096
	void cv_Formatter_get_FormatType(cv::Formatter::FormatType fmt, Result<cv::Ptr<cv::Formatter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Formatter> ret = cv::Formatter::get(fmt);
			Ok(new cv::Ptr<cv::Formatter>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::Formatter>*>))
	}
	
	void cv_Hamming_delete(cv::Hamming* instance) {
		delete instance;
	}
	// KeyPoint() /usr/include/opencv2/core/types.hpp:737
	void cv_KeyPoint_KeyPoint(Result<cv::KeyPoint>* ocvrs_return) {
		try {
			cv::KeyPoint ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::KeyPoint>))
	}
	
	// KeyPoint(cv::Point2f, float, float, float, int, int) /usr/include/opencv2/core/types.hpp:746
	void cv_KeyPoint_KeyPoint_Point2f_float_float_float_int_int(cv::Point2f* pt, float size, float angle, float response, int octave, int class_id, Result<cv::KeyPoint>* ocvrs_return) {
		try {
			cv::KeyPoint ret(*pt, size, angle, response, octave, class_id);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::KeyPoint>))
	}
	
	// KeyPoint(float, float, float, float, float, int, int) /usr/include/opencv2/core/types.hpp:756
	void cv_KeyPoint_KeyPoint_float_float_float_float_float_int_int(float x, float y, float size, float angle, float response, int octave, int class_id, Result<cv::KeyPoint>* ocvrs_return) {
		try {
			cv::KeyPoint ret(x, y, size, angle, response, octave, class_id);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::KeyPoint>))
	}
	
	// hash() /usr/include/opencv2/core/types.hpp:758
	void cv_KeyPoint_hash_const(const cv::KeyPoint instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance.hash();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// convert(const std::vector<KeyPoint> &, std::vector<Point2f> &, const std::vector<int> &) /usr/include/opencv2/core/types.hpp:769
	void cv_KeyPoint_convert_const_vector_KeyPoint_R_vector_Point2f_R_const_vector_int_R(const std::vector<cv::KeyPoint>* keypoints, std::vector<cv::Point2f>* points2f, const std::vector<int>* keypointIndexes, Result_void* ocvrs_return) {
		try {
			cv::KeyPoint::convert(*keypoints, *points2f, *keypointIndexes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// convert(const std::vector<Point2f> &, std::vector<KeyPoint> &, float, float, int, int) /usr/include/opencv2/core/types.hpp:780
	void cv_KeyPoint_convert_const_vector_Point2f_R_vector_KeyPoint_R_float_float_int_int(const std::vector<cv::Point2f>* points2f, std::vector<cv::KeyPoint>* keypoints, float size, float response, int octave, int class_id, Result_void* ocvrs_return) {
		try {
			cv::KeyPoint::convert(*points2f, *keypoints, size, response, octave, class_id);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// overlap(const cv::KeyPoint &, const cv::KeyPoint &) /usr/include/opencv2/core/types.hpp:791
	void cv_KeyPoint_overlap_const_KeyPointR_const_KeyPointR(const cv::KeyPoint* kp1, const cv::KeyPoint* kp2, Result<float>* ocvrs_return) {
		try {
			float ret = cv::KeyPoint::overlap(*kp1, *kp2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	void cv_LDA_delete(cv::LDA* instance) {
		delete instance;
	}
	// LDA(int) /usr/include/opencv2/core.hpp:2611
	void cv_LDA_LDA_int(int num_components, Result<cv::LDA*>* ocvrs_return) {
		try {
			cv::LDA* ret = new cv::LDA(num_components);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::LDA*>))
	}
	
	// LDA(cv::InputArrayOfArrays, cv::InputArray, int) /usr/include/opencv2/core.hpp:2618
	void cv_LDA_LDA_const__InputArrayR_const__InputArrayR_int(const cv::_InputArray* src, const cv::_InputArray* labels, int num_components, Result<cv::LDA*>* ocvrs_return) {
		try {
			cv::LDA* ret = new cv::LDA(*src, *labels, num_components);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::LDA*>))
	}
	
	// save(const cv::String &) /usr/include/opencv2/core.hpp:2622
	void cv_LDA_save_const_const_StringR(const cv::LDA* instance, const char* filename, Result_void* ocvrs_return) {
		try {
			instance->save(std::string(filename));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// load(const cv::String &) /usr/include/opencv2/core.hpp:2626
	void cv_LDA_load_const_StringR(cv::LDA* instance, const char* filename, Result_void* ocvrs_return) {
		try {
			instance->load(std::string(filename));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// save(cv::FileStorage &) /usr/include/opencv2/core.hpp:2630
	void cv_LDA_save_const_FileStorageR(const cv::LDA* instance, cv::FileStorage* fs, Result_void* ocvrs_return) {
		try {
			instance->save(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// load(const cv::FileStorage &) /usr/include/opencv2/core.hpp:2634
	void cv_LDA_load_const_FileStorageR(cv::LDA* instance, const cv::FileStorage* node, Result_void* ocvrs_return) {
		try {
			instance->load(*node);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// compute(cv::InputArrayOfArrays, cv::InputArray) /usr/include/opencv2/core.hpp:2642
	void cv_LDA_compute_const__InputArrayR_const__InputArrayR(cv::LDA* instance, const cv::_InputArray* src, const cv::_InputArray* labels, Result_void* ocvrs_return) {
		try {
			instance->compute(*src, *labels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// project(cv::InputArray) /usr/include/opencv2/core.hpp:2647
	void cv_LDA_project_const__InputArrayR(cv::LDA* instance, const cv::_InputArray* src, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->project(*src);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// reconstruct(cv::InputArray) /usr/include/opencv2/core.hpp:2652
	void cv_LDA_reconstruct_const__InputArrayR(cv::LDA* instance, const cv::_InputArray* src, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->reconstruct(*src);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// eigenvectors() /usr/include/opencv2/core.hpp:2656
	void cv_LDA_eigenvectors_const(const cv::LDA* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->eigenvectors();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// eigenvalues() /usr/include/opencv2/core.hpp:2660
	void cv_LDA_eigenvalues_const(const cv::LDA* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->eigenvalues();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// subspaceProject(cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/core.hpp:2662
	void cv_LDA_subspaceProject_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* W, const cv::_InputArray* mean, const cv::_InputArray* src, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::LDA::subspaceProject(*W, *mean, *src);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// subspaceReconstruct(cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/core.hpp:2663
	void cv_LDA_subspaceReconstruct_const__InputArrayR_const__InputArrayR_const__InputArrayR(const cv::_InputArray* W, const cv::_InputArray* mean, const cv::_InputArray* src, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::LDA::subspaceReconstruct(*W, *mean, *src);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// flags /usr/include/opencv2/core/mat.hpp:2112
	int cv_Mat_getPropFlags_const(const cv::Mat* instance) {
			int ret = instance->flags;
			return ret;
	}
	
	// flags /usr/include/opencv2/core/mat.hpp:2112
	void cv_Mat_setPropFlags_int(cv::Mat* instance, int val) {
			instance->flags = val;
	}
	
	// dims /usr/include/opencv2/core/mat.hpp:2114
	int cv_Mat_getPropDims_const(const cv::Mat* instance) {
			int ret = instance->dims;
			return ret;
	}
	
	// dims /usr/include/opencv2/core/mat.hpp:2114
	void cv_Mat_setPropDims_int(cv::Mat* instance, int val) {
			instance->dims = val;
	}
	
	// rows /usr/include/opencv2/core/mat.hpp:2116
	int cv_Mat_getPropRows_const(const cv::Mat* instance) {
			int ret = instance->rows;
			return ret;
	}
	
	// rows /usr/include/opencv2/core/mat.hpp:2116
	void cv_Mat_setPropRows_int(cv::Mat* instance, int val) {
			instance->rows = val;
	}
	
	// cols /usr/include/opencv2/core/mat.hpp:2116
	int cv_Mat_getPropCols_const(const cv::Mat* instance) {
			int ret = instance->cols;
			return ret;
	}
	
	// cols /usr/include/opencv2/core/mat.hpp:2116
	void cv_Mat_setPropCols_int(cv::Mat* instance, int val) {
			instance->cols = val;
	}
	
	// data /usr/include/opencv2/core/mat.hpp:2118
	unsigned char* cv_Mat_getPropData(cv::Mat* instance) {
			unsigned char* ret = instance->data;
			return ret;
	}
	
	// data /usr/include/opencv2/core/mat.hpp:2118
	void cv_Mat_setPropData_unsigned_charX(cv::Mat* instance, unsigned char* val) {
			instance->data = val;
	}
	
	// datastart /usr/include/opencv2/core/mat.hpp:2121
	const unsigned char* cv_Mat_getPropDatastart_const(const cv::Mat* instance) {
			const unsigned char* ret = instance->datastart;
			return ret;
	}
	
	// dataend /usr/include/opencv2/core/mat.hpp:2122
	const unsigned char* cv_Mat_getPropDataend_const(const cv::Mat* instance) {
			const unsigned char* ret = instance->dataend;
			return ret;
	}
	
	// datalimit /usr/include/opencv2/core/mat.hpp:2123
	const unsigned char* cv_Mat_getPropDatalimit_const(const cv::Mat* instance) {
			const unsigned char* ret = instance->datalimit;
			return ret;
	}
	
	// u /usr/include/opencv2/core/mat.hpp:2136
	cv::UMatData** cv_Mat_getPropU(cv::Mat* instance) {
			cv::UMatData* ret = instance->u;
			return new cv::UMatData*(ret);
	}
	
	// u /usr/include/opencv2/core/mat.hpp:2136
	void cv_Mat_setPropU_UMatDataX(cv::Mat* instance, cv::UMatData* val) {
			instance->u = val;
	}
	
	// size /usr/include/opencv2/core/mat.hpp:2138
	cv::MatSize* cv_Mat_getPropSize_const(const cv::Mat* instance) {
			cv::MatSize ret = instance->size;
			return new cv::MatSize(ret);
	}
	
	// step /usr/include/opencv2/core/mat.hpp:2139
	cv::MatStep* cv_Mat_getPropStep_const(const cv::Mat* instance) {
			cv::MatStep ret = instance->step;
			return new cv::MatStep(ret);
	}
	
	void cv_Mat_delete(cv::Mat* instance) {
		delete instance;
	}
	// Mat() /usr/include/opencv2/core/mat.hpp:819
	cv::Mat* cv_Mat_Mat() {
			cv::Mat* ret = new cv::Mat();
			return ret;
	}
	
	// Mat(int, int, int) /usr/include/opencv2/core/mat.hpp:827
	void cv_Mat_Mat_int_int_int(int rows, int cols, int type, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(rows, cols, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// Mat(cv::Size, int) /usr/include/opencv2/core/mat.hpp:835
	void cv_Mat_Mat_Size_int(cv::Size* size, int type, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*size, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// Mat(int, int, int, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:846
	void cv_Mat_Mat_int_int_int_const_ScalarR(int rows, int cols, int type, const cv::Scalar* s, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(rows, cols, type, *s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// Mat(cv::Size, int, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:857
	void cv_Mat_Mat_Size_int_const_ScalarR(cv::Size* size, int type, const cv::Scalar* s, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*size, type, *s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// Mat(int, const int *, int) /usr/include/opencv2/core/mat.hpp:865
	void cv_Mat_Mat_int_const_intX_int(int ndims, const int* sizes, int type, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(ndims, sizes, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// Mat(const std::vector<int> &, int) /usr/include/opencv2/core/mat.hpp:872
	void cv_Mat_Mat_const_vector_int_R_int(const std::vector<int>* sizes, int type, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*sizes, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// Mat(int, const int *, int, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:883
	void cv_Mat_Mat_int_const_intX_int_const_ScalarR(int ndims, const int* sizes, int type, const cv::Scalar* s, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(ndims, sizes, type, *s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// Mat(const std::vector<int> &, int, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:893
	void cv_Mat_Mat_const_vector_int_R_int_const_ScalarR(const std::vector<int>* sizes, int type, const cv::Scalar* s, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*sizes, type, *s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// Mat(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:903
	void cv_Mat_Mat_const_MatR(const cv::Mat* m, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// Mat(int, int, int, void *, size_t) /usr/include/opencv2/core/mat.hpp:919
	void cv_Mat_Mat_int_int_int_voidX_size_t(int rows, int cols, int type, void* data, size_t step, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(rows, cols, type, data, step);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// Mat(cv::Size, int, void *, size_t) /usr/include/opencv2/core/mat.hpp:935
	void cv_Mat_Mat_Size_int_voidX_size_t(cv::Size* size, int type, void* data, size_t step, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*size, type, data, step);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// Mat(int, const int *, int, void *, const size_t *) /usr/include/opencv2/core/mat.hpp:950
	void cv_Mat_Mat_int_const_intX_int_voidX_const_size_tX(int ndims, const int* sizes, int type, void* data, const size_t* steps, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(ndims, sizes, type, data, steps);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// Mat(const std::vector<int> &, int, void *, const size_t *) /usr/include/opencv2/core/mat.hpp:964
	void cv_Mat_Mat_const_vector_int_R_int_voidX_const_size_tX(const std::vector<int>* sizes, int type, void* data, const size_t* steps, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*sizes, type, data, steps);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// Mat(const cv::Mat &, const cv::Range &, const cv::Range &) /usr/include/opencv2/core/mat.hpp:976
	void cv_Mat_Mat_const_MatR_const_RangeR_const_RangeR(const cv::Mat* m, const cv::Range* rowRange, const cv::Range* colRange, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*m, *rowRange, *colRange);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// Mat(const cv::Mat &, const cv::Rect &) /usr/include/opencv2/core/mat.hpp:986
	void cv_Mat_Mat_const_MatR_const_RectR(const cv::Mat* m, const cv::Rect* roi, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*m, *roi);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// Mat(const cv::Mat &, const std::vector<Range> &) /usr/include/opencv2/core/mat.hpp:1006
	void cv_Mat_Mat_const_MatR_const_vector_Range_R(const cv::Mat* m, const std::vector<cv::Range>* ranges, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*m, *ranges);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// Mat(const cuda::GpuMat &) /usr/include/opencv2/core/mat.hpp:1060
	void cv_Mat_Mat_const_GpuMatR(const cv::cuda::GpuMat* m, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getUMat(cv::AccessFlag, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:1085
	void cv_Mat_getUMat_const_AccessFlag_UMatUsageFlags(const cv::Mat* instance, cv::AccessFlag accessFlags, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->getUMat(accessFlags, usageFlags);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// row(int) /usr/include/opencv2/core/mat.hpp:1120
	void cv_Mat_row_const_int(const cv::Mat* instance, int y, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->row(y);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// col(int) /usr/include/opencv2/core/mat.hpp:1129
	void cv_Mat_col_const_int(const cv::Mat* instance, int x, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->col(x);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// rowRange(int, int) /usr/include/opencv2/core/mat.hpp:1138
	void cv_Mat_rowRange_const_int_int(const cv::Mat* instance, int startrow, int endrow, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->rowRange(startrow, endrow);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// rowRange(const cv::Range &) /usr/include/opencv2/core/mat.hpp:1143
	void cv_Mat_rowRange_const_const_RangeR(const cv::Mat* instance, const cv::Range* r, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->rowRange(*r);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// colRange(int, int) /usr/include/opencv2/core/mat.hpp:1152
	void cv_Mat_colRange_const_int_int(const cv::Mat* instance, int startcol, int endcol, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->colRange(startcol, endcol);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// colRange(const cv::Range &) /usr/include/opencv2/core/mat.hpp:1157
	void cv_Mat_colRange_const_const_RangeR(const cv::Mat* instance, const cv::Range* r, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->colRange(*r);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// diag(int) /usr/include/opencv2/core/mat.hpp:1193
	void cv_Mat_diag_const_int(const cv::Mat* instance, int d, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->diag(d);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// diag(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:1200
	void cv_Mat_diag_const_MatR(const cv::Mat* d, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::Mat::diag(*d);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// clone() /usr/include/opencv2/core/mat.hpp:1207
	void cv_Mat_clone_const(const cv::Mat* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->clone();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// copyTo(cv::OutputArray) /usr/include/opencv2/core/mat.hpp:1224
	void cv_Mat_copyTo_const_const__OutputArrayR(const cv::Mat* instance, const cv::_OutputArray* m, Result_void* ocvrs_return) {
		try {
			instance->copyTo(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// copyTo(cv::OutputArray, cv::InputArray) /usr/include/opencv2/core/mat.hpp:1232
	void cv_Mat_copyTo_const_const__OutputArrayR_const__InputArrayR(const cv::Mat* instance, const cv::_OutputArray* m, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			instance->copyTo(*m, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// convertTo(cv::OutputArray, int, double, double) /usr/include/opencv2/core/mat.hpp:1247
	void cv_Mat_convertTo_const_const__OutputArrayR_int_double_double(const cv::Mat* instance, const cv::_OutputArray* m, int rtype, double alpha, double beta, Result_void* ocvrs_return) {
		try {
			instance->convertTo(*m, rtype, alpha, beta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// assignTo(cv::Mat &, int) /usr/include/opencv2/core/mat.hpp:1255
	void cv_Mat_assignTo_const_MatR_int(const cv::Mat* instance, cv::Mat* m, int type, Result_void* ocvrs_return) {
		try {
			instance->assignTo(*m, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setTo(cv::InputArray, cv::InputArray) /usr/include/opencv2/core/mat.hpp:1269
	void cv_Mat_setTo_const__InputArrayR_const__InputArrayR(cv::Mat* instance, const cv::_InputArray* value, const cv::_InputArray* mask, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->setTo(*value, *mask);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// reshape(int, int) /usr/include/opencv2/core/mat.hpp:1295
	void cv_Mat_reshape_const_int_int(const cv::Mat* instance, int cn, int rows, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->reshape(cn, rows);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// reshape(int, int, const int *) /usr/include/opencv2/core/mat.hpp:1298
	void cv_Mat_reshape_const_int_int_const_intX(const cv::Mat* instance, int cn, int newndims, const int* newsz, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->reshape(cn, newndims, newsz);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// reshape(int, const std::vector<int> &) /usr/include/opencv2/core/mat.hpp:1301
	void cv_Mat_reshape_const_int_const_vector_int_R(const cv::Mat* instance, int cn, const std::vector<int>* newshape, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->reshape(cn, *newshape);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// t() /usr/include/opencv2/core/mat.hpp:1313
	void cv_Mat_t_const(const cv::Mat* instance, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->t();
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// inv(int) /usr/include/opencv2/core/mat.hpp:1322
	void cv_Mat_inv_const_int(const cv::Mat* instance, int method, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->inv(method);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// mul(cv::InputArray, double) /usr/include/opencv2/core/mat.hpp:1336
	void cv_Mat_mul_const_const__InputArrayR_double(const cv::Mat* instance, const cv::_InputArray* m, double scale, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->mul(*m, scale);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// cross(cv::InputArray) /usr/include/opencv2/core/mat.hpp:1345
	void cv_Mat_cross_const_const__InputArrayR(const cv::Mat* instance, const cv::_InputArray* m, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->cross(*m);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// dot(cv::InputArray) /usr/include/opencv2/core/mat.hpp:1355
	void cv_Mat_dot_const_const__InputArrayR(const cv::Mat* instance, const cv::_InputArray* m, Result<double>* ocvrs_return) {
		try {
			double ret = instance->dot(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// zeros(int, int, int) /usr/include/opencv2/core/mat.hpp:1371
	void cv_Mat_zeros_int_int_int(int rows, int cols, int type, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::Mat::zeros(rows, cols, type);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// zeros(cv::Size, int) /usr/include/opencv2/core/mat.hpp:1377
	void cv_Mat_zeros_Size_int(cv::Size* size, int type, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::Mat::zeros(*size, type);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// zeros(int, const int *, int) /usr/include/opencv2/core/mat.hpp:1384
	void cv_Mat_zeros_int_const_intX_int(int ndims, const int* sz, int type, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::Mat::zeros(ndims, sz, type);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// ones(int, int, int) /usr/include/opencv2/core/mat.hpp:1402
	void cv_Mat_ones_int_int_int(int rows, int cols, int type, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::Mat::ones(rows, cols, type);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// ones(cv::Size, int) /usr/include/opencv2/core/mat.hpp:1408
	void cv_Mat_ones_Size_int(cv::Size* size, int type, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::Mat::ones(*size, type);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// ones(int, const int *, int) /usr/include/opencv2/core/mat.hpp:1415
	void cv_Mat_ones_int_const_intX_int(int ndims, const int* sz, int type, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::Mat::ones(ndims, sz, type);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// eye(int, int, int) /usr/include/opencv2/core/mat.hpp:1431
	void cv_Mat_eye_int_int_int(int rows, int cols, int type, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::Mat::eye(rows, cols, type);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// eye(cv::Size, int) /usr/include/opencv2/core/mat.hpp:1437
	void cv_Mat_eye_Size_int(cv::Size* size, int type, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = cv::Mat::eye(*size, type);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// create(int, int, int) /usr/include/opencv2/core/mat.hpp:1472
	void cv_Mat_create_int_int_int(cv::Mat* instance, int rows, int cols, int type, Result_void* ocvrs_return) {
		try {
			instance->create(rows, cols, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(cv::Size, int) /usr/include/opencv2/core/mat.hpp:1478
	void cv_Mat_create_Size_int(cv::Mat* instance, cv::Size* size, int type, Result_void* ocvrs_return) {
		try {
			instance->create(*size, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(int, const int *, int) /usr/include/opencv2/core/mat.hpp:1485
	void cv_Mat_create_int_const_intX_int(cv::Mat* instance, int ndims, const int* sizes, int type, Result_void* ocvrs_return) {
		try {
			instance->create(ndims, sizes, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(const std::vector<int> &, int) /usr/include/opencv2/core/mat.hpp:1491
	void cv_Mat_create_const_vector_int_R_int(cv::Mat* instance, const std::vector<int>* sizes, int type, Result_void* ocvrs_return) {
		try {
			instance->create(*sizes, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// addref() /usr/include/opencv2/core/mat.hpp:1502
	void cv_Mat_addref(cv::Mat* instance, Result_void* ocvrs_return) {
		try {
			instance->addref();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// release() /usr/include/opencv2/core/mat.hpp:1517
	void cv_Mat_release(cv::Mat* instance, Result_void* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// deallocate() /usr/include/opencv2/core/mat.hpp:1520
	void cv_Mat_deallocate(cv::Mat* instance, Result_void* ocvrs_return) {
		try {
			instance->deallocate();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// reserve(size_t) /usr/include/opencv2/core/mat.hpp:1531
	void cv_Mat_reserve_size_t(cv::Mat* instance, size_t sz, Result_void* ocvrs_return) {
		try {
			instance->reserve(sz);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// reserveBuffer(size_t) /usr/include/opencv2/core/mat.hpp:1539
	void cv_Mat_reserveBuffer_size_t(cv::Mat* instance, size_t sz, Result_void* ocvrs_return) {
		try {
			instance->reserveBuffer(sz);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// resize(size_t) /usr/include/opencv2/core/mat.hpp:1548
	void cv_Mat_resize_size_t(cv::Mat* instance, size_t sz, Result_void* ocvrs_return) {
		try {
			instance->resize(sz);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// resize(size_t, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:1554
	void cv_Mat_resize_size_t_const_ScalarR(cv::Mat* instance, size_t sz, const cv::Scalar* s, Result_void* ocvrs_return) {
		try {
			instance->resize(sz, *s);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// push_back(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:1581
	void cv_Mat_push_back_const_MatR(cv::Mat* instance, const cv::Mat* m, Result_void* ocvrs_return) {
		try {
			instance->push_back(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// pop_back(size_t) /usr/include/opencv2/core/mat.hpp:1589
	void cv_Mat_pop_back_size_t(cv::Mat* instance, size_t nelems, Result_void* ocvrs_return) {
		try {
			instance->pop_back(nelems);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// locateROI(cv::Size &, cv::Point &) /usr/include/opencv2/core/mat.hpp:1602
	void cv_Mat_locateROI_const_SizeR_PointR(const cv::Mat* instance, cv::Size* wholeSize, cv::Point* ofs, Result_void* ocvrs_return) {
		try {
			instance->locateROI(*wholeSize, *ofs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// adjustROI(int, int, int, int) /usr/include/opencv2/core/mat.hpp:1631
	void cv_Mat_adjustROI_int_int_int_int(cv::Mat* instance, int dtop, int dbottom, int dleft, int dright, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->adjustROI(dtop, dbottom, dleft, dright);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// isContinuous() /usr/include/opencv2/core/mat.hpp:1741
	bool cv_Mat_isContinuous_const(const cv::Mat* instance) {
			bool ret = instance->isContinuous();
			return ret;
	}
	
	// isSubmatrix() /usr/include/opencv2/core/mat.hpp:1744
	bool cv_Mat_isSubmatrix_const(const cv::Mat* instance) {
			bool ret = instance->isSubmatrix();
			return ret;
	}
	
	// elemSize() /usr/include/opencv2/core/mat.hpp:1751
	void cv_Mat_elemSize_const(const cv::Mat* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->elemSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// elemSize1() /usr/include/opencv2/core/mat.hpp:1758
	size_t cv_Mat_elemSize1_const(const cv::Mat* instance) {
			size_t ret = instance->elemSize1();
			return ret;
	}
	
	// type() /usr/include/opencv2/core/mat.hpp:1765
	int cv_Mat_type_const(const cv::Mat* instance) {
			int ret = instance->type();
			return ret;
	}
	
	// depth() /usr/include/opencv2/core/mat.hpp:1780
	int cv_Mat_depth_const(const cv::Mat* instance) {
			int ret = instance->depth();
			return ret;
	}
	
	// channels() /usr/include/opencv2/core/mat.hpp:1786
	int cv_Mat_channels_const(const cv::Mat* instance) {
			int ret = instance->channels();
			return ret;
	}
	
	// step1(int) /usr/include/opencv2/core/mat.hpp:1793
	void cv_Mat_step1_const_int(const cv::Mat* instance, int i, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->step1(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// empty() /usr/include/opencv2/core/mat.hpp:1800
	bool cv_Mat_empty_const(const cv::Mat* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	// total() /usr/include/opencv2/core/mat.hpp:1807
	size_t cv_Mat_total_const(const cv::Mat* instance) {
			size_t ret = instance->total();
			return ret;
	}
	
	// total(int, int) /usr/include/opencv2/core/mat.hpp:1813
	void cv_Mat_total_const_int_int(const cv::Mat* instance, int startDim, int endDim, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->total(startDim, endDim);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// checkVector(int, int, bool) /usr/include/opencv2/core/mat.hpp:1836
	void cv_Mat_checkVector_const_int_int_bool(const cv::Mat* instance, int elemChannels, int depth, bool requireContinuous, Result<int>* ocvrs_return) {
		try {
			int ret = instance->checkVector(elemChannels, depth, requireContinuous);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// ptr(int) /usr/include/opencv2/core/mat.hpp:1844
	void cv_Mat_ptr_int(cv::Mat* instance, int i0, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(i0);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	// ptr(int) /usr/include/opencv2/core/mat.hpp:1846
	void cv_Mat_ptr_const_int(const cv::Mat* instance, int i0, Result<const unsigned char*>* ocvrs_return) {
		try {
			const unsigned char* ret = instance->ptr(i0);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	// ptr(int, int) /usr/include/opencv2/core/mat.hpp:1852
	void cv_Mat_ptr_int_int(cv::Mat* instance, int row, int col, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(row, col);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	// ptr(int, int) /usr/include/opencv2/core/mat.hpp:1857
	void cv_Mat_ptr_const_int_int(const cv::Mat* instance, int row, int col, Result<const unsigned char*>* ocvrs_return) {
		try {
			const unsigned char* ret = instance->ptr(row, col);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	// ptr(int, int, int) /usr/include/opencv2/core/mat.hpp:1860
	void cv_Mat_ptr_int_int_int(cv::Mat* instance, int i0, int i1, int i2, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(i0, i1, i2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	// ptr(int, int, int) /usr/include/opencv2/core/mat.hpp:1862
	void cv_Mat_ptr_const_int_int_int(const cv::Mat* instance, int i0, int i1, int i2, Result<const unsigned char*>* ocvrs_return) {
		try {
			const unsigned char* ret = instance->ptr(i0, i1, i2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	// ptr(const int *) /usr/include/opencv2/core/mat.hpp:1865
	void cv_Mat_ptr_const_intX(cv::Mat* instance, const int* idx, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	// ptr(const int *) /usr/include/opencv2/core/mat.hpp:1867
	void cv_Mat_ptr_const_const_intX(const cv::Mat* instance, const int* idx, Result<const unsigned char*>* ocvrs_return) {
		try {
			const unsigned char* ret = instance->ptr(idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	// Mat(cv::Mat &&) /usr/include/opencv2/core/mat.hpp:2100
	void cv_Mat_Mat_MatR(cv::Mat* m, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat* ret = new cv::Mat(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// updateContinuityFlag() /usr/include/opencv2/core/mat.hpp:2133
	void cv_Mat_updateContinuityFlag(cv::Mat* instance, Result_void* ocvrs_return) {
		try {
			instance->updateContinuityFlag();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// m /usr/include/opencv2/core/mat.hpp:3112
	const cv::Mat** cv_MatConstIterator_getPropM_const(const cv::MatConstIterator* instance) {
			const cv::Mat* ret = instance->m;
			return new const cv::Mat*(ret);
	}
	
	// elemSize /usr/include/opencv2/core/mat.hpp:3113
	size_t cv_MatConstIterator_getPropElemSize_const(const cv::MatConstIterator* instance) {
			size_t ret = instance->elemSize;
			return ret;
	}
	
	// elemSize /usr/include/opencv2/core/mat.hpp:3113
	void cv_MatConstIterator_setPropElemSize_size_t(cv::MatConstIterator* instance, size_t val) {
			instance->elemSize = val;
	}
	
	// ptr /usr/include/opencv2/core/mat.hpp:3114
	const unsigned char* cv_MatConstIterator_getPropPtr_const(const cv::MatConstIterator* instance) {
			const unsigned char* ret = instance->ptr;
			return ret;
	}
	
	// sliceStart /usr/include/opencv2/core/mat.hpp:3115
	const unsigned char* cv_MatConstIterator_getPropSliceStart_const(const cv::MatConstIterator* instance) {
			const unsigned char* ret = instance->sliceStart;
			return ret;
	}
	
	// sliceEnd /usr/include/opencv2/core/mat.hpp:3116
	const unsigned char* cv_MatConstIterator_getPropSliceEnd_const(const cv::MatConstIterator* instance) {
			const unsigned char* ret = instance->sliceEnd;
			return ret;
	}
	
	void cv_MatConstIterator_delete(cv::MatConstIterator* instance) {
		delete instance;
	}
	// MatConstIterator() /usr/include/opencv2/core/mat.hpp:3072
	void cv_MatConstIterator_MatConstIterator(Result<cv::MatConstIterator*>* ocvrs_return) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatConstIterator*>))
	}
	
	// MatConstIterator(const cv::Mat *) /usr/include/opencv2/core/mat.hpp:3074
	void cv_MatConstIterator_MatConstIterator_const_MatX(const cv::Mat* _m, Result<cv::MatConstIterator*>* ocvrs_return) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator(_m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatConstIterator*>))
	}
	
	// MatConstIterator(const cv::Mat *, int, int) /usr/include/opencv2/core/mat.hpp:3076
	void cv_MatConstIterator_MatConstIterator_const_MatX_int_int(const cv::Mat* _m, int _row, int _col, Result<cv::MatConstIterator*>* ocvrs_return) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator(_m, _row, _col);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatConstIterator*>))
	}
	
	// MatConstIterator(const cv::Mat *, cv::Point) /usr/include/opencv2/core/mat.hpp:3078
	void cv_MatConstIterator_MatConstIterator_const_MatX_Point(const cv::Mat* _m, cv::Point* _pt, Result<cv::MatConstIterator*>* ocvrs_return) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator(_m, *_pt);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatConstIterator*>))
	}
	
	// MatConstIterator(const cv::MatConstIterator &) /usr/include/opencv2/core/mat.hpp:3082
	void cv_MatConstIterator_MatConstIterator_const_MatConstIteratorR(const cv::MatConstIterator* it, Result<cv::MatConstIterator*>* ocvrs_return) {
		try {
			cv::MatConstIterator* ret = new cv::MatConstIterator(*it);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatConstIterator*>))
	}
	
	// operator*() /usr/include/opencv2/core/mat.hpp:3087
	void cv_MatConstIterator_operatorX_const(const cv::MatConstIterator* instance, Result<const unsigned char*>* ocvrs_return) {
		try {
			const unsigned char* ret = instance->operator*();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	// operator[](ptrdiff_t) /usr/include/opencv2/core/mat.hpp:3089
	void cv_MatConstIterator_operator___const_ptrdiff_t(const cv::MatConstIterator* instance, ptrdiff_t i, Result<const unsigned char*>* ocvrs_return) {
		try {
			const unsigned char* ret = instance->operator[](i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	// operator--() /usr/include/opencv2/core/mat.hpp:3096
	void cv_MatConstIterator_operatorSS(cv::MatConstIterator* instance, Result<cv::MatConstIterator*>* ocvrs_return) {
		try {
			cv::MatConstIterator ret = instance->operator--();
			Ok(new cv::MatConstIterator(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatConstIterator*>))
	}
	
	// operator++() /usr/include/opencv2/core/mat.hpp:3100
	void cv_MatConstIterator_operatorAA(cv::MatConstIterator* instance, Result<cv::MatConstIterator*>* ocvrs_return) {
		try {
			cv::MatConstIterator ret = instance->operator++();
			Ok(new cv::MatConstIterator(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatConstIterator*>))
	}
	
	// pos() /usr/include/opencv2/core/mat.hpp:3104
	void cv_MatConstIterator_pos_const(const cv::MatConstIterator* instance, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->pos();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	// pos(int *) /usr/include/opencv2/core/mat.hpp:3106
	void cv_MatConstIterator_pos_const_intX(const cv::MatConstIterator* instance, int* _idx, Result_void* ocvrs_return) {
		try {
			instance->pos(_idx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// lpos() /usr/include/opencv2/core/mat.hpp:3108
	void cv_MatConstIterator_lpos_const(const cv::MatConstIterator* instance, Result<ptrdiff_t>* ocvrs_return) {
		try {
			ptrdiff_t ret = instance->lpos();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<ptrdiff_t>))
	}
	
	// seek(ptrdiff_t, bool) /usr/include/opencv2/core/mat.hpp:3109
	void cv_MatConstIterator_seek_ptrdiff_t_bool(cv::MatConstIterator* instance, ptrdiff_t ofs, bool relative, Result_void* ocvrs_return) {
		try {
			instance->seek(ofs, relative);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// seek(const int *, bool) /usr/include/opencv2/core/mat.hpp:3110
	void cv_MatConstIterator_seek_const_intX_bool(cv::MatConstIterator* instance, const int* _idx, bool relative, Result_void* ocvrs_return) {
		try {
			instance->seek(_idx, relative);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// flags /usr/include/opencv2/core/mat.hpp:3595
	int cv_MatExpr_getPropFlags_const(const cv::MatExpr* instance) {
			int ret = instance->flags;
			return ret;
	}
	
	// flags /usr/include/opencv2/core/mat.hpp:3595
	void cv_MatExpr_setPropFlags_int(cv::MatExpr* instance, int val) {
			instance->flags = val;
	}
	
	// a /usr/include/opencv2/core/mat.hpp:3597
	cv::Mat* cv_MatExpr_getPropA_const(const cv::MatExpr* instance) {
			cv::Mat ret = instance->a;
			return new cv::Mat(ret);
	}
	
	// a /usr/include/opencv2/core/mat.hpp:3597
	void cv_MatExpr_setPropA_Mat(cv::MatExpr* instance, cv::Mat* val) {
			instance->a = *val;
	}
	
	// b /usr/include/opencv2/core/mat.hpp:3597
	cv::Mat* cv_MatExpr_getPropB_const(const cv::MatExpr* instance) {
			cv::Mat ret = instance->b;
			return new cv::Mat(ret);
	}
	
	// b /usr/include/opencv2/core/mat.hpp:3597
	void cv_MatExpr_setPropB_Mat(cv::MatExpr* instance, cv::Mat* val) {
			instance->b = *val;
	}
	
	// c /usr/include/opencv2/core/mat.hpp:3597
	cv::Mat* cv_MatExpr_getPropC_const(const cv::MatExpr* instance) {
			cv::Mat ret = instance->c;
			return new cv::Mat(ret);
	}
	
	// c /usr/include/opencv2/core/mat.hpp:3597
	void cv_MatExpr_setPropC_Mat(cv::MatExpr* instance, cv::Mat* val) {
			instance->c = *val;
	}
	
	// alpha /usr/include/opencv2/core/mat.hpp:3598
	double cv_MatExpr_getPropAlpha_const(const cv::MatExpr* instance) {
			double ret = instance->alpha;
			return ret;
	}
	
	// alpha /usr/include/opencv2/core/mat.hpp:3598
	void cv_MatExpr_setPropAlpha_double(cv::MatExpr* instance, double val) {
			instance->alpha = val;
	}
	
	// beta /usr/include/opencv2/core/mat.hpp:3598
	double cv_MatExpr_getPropBeta_const(const cv::MatExpr* instance) {
			double ret = instance->beta;
			return ret;
	}
	
	// beta /usr/include/opencv2/core/mat.hpp:3598
	void cv_MatExpr_setPropBeta_double(cv::MatExpr* instance, double val) {
			instance->beta = val;
	}
	
	// s /usr/include/opencv2/core/mat.hpp:3599
	void cv_MatExpr_getPropS_const(const cv::MatExpr* instance, cv::Scalar* ocvrs_return) {
			cv::Scalar ret = instance->s;
			*ocvrs_return = ret;
	}
	
	// s /usr/include/opencv2/core/mat.hpp:3599
	void cv_MatExpr_setPropS_Scalar(cv::MatExpr* instance, cv::Scalar* val) {
			instance->s = *val;
	}
	
	void cv_MatExpr_delete(cv::MatExpr* instance) {
		delete instance;
	}
	// MatExpr() /usr/include/opencv2/core/mat.hpp:3566
	void cv_MatExpr_MatExpr(Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr* ret = new cv::MatExpr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// MatExpr(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3567
	void cv_MatExpr_MatExpr_const_MatR(const cv::Mat* m, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr* ret = new cv::MatExpr(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// MatExpr(const cv::MatOp *, int, const cv::Mat &, const cv::Mat &, const cv::Mat &, double, double, const cv::Scalar &) /usr/include/opencv2/core/mat.hpp:3569
	void cv_MatExpr_MatExpr_const_MatOpX_int_const_MatR_const_MatR_const_MatR_double_double_const_ScalarR(const cv::MatOp* _op, int _flags, const cv::Mat* _a, const cv::Mat* _b, const cv::Mat* _c, double _alpha, double _beta, const cv::Scalar* _s, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr* ret = new cv::MatExpr(_op, _flags, *_a, *_b, *_c, _alpha, _beta, *_s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// operator Mat() /usr/include/opencv2/core/mat.hpp:3572
	void cv_MatExpr_operator_cv_Mat_const(const cv::MatExpr* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->operator cv::Mat();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// size() /usr/include/opencv2/core/mat.hpp:3575
	void cv_MatExpr_size_const(const cv::MatExpr* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// type() /usr/include/opencv2/core/mat.hpp:3576
	void cv_MatExpr_type_const(const cv::MatExpr* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// row(int) /usr/include/opencv2/core/mat.hpp:3578
	void cv_MatExpr_row_const_int(const cv::MatExpr* instance, int y, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->row(y);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// col(int) /usr/include/opencv2/core/mat.hpp:3579
	void cv_MatExpr_col_const_int(const cv::MatExpr* instance, int x, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->col(x);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// diag(int) /usr/include/opencv2/core/mat.hpp:3580
	void cv_MatExpr_diag_const_int(const cv::MatExpr* instance, int d, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->diag(d);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// t() /usr/include/opencv2/core/mat.hpp:3584
	void cv_MatExpr_t_const(const cv::MatExpr* instance, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->t();
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// inv(int) /usr/include/opencv2/core/mat.hpp:3585
	void cv_MatExpr_inv_const_int(const cv::MatExpr* instance, int method, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->inv(method);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// mul(const cv::MatExpr &, double) /usr/include/opencv2/core/mat.hpp:3586
	void cv_MatExpr_mul_const_const_MatExprR_double(const cv::MatExpr* instance, const cv::MatExpr* e, double scale, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->mul(*e, scale);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// mul(const cv::Mat &, double) /usr/include/opencv2/core/mat.hpp:3587
	void cv_MatExpr_mul_const_const_MatR_double(const cv::MatExpr* instance, const cv::Mat* m, double scale, Result<cv::MatExpr*>* ocvrs_return) {
		try {
			cv::MatExpr ret = instance->mul(*m, scale);
			Ok(new cv::MatExpr(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MatExpr*>))
	}
	
	// cross(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3589
	void cv_MatExpr_cross_const_const_MatR(const cv::MatExpr* instance, const cv::Mat* m, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->cross(*m);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// dot(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:3590
	void cv_MatExpr_dot_const_const_MatR(const cv::MatExpr* instance, const cv::Mat* m, Result<double>* ocvrs_return) {
		try {
			double ret = instance->dot(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// swap(cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3592
	void cv_MatExpr_swap_MatExprR(cv::MatExpr* instance, cv::MatExpr* b, Result_void* ocvrs_return) {
		try {
			instance->swap(*b);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// elementWise(const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3482
	void cv_MatOp_elementWise_const_const_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->elementWise(*expr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// assign(const cv::MatExpr &, cv::Mat &, int) /usr/include/opencv2/core/mat.hpp:3483
	void cv_MatOp_assign_const_const_MatExprR_MatR_int(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m, int type, Result_void* ocvrs_return) {
		try {
			instance->assign(*expr, *m, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// roi(const cv::MatExpr &, const cv::Range &, const cv::Range &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3484
	void cv_MatOp_roi_const_const_MatExprR_const_RangeR_const_RangeR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, const cv::Range* rowRange, const cv::Range* colRange, cv::MatExpr* res, Result_void* ocvrs_return) {
		try {
			instance->roi(*expr, *rowRange, *colRange, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// diag(const cv::MatExpr &, int, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3486
	void cv_MatOp_diag_const_const_MatExprR_int_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, int d, cv::MatExpr* res, Result_void* ocvrs_return) {
		try {
			instance->diag(*expr, d, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// augAssignAdd(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3487
	void cv_MatOp_augAssignAdd_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m, Result_void* ocvrs_return) {
		try {
			instance->augAssignAdd(*expr, *m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// augAssignSubtract(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3488
	void cv_MatOp_augAssignSubtract_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m, Result_void* ocvrs_return) {
		try {
			instance->augAssignSubtract(*expr, *m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// augAssignMultiply(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3489
	void cv_MatOp_augAssignMultiply_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m, Result_void* ocvrs_return) {
		try {
			instance->augAssignMultiply(*expr, *m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// augAssignDivide(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3490
	void cv_MatOp_augAssignDivide_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m, Result_void* ocvrs_return) {
		try {
			instance->augAssignDivide(*expr, *m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// augAssignAnd(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3491
	void cv_MatOp_augAssignAnd_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m, Result_void* ocvrs_return) {
		try {
			instance->augAssignAnd(*expr, *m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// augAssignOr(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3492
	void cv_MatOp_augAssignOr_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m, Result_void* ocvrs_return) {
		try {
			instance->augAssignOr(*expr, *m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// augAssignXor(const cv::MatExpr &, cv::Mat &) /usr/include/opencv2/core/mat.hpp:3493
	void cv_MatOp_augAssignXor_const_const_MatExprR_MatR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::Mat* m, Result_void* ocvrs_return) {
		try {
			instance->augAssignXor(*expr, *m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// add(const cv::MatExpr &, const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3495
	void cv_MatOp_add_const_const_MatExprR_const_MatExprR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr1, const cv::MatExpr* expr2, cv::MatExpr* res, Result_void* ocvrs_return) {
		try {
			instance->add(*expr1, *expr2, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// add(const cv::MatExpr &, const cv::Scalar &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3496
	void cv_MatOp_add_const_const_MatExprR_const_ScalarR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr1, const cv::Scalar* s, cv::MatExpr* res, Result_void* ocvrs_return) {
		try {
			instance->add(*expr1, *s, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// subtract(const cv::MatExpr &, const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3498
	void cv_MatOp_subtract_const_const_MatExprR_const_MatExprR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr1, const cv::MatExpr* expr2, cv::MatExpr* res, Result_void* ocvrs_return) {
		try {
			instance->subtract(*expr1, *expr2, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// subtract(const cv::Scalar &, const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3499
	void cv_MatOp_subtract_const_const_ScalarR_const_MatExprR_MatExprR(const cv::MatOp* instance, const cv::Scalar* s, const cv::MatExpr* expr, cv::MatExpr* res, Result_void* ocvrs_return) {
		try {
			instance->subtract(*s, *expr, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// multiply(const cv::MatExpr &, const cv::MatExpr &, cv::MatExpr &, double) /usr/include/opencv2/core/mat.hpp:3501
	void cv_MatOp_multiply_const_const_MatExprR_const_MatExprR_MatExprR_double(const cv::MatOp* instance, const cv::MatExpr* expr1, const cv::MatExpr* expr2, cv::MatExpr* res, double scale, Result_void* ocvrs_return) {
		try {
			instance->multiply(*expr1, *expr2, *res, scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// multiply(const cv::MatExpr &, double, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3502
	void cv_MatOp_multiply_const_const_MatExprR_double_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr1, double s, cv::MatExpr* res, Result_void* ocvrs_return) {
		try {
			instance->multiply(*expr1, s, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// divide(const cv::MatExpr &, const cv::MatExpr &, cv::MatExpr &, double) /usr/include/opencv2/core/mat.hpp:3504
	void cv_MatOp_divide_const_const_MatExprR_const_MatExprR_MatExprR_double(const cv::MatOp* instance, const cv::MatExpr* expr1, const cv::MatExpr* expr2, cv::MatExpr* res, double scale, Result_void* ocvrs_return) {
		try {
			instance->divide(*expr1, *expr2, *res, scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// divide(double, const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3505
	void cv_MatOp_divide_const_double_const_MatExprR_MatExprR(const cv::MatOp* instance, double s, const cv::MatExpr* expr, cv::MatExpr* res, Result_void* ocvrs_return) {
		try {
			instance->divide(s, *expr, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// abs(const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3507
	void cv_MatOp_abs_const_const_MatExprR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::MatExpr* res, Result_void* ocvrs_return) {
		try {
			instance->abs(*expr, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// transpose(const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3509
	void cv_MatOp_transpose_const_const_MatExprR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, cv::MatExpr* res, Result_void* ocvrs_return) {
		try {
			instance->transpose(*expr, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// matmul(const cv::MatExpr &, const cv::MatExpr &, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3510
	void cv_MatOp_matmul_const_const_MatExprR_const_MatExprR_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr1, const cv::MatExpr* expr2, cv::MatExpr* res, Result_void* ocvrs_return) {
		try {
			instance->matmul(*expr1, *expr2, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// invert(const cv::MatExpr &, int, cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3511
	void cv_MatOp_invert_const_const_MatExprR_int_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, int method, cv::MatExpr* res, Result_void* ocvrs_return) {
		try {
			instance->invert(*expr, method, *res);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// size(const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3513
	void cv_MatOp_size_const_const_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->size(*expr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// type(const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:3514
	void cv_MatOp_type_const_const_MatExprR(const cv::MatOp* instance, const cv::MatExpr* expr, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type(*expr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// p /usr/include/opencv2/core/mat.hpp:597
	int* cv_MatSize_getPropP(cv::MatSize* instance) {
			int* ret = instance->p;
			return ret;
	}
	
	// p /usr/include/opencv2/core/mat.hpp:597
	void cv_MatSize_setPropP_intX(cv::MatSize* instance, int* val) {
			instance->p = val;
	}
	
	void cv_MatSize_delete(cv::MatSize* instance) {
		delete instance;
	}
	// MatSize(int *) /usr/include/opencv2/core/mat.hpp:588
	cv::MatSize* cv_MatSize_MatSize_intX(int* _p) {
			cv::MatSize* ret = new cv::MatSize(_p);
			return ret;
	}
	
	// dims() /usr/include/opencv2/core/mat.hpp:589
	int cv_MatSize_dims_const(const cv::MatSize* instance) {
			int ret = instance->dims();
			return ret;
	}
	
	// operator[](int) /usr/include/opencv2/core/mat.hpp:591
	void cv_MatSize_operator___const_int(const cv::MatSize* instance, int i, Result<int>* ocvrs_return) {
		try {
			const int ret = instance->operator[](i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// operator[](int) /usr/include/opencv2/core/mat.hpp:592
	void cv_MatSize_operator___int(cv::MatSize* instance, int i, Result<int>* ocvrs_return) {
		try {
			int ret = instance->operator[](i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// operator const int *() /usr/include/opencv2/core/mat.hpp:593
	const int* cv_MatSize_operator_const_intX_const(const cv::MatSize* instance) {
			const int* ret = instance->operator const int*();
			return ret;
	}
	
	// operator==(const cv::MatSize &) /usr/include/opencv2/core/mat.hpp:594
	bool cv_MatSize_operatorEQ_const_const_MatSizeR(const cv::MatSize* instance, const cv::MatSize* sz) {
			bool ret = instance->operator==(*sz);
			return ret;
	}
	
	// p /usr/include/opencv2/core/mat.hpp:609
	size_t* cv_MatStep_getPropP(cv::MatStep* instance) {
			size_t* ret = instance->p;
			return ret;
	}
	
	// p /usr/include/opencv2/core/mat.hpp:609
	void cv_MatStep_setPropP_size_tX(cv::MatStep* instance, size_t* val) {
			instance->p = val;
	}
	
	// buf /usr/include/opencv2/core/mat.hpp:610
	size_t** cv_MatStep_getPropBuf(cv::MatStep* instance) {
			size_t(*ret)[2] = &instance->buf;
			return (size_t**)ret;
	}
	
	void cv_MatStep_delete(cv::MatStep* instance) {
		delete instance;
	}
	// MatStep() /usr/include/opencv2/core/mat.hpp:602
	cv::MatStep* cv_MatStep_MatStep() {
			cv::MatStep* ret = new cv::MatStep();
			return ret;
	}
	
	// MatStep(size_t) /usr/include/opencv2/core/mat.hpp:603
	cv::MatStep* cv_MatStep_MatStep_size_t(size_t s) {
			cv::MatStep* ret = new cv::MatStep(s);
			return ret;
	}
	
	// operator[](int) /usr/include/opencv2/core/mat.hpp:604
	const size_t cv_MatStep_operator___const_int(const cv::MatStep* instance, int i) {
			const size_t ret = instance->operator[](i);
			return ret;
	}
	
	// operator[](int) /usr/include/opencv2/core/mat.hpp:605
	size_t cv_MatStep_operator___int(cv::MatStep* instance, int i) {
			size_t ret = instance->operator[](i);
			return ret;
	}
	
	// operator unsigned long() /usr/include/opencv2/core/mat.hpp:606
	void cv_MatStep_operator_size_t_const(const cv::MatStep* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->operator size_t();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	void cv_Matx_AddOp_delete(cv::Matx_AddOp* instance) {
		delete instance;
	}
	// Matx_AddOp() /usr/include/opencv2/core/matx.hpp:68
	void cv_Matx_AddOp_Matx_AddOp(Result<cv::Matx_AddOp*>* ocvrs_return) {
		try {
			cv::Matx_AddOp* ret = new cv::Matx_AddOp();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Matx_AddOp*>))
	}
	
	// Matx_AddOp(const cv::Matx_AddOp &) /usr/include/opencv2/core/matx.hpp:68
	void cv_Matx_AddOp_Matx_AddOp_const_Matx_AddOpR(const cv::Matx_AddOp* unnamed, Result<cv::Matx_AddOp*>* ocvrs_return) {
		try {
			cv::Matx_AddOp* ret = new cv::Matx_AddOp(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Matx_AddOp*>))
	}
	
	void cv_Matx_DivOp_delete(cv::Matx_DivOp* instance) {
		delete instance;
	}
	// Matx_DivOp() /usr/include/opencv2/core/matx.hpp:72
	void cv_Matx_DivOp_Matx_DivOp(Result<cv::Matx_DivOp*>* ocvrs_return) {
		try {
			cv::Matx_DivOp* ret = new cv::Matx_DivOp();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Matx_DivOp*>))
	}
	
	// Matx_DivOp(const cv::Matx_DivOp &) /usr/include/opencv2/core/matx.hpp:72
	void cv_Matx_DivOp_Matx_DivOp_const_Matx_DivOpR(const cv::Matx_DivOp* unnamed, Result<cv::Matx_DivOp*>* ocvrs_return) {
		try {
			cv::Matx_DivOp* ret = new cv::Matx_DivOp(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Matx_DivOp*>))
	}
	
	void cv_Matx_MatMulOp_delete(cv::Matx_MatMulOp* instance) {
		delete instance;
	}
	// Matx_MatMulOp() /usr/include/opencv2/core/matx.hpp:73
	void cv_Matx_MatMulOp_Matx_MatMulOp(Result<cv::Matx_MatMulOp*>* ocvrs_return) {
		try {
			cv::Matx_MatMulOp* ret = new cv::Matx_MatMulOp();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Matx_MatMulOp*>))
	}
	
	// Matx_MatMulOp(const cv::Matx_MatMulOp &) /usr/include/opencv2/core/matx.hpp:73
	void cv_Matx_MatMulOp_Matx_MatMulOp_const_Matx_MatMulOpR(const cv::Matx_MatMulOp* unnamed, Result<cv::Matx_MatMulOp*>* ocvrs_return) {
		try {
			cv::Matx_MatMulOp* ret = new cv::Matx_MatMulOp(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Matx_MatMulOp*>))
	}
	
	void cv_Matx_MulOp_delete(cv::Matx_MulOp* instance) {
		delete instance;
	}
	// Matx_MulOp() /usr/include/opencv2/core/matx.hpp:71
	void cv_Matx_MulOp_Matx_MulOp(Result<cv::Matx_MulOp*>* ocvrs_return) {
		try {
			cv::Matx_MulOp* ret = new cv::Matx_MulOp();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Matx_MulOp*>))
	}
	
	// Matx_MulOp(const cv::Matx_MulOp &) /usr/include/opencv2/core/matx.hpp:71
	void cv_Matx_MulOp_Matx_MulOp_const_Matx_MulOpR(const cv::Matx_MulOp* unnamed, Result<cv::Matx_MulOp*>* ocvrs_return) {
		try {
			cv::Matx_MulOp* ret = new cv::Matx_MulOp(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Matx_MulOp*>))
	}
	
	void cv_Matx_ScaleOp_delete(cv::Matx_ScaleOp* instance) {
		delete instance;
	}
	// Matx_ScaleOp() /usr/include/opencv2/core/matx.hpp:70
	void cv_Matx_ScaleOp_Matx_ScaleOp(Result<cv::Matx_ScaleOp*>* ocvrs_return) {
		try {
			cv::Matx_ScaleOp* ret = new cv::Matx_ScaleOp();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Matx_ScaleOp*>))
	}
	
	// Matx_ScaleOp(const cv::Matx_ScaleOp &) /usr/include/opencv2/core/matx.hpp:70
	void cv_Matx_ScaleOp_Matx_ScaleOp_const_Matx_ScaleOpR(const cv::Matx_ScaleOp* unnamed, Result<cv::Matx_ScaleOp*>* ocvrs_return) {
		try {
			cv::Matx_ScaleOp* ret = new cv::Matx_ScaleOp(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Matx_ScaleOp*>))
	}
	
	void cv_Matx_SubOp_delete(cv::Matx_SubOp* instance) {
		delete instance;
	}
	// Matx_SubOp() /usr/include/opencv2/core/matx.hpp:69
	void cv_Matx_SubOp_Matx_SubOp(Result<cv::Matx_SubOp*>* ocvrs_return) {
		try {
			cv::Matx_SubOp* ret = new cv::Matx_SubOp();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Matx_SubOp*>))
	}
	
	// Matx_SubOp(const cv::Matx_SubOp &) /usr/include/opencv2/core/matx.hpp:69
	void cv_Matx_SubOp_Matx_SubOp_const_Matx_SubOpR(const cv::Matx_SubOp* unnamed, Result<cv::Matx_SubOp*>* ocvrs_return) {
		try {
			cv::Matx_SubOp* ret = new cv::Matx_SubOp(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Matx_SubOp*>))
	}
	
	void cv_Matx_TOp_delete(cv::Matx_TOp* instance) {
		delete instance;
	}
	// Matx_TOp() /usr/include/opencv2/core/matx.hpp:74
	void cv_Matx_TOp_Matx_TOp(Result<cv::Matx_TOp*>* ocvrs_return) {
		try {
			cv::Matx_TOp* ret = new cv::Matx_TOp();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Matx_TOp*>))
	}
	
	// Matx_TOp(const cv::Matx_TOp &) /usr/include/opencv2/core/matx.hpp:74
	void cv_Matx_TOp_Matx_TOp_const_Matx_TOpR(const cv::Matx_TOp* unnamed, Result<cv::Matx_TOp*>* ocvrs_return) {
		try {
			cv::Matx_TOp* ret = new cv::Matx_TOp(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Matx_TOp*>))
	}
	
	// getFunction() /usr/include/opencv2/core/optim.hpp:81
	void cv_MinProblemSolver_getFunction_const(const cv::MinProblemSolver* instance, Result<cv::Ptr<cv::MinProblemSolver::Function>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::MinProblemSolver::Function> ret = instance->getFunction();
			Ok(new cv::Ptr<cv::MinProblemSolver::Function>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::MinProblemSolver::Function>*>))
	}
	
	// setFunction(const Ptr<cv::MinProblemSolver::Function> &) /usr/include/opencv2/core/optim.hpp:89
	void cv_MinProblemSolver_setFunction_const_Ptr_Function_R(cv::MinProblemSolver* instance, const cv::Ptr<cv::MinProblemSolver::Function>* f, Result_void* ocvrs_return) {
		try {
			instance->setFunction(*f);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTermCriteria() /usr/include/opencv2/core/optim.hpp:95
	void cv_MinProblemSolver_getTermCriteria_const(const cv::MinProblemSolver* instance, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret = instance->getTermCriteria();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TermCriteria>))
	}
	
	// setTermCriteria(const cv::TermCriteria &) /usr/include/opencv2/core/optim.hpp:108
	void cv_MinProblemSolver_setTermCriteria_const_TermCriteriaR(cv::MinProblemSolver* instance, const cv::TermCriteria* termcrit, Result_void* ocvrs_return) {
		try {
			instance->setTermCriteria(*termcrit);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// minimize(cv::InputOutputArray) /usr/include/opencv2/core/optim.hpp:122
	void cv_MinProblemSolver_minimize_const__InputOutputArrayR(cv::MinProblemSolver* instance, const cv::_InputOutputArray* x, Result<double>* ocvrs_return) {
		try {
			double ret = instance->minimize(*x);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// getDims() /usr/include/opencv2/core/optim.hpp:67
	void cv_MinProblemSolver_Function_getDims_const(const cv::MinProblemSolver::Function* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDims();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getGradientEps() /usr/include/opencv2/core/optim.hpp:68
	void cv_MinProblemSolver_Function_getGradientEps_const(const cv::MinProblemSolver::Function* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getGradientEps();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// calc(const double *) /usr/include/opencv2/core/optim.hpp:69
	void cv_MinProblemSolver_Function_calc_const_const_doubleX(const cv::MinProblemSolver::Function* instance, const double* x, Result<double>* ocvrs_return) {
		try {
			double ret = instance->calc(x);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// getGradient(const double *, double *) /usr/include/opencv2/core/optim.hpp:70
	void cv_MinProblemSolver_Function_getGradient_const_doubleX_doubleX(cv::MinProblemSolver::Function* instance, const double* x, double* grad, Result_void* ocvrs_return) {
		try {
			instance->getGradient(x, grad);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// Moments() /usr/include/opencv2/core/types.hpp:952
	void cv_Moments_Moments(Result<cv::Moments>* ocvrs_return) {
		try {
			cv::Moments ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Moments>))
	}
	
	// Moments(double, double, double, double, double, double, double, double, double, double) /usr/include/opencv2/core/types.hpp:954
	void cv_Moments_Moments_double_double_double_double_double_double_double_double_double_double(double m00, double m10, double m01, double m20, double m11, double m02, double m30, double m21, double m12, double m03, Result<cv::Moments>* ocvrs_return) {
		try {
			cv::Moments ret(m00, m10, m01, m20, m11, m02, m30, m21, m12, m03);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Moments>))
	}
	
	// eigenvectors /usr/include/opencv2/core.hpp:2588
	cv::Mat* cv_PCA_getPropEigenvectors_const(const cv::PCA* instance) {
			cv::Mat ret = instance->eigenvectors;
			return new cv::Mat(ret);
	}
	
	// eigenvectors /usr/include/opencv2/core.hpp:2588
	void cv_PCA_setPropEigenvectors_Mat(cv::PCA* instance, cv::Mat* val) {
			instance->eigenvectors = *val;
	}
	
	// eigenvalues /usr/include/opencv2/core.hpp:2589
	cv::Mat* cv_PCA_getPropEigenvalues_const(const cv::PCA* instance) {
			cv::Mat ret = instance->eigenvalues;
			return new cv::Mat(ret);
	}
	
	// eigenvalues /usr/include/opencv2/core.hpp:2589
	void cv_PCA_setPropEigenvalues_Mat(cv::PCA* instance, cv::Mat* val) {
			instance->eigenvalues = *val;
	}
	
	// mean /usr/include/opencv2/core.hpp:2590
	cv::Mat* cv_PCA_getPropMean_const(const cv::PCA* instance) {
			cv::Mat ret = instance->mean;
			return new cv::Mat(ret);
	}
	
	// mean /usr/include/opencv2/core.hpp:2590
	void cv_PCA_setPropMean_Mat(cv::PCA* instance, cv::Mat* val) {
			instance->mean = *val;
	}
	
	void cv_PCA_delete(cv::PCA* instance) {
		delete instance;
	}
	// PCA() /usr/include/opencv2/core.hpp:2462
	void cv_PCA_PCA(Result<cv::PCA*>* ocvrs_return) {
		try {
			cv::PCA* ret = new cv::PCA();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::PCA*>))
	}
	
	// PCA(cv::InputArray, cv::InputArray, int, int) /usr/include/opencv2/core.hpp:2473
	void cv_PCA_PCA_const__InputArrayR_const__InputArrayR_int_int(const cv::_InputArray* data, const cv::_InputArray* mean, int flags, int maxComponents, Result<cv::PCA*>* ocvrs_return) {
		try {
			cv::PCA* ret = new cv::PCA(*data, *mean, flags, maxComponents);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::PCA*>))
	}
	
	// PCA(cv::InputArray, cv::InputArray, int, double) /usr/include/opencv2/core.hpp:2485
	void cv_PCA_PCA_const__InputArrayR_const__InputArrayR_int_double(const cv::_InputArray* data, const cv::_InputArray* mean, int flags, double retainedVariance, Result<cv::PCA*>* ocvrs_return) {
		try {
			cv::PCA* ret = new cv::PCA(*data, *mean, flags, retainedVariance);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::PCA*>))
	}
	
	// project(cv::InputArray) /usr/include/opencv2/core.hpp:2536
	void cv_PCA_project_const_const__InputArrayR(const cv::PCA* instance, const cv::_InputArray* vec, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->project(*vec);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// project(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2550
	void cv_PCA_project_const_const__InputArrayR_const__OutputArrayR(const cv::PCA* instance, const cv::_InputArray* vec, const cv::_OutputArray* result, Result_void* ocvrs_return) {
		try {
			instance->project(*vec, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// backProject(cv::InputArray) /usr/include/opencv2/core.hpp:2565
	void cv_PCA_backProject_const_const__InputArrayR(const cv::PCA* instance, const cv::_InputArray* vec, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->backProject(*vec);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// backProject(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2574
	void cv_PCA_backProject_const_const__InputArrayR_const__OutputArrayR(const cv::PCA* instance, const cv::_InputArray* vec, const cv::_OutputArray* result, Result_void* ocvrs_return) {
		try {
			instance->backProject(*vec, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &) /usr/include/opencv2/core.hpp:2580
	void cv_PCA_write_const_FileStorageR(const cv::PCA* instance, cv::FileStorage* fs, Result_void* ocvrs_return) {
		try {
			instance->write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// read(const cv::FileNode &) /usr/include/opencv2/core.hpp:2586
	void cv_PCA_read_const_FileNodeR(cv::PCA* instance, const cv::FileNode* fn, Result_void* ocvrs_return) {
		try {
			instance->read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// state /usr/include/opencv2/core.hpp:2972
	uint64_t cv_RNG_getPropState_const(const cv::RNG* instance) {
			uint64_t ret = instance->state;
			return ret;
	}
	
	// state /usr/include/opencv2/core.hpp:2972
	void cv_RNG_setPropState_uint64_t(cv::RNG* instance, uint64_t val) {
			instance->state = val;
	}
	
	void cv_RNG_delete(cv::RNG* instance) {
		delete instance;
	}
	// RNG() /usr/include/opencv2/core.hpp:2840
	void cv_RNG_RNG(Result<cv::RNG*>* ocvrs_return) {
		try {
			cv::RNG* ret = new cv::RNG();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RNG*>))
	}
	
	// RNG(uint64) /usr/include/opencv2/core.hpp:2844
	void cv_RNG_RNG_uint64_t(uint64_t state, Result<cv::RNG*>* ocvrs_return) {
		try {
			cv::RNG* ret = new cv::RNG(state);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RNG*>))
	}
	
	// next() /usr/include/opencv2/core.hpp:2847
	void cv_RNG_next(cv::RNG* instance, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->next();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	// operator unsigned char() /usr/include/opencv2/core.hpp:2855
	void cv_RNG_operator_unsigned_char(cv::RNG* instance, Result<unsigned char>* ocvrs_return) {
		try {
			unsigned char ret = instance->operator unsigned char();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char>))
	}
	
	// operator signed char() /usr/include/opencv2/core.hpp:2857
	void cv_RNG_operator_signed_char(cv::RNG* instance, Result<signed char>* ocvrs_return) {
		try {
			signed char ret = instance->operator signed char();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<signed char>))
	}
	
	// operator unsigned short() /usr/include/opencv2/core.hpp:2859
	void cv_RNG_operator_unsigned_short(cv::RNG* instance, Result<unsigned short>* ocvrs_return) {
		try {
			unsigned short ret = instance->operator unsigned short();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned short>))
	}
	
	// operator short() /usr/include/opencv2/core.hpp:2861
	void cv_RNG_operator_short(cv::RNG* instance, Result<short>* ocvrs_return) {
		try {
			short ret = instance->operator short();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<short>))
	}
	
	// operator unsigned int() /usr/include/opencv2/core.hpp:2863
	void cv_RNG_operator_unsigned_int(cv::RNG* instance, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->operator unsigned int();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	// operator int() /usr/include/opencv2/core.hpp:2865
	void cv_RNG_operator_int(cv::RNG* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->operator int();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// operator float() /usr/include/opencv2/core.hpp:2867
	void cv_RNG_operator_float(cv::RNG* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->operator float();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// operator double() /usr/include/opencv2/core.hpp:2869
	void cv_RNG_operator_double(cv::RNG* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->operator double();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// uniform(int, int) /usr/include/opencv2/core.hpp:2920
	void cv_RNG_uniform_int_int(cv::RNG* instance, int a, int b, Result<int>* ocvrs_return) {
		try {
			int ret = instance->uniform(a, b);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// uniform(float, float) /usr/include/opencv2/core.hpp:2922
	void cv_RNG_uniform_float_float(cv::RNG* instance, float a, float b, Result<float>* ocvrs_return) {
		try {
			float ret = instance->uniform(a, b);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// uniform(double, double) /usr/include/opencv2/core.hpp:2924
	void cv_RNG_uniform_double_double(cv::RNG* instance, double a, double b, Result<double>* ocvrs_return) {
		try {
			double ret = instance->uniform(a, b);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// fill(cv::InputOutputArray, int, cv::InputArray, cv::InputArray, bool) /usr/include/opencv2/core.hpp:2960
	void cv_RNG_fill_const__InputOutputArrayR_int_const__InputArrayR_const__InputArrayR_bool(cv::RNG* instance, const cv::_InputOutputArray* mat, int distType, const cv::_InputArray* a, const cv::_InputArray* b, bool saturateRange, Result_void* ocvrs_return) {
		try {
			instance->fill(*mat, distType, *a, *b, saturateRange);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// gaussian(double) /usr/include/opencv2/core.hpp:2970
	void cv_RNG_gaussian_double(cv::RNG* instance, double sigma, Result<double>* ocvrs_return) {
		try {
			double ret = instance->gaussian(sigma);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// operator==(const cv::RNG &) /usr/include/opencv2/core.hpp:2974
	void cv_RNG_operatorEQ_const_const_RNGR(const cv::RNG* instance, const cv::RNG* other, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator==(*other);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_RNG_MT19937_delete(cv::RNG_MT19937* instance) {
		delete instance;
	}
	// RNG_MT19937() /usr/include/opencv2/core.hpp:2985
	void cv_RNG_MT19937_RNG_MT19937(Result<cv::RNG_MT19937*>* ocvrs_return) {
		try {
			cv::RNG_MT19937* ret = new cv::RNG_MT19937();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RNG_MT19937*>))
	}
	
	// RNG_MT19937(unsigned int) /usr/include/opencv2/core.hpp:2986
	void cv_RNG_MT19937_RNG_MT19937_unsigned_int(unsigned int s, Result<cv::RNG_MT19937*>* ocvrs_return) {
		try {
			cv::RNG_MT19937* ret = new cv::RNG_MT19937(s);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RNG_MT19937*>))
	}
	
	// seed(unsigned int) /usr/include/opencv2/core.hpp:2987
	void cv_RNG_MT19937_seed_unsigned_int(cv::RNG_MT19937* instance, unsigned int s, Result_void* ocvrs_return) {
		try {
			instance->seed(s);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// next() /usr/include/opencv2/core.hpp:2989
	void cv_RNG_MT19937_next(cv::RNG_MT19937* instance, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->next();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	// operator int() /usr/include/opencv2/core.hpp:2991
	void cv_RNG_MT19937_operator_int(cv::RNG_MT19937* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->operator int();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// operator unsigned int() /usr/include/opencv2/core.hpp:2992
	void cv_RNG_MT19937_operator_unsigned_int(cv::RNG_MT19937* instance, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->operator unsigned int();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	// operator float() /usr/include/opencv2/core.hpp:2993
	void cv_RNG_MT19937_operator_float(cv::RNG_MT19937* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->operator float();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// operator double() /usr/include/opencv2/core.hpp:2994
	void cv_RNG_MT19937_operator_double(cv::RNG_MT19937* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->operator double();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// uniform(int, int) /usr/include/opencv2/core.hpp:3000
	void cv_RNG_MT19937_uniform_int_int(cv::RNG_MT19937* instance, int a, int b, Result<int>* ocvrs_return) {
		try {
			int ret = instance->uniform(a, b);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// uniform(float, float) /usr/include/opencv2/core.hpp:3002
	void cv_RNG_MT19937_uniform_float_float(cv::RNG_MT19937* instance, float a, float b, Result<float>* ocvrs_return) {
		try {
			float ret = instance->uniform(a, b);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// uniform(double, double) /usr/include/opencv2/core.hpp:3004
	void cv_RNG_MT19937_uniform_double_double(cv::RNG_MT19937* instance, double a, double b, Result<double>* ocvrs_return) {
		try {
			double ret = instance->uniform(a, b);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// start /usr/include/opencv2/core/types.hpp:620
	int cv_Range_getPropStart_const(const cv::Range* instance) {
			int ret = instance->start;
			return ret;
	}
	
	// start /usr/include/opencv2/core/types.hpp:620
	void cv_Range_setPropStart_int(cv::Range* instance, int val) {
			instance->start = val;
	}
	
	// end /usr/include/opencv2/core/types.hpp:620
	int cv_Range_getPropEnd_const(const cv::Range* instance) {
			int ret = instance->end;
			return ret;
	}
	
	// end /usr/include/opencv2/core/types.hpp:620
	void cv_Range_setPropEnd_int(cv::Range* instance, int val) {
			instance->end = val;
	}
	
	void cv_Range_delete(cv::Range* instance) {
		delete instance;
	}
	// Range() /usr/include/opencv2/core/types.hpp:614
	void cv_Range_Range(Result<cv::Range*>* ocvrs_return) {
		try {
			cv::Range* ret = new cv::Range();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Range*>))
	}
	
	// Range(int, int) /usr/include/opencv2/core/types.hpp:615
	void cv_Range_Range_int_int(int _start, int _end, Result<cv::Range*>* ocvrs_return) {
		try {
			cv::Range* ret = new cv::Range(_start, _end);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Range*>))
	}
	
	// size() /usr/include/opencv2/core/types.hpp:616
	void cv_Range_size_const(const cv::Range* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// empty() /usr/include/opencv2/core/types.hpp:617
	void cv_Range_empty_const(const cv::Range* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// all() /usr/include/opencv2/core/types.hpp:618
	void cv_Range_all(Result<cv::Range*>* ocvrs_return) {
		try {
			cv::Range ret = cv::Range::all();
			Ok(new cv::Range(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Range*>))
	}
	
	// center /usr/include/opencv2/core/types.hpp:552
	void cv_RotatedRect_getPropCenter_const(const cv::RotatedRect* instance, cv::Point2f* ocvrs_return) {
			cv::Point2f ret = instance->center;
			*ocvrs_return = ret;
	}
	
	// center /usr/include/opencv2/core/types.hpp:552
	void cv_RotatedRect_setPropCenter_Point2f(cv::RotatedRect* instance, cv::Point2f* val) {
			instance->center = *val;
	}
	
	// size /usr/include/opencv2/core/types.hpp:554
	void cv_RotatedRect_getPropSize_const(const cv::RotatedRect* instance, cv::Size2f* ocvrs_return) {
			cv::Size2f ret = instance->size;
			*ocvrs_return = ret;
	}
	
	// size /usr/include/opencv2/core/types.hpp:554
	void cv_RotatedRect_setPropSize_Size2f(cv::RotatedRect* instance, cv::Size2f* val) {
			instance->size = *val;
	}
	
	// angle /usr/include/opencv2/core/types.hpp:556
	float cv_RotatedRect_getPropAngle_const(const cv::RotatedRect* instance) {
			float ret = instance->angle;
			return ret;
	}
	
	// angle /usr/include/opencv2/core/types.hpp:556
	void cv_RotatedRect_setPropAngle_float(cv::RotatedRect* instance, float val) {
			instance->angle = val;
	}
	
	void cv_RotatedRect_delete(cv::RotatedRect* instance) {
		delete instance;
	}
	// RotatedRect() /usr/include/opencv2/core/types.hpp:529
	void cv_RotatedRect_RotatedRect(Result<cv::RotatedRect*>* ocvrs_return) {
		try {
			cv::RotatedRect* ret = new cv::RotatedRect();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RotatedRect*>))
	}
	
	// RotatedRect(const cv::Point2f &, const cv::Size2f &, float) /usr/include/opencv2/core/types.hpp:536
	void cv_RotatedRect_RotatedRect_const_Point2fR_const_Size2fR_float(const cv::Point2f* center, const cv::Size2f* size, float angle, Result<cv::RotatedRect*>* ocvrs_return) {
		try {
			cv::RotatedRect* ret = new cv::RotatedRect(*center, *size, angle);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RotatedRect*>))
	}
	
	// RotatedRect(const cv::Point2f &, const cv::Point2f &, const cv::Point2f &) /usr/include/opencv2/core/types.hpp:541
	void cv_RotatedRect_RotatedRect_const_Point2fR_const_Point2fR_const_Point2fR(const cv::Point2f* point1, const cv::Point2f* point2, const cv::Point2f* point3, Result<cv::RotatedRect*>* ocvrs_return) {
		try {
			cv::RotatedRect* ret = new cv::RotatedRect(*point1, *point2, *point3);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RotatedRect*>))
	}
	
	// points(cv::Point2f *) /usr/include/opencv2/core/types.hpp:546
	void cv_RotatedRect_points_const_Point2fX(const cv::RotatedRect* instance, cv::Point2f* pts, Result_void* ocvrs_return) {
		try {
			instance->points(pts);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// boundingRect() /usr/include/opencv2/core/types.hpp:548
	void cv_RotatedRect_boundingRect_const(const cv::RotatedRect* instance, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->boundingRect();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// boundingRect2f() /usr/include/opencv2/core/types.hpp:550
	void cv_RotatedRect_boundingRect2f_const(const cv::RotatedRect* instance, Result<cv::Rect_<float>>* ocvrs_return) {
		try {
			cv::Rect_<float> ret = instance->boundingRect2f();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect_<float>>))
	}
	
	// u /usr/include/opencv2/core.hpp:2810
	cv::Mat* cv_SVD_getPropU_const(const cv::SVD* instance) {
			cv::Mat ret = instance->u;
			return new cv::Mat(ret);
	}
	
	// u /usr/include/opencv2/core.hpp:2810
	void cv_SVD_setPropU_Mat(cv::SVD* instance, cv::Mat* val) {
			instance->u = *val;
	}
	
	// w /usr/include/opencv2/core.hpp:2810
	cv::Mat* cv_SVD_getPropW_const(const cv::SVD* instance) {
			cv::Mat ret = instance->w;
			return new cv::Mat(ret);
	}
	
	// w /usr/include/opencv2/core.hpp:2810
	void cv_SVD_setPropW_Mat(cv::SVD* instance, cv::Mat* val) {
			instance->w = *val;
	}
	
	// vt /usr/include/opencv2/core.hpp:2810
	cv::Mat* cv_SVD_getPropVt_const(const cv::SVD* instance) {
			cv::Mat ret = instance->vt;
			return new cv::Mat(ret);
	}
	
	// vt /usr/include/opencv2/core.hpp:2810
	void cv_SVD_setPropVt_Mat(cv::SVD* instance, cv::Mat* val) {
			instance->vt = *val;
	}
	
	void cv_SVD_delete(cv::SVD* instance) {
		delete instance;
	}
	// SVD() /usr/include/opencv2/core.hpp:2706
	void cv_SVD_SVD(Result<cv::SVD*>* ocvrs_return) {
		try {
			cv::SVD* ret = new cv::SVD();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SVD*>))
	}
	
	// SVD(cv::InputArray, int) /usr/include/opencv2/core.hpp:2713
	void cv_SVD_SVD_const__InputArrayR_int(const cv::_InputArray* src, int flags, Result<cv::SVD*>* ocvrs_return) {
		try {
			cv::SVD* ret = new cv::SVD(*src, flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SVD*>))
	}
	
	// compute(cv::InputArray, cv::OutputArray, cv::OutputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2745
	void cv_SVD_compute_const__InputArrayR_const__OutputArrayR_const__OutputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* w, const cv::_OutputArray* u, const cv::_OutputArray* vt, int flags, Result_void* ocvrs_return) {
		try {
			cv::SVD::compute(*src, *w, *u, *vt, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// compute(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/core.hpp:2754
	void cv_SVD_compute_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* w, int flags, Result_void* ocvrs_return) {
		try {
			cv::SVD::compute(*src, *w, flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// backSubst(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2758
	void cv_SVD_backSubst_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* w, const cv::_InputArray* u, const cv::_InputArray* vt, const cv::_InputArray* rhs, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::SVD::backSubst(*w, *u, *vt, *rhs, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// solveZ(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2772
	void cv_SVD_solveZ_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::SVD::solveZ(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// backSubst(cv::InputArray, cv::OutputArray) /usr/include/opencv2/core.hpp:2796
	void cv_SVD_backSubst_const_const__InputArrayR_const__OutputArrayR(const cv::SVD* instance, const cv::_InputArray* rhs, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			instance->backSubst(*rhs, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// flags /usr/include/opencv2/core/mat.hpp:2971
	int cv_SparseMat_getPropFlags_const(const cv::SparseMat* instance) {
			int ret = instance->flags;
			return ret;
	}
	
	// flags /usr/include/opencv2/core/mat.hpp:2971
	void cv_SparseMat_setPropFlags_int(cv::SparseMat* instance, int val) {
			instance->flags = val;
	}
	
	// hdr /usr/include/opencv2/core/mat.hpp:2972
	cv::SparseMat::Hdr** cv_SparseMat_getPropHdr(cv::SparseMat* instance) {
			cv::SparseMat::Hdr* ret = instance->hdr;
			return new cv::SparseMat::Hdr*(ret);
	}
	
	// hdr /usr/include/opencv2/core/mat.hpp:2972
	void cv_SparseMat_setPropHdr_HdrX(cv::SparseMat* instance, cv::SparseMat::Hdr* val) {
			instance->hdr = val;
	}
	
	void cv_SparseMat_delete(cv::SparseMat* instance) {
		delete instance;
	}
	// SparseMat() /usr/include/opencv2/core/mat.hpp:2749
	void cv_SparseMat_SparseMat(Result<cv::SparseMat*>* ocvrs_return) {
		try {
			cv::SparseMat* ret = new cv::SparseMat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMat*>))
	}
	
	// SparseMat(int, const int *, int) /usr/include/opencv2/core/mat.hpp:2756
	void cv_SparseMat_SparseMat_int_const_intX_int(int dims, const int* _sizes, int _type, Result<cv::SparseMat*>* ocvrs_return) {
		try {
			cv::SparseMat* ret = new cv::SparseMat(dims, _sizes, _type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMat*>))
	}
	
	// SparseMat(const cv::SparseMat &) /usr/include/opencv2/core/mat.hpp:2762
	void cv_SparseMat_SparseMat_const_SparseMatR(const cv::SparseMat* m, Result<cv::SparseMat*>* ocvrs_return) {
		try {
			cv::SparseMat* ret = new cv::SparseMat(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMat*>))
	}
	
	// SparseMat(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:2768
	void cv_SparseMat_SparseMat_const_MatR(const cv::Mat* m, Result<cv::SparseMat*>* ocvrs_return) {
		try {
			cv::SparseMat* ret = new cv::SparseMat(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMat*>))
	}
	
	// clone() /usr/include/opencv2/core/mat.hpp:2779
	void cv_SparseMat_clone_const(const cv::SparseMat* instance, Result<cv::SparseMat*>* ocvrs_return) {
		try {
			cv::SparseMat ret = instance->clone();
			Ok(new cv::SparseMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMat*>))
	}
	
	// copyTo(cv::SparseMat &) /usr/include/opencv2/core/mat.hpp:2782
	void cv_SparseMat_copyTo_const_SparseMatR(const cv::SparseMat* instance, cv::SparseMat* m, Result_void* ocvrs_return) {
		try {
			instance->copyTo(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// copyTo(cv::Mat &) /usr/include/opencv2/core/mat.hpp:2784
	void cv_SparseMat_copyTo_const_MatR(const cv::SparseMat* instance, cv::Mat* m, Result_void* ocvrs_return) {
		try {
			instance->copyTo(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// convertTo(cv::SparseMat &, int, double) /usr/include/opencv2/core/mat.hpp:2786
	void cv_SparseMat_convertTo_const_SparseMatR_int_double(const cv::SparseMat* instance, cv::SparseMat* m, int rtype, double alpha, Result_void* ocvrs_return) {
		try {
			instance->convertTo(*m, rtype, alpha);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// convertTo(cv::Mat &, int, double, double) /usr/include/opencv2/core/mat.hpp:2797
	void cv_SparseMat_convertTo_const_MatR_int_double_double(const cv::SparseMat* instance, cv::Mat* m, int rtype, double alpha, double beta, Result_void* ocvrs_return) {
		try {
			instance->convertTo(*m, rtype, alpha, beta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// assignTo(cv::SparseMat &, int) /usr/include/opencv2/core/mat.hpp:2800
	void cv_SparseMat_assignTo_const_SparseMatR_int(const cv::SparseMat* instance, cv::SparseMat* m, int type, Result_void* ocvrs_return) {
		try {
			instance->assignTo(*m, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(int, const int *, int) /usr/include/opencv2/core/mat.hpp:2808
	void cv_SparseMat_create_int_const_intX_int(cv::SparseMat* instance, int dims, const int* _sizes, int _type, Result_void* ocvrs_return) {
		try {
			instance->create(dims, _sizes, _type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// clear() /usr/include/opencv2/core/mat.hpp:2810
	void cv_SparseMat_clear(cv::SparseMat* instance, Result_void* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// addref() /usr/include/opencv2/core/mat.hpp:2812
	void cv_SparseMat_addref(cv::SparseMat* instance, Result_void* ocvrs_return) {
		try {
			instance->addref();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// release() /usr/include/opencv2/core/mat.hpp:2814
	void cv_SparseMat_release(cv::SparseMat* instance, Result_void* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// elemSize() /usr/include/opencv2/core/mat.hpp:2819
	size_t cv_SparseMat_elemSize_const(const cv::SparseMat* instance) {
			size_t ret = instance->elemSize();
			return ret;
	}
	
	// elemSize1() /usr/include/opencv2/core/mat.hpp:2821
	size_t cv_SparseMat_elemSize1_const(const cv::SparseMat* instance) {
			size_t ret = instance->elemSize1();
			return ret;
	}
	
	// type() /usr/include/opencv2/core/mat.hpp:2824
	int cv_SparseMat_type_const(const cv::SparseMat* instance) {
			int ret = instance->type();
			return ret;
	}
	
	// depth() /usr/include/opencv2/core/mat.hpp:2826
	int cv_SparseMat_depth_const(const cv::SparseMat* instance) {
			int ret = instance->depth();
			return ret;
	}
	
	// channels() /usr/include/opencv2/core/mat.hpp:2828
	int cv_SparseMat_channels_const(const cv::SparseMat* instance) {
			int ret = instance->channels();
			return ret;
	}
	
	// size() /usr/include/opencv2/core/mat.hpp:2831
	void cv_SparseMat_size_const(const cv::SparseMat* instance, Result<const int*>* ocvrs_return) {
		try {
			const int* ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const int*>))
	}
	
	// size(int) /usr/include/opencv2/core/mat.hpp:2833
	void cv_SparseMat_size_const_int(const cv::SparseMat* instance, int i, Result<int>* ocvrs_return) {
		try {
			int ret = instance->size(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// dims() /usr/include/opencv2/core/mat.hpp:2835
	void cv_SparseMat_dims_const(const cv::SparseMat* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->dims();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// nzcount() /usr/include/opencv2/core/mat.hpp:2837
	void cv_SparseMat_nzcount_const(const cv::SparseMat* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->nzcount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// hash(int) /usr/include/opencv2/core/mat.hpp:2840
	void cv_SparseMat_hash_const_int(const cv::SparseMat* instance, int i0, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->hash(i0);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// hash(int, int) /usr/include/opencv2/core/mat.hpp:2842
	void cv_SparseMat_hash_const_int_int(const cv::SparseMat* instance, int i0, int i1, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->hash(i0, i1);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// hash(int, int, int) /usr/include/opencv2/core/mat.hpp:2844
	void cv_SparseMat_hash_const_int_int_int(const cv::SparseMat* instance, int i0, int i1, int i2, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->hash(i0, i1, i2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// hash(const int *) /usr/include/opencv2/core/mat.hpp:2846
	void cv_SparseMat_hash_const_const_intX(const cv::SparseMat* instance, const int* idx, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->hash(idx);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// ptr(int, bool, size_t *) /usr/include/opencv2/core/mat.hpp:2860
	void cv_SparseMat_ptr_int_bool_size_tX(cv::SparseMat* instance, int i0, bool createMissing, size_t* hashval, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(i0, createMissing, hashval);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	// ptr(int, int, bool, size_t *) /usr/include/opencv2/core/mat.hpp:2862
	void cv_SparseMat_ptr_int_int_bool_size_tX(cv::SparseMat* instance, int i0, int i1, bool createMissing, size_t* hashval, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(i0, i1, createMissing, hashval);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	// ptr(int, int, int, bool, size_t *) /usr/include/opencv2/core/mat.hpp:2864
	void cv_SparseMat_ptr_int_int_int_bool_size_tX(cv::SparseMat* instance, int i0, int i1, int i2, bool createMissing, size_t* hashval, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(i0, i1, i2, createMissing, hashval);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	// ptr(const int *, bool, size_t *) /usr/include/opencv2/core/mat.hpp:2866
	void cv_SparseMat_ptr_const_intX_bool_size_tX(cv::SparseMat* instance, const int* idx, bool createMissing, size_t* hashval, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(idx, createMissing, hashval);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	// erase(int, int, size_t *) /usr/include/opencv2/core/mat.hpp:2927
	void cv_SparseMat_erase_int_int_size_tX(cv::SparseMat* instance, int i0, int i1, size_t* hashval, Result_void* ocvrs_return) {
		try {
			instance->erase(i0, i1, hashval);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// erase(int, int, int, size_t *) /usr/include/opencv2/core/mat.hpp:2929
	void cv_SparseMat_erase_int_int_int_size_tX(cv::SparseMat* instance, int i0, int i1, int i2, size_t* hashval, Result_void* ocvrs_return) {
		try {
			instance->erase(i0, i1, i2, hashval);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// erase(const int *, size_t *) /usr/include/opencv2/core/mat.hpp:2931
	void cv_SparseMat_erase_const_intX_size_tX(cv::SparseMat* instance, const int* idx, size_t* hashval, Result_void* ocvrs_return) {
		try {
			instance->erase(idx, hashval);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// begin() /usr/include/opencv2/core/mat.hpp:2938
	void cv_SparseMat_begin(cv::SparseMat* instance, Result<cv::SparseMatIterator*>* ocvrs_return) {
		try {
			cv::SparseMatIterator ret = instance->begin();
			Ok(new cv::SparseMatIterator(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatIterator*>))
	}
	
	// begin() /usr/include/opencv2/core/mat.hpp:2942
	void cv_SparseMat_begin_const(const cv::SparseMat* instance, Result<cv::SparseMatConstIterator*>* ocvrs_return) {
		try {
			cv::SparseMatConstIterator ret = instance->begin();
			Ok(new cv::SparseMatConstIterator(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatConstIterator*>))
	}
	
	// end() /usr/include/opencv2/core/mat.hpp:2950
	void cv_SparseMat_end(cv::SparseMat* instance, Result<cv::SparseMatIterator*>* ocvrs_return) {
		try {
			cv::SparseMatIterator ret = instance->end();
			Ok(new cv::SparseMatIterator(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatIterator*>))
	}
	
	// end() /usr/include/opencv2/core/mat.hpp:2952
	void cv_SparseMat_end_const(const cv::SparseMat* instance, Result<cv::SparseMatConstIterator*>* ocvrs_return) {
		try {
			cv::SparseMatConstIterator ret = instance->end();
			Ok(new cv::SparseMatConstIterator(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatConstIterator*>))
	}
	
	// node(size_t) /usr/include/opencv2/core/mat.hpp:2964
	void cv_SparseMat_node_size_t(cv::SparseMat* instance, size_t nidx, Result<cv::SparseMat::Node**>* ocvrs_return) {
		try {
			cv::SparseMat::Node* ret = instance->node(nidx);
			Ok(new cv::SparseMat::Node*(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMat::Node**>))
	}
	
	// node(size_t) /usr/include/opencv2/core/mat.hpp:2965
	void cv_SparseMat_node_const_size_t(const cv::SparseMat* instance, size_t nidx, Result<const cv::SparseMat::Node**>* ocvrs_return) {
		try {
			const cv::SparseMat::Node* ret = instance->node(nidx);
			Ok(new const cv::SparseMat::Node*(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::SparseMat::Node**>))
	}
	
	// newNode(const int *, size_t) /usr/include/opencv2/core/mat.hpp:2967
	void cv_SparseMat_newNode_const_intX_size_t(cv::SparseMat* instance, const int* idx, size_t hashval, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->newNode(idx, hashval);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	// removeNode(size_t, size_t, size_t) /usr/include/opencv2/core/mat.hpp:2968
	void cv_SparseMat_removeNode_size_t_size_t_size_t(cv::SparseMat* instance, size_t hidx, size_t nidx, size_t previdx, Result_void* ocvrs_return) {
		try {
			instance->removeNode(hidx, nidx, previdx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// resizeHashTab(size_t) /usr/include/opencv2/core/mat.hpp:2969
	void cv_SparseMat_resizeHashTab_size_t(cv::SparseMat* instance, size_t newsize, Result_void* ocvrs_return) {
		try {
			instance->resizeHashTab(newsize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// refcount /usr/include/opencv2/core/mat.hpp:2725
	int cv_SparseMat_Hdr_getPropRefcount_const(const cv::SparseMat::Hdr* instance) {
			int ret = instance->refcount;
			return ret;
	}
	
	// refcount /usr/include/opencv2/core/mat.hpp:2725
	void cv_SparseMat_Hdr_setPropRefcount_int(cv::SparseMat::Hdr* instance, int val) {
			instance->refcount = val;
	}
	
	// dims /usr/include/opencv2/core/mat.hpp:2726
	int cv_SparseMat_Hdr_getPropDims_const(const cv::SparseMat::Hdr* instance) {
			int ret = instance->dims;
			return ret;
	}
	
	// dims /usr/include/opencv2/core/mat.hpp:2726
	void cv_SparseMat_Hdr_setPropDims_int(cv::SparseMat::Hdr* instance, int val) {
			instance->dims = val;
	}
	
	// valueOffset /usr/include/opencv2/core/mat.hpp:2727
	int cv_SparseMat_Hdr_getPropValueOffset_const(const cv::SparseMat::Hdr* instance) {
			int ret = instance->valueOffset;
			return ret;
	}
	
	// valueOffset /usr/include/opencv2/core/mat.hpp:2727
	void cv_SparseMat_Hdr_setPropValueOffset_int(cv::SparseMat::Hdr* instance, int val) {
			instance->valueOffset = val;
	}
	
	// nodeSize /usr/include/opencv2/core/mat.hpp:2728
	size_t cv_SparseMat_Hdr_getPropNodeSize_const(const cv::SparseMat::Hdr* instance) {
			size_t ret = instance->nodeSize;
			return ret;
	}
	
	// nodeSize /usr/include/opencv2/core/mat.hpp:2728
	void cv_SparseMat_Hdr_setPropNodeSize_size_t(cv::SparseMat::Hdr* instance, size_t val) {
			instance->nodeSize = val;
	}
	
	// nodeCount /usr/include/opencv2/core/mat.hpp:2729
	size_t cv_SparseMat_Hdr_getPropNodeCount_const(const cv::SparseMat::Hdr* instance) {
			size_t ret = instance->nodeCount;
			return ret;
	}
	
	// nodeCount /usr/include/opencv2/core/mat.hpp:2729
	void cv_SparseMat_Hdr_setPropNodeCount_size_t(cv::SparseMat::Hdr* instance, size_t val) {
			instance->nodeCount = val;
	}
	
	// freeList /usr/include/opencv2/core/mat.hpp:2730
	size_t cv_SparseMat_Hdr_getPropFreeList_const(const cv::SparseMat::Hdr* instance) {
			size_t ret = instance->freeList;
			return ret;
	}
	
	// freeList /usr/include/opencv2/core/mat.hpp:2730
	void cv_SparseMat_Hdr_setPropFreeList_size_t(cv::SparseMat::Hdr* instance, size_t val) {
			instance->freeList = val;
	}
	
	// pool /usr/include/opencv2/core/mat.hpp:2731
	std::vector<unsigned char>* cv_SparseMat_Hdr_getPropPool_const(const cv::SparseMat::Hdr* instance) {
			std::vector<unsigned char> ret = instance->pool;
			return new std::vector<unsigned char>(ret);
	}
	
	// pool /usr/include/opencv2/core/mat.hpp:2731
	void cv_SparseMat_Hdr_setPropPool_vector_unsigned_char_(cv::SparseMat::Hdr* instance, std::vector<unsigned char>* val) {
			instance->pool = *val;
	}
	
	// hashtab /usr/include/opencv2/core/mat.hpp:2732
	std::vector<size_t>* cv_SparseMat_Hdr_getPropHashtab_const(const cv::SparseMat::Hdr* instance) {
			std::vector<size_t> ret = instance->hashtab;
			return new std::vector<size_t>(ret);
	}
	
	// hashtab /usr/include/opencv2/core/mat.hpp:2732
	void cv_SparseMat_Hdr_setPropHashtab_vector_size_t_(cv::SparseMat::Hdr* instance, std::vector<size_t>* val) {
			instance->hashtab = *val;
	}
	
	// size /usr/include/opencv2/core/mat.hpp:2733
	int** cv_SparseMat_Hdr_getPropSize(cv::SparseMat::Hdr* instance) {
			int(*ret)[32] = &instance->size;
			return (int**)ret;
	}
	
	void cv_SparseMat_Hdr_delete(cv::SparseMat::Hdr* instance) {
		delete instance;
	}
	// Hdr(int, const int *, int) /usr/include/opencv2/core/mat.hpp:2723
	void cv_SparseMat_Hdr_Hdr_int_const_intX_int(int _dims, const int* _sizes, int _type, Result<cv::SparseMat::Hdr*>* ocvrs_return) {
		try {
			cv::SparseMat::Hdr* ret = new cv::SparseMat::Hdr(_dims, _sizes, _type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMat::Hdr*>))
	}
	
	// clear() /usr/include/opencv2/core/mat.hpp:2724
	void cv_SparseMat_Hdr_clear(cv::SparseMat::Hdr* instance, Result_void* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// hashval /usr/include/opencv2/core/mat.hpp:2740
	size_t cv_SparseMat_Node_getPropHashval_const(const cv::SparseMat::Node* instance) {
			size_t ret = instance->hashval;
			return ret;
	}
	
	// hashval /usr/include/opencv2/core/mat.hpp:2740
	void cv_SparseMat_Node_setPropHashval_size_t(cv::SparseMat::Node* instance, size_t val) {
			instance->hashval = val;
	}
	
	// next /usr/include/opencv2/core/mat.hpp:2742
	size_t cv_SparseMat_Node_getPropNext_const(const cv::SparseMat::Node* instance) {
			size_t ret = instance->next;
			return ret;
	}
	
	// next /usr/include/opencv2/core/mat.hpp:2742
	void cv_SparseMat_Node_setPropNext_size_t(cv::SparseMat::Node* instance, size_t val) {
			instance->next = val;
	}
	
	// idx /usr/include/opencv2/core/mat.hpp:2744
	int** cv_SparseMat_Node_getPropIdx(cv::SparseMat::Node* instance) {
			int(*ret)[32] = &instance->idx;
			return (int**)ret;
	}
	
	void cv_SparseMat_Node_delete(cv::SparseMat::Node* instance) {
		delete instance;
	}
	// m /usr/include/opencv2/core/mat.hpp:3267
	const cv::SparseMat** cv_SparseMatConstIterator_getPropM_const(const cv::SparseMatConstIterator* instance) {
			const cv::SparseMat* ret = instance->m;
			return new const cv::SparseMat*(ret);
	}
	
	// hashidx /usr/include/opencv2/core/mat.hpp:3268
	size_t cv_SparseMatConstIterator_getPropHashidx_const(const cv::SparseMatConstIterator* instance) {
			size_t ret = instance->hashidx;
			return ret;
	}
	
	// hashidx /usr/include/opencv2/core/mat.hpp:3268
	void cv_SparseMatConstIterator_setPropHashidx_size_t(cv::SparseMatConstIterator* instance, size_t val) {
			instance->hashidx = val;
	}
	
	// ptr /usr/include/opencv2/core/mat.hpp:3269
	unsigned char* cv_SparseMatConstIterator_getPropPtr(cv::SparseMatConstIterator* instance) {
			unsigned char* ret = instance->ptr;
			return ret;
	}
	
	// ptr /usr/include/opencv2/core/mat.hpp:3269
	void cv_SparseMatConstIterator_setPropPtr_unsigned_charX(cv::SparseMatConstIterator* instance, unsigned char* val) {
			instance->ptr = val;
	}
	
	void cv_SparseMatConstIterator_delete(cv::SparseMatConstIterator* instance) {
		delete instance;
	}
	// SparseMatConstIterator() /usr/include/opencv2/core/mat.hpp:3241
	void cv_SparseMatConstIterator_SparseMatConstIterator(Result<cv::SparseMatConstIterator*>* ocvrs_return) {
		try {
			cv::SparseMatConstIterator* ret = new cv::SparseMatConstIterator();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatConstIterator*>))
	}
	
	// SparseMatConstIterator(const cv::SparseMat *) /usr/include/opencv2/core/mat.hpp:3243
	void cv_SparseMatConstIterator_SparseMatConstIterator_const_SparseMatX(const cv::SparseMat* _m, Result<cv::SparseMatConstIterator*>* ocvrs_return) {
		try {
			cv::SparseMatConstIterator* ret = new cv::SparseMatConstIterator(_m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatConstIterator*>))
	}
	
	// SparseMatConstIterator(const cv::SparseMatConstIterator &) /usr/include/opencv2/core/mat.hpp:3245
	void cv_SparseMatConstIterator_SparseMatConstIterator_const_SparseMatConstIteratorR(const cv::SparseMatConstIterator* it, Result<cv::SparseMatConstIterator*>* ocvrs_return) {
		try {
			cv::SparseMatConstIterator* ret = new cv::SparseMatConstIterator(*it);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatConstIterator*>))
	}
	
	// node() /usr/include/opencv2/core/mat.hpp:3253
	void cv_SparseMatConstIterator_node_const(const cv::SparseMatConstIterator* instance, Result<const cv::SparseMat::Node**>* ocvrs_return) {
		try {
			const cv::SparseMat::Node* ret = instance->node();
			Ok(new const cv::SparseMat::Node*(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const cv::SparseMat::Node**>))
	}
	
	#if !defined(OCVRS_TARGET_OS_WINDOWS)
	// operator--() /usr/include/opencv2/core/mat.hpp:3256
	void cv_SparseMatConstIterator_operatorSS(cv::SparseMatConstIterator* instance, Result<cv::SparseMatConstIterator*>* ocvrs_return) {
		try {
			cv::SparseMatConstIterator ret = instance->operator--();
			Ok(new cv::SparseMatConstIterator(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatConstIterator*>))
	}
	#endif
	
	// operator++() /usr/include/opencv2/core/mat.hpp:3260
	void cv_SparseMatConstIterator_operatorAA(cv::SparseMatConstIterator* instance, Result<cv::SparseMatConstIterator*>* ocvrs_return) {
		try {
			cv::SparseMatConstIterator ret = instance->operator++();
			Ok(new cv::SparseMatConstIterator(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatConstIterator*>))
	}
	
	// seekEnd() /usr/include/opencv2/core/mat.hpp:3265
	void cv_SparseMatConstIterator_seekEnd(cv::SparseMatConstIterator* instance, Result_void* ocvrs_return) {
		try {
			instance->seekEnd();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::SparseMatConstIterator* cv_SparseMatIterator_to_SparseMatConstIterator(cv::SparseMatIterator* instance) {
		return dynamic_cast<cv::SparseMatConstIterator*>(instance);
	}
	
	void cv_SparseMatIterator_delete(cv::SparseMatIterator* instance) {
		delete instance;
	}
	// SparseMatIterator() /usr/include/opencv2/core/mat.hpp:3285
	void cv_SparseMatIterator_SparseMatIterator(Result<cv::SparseMatIterator*>* ocvrs_return) {
		try {
			cv::SparseMatIterator* ret = new cv::SparseMatIterator();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatIterator*>))
	}
	
	// SparseMatIterator(cv::SparseMat *) /usr/include/opencv2/core/mat.hpp:3287
	void cv_SparseMatIterator_SparseMatIterator_SparseMatX(cv::SparseMat* _m, Result<cv::SparseMatIterator*>* ocvrs_return) {
		try {
			cv::SparseMatIterator* ret = new cv::SparseMatIterator(_m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatIterator*>))
	}
	
	// SparseMatIterator(const cv::SparseMatIterator &) /usr/include/opencv2/core/mat.hpp:3291
	void cv_SparseMatIterator_SparseMatIterator_const_SparseMatIteratorR(const cv::SparseMatIterator* it, Result<cv::SparseMatIterator*>* ocvrs_return) {
		try {
			cv::SparseMatIterator* ret = new cv::SparseMatIterator(*it);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatIterator*>))
	}
	
	// node() /usr/include/opencv2/core/mat.hpp:3298
	void cv_SparseMatIterator_node_const(const cv::SparseMatIterator* instance, Result<cv::SparseMat::Node**>* ocvrs_return) {
		try {
			cv::SparseMat::Node* ret = instance->node();
			Ok(new cv::SparseMat::Node*(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMat::Node**>))
	}
	
	// operator++() /usr/include/opencv2/core/mat.hpp:3301
	void cv_SparseMatIterator_operatorAA(cv::SparseMatIterator* instance, Result<cv::SparseMatIterator*>* ocvrs_return) {
		try {
			cv::SparseMatIterator ret = instance->operator++();
			Ok(new cv::SparseMatIterator(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SparseMatIterator*>))
	}
	
	// cleanup() /usr/include/opencv2/core/utils/tls.hpp:49
	void cv_TLSDataContainer_cleanup(cv::TLSDataContainer* instance, Result_void* ocvrs_return) {
		try {
			instance->cleanup();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// TermCriteria() /usr/include/opencv2/core/types.hpp:888
	void cv_TermCriteria_TermCriteria(Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TermCriteria>))
	}
	
	// TermCriteria(int, int, double) /usr/include/opencv2/core/types.hpp:894
	void cv_TermCriteria_TermCriteria_int_int_double(int type, int maxCount, double epsilon, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret(type, maxCount, epsilon);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TermCriteria>))
	}
	
	// isValid() /usr/include/opencv2/core/types.hpp:896
	void cv_TermCriteria_isValid_const(const cv::TermCriteria instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance.isValid();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_TickMeter_delete(cv::TickMeter* instance) {
		delete instance;
	}
	// TickMeter() /usr/include/opencv2/core/utility.hpp:298
	void cv_TickMeter_TickMeter(Result<cv::TickMeter*>* ocvrs_return) {
		try {
			cv::TickMeter* ret = new cv::TickMeter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TickMeter*>))
	}
	
	// start() /usr/include/opencv2/core/utility.hpp:304
	void cv_TickMeter_start(cv::TickMeter* instance, Result_void* ocvrs_return) {
		try {
			instance->start();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// stop() /usr/include/opencv2/core/utility.hpp:310
	void cv_TickMeter_stop(cv::TickMeter* instance, Result_void* ocvrs_return) {
		try {
			instance->stop();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTimeTicks() /usr/include/opencv2/core/utility.hpp:321
	void cv_TickMeter_getTimeTicks_const(const cv::TickMeter* instance, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->getTimeTicks();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int64_t>))
	}
	
	// getTimeMicro() /usr/include/opencv2/core/utility.hpp:327
	void cv_TickMeter_getTimeMicro_const(const cv::TickMeter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTimeMicro();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// getTimeMilli() /usr/include/opencv2/core/utility.hpp:333
	void cv_TickMeter_getTimeMilli_const(const cv::TickMeter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTimeMilli();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// getTimeSec() /usr/include/opencv2/core/utility.hpp:339
	void cv_TickMeter_getTimeSec_const(const cv::TickMeter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTimeSec();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// getCounter() /usr/include/opencv2/core/utility.hpp:345
	void cv_TickMeter_getCounter_const(const cv::TickMeter* instance, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->getCounter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int64_t>))
	}
	
	// getFPS() /usr/include/opencv2/core/utility.hpp:351
	void cv_TickMeter_getFPS_const(const cv::TickMeter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getFPS();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// getAvgTimeSec() /usr/include/opencv2/core/utility.hpp:360
	void cv_TickMeter_getAvgTimeSec_const(const cv::TickMeter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAvgTimeSec();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// getAvgTimeMilli() /usr/include/opencv2/core/utility.hpp:368
	void cv_TickMeter_getAvgTimeMilli_const(const cv::TickMeter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getAvgTimeMilli();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// reset() /usr/include/opencv2/core/utility.hpp:374
	void cv_TickMeter_reset(cv::TickMeter* instance, Result_void* ocvrs_return) {
		try {
			instance->reset();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// flags /usr/include/opencv2/core/mat.hpp:2592
	int cv_UMat_getPropFlags_const(const cv::UMat* instance) {
			int ret = instance->flags;
			return ret;
	}
	
	// flags /usr/include/opencv2/core/mat.hpp:2592
	void cv_UMat_setPropFlags_int(cv::UMat* instance, int val) {
			instance->flags = val;
	}
	
	// dims /usr/include/opencv2/core/mat.hpp:2595
	int cv_UMat_getPropDims_const(const cv::UMat* instance) {
			int ret = instance->dims;
			return ret;
	}
	
	// dims /usr/include/opencv2/core/mat.hpp:2595
	void cv_UMat_setPropDims_int(cv::UMat* instance, int val) {
			instance->dims = val;
	}
	
	// rows /usr/include/opencv2/core/mat.hpp:2598
	int cv_UMat_getPropRows_const(const cv::UMat* instance) {
			int ret = instance->rows;
			return ret;
	}
	
	// rows /usr/include/opencv2/core/mat.hpp:2598
	void cv_UMat_setPropRows_int(cv::UMat* instance, int val) {
			instance->rows = val;
	}
	
	// cols /usr/include/opencv2/core/mat.hpp:2601
	int cv_UMat_getPropCols_const(const cv::UMat* instance) {
			int ret = instance->cols;
			return ret;
	}
	
	// cols /usr/include/opencv2/core/mat.hpp:2601
	void cv_UMat_setPropCols_int(cv::UMat* instance, int val) {
			instance->cols = val;
	}
	
	// usageFlags /usr/include/opencv2/core/mat.hpp:2607
	void cv_UMat_getPropUsageFlags_const(const cv::UMat* instance, cv::UMatUsageFlags* ocvrs_return) {
			cv::UMatUsageFlags ret = instance->usageFlags;
			*ocvrs_return = ret;
	}
	
	// usageFlags /usr/include/opencv2/core/mat.hpp:2607
	void cv_UMat_setPropUsageFlags_UMatUsageFlags(cv::UMat* instance, cv::UMatUsageFlags val) {
			instance->usageFlags = val;
	}
	
	// u /usr/include/opencv2/core/mat.hpp:2616
	cv::UMatData** cv_UMat_getPropU(cv::UMat* instance) {
			cv::UMatData* ret = instance->u;
			return new cv::UMatData*(ret);
	}
	
	// u /usr/include/opencv2/core/mat.hpp:2616
	void cv_UMat_setPropU_UMatDataX(cv::UMat* instance, cv::UMatData* val) {
			instance->u = val;
	}
	
	// offset /usr/include/opencv2/core/mat.hpp:2619
	size_t cv_UMat_getPropOffset_const(const cv::UMat* instance) {
			size_t ret = instance->offset;
			return ret;
	}
	
	// offset /usr/include/opencv2/core/mat.hpp:2619
	void cv_UMat_setPropOffset_size_t(cv::UMat* instance, size_t val) {
			instance->offset = val;
	}
	
	// size /usr/include/opencv2/core/mat.hpp:2622
	cv::MatSize* cv_UMat_getPropSize_const(const cv::UMat* instance) {
			cv::MatSize ret = instance->size;
			return new cv::MatSize(ret);
	}
	
	// step /usr/include/opencv2/core/mat.hpp:2625
	cv::MatStep* cv_UMat_getPropStep_const(const cv::UMat* instance) {
			cv::MatStep ret = instance->step;
			return new cv::MatStep(ret);
	}
	
	void cv_UMat_delete(cv::UMat* instance) {
		delete instance;
	}
	// UMat(cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2414
	cv::UMat* cv_UMat_UMat_UMatUsageFlags(cv::UMatUsageFlags usageFlags) {
			cv::UMat* ret = new cv::UMat(usageFlags);
			return ret;
	}
	
	// UMat(int, int, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2417
	void cv_UMat_UMat_int_int_int_UMatUsageFlags(int rows, int cols, int type, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(rows, cols, type, usageFlags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// UMat(cv::Size, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2418
	void cv_UMat_UMat_Size_int_UMatUsageFlags(cv::Size* size, int type, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(*size, type, usageFlags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// UMat(int, int, int, const cv::Scalar &, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2420
	void cv_UMat_UMat_int_int_int_const_ScalarR_UMatUsageFlags(int rows, int cols, int type, const cv::Scalar* s, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(rows, cols, type, *s, usageFlags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// UMat(cv::Size, int, const cv::Scalar &, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2421
	void cv_UMat_UMat_Size_int_const_ScalarR_UMatUsageFlags(cv::Size* size, int type, const cv::Scalar* s, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(*size, type, *s, usageFlags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// UMat(int, const int *, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2424
	void cv_UMat_UMat_int_const_intX_int_UMatUsageFlags(int ndims, const int* sizes, int type, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(ndims, sizes, type, usageFlags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// UMat(int, const int *, int, const cv::Scalar &, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2425
	void cv_UMat_UMat_int_const_intX_int_const_ScalarR_UMatUsageFlags(int ndims, const int* sizes, int type, const cv::Scalar* s, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(ndims, sizes, type, *s, usageFlags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// UMat(const cv::UMat &) /usr/include/opencv2/core/mat.hpp:2428
	void cv_UMat_UMat_const_UMatR(const cv::UMat* m, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// UMat(const cv::UMat &, const cv::Range &, const cv::Range &) /usr/include/opencv2/core/mat.hpp:2431
	void cv_UMat_UMat_const_UMatR_const_RangeR_const_RangeR(const cv::UMat* m, const cv::Range* rowRange, const cv::Range* colRange, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(*m, *rowRange, *colRange);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// UMat(const cv::UMat &, const cv::Rect &) /usr/include/opencv2/core/mat.hpp:2432
	void cv_UMat_UMat_const_UMatR_const_RectR(const cv::UMat* m, const cv::Rect* roi, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(*m, *roi);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// UMat(const cv::UMat &, const std::vector<Range> &) /usr/include/opencv2/core/mat.hpp:2434
	void cv_UMat_UMat_const_UMatR_const_vector_Range_R(const cv::UMat* m, const std::vector<cv::Range>* ranges, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(*m, *ranges);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// getMat(cv::AccessFlag) /usr/include/opencv2/core/mat.hpp:2445
	void cv_UMat_getMat_const_AccessFlag(const cv::UMat* instance, cv::AccessFlag flags, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMat(flags);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// row(int) /usr/include/opencv2/core/mat.hpp:2448
	void cv_UMat_row_const_int(const cv::UMat* instance, int y, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->row(y);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// col(int) /usr/include/opencv2/core/mat.hpp:2450
	void cv_UMat_col_const_int(const cv::UMat* instance, int x, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->col(x);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// rowRange(int, int) /usr/include/opencv2/core/mat.hpp:2452
	void cv_UMat_rowRange_const_int_int(const cv::UMat* instance, int startrow, int endrow, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->rowRange(startrow, endrow);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// rowRange(const cv::Range &) /usr/include/opencv2/core/mat.hpp:2453
	void cv_UMat_rowRange_const_const_RangeR(const cv::UMat* instance, const cv::Range* r, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->rowRange(*r);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// colRange(int, int) /usr/include/opencv2/core/mat.hpp:2455
	void cv_UMat_colRange_const_int_int(const cv::UMat* instance, int startcol, int endcol, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->colRange(startcol, endcol);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// colRange(const cv::Range &) /usr/include/opencv2/core/mat.hpp:2456
	void cv_UMat_colRange_const_const_RangeR(const cv::UMat* instance, const cv::Range* r, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->colRange(*r);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// diag(int) /usr/include/opencv2/core/mat.hpp:2461
	void cv_UMat_diag_const_int(const cv::UMat* instance, int d, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->diag(d);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// diag(const cv::UMat &, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2463
	void cv_UMat_diag_const_UMatR_UMatUsageFlags(const cv::UMat* d, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::diag(*d, usageFlags);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// diag(const cv::UMat &) /usr/include/opencv2/core/mat.hpp:2464
	void cv_UMat_diag_const_UMatR(const cv::UMat* d, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::diag(*d);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// clone() /usr/include/opencv2/core/mat.hpp:2467
	void cv_UMat_clone_const(const cv::UMat* instance, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->clone();
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// copyTo(cv::OutputArray) /usr/include/opencv2/core/mat.hpp:2470
	void cv_UMat_copyTo_const_const__OutputArrayR(const cv::UMat* instance, const cv::_OutputArray* m, Result_void* ocvrs_return) {
		try {
			instance->copyTo(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// copyTo(cv::OutputArray, cv::InputArray) /usr/include/opencv2/core/mat.hpp:2472
	void cv_UMat_copyTo_const_const__OutputArrayR_const__InputArrayR(const cv::UMat* instance, const cv::_OutputArray* m, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			instance->copyTo(*m, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// convertTo(cv::OutputArray, int, double, double) /usr/include/opencv2/core/mat.hpp:2474
	void cv_UMat_convertTo_const_const__OutputArrayR_int_double_double(const cv::UMat* instance, const cv::_OutputArray* m, int rtype, double alpha, double beta, Result_void* ocvrs_return) {
		try {
			instance->convertTo(*m, rtype, alpha, beta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// assignTo(cv::UMat &, int) /usr/include/opencv2/core/mat.hpp:2476
	void cv_UMat_assignTo_const_UMatR_int(const cv::UMat* instance, cv::UMat* m, int type, Result_void* ocvrs_return) {
		try {
			instance->assignTo(*m, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setTo(cv::InputArray, cv::InputArray) /usr/include/opencv2/core/mat.hpp:2481
	void cv_UMat_setTo_const__InputArrayR_const__InputArrayR(cv::UMat* instance, const cv::_InputArray* value, const cv::_InputArray* mask, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->setTo(*value, *mask);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// reshape(int, int) /usr/include/opencv2/core/mat.hpp:2484
	void cv_UMat_reshape_const_int_int(const cv::UMat* instance, int cn, int rows, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->reshape(cn, rows);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// reshape(int, int, const int *) /usr/include/opencv2/core/mat.hpp:2485
	void cv_UMat_reshape_const_int_int_const_intX(const cv::UMat* instance, int cn, int newndims, const int* newsz, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->reshape(cn, newndims, newsz);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// t() /usr/include/opencv2/core/mat.hpp:2488
	void cv_UMat_t_const(const cv::UMat* instance, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->t();
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// inv(int) /usr/include/opencv2/core/mat.hpp:2490
	void cv_UMat_inv_const_int(const cv::UMat* instance, int method, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->inv(method);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// mul(cv::InputArray, double) /usr/include/opencv2/core/mat.hpp:2492
	void cv_UMat_mul_const_const__InputArrayR_double(const cv::UMat* instance, const cv::_InputArray* m, double scale, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->mul(*m, scale);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// dot(cv::InputArray) /usr/include/opencv2/core/mat.hpp:2495
	void cv_UMat_dot_const_const__InputArrayR(const cv::UMat* instance, const cv::_InputArray* m, Result<double>* ocvrs_return) {
		try {
			double ret = instance->dot(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// zeros(int, int, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2498
	void cv_UMat_zeros_int_int_int_UMatUsageFlags(int rows, int cols, int type, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::zeros(rows, cols, type, usageFlags);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// zeros(cv::Size, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2499
	void cv_UMat_zeros_Size_int_UMatUsageFlags(cv::Size* size, int type, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::zeros(*size, type, usageFlags);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// zeros(int, const int *, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2500
	void cv_UMat_zeros_int_const_intX_int_UMatUsageFlags(int ndims, const int* sz, int type, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::zeros(ndims, sz, type, usageFlags);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// zeros(int, int, int) /usr/include/opencv2/core/mat.hpp:2501
	void cv_UMat_zeros_int_int_int(int rows, int cols, int type, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::zeros(rows, cols, type);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// zeros(cv::Size, int) /usr/include/opencv2/core/mat.hpp:2502
	void cv_UMat_zeros_Size_int(cv::Size* size, int type, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::zeros(*size, type);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// zeros(int, const int *, int) /usr/include/opencv2/core/mat.hpp:2503
	void cv_UMat_zeros_int_const_intX_int(int ndims, const int* sz, int type, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::zeros(ndims, sz, type);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// ones(int, int, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2504
	void cv_UMat_ones_int_int_int_UMatUsageFlags(int rows, int cols, int type, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::ones(rows, cols, type, usageFlags);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// ones(cv::Size, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2505
	void cv_UMat_ones_Size_int_UMatUsageFlags(cv::Size* size, int type, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::ones(*size, type, usageFlags);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// ones(int, const int *, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2506
	void cv_UMat_ones_int_const_intX_int_UMatUsageFlags(int ndims, const int* sz, int type, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::ones(ndims, sz, type, usageFlags);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// ones(int, int, int) /usr/include/opencv2/core/mat.hpp:2507
	void cv_UMat_ones_int_int_int(int rows, int cols, int type, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::ones(rows, cols, type);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// ones(cv::Size, int) /usr/include/opencv2/core/mat.hpp:2508
	void cv_UMat_ones_Size_int(cv::Size* size, int type, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::ones(*size, type);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// ones(int, const int *, int) /usr/include/opencv2/core/mat.hpp:2509
	void cv_UMat_ones_int_const_intX_int(int ndims, const int* sz, int type, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::ones(ndims, sz, type);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// eye(int, int, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2510
	void cv_UMat_eye_int_int_int_UMatUsageFlags(int rows, int cols, int type, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::eye(rows, cols, type, usageFlags);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// eye(cv::Size, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2511
	void cv_UMat_eye_Size_int_UMatUsageFlags(cv::Size* size, int type, cv::UMatUsageFlags usageFlags, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::eye(*size, type, usageFlags);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// eye(int, int, int) /usr/include/opencv2/core/mat.hpp:2512
	void cv_UMat_eye_int_int_int(int rows, int cols, int type, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::eye(rows, cols, type);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// eye(cv::Size, int) /usr/include/opencv2/core/mat.hpp:2513
	void cv_UMat_eye_Size_int(cv::Size* size, int type, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = cv::UMat::eye(*size, type);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// create(int, int, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2517
	void cv_UMat_create_int_int_int_UMatUsageFlags(cv::UMat* instance, int rows, int cols, int type, cv::UMatUsageFlags usageFlags, Result_void* ocvrs_return) {
		try {
			instance->create(rows, cols, type, usageFlags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(cv::Size, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2518
	void cv_UMat_create_Size_int_UMatUsageFlags(cv::UMat* instance, cv::Size* size, int type, cv::UMatUsageFlags usageFlags, Result_void* ocvrs_return) {
		try {
			instance->create(*size, type, usageFlags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(int, const int *, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2519
	void cv_UMat_create_int_const_intX_int_UMatUsageFlags(cv::UMat* instance, int ndims, const int* sizes, int type, cv::UMatUsageFlags usageFlags, Result_void* ocvrs_return) {
		try {
			instance->create(ndims, sizes, type, usageFlags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(const std::vector<int> &, int, cv::UMatUsageFlags) /usr/include/opencv2/core/mat.hpp:2520
	void cv_UMat_create_const_vector_int_R_int_UMatUsageFlags(cv::UMat* instance, const std::vector<int>* sizes, int type, cv::UMatUsageFlags usageFlags, Result_void* ocvrs_return) {
		try {
			instance->create(*sizes, type, usageFlags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// addref() /usr/include/opencv2/core/mat.hpp:2523
	void cv_UMat_addref(cv::UMat* instance, Result_void* ocvrs_return) {
		try {
			instance->addref();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// release() /usr/include/opencv2/core/mat.hpp:2526
	void cv_UMat_release(cv::UMat* instance, Result_void* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// deallocate() /usr/include/opencv2/core/mat.hpp:2529
	void cv_UMat_deallocate(cv::UMat* instance, Result_void* ocvrs_return) {
		try {
			instance->deallocate();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// locateROI(cv::Size &, cv::Point &) /usr/include/opencv2/core/mat.hpp:2534
	void cv_UMat_locateROI_const_SizeR_PointR(const cv::UMat* instance, cv::Size* wholeSize, cv::Point* ofs, Result_void* ocvrs_return) {
		try {
			instance->locateROI(*wholeSize, *ofs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// adjustROI(int, int, int, int) /usr/include/opencv2/core/mat.hpp:2536
	void cv_UMat_adjustROI_int_int_int_int(cv::UMat* instance, int dtop, int dbottom, int dleft, int dright, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->adjustROI(dtop, dbottom, dleft, dright);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// isContinuous() /usr/include/opencv2/core/mat.hpp:2547
	bool cv_UMat_isContinuous_const(const cv::UMat* instance) {
			bool ret = instance->isContinuous();
			return ret;
	}
	
	// isSubmatrix() /usr/include/opencv2/core/mat.hpp:2550
	bool cv_UMat_isSubmatrix_const(const cv::UMat* instance) {
			bool ret = instance->isSubmatrix();
			return ret;
	}
	
	// elemSize() /usr/include/opencv2/core/mat.hpp:2554
	void cv_UMat_elemSize_const(const cv::UMat* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->elemSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// elemSize1() /usr/include/opencv2/core/mat.hpp:2556
	size_t cv_UMat_elemSize1_const(const cv::UMat* instance) {
			size_t ret = instance->elemSize1();
			return ret;
	}
	
	// type() /usr/include/opencv2/core/mat.hpp:2558
	int cv_UMat_type_const(const cv::UMat* instance) {
			int ret = instance->type();
			return ret;
	}
	
	// depth() /usr/include/opencv2/core/mat.hpp:2560
	int cv_UMat_depth_const(const cv::UMat* instance) {
			int ret = instance->depth();
			return ret;
	}
	
	// channels() /usr/include/opencv2/core/mat.hpp:2562
	int cv_UMat_channels_const(const cv::UMat* instance) {
			int ret = instance->channels();
			return ret;
	}
	
	// step1(int) /usr/include/opencv2/core/mat.hpp:2564
	void cv_UMat_step1_const_int(const cv::UMat* instance, int i, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->step1(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// empty() /usr/include/opencv2/core/mat.hpp:2566
	bool cv_UMat_empty_const(const cv::UMat* instance) {
			bool ret = instance->empty();
			return ret;
	}
	
	// total() /usr/include/opencv2/core/mat.hpp:2568
	size_t cv_UMat_total_const(const cv::UMat* instance) {
			size_t ret = instance->total();
			return ret;
	}
	
	// checkVector(int, int, bool) /usr/include/opencv2/core/mat.hpp:2571
	void cv_UMat_checkVector_const_int_int_bool(const cv::UMat* instance, int elemChannels, int depth, bool requireContinuous, Result<int>* ocvrs_return) {
		try {
			int ret = instance->checkVector(elemChannels, depth, requireContinuous);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// UMat(cv::UMat &&) /usr/include/opencv2/core/mat.hpp:2573
	void cv_UMat_UMat_UMatR(cv::UMat* m, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat* ret = new cv::UMat(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// handle(cv::AccessFlag) /usr/include/opencv2/core/mat.hpp:2580
	void cv_UMat_handle_const_AccessFlag(const cv::UMat* instance, cv::AccessFlag accessFlags, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->handle(accessFlags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// ndoffset(size_t *) /usr/include/opencv2/core/mat.hpp:2581
	void cv_UMat_ndoffset_const_size_tX(const cv::UMat* instance, size_t* ofs, Result_void* ocvrs_return) {
		try {
			instance->ndoffset(ofs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// updateContinuityFlag() /usr/include/opencv2/core/mat.hpp:2613
	void cv_UMat_updateContinuityFlag(cv::UMat* instance, Result_void* ocvrs_return) {
		try {
			instance->updateContinuityFlag();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// urefcount /usr/include/opencv2/core/mat.hpp:569
	int cv_UMatData_getPropUrefcount_const(const cv::UMatData* instance) {
			int ret = instance->urefcount;
			return ret;
	}
	
	// urefcount /usr/include/opencv2/core/mat.hpp:569
	void cv_UMatData_setPropUrefcount_int(cv::UMatData* instance, int val) {
			instance->urefcount = val;
	}
	
	// refcount /usr/include/opencv2/core/mat.hpp:570
	int cv_UMatData_getPropRefcount_const(const cv::UMatData* instance) {
			int ret = instance->refcount;
			return ret;
	}
	
	// refcount /usr/include/opencv2/core/mat.hpp:570
	void cv_UMatData_setPropRefcount_int(cv::UMatData* instance, int val) {
			instance->refcount = val;
	}
	
	// data /usr/include/opencv2/core/mat.hpp:571
	unsigned char* cv_UMatData_getPropData(cv::UMatData* instance) {
			unsigned char* ret = instance->data;
			return ret;
	}
	
	// data /usr/include/opencv2/core/mat.hpp:571
	void cv_UMatData_setPropData_unsigned_charX(cv::UMatData* instance, unsigned char* val) {
			instance->data = val;
	}
	
	// origdata /usr/include/opencv2/core/mat.hpp:572
	unsigned char* cv_UMatData_getPropOrigdata(cv::UMatData* instance) {
			unsigned char* ret = instance->origdata;
			return ret;
	}
	
	// origdata /usr/include/opencv2/core/mat.hpp:572
	void cv_UMatData_setPropOrigdata_unsigned_charX(cv::UMatData* instance, unsigned char* val) {
			instance->origdata = val;
	}
	
	// size /usr/include/opencv2/core/mat.hpp:573
	size_t cv_UMatData_getPropSize_const(const cv::UMatData* instance) {
			size_t ret = instance->size;
			return ret;
	}
	
	// size /usr/include/opencv2/core/mat.hpp:573
	void cv_UMatData_setPropSize_size_t(cv::UMatData* instance, size_t val) {
			instance->size = val;
	}
	
	// flags /usr/include/opencv2/core/mat.hpp:575
	void cv_UMatData_getPropFlags_const(const cv::UMatData* instance, cv::UMatData::MemoryFlag* ocvrs_return) {
			cv::UMatData::MemoryFlag ret = instance->flags;
			*ocvrs_return = ret;
	}
	
	// flags /usr/include/opencv2/core/mat.hpp:575
	void cv_UMatData_setPropFlags_MemoryFlag(cv::UMatData* instance, cv::UMatData::MemoryFlag val) {
			instance->flags = val;
	}
	
	// handle /usr/include/opencv2/core/mat.hpp:576
	void* cv_UMatData_getPropHandle(cv::UMatData* instance) {
			void* ret = instance->handle;
			return ret;
	}
	
	// handle /usr/include/opencv2/core/mat.hpp:576
	void cv_UMatData_setPropHandle_voidX(cv::UMatData* instance, void* val) {
			instance->handle = val;
	}
	
	// userdata /usr/include/opencv2/core/mat.hpp:577
	void* cv_UMatData_getPropUserdata(cv::UMatData* instance) {
			void* ret = instance->userdata;
			return ret;
	}
	
	// userdata /usr/include/opencv2/core/mat.hpp:577
	void cv_UMatData_setPropUserdata_voidX(cv::UMatData* instance, void* val) {
			instance->userdata = val;
	}
	
	// allocatorFlags_ /usr/include/opencv2/core/mat.hpp:578
	int cv_UMatData_getPropAllocatorFlags__const(const cv::UMatData* instance) {
			int ret = instance->allocatorFlags_;
			return ret;
	}
	
	// allocatorFlags_ /usr/include/opencv2/core/mat.hpp:578
	void cv_UMatData_setPropAllocatorFlags__int(cv::UMatData* instance, int val) {
			instance->allocatorFlags_ = val;
	}
	
	// mapcount /usr/include/opencv2/core/mat.hpp:579
	int cv_UMatData_getPropMapcount_const(const cv::UMatData* instance) {
			int ret = instance->mapcount;
			return ret;
	}
	
	// mapcount /usr/include/opencv2/core/mat.hpp:579
	void cv_UMatData_setPropMapcount_int(cv::UMatData* instance, int val) {
			instance->mapcount = val;
	}
	
	// originalUMatData /usr/include/opencv2/core/mat.hpp:580
	cv::UMatData** cv_UMatData_getPropOriginalUMatData(cv::UMatData* instance) {
			cv::UMatData* ret = instance->originalUMatData;
			return new cv::UMatData*(ret);
	}
	
	// originalUMatData /usr/include/opencv2/core/mat.hpp:580
	void cv_UMatData_setPropOriginalUMatData_UMatDataX(cv::UMatData* instance, cv::UMatData* val) {
			instance->originalUMatData = val;
	}
	
	void cv_UMatData_delete(cv::UMatData* instance) {
		delete instance;
	}
	// lock() /usr/include/opencv2/core/mat.hpp:554
	void cv_UMatData_lock(cv::UMatData* instance, Result_void* ocvrs_return) {
		try {
			instance->lock();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// unlock() /usr/include/opencv2/core/mat.hpp:555
	void cv_UMatData_unlock(cv::UMatData* instance, Result_void* ocvrs_return) {
		try {
			instance->unlock();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// hostCopyObsolete() /usr/include/opencv2/core/mat.hpp:557
	void cv_UMatData_hostCopyObsolete_const(const cv::UMatData* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->hostCopyObsolete();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// deviceCopyObsolete() /usr/include/opencv2/core/mat.hpp:558
	void cv_UMatData_deviceCopyObsolete_const(const cv::UMatData* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->deviceCopyObsolete();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// deviceMemMapped() /usr/include/opencv2/core/mat.hpp:559
	void cv_UMatData_deviceMemMapped_const(const cv::UMatData* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->deviceMemMapped();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// copyOnMap() /usr/include/opencv2/core/mat.hpp:560
	void cv_UMatData_copyOnMap_const(const cv::UMatData* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->copyOnMap();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// tempUMat() /usr/include/opencv2/core/mat.hpp:561
	void cv_UMatData_tempUMat_const(const cv::UMatData* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->tempUMat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// tempCopiedUMat() /usr/include/opencv2/core/mat.hpp:562
	void cv_UMatData_tempCopiedUMat_const(const cv::UMatData* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->tempCopiedUMat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// markHostCopyObsolete(bool) /usr/include/opencv2/core/mat.hpp:563
	void cv_UMatData_markHostCopyObsolete_bool(cv::UMatData* instance, bool flag, Result_void* ocvrs_return) {
		try {
			instance->markHostCopyObsolete(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// markDeviceCopyObsolete(bool) /usr/include/opencv2/core/mat.hpp:564
	void cv_UMatData_markDeviceCopyObsolete_bool(cv::UMatData* instance, bool flag, Result_void* ocvrs_return) {
		try {
			instance->markDeviceCopyObsolete(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// markDeviceMemMapped(bool) /usr/include/opencv2/core/mat.hpp:565
	void cv_UMatData_markDeviceMemMapped_bool(cv::UMatData* instance, bool flag, Result_void* ocvrs_return) {
		try {
			instance->markDeviceMemMapped(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv__InputArray_delete(cv::_InputArray* instance) {
		delete instance;
	}
	// _InputArray() /usr/include/opencv2/core/mat.hpp:189
	void cv__InputArray__InputArray(Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray* ret = new cv::_InputArray();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	// _InputArray(int, void *) /usr/include/opencv2/core/mat.hpp:190
	void cv__InputArray__InputArray_int_voidX(int _flags, void* _obj, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(_flags, _obj);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	// _InputArray(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:191
	void cv__InputArray__InputArray_const_MatR(const cv::Mat* m, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	// _InputArray(const cv::MatExpr &) /usr/include/opencv2/core/mat.hpp:192
	void cv__InputArray__InputArray_const_MatExprR(const cv::MatExpr* expr, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*expr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	// _InputArray(const std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:193
	void cv__InputArray__InputArray_const_vector_Mat_R(const std::vector<cv::Mat>* vec, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	// _InputArray(const std::vector<bool> &) /usr/include/opencv2/core/mat.hpp:196
	void cv__InputArray__InputArray_const_vector_bool_R(const std::vector<bool>* vec, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	// _InputArray(const double &) /usr/include/opencv2/core/mat.hpp:202
	void cv__InputArray__InputArray_const_doubleR(const double* val, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*val);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	// _InputArray(const cuda::GpuMat &) /usr/include/opencv2/core/mat.hpp:203
	void cv__InputArray__InputArray_const_GpuMatR(const cv::cuda::GpuMat* d_mat, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*d_mat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	// _InputArray(const std::vector<cuda::GpuMat> &) /usr/include/opencv2/core/mat.hpp:204
	void cv__InputArray__InputArray_const_vector_GpuMat_R(const std::vector<cv::cuda::GpuMat>* d_mat_array, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*d_mat_array);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	// _InputArray(const ogl::Buffer &) /usr/include/opencv2/core/mat.hpp:205
	void cv__InputArray__InputArray_const_BufferR(const cv::ogl::Buffer* buf, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*buf);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	// _InputArray(const cuda::HostMem &) /usr/include/opencv2/core/mat.hpp:206
	void cv__InputArray__InputArray_const_HostMemR(const cv::cuda::HostMem* cuda_mem, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*cuda_mem);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	// _InputArray(const cv::UMat &) /usr/include/opencv2/core/mat.hpp:208
	void cv__InputArray__InputArray_const_UMatR(const cv::UMat* um, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*um);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	// _InputArray(const std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:209
	void cv__InputArray__InputArray_const_vector_UMat_R(const std::vector<cv::UMat>* umv, Result<cv::_InputArray*>* ocvrs_return) {
		try {
			cv::_InputArray* ret = new cv::_InputArray(*umv);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray*>))
	}
	
	// getMat(int) /usr/include/opencv2/core/mat.hpp:217
	void cv__InputArray_getMat_const_int(const cv::_InputArray* instance, int idx, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMat(idx);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getMat_(int) /usr/include/opencv2/core/mat.hpp:218
	void cv__InputArray_getMat__const_int(const cv::_InputArray* instance, int idx, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMat_(idx);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getUMat(int) /usr/include/opencv2/core/mat.hpp:219
	void cv__InputArray_getUMat_const_int(const cv::_InputArray* instance, int idx, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->getUMat(idx);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// getMatVector(std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:220
	void cv__InputArray_getMatVector_const_vector_Mat_R(const cv::_InputArray* instance, std::vector<cv::Mat>* mv, Result_void* ocvrs_return) {
		try {
			instance->getMatVector(*mv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getUMatVector(std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:221
	void cv__InputArray_getUMatVector_const_vector_UMat_R(const cv::_InputArray* instance, std::vector<cv::UMat>* umv, Result_void* ocvrs_return) {
		try {
			instance->getUMatVector(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getGpuMatVector(std::vector<cuda::GpuMat> &) /usr/include/opencv2/core/mat.hpp:222
	void cv__InputArray_getGpuMatVector_const_vector_GpuMat_R(const cv::_InputArray* instance, std::vector<cv::cuda::GpuMat>* gpumv, Result_void* ocvrs_return) {
		try {
			instance->getGpuMatVector(*gpumv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getGpuMat() /usr/include/opencv2/core/mat.hpp:223
	void cv__InputArray_getGpuMat_const(const cv::_InputArray* instance, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->getGpuMat();
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// getOGlBuffer() /usr/include/opencv2/core/mat.hpp:224
	void cv__InputArray_getOGlBuffer_const(const cv::_InputArray* instance, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer ret = instance->getOGlBuffer();
			Ok(new cv::ogl::Buffer(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ogl::Buffer*>))
	}
	
	// getFlags() /usr/include/opencv2/core/mat.hpp:226
	void cv__InputArray_getFlags_const(const cv::_InputArray* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFlags();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getObj() /usr/include/opencv2/core/mat.hpp:227
	void cv__InputArray_getObj_const(const cv::_InputArray* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->getObj();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// getSz() /usr/include/opencv2/core/mat.hpp:228
	void cv__InputArray_getSz_const(const cv::_InputArray* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getSz();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// kind() /usr/include/opencv2/core/mat.hpp:230
	void cv__InputArray_kind_const(const cv::_InputArray* instance, Result<cv::_InputArray::KindFlag>* ocvrs_return) {
		try {
			cv::_InputArray::KindFlag ret = instance->kind();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputArray::KindFlag>))
	}
	
	// dims(int) /usr/include/opencv2/core/mat.hpp:231
	void cv__InputArray_dims_const_int(const cv::_InputArray* instance, int i, Result<int>* ocvrs_return) {
		try {
			int ret = instance->dims(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// cols(int) /usr/include/opencv2/core/mat.hpp:232
	void cv__InputArray_cols_const_int(const cv::_InputArray* instance, int i, Result<int>* ocvrs_return) {
		try {
			int ret = instance->cols(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// rows(int) /usr/include/opencv2/core/mat.hpp:233
	void cv__InputArray_rows_const_int(const cv::_InputArray* instance, int i, Result<int>* ocvrs_return) {
		try {
			int ret = instance->rows(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// size(int) /usr/include/opencv2/core/mat.hpp:234
	void cv__InputArray_size_const_int(const cv::_InputArray* instance, int i, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->size(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// sizend(int *, int) /usr/include/opencv2/core/mat.hpp:235
	void cv__InputArray_sizend_const_intX_int(const cv::_InputArray* instance, int* sz, int i, Result<int>* ocvrs_return) {
		try {
			int ret = instance->sizend(sz, i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// sameSize(const cv::_InputArray &) /usr/include/opencv2/core/mat.hpp:236
	void cv__InputArray_sameSize_const_const__InputArrayR(const cv::_InputArray* instance, const cv::_InputArray* arr, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->sameSize(*arr);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// total(int) /usr/include/opencv2/core/mat.hpp:237
	void cv__InputArray_total_const_int(const cv::_InputArray* instance, int i, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->total(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// type(int) /usr/include/opencv2/core/mat.hpp:238
	void cv__InputArray_type_const_int(const cv::_InputArray* instance, int i, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// depth(int) /usr/include/opencv2/core/mat.hpp:239
	void cv__InputArray_depth_const_int(const cv::_InputArray* instance, int i, Result<int>* ocvrs_return) {
		try {
			int ret = instance->depth(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// channels(int) /usr/include/opencv2/core/mat.hpp:240
	void cv__InputArray_channels_const_int(const cv::_InputArray* instance, int i, Result<int>* ocvrs_return) {
		try {
			int ret = instance->channels(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// isContinuous(int) /usr/include/opencv2/core/mat.hpp:241
	void cv__InputArray_isContinuous_const_int(const cv::_InputArray* instance, int i, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isContinuous(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isSubmatrix(int) /usr/include/opencv2/core/mat.hpp:242
	void cv__InputArray_isSubmatrix_const_int(const cv::_InputArray* instance, int i, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isSubmatrix(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// empty() /usr/include/opencv2/core/mat.hpp:243
	void cv__InputArray_empty_const(const cv::_InputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// copyTo(const cv::_OutputArray &) /usr/include/opencv2/core/mat.hpp:244
	void cv__InputArray_copyTo_const_const__OutputArrayR(const cv::_InputArray* instance, const cv::_OutputArray* arr, Result_void* ocvrs_return) {
		try {
			instance->copyTo(*arr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// copyTo(const cv::_OutputArray &, const cv::_InputArray &) /usr/include/opencv2/core/mat.hpp:245
	void cv__InputArray_copyTo_const_const__OutputArrayR_const__InputArrayR(const cv::_InputArray* instance, const cv::_OutputArray* arr, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			instance->copyTo(*arr, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// offset(int) /usr/include/opencv2/core/mat.hpp:246
	void cv__InputArray_offset_const_int(const cv::_InputArray* instance, int i, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->offset(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// step(int) /usr/include/opencv2/core/mat.hpp:247
	void cv__InputArray_step_const_int(const cv::_InputArray* instance, int i, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->step(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// isMat() /usr/include/opencv2/core/mat.hpp:248
	void cv__InputArray_isMat_const(const cv::_InputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isMat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isUMat() /usr/include/opencv2/core/mat.hpp:249
	void cv__InputArray_isUMat_const(const cv::_InputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isUMat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isMatVector() /usr/include/opencv2/core/mat.hpp:250
	void cv__InputArray_isMatVector_const(const cv::_InputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isMatVector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isUMatVector() /usr/include/opencv2/core/mat.hpp:251
	void cv__InputArray_isUMatVector_const(const cv::_InputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isUMatVector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isMatx() /usr/include/opencv2/core/mat.hpp:252
	void cv__InputArray_isMatx_const(const cv::_InputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isMatx();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isVector() /usr/include/opencv2/core/mat.hpp:253
	void cv__InputArray_isVector_const(const cv::_InputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isVector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isGpuMat() /usr/include/opencv2/core/mat.hpp:254
	void cv__InputArray_isGpuMat_const(const cv::_InputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isGpuMat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isGpuMatVector() /usr/include/opencv2/core/mat.hpp:255
	void cv__InputArray_isGpuMatVector_const(const cv::_InputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isGpuMatVector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	cv::_InputArray* cv__InputOutputArray_to__InputArray(cv::_InputOutputArray* instance) {
		return dynamic_cast<cv::_InputArray*>(instance);
	}
	
	cv::_OutputArray* cv__InputOutputArray_to__OutputArray(cv::_InputOutputArray* instance) {
		return dynamic_cast<cv::_OutputArray*>(instance);
	}
	
	void cv__InputOutputArray_delete(cv::_InputOutputArray* instance) {
		delete instance;
	}
	// _InputOutputArray() /usr/include/opencv2/core/mat.hpp:388
	void cv__InputOutputArray__InputOutputArray(Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	// _InputOutputArray(int, void *) /usr/include/opencv2/core/mat.hpp:389
	void cv__InputOutputArray__InputOutputArray_int_voidX(int _flags, void* _obj, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(_flags, _obj);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	// _InputOutputArray(cv::Mat &) /usr/include/opencv2/core/mat.hpp:390
	void cv__InputOutputArray__InputOutputArray_MatR(cv::Mat* m, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	// _InputOutputArray(std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:391
	void cv__InputOutputArray__InputOutputArray_vector_Mat_R(std::vector<cv::Mat>* vec, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	// _InputOutputArray(cuda::GpuMat &) /usr/include/opencv2/core/mat.hpp:392
	void cv__InputOutputArray__InputOutputArray_GpuMatR(cv::cuda::GpuMat* d_mat, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*d_mat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	// _InputOutputArray(ogl::Buffer &) /usr/include/opencv2/core/mat.hpp:393
	void cv__InputOutputArray__InputOutputArray_BufferR(cv::ogl::Buffer* buf, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*buf);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	// _InputOutputArray(cuda::HostMem &) /usr/include/opencv2/core/mat.hpp:394
	void cv__InputOutputArray__InputOutputArray_HostMemR(cv::cuda::HostMem* cuda_mem, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*cuda_mem);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	// _InputOutputArray(cv::UMat &) /usr/include/opencv2/core/mat.hpp:403
	void cv__InputOutputArray__InputOutputArray_UMatR(cv::UMat* m, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	// _InputOutputArray(std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:404
	void cv__InputOutputArray__InputOutputArray_vector_UMat_R(std::vector<cv::UMat>* vec, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	// _InputOutputArray(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:406
	void cv__InputOutputArray__InputOutputArray_const_MatR(const cv::Mat* m, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	// _InputOutputArray(const std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:407
	void cv__InputOutputArray__InputOutputArray_const_vector_Mat_R(const std::vector<cv::Mat>* vec, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	// _InputOutputArray(const cuda::GpuMat &) /usr/include/opencv2/core/mat.hpp:408
	void cv__InputOutputArray__InputOutputArray_const_GpuMatR(const cv::cuda::GpuMat* d_mat, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*d_mat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	// _InputOutputArray(const std::vector<cuda::GpuMat> &) /usr/include/opencv2/core/mat.hpp:409
	void cv__InputOutputArray__InputOutputArray_const_vector_GpuMat_R(const std::vector<cv::cuda::GpuMat>* d_mat, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*d_mat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	// _InputOutputArray(const ogl::Buffer &) /usr/include/opencv2/core/mat.hpp:410
	void cv__InputOutputArray__InputOutputArray_const_BufferR(const cv::ogl::Buffer* buf, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*buf);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	// _InputOutputArray(const cuda::HostMem &) /usr/include/opencv2/core/mat.hpp:411
	void cv__InputOutputArray__InputOutputArray_const_HostMemR(const cv::cuda::HostMem* cuda_mem, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*cuda_mem);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	// _InputOutputArray(const cv::UMat &) /usr/include/opencv2/core/mat.hpp:419
	void cv__InputOutputArray__InputOutputArray_const_UMatR(const cv::UMat* m, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	// _InputOutputArray(const std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:420
	void cv__InputOutputArray__InputOutputArray_const_vector_UMat_R(const std::vector<cv::UMat>* vec, Result<cv::_InputOutputArray*>* ocvrs_return) {
		try {
			cv::_InputOutputArray* ret = new cv::_InputOutputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_InputOutputArray*>))
	}
	
	cv::_InputArray* cv__OutputArray_to__InputArray(cv::_OutputArray* instance) {
		return dynamic_cast<cv::_InputArray*>(instance);
	}
	
	void cv__OutputArray_delete(cv::_OutputArray* instance) {
		delete instance;
	}
	// _OutputArray() /usr/include/opencv2/core/mat.hpp:313
	void cv__OutputArray__OutputArray(Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	// _OutputArray(int, void *) /usr/include/opencv2/core/mat.hpp:314
	void cv__OutputArray__OutputArray_int_voidX(int _flags, void* _obj, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(_flags, _obj);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	// _OutputArray(cv::Mat &) /usr/include/opencv2/core/mat.hpp:315
	void cv__OutputArray__OutputArray_MatR(cv::Mat* m, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	// _OutputArray(std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:316
	void cv__OutputArray__OutputArray_vector_Mat_R(std::vector<cv::Mat>* vec, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	// _OutputArray(cuda::GpuMat &) /usr/include/opencv2/core/mat.hpp:317
	void cv__OutputArray__OutputArray_GpuMatR(cv::cuda::GpuMat* d_mat, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*d_mat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	// _OutputArray(std::vector<cuda::GpuMat> &) /usr/include/opencv2/core/mat.hpp:318
	void cv__OutputArray__OutputArray_vector_GpuMat_R(std::vector<cv::cuda::GpuMat>* d_mat, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*d_mat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	// _OutputArray(ogl::Buffer &) /usr/include/opencv2/core/mat.hpp:319
	void cv__OutputArray__OutputArray_BufferR(cv::ogl::Buffer* buf, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*buf);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	// _OutputArray(cuda::HostMem &) /usr/include/opencv2/core/mat.hpp:320
	void cv__OutputArray__OutputArray_HostMemR(cv::cuda::HostMem* cuda_mem, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*cuda_mem);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	// _OutputArray(cv::UMat &) /usr/include/opencv2/core/mat.hpp:330
	void cv__OutputArray__OutputArray_UMatR(cv::UMat* m, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	// _OutputArray(std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:331
	void cv__OutputArray__OutputArray_vector_UMat_R(std::vector<cv::UMat>* vec, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	// _OutputArray(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:333
	void cv__OutputArray__OutputArray_const_MatR(const cv::Mat* m, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	// _OutputArray(const std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:334
	void cv__OutputArray__OutputArray_const_vector_Mat_R(const std::vector<cv::Mat>* vec, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	// _OutputArray(const cuda::GpuMat &) /usr/include/opencv2/core/mat.hpp:335
	void cv__OutputArray__OutputArray_const_GpuMatR(const cv::cuda::GpuMat* d_mat, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*d_mat);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	// _OutputArray(const ogl::Buffer &) /usr/include/opencv2/core/mat.hpp:337
	void cv__OutputArray__OutputArray_const_BufferR(const cv::ogl::Buffer* buf, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*buf);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	// _OutputArray(const cuda::HostMem &) /usr/include/opencv2/core/mat.hpp:338
	void cv__OutputArray__OutputArray_const_HostMemR(const cv::cuda::HostMem* cuda_mem, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*cuda_mem);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	// _OutputArray(const cv::UMat &) /usr/include/opencv2/core/mat.hpp:346
	void cv__OutputArray__OutputArray_const_UMatR(const cv::UMat* m, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	// _OutputArray(const std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:347
	void cv__OutputArray__OutputArray_const_vector_UMat_R(const std::vector<cv::UMat>* vec, Result<cv::_OutputArray*>* ocvrs_return) {
		try {
			cv::_OutputArray* ret = new cv::_OutputArray(*vec);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::_OutputArray*>))
	}
	
	// fixedSize() /usr/include/opencv2/core/mat.hpp:357
	void cv__OutputArray_fixedSize_const(const cv::_OutputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->fixedSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// fixedType() /usr/include/opencv2/core/mat.hpp:358
	void cv__OutputArray_fixedType_const(const cv::_OutputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->fixedType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// needed() /usr/include/opencv2/core/mat.hpp:359
	void cv__OutputArray_needed_const(const cv::_OutputArray* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->needed();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// getMatRef(int) /usr/include/opencv2/core/mat.hpp:360
	void cv__OutputArray_getMatRef_const_int(const cv::_OutputArray* instance, int i, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getMatRef(i);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getUMatRef(int) /usr/include/opencv2/core/mat.hpp:361
	void cv__OutputArray_getUMatRef_const_int(const cv::_OutputArray* instance, int i, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->getUMatRef(i);
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// getGpuMatRef() /usr/include/opencv2/core/mat.hpp:362
	void cv__OutputArray_getGpuMatRef_const(const cv::_OutputArray* instance, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->getGpuMatRef();
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// getGpuMatVecRef() /usr/include/opencv2/core/mat.hpp:363
	void cv__OutputArray_getGpuMatVecRef_const(const cv::_OutputArray* instance, Result<std::vector<cv::cuda::GpuMat>*>* ocvrs_return) {
		try {
			std::vector<cv::cuda::GpuMat> ret = instance->getGpuMatVecRef();
			Ok(new std::vector<cv::cuda::GpuMat>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::cuda::GpuMat>*>))
	}
	
	// getOGlBufferRef() /usr/include/opencv2/core/mat.hpp:364
	void cv__OutputArray_getOGlBufferRef_const(const cv::_OutputArray* instance, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer ret = instance->getOGlBufferRef();
			Ok(new cv::ogl::Buffer(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ogl::Buffer*>))
	}
	
	// getHostMemRef() /usr/include/opencv2/core/mat.hpp:365
	void cv__OutputArray_getHostMemRef_const(const cv::_OutputArray* instance, Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem ret = instance->getHostMemRef();
			Ok(new cv::cuda::HostMem(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::HostMem*>))
	}
	
	// create(cv::Size, int, int, bool, _OutputArray::DepthMask) /usr/include/opencv2/core/mat.hpp:366
	void cv__OutputArray_create_const_Size_int_int_bool_DepthMask(const cv::_OutputArray* instance, cv::Size* sz, int type, int i, bool allowTransposed, cv::_OutputArray::DepthMask fixedDepthMask, Result_void* ocvrs_return) {
		try {
			instance->create(*sz, type, i, allowTransposed, fixedDepthMask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(int, int, int, int, bool, _OutputArray::DepthMask) /usr/include/opencv2/core/mat.hpp:367
	void cv__OutputArray_create_const_int_int_int_int_bool_DepthMask(const cv::_OutputArray* instance, int rows, int cols, int type, int i, bool allowTransposed, cv::_OutputArray::DepthMask fixedDepthMask, Result_void* ocvrs_return) {
		try {
			instance->create(rows, cols, type, i, allowTransposed, fixedDepthMask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(int, const int *, int, int, bool, _OutputArray::DepthMask) /usr/include/opencv2/core/mat.hpp:368
	void cv__OutputArray_create_const_int_const_intX_int_int_bool_DepthMask(const cv::_OutputArray* instance, int dims, const int* size, int type, int i, bool allowTransposed, cv::_OutputArray::DepthMask fixedDepthMask, Result_void* ocvrs_return) {
		try {
			instance->create(dims, size, type, i, allowTransposed, fixedDepthMask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// createSameSize(const cv::_InputArray &, int) /usr/include/opencv2/core/mat.hpp:369
	void cv__OutputArray_createSameSize_const_const__InputArrayR_int(const cv::_OutputArray* instance, const cv::_InputArray* arr, int mtype, Result_void* ocvrs_return) {
		try {
			instance->createSameSize(*arr, mtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// release() /usr/include/opencv2/core/mat.hpp:370
	void cv__OutputArray_release_const(const cv::_OutputArray* instance, Result_void* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// clear() /usr/include/opencv2/core/mat.hpp:371
	void cv__OutputArray_clear_const(const cv::_OutputArray* instance, Result_void* ocvrs_return) {
		try {
			instance->clear();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setTo(const cv::_InputArray &, const cv::_InputArray &) /usr/include/opencv2/core/mat.hpp:372
	void cv__OutputArray_setTo_const_const__InputArrayR_const__InputArrayR(const cv::_OutputArray* instance, const cv::_InputArray* value, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			instance->setTo(*value, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// assign(const cv::UMat &) /usr/include/opencv2/core/mat.hpp:374
	void cv__OutputArray_assign_const_const_UMatR(const cv::_OutputArray* instance, const cv::UMat* u, Result_void* ocvrs_return) {
		try {
			instance->assign(*u);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// assign(const cv::Mat &) /usr/include/opencv2/core/mat.hpp:375
	void cv__OutputArray_assign_const_const_MatR(const cv::_OutputArray* instance, const cv::Mat* m, Result_void* ocvrs_return) {
		try {
			instance->assign(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// assign(const std::vector<UMat> &) /usr/include/opencv2/core/mat.hpp:377
	void cv__OutputArray_assign_const_const_vector_UMat_R(const cv::_OutputArray* instance, const std::vector<cv::UMat>* v, Result_void* ocvrs_return) {
		try {
			instance->assign(*v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// assign(const std::vector<Mat> &) /usr/include/opencv2/core/mat.hpp:378
	void cv__OutputArray_assign_const_const_vector_Mat_R(const cv::_OutputArray* instance, const std::vector<cv::Mat>* v, Result_void* ocvrs_return) {
		try {
			instance->assign(*v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// move(cv::UMat &) /usr/include/opencv2/core/mat.hpp:380
	void cv__OutputArray_move_const_UMatR(const cv::_OutputArray* instance, cv::UMat* u, Result_void* ocvrs_return) {
		try {
			instance->move(*u);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// move(cv::Mat &) /usr/include/opencv2/core/mat.hpp:381
	void cv__OutputArray_move_const_MatR(const cv::_OutputArray* instance, cv::Mat* m, Result_void* ocvrs_return) {
		try {
			instance->move(*m);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_BufferPool_delete(cv::cuda::BufferPool* instance) {
		delete instance;
	}
	// BufferPool(cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:692
	void cv_cuda_BufferPool_BufferPool_StreamR(cv::cuda::Stream* stream, Result<cv::cuda::BufferPool*>* ocvrs_return) {
		try {
			cv::cuda::BufferPool* ret = new cv::cuda::BufferPool(*stream);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::BufferPool*>))
	}
	
	// getBuffer(int, int, int) /usr/include/opencv2/core/cuda.hpp:695
	void cv_cuda_BufferPool_getBuffer_int_int_int(cv::cuda::BufferPool* instance, int rows, int cols, int type, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->getBuffer(rows, cols, type);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// getBuffer(cv::Size, int) /usr/include/opencv2/core/cuda.hpp:698
	void cv_cuda_BufferPool_getBuffer_Size_int(cv::cuda::BufferPool* instance, cv::Size* size, int type, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->getBuffer(*size, type);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// getAllocator() /usr/include/opencv2/core/cuda.hpp:701
	void cv_cuda_BufferPool_getAllocator_const(const cv::cuda::BufferPool* instance, Result<cv::Ptr<cv::cuda::GpuMat::Allocator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::cuda::GpuMat::Allocator> ret = instance->getAllocator();
			Ok(new cv::Ptr<cv::cuda::GpuMat::Allocator>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::cuda::GpuMat::Allocator>*>))
	}
	
	void cv_DeviceInfo_delete(cv::cuda::DeviceInfo* instance) {
		delete instance;
	}
	// DeviceInfo() /usr/include/opencv2/core/cuda.hpp:1049
	void cv_cuda_DeviceInfo_DeviceInfo(Result<cv::cuda::DeviceInfo*>* ocvrs_return) {
		try {
			cv::cuda::DeviceInfo* ret = new cv::cuda::DeviceInfo();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::DeviceInfo*>))
	}
	
	// DeviceInfo(int) /usr/include/opencv2/core/cuda.hpp:1058
	void cv_cuda_DeviceInfo_DeviceInfo_int(int device_id, Result<cv::cuda::DeviceInfo*>* ocvrs_return) {
		try {
			cv::cuda::DeviceInfo* ret = new cv::cuda::DeviceInfo(device_id);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::DeviceInfo*>))
	}
	
	// deviceID() /usr/include/opencv2/core/cuda.hpp:1062
	void cv_cuda_DeviceInfo_deviceID_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->deviceID();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// name() /usr/include/opencv2/core/cuda.hpp:1065
	void cv_cuda_DeviceInfo_name_const(const cv::cuda::DeviceInfo* instance, Result<void*>* ocvrs_return) {
		try {
			const char* ret = instance->name();
			Ok(ocvrs_create_string(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// totalGlobalMem() /usr/include/opencv2/core/cuda.hpp:1068
	void cv_cuda_DeviceInfo_totalGlobalMem_const(const cv::cuda::DeviceInfo* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->totalGlobalMem();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// sharedMemPerBlock() /usr/include/opencv2/core/cuda.hpp:1071
	void cv_cuda_DeviceInfo_sharedMemPerBlock_const(const cv::cuda::DeviceInfo* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->sharedMemPerBlock();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// regsPerBlock() /usr/include/opencv2/core/cuda.hpp:1074
	void cv_cuda_DeviceInfo_regsPerBlock_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->regsPerBlock();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// warpSize() /usr/include/opencv2/core/cuda.hpp:1077
	void cv_cuda_DeviceInfo_warpSize_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->warpSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// memPitch() /usr/include/opencv2/core/cuda.hpp:1080
	void cv_cuda_DeviceInfo_memPitch_const(const cv::cuda::DeviceInfo* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->memPitch();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// maxThreadsPerBlock() /usr/include/opencv2/core/cuda.hpp:1083
	void cv_cuda_DeviceInfo_maxThreadsPerBlock_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxThreadsPerBlock();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// maxThreadsDim() /usr/include/opencv2/core/cuda.hpp:1086
	void cv_cuda_DeviceInfo_maxThreadsDim_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec3i>* ocvrs_return) {
		try {
			cv::Vec3i ret = instance->maxThreadsDim();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec3i>))
	}
	
	// maxGridSize() /usr/include/opencv2/core/cuda.hpp:1089
	void cv_cuda_DeviceInfo_maxGridSize_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec3i>* ocvrs_return) {
		try {
			cv::Vec3i ret = instance->maxGridSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec3i>))
	}
	
	// clockRate() /usr/include/opencv2/core/cuda.hpp:1092
	void cv_cuda_DeviceInfo_clockRate_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->clockRate();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// totalConstMem() /usr/include/opencv2/core/cuda.hpp:1095
	void cv_cuda_DeviceInfo_totalConstMem_const(const cv::cuda::DeviceInfo* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->totalConstMem();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// majorVersion() /usr/include/opencv2/core/cuda.hpp:1098
	void cv_cuda_DeviceInfo_majorVersion_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->majorVersion();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// minorVersion() /usr/include/opencv2/core/cuda.hpp:1101
	void cv_cuda_DeviceInfo_minorVersion_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->minorVersion();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// textureAlignment() /usr/include/opencv2/core/cuda.hpp:1104
	void cv_cuda_DeviceInfo_textureAlignment_const(const cv::cuda::DeviceInfo* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->textureAlignment();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// texturePitchAlignment() /usr/include/opencv2/core/cuda.hpp:1107
	void cv_cuda_DeviceInfo_texturePitchAlignment_const(const cv::cuda::DeviceInfo* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->texturePitchAlignment();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// multiProcessorCount() /usr/include/opencv2/core/cuda.hpp:1110
	void cv_cuda_DeviceInfo_multiProcessorCount_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->multiProcessorCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// kernelExecTimeoutEnabled() /usr/include/opencv2/core/cuda.hpp:1113
	void cv_cuda_DeviceInfo_kernelExecTimeoutEnabled_const(const cv::cuda::DeviceInfo* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->kernelExecTimeoutEnabled();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// integrated() /usr/include/opencv2/core/cuda.hpp:1116
	void cv_cuda_DeviceInfo_integrated_const(const cv::cuda::DeviceInfo* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->integrated();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// canMapHostMemory() /usr/include/opencv2/core/cuda.hpp:1119
	void cv_cuda_DeviceInfo_canMapHostMemory_const(const cv::cuda::DeviceInfo* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->canMapHostMemory();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// computeMode() /usr/include/opencv2/core/cuda.hpp:1130
	void cv_cuda_DeviceInfo_computeMode_const(const cv::cuda::DeviceInfo* instance, Result<cv::cuda::DeviceInfo::ComputeMode>* ocvrs_return) {
		try {
			cv::cuda::DeviceInfo::ComputeMode ret = instance->computeMode();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::DeviceInfo::ComputeMode>))
	}
	
	// maxTexture1D() /usr/include/opencv2/core/cuda.hpp:1133
	void cv_cuda_DeviceInfo_maxTexture1D_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxTexture1D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// maxTexture1DMipmap() /usr/include/opencv2/core/cuda.hpp:1136
	void cv_cuda_DeviceInfo_maxTexture1DMipmap_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxTexture1DMipmap();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// maxTexture1DLinear() /usr/include/opencv2/core/cuda.hpp:1139
	void cv_cuda_DeviceInfo_maxTexture1DLinear_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxTexture1DLinear();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// maxTexture2D() /usr/include/opencv2/core/cuda.hpp:1142
	void cv_cuda_DeviceInfo_maxTexture2D_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec2i>* ocvrs_return) {
		try {
			cv::Vec2i ret = instance->maxTexture2D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2i>))
	}
	
	// maxTexture2DMipmap() /usr/include/opencv2/core/cuda.hpp:1145
	void cv_cuda_DeviceInfo_maxTexture2DMipmap_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec2i>* ocvrs_return) {
		try {
			cv::Vec2i ret = instance->maxTexture2DMipmap();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2i>))
	}
	
	// maxTexture2DLinear() /usr/include/opencv2/core/cuda.hpp:1148
	void cv_cuda_DeviceInfo_maxTexture2DLinear_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec3i>* ocvrs_return) {
		try {
			cv::Vec3i ret = instance->maxTexture2DLinear();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec3i>))
	}
	
	// maxTexture2DGather() /usr/include/opencv2/core/cuda.hpp:1151
	void cv_cuda_DeviceInfo_maxTexture2DGather_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec2i>* ocvrs_return) {
		try {
			cv::Vec2i ret = instance->maxTexture2DGather();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2i>))
	}
	
	// maxTexture3D() /usr/include/opencv2/core/cuda.hpp:1154
	void cv_cuda_DeviceInfo_maxTexture3D_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec3i>* ocvrs_return) {
		try {
			cv::Vec3i ret = instance->maxTexture3D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec3i>))
	}
	
	// maxTextureCubemap() /usr/include/opencv2/core/cuda.hpp:1157
	void cv_cuda_DeviceInfo_maxTextureCubemap_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxTextureCubemap();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// maxTexture1DLayered() /usr/include/opencv2/core/cuda.hpp:1160
	void cv_cuda_DeviceInfo_maxTexture1DLayered_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec2i>* ocvrs_return) {
		try {
			cv::Vec2i ret = instance->maxTexture1DLayered();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2i>))
	}
	
	// maxTexture2DLayered() /usr/include/opencv2/core/cuda.hpp:1163
	void cv_cuda_DeviceInfo_maxTexture2DLayered_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec3i>* ocvrs_return) {
		try {
			cv::Vec3i ret = instance->maxTexture2DLayered();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec3i>))
	}
	
	// maxTextureCubemapLayered() /usr/include/opencv2/core/cuda.hpp:1166
	void cv_cuda_DeviceInfo_maxTextureCubemapLayered_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec2i>* ocvrs_return) {
		try {
			cv::Vec2i ret = instance->maxTextureCubemapLayered();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2i>))
	}
	
	// maxSurface1D() /usr/include/opencv2/core/cuda.hpp:1169
	void cv_cuda_DeviceInfo_maxSurface1D_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxSurface1D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// maxSurface2D() /usr/include/opencv2/core/cuda.hpp:1172
	void cv_cuda_DeviceInfo_maxSurface2D_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec2i>* ocvrs_return) {
		try {
			cv::Vec2i ret = instance->maxSurface2D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2i>))
	}
	
	// maxSurface3D() /usr/include/opencv2/core/cuda.hpp:1175
	void cv_cuda_DeviceInfo_maxSurface3D_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec3i>* ocvrs_return) {
		try {
			cv::Vec3i ret = instance->maxSurface3D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec3i>))
	}
	
	// maxSurface1DLayered() /usr/include/opencv2/core/cuda.hpp:1178
	void cv_cuda_DeviceInfo_maxSurface1DLayered_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec2i>* ocvrs_return) {
		try {
			cv::Vec2i ret = instance->maxSurface1DLayered();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2i>))
	}
	
	// maxSurface2DLayered() /usr/include/opencv2/core/cuda.hpp:1181
	void cv_cuda_DeviceInfo_maxSurface2DLayered_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec3i>* ocvrs_return) {
		try {
			cv::Vec3i ret = instance->maxSurface2DLayered();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec3i>))
	}
	
	// maxSurfaceCubemap() /usr/include/opencv2/core/cuda.hpp:1184
	void cv_cuda_DeviceInfo_maxSurfaceCubemap_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxSurfaceCubemap();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// maxSurfaceCubemapLayered() /usr/include/opencv2/core/cuda.hpp:1187
	void cv_cuda_DeviceInfo_maxSurfaceCubemapLayered_const(const cv::cuda::DeviceInfo* instance, Result<cv::Vec2i>* ocvrs_return) {
		try {
			cv::Vec2i ret = instance->maxSurfaceCubemapLayered();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec2i>))
	}
	
	// surfaceAlignment() /usr/include/opencv2/core/cuda.hpp:1190
	void cv_cuda_DeviceInfo_surfaceAlignment_const(const cv::cuda::DeviceInfo* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->surfaceAlignment();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// concurrentKernels() /usr/include/opencv2/core/cuda.hpp:1193
	void cv_cuda_DeviceInfo_concurrentKernels_const(const cv::cuda::DeviceInfo* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->concurrentKernels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// ECCEnabled() /usr/include/opencv2/core/cuda.hpp:1196
	void cv_cuda_DeviceInfo_ECCEnabled_const(const cv::cuda::DeviceInfo* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->ECCEnabled();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// pciBusID() /usr/include/opencv2/core/cuda.hpp:1199
	void cv_cuda_DeviceInfo_pciBusID_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->pciBusID();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// pciDeviceID() /usr/include/opencv2/core/cuda.hpp:1202
	void cv_cuda_DeviceInfo_pciDeviceID_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->pciDeviceID();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// pciDomainID() /usr/include/opencv2/core/cuda.hpp:1205
	void cv_cuda_DeviceInfo_pciDomainID_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->pciDomainID();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// tccDriver() /usr/include/opencv2/core/cuda.hpp:1208
	void cv_cuda_DeviceInfo_tccDriver_const(const cv::cuda::DeviceInfo* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->tccDriver();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// asyncEngineCount() /usr/include/opencv2/core/cuda.hpp:1211
	void cv_cuda_DeviceInfo_asyncEngineCount_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->asyncEngineCount();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// unifiedAddressing() /usr/include/opencv2/core/cuda.hpp:1214
	void cv_cuda_DeviceInfo_unifiedAddressing_const(const cv::cuda::DeviceInfo* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->unifiedAddressing();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// memoryClockRate() /usr/include/opencv2/core/cuda.hpp:1217
	void cv_cuda_DeviceInfo_memoryClockRate_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->memoryClockRate();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// memoryBusWidth() /usr/include/opencv2/core/cuda.hpp:1220
	void cv_cuda_DeviceInfo_memoryBusWidth_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->memoryBusWidth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// l2CacheSize() /usr/include/opencv2/core/cuda.hpp:1223
	void cv_cuda_DeviceInfo_l2CacheSize_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->l2CacheSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// maxThreadsPerMultiProcessor() /usr/include/opencv2/core/cuda.hpp:1226
	void cv_cuda_DeviceInfo_maxThreadsPerMultiProcessor_const(const cv::cuda::DeviceInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxThreadsPerMultiProcessor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// queryMemory(size_t &, size_t &) /usr/include/opencv2/core/cuda.hpp:1229
	void cv_cuda_DeviceInfo_queryMemory_const_size_tR_size_tR(const cv::cuda::DeviceInfo* instance, size_t* totalMemory, size_t* freeMemory, Result_void* ocvrs_return) {
		try {
			instance->queryMemory(*totalMemory, *freeMemory);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// freeMemory() /usr/include/opencv2/core/cuda.hpp:1230
	void cv_cuda_DeviceInfo_freeMemory_const(const cv::cuda::DeviceInfo* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->freeMemory();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// totalMemory() /usr/include/opencv2/core/cuda.hpp:1231
	void cv_cuda_DeviceInfo_totalMemory_const(const cv::cuda::DeviceInfo* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->totalMemory();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// supports(cv::cuda::FeatureSet) /usr/include/opencv2/core/cuda.hpp:1239
	void cv_cuda_DeviceInfo_supports_const_FeatureSet(const cv::cuda::DeviceInfo* instance, cv::cuda::FeatureSet feature_set, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->supports(feature_set);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isCompatible() /usr/include/opencv2/core/cuda.hpp:1246
	void cv_cuda_DeviceInfo_isCompatible_const(const cv::cuda::DeviceInfo* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isCompatible();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_Event_delete(cv::cuda::Event* instance) {
		delete instance;
	}
	// Event(const Event::CreateFlags) /usr/include/opencv2/core/cuda.hpp:927
	void cv_cuda_Event_Event_const_CreateFlags(const cv::cuda::Event::CreateFlags flags, Result<cv::cuda::Event*>* ocvrs_return) {
		try {
			cv::cuda::Event* ret = new cv::cuda::Event(flags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::Event*>))
	}
	
	// record(cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:930
	void cv_cuda_Event_record_StreamR(cv::cuda::Event* instance, cv::cuda::Stream* stream, Result_void* ocvrs_return) {
		try {
			instance->record(*stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// queryIfComplete() /usr/include/opencv2/core/cuda.hpp:933
	void cv_cuda_Event_queryIfComplete_const(const cv::cuda::Event* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->queryIfComplete();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// waitForCompletion() /usr/include/opencv2/core/cuda.hpp:936
	void cv_cuda_Event_waitForCompletion(cv::cuda::Event* instance, Result_void* ocvrs_return) {
		try {
			instance->waitForCompletion();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// elapsedTime(const cv::cuda::Event &, const cv::cuda::Event &) /usr/include/opencv2/core/cuda.hpp:939
	void cv_cuda_Event_elapsedTime_const_EventR_const_EventR(const cv::cuda::Event* start, const cv::cuda::Event* end, Result<float>* ocvrs_return) {
		try {
			float ret = cv::cuda::Event::elapsedTime(*start, *end);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// data /usr/include/opencv2/core/cuda.hpp:354
	unsigned char* cv_cuda_GpuData_getPropData(cv::cuda::GpuData* instance) {
			unsigned char* ret = instance->data;
			return ret;
	}
	
	// data /usr/include/opencv2/core/cuda.hpp:354
	void cv_cuda_GpuData_setPropData_unsigned_charX(cv::cuda::GpuData* instance, unsigned char* val) {
			instance->data = val;
	}
	
	// size /usr/include/opencv2/core/cuda.hpp:355
	size_t cv_cuda_GpuData_getPropSize_const(const cv::cuda::GpuData* instance) {
			size_t ret = instance->size;
			return ret;
	}
	
	// size /usr/include/opencv2/core/cuda.hpp:355
	void cv_cuda_GpuData_setPropSize_size_t(cv::cuda::GpuData* instance, size_t val) {
			instance->size = val;
	}
	
	void cv_GpuData_delete(cv::cuda::GpuData* instance) {
		delete instance;
	}
	// GpuData(size_t) /usr/include/opencv2/core/cuda.hpp:345
	void cv_cuda_GpuData_GpuData_size_t(size_t _size, Result<cv::cuda::GpuData*>* ocvrs_return) {
		try {
			cv::cuda::GpuData* ret = new cv::cuda::GpuData(_size);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuData*>))
	}
	
	// flags /usr/include/opencv2/core/cuda.hpp:320
	int cv_cuda_GpuMat_getPropFlags_const(const cv::cuda::GpuMat* instance) {
			int ret = instance->flags;
			return ret;
	}
	
	// flags /usr/include/opencv2/core/cuda.hpp:320
	void cv_cuda_GpuMat_setPropFlags_int(cv::cuda::GpuMat* instance, int val) {
			instance->flags = val;
	}
	
	// rows /usr/include/opencv2/core/cuda.hpp:323
	int cv_cuda_GpuMat_getPropRows_const(const cv::cuda::GpuMat* instance) {
			int ret = instance->rows;
			return ret;
	}
	
	// rows /usr/include/opencv2/core/cuda.hpp:323
	void cv_cuda_GpuMat_setPropRows_int(cv::cuda::GpuMat* instance, int val) {
			instance->rows = val;
	}
	
	// cols /usr/include/opencv2/core/cuda.hpp:323
	int cv_cuda_GpuMat_getPropCols_const(const cv::cuda::GpuMat* instance) {
			int ret = instance->cols;
			return ret;
	}
	
	// cols /usr/include/opencv2/core/cuda.hpp:323
	void cv_cuda_GpuMat_setPropCols_int(cv::cuda::GpuMat* instance, int val) {
			instance->cols = val;
	}
	
	// step /usr/include/opencv2/core/cuda.hpp:326
	size_t cv_cuda_GpuMat_getPropStep_const(const cv::cuda::GpuMat* instance) {
			size_t ret = instance->step;
			return ret;
	}
	
	// step /usr/include/opencv2/core/cuda.hpp:326
	void cv_cuda_GpuMat_setPropStep_size_t(cv::cuda::GpuMat* instance, size_t val) {
			instance->step = val;
	}
	
	// data /usr/include/opencv2/core/cuda.hpp:329
	unsigned char* cv_cuda_GpuMat_getPropData(cv::cuda::GpuMat* instance) {
			unsigned char* ret = instance->data;
			return ret;
	}
	
	// data /usr/include/opencv2/core/cuda.hpp:329
	void cv_cuda_GpuMat_setPropData_unsigned_charX(cv::cuda::GpuMat* instance, unsigned char* val) {
			instance->data = val;
	}
	
	// refcount /usr/include/opencv2/core/cuda.hpp:333
	int* cv_cuda_GpuMat_getPropRefcount(cv::cuda::GpuMat* instance) {
			int* ret = instance->refcount;
			return ret;
	}
	
	// refcount /usr/include/opencv2/core/cuda.hpp:333
	void cv_cuda_GpuMat_setPropRefcount_intX(cv::cuda::GpuMat* instance, int* val) {
			instance->refcount = val;
	}
	
	// datastart /usr/include/opencv2/core/cuda.hpp:336
	unsigned char* cv_cuda_GpuMat_getPropDatastart(cv::cuda::GpuMat* instance) {
			unsigned char* ret = instance->datastart;
			return ret;
	}
	
	// datastart /usr/include/opencv2/core/cuda.hpp:336
	void cv_cuda_GpuMat_setPropDatastart_unsigned_charX(cv::cuda::GpuMat* instance, unsigned char* val) {
			instance->datastart = val;
	}
	
	// dataend /usr/include/opencv2/core/cuda.hpp:337
	const unsigned char* cv_cuda_GpuMat_getPropDataend_const(const cv::cuda::GpuMat* instance) {
			const unsigned char* ret = instance->dataend;
			return ret;
	}
	
	// allocator /usr/include/opencv2/core/cuda.hpp:340
	cv::cuda::GpuMat::Allocator* cv_cuda_GpuMat_getPropAllocator(cv::cuda::GpuMat* instance) {
			cv::cuda::GpuMat::Allocator* ret = instance->allocator;
			return ret;
	}
	
	// allocator /usr/include/opencv2/core/cuda.hpp:340
	void cv_cuda_GpuMat_setPropAllocator_AllocatorX(cv::cuda::GpuMat* instance, cv::cuda::GpuMat::Allocator* val) {
			instance->allocator = val;
	}
	
	void cv_GpuMat_delete(cv::cuda::GpuMat* instance) {
		delete instance;
	}
	// defaultAllocator() /usr/include/opencv2/core/cuda.hpp:119
	void cv_cuda_GpuMat_defaultAllocator(Result<cv::cuda::GpuMat::Allocator*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat::Allocator* ret = cv::cuda::GpuMat::defaultAllocator();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat::Allocator*>))
	}
	
	// setDefaultAllocator(GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:120
	void cv_cuda_GpuMat_setDefaultAllocator_AllocatorX(cv::cuda::GpuMat::Allocator* allocator, Result_void* ocvrs_return) {
		try {
			cv::cuda::GpuMat::setDefaultAllocator(allocator);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// GpuMat(GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:123
	void cv_cuda_GpuMat_GpuMat_AllocatorX(cv::cuda::GpuMat::Allocator* allocator, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(allocator);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// GpuMat(int, int, int, GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:126
	void cv_cuda_GpuMat_GpuMat_int_int_int_AllocatorX(int rows, int cols, int type, cv::cuda::GpuMat::Allocator* allocator, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(rows, cols, type, allocator);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// GpuMat(cv::Size, int, GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:127
	void cv_cuda_GpuMat_GpuMat_Size_int_AllocatorX(cv::Size* size, int type, cv::cuda::GpuMat::Allocator* allocator, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*size, type, allocator);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// GpuMat(int, int, int, cv::Scalar, GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:130
	void cv_cuda_GpuMat_GpuMat_int_int_int_Scalar_AllocatorX(int rows, int cols, int type, cv::Scalar* s, cv::cuda::GpuMat::Allocator* allocator, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(rows, cols, type, *s, allocator);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// GpuMat(cv::Size, int, cv::Scalar, GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:131
	void cv_cuda_GpuMat_GpuMat_Size_int_Scalar_AllocatorX(cv::Size* size, int type, cv::Scalar* s, cv::cuda::GpuMat::Allocator* allocator, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*size, type, *s, allocator);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// GpuMat(const cv::cuda::GpuMat &) /usr/include/opencv2/core/cuda.hpp:134
	void cv_cuda_GpuMat_GpuMat_const_GpuMatR(const cv::cuda::GpuMat* m, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// GpuMat(int, int, int, void *, size_t) /usr/include/opencv2/core/cuda.hpp:137
	void cv_cuda_GpuMat_GpuMat_int_int_int_voidX_size_t(int rows, int cols, int type, void* data, size_t step, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(rows, cols, type, data, step);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// GpuMat(cv::Size, int, void *, size_t) /usr/include/opencv2/core/cuda.hpp:138
	void cv_cuda_GpuMat_GpuMat_Size_int_voidX_size_t(cv::Size* size, int type, void* data, size_t step, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*size, type, data, step);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// GpuMat(const cv::cuda::GpuMat &, cv::Range, cv::Range) /usr/include/opencv2/core/cuda.hpp:141
	void cv_cuda_GpuMat_GpuMat_const_GpuMatR_Range_Range(const cv::cuda::GpuMat* m, cv::Range* rowRange, cv::Range* colRange, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*m, *rowRange, *colRange);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// GpuMat(const cv::cuda::GpuMat &, cv::Rect) /usr/include/opencv2/core/cuda.hpp:142
	void cv_cuda_GpuMat_GpuMat_const_GpuMatR_Rect(const cv::cuda::GpuMat* m, cv::Rect* roi, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*m, *roi);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// GpuMat(cv::InputArray, GpuMat::Allocator *) /usr/include/opencv2/core/cuda.hpp:145
	void cv_cuda_GpuMat_GpuMat_const__InputArrayR_AllocatorX(const cv::_InputArray* arr, cv::cuda::GpuMat::Allocator* allocator, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat* ret = new cv::cuda::GpuMat(*arr, allocator);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// create(int, int, int) /usr/include/opencv2/core/cuda.hpp:154
	void cv_cuda_GpuMat_create_int_int_int(cv::cuda::GpuMat* instance, int rows, int cols, int type, Result_void* ocvrs_return) {
		try {
			instance->create(rows, cols, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(cv::Size, int) /usr/include/opencv2/core/cuda.hpp:155
	void cv_cuda_GpuMat_create_Size_int(cv::cuda::GpuMat* instance, cv::Size* size, int type, Result_void* ocvrs_return) {
		try {
			instance->create(*size, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// release() /usr/include/opencv2/core/cuda.hpp:158
	void cv_cuda_GpuMat_release(cv::cuda::GpuMat* instance, Result_void* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// swap(cv::cuda::GpuMat &) /usr/include/opencv2/core/cuda.hpp:161
	void cv_cuda_GpuMat_swap_GpuMatR(cv::cuda::GpuMat* instance, cv::cuda::GpuMat* mat, Result_void* ocvrs_return) {
		try {
			instance->swap(*mat);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// upload(cv::InputArray) /usr/include/opencv2/core/cuda.hpp:168
	void cv_cuda_GpuMat_upload_const__InputArrayR(cv::cuda::GpuMat* instance, const cv::_InputArray* arr, Result_void* ocvrs_return) {
		try {
			instance->upload(*arr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// upload(cv::InputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:178
	void cv_cuda_GpuMat_upload_const__InputArrayR_StreamR(cv::cuda::GpuMat* instance, const cv::_InputArray* arr, cv::cuda::Stream* stream, Result_void* ocvrs_return) {
		try {
			instance->upload(*arr, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// download(cv::OutputArray) /usr/include/opencv2/core/cuda.hpp:185
	void cv_cuda_GpuMat_download_const_const__OutputArrayR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			instance->download(*dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// download(cv::OutputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:195
	void cv_cuda_GpuMat_download_const_const__OutputArrayR_StreamR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, cv::cuda::Stream* stream, Result_void* ocvrs_return) {
		try {
			instance->download(*dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// clone() /usr/include/opencv2/core/cuda.hpp:198
	void cv_cuda_GpuMat_clone_const(const cv::cuda::GpuMat* instance, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->clone();
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// copyTo(cv::OutputArray) /usr/include/opencv2/core/cuda.hpp:201
	void cv_cuda_GpuMat_copyTo_const_const__OutputArrayR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			instance->copyTo(*dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// copyTo(cv::OutputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:204
	void cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_StreamR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, cv::cuda::Stream* stream, Result_void* ocvrs_return) {
		try {
			instance->copyTo(*dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// copyTo(cv::OutputArray, cv::InputArray) /usr/include/opencv2/core/cuda.hpp:207
	void cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_const__InputArrayR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			instance->copyTo(*dst, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// copyTo(cv::OutputArray, cv::InputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:210
	void cv_cuda_GpuMat_copyTo_const_const__OutputArrayR_const__InputArrayR_StreamR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, const cv::_InputArray* mask, cv::cuda::Stream* stream, Result_void* ocvrs_return) {
		try {
			instance->copyTo(*dst, *mask, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setTo(cv::Scalar) /usr/include/opencv2/core/cuda.hpp:213
	void cv_cuda_GpuMat_setTo_Scalar(cv::cuda::GpuMat* instance, cv::Scalar* s, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->setTo(*s);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// setTo(cv::Scalar, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:216
	void cv_cuda_GpuMat_setTo_Scalar_StreamR(cv::cuda::GpuMat* instance, cv::Scalar* s, cv::cuda::Stream* stream, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->setTo(*s, *stream);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// setTo(cv::Scalar, cv::InputArray) /usr/include/opencv2/core/cuda.hpp:219
	void cv_cuda_GpuMat_setTo_Scalar_const__InputArrayR(cv::cuda::GpuMat* instance, cv::Scalar* s, const cv::_InputArray* mask, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->setTo(*s, *mask);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// setTo(cv::Scalar, cv::InputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:222
	void cv_cuda_GpuMat_setTo_Scalar_const__InputArrayR_StreamR(cv::cuda::GpuMat* instance, cv::Scalar* s, const cv::_InputArray* mask, cv::cuda::Stream* stream, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->setTo(*s, *mask, *stream);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// convertTo(cv::OutputArray, int) /usr/include/opencv2/core/cuda.hpp:225
	void cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, int rtype, Result_void* ocvrs_return) {
		try {
			instance->convertTo(*dst, rtype);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// convertTo(cv::OutputArray, int, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:228
	void cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_StreamR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, int rtype, cv::cuda::Stream* stream, Result_void* ocvrs_return) {
		try {
			instance->convertTo(*dst, rtype, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// convertTo(cv::OutputArray, int, double, double) /usr/include/opencv2/core/cuda.hpp:231
	void cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_double(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, int rtype, double alpha, double beta, Result_void* ocvrs_return) {
		try {
			instance->convertTo(*dst, rtype, alpha, beta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// convertTo(cv::OutputArray, int, double, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:234
	void cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_StreamR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, int rtype, double alpha, cv::cuda::Stream* stream, Result_void* ocvrs_return) {
		try {
			instance->convertTo(*dst, rtype, alpha, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// convertTo(cv::OutputArray, int, double, double, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:237
	void cv_cuda_GpuMat_convertTo_const_const__OutputArrayR_int_double_double_StreamR(const cv::cuda::GpuMat* instance, const cv::_OutputArray* dst, int rtype, double alpha, double beta, cv::cuda::Stream* stream, Result_void* ocvrs_return) {
		try {
			instance->convertTo(*dst, rtype, alpha, beta, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// assignTo(cv::cuda::GpuMat &, int) /usr/include/opencv2/core/cuda.hpp:239
	void cv_cuda_GpuMat_assignTo_const_GpuMatR_int(const cv::cuda::GpuMat* instance, cv::cuda::GpuMat* m, int type, Result_void* ocvrs_return) {
		try {
			instance->assignTo(*m, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// ptr(int) /usr/include/opencv2/core/cuda.hpp:242
	void cv_cuda_GpuMat_ptr_int(cv::cuda::GpuMat* instance, int y, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->ptr(y);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	// ptr(int) /usr/include/opencv2/core/cuda.hpp:243
	void cv_cuda_GpuMat_ptr_const_int(const cv::cuda::GpuMat* instance, int y, Result<const unsigned char*>* ocvrs_return) {
		try {
			const unsigned char* ret = instance->ptr(y);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<const unsigned char*>))
	}
	
	// row(int) /usr/include/opencv2/core/cuda.hpp:253
	void cv_cuda_GpuMat_row_const_int(const cv::cuda::GpuMat* instance, int y, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->row(y);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// col(int) /usr/include/opencv2/core/cuda.hpp:256
	void cv_cuda_GpuMat_col_const_int(const cv::cuda::GpuMat* instance, int x, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->col(x);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// rowRange(int, int) /usr/include/opencv2/core/cuda.hpp:259
	void cv_cuda_GpuMat_rowRange_const_int_int(const cv::cuda::GpuMat* instance, int startrow, int endrow, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->rowRange(startrow, endrow);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// rowRange(cv::Range) /usr/include/opencv2/core/cuda.hpp:260
	void cv_cuda_GpuMat_rowRange_const_Range(const cv::cuda::GpuMat* instance, cv::Range* r, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->rowRange(*r);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// colRange(int, int) /usr/include/opencv2/core/cuda.hpp:263
	void cv_cuda_GpuMat_colRange_const_int_int(const cv::cuda::GpuMat* instance, int startcol, int endcol, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->colRange(startcol, endcol);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// colRange(cv::Range) /usr/include/opencv2/core/cuda.hpp:264
	void cv_cuda_GpuMat_colRange_const_Range(const cv::cuda::GpuMat* instance, cv::Range* r, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->colRange(*r);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// reshape(int, int) /usr/include/opencv2/core/cuda.hpp:272
	void cv_cuda_GpuMat_reshape_const_int_int(const cv::cuda::GpuMat* instance, int cn, int rows, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->reshape(cn, rows);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// locateROI(cv::Size &, cv::Point &) /usr/include/opencv2/core/cuda.hpp:275
	void cv_cuda_GpuMat_locateROI_const_SizeR_PointR(const cv::cuda::GpuMat* instance, cv::Size* wholeSize, cv::Point* ofs, Result_void* ocvrs_return) {
		try {
			instance->locateROI(*wholeSize, *ofs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// adjustROI(int, int, int, int) /usr/include/opencv2/core/cuda.hpp:278
	void cv_cuda_GpuMat_adjustROI_int_int_int_int(cv::cuda::GpuMat* instance, int dtop, int dbottom, int dleft, int dright, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->adjustROI(dtop, dbottom, dleft, dright);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// isContinuous() /usr/include/opencv2/core/cuda.hpp:282
	void cv_cuda_GpuMat_isContinuous_const(const cv::cuda::GpuMat* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isContinuous();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// elemSize() /usr/include/opencv2/core/cuda.hpp:285
	void cv_cuda_GpuMat_elemSize_const(const cv::cuda::GpuMat* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->elemSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// elemSize1() /usr/include/opencv2/core/cuda.hpp:288
	void cv_cuda_GpuMat_elemSize1_const(const cv::cuda::GpuMat* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->elemSize1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// type() /usr/include/opencv2/core/cuda.hpp:291
	void cv_cuda_GpuMat_type_const(const cv::cuda::GpuMat* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// depth() /usr/include/opencv2/core/cuda.hpp:294
	void cv_cuda_GpuMat_depth_const(const cv::cuda::GpuMat* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->depth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// channels() /usr/include/opencv2/core/cuda.hpp:297
	void cv_cuda_GpuMat_channels_const(const cv::cuda::GpuMat* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->channels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// step1() /usr/include/opencv2/core/cuda.hpp:300
	void cv_cuda_GpuMat_step1_const(const cv::cuda::GpuMat* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->step1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// size() /usr/include/opencv2/core/cuda.hpp:303
	void cv_cuda_GpuMat_size_const(const cv::cuda::GpuMat* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// empty() /usr/include/opencv2/core/cuda.hpp:306
	void cv_cuda_GpuMat_empty_const(const cv::cuda::GpuMat* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// cudaPtr() /usr/include/opencv2/core/cuda.hpp:309
	void cv_cuda_GpuMat_cudaPtr_const(const cv::cuda::GpuMat* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->cudaPtr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// updateContinuityFlag() /usr/include/opencv2/core/cuda.hpp:312
	void cv_cuda_GpuMat_updateContinuityFlag(cv::cuda::GpuMat* instance, Result_void* ocvrs_return) {
		try {
			instance->updateContinuityFlag();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// allocate(cv::cuda::GpuMat *, int, int, size_t) /usr/include/opencv2/core/cuda.hpp:114
	void cv_cuda_GpuMat_Allocator_allocate_GpuMatX_int_int_size_t(cv::cuda::GpuMat::Allocator* instance, cv::cuda::GpuMat* mat, int rows, int cols, size_t elemSize, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->allocate(mat, rows, cols, elemSize);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// free(cv::cuda::GpuMat *) /usr/include/opencv2/core/cuda.hpp:115
	void cv_cuda_GpuMat_Allocator_free_GpuMatX(cv::cuda::GpuMat::Allocator* instance, cv::cuda::GpuMat* mat, Result_void* ocvrs_return) {
		try {
			instance->free(mat);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// flags /usr/include/opencv2/core/cuda.hpp:513
	int cv_cuda_GpuMatND_getPropFlags_const(const cv::cuda::GpuMatND* instance) {
			int ret = instance->flags;
			return ret;
	}
	
	// flags /usr/include/opencv2/core/cuda.hpp:513
	void cv_cuda_GpuMatND_setPropFlags_int(cv::cuda::GpuMatND* instance, int val) {
			instance->flags = val;
	}
	
	// dims /usr/include/opencv2/core/cuda.hpp:516
	int cv_cuda_GpuMatND_getPropDims_const(const cv::cuda::GpuMatND* instance) {
			int ret = instance->dims;
			return ret;
	}
	
	// dims /usr/include/opencv2/core/cuda.hpp:516
	void cv_cuda_GpuMatND_setPropDims_int(cv::cuda::GpuMatND* instance, int val) {
			instance->dims = val;
	}
	
	// size /usr/include/opencv2/core/cuda.hpp:519
	cv::cuda::GpuMatND::SizeArray* cv_cuda_GpuMatND_getPropSize_const(const cv::cuda::GpuMatND* instance) {
			cv::cuda::GpuMatND::SizeArray ret = instance->size;
			return new cv::cuda::GpuMatND::SizeArray(ret);
	}
	
	// size /usr/include/opencv2/core/cuda.hpp:519
	void cv_cuda_GpuMatND_setPropSize_SizeArray(cv::cuda::GpuMatND* instance, cv::cuda::GpuMatND::SizeArray* val) {
			instance->size = *val;
	}
	
	// step /usr/include/opencv2/core/cuda.hpp:524
	cv::cuda::GpuMatND::StepArray* cv_cuda_GpuMatND_getPropStep_const(const cv::cuda::GpuMatND* instance) {
			cv::cuda::GpuMatND::StepArray ret = instance->step;
			return new cv::cuda::GpuMatND::StepArray(ret);
	}
	
	// step /usr/include/opencv2/core/cuda.hpp:524
	void cv_cuda_GpuMatND_setPropStep_StepArray(cv::cuda::GpuMatND* instance, cv::cuda::GpuMatND::StepArray* val) {
			instance->step = *val;
	}
	
	void cv_GpuMatND_delete(cv::cuda::GpuMatND* instance) {
		delete instance;
	}
	// GpuMatND() /usr/include/opencv2/core/cuda.hpp:369
	void cv_cuda_GpuMatND_GpuMatND(Result<cv::cuda::GpuMatND*>* ocvrs_return) {
		try {
			cv::cuda::GpuMatND* ret = new cv::cuda::GpuMatND();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMatND*>))
	}
	
	// GpuMatND(cv::cuda::GpuMatND::SizeArray, int) /usr/include/opencv2/core/cuda.hpp:376
	void cv_cuda_GpuMatND_GpuMatND_SizeArray_int(cv::cuda::GpuMatND::SizeArray* size, int type, Result<cv::cuda::GpuMatND*>* ocvrs_return) {
		try {
			cv::cuda::GpuMatND* ret = new cv::cuda::GpuMatND(*size, type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMatND*>))
	}
	
	// GpuMatND(cv::cuda::GpuMatND::SizeArray, int, void *, cv::cuda::GpuMatND::StepArray) /usr/include/opencv2/core/cuda.hpp:390
	void cv_cuda_GpuMatND_GpuMatND_SizeArray_int_voidX_StepArray(cv::cuda::GpuMatND::SizeArray* size, int type, void* data, cv::cuda::GpuMatND::StepArray* step, Result<cv::cuda::GpuMatND*>* ocvrs_return) {
		try {
			cv::cuda::GpuMatND* ret = new cv::cuda::GpuMatND(*size, type, data, *step);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMatND*>))
	}
	
	// create(cv::cuda::GpuMatND::SizeArray, int) /usr/include/opencv2/core/cuda.hpp:399
	void cv_cuda_GpuMatND_create_SizeArray_int(cv::cuda::GpuMatND* instance, cv::cuda::GpuMatND::SizeArray* size, int type, Result_void* ocvrs_return) {
		try {
			instance->create(*size, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// release() /usr/include/opencv2/core/cuda.hpp:401
	void cv_cuda_GpuMatND_release(cv::cuda::GpuMatND* instance, Result_void* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// swap(cv::cuda::GpuMatND &) /usr/include/opencv2/core/cuda.hpp:403
	void cv_cuda_GpuMatND_swap_GpuMatNDR(cv::cuda::GpuMatND* instance, cv::cuda::GpuMatND* m) {
			instance->swap(*m);
	}
	
	// clone() /usr/include/opencv2/core/cuda.hpp:410
	void cv_cuda_GpuMatND_clone_const(const cv::cuda::GpuMatND* instance, Result<cv::cuda::GpuMatND*>* ocvrs_return) {
		try {
			cv::cuda::GpuMatND ret = instance->clone();
			Ok(new cv::cuda::GpuMatND(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMatND*>))
	}
	
	// clone(cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:415
	void cv_cuda_GpuMatND_clone_const_StreamR(const cv::cuda::GpuMatND* instance, cv::cuda::Stream* stream, Result<cv::cuda::GpuMatND*>* ocvrs_return) {
		try {
			cv::cuda::GpuMatND ret = instance->clone(*stream);
			Ok(new cv::cuda::GpuMatND(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMatND*>))
	}
	
	// createGpuMatHeader(cv::cuda::GpuMatND::IndexArray, cv::Range, cv::Range) /usr/include/opencv2/core/cuda.hpp:429
	void cv_cuda_GpuMatND_createGpuMatHeader_const_IndexArray_Range_Range(const cv::cuda::GpuMatND* instance, cv::cuda::GpuMatND::IndexArray* idx, cv::Range* rowRange, cv::Range* colRange, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->createGpuMatHeader(*idx, *rowRange, *colRange);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// createGpuMatHeader() /usr/include/opencv2/core/cuda.hpp:437
	void cv_cuda_GpuMatND_createGpuMatHeader_const(const cv::cuda::GpuMatND* instance, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->createGpuMatHeader();
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// operator GpuMat() /usr/include/opencv2/core/cuda.hpp:450
	void cv_cuda_GpuMatND_operator_cv_cuda_GpuMat_const(const cv::cuda::GpuMatND* instance, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->operator cv::cuda::GpuMat();
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// GpuMatND(const cv::cuda::GpuMatND &) /usr/include/opencv2/core/cuda.hpp:452
	cv::cuda::GpuMatND* cv_cuda_GpuMatND_GpuMatND_const_GpuMatNDR(const cv::cuda::GpuMatND* unnamed) {
			cv::cuda::GpuMatND* ret = new cv::cuda::GpuMatND(*unnamed);
			return ret;
	}
	
	// GpuMatND(cv::cuda::GpuMatND &&) /usr/include/opencv2/core/cuda.hpp:459
	cv::cuda::GpuMatND* cv_cuda_GpuMatND_GpuMatND_GpuMatNDR(cv::cuda::GpuMatND* unnamed) {
			cv::cuda::GpuMatND* ret = new cv::cuda::GpuMatND(*unnamed);
			return ret;
	}
	
	// upload(cv::InputArray) /usr/include/opencv2/core/cuda.hpp:466
	void cv_cuda_GpuMatND_upload_const__InputArrayR(cv::cuda::GpuMatND* instance, const cv::_InputArray* src, Result_void* ocvrs_return) {
		try {
			instance->upload(*src);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// upload(cv::InputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:467
	void cv_cuda_GpuMatND_upload_const__InputArrayR_StreamR(cv::cuda::GpuMatND* instance, const cv::_InputArray* src, cv::cuda::Stream* stream, Result_void* ocvrs_return) {
		try {
			instance->upload(*src, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// download(cv::OutputArray) /usr/include/opencv2/core/cuda.hpp:468
	void cv_cuda_GpuMatND_download_const_const__OutputArrayR(const cv::cuda::GpuMatND* instance, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			instance->download(*dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// download(cv::OutputArray, cv::cuda::Stream &) /usr/include/opencv2/core/cuda.hpp:469
	void cv_cuda_GpuMatND_download_const_const__OutputArrayR_StreamR(const cv::cuda::GpuMatND* instance, const cv::_OutputArray* dst, cv::cuda::Stream* stream, Result_void* ocvrs_return) {
		try {
			instance->download(*dst, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// isContinuous() /usr/include/opencv2/core/cuda.hpp:473
	void cv_cuda_GpuMatND_isContinuous_const(const cv::cuda::GpuMatND* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isContinuous();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isSubmatrix() /usr/include/opencv2/core/cuda.hpp:476
	void cv_cuda_GpuMatND_isSubmatrix_const(const cv::cuda::GpuMatND* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isSubmatrix();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// elemSize() /usr/include/opencv2/core/cuda.hpp:479
	void cv_cuda_GpuMatND_elemSize_const(const cv::cuda::GpuMatND* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->elemSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// elemSize1() /usr/include/opencv2/core/cuda.hpp:482
	void cv_cuda_GpuMatND_elemSize1_const(const cv::cuda::GpuMatND* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->elemSize1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// empty() /usr/include/opencv2/core/cuda.hpp:485
	void cv_cuda_GpuMatND_empty_const(const cv::cuda::GpuMatND* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// external() /usr/include/opencv2/core/cuda.hpp:488
	void cv_cuda_GpuMatND_external_const(const cv::cuda::GpuMatND* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->external();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// getDevicePtr() /usr/include/opencv2/core/cuda.hpp:491
	void cv_cuda_GpuMatND_getDevicePtr_const(const cv::cuda::GpuMatND* instance, Result<unsigned char*>* ocvrs_return) {
		try {
			unsigned char* ret = instance->getDevicePtr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned char*>))
	}
	
	// total() /usr/include/opencv2/core/cuda.hpp:494
	void cv_cuda_GpuMatND_total_const(const cv::cuda::GpuMatND* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->total();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// totalMemSize() /usr/include/opencv2/core/cuda.hpp:497
	void cv_cuda_GpuMatND_totalMemSize_const(const cv::cuda::GpuMatND* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->totalMemSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// type() /usr/include/opencv2/core/cuda.hpp:500
	void cv_cuda_GpuMatND_type_const(const cv::cuda::GpuMatND* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// flags /usr/include/opencv2/core/cuda.hpp:792
	int cv_cuda_HostMem_getPropFlags_const(const cv::cuda::HostMem* instance) {
			int ret = instance->flags;
			return ret;
	}
	
	// flags /usr/include/opencv2/core/cuda.hpp:792
	void cv_cuda_HostMem_setPropFlags_int(cv::cuda::HostMem* instance, int val) {
			instance->flags = val;
	}
	
	// rows /usr/include/opencv2/core/cuda.hpp:793
	int cv_cuda_HostMem_getPropRows_const(const cv::cuda::HostMem* instance) {
			int ret = instance->rows;
			return ret;
	}
	
	// rows /usr/include/opencv2/core/cuda.hpp:793
	void cv_cuda_HostMem_setPropRows_int(cv::cuda::HostMem* instance, int val) {
			instance->rows = val;
	}
	
	// cols /usr/include/opencv2/core/cuda.hpp:793
	int cv_cuda_HostMem_getPropCols_const(const cv::cuda::HostMem* instance) {
			int ret = instance->cols;
			return ret;
	}
	
	// cols /usr/include/opencv2/core/cuda.hpp:793
	void cv_cuda_HostMem_setPropCols_int(cv::cuda::HostMem* instance, int val) {
			instance->cols = val;
	}
	
	// step /usr/include/opencv2/core/cuda.hpp:794
	size_t cv_cuda_HostMem_getPropStep_const(const cv::cuda::HostMem* instance) {
			size_t ret = instance->step;
			return ret;
	}
	
	// step /usr/include/opencv2/core/cuda.hpp:794
	void cv_cuda_HostMem_setPropStep_size_t(cv::cuda::HostMem* instance, size_t val) {
			instance->step = val;
	}
	
	// data /usr/include/opencv2/core/cuda.hpp:796
	unsigned char* cv_cuda_HostMem_getPropData(cv::cuda::HostMem* instance) {
			unsigned char* ret = instance->data;
			return ret;
	}
	
	// data /usr/include/opencv2/core/cuda.hpp:796
	void cv_cuda_HostMem_setPropData_unsigned_charX(cv::cuda::HostMem* instance, unsigned char* val) {
			instance->data = val;
	}
	
	// refcount /usr/include/opencv2/core/cuda.hpp:797
	int* cv_cuda_HostMem_getPropRefcount(cv::cuda::HostMem* instance) {
			int* ret = instance->refcount;
			return ret;
	}
	
	// refcount /usr/include/opencv2/core/cuda.hpp:797
	void cv_cuda_HostMem_setPropRefcount_intX(cv::cuda::HostMem* instance, int* val) {
			instance->refcount = val;
	}
	
	// datastart /usr/include/opencv2/core/cuda.hpp:799
	unsigned char* cv_cuda_HostMem_getPropDatastart(cv::cuda::HostMem* instance) {
			unsigned char* ret = instance->datastart;
			return ret;
	}
	
	// datastart /usr/include/opencv2/core/cuda.hpp:799
	void cv_cuda_HostMem_setPropDatastart_unsigned_charX(cv::cuda::HostMem* instance, unsigned char* val) {
			instance->datastart = val;
	}
	
	// dataend /usr/include/opencv2/core/cuda.hpp:800
	const unsigned char* cv_cuda_HostMem_getPropDataend_const(const cv::cuda::HostMem* instance) {
			const unsigned char* ret = instance->dataend;
			return ret;
	}
	
	// alloc_type /usr/include/opencv2/core/cuda.hpp:802
	void cv_cuda_HostMem_getPropAlloc_type_const(const cv::cuda::HostMem* instance, cv::cuda::HostMem::AllocType* ocvrs_return) {
			cv::cuda::HostMem::AllocType ret = instance->alloc_type;
			*ocvrs_return = ret;
	}
	
	// alloc_type /usr/include/opencv2/core/cuda.hpp:802
	void cv_cuda_HostMem_setPropAlloc_type_AllocType(cv::cuda::HostMem* instance, cv::cuda::HostMem::AllocType val) {
			instance->alloc_type = val;
	}
	
	void cv_HostMem_delete(cv::cuda::HostMem* instance) {
		delete instance;
	}
	// HostMem(HostMem::AllocType) /usr/include/opencv2/core/cuda.hpp:737
	void cv_cuda_HostMem_HostMem_AllocType(cv::cuda::HostMem::AllocType alloc_type, Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem* ret = new cv::cuda::HostMem(alloc_type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::HostMem*>))
	}
	
	// HostMem(const cv::cuda::HostMem &) /usr/include/opencv2/core/cuda.hpp:739
	void cv_cuda_HostMem_HostMem_const_HostMemR(const cv::cuda::HostMem* m, Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem* ret = new cv::cuda::HostMem(*m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::HostMem*>))
	}
	
	// HostMem(int, int, int, HostMem::AllocType) /usr/include/opencv2/core/cuda.hpp:741
	void cv_cuda_HostMem_HostMem_int_int_int_AllocType(int rows, int cols, int type, cv::cuda::HostMem::AllocType alloc_type, Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem* ret = new cv::cuda::HostMem(rows, cols, type, alloc_type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::HostMem*>))
	}
	
	// HostMem(cv::Size, int, HostMem::AllocType) /usr/include/opencv2/core/cuda.hpp:742
	void cv_cuda_HostMem_HostMem_Size_int_AllocType(cv::Size* size, int type, cv::cuda::HostMem::AllocType alloc_type, Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem* ret = new cv::cuda::HostMem(*size, type, alloc_type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::HostMem*>))
	}
	
	// HostMem(cv::InputArray, HostMem::AllocType) /usr/include/opencv2/core/cuda.hpp:745
	void cv_cuda_HostMem_HostMem_const__InputArrayR_AllocType(const cv::_InputArray* arr, cv::cuda::HostMem::AllocType alloc_type, Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem* ret = new cv::cuda::HostMem(*arr, alloc_type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::HostMem*>))
	}
	
	// swap(cv::cuda::HostMem &) /usr/include/opencv2/core/cuda.hpp:752
	void cv_cuda_HostMem_swap_HostMemR(cv::cuda::HostMem* instance, cv::cuda::HostMem* b, Result_void* ocvrs_return) {
		try {
			instance->swap(*b);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// clone() /usr/include/opencv2/core/cuda.hpp:755
	void cv_cuda_HostMem_clone_const(const cv::cuda::HostMem* instance, Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem ret = instance->clone();
			Ok(new cv::cuda::HostMem(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::HostMem*>))
	}
	
	// create(int, int, int) /usr/include/opencv2/core/cuda.hpp:758
	void cv_cuda_HostMem_create_int_int_int(cv::cuda::HostMem* instance, int rows, int cols, int type, Result_void* ocvrs_return) {
		try {
			instance->create(rows, cols, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(cv::Size, int) /usr/include/opencv2/core/cuda.hpp:759
	void cv_cuda_HostMem_create_Size_int(cv::cuda::HostMem* instance, cv::Size* size, int type, Result_void* ocvrs_return) {
		try {
			instance->create(*size, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// reshape(int, int) /usr/include/opencv2/core/cuda.hpp:763
	void cv_cuda_HostMem_reshape_const_int_int(const cv::cuda::HostMem* instance, int cn, int rows, Result<cv::cuda::HostMem*>* ocvrs_return) {
		try {
			cv::cuda::HostMem ret = instance->reshape(cn, rows);
			Ok(new cv::cuda::HostMem(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::HostMem*>))
	}
	
	// release() /usr/include/opencv2/core/cuda.hpp:766
	void cv_cuda_HostMem_release(cv::cuda::HostMem* instance, Result_void* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// createMatHeader() /usr/include/opencv2/core/cuda.hpp:769
	void cv_cuda_HostMem_createMatHeader_const(const cv::cuda::HostMem* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->createMatHeader();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// createGpuMatHeader() /usr/include/opencv2/core/cuda.hpp:778
	void cv_cuda_HostMem_createGpuMatHeader_const(const cv::cuda::HostMem* instance, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->createGpuMatHeader();
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// isContinuous() /usr/include/opencv2/core/cuda.hpp:781
	void cv_cuda_HostMem_isContinuous_const(const cv::cuda::HostMem* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isContinuous();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// elemSize() /usr/include/opencv2/core/cuda.hpp:782
	void cv_cuda_HostMem_elemSize_const(const cv::cuda::HostMem* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->elemSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// elemSize1() /usr/include/opencv2/core/cuda.hpp:783
	void cv_cuda_HostMem_elemSize1_const(const cv::cuda::HostMem* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->elemSize1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// type() /usr/include/opencv2/core/cuda.hpp:784
	void cv_cuda_HostMem_type_const(const cv::cuda::HostMem* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// depth() /usr/include/opencv2/core/cuda.hpp:785
	void cv_cuda_HostMem_depth_const(const cv::cuda::HostMem* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->depth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// channels() /usr/include/opencv2/core/cuda.hpp:786
	void cv_cuda_HostMem_channels_const(const cv::cuda::HostMem* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->channels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// step1() /usr/include/opencv2/core/cuda.hpp:787
	void cv_cuda_HostMem_step1_const(const cv::cuda::HostMem* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->step1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// size() /usr/include/opencv2/core/cuda.hpp:788
	void cv_cuda_HostMem_size_const(const cv::cuda::HostMem* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// empty() /usr/include/opencv2/core/cuda.hpp:789
	void cv_cuda_HostMem_empty_const(const cv::cuda::HostMem* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_Stream_delete(cv::cuda::Stream* instance) {
		delete instance;
	}
	// Stream() /usr/include/opencv2/core/cuda.hpp:857
	void cv_cuda_Stream_Stream(Result<cv::cuda::Stream*>* ocvrs_return) {
		try {
			cv::cuda::Stream* ret = new cv::cuda::Stream();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::Stream*>))
	}
	
	// Stream(const Ptr<GpuMat::Allocator> &) /usr/include/opencv2/core/cuda.hpp:860
	void cv_cuda_Stream_Stream_const_Ptr_Allocator_R(const cv::Ptr<cv::cuda::GpuMat::Allocator>* allocator, Result<cv::cuda::Stream*>* ocvrs_return) {
		try {
			cv::cuda::Stream* ret = new cv::cuda::Stream(*allocator);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::Stream*>))
	}
	
	// Stream(const size_t) /usr/include/opencv2/core/cuda.hpp:872
	void cv_cuda_Stream_Stream_const_size_t(const size_t cudaFlags, Result<cv::cuda::Stream*>* ocvrs_return) {
		try {
			cv::cuda::Stream* ret = new cv::cuda::Stream(cudaFlags);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::Stream*>))
	}
	
	// queryIfComplete() /usr/include/opencv2/core/cuda.hpp:876
	void cv_cuda_Stream_queryIfComplete_const(const cv::cuda::Stream* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->queryIfComplete();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// waitForCompletion() /usr/include/opencv2/core/cuda.hpp:880
	void cv_cuda_Stream_waitForCompletion(cv::cuda::Stream* instance, Result_void* ocvrs_return) {
		try {
			instance->waitForCompletion();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// waitEvent(const cv::cuda::Event &) /usr/include/opencv2/core/cuda.hpp:884
	void cv_cuda_Stream_waitEvent_const_EventR(cv::cuda::Stream* instance, const cv::cuda::Event* event, Result_void* ocvrs_return) {
		try {
			instance->waitEvent(*event);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// enqueueHostCallback(cv::cuda::Stream::StreamCallback, void *) /usr/include/opencv2/core/cuda.hpp:894
	void cv_cuda_Stream_enqueueHostCallback_StreamCallback_voidX(cv::cuda::Stream* instance, cv::cuda::Stream::StreamCallback callback, void* userData, Result_void* ocvrs_return) {
		try {
			instance->enqueueHostCallback(callback, userData);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// Null() /usr/include/opencv2/core/cuda.hpp:897
	void cv_cuda_Stream_Null(Result<cv::cuda::Stream*>* ocvrs_return) {
		try {
			cv::cuda::Stream ret = cv::cuda::Stream::Null();
			Ok(new cv::cuda::Stream(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::Stream*>))
	}
	
	// cudaPtr() /usr/include/opencv2/core/cuda.hpp:903
	void cv_cuda_Stream_cudaPtr_const(const cv::cuda::Stream* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->cudaPtr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	void cv_TargetArchs_delete(cv::cuda::TargetArchs* instance) {
		delete instance;
	}
	// builtWith(cv::cuda::FeatureSet) /usr/include/opencv2/core/cuda.hpp:1025
	void cv_cuda_TargetArchs_builtWith_FeatureSet(cv::cuda::FeatureSet feature_set, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::cuda::TargetArchs::builtWith(feature_set);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// has(int, int) /usr/include/opencv2/core/cuda.hpp:1033
	void cv_cuda_TargetArchs_has_int_int(int major, int minor, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::cuda::TargetArchs::has(major, minor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// hasPtx(int, int) /usr/include/opencv2/core/cuda.hpp:1034
	void cv_cuda_TargetArchs_hasPtx_int_int(int major, int minor, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::cuda::TargetArchs::hasPtx(major, minor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// hasBin(int, int) /usr/include/opencv2/core/cuda.hpp:1035
	void cv_cuda_TargetArchs_hasBin_int_int(int major, int minor, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::cuda::TargetArchs::hasBin(major, minor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// hasEqualOrLessPtx(int, int) /usr/include/opencv2/core/cuda.hpp:1037
	void cv_cuda_TargetArchs_hasEqualOrLessPtx_int_int(int major, int minor, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::cuda::TargetArchs::hasEqualOrLessPtx(major, minor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// hasEqualOrGreater(int, int) /usr/include/opencv2/core/cuda.hpp:1038
	void cv_cuda_TargetArchs_hasEqualOrGreater_int_int(int major, int minor, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::cuda::TargetArchs::hasEqualOrGreater(major, minor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// hasEqualOrGreaterPtx(int, int) /usr/include/opencv2/core/cuda.hpp:1039
	void cv_cuda_TargetArchs_hasEqualOrGreaterPtx_int_int(int major, int minor, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::cuda::TargetArchs::hasEqualOrGreaterPtx(major, minor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// hasEqualOrGreaterBin(int, int) /usr/include/opencv2/core/cuda.hpp:1040
	void cv_cuda_TargetArchs_hasEqualOrGreaterBin_int_int(int major, int minor, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::cuda::TargetArchs::hasEqualOrGreaterBin(major, minor);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// func /usr/include/opencv2/core/check.hpp:40
	void* cv_detail_CheckContext_getPropFunc_const(const cv::detail::CheckContext* instance) {
			const char* ret = instance->func;
			return ocvrs_create_string(ret);
	}
	
	// file /usr/include/opencv2/core/check.hpp:41
	void* cv_detail_CheckContext_getPropFile_const(const cv::detail::CheckContext* instance) {
			const char* ret = instance->file;
			return ocvrs_create_string(ret);
	}
	
	// line /usr/include/opencv2/core/check.hpp:42
	int cv_detail_CheckContext_getPropLine_const(const cv::detail::CheckContext* instance) {
			int ret = instance->line;
			return ret;
	}
	
	// line /usr/include/opencv2/core/check.hpp:42
	void cv_detail_CheckContext_setPropLine_int(cv::detail::CheckContext* instance, int val) {
			instance->line = val;
	}
	
	// testOp /usr/include/opencv2/core/check.hpp:43
	void cv_detail_CheckContext_getPropTestOp_const(const cv::detail::CheckContext* instance, cv::detail::TestOp* ocvrs_return) {
			cv::detail::TestOp ret = instance->testOp;
			*ocvrs_return = ret;
	}
	
	// testOp /usr/include/opencv2/core/check.hpp:43
	void cv_detail_CheckContext_setPropTestOp_TestOp(cv::detail::CheckContext* instance, cv::detail::TestOp val) {
			instance->testOp = val;
	}
	
	// message /usr/include/opencv2/core/check.hpp:44
	void* cv_detail_CheckContext_getPropMessage_const(const cv::detail::CheckContext* instance) {
			const char* ret = instance->message;
			return ocvrs_create_string(ret);
	}
	
	// p1_str /usr/include/opencv2/core/check.hpp:45
	void* cv_detail_CheckContext_getPropP1_str_const(const cv::detail::CheckContext* instance) {
			const char* ret = instance->p1_str;
			return ocvrs_create_string(ret);
	}
	
	// p2_str /usr/include/opencv2/core/check.hpp:46
	void* cv_detail_CheckContext_getPropP2_str_const(const cv::detail::CheckContext* instance) {
			const char* ret = instance->p2_str;
			return ocvrs_create_string(ret);
	}
	
	void cv_Detail_CheckContext_delete(cv::detail::CheckContext* instance) {
		delete instance;
	}
	// m_funName /usr/include/opencv2/core/utils/instrumentation.hpp:77
	void* cv_instr_NodeData_getPropM_funName_const(const cv::instr::NodeData* instance) {
			cv::String ret = instance->m_funName;
			return ocvrs_create_string(ret.c_str());
	}
	
	// m_funName /usr/include/opencv2/core/utils/instrumentation.hpp:77
	void cv_instr_NodeData_setPropM_funName_String(cv::instr::NodeData* instance, char* val) {
			instance->m_funName = std::string(val);
	}
	
	// m_instrType /usr/include/opencv2/core/utils/instrumentation.hpp:78
	void cv_instr_NodeData_getPropM_instrType_const(const cv::instr::NodeData* instance, cv::instr::TYPE* ocvrs_return) {
			cv::instr::TYPE ret = instance->m_instrType;
			*ocvrs_return = ret;
	}
	
	// m_instrType /usr/include/opencv2/core/utils/instrumentation.hpp:78
	void cv_instr_NodeData_setPropM_instrType_TYPE(cv::instr::NodeData* instance, cv::instr::TYPE val) {
			instance->m_instrType = val;
	}
	
	// m_implType /usr/include/opencv2/core/utils/instrumentation.hpp:79
	void cv_instr_NodeData_getPropM_implType_const(const cv::instr::NodeData* instance, cv::instr::IMPL* ocvrs_return) {
			cv::instr::IMPL ret = instance->m_implType;
			*ocvrs_return = ret;
	}
	
	// m_implType /usr/include/opencv2/core/utils/instrumentation.hpp:79
	void cv_instr_NodeData_setPropM_implType_IMPL(cv::instr::NodeData* instance, cv::instr::IMPL val) {
			instance->m_implType = val;
	}
	
	// m_fileName /usr/include/opencv2/core/utils/instrumentation.hpp:80
	void* cv_instr_NodeData_getPropM_fileName_const(const cv::instr::NodeData* instance) {
			const char* ret = instance->m_fileName;
			return ocvrs_create_string(ret);
	}
	
	// m_lineNum /usr/include/opencv2/core/utils/instrumentation.hpp:81
	int cv_instr_NodeData_getPropM_lineNum_const(const cv::instr::NodeData* instance) {
			int ret = instance->m_lineNum;
			return ret;
	}
	
	// m_lineNum /usr/include/opencv2/core/utils/instrumentation.hpp:81
	void cv_instr_NodeData_setPropM_lineNum_int(cv::instr::NodeData* instance, int val) {
			instance->m_lineNum = val;
	}
	
	// m_retAddress /usr/include/opencv2/core/utils/instrumentation.hpp:82
	void* cv_instr_NodeData_getPropM_retAddress(cv::instr::NodeData* instance) {
			void* ret = instance->m_retAddress;
			return ret;
	}
	
	// m_retAddress /usr/include/opencv2/core/utils/instrumentation.hpp:82
	void cv_instr_NodeData_setPropM_retAddress_voidX(cv::instr::NodeData* instance, void* val) {
			instance->m_retAddress = val;
	}
	
	// m_alwaysExpand /usr/include/opencv2/core/utils/instrumentation.hpp:83
	bool cv_instr_NodeData_getPropM_alwaysExpand_const(const cv::instr::NodeData* instance) {
			bool ret = instance->m_alwaysExpand;
			return ret;
	}
	
	// m_alwaysExpand /usr/include/opencv2/core/utils/instrumentation.hpp:83
	void cv_instr_NodeData_setPropM_alwaysExpand_bool(cv::instr::NodeData* instance, bool val) {
			instance->m_alwaysExpand = val;
	}
	
	// m_funError /usr/include/opencv2/core/utils/instrumentation.hpp:84
	bool cv_instr_NodeData_getPropM_funError_const(const cv::instr::NodeData* instance) {
			bool ret = instance->m_funError;
			return ret;
	}
	
	// m_funError /usr/include/opencv2/core/utils/instrumentation.hpp:84
	void cv_instr_NodeData_setPropM_funError_bool(cv::instr::NodeData* instance, bool val) {
			instance->m_funError = val;
	}
	
	// m_counter /usr/include/opencv2/core/utils/instrumentation.hpp:86
	int cv_instr_NodeData_getPropM_counter_const(const cv::instr::NodeData* instance) {
			int ret = instance->m_counter;
			return ret;
	}
	
	// m_counter /usr/include/opencv2/core/utils/instrumentation.hpp:86
	void cv_instr_NodeData_setPropM_counter_int(cv::instr::NodeData* instance, int val) {
			instance->m_counter = val;
	}
	
	// m_ticksTotal /usr/include/opencv2/core/utils/instrumentation.hpp:87
	uint64_t cv_instr_NodeData_getPropM_ticksTotal_const(const cv::instr::NodeData* instance) {
			uint64_t ret = instance->m_ticksTotal;
			return ret;
	}
	
	// m_ticksTotal /usr/include/opencv2/core/utils/instrumentation.hpp:87
	void cv_instr_NodeData_setPropM_ticksTotal_uint64_t(cv::instr::NodeData* instance, uint64_t val) {
			instance->m_ticksTotal = val;
	}
	
	// m_threads /usr/include/opencv2/core/utils/instrumentation.hpp:89
	int cv_instr_NodeData_getPropM_threads_const(const cv::instr::NodeData* instance) {
			int ret = instance->m_threads;
			return ret;
	}
	
	// m_threads /usr/include/opencv2/core/utils/instrumentation.hpp:89
	void cv_instr_NodeData_setPropM_threads_int(cv::instr::NodeData* instance, int val) {
			instance->m_threads = val;
	}
	
	void cv_NodeData_delete(cv::instr::NodeData* instance) {
		delete instance;
	}
	// NodeData(const char *, const char *, int, void *, bool, cv::instr::TYPE, cv::instr::IMPL) /usr/include/opencv2/core/utils/instrumentation.hpp:72
	void cv_instr_NodeData_NodeData_const_charX_const_charX_int_voidX_bool_TYPE_IMPL(const char* funName, const char* fileName, int lineNum, void* retAddress, bool alwaysExpand, cv::instr::TYPE instrType, cv::instr::IMPL implType, Result<cv::instr::NodeData*>* ocvrs_return) {
		try {
			cv::instr::NodeData* ret = new cv::instr::NodeData(funName, fileName, lineNum, retAddress, alwaysExpand, instrType, implType);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::instr::NodeData*>))
	}
	
	// NodeData(cv::instr::NodeData &) /usr/include/opencv2/core/utils/instrumentation.hpp:73
	void cv_instr_NodeData_NodeData_NodeDataR(cv::instr::NodeData* ref, Result<cv::instr::NodeData*>* ocvrs_return) {
		try {
			cv::instr::NodeData* ret = new cv::instr::NodeData(*ref);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::instr::NodeData*>))
	}
	
	// getTotalMs() /usr/include/opencv2/core/utils/instrumentation.hpp:92
	void cv_instr_NodeData_getTotalMs_const(const cv::instr::NodeData* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getTotalMs();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// getMeanMs() /usr/include/opencv2/core/utils/instrumentation.hpp:93
	void cv_instr_NodeData_getMeanMs_const(const cv::instr::NodeData* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getMeanMs();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	void cv_WriteStructContext_delete(cv::internal::WriteStructContext* instance) {
		delete instance;
	}
	// WriteStructContext(cv::FileStorage &, const cv::String &, int, const cv::String &) /usr/include/opencv2/core/persistence.hpp:813
	void cv_internal_WriteStructContext_WriteStructContext_FileStorageR_const_StringR_int_const_StringR(cv::FileStorage* _fs, const char* name, int flags, const char* typeName, Result<cv::internal::WriteStructContext*>* ocvrs_return) {
		try {
			cv::internal::WriteStructContext* ret = new cv::internal::WriteStructContext(*_fs, std::string(name), flags, std::string(typeName));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::internal::WriteStructContext*>))
	}
	
	void cv_Context_delete(cv::ocl::Context* instance) {
		delete instance;
	}
	// Context() /usr/include/opencv2/core/ocl.hpp:256
	cv::ocl::Context* cv_ocl_Context_Context() {
			cv::ocl::Context* ret = new cv::ocl::Context();
			return ret;
	}
	
	// Context(int) /usr/include/opencv2/core/ocl.hpp:257
	void cv_ocl_Context_Context_int(int dtype, Result<cv::ocl::Context*>* ocvrs_return) {
		try {
			cv::ocl::Context* ret = new cv::ocl::Context(dtype);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Context*>))
	}
	
	// Context(const cv::ocl::Context &) /usr/include/opencv2/core/ocl.hpp:259
	void cv_ocl_Context_Context_const_ContextR(const cv::ocl::Context* c, Result<cv::ocl::Context*>* ocvrs_return) {
		try {
			cv::ocl::Context* ret = new cv::ocl::Context(*c);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Context*>))
	}
	
	// Context(cv::ocl::Context &&) /usr/include/opencv2/core/ocl.hpp:261
	cv::ocl::Context* cv_ocl_Context_Context_ContextR(cv::ocl::Context* c) {
			cv::ocl::Context* ret = new cv::ocl::Context(*c);
			return ret;
	}
	
	// create() /usr/include/opencv2/core/ocl.hpp:265
	void cv_ocl_Context_create(cv::ocl::Context* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->create();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// create(int) /usr/include/opencv2/core/ocl.hpp:267
	void cv_ocl_Context_create_int(cv::ocl::Context* instance, int dtype, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->create(dtype);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// ndevices() /usr/include/opencv2/core/ocl.hpp:269
	void cv_ocl_Context_ndevices_const(const cv::ocl::Context* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->ndevices();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// device(size_t) /usr/include/opencv2/core/ocl.hpp:270
	void cv_ocl_Context_device_const_size_t(const cv::ocl::Context* instance, size_t idx, Result<cv::ocl::Device*>* ocvrs_return) {
		try {
			cv::ocl::Device ret = instance->device(idx);
			Ok(new cv::ocl::Device(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Device*>))
	}
	
	// getProg(const cv::ocl::ProgramSource &, const cv::String &, cv::String &) /usr/include/opencv2/core/ocl.hpp:271
	void cv_ocl_Context_getProg_const_ProgramSourceR_const_StringR_StringR(cv::ocl::Context* instance, const cv::ocl::ProgramSource* prog, const char* buildopt, void** errmsg, Result<cv::ocl::Program*>* ocvrs_return) {
		try {
			std::string errmsg_out;
			cv::ocl::Program ret = instance->getProg(*prog, std::string(buildopt), errmsg_out);
			*errmsg = ocvrs_create_string(errmsg_out.c_str());
			Ok(new cv::ocl::Program(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Program*>))
	}
	
	// unloadProg(cv::ocl::Program &) /usr/include/opencv2/core/ocl.hpp:273
	void cv_ocl_Context_unloadProg_ProgramR(cv::ocl::Context* instance, cv::ocl::Program* prog, Result_void* ocvrs_return) {
		try {
			instance->unloadProg(*prog);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDefault(bool) /usr/include/opencv2/core/ocl.hpp:280
	void cv_ocl_Context_getDefault_bool(bool initialize, Result<cv::ocl::Context*>* ocvrs_return) {
		try {
			cv::ocl::Context ret = cv::ocl::Context::getDefault(initialize);
			Ok(new cv::ocl::Context(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Context*>))
	}
	
	// ptr() /usr/include/opencv2/core/ocl.hpp:284
	void cv_ocl_Context_ptr_const(const cv::ocl::Context* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->ptr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// getOpenCLContextProperty(int) /usr/include/opencv2/core/ocl.hpp:291
	void cv_ocl_Context_getOpenCLContextProperty_const_int(const cv::ocl::Context* instance, int propertyId, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->getOpenCLContextProperty(propertyId);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// useSVM() /usr/include/opencv2/core/ocl.hpp:293
	void cv_ocl_Context_useSVM_const(const cv::ocl::Context* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->useSVM();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setUseSVM(bool) /usr/include/opencv2/core/ocl.hpp:294
	void cv_ocl_Context_setUseSVM_bool(cv::ocl::Context* instance, bool enabled, Result_void* ocvrs_return) {
		try {
			instance->setUseSVM(enabled);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// fromHandle(void *) /usr/include/opencv2/core/ocl.hpp:299
	void cv_ocl_Context_fromHandle_voidX(void* context, Result<cv::ocl::Context*>* ocvrs_return) {
		try {
			cv::ocl::Context ret = cv::ocl::Context::fromHandle(context);
			Ok(new cv::ocl::Context(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Context*>))
	}
	
	// fromDevice(const ocl::Device &) /usr/include/opencv2/core/ocl.hpp:300
	void cv_ocl_Context_fromDevice_const_DeviceR(const cv::ocl::Device* device, Result<cv::ocl::Context*>* ocvrs_return) {
		try {
			cv::ocl::Context ret = cv::ocl::Context::fromDevice(*device);
			Ok(new cv::ocl::Context(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Context*>))
	}
	
	// create(const std::string &) /usr/include/opencv2/core/ocl.hpp:301
	void cv_ocl_Context_create_const_stringR(const char* configuration, Result<cv::ocl::Context*>* ocvrs_return) {
		try {
			cv::ocl::Context ret = cv::ocl::Context::create(std::string(configuration));
			Ok(new cv::ocl::Context(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Context*>))
	}
	
	// release() /usr/include/opencv2/core/ocl.hpp:303
	void cv_ocl_Context_release(cv::ocl::Context* instance, Result_void* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// empty() /usr/include/opencv2/core/ocl.hpp:322
	void cv_ocl_Context_empty_const(const cv::ocl::Context* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_Context_UserContext_delete(cv::ocl::Context::UserContext* instance) {
		delete instance;
	}
	void cv_Device_delete(cv::ocl::Device* instance) {
		delete instance;
	}
	// Device() /usr/include/opencv2/core/ocl.hpp:75
	cv::ocl::Device* cv_ocl_Device_Device() {
			cv::ocl::Device* ret = new cv::ocl::Device();
			return ret;
	}
	
	// Device(void *) /usr/include/opencv2/core/ocl.hpp:76
	void cv_ocl_Device_Device_voidX(void* d, Result<cv::ocl::Device*>* ocvrs_return) {
		try {
			cv::ocl::Device* ret = new cv::ocl::Device(d);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Device*>))
	}
	
	// Device(const cv::ocl::Device &) /usr/include/opencv2/core/ocl.hpp:77
	void cv_ocl_Device_Device_const_DeviceR(const cv::ocl::Device* d, Result<cv::ocl::Device*>* ocvrs_return) {
		try {
			cv::ocl::Device* ret = new cv::ocl::Device(*d);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Device*>))
	}
	
	// Device(cv::ocl::Device &&) /usr/include/opencv2/core/ocl.hpp:79
	cv::ocl::Device* cv_ocl_Device_Device_DeviceR(cv::ocl::Device* d) {
			cv::ocl::Device* ret = new cv::ocl::Device(*d);
			return ret;
	}
	
	// set(void *) /usr/include/opencv2/core/ocl.hpp:83
	void cv_ocl_Device_set_voidX(cv::ocl::Device* instance, void* d, Result_void* ocvrs_return) {
		try {
			instance->set(d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// name() /usr/include/opencv2/core/ocl.hpp:96
	void cv_ocl_Device_name_const(const cv::ocl::Device* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->name();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// extensions() /usr/include/opencv2/core/ocl.hpp:97
	void cv_ocl_Device_extensions_const(const cv::ocl::Device* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->extensions();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// isExtensionSupported(const cv::String &) /usr/include/opencv2/core/ocl.hpp:98
	void cv_ocl_Device_isExtensionSupported_const_const_StringR(const cv::ocl::Device* instance, const char* extensionName, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isExtensionSupported(std::string(extensionName));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// version() /usr/include/opencv2/core/ocl.hpp:99
	void cv_ocl_Device_version_const(const cv::ocl::Device* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->version();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// vendorName() /usr/include/opencv2/core/ocl.hpp:100
	void cv_ocl_Device_vendorName_const(const cv::ocl::Device* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->vendorName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// OpenCL_C_Version() /usr/include/opencv2/core/ocl.hpp:101
	void cv_ocl_Device_OpenCL_C_Version_const(const cv::ocl::Device* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->OpenCL_C_Version();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// OpenCLVersion() /usr/include/opencv2/core/ocl.hpp:102
	void cv_ocl_Device_OpenCLVersion_const(const cv::ocl::Device* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->OpenCLVersion();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// deviceVersionMajor() /usr/include/opencv2/core/ocl.hpp:103
	void cv_ocl_Device_deviceVersionMajor_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->deviceVersionMajor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// deviceVersionMinor() /usr/include/opencv2/core/ocl.hpp:104
	void cv_ocl_Device_deviceVersionMinor_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->deviceVersionMinor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// driverVersion() /usr/include/opencv2/core/ocl.hpp:105
	void cv_ocl_Device_driverVersion_const(const cv::ocl::Device* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->driverVersion();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// ptr() /usr/include/opencv2/core/ocl.hpp:106
	void cv_ocl_Device_ptr_const(const cv::ocl::Device* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->ptr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// type() /usr/include/opencv2/core/ocl.hpp:108
	void cv_ocl_Device_type_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// addressBits() /usr/include/opencv2/core/ocl.hpp:110
	void cv_ocl_Device_addressBits_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addressBits();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// available() /usr/include/opencv2/core/ocl.hpp:111
	void cv_ocl_Device_available_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->available();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// compilerAvailable() /usr/include/opencv2/core/ocl.hpp:112
	void cv_ocl_Device_compilerAvailable_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->compilerAvailable();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// linkerAvailable() /usr/include/opencv2/core/ocl.hpp:113
	void cv_ocl_Device_linkerAvailable_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->linkerAvailable();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// doubleFPConfig() /usr/include/opencv2/core/ocl.hpp:126
	void cv_ocl_Device_doubleFPConfig_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->doubleFPConfig();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// singleFPConfig() /usr/include/opencv2/core/ocl.hpp:127
	void cv_ocl_Device_singleFPConfig_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->singleFPConfig();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// halfFPConfig() /usr/include/opencv2/core/ocl.hpp:128
	void cv_ocl_Device_halfFPConfig_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->halfFPConfig();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// endianLittle() /usr/include/opencv2/core/ocl.hpp:130
	void cv_ocl_Device_endianLittle_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->endianLittle();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// errorCorrectionSupport() /usr/include/opencv2/core/ocl.hpp:131
	void cv_ocl_Device_errorCorrectionSupport_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->errorCorrectionSupport();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// executionCapabilities() /usr/include/opencv2/core/ocl.hpp:138
	void cv_ocl_Device_executionCapabilities_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->executionCapabilities();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// globalMemCacheSize() /usr/include/opencv2/core/ocl.hpp:140
	void cv_ocl_Device_globalMemCacheSize_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->globalMemCacheSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// globalMemCacheType() /usr/include/opencv2/core/ocl.hpp:148
	void cv_ocl_Device_globalMemCacheType_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->globalMemCacheType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// globalMemCacheLineSize() /usr/include/opencv2/core/ocl.hpp:149
	void cv_ocl_Device_globalMemCacheLineSize_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->globalMemCacheLineSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// globalMemSize() /usr/include/opencv2/core/ocl.hpp:150
	void cv_ocl_Device_globalMemSize_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->globalMemSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// localMemSize() /usr/include/opencv2/core/ocl.hpp:152
	void cv_ocl_Device_localMemSize_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->localMemSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// localMemType() /usr/include/opencv2/core/ocl.hpp:159
	void cv_ocl_Device_localMemType_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->localMemType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// hostUnifiedMemory() /usr/include/opencv2/core/ocl.hpp:160
	void cv_ocl_Device_hostUnifiedMemory_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->hostUnifiedMemory();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// imageSupport() /usr/include/opencv2/core/ocl.hpp:162
	void cv_ocl_Device_imageSupport_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->imageSupport();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// imageFromBufferSupport() /usr/include/opencv2/core/ocl.hpp:164
	void cv_ocl_Device_imageFromBufferSupport_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->imageFromBufferSupport();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// imagePitchAlignment() /usr/include/opencv2/core/ocl.hpp:165
	void cv_ocl_Device_imagePitchAlignment_const(const cv::ocl::Device* instance, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->imagePitchAlignment();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	// imageBaseAddressAlignment() /usr/include/opencv2/core/ocl.hpp:166
	void cv_ocl_Device_imageBaseAddressAlignment_const(const cv::ocl::Device* instance, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->imageBaseAddressAlignment();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	// intelSubgroupsSupport() /usr/include/opencv2/core/ocl.hpp:169
	void cv_ocl_Device_intelSubgroupsSupport_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->intelSubgroupsSupport();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// image2DMaxWidth() /usr/include/opencv2/core/ocl.hpp:171
	void cv_ocl_Device_image2DMaxWidth_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->image2DMaxWidth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// image2DMaxHeight() /usr/include/opencv2/core/ocl.hpp:172
	void cv_ocl_Device_image2DMaxHeight_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->image2DMaxHeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// image3DMaxWidth() /usr/include/opencv2/core/ocl.hpp:174
	void cv_ocl_Device_image3DMaxWidth_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->image3DMaxWidth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// image3DMaxHeight() /usr/include/opencv2/core/ocl.hpp:175
	void cv_ocl_Device_image3DMaxHeight_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->image3DMaxHeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// image3DMaxDepth() /usr/include/opencv2/core/ocl.hpp:176
	void cv_ocl_Device_image3DMaxDepth_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->image3DMaxDepth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// imageMaxBufferSize() /usr/include/opencv2/core/ocl.hpp:178
	void cv_ocl_Device_imageMaxBufferSize_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->imageMaxBufferSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// imageMaxArraySize() /usr/include/opencv2/core/ocl.hpp:179
	void cv_ocl_Device_imageMaxArraySize_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->imageMaxArraySize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// vendorID() /usr/include/opencv2/core/ocl.hpp:188
	void cv_ocl_Device_vendorID_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->vendorID();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// isAMD() /usr/include/opencv2/core/ocl.hpp:193
	void cv_ocl_Device_isAMD_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isAMD();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isIntel() /usr/include/opencv2/core/ocl.hpp:194
	void cv_ocl_Device_isIntel_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isIntel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isNVidia() /usr/include/opencv2/core/ocl.hpp:195
	void cv_ocl_Device_isNVidia_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isNVidia();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// maxClockFrequency() /usr/include/opencv2/core/ocl.hpp:197
	void cv_ocl_Device_maxClockFrequency_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxClockFrequency();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// maxComputeUnits() /usr/include/opencv2/core/ocl.hpp:198
	void cv_ocl_Device_maxComputeUnits_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxComputeUnits();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// maxConstantArgs() /usr/include/opencv2/core/ocl.hpp:199
	void cv_ocl_Device_maxConstantArgs_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxConstantArgs();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// maxConstantBufferSize() /usr/include/opencv2/core/ocl.hpp:200
	void cv_ocl_Device_maxConstantBufferSize_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->maxConstantBufferSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// maxMemAllocSize() /usr/include/opencv2/core/ocl.hpp:202
	void cv_ocl_Device_maxMemAllocSize_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->maxMemAllocSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// maxParameterSize() /usr/include/opencv2/core/ocl.hpp:203
	void cv_ocl_Device_maxParameterSize_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->maxParameterSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// maxReadImageArgs() /usr/include/opencv2/core/ocl.hpp:205
	void cv_ocl_Device_maxReadImageArgs_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxReadImageArgs();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// maxWriteImageArgs() /usr/include/opencv2/core/ocl.hpp:206
	void cv_ocl_Device_maxWriteImageArgs_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxWriteImageArgs();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// maxSamplers() /usr/include/opencv2/core/ocl.hpp:207
	void cv_ocl_Device_maxSamplers_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxSamplers();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// maxWorkGroupSize() /usr/include/opencv2/core/ocl.hpp:209
	void cv_ocl_Device_maxWorkGroupSize_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->maxWorkGroupSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// maxWorkItemDims() /usr/include/opencv2/core/ocl.hpp:210
	void cv_ocl_Device_maxWorkItemDims_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->maxWorkItemDims();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// maxWorkItemSizes(size_t *) /usr/include/opencv2/core/ocl.hpp:211
	void cv_ocl_Device_maxWorkItemSizes_const_size_tX(const cv::ocl::Device* instance, size_t* unnamed, Result_void* ocvrs_return) {
		try {
			instance->maxWorkItemSizes(unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// memBaseAddrAlign() /usr/include/opencv2/core/ocl.hpp:213
	void cv_ocl_Device_memBaseAddrAlign_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->memBaseAddrAlign();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// nativeVectorWidthChar() /usr/include/opencv2/core/ocl.hpp:215
	void cv_ocl_Device_nativeVectorWidthChar_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->nativeVectorWidthChar();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// nativeVectorWidthShort() /usr/include/opencv2/core/ocl.hpp:216
	void cv_ocl_Device_nativeVectorWidthShort_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->nativeVectorWidthShort();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// nativeVectorWidthInt() /usr/include/opencv2/core/ocl.hpp:217
	void cv_ocl_Device_nativeVectorWidthInt_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->nativeVectorWidthInt();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// nativeVectorWidthLong() /usr/include/opencv2/core/ocl.hpp:218
	void cv_ocl_Device_nativeVectorWidthLong_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->nativeVectorWidthLong();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// nativeVectorWidthFloat() /usr/include/opencv2/core/ocl.hpp:219
	void cv_ocl_Device_nativeVectorWidthFloat_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->nativeVectorWidthFloat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// nativeVectorWidthDouble() /usr/include/opencv2/core/ocl.hpp:220
	void cv_ocl_Device_nativeVectorWidthDouble_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->nativeVectorWidthDouble();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// nativeVectorWidthHalf() /usr/include/opencv2/core/ocl.hpp:221
	void cv_ocl_Device_nativeVectorWidthHalf_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->nativeVectorWidthHalf();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// preferredVectorWidthChar() /usr/include/opencv2/core/ocl.hpp:223
	void cv_ocl_Device_preferredVectorWidthChar_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->preferredVectorWidthChar();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// preferredVectorWidthShort() /usr/include/opencv2/core/ocl.hpp:224
	void cv_ocl_Device_preferredVectorWidthShort_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->preferredVectorWidthShort();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// preferredVectorWidthInt() /usr/include/opencv2/core/ocl.hpp:225
	void cv_ocl_Device_preferredVectorWidthInt_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->preferredVectorWidthInt();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// preferredVectorWidthLong() /usr/include/opencv2/core/ocl.hpp:226
	void cv_ocl_Device_preferredVectorWidthLong_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->preferredVectorWidthLong();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// preferredVectorWidthFloat() /usr/include/opencv2/core/ocl.hpp:227
	void cv_ocl_Device_preferredVectorWidthFloat_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->preferredVectorWidthFloat();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// preferredVectorWidthDouble() /usr/include/opencv2/core/ocl.hpp:228
	void cv_ocl_Device_preferredVectorWidthDouble_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->preferredVectorWidthDouble();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// preferredVectorWidthHalf() /usr/include/opencv2/core/ocl.hpp:229
	void cv_ocl_Device_preferredVectorWidthHalf_const(const cv::ocl::Device* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->preferredVectorWidthHalf();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// printfBufferSize() /usr/include/opencv2/core/ocl.hpp:231
	void cv_ocl_Device_printfBufferSize_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->printfBufferSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// profilingTimerResolution() /usr/include/opencv2/core/ocl.hpp:232
	void cv_ocl_Device_profilingTimerResolution_const(const cv::ocl::Device* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->profilingTimerResolution();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// getDefault() /usr/include/opencv2/core/ocl.hpp:234
	void cv_ocl_Device_getDefault(Result<cv::ocl::Device*>* ocvrs_return) {
		try {
			const cv::ocl::Device ret = cv::ocl::Device::getDefault();
			Ok(new const cv::ocl::Device(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Device*>))
	}
	
	// fromHandle(void *) /usr/include/opencv2/core/ocl.hpp:243
	void cv_ocl_Device_fromHandle_voidX(void* d, Result<cv::ocl::Device*>* ocvrs_return) {
		try {
			cv::ocl::Device ret = cv::ocl::Device::fromHandle(d);
			Ok(new cv::ocl::Device(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Device*>))
	}
	
	// empty() /usr/include/opencv2/core/ocl.hpp:247
	void cv_ocl_Device_empty_const(const cv::ocl::Device* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_Image2D_delete(cv::ocl::Image2D* instance) {
		delete instance;
	}
	// Image2D() /usr/include/opencv2/core/ocl.hpp:742
	cv::ocl::Image2D* cv_ocl_Image2D_Image2D() {
			cv::ocl::Image2D* ret = new cv::ocl::Image2D();
			return ret;
	}
	
	// Image2D(const cv::UMat &, bool, bool) /usr/include/opencv2/core/ocl.hpp:750
	void cv_ocl_Image2D_Image2D_const_UMatR_bool_bool(const cv::UMat* src, bool norm, bool alias, Result<cv::ocl::Image2D*>* ocvrs_return) {
		try {
			cv::ocl::Image2D* ret = new cv::ocl::Image2D(*src, norm, alias);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Image2D*>))
	}
	
	// Image2D(const cv::ocl::Image2D &) /usr/include/opencv2/core/ocl.hpp:751
	void cv_ocl_Image2D_Image2D_const_Image2DR(const cv::ocl::Image2D* i, Result<cv::ocl::Image2D*>* ocvrs_return) {
		try {
			cv::ocl::Image2D* ret = new cv::ocl::Image2D(*i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Image2D*>))
	}
	
	// Image2D(cv::ocl::Image2D &&) /usr/include/opencv2/core/ocl.hpp:755
	cv::ocl::Image2D* cv_ocl_Image2D_Image2D_Image2DR(cv::ocl::Image2D* unnamed) {
			cv::ocl::Image2D* ret = new cv::ocl::Image2D(*unnamed);
			return ret;
	}
	
	// canCreateAlias(const cv::UMat &) /usr/include/opencv2/core/ocl.hpp:761
	void cv_ocl_Image2D_canCreateAlias_const_UMatR(const cv::UMat* u, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ocl::Image2D::canCreateAlias(*u);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// isFormatSupported(int, int, bool) /usr/include/opencv2/core/ocl.hpp:765
	void cv_ocl_Image2D_isFormatSupported_int_int_bool(int depth, int cn, bool norm, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ocl::Image2D::isFormatSupported(depth, cn, norm);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// ptr() /usr/include/opencv2/core/ocl.hpp:767
	void cv_ocl_Image2D_ptr_const(const cv::ocl::Image2D* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->ptr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	void cv_Kernel_delete(cv::ocl::Kernel* instance) {
		delete instance;
	}
	// Kernel() /usr/include/opencv2/core/ocl.hpp:459
	cv::ocl::Kernel* cv_ocl_Kernel_Kernel() {
			cv::ocl::Kernel* ret = new cv::ocl::Kernel();
			return ret;
	}
	
	// Kernel(const char *, const cv::ocl::Program &) /usr/include/opencv2/core/ocl.hpp:460
	void cv_ocl_Kernel_Kernel_const_charX_const_ProgramR(const char* kname, const cv::ocl::Program* prog, Result<cv::ocl::Kernel*>* ocvrs_return) {
		try {
			cv::ocl::Kernel* ret = new cv::ocl::Kernel(kname, *prog);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Kernel*>))
	}
	
	// Kernel(const char *, const cv::ocl::ProgramSource &, const cv::String &, cv::String *) /usr/include/opencv2/core/ocl.hpp:461
	void cv_ocl_Kernel_Kernel_const_charX_const_ProgramSourceR_const_StringR_StringX(const char* kname, const cv::ocl::ProgramSource* prog, const char* buildopts, void** errmsg, Result<cv::ocl::Kernel*>* ocvrs_return) {
		try {
			std::string errmsg_out;
			cv::ocl::Kernel* ret = new cv::ocl::Kernel(kname, *prog, std::string(buildopts), &errmsg_out);
			*errmsg = ocvrs_create_string(errmsg_out.c_str());
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Kernel*>))
	}
	
	// Kernel(const cv::ocl::Kernel &) /usr/include/opencv2/core/ocl.hpp:464
	void cv_ocl_Kernel_Kernel_const_KernelR(const cv::ocl::Kernel* k, Result<cv::ocl::Kernel*>* ocvrs_return) {
		try {
			cv::ocl::Kernel* ret = new cv::ocl::Kernel(*k);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Kernel*>))
	}
	
	// Kernel(cv::ocl::Kernel &&) /usr/include/opencv2/core/ocl.hpp:466
	cv::ocl::Kernel* cv_ocl_Kernel_Kernel_KernelR(cv::ocl::Kernel* k) {
			cv::ocl::Kernel* ret = new cv::ocl::Kernel(*k);
			return ret;
	}
	
	// empty() /usr/include/opencv2/core/ocl.hpp:469
	void cv_ocl_Kernel_empty_const(const cv::ocl::Kernel* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// create(const char *, const cv::ocl::Program &) /usr/include/opencv2/core/ocl.hpp:470
	void cv_ocl_Kernel_create_const_charX_const_ProgramR(cv::ocl::Kernel* instance, const char* kname, const cv::ocl::Program* prog, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->create(kname, *prog);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// create(const char *, const cv::ocl::ProgramSource &, const cv::String &, cv::String *) /usr/include/opencv2/core/ocl.hpp:471
	void cv_ocl_Kernel_create_const_charX_const_ProgramSourceR_const_StringR_StringX(cv::ocl::Kernel* instance, const char* kname, const cv::ocl::ProgramSource* prog, const char* buildopts, void** errmsg, Result<bool>* ocvrs_return) {
		try {
			std::string errmsg_out;
			bool ret = instance->create(kname, *prog, std::string(buildopts), &errmsg_out);
			*errmsg = ocvrs_create_string(errmsg_out.c_str());
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// set(int, const void *, size_t) /usr/include/opencv2/core/ocl.hpp:474
	void cv_ocl_Kernel_set_int_const_voidX_size_t(cv::ocl::Kernel* instance, int i, const void* value, size_t sz, Result<int>* ocvrs_return) {
		try {
			int ret = instance->set(i, value, sz);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// set(int, const cv::ocl::Image2D &) /usr/include/opencv2/core/ocl.hpp:475
	void cv_ocl_Kernel_set_int_const_Image2DR(cv::ocl::Kernel* instance, int i, const cv::ocl::Image2D* image2D, Result<int>* ocvrs_return) {
		try {
			int ret = instance->set(i, *image2D);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// set(int, const cv::UMat &) /usr/include/opencv2/core/ocl.hpp:476
	void cv_ocl_Kernel_set_int_const_UMatR(cv::ocl::Kernel* instance, int i, const cv::UMat* m, Result<int>* ocvrs_return) {
		try {
			int ret = instance->set(i, *m);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// set(int, const cv::ocl::KernelArg &) /usr/include/opencv2/core/ocl.hpp:477
	void cv_ocl_Kernel_set_int_const_KernelArgR(cv::ocl::Kernel* instance, int i, const cv::ocl::KernelArg* arg, Result<int>* ocvrs_return) {
		try {
			int ret = instance->set(i, *arg);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// run(int, size_t *, size_t *, bool, const cv::ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:515
	void cv_ocl_Kernel_run_int_size_tX_size_tX_bool_const_QueueR(cv::ocl::Kernel* instance, int dims, size_t* globalsize, size_t* localsize, bool sync, const cv::ocl::Queue* q, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->run(dims, globalsize, localsize, sync, *q);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// run_(int, size_t *, size_t *, bool, const cv::ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:526
	void cv_ocl_Kernel_run__int_size_tX_size_tX_bool_const_QueueR(cv::ocl::Kernel* instance, int dims, size_t* globalsize, size_t* localsize, bool sync, const cv::ocl::Queue* q, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->run_(dims, globalsize, localsize, sync, *q);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// runTask(bool, const cv::ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:528
	void cv_ocl_Kernel_runTask_bool_const_QueueR(cv::ocl::Kernel* instance, bool sync, const cv::ocl::Queue* q, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->runTask(sync, *q);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// runProfiling(int, size_t *, size_t *, const cv::ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:535
	void cv_ocl_Kernel_runProfiling_int_size_tX_size_tX_const_QueueR(cv::ocl::Kernel* instance, int dims, size_t* globalsize, size_t* localsize, const cv::ocl::Queue* q, Result<int64_t>* ocvrs_return) {
		try {
			int64_t ret = instance->runProfiling(dims, globalsize, localsize, *q);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int64_t>))
	}
	
	// workGroupSize() /usr/include/opencv2/core/ocl.hpp:537
	void cv_ocl_Kernel_workGroupSize_const(const cv::ocl::Kernel* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->workGroupSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// preferedWorkGroupSizeMultiple() /usr/include/opencv2/core/ocl.hpp:538
	void cv_ocl_Kernel_preferedWorkGroupSizeMultiple_const(const cv::ocl::Kernel* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->preferedWorkGroupSizeMultiple();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// compileWorkGroupSize(size_t *) /usr/include/opencv2/core/ocl.hpp:539
	void cv_ocl_Kernel_compileWorkGroupSize_const_size_tX(const cv::ocl::Kernel* instance, size_t* wsz, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->compileWorkGroupSize(wsz);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// localMemSize() /usr/include/opencv2/core/ocl.hpp:540
	void cv_ocl_Kernel_localMemSize_const(const cv::ocl::Kernel* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->localMemSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// ptr() /usr/include/opencv2/core/ocl.hpp:542
	void cv_ocl_Kernel_ptr_const(const cv::ocl::Kernel* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->ptr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// flags /usr/include/opencv2/core/ocl.hpp:448
	int cv_ocl_KernelArg_getPropFlags_const(const cv::ocl::KernelArg* instance) {
			int ret = instance->flags;
			return ret;
	}
	
	// flags /usr/include/opencv2/core/ocl.hpp:448
	void cv_ocl_KernelArg_setPropFlags_int(cv::ocl::KernelArg* instance, int val) {
			instance->flags = val;
	}
	
	// m /usr/include/opencv2/core/ocl.hpp:449
	cv::UMat** cv_ocl_KernelArg_getPropM(cv::ocl::KernelArg* instance) {
			cv::UMat* ret = instance->m;
			return new cv::UMat*(ret);
	}
	
	// m /usr/include/opencv2/core/ocl.hpp:449
	void cv_ocl_KernelArg_setPropM_UMatX(cv::ocl::KernelArg* instance, cv::UMat* val) {
			instance->m = val;
	}
	
	// obj /usr/include/opencv2/core/ocl.hpp:450
	const void* cv_ocl_KernelArg_getPropObj_const(const cv::ocl::KernelArg* instance) {
			const void* ret = instance->obj;
			return ret;
	}
	
	// sz /usr/include/opencv2/core/ocl.hpp:451
	size_t cv_ocl_KernelArg_getPropSz_const(const cv::ocl::KernelArg* instance) {
			size_t ret = instance->sz;
			return ret;
	}
	
	// sz /usr/include/opencv2/core/ocl.hpp:451
	void cv_ocl_KernelArg_setPropSz_size_t(cv::ocl::KernelArg* instance, size_t val) {
			instance->sz = val;
	}
	
	// wscale /usr/include/opencv2/core/ocl.hpp:452
	int cv_ocl_KernelArg_getPropWscale_const(const cv::ocl::KernelArg* instance) {
			int ret = instance->wscale;
			return ret;
	}
	
	// wscale /usr/include/opencv2/core/ocl.hpp:452
	void cv_ocl_KernelArg_setPropWscale_int(cv::ocl::KernelArg* instance, int val) {
			instance->wscale = val;
	}
	
	// iwscale /usr/include/opencv2/core/ocl.hpp:452
	int cv_ocl_KernelArg_getPropIwscale_const(const cv::ocl::KernelArg* instance) {
			int ret = instance->iwscale;
			return ret;
	}
	
	// iwscale /usr/include/opencv2/core/ocl.hpp:452
	void cv_ocl_KernelArg_setPropIwscale_int(cv::ocl::KernelArg* instance, int val) {
			instance->iwscale = val;
	}
	
	void cv_KernelArg_delete(cv::ocl::KernelArg* instance) {
		delete instance;
	}
	// KernelArg(int, cv::UMat *, int, int, const void *, size_t) /usr/include/opencv2/core/ocl.hpp:421
	void cv_ocl_KernelArg_KernelArg_int_UMatX_int_int_const_voidX_size_t(int _flags, cv::UMat* _m, int wscale, int iwscale, const void* _obj, size_t _sz, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg* ret = new cv::ocl::KernelArg(_flags, _m, wscale, iwscale, _obj, _sz);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	// KernelArg() /usr/include/opencv2/core/ocl.hpp:422
	cv::ocl::KernelArg* cv_ocl_KernelArg_KernelArg() {
			cv::ocl::KernelArg* ret = new cv::ocl::KernelArg();
			return ret;
	}
	
	// Local(size_t) /usr/include/opencv2/core/ocl.hpp:424
	void cv_ocl_KernelArg_Local_size_t(size_t localMemSize, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::Local(localMemSize);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	// PtrWriteOnly(const cv::UMat &) /usr/include/opencv2/core/ocl.hpp:426
	void cv_ocl_KernelArg_PtrWriteOnly_const_UMatR(const cv::UMat* m, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::PtrWriteOnly(*m);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	// PtrReadOnly(const cv::UMat &) /usr/include/opencv2/core/ocl.hpp:428
	void cv_ocl_KernelArg_PtrReadOnly_const_UMatR(const cv::UMat* m, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::PtrReadOnly(*m);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	// PtrReadWrite(const cv::UMat &) /usr/include/opencv2/core/ocl.hpp:430
	void cv_ocl_KernelArg_PtrReadWrite_const_UMatR(const cv::UMat* m, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::PtrReadWrite(*m);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	// ReadWrite(const cv::UMat &, int, int) /usr/include/opencv2/core/ocl.hpp:432
	void cv_ocl_KernelArg_ReadWrite_const_UMatR_int_int(const cv::UMat* m, int wscale, int iwscale, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::ReadWrite(*m, wscale, iwscale);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	// ReadWriteNoSize(const cv::UMat &, int, int) /usr/include/opencv2/core/ocl.hpp:434
	void cv_ocl_KernelArg_ReadWriteNoSize_const_UMatR_int_int(const cv::UMat* m, int wscale, int iwscale, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::ReadWriteNoSize(*m, wscale, iwscale);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	// ReadOnly(const cv::UMat &, int, int) /usr/include/opencv2/core/ocl.hpp:436
	void cv_ocl_KernelArg_ReadOnly_const_UMatR_int_int(const cv::UMat* m, int wscale, int iwscale, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::ReadOnly(*m, wscale, iwscale);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	// WriteOnly(const cv::UMat &, int, int) /usr/include/opencv2/core/ocl.hpp:438
	void cv_ocl_KernelArg_WriteOnly_const_UMatR_int_int(const cv::UMat* m, int wscale, int iwscale, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::WriteOnly(*m, wscale, iwscale);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	// ReadOnlyNoSize(const cv::UMat &, int, int) /usr/include/opencv2/core/ocl.hpp:440
	void cv_ocl_KernelArg_ReadOnlyNoSize_const_UMatR_int_int(const cv::UMat* m, int wscale, int iwscale, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::ReadOnlyNoSize(*m, wscale, iwscale);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	// WriteOnlyNoSize(const cv::UMat &, int, int) /usr/include/opencv2/core/ocl.hpp:442
	void cv_ocl_KernelArg_WriteOnlyNoSize_const_UMatR_int_int(const cv::UMat* m, int wscale, int iwscale, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::WriteOnlyNoSize(*m, wscale, iwscale);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	// Constant(const cv::Mat &) /usr/include/opencv2/core/ocl.hpp:444
	void cv_ocl_KernelArg_Constant_const_MatR(const cv::Mat* m, Result<cv::ocl::KernelArg*>* ocvrs_return) {
		try {
			cv::ocl::KernelArg ret = cv::ocl::KernelArg::Constant(*m);
			Ok(new cv::ocl::KernelArg(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::KernelArg*>))
	}
	
	void cv_OpenCLExecutionContext_delete(cv::ocl::OpenCLExecutionContext* instance) {
		delete instance;
	}
	// OpenCLExecutionContext() /usr/include/opencv2/core/ocl.hpp:798
	cv::ocl::OpenCLExecutionContext* cv_ocl_OpenCLExecutionContext_OpenCLExecutionContext() {
			cv::ocl::OpenCLExecutionContext* ret = new cv::ocl::OpenCLExecutionContext();
			return ret;
	}
	
	// OpenCLExecutionContext(const cv::ocl::OpenCLExecutionContext &) /usr/include/opencv2/core/ocl.hpp:801
	cv::ocl::OpenCLExecutionContext* cv_ocl_OpenCLExecutionContext_OpenCLExecutionContext_const_OpenCLExecutionContextR(const cv::ocl::OpenCLExecutionContext* unnamed) {
			cv::ocl::OpenCLExecutionContext* ret = new cv::ocl::OpenCLExecutionContext(*unnamed);
			return ret;
	}
	
	// OpenCLExecutionContext(cv::ocl::OpenCLExecutionContext &&) /usr/include/opencv2/core/ocl.hpp:802
	cv::ocl::OpenCLExecutionContext* cv_ocl_OpenCLExecutionContext_OpenCLExecutionContext_OpenCLExecutionContextR(cv::ocl::OpenCLExecutionContext* unnamed) {
			cv::ocl::OpenCLExecutionContext* ret = new cv::ocl::OpenCLExecutionContext(*unnamed);
			return ret;
	}
	
	// getContext() /usr/include/opencv2/core/ocl.hpp:808
	void cv_ocl_OpenCLExecutionContext_getContext_const(const cv::ocl::OpenCLExecutionContext* instance, Result<cv::ocl::Context*>* ocvrs_return) {
		try {
			cv::ocl::Context ret = instance->getContext();
			Ok(new cv::ocl::Context(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Context*>))
	}
	
	// getDevice() /usr/include/opencv2/core/ocl.hpp:810
	void cv_ocl_OpenCLExecutionContext_getDevice_const(const cv::ocl::OpenCLExecutionContext* instance, Result<cv::ocl::Device*>* ocvrs_return) {
		try {
			cv::ocl::Device ret = instance->getDevice();
			Ok(new cv::ocl::Device(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Device*>))
	}
	
	// getQueue() /usr/include/opencv2/core/ocl.hpp:814
	void cv_ocl_OpenCLExecutionContext_getQueue_const(const cv::ocl::OpenCLExecutionContext* instance, Result<cv::ocl::Queue*>* ocvrs_return) {
		try {
			cv::ocl::Queue ret = instance->getQueue();
			Ok(new cv::ocl::Queue(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Queue*>))
	}
	
	// useOpenCL() /usr/include/opencv2/core/ocl.hpp:816
	void cv_ocl_OpenCLExecutionContext_useOpenCL_const(const cv::ocl::OpenCLExecutionContext* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->useOpenCL();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setUseOpenCL(bool) /usr/include/opencv2/core/ocl.hpp:817
	void cv_ocl_OpenCLExecutionContext_setUseOpenCL_bool(cv::ocl::OpenCLExecutionContext* instance, bool flag, Result_void* ocvrs_return) {
		try {
			instance->setUseOpenCL(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getCurrent() /usr/include/opencv2/core/ocl.hpp:825
	void cv_ocl_OpenCLExecutionContext_getCurrent(Result<cv::ocl::OpenCLExecutionContext*>* ocvrs_return) {
		try {
			cv::ocl::OpenCLExecutionContext ret = cv::ocl::OpenCLExecutionContext::getCurrent();
			Ok(new cv::ocl::OpenCLExecutionContext(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::OpenCLExecutionContext*>))
	}
	
	// getCurrentRef() /usr/include/opencv2/core/ocl.hpp:828
	void cv_ocl_OpenCLExecutionContext_getCurrentRef(Result<cv::ocl::OpenCLExecutionContext*>* ocvrs_return) {
		try {
			cv::ocl::OpenCLExecutionContext ret = cv::ocl::OpenCLExecutionContext::getCurrentRef();
			Ok(new cv::ocl::OpenCLExecutionContext(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::OpenCLExecutionContext*>))
	}
	
	// bind() /usr/include/opencv2/core/ocl.hpp:836
	void cv_ocl_OpenCLExecutionContext_bind_const(const cv::ocl::OpenCLExecutionContext* instance, Result_void* ocvrs_return) {
		try {
			instance->bind();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// cloneWithNewQueue(const ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:842
	void cv_ocl_OpenCLExecutionContext_cloneWithNewQueue_const_const_QueueR(const cv::ocl::OpenCLExecutionContext* instance, const cv::ocl::Queue* q, Result<cv::ocl::OpenCLExecutionContext*>* ocvrs_return) {
		try {
			cv::ocl::OpenCLExecutionContext ret = instance->cloneWithNewQueue(*q);
			Ok(new cv::ocl::OpenCLExecutionContext(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::OpenCLExecutionContext*>))
	}
	
	// cloneWithNewQueue() /usr/include/opencv2/core/ocl.hpp:844
	void cv_ocl_OpenCLExecutionContext_cloneWithNewQueue_const(const cv::ocl::OpenCLExecutionContext* instance, Result<cv::ocl::OpenCLExecutionContext*>* ocvrs_return) {
		try {
			cv::ocl::OpenCLExecutionContext ret = instance->cloneWithNewQueue();
			Ok(new cv::ocl::OpenCLExecutionContext(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::OpenCLExecutionContext*>))
	}
	
	// create(const std::string &, void *, void *, void *) /usr/include/opencv2/core/ocl.hpp:860
	void cv_ocl_OpenCLExecutionContext_create_const_stringR_voidX_voidX_voidX(const char* platformName, void* platformID, void* context, void* deviceID, Result<cv::ocl::OpenCLExecutionContext*>* ocvrs_return) {
		try {
			cv::ocl::OpenCLExecutionContext ret = cv::ocl::OpenCLExecutionContext::create(std::string(platformName), platformID, context, deviceID);
			Ok(new cv::ocl::OpenCLExecutionContext(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::OpenCLExecutionContext*>))
	}
	
	// create(const cv::ocl::Context &, const cv::ocl::Device &, const ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:868
	void cv_ocl_OpenCLExecutionContext_create_const_ContextR_const_DeviceR_const_QueueR(const cv::ocl::Context* context, const cv::ocl::Device* device, const cv::ocl::Queue* queue, Result<cv::ocl::OpenCLExecutionContext*>* ocvrs_return) {
		try {
			cv::ocl::OpenCLExecutionContext ret = cv::ocl::OpenCLExecutionContext::create(*context, *device, *queue);
			Ok(new cv::ocl::OpenCLExecutionContext(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::OpenCLExecutionContext*>))
	}
	
	// create(const cv::ocl::Context &, const cv::ocl::Device &) /usr/include/opencv2/core/ocl.hpp:870
	void cv_ocl_OpenCLExecutionContext_create_const_ContextR_const_DeviceR(const cv::ocl::Context* context, const cv::ocl::Device* device, Result<cv::ocl::OpenCLExecutionContext*>* ocvrs_return) {
		try {
			cv::ocl::OpenCLExecutionContext ret = cv::ocl::OpenCLExecutionContext::create(*context, *device);
			Ok(new cv::ocl::OpenCLExecutionContext(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::OpenCLExecutionContext*>))
	}
	
	// empty() /usr/include/opencv2/core/ocl.hpp:873
	void cv_ocl_OpenCLExecutionContext_empty_const(const cv::ocl::OpenCLExecutionContext* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// release() /usr/include/opencv2/core/ocl.hpp:874
	void cv_ocl_OpenCLExecutionContext_release(cv::ocl::OpenCLExecutionContext* instance, Result_void* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Platform_delete(cv::ocl::Platform* instance) {
		delete instance;
	}
	// Platform() /usr/include/opencv2/core/ocl.hpp:332
	cv::ocl::Platform* cv_ocl_Platform_Platform() {
			cv::ocl::Platform* ret = new cv::ocl::Platform();
			return ret;
	}
	
	// Platform(const cv::ocl::Platform &) /usr/include/opencv2/core/ocl.hpp:334
	void cv_ocl_Platform_Platform_const_PlatformR(const cv::ocl::Platform* p, Result<cv::ocl::Platform*>* ocvrs_return) {
		try {
			cv::ocl::Platform* ret = new cv::ocl::Platform(*p);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Platform*>))
	}
	
	// Platform(cv::ocl::Platform &&) /usr/include/opencv2/core/ocl.hpp:336
	cv::ocl::Platform* cv_ocl_Platform_Platform_PlatformR(cv::ocl::Platform* p) {
			cv::ocl::Platform* ret = new cv::ocl::Platform(*p);
			return ret;
	}
	
	// ptr() /usr/include/opencv2/core/ocl.hpp:339
	void cv_ocl_Platform_ptr_const(const cv::ocl::Platform* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->ptr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// getDefault() /usr/include/opencv2/core/ocl.hpp:342
	void cv_ocl_Platform_getDefault(Result<cv::ocl::Platform*>* ocvrs_return) {
		try {
			cv::ocl::Platform ret = cv::ocl::Platform::getDefault();
			Ok(new cv::ocl::Platform(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Platform*>))
	}
	
	// empty() /usr/include/opencv2/core/ocl.hpp:346
	void cv_ocl_Platform_empty_const(const cv::ocl::Platform* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_PlatformInfo_delete(cv::ocl::PlatformInfo* instance) {
		delete instance;
	}
	// PlatformInfo() /usr/include/opencv2/core/ocl.hpp:671
	cv::ocl::PlatformInfo* cv_ocl_PlatformInfo_PlatformInfo() {
			cv::ocl::PlatformInfo* ret = new cv::ocl::PlatformInfo();
			return ret;
	}
	
	// PlatformInfo(void *) /usr/include/opencv2/core/ocl.hpp:675
	void cv_ocl_PlatformInfo_PlatformInfo_voidX(void* id, Result<cv::ocl::PlatformInfo*>* ocvrs_return) {
		try {
			cv::ocl::PlatformInfo* ret = new cv::ocl::PlatformInfo(id);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::PlatformInfo*>))
	}
	
	// PlatformInfo(const cv::ocl::PlatformInfo &) /usr/include/opencv2/core/ocl.hpp:678
	void cv_ocl_PlatformInfo_PlatformInfo_const_PlatformInfoR(const cv::ocl::PlatformInfo* i, Result<cv::ocl::PlatformInfo*>* ocvrs_return) {
		try {
			cv::ocl::PlatformInfo* ret = new cv::ocl::PlatformInfo(*i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::PlatformInfo*>))
	}
	
	// PlatformInfo(cv::ocl::PlatformInfo &&) /usr/include/opencv2/core/ocl.hpp:680
	cv::ocl::PlatformInfo* cv_ocl_PlatformInfo_PlatformInfo_PlatformInfoR(cv::ocl::PlatformInfo* i) {
			cv::ocl::PlatformInfo* ret = new cv::ocl::PlatformInfo(*i);
			return ret;
	}
	
	// name() /usr/include/opencv2/core/ocl.hpp:683
	void cv_ocl_PlatformInfo_name_const(const cv::ocl::PlatformInfo* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->name();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// vendor() /usr/include/opencv2/core/ocl.hpp:684
	void cv_ocl_PlatformInfo_vendor_const(const cv::ocl::PlatformInfo* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->vendor();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// version() /usr/include/opencv2/core/ocl.hpp:687
	void cv_ocl_PlatformInfo_version_const(const cv::ocl::PlatformInfo* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->version();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// versionMajor() /usr/include/opencv2/core/ocl.hpp:688
	void cv_ocl_PlatformInfo_versionMajor_const(const cv::ocl::PlatformInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->versionMajor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// versionMinor() /usr/include/opencv2/core/ocl.hpp:689
	void cv_ocl_PlatformInfo_versionMinor_const(const cv::ocl::PlatformInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->versionMinor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// deviceNumber() /usr/include/opencv2/core/ocl.hpp:691
	void cv_ocl_PlatformInfo_deviceNumber_const(const cv::ocl::PlatformInfo* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->deviceNumber();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getDevice(cv::ocl::Device &, int) /usr/include/opencv2/core/ocl.hpp:692
	void cv_ocl_PlatformInfo_getDevice_const_DeviceR_int(const cv::ocl::PlatformInfo* instance, cv::ocl::Device* device, int d, Result_void* ocvrs_return) {
		try {
			instance->getDevice(*device, d);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// empty() /usr/include/opencv2/core/ocl.hpp:695
	void cv_ocl_PlatformInfo_empty_const(const cv::ocl::PlatformInfo* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_Program_delete(cv::ocl::Program* instance) {
		delete instance;
	}
	// Program() /usr/include/opencv2/core/ocl.hpp:552
	cv::ocl::Program* cv_ocl_Program_Program() {
			cv::ocl::Program* ret = new cv::ocl::Program();
			return ret;
	}
	
	// Program(const cv::ocl::ProgramSource &, const cv::String &, cv::String &) /usr/include/opencv2/core/ocl.hpp:553
	void cv_ocl_Program_Program_const_ProgramSourceR_const_StringR_StringR(const cv::ocl::ProgramSource* src, const char* buildflags, void** errmsg, Result<cv::ocl::Program*>* ocvrs_return) {
		try {
			std::string errmsg_out;
			cv::ocl::Program* ret = new cv::ocl::Program(*src, std::string(buildflags), errmsg_out);
			*errmsg = ocvrs_create_string(errmsg_out.c_str());
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Program*>))
	}
	
	// Program(const cv::ocl::Program &) /usr/include/opencv2/core/ocl.hpp:555
	void cv_ocl_Program_Program_const_ProgramR(const cv::ocl::Program* prog, Result<cv::ocl::Program*>* ocvrs_return) {
		try {
			cv::ocl::Program* ret = new cv::ocl::Program(*prog);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Program*>))
	}
	
	// Program(cv::ocl::Program &&) /usr/include/opencv2/core/ocl.hpp:557
	cv::ocl::Program* cv_ocl_Program_Program_ProgramR(cv::ocl::Program* prog) {
			cv::ocl::Program* ret = new cv::ocl::Program(*prog);
			return ret;
	}
	
	// create(const cv::ocl::ProgramSource &, const cv::String &, cv::String &) /usr/include/opencv2/core/ocl.hpp:561
	void cv_ocl_Program_create_const_ProgramSourceR_const_StringR_StringR(cv::ocl::Program* instance, const cv::ocl::ProgramSource* src, const char* buildflags, void** errmsg, Result<bool>* ocvrs_return) {
		try {
			std::string errmsg_out;
			bool ret = instance->create(*src, std::string(buildflags), errmsg_out);
			*errmsg = ocvrs_create_string(errmsg_out.c_str());
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// ptr() /usr/include/opencv2/core/ocl.hpp:564
	void cv_ocl_Program_ptr_const(const cv::ocl::Program* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->ptr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// getBinary(std::vector<char> &) /usr/include/opencv2/core/ocl.hpp:575
	void cv_ocl_Program_getBinary_const_vector_char_R(const cv::ocl::Program* instance, std::vector<char>* binary, Result_void* ocvrs_return) {
		try {
			instance->getBinary(*binary);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// empty() /usr/include/opencv2/core/ocl.hpp:579
	void cv_ocl_Program_empty_const(const cv::ocl::Program* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// read(const cv::String &, const cv::String &) /usr/include/opencv2/core/ocl.hpp:585
	void cv_ocl_Program_read_const_StringR_const_StringR(cv::ocl::Program* instance, const char* buf, const char* buildflags, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->read(std::string(buf), std::string(buildflags));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// write(cv::String &) /usr/include/opencv2/core/ocl.hpp:586
	void cv_ocl_Program_write_const_StringR(const cv::ocl::Program* instance, void** buf, Result<bool>* ocvrs_return) {
		try {
			std::string buf_out;
			bool ret = instance->write(buf_out);
			*buf = ocvrs_create_string(buf_out.c_str());
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// source() /usr/include/opencv2/core/ocl.hpp:587
	void cv_ocl_Program_source_const(const cv::ocl::Program* instance, Result<cv::ocl::ProgramSource*>* ocvrs_return) {
		try {
			const cv::ocl::ProgramSource ret = instance->source();
			Ok(new const cv::ocl::ProgramSource(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::ProgramSource*>))
	}
	
	// getPrefix() /usr/include/opencv2/core/ocl.hpp:588
	void cv_ocl_Program_getPrefix_const(const cv::ocl::Program* instance, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->getPrefix();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// getPrefix(const cv::String &) /usr/include/opencv2/core/ocl.hpp:589
	void cv_ocl_Program_getPrefix_const_StringR(const char* buildflags, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::ocl::Program::getPrefix(std::string(buildflags));
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	void cv_ProgramSource_delete(cv::ocl::ProgramSource* instance) {
		delete instance;
	}
	// ProgramSource() /usr/include/opencv2/core/ocl.hpp:599
	cv::ocl::ProgramSource* cv_ocl_ProgramSource_ProgramSource() {
			cv::ocl::ProgramSource* ret = new cv::ocl::ProgramSource();
			return ret;
	}
	
	// ProgramSource(const cv::String &, const cv::String &, const cv::String &, const cv::String &) /usr/include/opencv2/core/ocl.hpp:600
	void cv_ocl_ProgramSource_ProgramSource_const_StringR_const_StringR_const_StringR_const_StringR(const char* module, const char* name, const char* codeStr, const char* codeHash, Result<cv::ocl::ProgramSource*>* ocvrs_return) {
		try {
			cv::ocl::ProgramSource* ret = new cv::ocl::ProgramSource(std::string(module), std::string(name), std::string(codeStr), std::string(codeHash));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::ProgramSource*>))
	}
	
	// ProgramSource(const cv::String &) /usr/include/opencv2/core/ocl.hpp:601
	void cv_ocl_ProgramSource_ProgramSource_const_StringR(const char* prog, Result<cv::ocl::ProgramSource*>* ocvrs_return) {
		try {
			cv::ocl::ProgramSource* ret = new cv::ocl::ProgramSource(std::string(prog));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::ProgramSource*>))
	}
	
	// ProgramSource(const cv::ocl::ProgramSource &) /usr/include/opencv2/core/ocl.hpp:604
	void cv_ocl_ProgramSource_ProgramSource_const_ProgramSourceR(const cv::ocl::ProgramSource* prog, Result<cv::ocl::ProgramSource*>* ocvrs_return) {
		try {
			cv::ocl::ProgramSource* ret = new cv::ocl::ProgramSource(*prog);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::ProgramSource*>))
	}
	
	// ProgramSource(cv::ocl::ProgramSource &&) /usr/include/opencv2/core/ocl.hpp:606
	cv::ocl::ProgramSource* cv_ocl_ProgramSource_ProgramSource_ProgramSourceR(cv::ocl::ProgramSource* prog) {
			cv::ocl::ProgramSource* ret = new cv::ocl::ProgramSource(*prog);
			return ret;
	}
	
	// source() /usr/include/opencv2/core/ocl.hpp:609
	void cv_ocl_ProgramSource_source_const(const cv::ocl::ProgramSource* instance, Result<void*>* ocvrs_return) {
		try {
			const cv::String ret = instance->source();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// hash() /usr/include/opencv2/core/ocl.hpp:610
	void cv_ocl_ProgramSource_hash_const(const cv::ocl::ProgramSource* instance, Result<cv::ocl::ProgramSource::hash_t>* ocvrs_return) {
		try {
			cv::ocl::ProgramSource::hash_t ret = instance->hash();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::ProgramSource::hash_t>))
	}
	
	// fromBinary(const cv::String &, const cv::String &, const unsigned char *, const size_t, const cv::String &) /usr/include/opencv2/core/ocl.hpp:627
	void cv_ocl_ProgramSource_fromBinary_const_StringR_const_StringR_const_unsigned_charX_const_size_t_const_StringR(const char* module, const char* name, const unsigned char* binary, const size_t size, const char* buildOptions, Result<cv::ocl::ProgramSource*>* ocvrs_return) {
		try {
			cv::ocl::ProgramSource ret = cv::ocl::ProgramSource::fromBinary(std::string(module), std::string(name), binary, size, std::string(buildOptions));
			Ok(new cv::ocl::ProgramSource(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::ProgramSource*>))
	}
	
	// fromSPIR(const cv::String &, const cv::String &, const unsigned char *, const size_t, const cv::String &) /usr/include/opencv2/core/ocl.hpp:652
	void cv_ocl_ProgramSource_fromSPIR_const_StringR_const_StringR_const_unsigned_charX_const_size_t_const_StringR(const char* module, const char* name, const unsigned char* binary, const size_t size, const char* buildOptions, Result<cv::ocl::ProgramSource*>* ocvrs_return) {
		try {
			cv::ocl::ProgramSource ret = cv::ocl::ProgramSource::fromSPIR(std::string(module), std::string(name), binary, size, std::string(buildOptions));
			Ok(new cv::ocl::ProgramSource(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::ProgramSource*>))
	}
	
	// empty() /usr/include/opencv2/core/ocl.hpp:663
	void cv_ocl_ProgramSource_empty_const(const cv::ocl::ProgramSource* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_Queue_delete(cv::ocl::Queue* instance) {
		delete instance;
	}
	// Queue() /usr/include/opencv2/core/ocl.hpp:393
	cv::ocl::Queue* cv_ocl_Queue_Queue() {
			cv::ocl::Queue* ret = new cv::ocl::Queue();
			return ret;
	}
	
	// Queue(const cv::ocl::Context &, const cv::ocl::Device &) /usr/include/opencv2/core/ocl.hpp:394
	void cv_ocl_Queue_Queue_const_ContextR_const_DeviceR(const cv::ocl::Context* c, const cv::ocl::Device* d, Result<cv::ocl::Queue*>* ocvrs_return) {
		try {
			cv::ocl::Queue* ret = new cv::ocl::Queue(*c, *d);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Queue*>))
	}
	
	// Queue(const cv::ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:396
	void cv_ocl_Queue_Queue_const_QueueR(const cv::ocl::Queue* q, Result<cv::ocl::Queue*>* ocvrs_return) {
		try {
			cv::ocl::Queue* ret = new cv::ocl::Queue(*q);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Queue*>))
	}
	
	// Queue(cv::ocl::Queue &&) /usr/include/opencv2/core/ocl.hpp:398
	cv::ocl::Queue* cv_ocl_Queue_Queue_QueueR(cv::ocl::Queue* q) {
			cv::ocl::Queue* ret = new cv::ocl::Queue(*q);
			return ret;
	}
	
	// create(const cv::ocl::Context &, const cv::ocl::Device &) /usr/include/opencv2/core/ocl.hpp:401
	void cv_ocl_Queue_create_const_ContextR_const_DeviceR(cv::ocl::Queue* instance, const cv::ocl::Context* c, const cv::ocl::Device* d, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->create(*c, *d);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// finish() /usr/include/opencv2/core/ocl.hpp:402
	void cv_ocl_Queue_finish(cv::ocl::Queue* instance, Result_void* ocvrs_return) {
		try {
			instance->finish();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// ptr() /usr/include/opencv2/core/ocl.hpp:403
	void cv_ocl_Queue_ptr_const(const cv::ocl::Queue* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->ptr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// getDefault() /usr/include/opencv2/core/ocl.hpp:404
	void cv_ocl_Queue_getDefault(Result<cv::ocl::Queue*>* ocvrs_return) {
		try {
			cv::ocl::Queue ret = cv::ocl::Queue::getDefault();
			Ok(new cv::ocl::Queue(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Queue*>))
	}
	
	// getProfilingQueue() /usr/include/opencv2/core/ocl.hpp:407
	void cv_ocl_Queue_getProfilingQueue_const(const cv::ocl::Queue* instance, Result<cv::ocl::Queue*>* ocvrs_return) {
		try {
			const cv::ocl::Queue ret = instance->getProfilingQueue();
			Ok(new const cv::ocl::Queue(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Queue*>))
	}
	
	// empty() /usr/include/opencv2/core/ocl.hpp:411
	void cv_ocl_Queue_empty_const(const cv::ocl::Queue* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_Timer_delete(cv::ocl::Timer* instance) {
		delete instance;
	}
	// Timer(const cv::ocl::Queue &) /usr/include/opencv2/core/ocl.hpp:776
	void cv_ocl_Timer_Timer_const_QueueR(const cv::ocl::Queue* q, Result<cv::ocl::Timer*>* ocvrs_return) {
		try {
			cv::ocl::Timer* ret = new cv::ocl::Timer(*q);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ocl::Timer*>))
	}
	
	// start() /usr/include/opencv2/core/ocl.hpp:778
	void cv_ocl_Timer_start(cv::ocl::Timer* instance, Result_void* ocvrs_return) {
		try {
			instance->start();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// stop() /usr/include/opencv2/core/ocl.hpp:779
	void cv_ocl_Timer_stop(cv::ocl::Timer* instance, Result_void* ocvrs_return) {
		try {
			instance->stop();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// durationNS() /usr/include/opencv2/core/ocl.hpp:781
	void cv_ocl_Timer_durationNS_const(const cv::ocl::Timer* instance, Result<uint64_t>* ocvrs_return) {
		try {
			uint64_t ret = instance->durationNS();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<uint64_t>))
	}
	
	void cv_Arrays_delete(cv::ogl::Arrays* instance) {
		delete instance;
	}
	// Arrays() /usr/include/opencv2/core/opengl.hpp:411
	void cv_ogl_Arrays_Arrays(Result<cv::ogl::Arrays*>* ocvrs_return) {
		try {
			cv::ogl::Arrays* ret = new cv::ogl::Arrays();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ogl::Arrays*>))
	}
	
	// setVertexArray(cv::InputArray) /usr/include/opencv2/core/opengl.hpp:416
	void cv_ogl_Arrays_setVertexArray_const__InputArrayR(cv::ogl::Arrays* instance, const cv::_InputArray* vertex, Result_void* ocvrs_return) {
		try {
			instance->setVertexArray(*vertex);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// resetVertexArray() /usr/include/opencv2/core/opengl.hpp:420
	void cv_ogl_Arrays_resetVertexArray(cv::ogl::Arrays* instance, Result_void* ocvrs_return) {
		try {
			instance->resetVertexArray();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setColorArray(cv::InputArray) /usr/include/opencv2/core/opengl.hpp:425
	void cv_ogl_Arrays_setColorArray_const__InputArrayR(cv::ogl::Arrays* instance, const cv::_InputArray* color, Result_void* ocvrs_return) {
		try {
			instance->setColorArray(*color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// resetColorArray() /usr/include/opencv2/core/opengl.hpp:429
	void cv_ogl_Arrays_resetColorArray(cv::ogl::Arrays* instance, Result_void* ocvrs_return) {
		try {
			instance->resetColorArray();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setNormalArray(cv::InputArray) /usr/include/opencv2/core/opengl.hpp:434
	void cv_ogl_Arrays_setNormalArray_const__InputArrayR(cv::ogl::Arrays* instance, const cv::_InputArray* normal, Result_void* ocvrs_return) {
		try {
			instance->setNormalArray(*normal);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// resetNormalArray() /usr/include/opencv2/core/opengl.hpp:438
	void cv_ogl_Arrays_resetNormalArray(cv::ogl::Arrays* instance, Result_void* ocvrs_return) {
		try {
			instance->resetNormalArray();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setTexCoordArray(cv::InputArray) /usr/include/opencv2/core/opengl.hpp:443
	void cv_ogl_Arrays_setTexCoordArray_const__InputArrayR(cv::ogl::Arrays* instance, const cv::_InputArray* texCoord, Result_void* ocvrs_return) {
		try {
			instance->setTexCoordArray(*texCoord);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// resetTexCoordArray() /usr/include/opencv2/core/opengl.hpp:447
	void cv_ogl_Arrays_resetTexCoordArray(cv::ogl::Arrays* instance, Result_void* ocvrs_return) {
		try {
			instance->resetTexCoordArray();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// release() /usr/include/opencv2/core/opengl.hpp:451
	void cv_ogl_Arrays_release(cv::ogl::Arrays* instance, Result_void* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setAutoRelease(bool) /usr/include/opencv2/core/opengl.hpp:456
	void cv_ogl_Arrays_setAutoRelease_bool(cv::ogl::Arrays* instance, bool flag, Result_void* ocvrs_return) {
		try {
			instance->setAutoRelease(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// bind() /usr/include/opencv2/core/opengl.hpp:460
	void cv_ogl_Arrays_bind_const(const cv::ogl::Arrays* instance, Result_void* ocvrs_return) {
		try {
			instance->bind();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// size() /usr/include/opencv2/core/opengl.hpp:464
	void cv_ogl_Arrays_size_const(const cv::ogl::Arrays* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// empty() /usr/include/opencv2/core/opengl.hpp:465
	void cv_ogl_Arrays_empty_const(const cv::ogl::Arrays* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	void cv_Buffer_delete(cv::ogl::Buffer* instance) {
		delete instance;
	}
	// Buffer() /usr/include/opencv2/core/opengl.hpp:104
	void cv_ogl_Buffer_Buffer(Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer* ret = new cv::ogl::Buffer();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ogl::Buffer*>))
	}
	
	// Buffer(int, int, int, unsigned int, bool) /usr/include/opencv2/core/opengl.hpp:113
	void cv_ogl_Buffer_Buffer_int_int_int_unsigned_int_bool(int arows, int acols, int atype, unsigned int abufId, bool autoRelease, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer* ret = new cv::ogl::Buffer(arows, acols, atype, abufId, autoRelease);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ogl::Buffer*>))
	}
	
	// Buffer(cv::Size, int, unsigned int, bool) /usr/include/opencv2/core/opengl.hpp:121
	void cv_ogl_Buffer_Buffer_Size_int_unsigned_int_bool(cv::Size* asize, int atype, unsigned int abufId, bool autoRelease, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer* ret = new cv::ogl::Buffer(*asize, atype, abufId, autoRelease);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ogl::Buffer*>))
	}
	
	// Buffer(int, int, int, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:130
	void cv_ogl_Buffer_Buffer_int_int_int_Target_bool(int arows, int acols, int atype, cv::ogl::Buffer::Target target, bool autoRelease, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer* ret = new cv::ogl::Buffer(arows, acols, atype, target, autoRelease);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ogl::Buffer*>))
	}
	
	// Buffer(cv::Size, int, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:138
	void cv_ogl_Buffer_Buffer_Size_int_Target_bool(cv::Size* asize, int atype, cv::ogl::Buffer::Target target, bool autoRelease, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer* ret = new cv::ogl::Buffer(*asize, atype, target, autoRelease);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ogl::Buffer*>))
	}
	
	// Buffer(cv::InputArray, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:145
	void cv_ogl_Buffer_Buffer_const__InputArrayR_Target_bool(const cv::_InputArray* arr, cv::ogl::Buffer::Target target, bool autoRelease, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer* ret = new cv::ogl::Buffer(*arr, target, autoRelease);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ogl::Buffer*>))
	}
	
	// create(int, int, int, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:155
	void cv_ogl_Buffer_create_int_int_int_Target_bool(cv::ogl::Buffer* instance, int arows, int acols, int atype, cv::ogl::Buffer::Target target, bool autoRelease, Result_void* ocvrs_return) {
		try {
			instance->create(arows, acols, atype, target, autoRelease);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(cv::Size, int, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:163
	void cv_ogl_Buffer_create_Size_int_Target_bool(cv::ogl::Buffer* instance, cv::Size* asize, int atype, cv::ogl::Buffer::Target target, bool autoRelease, Result_void* ocvrs_return) {
		try {
			instance->create(*asize, atype, target, autoRelease);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// release() /usr/include/opencv2/core/opengl.hpp:169
	void cv_ogl_Buffer_release(cv::ogl::Buffer* instance, Result_void* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setAutoRelease(bool) /usr/include/opencv2/core/opengl.hpp:180
	void cv_ogl_Buffer_setAutoRelease_bool(cv::ogl::Buffer* instance, bool flag, Result_void* ocvrs_return) {
		try {
			instance->setAutoRelease(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// copyFrom(cv::InputArray, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:187
	void cv_ogl_Buffer_copyFrom_const__InputArrayR_Target_bool(cv::ogl::Buffer* instance, const cv::_InputArray* arr, cv::ogl::Buffer::Target target, bool autoRelease, Result_void* ocvrs_return) {
		try {
			instance->copyFrom(*arr, target, autoRelease);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// copyFrom(cv::InputArray, cuda::Stream &, cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:190
	void cv_ogl_Buffer_copyFrom_const__InputArrayR_StreamR_Target_bool(cv::ogl::Buffer* instance, const cv::_InputArray* arr, cv::cuda::Stream* stream, cv::ogl::Buffer::Target target, bool autoRelease, Result_void* ocvrs_return) {
		try {
			instance->copyFrom(*arr, *stream, target, autoRelease);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// copyTo(cv::OutputArray) /usr/include/opencv2/core/opengl.hpp:197
	void cv_ogl_Buffer_copyTo_const_const__OutputArrayR(const cv::ogl::Buffer* instance, const cv::_OutputArray* arr, Result_void* ocvrs_return) {
		try {
			instance->copyTo(*arr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// copyTo(cv::OutputArray, cuda::Stream &) /usr/include/opencv2/core/opengl.hpp:200
	void cv_ogl_Buffer_copyTo_const_const__OutputArrayR_StreamR(const cv::ogl::Buffer* instance, const cv::_OutputArray* arr, cv::cuda::Stream* stream, Result_void* ocvrs_return) {
		try {
			instance->copyTo(*arr, *stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// clone(cv::ogl::Buffer::Target, bool) /usr/include/opencv2/core/opengl.hpp:207
	void cv_ogl_Buffer_clone_const_Target_bool(const cv::ogl::Buffer* instance, cv::ogl::Buffer::Target target, bool autoRelease, Result<cv::ogl::Buffer*>* ocvrs_return) {
		try {
			cv::ogl::Buffer ret = instance->clone(target, autoRelease);
			Ok(new cv::ogl::Buffer(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ogl::Buffer*>))
	}
	
	// bind(cv::ogl::Buffer::Target) /usr/include/opencv2/core/opengl.hpp:213
	void cv_ogl_Buffer_bind_const_Target(const cv::ogl::Buffer* instance, cv::ogl::Buffer::Target target, Result_void* ocvrs_return) {
		try {
			instance->bind(target);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// unbind(cv::ogl::Buffer::Target) /usr/include/opencv2/core/opengl.hpp:219
	void cv_ogl_Buffer_unbind_Target(cv::ogl::Buffer::Target target, Result_void* ocvrs_return) {
		try {
			cv::ogl::Buffer::unbind(target);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mapHost(cv::ogl::Buffer::Access) /usr/include/opencv2/core/opengl.hpp:236
	void cv_ogl_Buffer_mapHost_Access(cv::ogl::Buffer* instance, cv::ogl::Buffer::Access access, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->mapHost(access);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// unmapHost() /usr/include/opencv2/core/opengl.hpp:240
	void cv_ogl_Buffer_unmapHost(cv::ogl::Buffer* instance, Result_void* ocvrs_return) {
		try {
			instance->unmapHost();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mapDevice() /usr/include/opencv2/core/opengl.hpp:243
	void cv_ogl_Buffer_mapDevice(cv::ogl::Buffer* instance, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->mapDevice();
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// unmapDevice() /usr/include/opencv2/core/opengl.hpp:244
	void cv_ogl_Buffer_unmapDevice(cv::ogl::Buffer* instance, Result_void* ocvrs_return) {
		try {
			instance->unmapDevice();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mapDevice(cuda::Stream &) /usr/include/opencv2/core/opengl.hpp:252
	void cv_ogl_Buffer_mapDevice_StreamR(cv::ogl::Buffer* instance, cv::cuda::Stream* stream, Result<cv::cuda::GpuMat*>* ocvrs_return) {
		try {
			cv::cuda::GpuMat ret = instance->mapDevice(*stream);
			Ok(new cv::cuda::GpuMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::cuda::GpuMat*>))
	}
	
	// unmapDevice(cuda::Stream &) /usr/include/opencv2/core/opengl.hpp:256
	void cv_ogl_Buffer_unmapDevice_StreamR(cv::ogl::Buffer* instance, cv::cuda::Stream* stream, Result_void* ocvrs_return) {
		try {
			instance->unmapDevice(*stream);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// rows() /usr/include/opencv2/core/opengl.hpp:258
	void cv_ogl_Buffer_rows_const(const cv::ogl::Buffer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->rows();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// cols() /usr/include/opencv2/core/opengl.hpp:259
	void cv_ogl_Buffer_cols_const(const cv::ogl::Buffer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->cols();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// size() /usr/include/opencv2/core/opengl.hpp:260
	void cv_ogl_Buffer_size_const(const cv::ogl::Buffer* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// empty() /usr/include/opencv2/core/opengl.hpp:261
	void cv_ogl_Buffer_empty_const(const cv::ogl::Buffer* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// type() /usr/include/opencv2/core/opengl.hpp:263
	void cv_ogl_Buffer_type_const(const cv::ogl::Buffer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// depth() /usr/include/opencv2/core/opengl.hpp:264
	void cv_ogl_Buffer_depth_const(const cv::ogl::Buffer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->depth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// channels() /usr/include/opencv2/core/opengl.hpp:265
	void cv_ogl_Buffer_channels_const(const cv::ogl::Buffer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->channels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// elemSize() /usr/include/opencv2/core/opengl.hpp:266
	void cv_ogl_Buffer_elemSize_const(const cv::ogl::Buffer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->elemSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// elemSize1() /usr/include/opencv2/core/opengl.hpp:267
	void cv_ogl_Buffer_elemSize1_const(const cv::ogl::Buffer* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->elemSize1();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// bufId() /usr/include/opencv2/core/opengl.hpp:270
	void cv_ogl_Buffer_bufId_const(const cv::ogl::Buffer* instance, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->bufId();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	void cv_Texture2D_delete(cv::ogl::Texture2D* instance) {
		delete instance;
	}
	// Texture2D() /usr/include/opencv2/core/opengl.hpp:301
	void cv_ogl_Texture2D_Texture2D(Result<cv::ogl::Texture2D*>* ocvrs_return) {
		try {
			cv::ogl::Texture2D* ret = new cv::ogl::Texture2D();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ogl::Texture2D*>))
	}
	
	// Texture2D(int, int, cv::ogl::Texture2D::Format, unsigned int, bool) /usr/include/opencv2/core/opengl.hpp:304
	void cv_ogl_Texture2D_Texture2D_int_int_Format_unsigned_int_bool(int arows, int acols, cv::ogl::Texture2D::Format aformat, unsigned int atexId, bool autoRelease, Result<cv::ogl::Texture2D*>* ocvrs_return) {
		try {
			cv::ogl::Texture2D* ret = new cv::ogl::Texture2D(arows, acols, aformat, atexId, autoRelease);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ogl::Texture2D*>))
	}
	
	// Texture2D(cv::Size, cv::ogl::Texture2D::Format, unsigned int, bool) /usr/include/opencv2/core/opengl.hpp:307
	void cv_ogl_Texture2D_Texture2D_Size_Format_unsigned_int_bool(cv::Size* asize, cv::ogl::Texture2D::Format aformat, unsigned int atexId, bool autoRelease, Result<cv::ogl::Texture2D*>* ocvrs_return) {
		try {
			cv::ogl::Texture2D* ret = new cv::ogl::Texture2D(*asize, aformat, atexId, autoRelease);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ogl::Texture2D*>))
	}
	
	// Texture2D(int, int, cv::ogl::Texture2D::Format, bool) /usr/include/opencv2/core/opengl.hpp:315
	void cv_ogl_Texture2D_Texture2D_int_int_Format_bool(int arows, int acols, cv::ogl::Texture2D::Format aformat, bool autoRelease, Result<cv::ogl::Texture2D*>* ocvrs_return) {
		try {
			cv::ogl::Texture2D* ret = new cv::ogl::Texture2D(arows, acols, aformat, autoRelease);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ogl::Texture2D*>))
	}
	
	// Texture2D(cv::Size, cv::ogl::Texture2D::Format, bool) /usr/include/opencv2/core/opengl.hpp:322
	void cv_ogl_Texture2D_Texture2D_Size_Format_bool(cv::Size* asize, cv::ogl::Texture2D::Format aformat, bool autoRelease, Result<cv::ogl::Texture2D*>* ocvrs_return) {
		try {
			cv::ogl::Texture2D* ret = new cv::ogl::Texture2D(*asize, aformat, autoRelease);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ogl::Texture2D*>))
	}
	
	// Texture2D(cv::InputArray, bool) /usr/include/opencv2/core/opengl.hpp:328
	void cv_ogl_Texture2D_Texture2D_const__InputArrayR_bool(const cv::_InputArray* arr, bool autoRelease, Result<cv::ogl::Texture2D*>* ocvrs_return) {
		try {
			cv::ogl::Texture2D* ret = new cv::ogl::Texture2D(*arr, autoRelease);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ogl::Texture2D*>))
	}
	
	// create(int, int, cv::ogl::Texture2D::Format, bool) /usr/include/opencv2/core/opengl.hpp:337
	void cv_ogl_Texture2D_create_int_int_Format_bool(cv::ogl::Texture2D* instance, int arows, int acols, cv::ogl::Texture2D::Format aformat, bool autoRelease, Result_void* ocvrs_return) {
		try {
			instance->create(arows, acols, aformat, autoRelease);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(cv::Size, cv::ogl::Texture2D::Format, bool) /usr/include/opencv2/core/opengl.hpp:343
	void cv_ogl_Texture2D_create_Size_Format_bool(cv::ogl::Texture2D* instance, cv::Size* asize, cv::ogl::Texture2D::Format aformat, bool autoRelease, Result_void* ocvrs_return) {
		try {
			instance->create(*asize, aformat, autoRelease);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// release() /usr/include/opencv2/core/opengl.hpp:349
	void cv_ogl_Texture2D_release(cv::ogl::Texture2D* instance, Result_void* ocvrs_return) {
		try {
			instance->release();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setAutoRelease(bool) /usr/include/opencv2/core/opengl.hpp:361
	void cv_ogl_Texture2D_setAutoRelease_bool(cv::ogl::Texture2D* instance, bool flag, Result_void* ocvrs_return) {
		try {
			instance->setAutoRelease(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// copyFrom(cv::InputArray, bool) /usr/include/opencv2/core/opengl.hpp:368
	void cv_ogl_Texture2D_copyFrom_const__InputArrayR_bool(cv::ogl::Texture2D* instance, const cv::_InputArray* arr, bool autoRelease, Result_void* ocvrs_return) {
		try {
			instance->copyFrom(*arr, autoRelease);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// copyTo(cv::OutputArray, int, bool) /usr/include/opencv2/core/opengl.hpp:377
	void cv_ogl_Texture2D_copyTo_const_const__OutputArrayR_int_bool(const cv::ogl::Texture2D* instance, const cv::_OutputArray* arr, int ddepth, bool autoRelease, Result_void* ocvrs_return) {
		try {
			instance->copyTo(*arr, ddepth, autoRelease);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// bind() /usr/include/opencv2/core/opengl.hpp:381
	void cv_ogl_Texture2D_bind_const(const cv::ogl::Texture2D* instance, Result_void* ocvrs_return) {
		try {
			instance->bind();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// rows() /usr/include/opencv2/core/opengl.hpp:383
	void cv_ogl_Texture2D_rows_const(const cv::ogl::Texture2D* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->rows();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// cols() /usr/include/opencv2/core/opengl.hpp:384
	void cv_ogl_Texture2D_cols_const(const cv::ogl::Texture2D* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->cols();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// size() /usr/include/opencv2/core/opengl.hpp:385
	void cv_ogl_Texture2D_size_const(const cv::ogl::Texture2D* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// empty() /usr/include/opencv2/core/opengl.hpp:386
	void cv_ogl_Texture2D_empty_const(const cv::ogl::Texture2D* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// format() /usr/include/opencv2/core/opengl.hpp:388
	void cv_ogl_Texture2D_format_const(const cv::ogl::Texture2D* instance, Result<cv::ogl::Texture2D::Format>* ocvrs_return) {
		try {
			cv::ogl::Texture2D::Format ret = instance->format();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ogl::Texture2D::Format>))
	}
	
	// texId() /usr/include/opencv2/core/opengl.hpp:391
	void cv_ogl_Texture2D_texId_const(const cv::ogl::Texture2D* instance, Result<unsigned int>* ocvrs_return) {
		try {
			unsigned int ret = instance->texId();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<unsigned int>))
	}
	
	// name /usr/include/opencv2/core/utils/logtag.hpp:17
	void* cv_utils_logging_LogTag_getPropName_const(const cv::utils::logging::LogTag* instance) {
			const char* ret = instance->name;
			return ocvrs_create_string(ret);
	}
	
	// level /usr/include/opencv2/core/utils/logtag.hpp:18
	void cv_utils_logging_LogTag_getPropLevel_const(const cv::utils::logging::LogTag* instance, cv::utils::logging::LogLevel* ocvrs_return) {
			cv::utils::logging::LogLevel ret = instance->level;
			*ocvrs_return = ret;
	}
	
	// level /usr/include/opencv2/core/utils/logtag.hpp:18
	void cv_utils_logging_LogTag_setPropLevel_LogLevel(cv::utils::logging::LogTag* instance, cv::utils::logging::LogLevel val) {
			instance->level = val;
	}
	
	void cv_LogTag_delete(cv::utils::logging::LogTag* instance) {
		delete instance;
	}
	// LogTag(const char *, cv::utils::logging::LogLevel) /usr/include/opencv2/core/utils/logtag.hpp:20
	void cv_utils_logging_LogTag_LogTag_const_charX_LogLevel(const char* _name, cv::utils::logging::LogLevel _level, Result<cv::utils::logging::LogTag*>* ocvrs_return) {
		try {
			cv::utils::logging::LogTag* ret = new cv::utils::logging::LogTag(_name, _level);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::utils::logging::LogTag*>))
	}
	
	void cv_OriginalClassName_delete(cv::utils::nested::OriginalClassName* instance) {
		delete instance;
	}
	// OriginalClassName(const OriginalClassName::Params &) /usr/include/opencv2/core/bindings_utils.hpp:242
	void cv_utils_nested_OriginalClassName_OriginalClassName_const_ParamsR(const cv::utils::nested::OriginalClassName::Params* params, Result<cv::utils::nested::OriginalClassName*>* ocvrs_return) {
		try {
			cv::utils::nested::OriginalClassName* ret = new cv::utils::nested::OriginalClassName(*params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::utils::nested::OriginalClassName*>))
	}
	
	// getIntParam() /usr/include/opencv2/core/bindings_utils.hpp:247
	void cv_utils_nested_OriginalClassName_getIntParam_const(const cv::utils::nested::OriginalClassName* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIntParam();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getFloatParam() /usr/include/opencv2/core/bindings_utils.hpp:252
	void cv_utils_nested_OriginalClassName_getFloatParam_const(const cv::utils::nested::OriginalClassName* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getFloatParam();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// originalName() /usr/include/opencv2/core/bindings_utils.hpp:257
	void cv_utils_nested_OriginalClassName_originalName(Result<void*>* ocvrs_return) {
		try {
			std::string ret = cv::utils::nested::OriginalClassName::originalName();
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// create(const OriginalClassName::Params &) /usr/include/opencv2/core/bindings_utils.hpp:263
	void cv_utils_nested_OriginalClassName_create_const_ParamsR(const cv::utils::nested::OriginalClassName::Params* params, Result<cv::Ptr<cv::utils::nested::OriginalClassName>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::utils::nested::OriginalClassName> ret = cv::utils::nested::OriginalClassName::create(*params);
			Ok(new cv::Ptr<cv::utils::nested::OriginalClassName>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::utils::nested::OriginalClassName>*>))
	}
	
	// Params(int, float) /usr/include/opencv2/core/bindings_utils.hpp:235
	void cv_utils_nested_OriginalClassName_Params_Params_int_float(int int_param, float float_param, Result<cv::utils::nested::OriginalClassName::Params>* ocvrs_return) {
		try {
			cv::utils::nested::OriginalClassName::Params ret(int_param, float_param);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::utils::nested::OriginalClassName::Params>))
	}
	
}
