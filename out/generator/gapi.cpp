#include "gapi.hpp"
#include "gapi_types.hpp"

extern "C" {
	// descr_of(const cv::Mat &) /usr/include/opencv2/gapi/gmat.hpp:268
	void cv_descr_of_const_MatR(const cv::Mat* mat, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc ret = cv::descr_of(*mat);
			Ok(new cv::GMatDesc(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMatDesc*>))
	}
	
	// descr_of(const cv::MediaFrame &) /usr/include/opencv2/gapi/gframe.hpp:107
	void cv_descr_of_const_MediaFrameR(const cv::MediaFrame* frame, Result<cv::GFrameDesc*>* ocvrs_return) {
		try {
			cv::GFrameDesc ret = cv::descr_of(*frame);
			Ok(new cv::GFrameDesc(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GFrameDesc*>))
	}
	
	// descr_of(const cv::RMat &) /usr/include/opencv2/gapi/gmat.hpp:265
	void cv_descr_of_const_RMatR(const cv::RMat* mat, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc ret = cv::descr_of(*mat);
			Ok(new cv::GMatDesc(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMatDesc*>))
	}
	
	// descr_of(const cv::Scalar &) /usr/include/opencv2/gapi/gscalar.hpp:134
	void cv_descr_of_const_ScalarR(const cv::Scalar* scalar, Result<cv::GScalarDesc*>* ocvrs_return) {
		try {
			cv::GScalarDesc ret = cv::descr_of(*scalar);
			Ok(new cv::GScalarDesc(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GScalarDesc*>))
	}
	
	// descr_of(const cv::UMat &) /usr/include/opencv2/gapi/gmat.hpp:256
	void cv_descr_of_const_UMatR(const cv::UMat* mat, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc ret = cv::descr_of(*mat);
			Ok(new cv::GMatDesc(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMatDesc*>))
	}
	
	// BGR2Gray(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1415
	void cv_gapi_BGR2Gray_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::BGR2Gray(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// BGR2I420(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1448
	void cv_gapi_BGR2I420_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::BGR2I420(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// BGR2LUV(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1510
	void cv_gapi_BGR2LUV_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::BGR2LUV(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// BGR2YUV(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1552
	void cv_gapi_BGR2YUV_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::BGR2YUV(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// BayerGR2RGB(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1639
	void cv_gapi_BayerGR2RGB_const_GMatR(const cv::GMat* src_gr, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::BayerGR2RGB(*src_gr);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// Canny(const cv::GMat &, double, double, int, bool) /usr/include/opencv2/gapi/imgproc.hpp:1026
	void cv_gapi_Canny_const_GMatR_double_double_int_bool(const cv::GMat* image, double threshold1, double threshold2, int apertureSize, bool L2gradient, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::Canny(*image, threshold1, threshold2, apertureSize, L2gradient);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// I4202BGR(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1480
	void cv_gapi_I4202BGR_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::I4202BGR(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// I4202RGB(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1496
	void cv_gapi_I4202RGB_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::I4202RGB(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// LUT(const cv::GMat &, const cv::Mat &) /usr/include/opencv2/gapi/core.hpp:1699
	void cv_gapi_LUT_const_GMatR_const_MatR(const cv::GMat* src, const cv::Mat* lut, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::LUT(*src, *lut);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// LUV2BGR(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1524
	void cv_gapi_LUV2BGR_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::LUV2BGR(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// Laplacian(const cv::GMat &, int, int, double, double, int) /usr/include/opencv2/gapi/imgproc.hpp:967
	void cv_gapi_Laplacian_const_GMatR_int_int_double_double_int(const cv::GMat* src, int ddepth, int ksize, double scale, double delta, int borderType, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::Laplacian(*src, ddepth, ksize, scale, delta, borderType);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// NV12toBGR(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1625
	void cv_gapi_NV12toBGR_const_GMatR_const_GMatR(const cv::GMat* src_y, const cv::GMat* src_uv, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::NV12toBGR(*src_y, *src_uv);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// NV12toBGRp(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1703
	void cv_gapi_NV12toBGRp_const_GMatR_const_GMatR(const cv::GMat* src_y, const cv::GMat* src_uv, Result<cv::GMatP*>* ocvrs_return) {
		try {
			cv::GMatP ret = cv::gapi::NV12toBGRp(*src_y, *src_uv);
			Ok(new cv::GMatP(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMatP*>))
	}
	
	// NV12toGray(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1610
	void cv_gapi_NV12toGray_const_GMatR_const_GMatR(const cv::GMat* src_y, const cv::GMat* src_uv, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::NV12toGray(*src_y, *src_uv);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// NV12toRGB(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1595
	void cv_gapi_NV12toRGB_const_GMatR_const_GMatR(const cv::GMat* src_y, const cv::GMat* src_uv, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::NV12toRGB(*src_y, *src_uv);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// NV12toRGBp(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1685
	void cv_gapi_NV12toRGBp_const_GMatR_const_GMatR(const cv::GMat* src_y, const cv::GMat* src_uv, Result<cv::GMatP*>* ocvrs_return) {
		try {
			cv::GMatP ret = cv::gapi::NV12toRGBp(*src_y, *src_uv);
			Ok(new cv::GMatP(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMatP*>))
	}
	
	// RGB2Gray(const cv::GMat &, float, float, float) /usr/include/opencv2/gapi/imgproc.hpp:1402
	void cv_gapi_RGB2Gray_const_GMatR_float_float_float(const cv::GMat* src, float rY, float gY, float bY, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::RGB2Gray(*src, rY, gY, bY);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// RGB2HSV(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1653
	void cv_gapi_RGB2HSV_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::RGB2HSV(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// RGB2I420(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1464
	void cv_gapi_RGB2I420_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::RGB2I420(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// RGB2Lab(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1566
	void cv_gapi_RGB2Lab_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::RGB2Lab(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// RGB2YUV422(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1667
	void cv_gapi_RGB2YUV422_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::RGB2YUV422(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// RGB2YUV(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1432
	void cv_gapi_RGB2YUV_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::RGB2YUV(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// Sobel(const cv::GMat &, int, int, int, int, double, double, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:886
	void cv_gapi_Sobel_const_GMatR_int_int_int_int_double_double_int_const_ScalarR(const cv::GMat* src, int ddepth, int dx, int dy, int ksize, double scale, double delta, int borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::Sobel(*src, ddepth, dx, dy, ksize, scale, delta, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// YUV2BGR(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1538
	void cv_gapi_YUV2BGR_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::YUV2BGR(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// YUV2RGB(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1580
	void cv_gapi_YUV2RGB_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::YUV2RGB(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// absDiffC(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1254
	void cv_gapi_absDiffC_const_GMatR_const_GScalarR(const cv::GMat* src, const cv::GScalar* c, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::absDiffC(*src, *c);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// absDiff(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1237
	void cv_gapi_absDiff_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::absDiff(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// addC(const cv::GScalar &, const cv::GMat &, int) /usr/include/opencv2/gapi/core.hpp:638
	void cv_gapi_addC_const_GScalarR_const_GMatR_int(const cv::GScalar* c, const cv::GMat* src1, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::addC(*c, *src1, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// addWeighted(const cv::GMat &, double, const cv::GMat &, double, double, int) /usr/include/opencv2/gapi/core.hpp:1302
	void cv_gapi_addWeighted_const_GMatR_double_const_GMatR_double_double_int(const cv::GMat* src1, double alpha, const cv::GMat* src2, double beta, double gamma, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::addWeighted(*src1, alpha, *src2, beta, gamma, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// bilateralFilter(const cv::GMat &, int, double, double, int) /usr/include/opencv2/gapi/imgproc.hpp:1001
	void cv_gapi_bilateralFilter_const_GMatR_int_double_double_int(const cv::GMat* src, int d, double sigmaColor, double sigmaSpace, int borderType, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::bilateralFilter(*src, d, sigmaColor, sigmaSpace, borderType);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// bitwise_and(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1089
	void cv_gapi_bitwise_and_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::bitwise_and(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// bitwise_and(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1095
	void cv_gapi_bitwise_and_const_GMatR_const_GScalarR(const cv::GMat* src1, const cv::GScalar* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::bitwise_and(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// bitwise_not(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1165
	void cv_gapi_bitwise_not_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::bitwise_not(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// bitwise_or(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1113
	void cv_gapi_bitwise_or_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::bitwise_or(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// bitwise_or(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1119
	void cv_gapi_bitwise_or_const_GMatR_const_GScalarR(const cv::GMat* src1, const cv::GScalar* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::bitwise_or(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// bitwise_xor(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1138
	void cv_gapi_bitwise_xor_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::bitwise_xor(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// bitwise_xor(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1144
	void cv_gapi_bitwise_xor_const_GMatR_const_GScalarR(const cv::GMat* src1, const cv::GScalar* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::bitwise_xor(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// blur(const cv::GMat &, const cv::Size &, const cv::Point &, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:657
	void cv_gapi_blur_const_GMatR_const_SizeR_const_PointR_int_const_ScalarR(const cv::GMat* src, const cv::Size* ksize, const cv::Point* anchor, int borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::blur(*src, *ksize, *anchor, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// boxFilter(const cv::GMat &, int, const cv::Size &, const cv::Point &, bool, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:630
	void cv_gapi_boxFilter_const_GMatR_int_const_SizeR_const_PointR_bool_int_const_ScalarR(const cv::GMat* src, int dtype, const cv::Size* ksize, const cv::Point* anchor, bool normalize, int borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::boxFilter(*src, dtype, *ksize, *anchor, normalize, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// cmpEQ(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1041
	void cv_gapi_cmpEQ_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpEQ(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// cmpEQ(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1045
	void cv_gapi_cmpEQ_const_GMatR_const_GScalarR(const cv::GMat* src1, const cv::GScalar* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpEQ(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// cmpGE(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:989
	void cv_gapi_cmpGE_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpGE(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// cmpGE(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:993
	void cv_gapi_cmpGE_const_GMatR_const_GScalarR(const cv::GMat* src1, const cv::GScalar* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpGE(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// cmpGT(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:937
	void cv_gapi_cmpGT_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpGT(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// cmpGT(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:941
	void cv_gapi_cmpGT_const_GMatR_const_GScalarR(const cv::GMat* src1, const cv::GScalar* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpGT(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// cmpLE(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1015
	void cv_gapi_cmpLE_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpLE(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// cmpLE(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1019
	void cv_gapi_cmpLE_const_GMatR_const_GScalarR(const cv::GMat* src1, const cv::GScalar* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpLE(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// cmpLT(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:963
	void cv_gapi_cmpLT_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpLT(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// cmpLT(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:967
	void cv_gapi_cmpLT_const_GMatR_const_GScalarR(const cv::GMat* src1, const cv::GScalar* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpLT(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// cmpNE(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1067
	void cv_gapi_cmpNE_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpNE(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// cmpNE(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1071
	void cv_gapi_cmpNE_const_GMatR_const_GScalarR(const cv::GMat* src1, const cv::GScalar* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::cmpNE(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// combine(const cv::GKernelPackage &, const cv::GKernelPackage &) /usr/include/opencv2/gapi/gkernel.hpp:417
	void cv_gapi_combine_const_GKernelPackageR_const_GKernelPackageR(const cv::GKernelPackage* lhs, const cv::GKernelPackage* rhs, Result<cv::GKernelPackage*>* ocvrs_return) {
		try {
			cv::GKernelPackage ret = cv::gapi::combine(*lhs, *rhs);
			Ok(new cv::GKernelPackage(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GKernelPackage*>))
	}
	
	// concatHor(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1632
	void cv_gapi_concatHor_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::concatHor(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// concatHor(const std::vector<GMat> &) /usr/include/opencv2/gapi/core.hpp:1640
	void cv_gapi_concatHor_const_vector_GMat_R(const std::vector<cv::GMat>* v, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::concatHor(*v);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// concatVert(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1672
	void cv_gapi_concatVert_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::concatVert(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// concatVert(const std::vector<GMat> &) /usr/include/opencv2/gapi/core.hpp:1680
	void cv_gapi_concatVert_const_vector_GMat_R(const std::vector<cv::GMat>* v, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::concatVert(*v);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// convertTo(const cv::GMat &, int, double, double) /usr/include/opencv2/gapi/core.hpp:1716
	void cv_gapi_convertTo_const_GMatR_int_double_double(const cv::GMat* src, int rdepth, double alpha, double beta, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::convertTo(*src, rdepth, alpha, beta);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// copy(const cv::GFrame &) /usr/include/opencv2/gapi/streaming/format.hpp:88
	void cv_gapi_copy_const_GFrameR(const cv::GFrame* in, Result<cv::GFrame*>* ocvrs_return) {
		try {
			cv::GFrame ret = cv::gapi::copy(*in);
			Ok(new cv::GFrame(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GFrame*>))
	}
	
	// crop(const cv::GMat &, const cv::Rect &) /usr/include/opencv2/gapi/core.hpp:1604
	void cv_gapi_crop_const_GMatR_const_RectR(const cv::GMat* src, const cv::Rect* rect, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::crop(*src, *rect);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// dilate3x3(const cv::GMat &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:804
	void cv_gapi_dilate3x3_const_GMatR_int_int_const_ScalarR(const cv::GMat* src, int iterations, int borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::dilate3x3(*src, iterations, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// dilate(const cv::GMat &, const cv::Mat &, const cv::Point &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:780
	void cv_gapi_dilate_const_GMatR_const_MatR_const_PointR_int_int_const_ScalarR(const cv::GMat* src, const cv::Mat* kernel, const cv::Point* anchor, int iterations, int borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::dilate(*src, *kernel, *anchor, iterations, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// divC(const cv::GMat &, const cv::GScalar &, double, int) /usr/include/opencv2/gapi/core.hpp:788
	void cv_gapi_divC_const_GMatR_const_GScalarR_double_int(const cv::GMat* src, const cv::GScalar* divisor, double scale, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::divC(*src, *divisor, scale, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// divRC(const cv::GScalar &, const cv::GMat &, double, int) /usr/include/opencv2/gapi/core.hpp:809
	void cv_gapi_divRC_const_GScalarR_const_GMatR_double_int(const cv::GScalar* divident, const cv::GMat* src, double scale, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::divRC(*divident, *src, scale, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// div(const cv::GMat &, const cv::GMat &, double, int) /usr/include/opencv2/gapi/core.hpp:767
	void cv_gapi_div_const_GMatR_const_GMatR_double_int(const cv::GMat* src1, const cv::GMat* src2, double scale, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::div(*src1, *src2, scale, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// equalizeHist(const cv::GMat &) /usr/include/opencv2/gapi/imgproc.hpp:1101
	void cv_gapi_equalizeHist_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::equalizeHist(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// erode3x3(const cv::GMat &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:753
	void cv_gapi_erode3x3_const_GMatR_int_int_const_ScalarR(const cv::GMat* src, int iterations, int borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::erode3x3(*src, iterations, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// erode(const cv::GMat &, const cv::Mat &, const cv::Point &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:733
	void cv_gapi_erode_const_GMatR_const_MatR_const_PointR_int_int_const_ScalarR(const cv::GMat* src, const cv::Mat* kernel, const cv::Point* anchor, int iterations, int borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::erode(*src, *kernel, *anchor, iterations, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// filter2D(const cv::GMat &, int, const cv::Mat &, const cv::Point &, const cv::Scalar &, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:596
	void cv_gapi_filter2D_const_GMatR_int_const_MatR_const_PointR_const_ScalarR_int_const_ScalarR(const cv::GMat* src, int ddepth, const cv::Mat* kernel, const cv::Point* anchor, const cv::Scalar* delta, int borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::filter2D(*src, ddepth, *kernel, *anchor, *delta, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// flip(const cv::GMat &, int) /usr/include/opencv2/gapi/core.hpp:1590
	void cv_gapi_flip_const_GMatR_int(const cv::GMat* src, int flipCode, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::flip(*src, flipCode);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// gaussianBlur(const cv::GMat &, const cv::Size &, double, double, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:690
	void cv_gapi_gaussianBlur_const_GMatR_const_SizeR_double_double_int_const_ScalarR(const cv::GMat* src, const cv::Size* ksize, double sigmaX, double sigmaY, int borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::gaussianBlur(*src, *ksize, sigmaX, sigmaY, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// inRange(const cv::GMat &, const cv::GScalar &, const cv::GScalar &) /usr/include/opencv2/gapi/core.hpp:1441
	void cv_gapi_inRange_const_GMatR_const_GScalarR_const_GScalarR(const cv::GMat* src, const cv::GScalar* threshLow, const cv::GScalar* threshUp, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::inRange(*src, *threshLow, *threshUp);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// mask(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:822
	void cv_gapi_mask_const_GMatR_const_GMatR(const cv::GMat* src, const cv::GMat* mask, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::mask(*src, *mask);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// max(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1220
	void cv_gapi_max_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::max(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// merge3(const cv::GMat &, const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1484
	void cv_gapi_merge3_const_GMatR_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, const cv::GMat* src3, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::merge3(*src1, *src2, *src3);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// merge4(const cv::GMat &, const cv::GMat &, const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1465
	void cv_gapi_merge4_const_GMatR_const_GMatR_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, const cv::GMat* src3, const cv::GMat* src4, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::merge4(*src1, *src2, *src3, *src4);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// min(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1203
	void cv_gapi_min_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::min(*src1, *src2);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// morphologyEx(const cv::GMat &, const cv::MorphTypes, const cv::Mat &, const cv::Point &, const int, const cv::BorderTypes, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:834
	void cv_gapi_morphologyEx_const_GMatR_const_MorphTypes_const_MatR_const_PointR_const_int_const_BorderTypes_const_ScalarR(const cv::GMat* src, const cv::MorphTypes op, const cv::Mat* kernel, const cv::Point* anchor, const int iterations, const cv::BorderTypes borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::morphologyEx(*src, op, *kernel, *anchor, iterations, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// mulC(const cv::GMat &, const cv::GScalar &, int) /usr/include/opencv2/gapi/core.hpp:742
	void cv_gapi_mulC_const_GMatR_const_GScalarR_int(const cv::GMat* src, const cv::GScalar* multiplier, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::mulC(*src, *multiplier, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// mulC(const cv::GMat &, double, int) /usr/include/opencv2/gapi/core.hpp:740
	void cv_gapi_mulC_const_GMatR_double_int(const cv::GMat* src, double multiplier, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::mulC(*src, multiplier, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// mulC(const cv::GScalar &, const cv::GMat &, int) /usr/include/opencv2/gapi/core.hpp:744
	void cv_gapi_mulC_const_GScalarR_const_GMatR_int(const cv::GScalar* multiplier, const cv::GMat* src, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::mulC(*multiplier, *src, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// mul(const cv::GMat &, const cv::GMat &, double, int) /usr/include/opencv2/gapi/core.hpp:722
	void cv_gapi_mul_const_GMatR_const_GMatR_double_int(const cv::GMat* src1, const cv::GMat* src2, double scale, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::mul(*src1, *src2, scale, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// normInf(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1370
	void cv_gapi_normInf_const_GMatR(const cv::GMat* src, Result<cv::GScalar*>* ocvrs_return) {
		try {
			cv::GScalar ret = cv::gapi::normInf(*src);
			Ok(new cv::GScalar(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GScalar*>))
	}
	
	// normL1(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1325
	void cv_gapi_normL1_const_GMatR(const cv::GMat* src, Result<cv::GScalar*>* ocvrs_return) {
		try {
			cv::GScalar ret = cv::gapi::normL1(*src);
			Ok(new cv::GScalar(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GScalar*>))
	}
	
	// normL2(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1347
	void cv_gapi_normL2_const_GMatR(const cv::GMat* src, Result<cv::GScalar*>* ocvrs_return) {
		try {
			cv::GScalar ret = cv::gapi::normL2(*src);
			Ok(new cv::GScalar(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GScalar*>))
	}
	
	// normalize(const cv::GMat &, double, double, int, int) /usr/include/opencv2/gapi/core.hpp:1738
	void cv_gapi_normalize_const_GMatR_double_double_int_int(const cv::GMat* src, double alpha, double beta, int norm_type, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::normalize(*src, alpha, beta, norm_type, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// descr_of(const cv::gapi::own::Mat &) /usr/include/opencv2/gapi/gmat.hpp:262
	void cv_gapi_own_descr_of_const_MatR(const cv::gapi::own::Mat* mat, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc ret = cv::gapi::own::descr_of(*mat);
			Ok(new cv::GMatDesc(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMatDesc*>))
	}
	
	// phase(const cv::GMat &, const cv::GMat &, bool) /usr/include/opencv2/gapi/core.hpp:899
	void cv_gapi_phase_const_GMatR_const_GMatR_bool(const cv::GMat* x, const cv::GMat* y, bool angleInDegrees, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::phase(*x, *y, angleInDegrees);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// remap(const cv::GMat &, const cv::Mat &, const cv::Mat &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/core.hpp:1551
	void cv_gapi_remap_const_GMatR_const_MatR_const_MatR_int_int_const_ScalarR(const cv::GMat* src, const cv::Mat* map1, const cv::Mat* map2, int interpolation, int borderMode, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::remap(*src, *map1, *map2, interpolation, borderMode, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// resizeP(const cv::GMatP &, const cv::Size &, int) /usr/include/opencv2/gapi/imgproc.hpp:1763
	void cv_gapi_resizeP_const_GMatPR_const_SizeR_int(const cv::GMatP* src, const cv::Size* dsize, int interpolation, Result<cv::GMatP*>* ocvrs_return) {
		try {
			cv::GMatP ret = cv::gapi::resizeP(*src, *dsize, interpolation);
			Ok(new cv::GMatP(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMatP*>))
	}
	
	// select(const cv::GMat &, const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1181
	void cv_gapi_select_const_GMatR_const_GMatR_const_GMatR(const cv::GMat* src1, const cv::GMat* src2, const cv::GMat* mask, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::select(*src1, *src2, *mask);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// sepFilter(const cv::GMat &, int, const cv::Mat &, const cv::Mat &, const cv::Point &, const cv::Scalar &, int, const cv::Scalar &) /usr/include/opencv2/gapi/imgproc.hpp:559
	void cv_gapi_sepFilter_const_GMatR_int_const_MatR_const_MatR_const_PointR_const_ScalarR_int_const_ScalarR(const cv::GMat* src, int ddepth, const cv::Mat* kernelX, const cv::Mat* kernelY, const cv::Point* anchor, const cv::Scalar* delta, int borderType, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::sepFilter(*src, ddepth, *kernelX, *kernelY, *anchor, *delta, borderType, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// sqrt(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:910
	void cv_gapi_sqrt_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::sqrt(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// BGR(const cv::GFrame &) /usr/include/opencv2/gapi/streaming/format.hpp:43
	void cv_gapi_streaming_BGR_const_GFrameR(const cv::GFrame* in, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::streaming::BGR(*in);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// UV(const cv::GFrame &) /usr/include/opencv2/gapi/streaming/format.hpp:63
	void cv_gapi_streaming_UV_const_GFrameR(const cv::GFrame* frame, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::streaming::UV(*frame);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// Y(const cv::GFrame &) /usr/include/opencv2/gapi/streaming/format.hpp:53
	void cv_gapi_streaming_Y_const_GFrameR(const cv::GFrame* frame, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::streaming::Y(*frame);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// desync(const cv::GFrame &) /usr/include/opencv2/gapi/streaming/desync.hpp:79
	void cv_gapi_streaming_desync_const_GFrameR(const cv::GFrame* f, Result<cv::GFrame*>* ocvrs_return) {
		try {
			cv::GFrame ret = cv::gapi::streaming::desync(*f);
			Ok(new cv::GFrame(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GFrame*>))
	}
	
	// desync(const cv::GMat &) /usr/include/opencv2/gapi/streaming/desync.hpp:78
	void cv_gapi_streaming_desync_const_GMatR(const cv::GMat* g, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::streaming::desync(*g);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// kernels() /usr/include/opencv2/gapi/streaming/format.hpp:16
	void cv_gapi_streaming_kernels(Result<cv::GKernelPackage*>* ocvrs_return) {
		try {
			cv::GKernelPackage ret = cv::gapi::streaming::kernels();
			Ok(new cv::GKernelPackage(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GKernelPackage*>))
	}
	
	// subC(const cv::GMat &, const cv::GScalar &, int) /usr/include/opencv2/gapi/core.hpp:682
	void cv_gapi_subC_const_GMatR_const_GScalarR_int(const cv::GMat* src, const cv::GScalar* c, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::subC(*src, *c, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// subRC(const cv::GScalar &, const cv::GMat &, int) /usr/include/opencv2/gapi/core.hpp:701
	void cv_gapi_subRC_const_GScalarR_const_GMatR_int(const cv::GScalar* c, const cv::GMat* src, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::subRC(*c, *src, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// sub(const cv::GMat &, const cv::GMat &, int) /usr/include/opencv2/gapi/core.hpp:663
	void cv_gapi_sub_const_GMatR_const_GMatR_int(const cv::GMat* src1, const cv::GMat* src2, int ddepth, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::sub(*src1, *src2, ddepth);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// sum(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1266
	void cv_gapi_sum_const_GMatR(const cv::GMat* src, Result<cv::GScalar*>* ocvrs_return) {
		try {
			cv::GScalar ret = cv::gapi::sum(*src);
			Ok(new cv::GScalar(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GScalar*>))
	}
	
	// threshold(const cv::GMat &, const cv::GScalar &, const cv::GScalar &, int) /usr/include/opencv2/gapi/core.hpp:1419
	void cv_gapi_threshold_const_GMatR_const_GScalarR_const_GScalarR_int(const cv::GMat* src, const cv::GScalar* thresh, const cv::GScalar* maxval, int type, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::threshold(*src, *thresh, *maxval, type);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// transpose(const cv::GMat &) /usr/include/opencv2/gapi/core.hpp:1876
	void cv_gapi_transpose_const_GMatR(const cv::GMat* src, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::transpose(*src);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// warpAffine(const cv::GMat &, const cv::Mat &, const cv::Size &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/core.hpp:1787
	void cv_gapi_warpAffine_const_GMatR_const_MatR_const_SizeR_int_int_const_ScalarR(const cv::GMat* src, const cv::Mat* M, const cv::Size* dsize, int flags, int borderMode, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::warpAffine(*src, *M, *dsize, flags, borderMode, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// warpPerspective(const cv::GMat &, const cv::Mat &, const cv::Size &, int, int, const cv::Scalar &) /usr/include/opencv2/gapi/core.hpp:1762
	void cv_gapi_warpPerspective_const_GMatR_const_MatR_const_SizeR_int_int_const_ScalarR(const cv::GMat* src, const cv::Mat* M, const cv::Size* dsize, int flags, int borderMode, const cv::Scalar* borderValue, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::gapi::warpPerspective(*src, *M, *dsize, flags, borderMode, *borderValue);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// operator+(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:16
	void cv_operatorA_const_GMatR_const_GMatR(const cv::GMat* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator+(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// operator+(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/operators.hpp:18
	void cv_operatorA_const_GMatR_const_GScalarR(const cv::GMat* lhs, const cv::GScalar* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator+(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// operator+(const cv::GScalar &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:19
	void cv_operatorA_const_GScalarR_const_GMatR(const cv::GScalar* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator+(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// operator/(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:33
	void cv_operatorD_const_GMatR_const_GMatR(const cv::GMat* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator/(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// operator/(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/operators.hpp:31
	void cv_operatorD_const_GMatR_const_GScalarR(const cv::GMat* lhs, const cv::GScalar* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator/(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// operator/(const cv::GScalar &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:32
	void cv_operatorD_const_GScalarR_const_GMatR(const cv::GScalar* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator/(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// operator==(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:52
	void cv_operatorEQ_const_GMatR_const_GMatR(const cv::GMat* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator==(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// operator==(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/operators.hpp:59
	void cv_operatorEQ_const_GMatR_const_GScalarR(const cv::GMat* lhs, const cv::GScalar* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator==(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// operator==(const cv::GScalar &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:66
	void cv_operatorEQ_const_GScalarR_const_GMatR(const cv::GScalar* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator==(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// operator-(const cv::GMat &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:21
	void cv_operatorS_const_GMatR_const_GMatR(const cv::GMat* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator-(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// operator-(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/operators.hpp:23
	void cv_operatorS_const_GMatR_const_GScalarR(const cv::GMat* lhs, const cv::GScalar* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator-(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// operator-(const cv::GScalar &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:24
	void cv_operatorS_const_GScalarR_const_GMatR(const cv::GScalar* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator-(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// operator*(const cv::GMat &, const cv::GScalar &) /usr/include/opencv2/gapi/operators.hpp:28
	void cv_operatorX_const_GMatR_const_GScalarR(const cv::GMat* lhs, const cv::GScalar* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator*(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// operator*(const cv::GMat &, float) /usr/include/opencv2/gapi/operators.hpp:26
	void cv_operatorX_const_GMatR_float(const cv::GMat* lhs, float rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator*(*lhs, rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// operator*(const cv::GScalar &, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:29
	void cv_operatorX_const_GScalarR_const_GMatR(const cv::GScalar* lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator*(*lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// operator*(float, const cv::GMat &) /usr/include/opencv2/gapi/operators.hpp:27
	void cv_operatorX_float_const_GMatR(float lhs, const cv::GMat* rhs, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = cv::operator*(lhs, *rhs);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// validate_input_arg(const cv::GRunArg &) /usr/include/opencv2/gapi/gproto.hpp:154
	void cv_validate_input_arg_const_GRunArgR(const cv::GRunArg* arg, Result_void* ocvrs_return) {
		try {
			cv::validate_input_arg(*arg);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// validate_input_args(const cv::GRunArgs &) /usr/include/opencv2/gapi/gproto.hpp:155
	void cv_validate_input_args_const_GRunArgsR(const cv::GRunArgs* args, Result_void* ocvrs_return) {
		try {
			cv::validate_input_args(*args);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// kind /usr/include/opencv2/gapi/garg.hpp:87
	void cv_GArg_getPropKind_const(const cv::GArg* instance, cv::detail::ArgKind* ocvrs_return) {
			cv::detail::ArgKind ret = instance->kind;
			*ocvrs_return = ret;
	}
	
	// kind /usr/include/opencv2/gapi/garg.hpp:87
	void cv_GArg_setPropKind_ArgKind(cv::GArg* instance, cv::detail::ArgKind val) {
			instance->kind = val;
	}
	
	// opaque_kind /usr/include/opencv2/gapi/garg.hpp:88
	void cv_GArg_getPropOpaque_kind_const(const cv::GArg* instance, cv::detail::OpaqueKind* ocvrs_return) {
			cv::detail::OpaqueKind ret = instance->opaque_kind;
			*ocvrs_return = ret;
	}
	
	// opaque_kind /usr/include/opencv2/gapi/garg.hpp:88
	void cv_GArg_setPropOpaque_kind_OpaqueKind(cv::GArg* instance, cv::detail::OpaqueKind val) {
			instance->opaque_kind = val;
	}
	
	void cv_GArg_delete(cv::GArg* instance) {
		delete instance;
	}
	// GArg() /usr/include/opencv2/gapi/garg.hpp:49
	void cv_GArg_GArg(Result<cv::GArg*>* ocvrs_return) {
		try {
			cv::GArg* ret = new cv::GArg();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GArg*>))
	}
	
	void cv_GCall_delete(cv::GCall* instance) {
		delete instance;
	}
	// GCall(const cv::GKernel &) /usr/include/opencv2/gapi/gcall.hpp:31
	void cv_GCall_GCall_const_GKernelR(const cv::GKernel* k, Result<cv::GCall*>* ocvrs_return) {
		try {
			cv::GCall* ret = new cv::GCall(*k);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GCall*>))
	}
	
	// yield(int) /usr/include/opencv2/gapi/gcall.hpp:42
	void cv_GCall_yield_int(cv::GCall* instance, int output, Result<cv::GMat*>* ocvrs_return) {
		try {
			cv::GMat ret = instance->yield(output);
			Ok(new cv::GMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMat*>))
	}
	
	// yieldP(int) /usr/include/opencv2/gapi/gcall.hpp:43
	void cv_GCall_yieldP_int(cv::GCall* instance, int output, Result<cv::GMatP*>* ocvrs_return) {
		try {
			cv::GMatP ret = instance->yieldP(output);
			Ok(new cv::GMatP(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMatP*>))
	}
	
	// yieldScalar(int) /usr/include/opencv2/gapi/gcall.hpp:44
	void cv_GCall_yieldScalar_int(cv::GCall* instance, int output, Result<cv::GScalar*>* ocvrs_return) {
		try {
			cv::GScalar ret = instance->yieldScalar(output);
			Ok(new cv::GScalar(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GScalar*>))
	}
	
	// yieldFrame(int) /usr/include/opencv2/gapi/gcall.hpp:45
	void cv_GCall_yieldFrame_int(cv::GCall* instance, int output, Result<cv::GFrame*>* ocvrs_return) {
		try {
			cv::GFrame ret = instance->yieldFrame(output);
			Ok(new cv::GFrame(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GFrame*>))
	}
	
	// kernel() /usr/include/opencv2/gapi/gcall.hpp:63
	void cv_GCall_kernel(cv::GCall* instance, Result<cv::GKernel*>* ocvrs_return) {
		try {
			cv::GKernel ret = instance->kernel();
			Ok(new cv::GKernel(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GKernel*>))
	}
	
	// params() /usr/include/opencv2/gapi/gcall.hpp:64
	void cv_GCall_params(cv::GCall* instance, Result<cv::util::any>* ocvrs_return) {
		try {
			cv::util::any ret = instance->params();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::util::any>))
	}
	
	// setArgs(std::vector<GArg> &&) /usr/include/opencv2/gapi/gcall.hpp:66
	void cv_GCall_setArgs_vector_GArg_R(cv::GCall* instance, std::vector<cv::GArg>* args, Result_void* ocvrs_return) {
		try {
			instance->setArgs(*args);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_GCompiled_delete(cv::GCompiled* instance) {
		delete instance;
	}
	// GCompiled() /usr/include/opencv2/gapi/gcompiled.hpp:75
	void cv_GCompiled_GCompiled(Result<cv::GCompiled*>* ocvrs_return) {
		try {
			cv::GCompiled* ret = new cv::GCompiled();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GCompiled*>))
	}
	
	// operator bool() /usr/include/opencv2/gapi/gcompiled.hpp:165
	void cv_GCompiled_operator_bool_const(const cv::GCompiled* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator bool();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// canReshape() /usr/include/opencv2/gapi/gcompiled.hpp:196
	void cv_GCompiled_canReshape_const(const cv::GCompiled* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->canReshape();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// prepareForNewStream() /usr/include/opencv2/gapi/gcompiled.hpp:222
	void cv_GCompiled_prepareForNewStream(cv::GCompiled* instance, Result_void* ocvrs_return) {
		try {
			instance->prepareForNewStream();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// fmt /usr/include/opencv2/gapi/gframe.hpp:98
	void cv_GFrameDesc_getPropFmt_const(const cv::GFrameDesc* instance, cv::MediaFormat* ocvrs_return) {
			cv::MediaFormat ret = instance->fmt;
			*ocvrs_return = ret;
	}
	
	// fmt /usr/include/opencv2/gapi/gframe.hpp:98
	void cv_GFrameDesc_setPropFmt_MediaFormat(cv::GFrameDesc* instance, cv::MediaFormat val) {
			instance->fmt = val;
	}
	
	// size /usr/include/opencv2/gapi/gframe.hpp:99
	void cv_GFrameDesc_getPropSize_const(const cv::GFrameDesc* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->size;
			*ocvrs_return = ret;
	}
	
	// size /usr/include/opencv2/gapi/gframe.hpp:99
	void cv_GFrameDesc_setPropSize_Size(cv::GFrameDesc* instance, cv::Size* val) {
			instance->size = *val;
	}
	
	void cv_GFrameDesc_delete(cv::GFrameDesc* instance) {
		delete instance;
	}
	// operator==(const cv::GFrameDesc &) /usr/include/opencv2/gapi/gframe.hpp:101
	void cv_GFrameDesc_operatorEQ_const_const_GFrameDescR(const cv::GFrameDesc* instance, const cv::GFrameDesc* unnamed, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator==(*unnamed);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// name /usr/include/opencv2/gapi/gkernel.hpp:48
	void* cv_GKernel_getPropName_const(const cv::GKernel* instance) {
			std::string ret = instance->name;
			return ocvrs_create_string(ret.c_str());
	}
	
	// name /usr/include/opencv2/gapi/gkernel.hpp:48
	void cv_GKernel_setPropName_string(cv::GKernel* instance, char* val) {
			instance->name = std::string(val);
	}
	
	// tag /usr/include/opencv2/gapi/gkernel.hpp:49
	void* cv_GKernel_getPropTag_const(const cv::GKernel* instance) {
			std::string ret = instance->tag;
			return ocvrs_create_string(ret.c_str());
	}
	
	// tag /usr/include/opencv2/gapi/gkernel.hpp:49
	void cv_GKernel_setPropTag_string(cv::GKernel* instance, char* val) {
			instance->tag = std::string(val);
	}
	
	// outShapes /usr/include/opencv2/gapi/gkernel.hpp:51
	cv::GShapes* cv_GKernel_getPropOutShapes_const(const cv::GKernel* instance) {
			cv::GShapes ret = instance->outShapes;
			return new cv::GShapes(ret);
	}
	
	// outShapes /usr/include/opencv2/gapi/gkernel.hpp:51
	void cv_GKernel_setPropOutShapes_GShapes(cv::GKernel* instance, cv::GShapes* val) {
			instance->outShapes = *val;
	}
	
	// inKinds /usr/include/opencv2/gapi/gkernel.hpp:52
	cv::GKinds* cv_GKernel_getPropInKinds_const(const cv::GKernel* instance) {
			cv::GKinds ret = instance->inKinds;
			return new cv::GKinds(ret);
	}
	
	// inKinds /usr/include/opencv2/gapi/gkernel.hpp:52
	void cv_GKernel_setPropInKinds_GKinds(cv::GKernel* instance, cv::GKinds* val) {
			instance->inKinds = *val;
	}
	
	void cv_GKernel_delete(cv::GKernel* instance) {
		delete instance;
	}
	// opaque /usr/include/opencv2/gapi/gkernel.hpp:61
	void cv_GKernelImpl_getPropOpaque_const(const cv::GKernelImpl* instance, cv::util::any* ocvrs_return) {
			cv::util::any ret = instance->opaque;
			*ocvrs_return = ret;
	}
	
	// opaque /usr/include/opencv2/gapi/gkernel.hpp:61
	void cv_GKernelImpl_setPropOpaque_any(cv::GKernelImpl* instance, cv::util::any val) {
			instance->opaque = val;
	}
	
	void cv_GKernelImpl_delete(cv::GKernelImpl* instance) {
		delete instance;
	}
	cv::GMat* cv_GMatP_to_GMat(cv::GMatP* instance) {
		return dynamic_cast<cv::GMat*>(instance);
	}
	
	void cv_GMatP_delete(cv::GMatP* instance) {
		delete instance;
	}
	void cv_GRunArg_delete(cv::GRunArg* instance) {
		delete instance;
	}
	// GRunArg() /usr/include/opencv2/gapi/garg.hpp:129
	void cv_GRunArg_GRunArg(Result<cv::GRunArg*>* ocvrs_return) {
		try {
			cv::GRunArg* ret = new cv::GRunArg();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GRunArg*>))
	}
	
	// GRunArg(const cv::GRunArg &) /usr/include/opencv2/gapi/garg.hpp:130
	void cv_GRunArg_GRunArg_const_GRunArgR(const cv::GRunArg* arg, Result<cv::GRunArg*>* ocvrs_return) {
		try {
			cv::GRunArg* ret = new cv::GRunArg(*arg);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GRunArg*>))
	}
	
	// GRunArg(cv::GRunArg &&) /usr/include/opencv2/gapi/garg.hpp:131
	void cv_GRunArg_GRunArg_GRunArgR(cv::GRunArg* arg, Result<cv::GRunArg*>* ocvrs_return) {
		try {
			cv::GRunArg* ret = new cv::GRunArg(*arg);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GRunArg*>))
	}
	
	// description /usr/include/opencv2/gapi/gtransform.hpp:30
	void* cv_GTransform_getPropDescription_const(const cv::GTransform* instance) {
			std::string ret = instance->description;
			return ocvrs_create_string(ret.c_str());
	}
	
	// description /usr/include/opencv2/gapi/gtransform.hpp:30
	void cv_GTransform_setPropDescription_string(cv::GTransform* instance, char* val) {
			instance->description = std::string(val);
	}
	
	void cv_GTransform_delete(cv::GTransform* instance) {
		delete instance;
	}
	void cv_MediaFrame_delete(cv::MediaFrame* instance) {
		delete instance;
	}
	// MediaFrame() /usr/include/opencv2/gapi/media.hpp:70
	void cv_MediaFrame_MediaFrame(Result<cv::MediaFrame*>* ocvrs_return) {
		try {
			cv::MediaFrame* ret = new cv::MediaFrame();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MediaFrame*>))
	}
	
	// access(cv::MediaFrame::Access) /usr/include/opencv2/gapi/media.hpp:103
	void cv_MediaFrame_access_const_Access(const cv::MediaFrame* instance, cv::MediaFrame::Access mode, Result<cv::MediaFrame::View*>* ocvrs_return) {
		try {
			cv::MediaFrame::View ret = instance->access(mode);
			Ok(new cv::MediaFrame::View(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MediaFrame::View*>))
	}
	
	// desc() /usr/include/opencv2/gapi/media.hpp:110
	void cv_MediaFrame_desc_const(const cv::MediaFrame* instance, Result<cv::GFrameDesc*>* ocvrs_return) {
		try {
			cv::GFrameDesc ret = instance->desc();
			Ok(new cv::GFrameDesc(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GFrameDesc*>))
	}
	
	// blobParams() /usr/include/opencv2/gapi/media.hpp:115
	void cv_MediaFrame_blobParams_const(const cv::MediaFrame* instance, Result<cv::util::any>* ocvrs_return) {
		try {
			cv::util::any ret = instance->blobParams();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::util::any>))
	}
	
	// meta() /usr/include/opencv2/gapi/media.hpp:239
	void cv_MediaFrame_IAdapter_meta_const(const cv::MediaFrame::IAdapter* instance, Result<cv::GFrameDesc*>* ocvrs_return) {
		try {
			cv::GFrameDesc ret = instance->meta();
			Ok(new cv::GFrameDesc(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GFrameDesc*>))
	}
	
	// access(MediaFrame::Access) /usr/include/opencv2/gapi/media.hpp:240
	void cv_MediaFrame_IAdapter_access_Access(cv::MediaFrame::IAdapter* instance, cv::MediaFrame::Access unnamed, Result<cv::MediaFrame::View*>* ocvrs_return) {
		try {
			cv::MediaFrame::View ret = instance->access(unnamed);
			Ok(new cv::MediaFrame::View(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::MediaFrame::View*>))
	}
	
	// blobParams() /usr/include/opencv2/gapi/media.hpp:243
	void cv_MediaFrame_IAdapter_blobParams_const(const cv::MediaFrame::IAdapter* instance, Result<cv::util::any>* ocvrs_return) {
		try {
			cv::util::any ret = instance->blobParams();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::util::any>))
	}
	
	void cv_MediaFrame_View_delete(cv::MediaFrame::View* instance) {
		delete instance;
	}
	// View(cv::MediaFrame::View &&) /usr/include/opencv2/gapi/media.hpp:211
	cv::MediaFrame::View* cv_MediaFrame_View_View_ViewR(cv::MediaFrame::View* unnamed) {
			cv::MediaFrame::View* ret = new cv::MediaFrame::View(*unnamed);
			return ret;
	}
	
	void cv_RMat_delete(cv::RMat* instance) {
		delete instance;
	}
	// RMat() /usr/include/opencv2/gapi/rmat.hpp:126
	cv::RMat* cv_RMat_RMat() {
			cv::RMat* ret = new cv::RMat();
			return ret;
	}
	
	// desc() /usr/include/opencv2/gapi/rmat.hpp:128
	void cv_RMat_desc_const(const cv::RMat* instance, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc ret = instance->desc();
			Ok(new cv::GMatDesc(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMatDesc*>))
	}
	
	// access(cv::RMat::Access) /usr/include/opencv2/gapi/rmat.hpp:135
	void cv_RMat_access_const_Access(const cv::RMat* instance, cv::RMat::Access a, Result<cv::RMat::View*>* ocvrs_return) {
		try {
			cv::RMat::View ret = instance->access(a);
			Ok(new cv::RMat::View(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RMat::View*>))
	}
	
	// desc() /usr/include/opencv2/gapi/rmat.hpp:109
	void cv_RMat_IAdapter_desc_const(const cv::RMat::IAdapter instance, Result<cv::GMatDesc*>* ocvrs_return) {
		try {
			cv::GMatDesc ret = instance.desc();
			Ok(new cv::GMatDesc(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::GMatDesc*>))
	}
	
	// access(cv::RMat::Access) /usr/include/opencv2/gapi/rmat.hpp:113
	void cv_RMat_IAdapter_access_Access(cv::RMat::IAdapter instance, cv::RMat::Access unnamed, Result<cv::RMat::View*>* ocvrs_return) {
		try {
			cv::RMat::View ret = instance.access(unnamed);
			Ok(new cv::RMat::View(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RMat::View*>))
	}
	
	void cv_RMat_View_delete(cv::RMat::View* instance) {
		delete instance;
	}
	// View() /usr/include/opencv2/gapi/rmat.hpp:62
	cv::RMat::View* cv_RMat_View_View() {
			cv::RMat::View* ret = new cv::RMat::View();
			return ret;
	}
	
	// View(cv::RMat::View &&) /usr/include/opencv2/gapi/rmat.hpp:68
	cv::RMat::View* cv_RMat_View_View_ViewR(cv::RMat::View* unnamed) {
			cv::RMat::View* ret = new cv::RMat::View(*unnamed);
			return ret;
	}
	
	// size() /usr/include/opencv2/gapi/rmat.hpp:72
	void cv_RMat_View_size_const(const cv::RMat::View* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->size();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// dims() /usr/include/opencv2/gapi/rmat.hpp:73
	void cv_RMat_View_dims_const(const cv::RMat::View* instance, Result<std::vector<int>*>* ocvrs_return) {
		try {
			const std::vector<int> ret = instance->dims();
			Ok(new const std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	// cols() /usr/include/opencv2/gapi/rmat.hpp:74
	void cv_RMat_View_cols_const(const cv::RMat::View* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->cols();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// rows() /usr/include/opencv2/gapi/rmat.hpp:75
	void cv_RMat_View_rows_const(const cv::RMat::View* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->rows();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// type() /usr/include/opencv2/gapi/rmat.hpp:76
	void cv_RMat_View_type_const(const cv::RMat::View* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->type();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// depth() /usr/include/opencv2/gapi/rmat.hpp:77
	void cv_RMat_View_depth_const(const cv::RMat::View* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->depth();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// chan() /usr/include/opencv2/gapi/rmat.hpp:78
	void cv_RMat_View_chan_const(const cv::RMat::View* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->chan();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// elemSize() /usr/include/opencv2/gapi/rmat.hpp:79
	void cv_RMat_View_elemSize_const(const cv::RMat::View* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->elemSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// step(size_t) /usr/include/opencv2/gapi/rmat.hpp:93
	void cv_RMat_View_step_const_size_t(const cv::RMat::View* instance, size_t i, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->step(i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// steps() /usr/include/opencv2/gapi/rmat.hpp:94
	void cv_RMat_View_steps_const(const cv::RMat::View* instance, Result<cv::RMat::View::stepsT*>* ocvrs_return) {
		try {
			const cv::RMat::View::stepsT ret = instance->steps();
			Ok(new const cv::RMat::View::stepsT(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::RMat::View::stepsT*>))
	}
	
	void cv_GArrayU_delete(cv::detail::GArrayU* instance) {
		delete instance;
	}
	void cv_GOpaqueU_delete(cv::detail::GOpaqueU* instance) {
		delete instance;
	}
	void cv_GBackend_delete(cv::gapi::GBackend* instance) {
		delete instance;
	}
	// GBackend() /usr/include/opencv2/gapi/gkernel.hpp:382
	void cv_gapi_GBackend_GBackend(Result<cv::gapi::GBackend*>* ocvrs_return) {
		try {
			cv::gapi::GBackend* ret = new cv::gapi::GBackend();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::gapi::GBackend*>))
	}
	
	// operator==(const cv::gapi::GBackend &) /usr/include/opencv2/gapi/gkernel.hpp:389
	void cv_gapi_GBackend_operatorEQ_const_const_GBackendR(const cv::gapi::GBackend* instance, const cv::gapi::GBackend* rhs, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->operator==(*rhs);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// val /usr/include/opencv2/gapi/own/scalar.hpp:35
	double** cv_gapi_own_Scalar_getPropVal(cv::gapi::own::Scalar* instance) {
			double(*ret)[4] = &instance->val;
			return (double**)ret;
	}
	
	void cv_Own_Scalar_delete(cv::gapi::own::Scalar* instance) {
		delete instance;
	}
	// Scalar() /usr/include/opencv2/gapi/own/scalar.hpp:23
	cv::gapi::own::Scalar* cv_gapi_own_Scalar_Scalar() {
			cv::gapi::own::Scalar* ret = new cv::gapi::own::Scalar();
			return ret;
	}
	
	// Scalar(double) /usr/include/opencv2/gapi/own/scalar.hpp:24
	void cv_gapi_own_Scalar_Scalar_double(double v0, Result<cv::gapi::own::Scalar*>* ocvrs_return) {
		try {
			cv::gapi::own::Scalar* ret = new cv::gapi::own::Scalar(v0);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::gapi::own::Scalar*>))
	}
	
	// Scalar(double, double, double, double) /usr/include/opencv2/gapi/own/scalar.hpp:25
	void cv_gapi_own_Scalar_Scalar_double_double_double_double(double v0, double v1, double v2, double v3, Result<cv::gapi::own::Scalar*>* ocvrs_return) {
		try {
			cv::gapi::own::Scalar* ret = new cv::gapi::own::Scalar(v0, v1, v2, v3);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::gapi::own::Scalar*>))
	}
	
	// operator[](int) /usr/include/opencv2/gapi/own/scalar.hpp:30
	void cv_gapi_own_Scalar_operator___const_int(const cv::gapi::own::Scalar* instance, int i, Result<double>* ocvrs_return) {
		try {
			const double ret = instance->operator[](i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// operator[](int) /usr/include/opencv2/gapi/own/scalar.hpp:31
	void cv_gapi_own_Scalar_operator___int(cv::gapi::own::Scalar* instance, int i, Result<double>* ocvrs_return) {
		try {
			double ret = instance->operator[](i);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// all(double) /usr/include/opencv2/gapi/own/scalar.hpp:33
	void cv_gapi_own_Scalar_all_double(double v0, Result<cv::gapi::own::Scalar*>* ocvrs_return) {
		try {
			cv::gapi::own::Scalar ret = cv::gapi::own::Scalar::all(v0);
			Ok(new cv::gapi::own::Scalar(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::gapi::own::Scalar*>))
	}
	
	// pkg /usr/include/opencv2/gapi/gkernel.hpp:731
	cv::gapi::GKernelPackage* cv_gapi_use_only_getPropPkg_const(const cv::gapi::use_only* instance) {
			cv::gapi::GKernelPackage ret = instance->pkg;
			return new cv::gapi::GKernelPackage(ret);
	}
	
	// pkg /usr/include/opencv2/gapi/gkernel.hpp:731
	void cv_gapi_use_only_setPropPkg_GKernelPackage(cv::gapi::use_only* instance, cv::gapi::GKernelPackage* val) {
			instance->pkg = *val;
	}
	
	void cv_use_only_delete(cv::gapi::use_only* instance) {
		delete instance;
	}
	cv::GRunArg* cv_Data_to_GRunArg(cv::gapi::wip::Data* instance) {
		return dynamic_cast<cv::GRunArg*>(instance);
	}
	
	void cv_Data_delete(cv::gapi::wip::Data* instance) {
		delete instance;
	}
}
