#include "ocvrs_common.hpp"
#include <opencv2/ximgproc.hpp>
#include "ximgproc_types.hpp"

extern "C" {
	// BrightEdges(cv::Mat &, cv::Mat &, int, int, int) /usr/include/opencv2/ximgproc/brightedges.hpp:48
	void cv_ximgproc_BrightEdges_MatR_MatR_int_int_int(cv::Mat* _original, cv::Mat* _edgeview, int contrast, int shortrange, int longrange, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::BrightEdges(*_original, *_edgeview, contrast, shortrange, longrange);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// FastHoughTransform(cv::InputArray, cv::OutputArray, int, int, int, int) /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:133
	void cv_ximgproc_FastHoughTransform_const__InputArrayR_const__OutputArrayR_int_int_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int dstMatDepth, int angleRange, int op, int makeSkew, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::FastHoughTransform(*src, *dst, dstMatDepth, angleRange, op, makeSkew);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// GradientDericheX(cv::InputArray, cv::OutputArray, double, double) /usr/include/opencv2/ximgproc/deriche_filter.hpp:72
	void cv_ximgproc_GradientDericheX_const__InputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* op, const cv::_OutputArray* dst, double alpha, double omega, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::GradientDericheX(*op, *dst, alpha, omega);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// GradientDericheY(cv::InputArray, cv::OutputArray, double, double) /usr/include/opencv2/ximgproc/deriche_filter.hpp:60
	void cv_ximgproc_GradientDericheY_const__InputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* op, const cv::_OutputArray* dst, double alpha, double omega, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::GradientDericheY(*op, *dst, alpha, omega);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// GradientPaillouX(cv::InputArray, cv::OutputArray, double, double) /usr/include/opencv2/ximgproc/paillou_filter.hpp:62
	void cv_ximgproc_GradientPaillouX_const__InputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* op, const cv::_OutputArray* _dst, double alpha, double omega, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::GradientPaillouX(*op, *_dst, alpha, omega);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// GradientPaillouY(cv::InputArray, cv::OutputArray, double, double) /usr/include/opencv2/ximgproc/paillou_filter.hpp:61
	void cv_ximgproc_GradientPaillouY_const__InputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* op, const cv::_OutputArray* _dst, double alpha, double omega, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::GradientPaillouY(*op, *_dst, alpha, omega);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// HoughPoint2Line(const cv::Point &, cv::InputArray, int, int, int) /usr/include/opencv2/ximgproc/fast_hough_transform.hpp:155
	void cv_ximgproc_HoughPoint2Line_const_PointR_const__InputArrayR_int_int_int(const cv::Point* houghPoint, const cv::_InputArray* srcImgInfo, int angleRange, int makeSkew, int rules, Result<cv::Vec4i>* ocvrs_return) {
		try {
			cv::Vec4i ret = cv::ximgproc::HoughPoint2Line(*houghPoint, *srcImgInfo, angleRange, makeSkew, rules);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Vec4i>))
	}
	
	// PeiLinNormalization(cv::InputArray) /usr/include/opencv2/ximgproc/peilin.hpp:26
	void cv_ximgproc_PeiLinNormalization_const__InputArrayR(const cv::_InputArray* I, Result<cv::Matx23d>* ocvrs_return) {
		try {
			cv::Matx23d ret = cv::ximgproc::PeiLinNormalization(*I);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Matx23d>))
	}
	
	// PeiLinNormalization(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/peilin.hpp:28
	void cv_ximgproc_PeiLinNormalization_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* I, const cv::_OutputArray* T, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::PeiLinNormalization(*I, *T);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// RadonTransform(cv::InputArray, cv::OutputArray, double, double, double, bool, bool) /usr/include/opencv2/ximgproc/radon_transform.hpp:31
	void cv_ximgproc_RadonTransform_const__InputArrayR_const__OutputArrayR_double_double_double_bool_bool(const cv::_InputArray* src, const cv::_OutputArray* dst, double theta, double start_angle, double end_angle, bool crop, bool norm, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::RadonTransform(*src, *dst, theta, start_angle, end_angle, crop, norm);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// amFilter(cv::InputArray, cv::InputArray, cv::OutputArray, double, double, bool) /usr/include/opencv2/ximgproc/edge_filter.hpp:284
	void cv_ximgproc_amFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_bool(const cv::_InputArray* joint, const cv::_InputArray* src, const cv::_OutputArray* dst, double sigma_s, double sigma_r, bool adjust_outliers, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::amFilter(*joint, *src, *dst, sigma_s, sigma_r, adjust_outliers);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// anisotropicDiffusion(cv::InputArray, cv::OutputArray, float, float, int) /usr/include/opencv2/ximgproc.hpp:211
	void cv_ximgproc_anisotropicDiffusion_const__InputArrayR_const__OutputArrayR_float_float_int(const cv::_InputArray* src, const cv::_OutputArray* dst, float alpha, float K, int niters, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::anisotropicDiffusion(*src, *dst, alpha, K, niters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// bilateralTextureFilter(cv::InputArray, cv::OutputArray, int, int, double, double) /usr/include/opencv2/ximgproc/edge_filter.hpp:339
	void cv_ximgproc_bilateralTextureFilter_const__InputArrayR_const__OutputArrayR_int_int_double_double(const cv::_InputArray* src, const cv::_OutputArray* dst, int fr, int numIter, double sigmaAlpha, double sigmaAvg, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::bilateralTextureFilter(*src, *dst, fr, numIter, sigmaAlpha, sigmaAvg);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// colorMatchTemplate(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/color_match.hpp:62
	void cv_ximgproc_colorMatchTemplate_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* img, const cv::_InputArray* templ, const cv::_OutputArray* result, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::colorMatchTemplate(*img, *templ, *result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// computeBadPixelPercent(cv::InputArray, cv::InputArray, cv::Rect, int) /usr/include/opencv2/ximgproc/disparity_filter.hpp:193
	void cv_ximgproc_computeBadPixelPercent_const__InputArrayR_const__InputArrayR_Rect_int(const cv::_InputArray* GT, const cv::_InputArray* src, cv::Rect* ROI, int thresh, Result<double>* ocvrs_return) {
		try {
			double ret = cv::ximgproc::computeBadPixelPercent(*GT, *src, *ROI, thresh);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// computeMSE(cv::InputArray, cv::InputArray, cv::Rect) /usr/include/opencv2/ximgproc/disparity_filter.hpp:177
	void cv_ximgproc_computeMSE_const__InputArrayR_const__InputArrayR_Rect(const cv::_InputArray* GT, const cv::_InputArray* src, cv::Rect* ROI, Result<double>* ocvrs_return) {
		try {
			double ret = cv::ximgproc::computeMSE(*GT, *src, *ROI);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// contourSampling(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:106
	void cv_ximgproc_contourSampling_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* out, int nbElt, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::contourSampling(*src, *out, nbElt);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// covarianceEstimation(cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/ximgproc/estimated_covariance.hpp:77
	void cv_ximgproc_covarianceEstimation_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int windowRows, int windowCols, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::covarianceEstimation(*src, *dst, windowRows, windowCols);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// createAMFilter(double, double, bool) /usr/include/opencv2/ximgproc/edge_filter.hpp:262
	void cv_ximgproc_createAMFilter_double_double_bool(double sigma_s, double sigma_r, bool adjust_outliers, Result<cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter> ret = cv::ximgproc::createAMFilter(sigma_s, sigma_r, adjust_outliers);
			Ok(new cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>*>))
	}
	
	// createContourFitting(int, int) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:114
	void cv_ximgproc_createContourFitting_int_int(int ctr, int fd, Result<cv::Ptr<cv::ximgproc::ContourFitting>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::ContourFitting> ret = cv::ximgproc::createContourFitting(ctr, fd);
			Ok(new cv::Ptr<cv::ximgproc::ContourFitting>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::ContourFitting>*>))
	}
	
	// createDTFilter(cv::InputArray, double, double, int, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:102
	void cv_ximgproc_createDTFilter_const__InputArrayR_double_double_int_int(const cv::_InputArray* guide, double sigmaSpatial, double sigmaColor, int mode, int numIters, Result<cv::Ptr<cv::ximgproc::DTFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::DTFilter> ret = cv::ximgproc::createDTFilter(*guide, sigmaSpatial, sigmaColor, mode, numIters);
			Ok(new cv::Ptr<cv::ximgproc::DTFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::DTFilter>*>))
	}
	
	// createDisparityWLSFilterGeneric(bool) /usr/include/opencv2/ximgproc/disparity_filter.hpp:149
	void cv_ximgproc_createDisparityWLSFilterGeneric_bool(bool use_confidence, Result<cv::Ptr<cv::ximgproc::DisparityWLSFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::DisparityWLSFilter> ret = cv::ximgproc::createDisparityWLSFilterGeneric(use_confidence);
			Ok(new cv::Ptr<cv::ximgproc::DisparityWLSFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::DisparityWLSFilter>*>))
	}
	
	// createDisparityWLSFilter(Ptr<cv::StereoMatcher>) /usr/include/opencv2/ximgproc/disparity_filter.hpp:131
	void cv_ximgproc_createDisparityWLSFilter_Ptr_StereoMatcher_(cv::Ptr<cv::StereoMatcher>* matcher_left, Result<cv::Ptr<cv::ximgproc::DisparityWLSFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::DisparityWLSFilter> ret = cv::ximgproc::createDisparityWLSFilter(*matcher_left);
			Ok(new cv::Ptr<cv::ximgproc::DisparityWLSFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::DisparityWLSFilter>*>))
	}
	
	// createEdgeAwareInterpolator() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:138
	void cv_ximgproc_createEdgeAwareInterpolator(Result<cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::EdgeAwareInterpolator> ret = cv::ximgproc::createEdgeAwareInterpolator();
			Ok(new cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::EdgeAwareInterpolator>*>))
	}
	
	// createEdgeBoxes(float, float, float, float, int, float, float, float, float, float, float, float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:183
	void cv_ximgproc_createEdgeBoxes_float_float_float_float_int_float_float_float_float_float_float_float(float alpha, float beta, float eta, float minScore, int maxBoxes, float edgeMinMag, float edgeMergeThr, float clusterMinMag, float maxAspectRatio, float minBoxArea, float gamma, float kappa, Result<cv::Ptr<cv::ximgproc::EdgeBoxes>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::EdgeBoxes> ret = cv::ximgproc::createEdgeBoxes(alpha, beta, eta, minScore, maxBoxes, edgeMinMag, edgeMergeThr, clusterMinMag, maxAspectRatio, minBoxArea, gamma, kappa);
			Ok(new cv::Ptr<cv::ximgproc::EdgeBoxes>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::EdgeBoxes>*>))
	}
	
	// createEdgeDrawing() /usr/include/opencv2/ximgproc/edge_drawing.hpp:126
	void cv_ximgproc_createEdgeDrawing(Result<cv::Ptr<cv::ximgproc::EdgeDrawing>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::EdgeDrawing> ret = cv::ximgproc::createEdgeDrawing();
			Ok(new cv::Ptr<cv::ximgproc::EdgeDrawing>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::EdgeDrawing>*>))
	}
	
	// createFastBilateralSolverFilter(cv::InputArray, double, double, double, double, int, double) /usr/include/opencv2/ximgproc/edge_filter.hpp:417
	void cv_ximgproc_createFastBilateralSolverFilter_const__InputArrayR_double_double_double_double_int_double(const cv::_InputArray* guide, double sigma_spatial, double sigma_luma, double sigma_chroma, double lambda, int num_iter, double max_tol, Result<cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::FastBilateralSolverFilter> ret = cv::ximgproc::createFastBilateralSolverFilter(*guide, sigma_spatial, sigma_luma, sigma_chroma, lambda, num_iter, max_tol);
			Ok(new cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::FastBilateralSolverFilter>*>))
	}
	
	// createFastGlobalSmootherFilter(cv::InputArray, double, double, double, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:489
	void cv_ximgproc_createFastGlobalSmootherFilter_const__InputArrayR_double_double_double_int(const cv::_InputArray* guide, double lambda, double sigma_color, double lambda_attenuation, int num_iter, Result<cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter> ret = cv::ximgproc::createFastGlobalSmootherFilter(*guide, lambda, sigma_color, lambda_attenuation, num_iter);
			Ok(new cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::FastGlobalSmootherFilter>*>))
	}
	
	// createFastLineDetector(int, float, double, double, int, bool) /usr/include/opencv2/ximgproc/fast_line_detector.hpp:71
	void cv_ximgproc_createFastLineDetector_int_float_double_double_int_bool(int length_threshold, float distance_threshold, double canny_th1, double canny_th2, int canny_aperture_size, bool do_merge, Result<cv::Ptr<cv::ximgproc::FastLineDetector>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::FastLineDetector> ret = cv::ximgproc::createFastLineDetector(length_threshold, distance_threshold, canny_th1, canny_th2, canny_aperture_size, do_merge);
			Ok(new cv::Ptr<cv::ximgproc::FastLineDetector>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::FastLineDetector>*>))
	}
	
	// createGuidedFilter(cv::InputArray, int, double) /usr/include/opencv2/ximgproc/edge_filter.hpp:158
	void cv_ximgproc_createGuidedFilter_const__InputArrayR_int_double(const cv::_InputArray* guide, int radius, double eps, Result<cv::Ptr<cv::ximgproc::GuidedFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::GuidedFilter> ret = cv::ximgproc::createGuidedFilter(*guide, radius, eps);
			Ok(new cv::Ptr<cv::ximgproc::GuidedFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::GuidedFilter>*>))
	}
	
	// createQuaternionImage(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/color_match.hpp:22
	void cv_ximgproc_createQuaternionImage_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* img, const cv::_OutputArray* qimg, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::createQuaternionImage(*img, *qimg);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// createRFFeatureGetter() /usr/include/opencv2/ximgproc/structured_edge_detection.hpp:91
	void cv_ximgproc_createRFFeatureGetter(Result<cv::Ptr<cv::ximgproc::RFFeatureGetter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::RFFeatureGetter> ret = cv::ximgproc::createRFFeatureGetter();
			Ok(new cv::Ptr<cv::ximgproc::RFFeatureGetter>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::RFFeatureGetter>*>))
	}
	
	// createRICInterpolator() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:265
	void cv_ximgproc_createRICInterpolator(Result<cv::Ptr<cv::ximgproc::RICInterpolator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::RICInterpolator> ret = cv::ximgproc::createRICInterpolator();
			Ok(new cv::Ptr<cv::ximgproc::RICInterpolator>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::RICInterpolator>*>))
	}
	
	// createRightMatcher(Ptr<cv::StereoMatcher>) /usr/include/opencv2/ximgproc/disparity_filter.hpp:139
	void cv_ximgproc_createRightMatcher_Ptr_StereoMatcher_(cv::Ptr<cv::StereoMatcher>* matcher_left, Result<cv::Ptr<cv::StereoMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::StereoMatcher> ret = cv::ximgproc::createRightMatcher(*matcher_left);
			Ok(new cv::Ptr<cv::StereoMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::StereoMatcher>*>))
	}
	
	// createScanSegment(int, int, int, int, bool) /usr/include/opencv2/ximgproc/scansegment.hpp:80
	void cv_ximgproc_createScanSegment_int_int_int_int_bool(int image_width, int image_height, int num_superpixels, int slices, bool merge_small, Result<cv::Ptr<cv::ximgproc::ScanSegment>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::ScanSegment> ret = cv::ximgproc::createScanSegment(image_width, image_height, num_superpixels, slices, merge_small);
			Ok(new cv::Ptr<cv::ximgproc::ScanSegment>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::ScanSegment>*>))
	}
	
	// createStructuredEdgeDetection(const cv::String &, Ptr<const cv::ximgproc::RFFeatureGetter>) /usr/include/opencv2/ximgproc/structured_edge_detection.hpp:140
	void cv_ximgproc_createStructuredEdgeDetection_const_StringR_Ptr_const_RFFeatureGetter_(const char* model, cv::Ptr<const cv::ximgproc::RFFeatureGetter>* howToGetFeatures, Result<cv::Ptr<cv::ximgproc::StructuredEdgeDetection>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::StructuredEdgeDetection> ret = cv::ximgproc::createStructuredEdgeDetection(std::string(model), *howToGetFeatures);
			Ok(new cv::Ptr<cv::ximgproc::StructuredEdgeDetection>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::StructuredEdgeDetection>*>))
	}
	
	// createSuperpixelLSC(cv::InputArray, int, float) /usr/include/opencv2/ximgproc/lsc.hpp:150
	void cv_ximgproc_createSuperpixelLSC_const__InputArrayR_int_float(const cv::_InputArray* image, int region_size, float ratio, Result<cv::Ptr<cv::ximgproc::SuperpixelLSC>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::SuperpixelLSC> ret = cv::ximgproc::createSuperpixelLSC(*image, region_size, ratio);
			Ok(new cv::Ptr<cv::ximgproc::SuperpixelLSC>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::SuperpixelLSC>*>))
	}
	
	// createSuperpixelSEEDS(int, int, int, int, int, int, int, bool) /usr/include/opencv2/ximgproc/seeds.hpp:173
	void cv_ximgproc_createSuperpixelSEEDS_int_int_int_int_int_int_int_bool(int image_width, int image_height, int image_channels, int num_superpixels, int num_levels, int prior, int histogram_bins, bool double_step, Result<cv::Ptr<cv::ximgproc::SuperpixelSEEDS>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::SuperpixelSEEDS> ret = cv::ximgproc::createSuperpixelSEEDS(image_width, image_height, image_channels, num_superpixels, num_levels, prior, histogram_bins, double_step);
			Ok(new cv::Ptr<cv::ximgproc::SuperpixelSEEDS>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::SuperpixelSEEDS>*>))
	}
	
	// createSuperpixelSLIC(cv::InputArray, int, int, float) /usr/include/opencv2/ximgproc/slic.hpp:160
	void cv_ximgproc_createSuperpixelSLIC_const__InputArrayR_int_int_float(const cv::_InputArray* image, int algorithm, int region_size, float ruler, Result<cv::Ptr<cv::ximgproc::SuperpixelSLIC>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::SuperpixelSLIC> ret = cv::ximgproc::createSuperpixelSLIC(*image, algorithm, region_size, ruler);
			Ok(new cv::Ptr<cv::ximgproc::SuperpixelSLIC>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::SuperpixelSLIC>*>))
	}
	
	// dtFilter(cv::InputArray, cv::InputArray, cv::OutputArray, double, double, int, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:121
	void cv_ximgproc_dtFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_int_int(const cv::_InputArray* guide, const cv::_InputArray* src, const cv::_OutputArray* dst, double sigmaSpatial, double sigmaColor, int mode, int numIters, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::dtFilter(*guide, *src, *dst, sigmaSpatial, sigmaColor, mode, numIters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// edgePreservingFilter(cv::InputArray, cv::OutputArray, int, double) /usr/include/opencv2/ximgproc/edgepreserving_filter.hpp:27
	void cv_ximgproc_edgePreservingFilter_const__InputArrayR_const__OutputArrayR_int_double(const cv::_InputArray* src, const cv::_OutputArray* dst, int d, double threshold, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::edgePreservingFilter(*src, *dst, d, threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// fastBilateralSolverFilter(cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray, double, double, double, double, int, double) /usr/include/opencv2/ximgproc/edge_filter.hpp:448
	void cv_ximgproc_fastBilateralSolverFilter_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_double_double_int_double(const cv::_InputArray* guide, const cv::_InputArray* src, const cv::_InputArray* confidence, const cv::_OutputArray* dst, double sigma_spatial, double sigma_luma, double sigma_chroma, double lambda, int num_iter, double max_tol, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::fastBilateralSolverFilter(*guide, *src, *confidence, *dst, sigma_spatial, sigma_luma, sigma_chroma, lambda, num_iter, max_tol);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// fastGlobalSmootherFilter(cv::InputArray, cv::InputArray, cv::OutputArray, double, double, double, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:509
	void cv_ximgproc_fastGlobalSmootherFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_double_double_double_int(const cv::_InputArray* guide, const cv::_InputArray* src, const cv::_OutputArray* dst, double lambda, double sigma_color, double lambda_attenuation, int num_iter, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::fastGlobalSmootherFilter(*guide, *src, *dst, lambda, sigma_color, lambda_attenuation, num_iter);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// fourierDescriptor(cv::InputArray, cv::OutputArray, int, int) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:87
	void cv_ximgproc_fourierDescriptor_const__InputArrayR_const__OutputArrayR_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int nbElt, int nbFD, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::fourierDescriptor(*src, *dst, nbElt, nbFD);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDisparityVis(cv::InputArray, cv::OutputArray, double) /usr/include/opencv2/ximgproc/disparity_filter.hpp:204
	void cv_ximgproc_getDisparityVis_const__InputArrayR_const__OutputArrayR_double(const cv::_InputArray* src, const cv::_OutputArray* dst, double scale, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::getDisparityVis(*src, *dst, scale);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// guidedFilter(cv::InputArray, cv::InputArray, cv::OutputArray, int, double, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:180
	void cv_ximgproc_guidedFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_int(const cv::_InputArray* guide, const cv::_InputArray* src, const cv::_OutputArray* dst, int radius, double eps, int dDepth, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::guidedFilter(*guide, *src, *dst, radius, eps, dDepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// jointBilateralFilter(cv::InputArray, cv::InputArray, cv::OutputArray, int, double, double, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:317
	void cv_ximgproc_jointBilateralFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_double_int(const cv::_InputArray* joint, const cv::_InputArray* src, const cv::_OutputArray* dst, int d, double sigmaColor, double sigmaSpace, int borderType, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::jointBilateralFilter(*joint, *src, *dst, d, sigmaColor, sigmaSpace, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// l0Smooth(cv::InputArray, cv::OutputArray, double, double) /usr/include/opencv2/ximgproc/edge_filter.hpp:523
	void cv_ximgproc_l0Smooth_const__InputArrayR_const__OutputArrayR_double_double(const cv::_InputArray* src, const cv::_OutputArray* dst, double lambda, double kappa, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::l0Smooth(*src, *dst, lambda, kappa);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// niBlackThreshold(cv::InputArray, cv::OutputArray, double, int, int, double, int, double) /usr/include/opencv2/ximgproc.hpp:176
	void cv_ximgproc_niBlackThreshold_const__InputArrayR_const__OutputArrayR_double_int_int_double_int_double(const cv::_InputArray* _src, const cv::_OutputArray* _dst, double maxValue, int type, int blockSize, double k, int binarizationMethod, double r, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::niBlackThreshold(*_src, *_dst, maxValue, type, blockSize, k, binarizationMethod, r);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// qconj(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/color_match.hpp:30
	void cv_ximgproc_qconj_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* qimg, const cv::_OutputArray* qcimg, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::qconj(*qimg, *qcimg);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// qdft(cv::InputArray, cv::OutputArray, int, bool) /usr/include/opencv2/ximgproc/color_match.hpp:54
	void cv_ximgproc_qdft_const__InputArrayR_const__OutputArrayR_int_bool(const cv::_InputArray* img, const cv::_OutputArray* qimg, int flags, bool sideLeft, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::qdft(*img, *qimg, flags, sideLeft);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// qmultiply(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/color_match.hpp:45
	void cv_ximgproc_qmultiply_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* src1, const cv::_InputArray* src2, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::qmultiply(*src1, *src2, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// qunitary(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/color_match.hpp:37
	void cv_ximgproc_qunitary_const__InputArrayR_const__OutputArrayR(const cv::_InputArray* qimg, const cv::_OutputArray* qnimg, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::qunitary(*qimg, *qnimg);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// readGT(cv::String, cv::OutputArray) /usr/include/opencv2/ximgproc/disparity_filter.hpp:164
	void cv_ximgproc_readGT_String_const__OutputArrayR(char* src_path, const cv::_OutputArray* dst, Result<int>* ocvrs_return) {
		try {
			int ret = cv::ximgproc::readGT(std::string(src_path), *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// createRLEImage(const std::vector<cv::Point3i> &, cv::OutputArray, cv::Size) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:97
	void cv_ximgproc_rl_createRLEImage_const_vector_Point3i_R_const__OutputArrayR_Size(const std::vector<cv::Point3i>* runs, const cv::_OutputArray* res, cv::Size* size, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::rl::createRLEImage(*runs, *res, *size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// dilate(cv::InputArray, cv::OutputArray, cv::InputArray, cv::Point) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:42
	void cv_ximgproc_rl_dilate_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Point(const cv::_InputArray* rlSrc, const cv::_OutputArray* rlDest, const cv::_InputArray* rlKernel, cv::Point* anchor, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::rl::dilate(*rlSrc, *rlDest, *rlKernel, *anchor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// erode(cv::InputArray, cv::OutputArray, cv::InputArray, bool, cv::Point) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:57
	void cv_ximgproc_rl_erode_const__InputArrayR_const__OutputArrayR_const__InputArrayR_bool_Point(const cv::_InputArray* rlSrc, const cv::_OutputArray* rlDest, const cv::_InputArray* rlKernel, bool bBoundaryOn, cv::Point* anchor, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::rl::erode(*rlSrc, *rlDest, *rlKernel, bBoundaryOn, *anchor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getStructuringElement(int, cv::Size) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:68
	void cv_ximgproc_rl_getStructuringElement_int_Size(int shape, cv::Size* ksize, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = cv::ximgproc::rl::getStructuringElement(shape, *ksize);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// isRLMorphologyPossible(cv::InputArray) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:87
	void cv_ximgproc_rl_isRLMorphologyPossible_const__InputArrayR(const cv::_InputArray* rlStructuringElement, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::ximgproc::rl::isRLMorphologyPossible(*rlStructuringElement);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// morphologyEx(cv::InputArray, cv::OutputArray, int, cv::InputArray, bool, cv::Point) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:113
	void cv_ximgproc_rl_morphologyEx_const__InputArrayR_const__OutputArrayR_int_const__InputArrayR_bool_Point(const cv::_InputArray* rlSrc, const cv::_OutputArray* rlDest, int op, const cv::_InputArray* rlKernel, bool bBoundaryOnForErosion, cv::Point* anchor, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::rl::morphologyEx(*rlSrc, *rlDest, op, *rlKernel, bBoundaryOnForErosion, *anchor);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// paint(cv::InputOutputArray, cv::InputArray, const cv::Scalar &) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:79
	void cv_ximgproc_rl_paint_const__InputOutputArrayR_const__InputArrayR_const_ScalarR(const cv::_InputOutputArray* image, const cv::_InputArray* rlSrc, const cv::Scalar* value, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::rl::paint(*image, *rlSrc, *value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// threshold(cv::InputArray, cv::OutputArray, double, int) /usr/include/opencv2/ximgproc/run_length_morphology.hpp:28
	void cv_ximgproc_rl_threshold_const__InputArrayR_const__OutputArrayR_double_int(const cv::_InputArray* src, const cv::_OutputArray* rlDest, double thresh, int type, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::rl::threshold(*src, *rlDest, thresh, type);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// rollingGuidanceFilter(cv::InputArray, cv::OutputArray, int, double, double, int, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:373
	void cv_ximgproc_rollingGuidanceFilter_const__InputArrayR_const__OutputArrayR_int_double_double_int_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int d, double sigmaColor, double sigmaSpace, int numOfIter, int borderType, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::rollingGuidanceFilter(*src, *dst, d, sigmaColor, sigmaSpace, numOfIter, borderType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// createGraphSegmentation(double, float, int) /usr/include/opencv2/ximgproc/segmentation.hpp:69
	void cv_ximgproc_segmentation_createGraphSegmentation_double_float_int(double sigma, float k, int min_size, Result<cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation> ret = cv::ximgproc::segmentation::createGraphSegmentation(sigma, k, min_size);
			Ok(new cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>*>))
	}
	
	// createSelectiveSearchSegmentation() /usr/include/opencv2/ximgproc/segmentation.hpp:244
	void cv_ximgproc_segmentation_createSelectiveSearchSegmentation(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation> ret = cv::ximgproc::segmentation::createSelectiveSearchSegmentation();
			Ok(new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentation>*>))
	}
	
	// createSelectiveSearchSegmentationStrategyColor() /usr/include/opencv2/ximgproc/segmentation.hpp:104
	void cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyColor(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor> ret = cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyColor();
			Ok(new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyColor>*>))
	}
	
	// createSelectiveSearchSegmentationStrategyFill() /usr/include/opencv2/ximgproc/segmentation.hpp:131
	void cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyFill(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill> ret = cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyFill();
			Ok(new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyFill>*>))
	}
	
	// createSelectiveSearchSegmentationStrategyMultiple() /usr/include/opencv2/ximgproc/segmentation.hpp:149
	void cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple> ret = cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyMultiple();
			Ok(new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>*>))
	}
	
	// createSelectiveSearchSegmentationStrategyMultiple(Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>) /usr/include/opencv2/ximgproc/segmentation.hpp:154
	void cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_Ptr_SelectiveSearchSegmentationStrategy_(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s1, Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple> ret = cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyMultiple(*s1);
			Ok(new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>*>))
	}
	
	// createSelectiveSearchSegmentationStrategyMultiple(Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>) /usr/include/opencv2/ximgproc/segmentation.hpp:160
	void cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy_(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s1, cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s2, Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple> ret = cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyMultiple(*s1, *s2);
			Ok(new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>*>))
	}
	
	// createSelectiveSearchSegmentationStrategyMultiple(Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>) /usr/include/opencv2/ximgproc/segmentation.hpp:168
	void cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy_(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s1, cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s2, cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s3, Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple> ret = cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyMultiple(*s1, *s2, *s3);
			Ok(new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>*>))
	}
	
	// createSelectiveSearchSegmentationStrategyMultiple(Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>) /usr/include/opencv2/ximgproc/segmentation.hpp:176
	void cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyMultiple_Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy__Ptr_SelectiveSearchSegmentationStrategy_(cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s1, cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s2, cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s3, cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s4, Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple> ret = cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyMultiple(*s1, *s2, *s3, *s4);
			Ok(new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple>*>))
	}
	
	// createSelectiveSearchSegmentationStrategySize() /usr/include/opencv2/ximgproc/segmentation.hpp:113
	void cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategySize(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize> ret = cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategySize();
			Ok(new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategySize>*>))
	}
	
	// createSelectiveSearchSegmentationStrategyTexture() /usr/include/opencv2/ximgproc/segmentation.hpp:122
	void cv_ximgproc_segmentation_createSelectiveSearchSegmentationStrategyTexture(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture> ret = cv::ximgproc::segmentation::createSelectiveSearchSegmentationStrategyTexture();
			Ok(new cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyTexture>*>))
	}
	
	// thinning(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ximgproc.hpp:189
	void cv_ximgproc_thinning_const__InputArrayR_const__OutputArrayR_int(const cv::_InputArray* src, const cv::_OutputArray* dst, int thinningType, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::thinning(*src, *dst, thinningType);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// transformFD(cv::InputArray, cv::InputArray, cv::OutputArray, bool) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:97
	void cv_ximgproc_transformFD_const__InputArrayR_const__InputArrayR_const__OutputArrayR_bool(const cv::_InputArray* src, const cv::_InputArray* t, const cv::_OutputArray* dst, bool fdContour, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::transformFD(*src, *t, *dst, fdContour);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// weightedMedianFilter(cv::InputArray, cv::InputArray, cv::OutputArray, int, double, int, cv::InputArray) /usr/include/opencv2/ximgproc/weighted_median_filter.hpp:90
	void cv_ximgproc_weightedMedianFilter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_double_int_const__InputArrayR(const cv::_InputArray* joint, const cv::_InputArray* src, const cv::_OutputArray* dst, int r, double sigma, int weightType, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::ximgproc::weightedMedianFilter(*joint, *src, *dst, r, sigma, weightType, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// filter(cv::InputArray, cv::OutputArray, cv::InputArray) /usr/include/opencv2/ximgproc/edge_filter.hpp:214
	void cv_ximgproc_AdaptiveManifoldFilter_filter_const__InputArrayR_const__OutputArrayR_const__InputArrayR(cv::ximgproc::AdaptiveManifoldFilter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, const cv::_InputArray* joint, Result_void* ocvrs_return) {
		try {
			instance->filter(*src, *dst, *joint);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// collectGarbage() /usr/include/opencv2/ximgproc/edge_filter.hpp:216
	void cv_ximgproc_AdaptiveManifoldFilter_collectGarbage(cv::ximgproc::AdaptiveManifoldFilter* instance, Result_void* ocvrs_return) {
		try {
			instance->collectGarbage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create() /usr/include/opencv2/ximgproc/edge_filter.hpp:218
	void cv_ximgproc_AdaptiveManifoldFilter_create(Result<cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter> ret = cv::ximgproc::AdaptiveManifoldFilter::create();
			Ok(new cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::AdaptiveManifoldFilter>*>))
	}
	
	// getSigmaS() /usr/include/opencv2/ximgproc/edge_filter.hpp:221
	void cv_ximgproc_AdaptiveManifoldFilter_getSigmaS_const(const cv::ximgproc::AdaptiveManifoldFilter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSigmaS();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setSigmaS(double) /usr/include/opencv2/ximgproc/edge_filter.hpp:223
	void cv_ximgproc_AdaptiveManifoldFilter_setSigmaS_double(cv::ximgproc::AdaptiveManifoldFilter* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setSigmaS(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSigmaR() /usr/include/opencv2/ximgproc/edge_filter.hpp:225
	void cv_ximgproc_AdaptiveManifoldFilter_getSigmaR_const(const cv::ximgproc::AdaptiveManifoldFilter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSigmaR();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setSigmaR(double) /usr/include/opencv2/ximgproc/edge_filter.hpp:227
	void cv_ximgproc_AdaptiveManifoldFilter_setSigmaR_double(cv::ximgproc::AdaptiveManifoldFilter* instance, double val, Result_void* ocvrs_return) {
		try {
			instance->setSigmaR(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTreeHeight() /usr/include/opencv2/ximgproc/edge_filter.hpp:229
	void cv_ximgproc_AdaptiveManifoldFilter_getTreeHeight_const(const cv::ximgproc::AdaptiveManifoldFilter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTreeHeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setTreeHeight(int) /usr/include/opencv2/ximgproc/edge_filter.hpp:231
	void cv_ximgproc_AdaptiveManifoldFilter_setTreeHeight_int(cv::ximgproc::AdaptiveManifoldFilter* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setTreeHeight(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getPCAIterations() /usr/include/opencv2/ximgproc/edge_filter.hpp:233
	void cv_ximgproc_AdaptiveManifoldFilter_getPCAIterations_const(const cv::ximgproc::AdaptiveManifoldFilter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getPCAIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setPCAIterations(int) /usr/include/opencv2/ximgproc/edge_filter.hpp:235
	void cv_ximgproc_AdaptiveManifoldFilter_setPCAIterations_int(cv::ximgproc::AdaptiveManifoldFilter* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setPCAIterations(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getAdjustOutliers() /usr/include/opencv2/ximgproc/edge_filter.hpp:237
	void cv_ximgproc_AdaptiveManifoldFilter_getAdjustOutliers_const(const cv::ximgproc::AdaptiveManifoldFilter* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getAdjustOutliers();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setAdjustOutliers(bool) /usr/include/opencv2/ximgproc/edge_filter.hpp:239
	void cv_ximgproc_AdaptiveManifoldFilter_setAdjustOutliers_bool(cv::ximgproc::AdaptiveManifoldFilter* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setAdjustOutliers(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getUseRNG() /usr/include/opencv2/ximgproc/edge_filter.hpp:241
	void cv_ximgproc_AdaptiveManifoldFilter_getUseRNG_const(const cv::ximgproc::AdaptiveManifoldFilter* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseRNG();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setUseRNG(bool) /usr/include/opencv2/ximgproc/edge_filter.hpp:243
	void cv_ximgproc_AdaptiveManifoldFilter_setUseRNG_bool(cv::ximgproc::AdaptiveManifoldFilter* instance, bool val, Result_void* ocvrs_return) {
		try {
			instance->setUseRNG(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::Algorithm* cv_ContourFitting_to_Algorithm(cv::ximgproc::ContourFitting* instance) {
		return dynamic_cast<cv::Algorithm*>(instance);
	}
	
	void cv_ContourFitting_delete(cv::ximgproc::ContourFitting* instance) {
		delete instance;
	}
	// ContourFitting(int, int) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:38
	void cv_ximgproc_ContourFitting_ContourFitting_int_int(int ctr, int fd, Result<cv::ximgproc::ContourFitting*>* ocvrs_return) {
		try {
			cv::ximgproc::ContourFitting* ret = new cv::ximgproc::ContourFitting(ctr, fd);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ximgproc::ContourFitting*>))
	}
	
	// estimateTransformation(cv::InputArray, cv::InputArray, cv::OutputArray, double *, bool) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:47
	void cv_ximgproc_ContourFitting_estimateTransformation_const__InputArrayR_const__InputArrayR_const__OutputArrayR_doubleX_bool(cv::ximgproc::ContourFitting* instance, const cv::_InputArray* src, const cv::_InputArray* dst, const cv::_OutputArray* alphaPhiST, double* dist, bool fdContour, Result_void* ocvrs_return) {
		try {
			instance->estimateTransformation(*src, *dst, *alphaPhiST, dist, fdContour);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// estimateTransformation(cv::InputArray, cv::InputArray, cv::OutputArray, double &, bool) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:56
	void cv_ximgproc_ContourFitting_estimateTransformation_const__InputArrayR_const__InputArrayR_const__OutputArrayR_doubleR_bool(cv::ximgproc::ContourFitting* instance, const cv::_InputArray* src, const cv::_InputArray* dst, const cv::_OutputArray* alphaPhiST, double* dist, bool fdContour, Result_void* ocvrs_return) {
		try {
			instance->estimateTransformation(*src, *dst, *alphaPhiST, *dist, fdContour);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setCtrSize(int) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:61
	void cv_ximgproc_ContourFitting_setCtrSize_int(cv::ximgproc::ContourFitting* instance, int n, Result_void* ocvrs_return) {
		try {
			instance->setCtrSize(n);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setFDSize(int) /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:66
	void cv_ximgproc_ContourFitting_setFDSize_int(cv::ximgproc::ContourFitting* instance, int n, Result_void* ocvrs_return) {
		try {
			instance->setFDSize(n);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getCtrSize() /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:70
	void cv_ximgproc_ContourFitting_getCtrSize(cv::ximgproc::ContourFitting* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getCtrSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getFDSize() /usr/include/opencv2/ximgproc/fourier_descriptors.hpp:74
	void cv_ximgproc_ContourFitting_getFDSize(cv::ximgproc::ContourFitting* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFDSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// filter(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:79
	void cv_ximgproc_DTFilter_filter_const__InputArrayR_const__OutputArrayR_int(cv::ximgproc::DTFilter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, int dDepth, Result_void* ocvrs_return) {
		try {
			instance->filter(*src, *dst, dDepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// filter(cv::InputArray, cv::InputArray, cv::OutputArray, cv::InputArray, cv::Rect, cv::InputArray) /usr/include/opencv2/ximgproc/disparity_filter.hpp:75
	void cv_ximgproc_DisparityFilter_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__InputArrayR_Rect_const__InputArrayR(cv::ximgproc::DisparityFilter* instance, const cv::_InputArray* disparity_map_left, const cv::_InputArray* left_view, const cv::_OutputArray* filtered_disparity_map, const cv::_InputArray* disparity_map_right, cv::Rect* ROI, const cv::_InputArray* right_view, Result_void* ocvrs_return) {
		try {
			instance->filter(*disparity_map_left, *left_view, *filtered_disparity_map, *disparity_map_right, *ROI, *right_view);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getLambda() /usr/include/opencv2/ximgproc/disparity_filter.hpp:90
	void cv_ximgproc_DisparityWLSFilter_getLambda(cv::ximgproc::DisparityWLSFilter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setLambda(double) /usr/include/opencv2/ximgproc/disparity_filter.hpp:92
	void cv_ximgproc_DisparityWLSFilter_setLambda_double(cv::ximgproc::DisparityWLSFilter* instance, double _lambda, Result_void* ocvrs_return) {
		try {
			instance->setLambda(_lambda);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSigmaColor() /usr/include/opencv2/ximgproc/disparity_filter.hpp:97
	void cv_ximgproc_DisparityWLSFilter_getSigmaColor(cv::ximgproc::DisparityWLSFilter* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSigmaColor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setSigmaColor(double) /usr/include/opencv2/ximgproc/disparity_filter.hpp:99
	void cv_ximgproc_DisparityWLSFilter_setSigmaColor_double(cv::ximgproc::DisparityWLSFilter* instance, double _sigma_color, Result_void* ocvrs_return) {
		try {
			instance->setSigmaColor(_sigma_color);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getLRCthresh() /usr/include/opencv2/ximgproc/disparity_filter.hpp:106
	void cv_ximgproc_DisparityWLSFilter_getLRCthresh(cv::ximgproc::DisparityWLSFilter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getLRCthresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setLRCthresh(int) /usr/include/opencv2/ximgproc/disparity_filter.hpp:108
	void cv_ximgproc_DisparityWLSFilter_setLRCthresh_int(cv::ximgproc::DisparityWLSFilter* instance, int _LRC_thresh, Result_void* ocvrs_return) {
		try {
			instance->setLRCthresh(_LRC_thresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDepthDiscontinuityRadius() /usr/include/opencv2/ximgproc/disparity_filter.hpp:112
	void cv_ximgproc_DisparityWLSFilter_getDepthDiscontinuityRadius(cv::ximgproc::DisparityWLSFilter* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDepthDiscontinuityRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setDepthDiscontinuityRadius(int) /usr/include/opencv2/ximgproc/disparity_filter.hpp:114
	void cv_ximgproc_DisparityWLSFilter_setDepthDiscontinuityRadius_int(cv::ximgproc::DisparityWLSFilter* instance, int _disc_radius, Result_void* ocvrs_return) {
		try {
			instance->setDepthDiscontinuityRadius(_disc_radius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getConfidenceMap() /usr/include/opencv2/ximgproc/disparity_filter.hpp:119
	void cv_ximgproc_DisparityWLSFilter_getConfidenceMap(cv::ximgproc::DisparityWLSFilter* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->getConfidenceMap();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// getROI() /usr/include/opencv2/ximgproc/disparity_filter.hpp:122
	void cv_ximgproc_DisparityWLSFilter_getROI(cv::ximgproc::DisparityWLSFilter* instance, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->getROI();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// setCostMap(const cv::Mat &) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:88
	void cv_ximgproc_EdgeAwareInterpolator_setCostMap_const_MatR(cv::ximgproc::EdgeAwareInterpolator* instance, const cv::Mat* _costMap, Result_void* ocvrs_return) {
		try {
			instance->setCostMap(*_costMap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setK(int) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:96
	void cv_ximgproc_EdgeAwareInterpolator_setK_int(cv::ximgproc::EdgeAwareInterpolator* instance, int _k, Result_void* ocvrs_return) {
		try {
			instance->setK(_k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getK() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:98
	void cv_ximgproc_EdgeAwareInterpolator_getK(cv::ximgproc::EdgeAwareInterpolator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getK();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setSigma(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:104
	void cv_ximgproc_EdgeAwareInterpolator_setSigma_float(cv::ximgproc::EdgeAwareInterpolator* instance, float _sigma, Result_void* ocvrs_return) {
		try {
			instance->setSigma(_sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSigma() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:106
	void cv_ximgproc_EdgeAwareInterpolator_getSigma(cv::ximgproc::EdgeAwareInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setLambda(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:111
	void cv_ximgproc_EdgeAwareInterpolator_setLambda_float(cv::ximgproc::EdgeAwareInterpolator* instance, float _lambda, Result_void* ocvrs_return) {
		try {
			instance->setLambda(_lambda);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getLambda() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:113
	void cv_ximgproc_EdgeAwareInterpolator_getLambda(cv::ximgproc::EdgeAwareInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setUsePostProcessing(bool) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:118
	void cv_ximgproc_EdgeAwareInterpolator_setUsePostProcessing_bool(cv::ximgproc::EdgeAwareInterpolator* instance, bool _use_post_proc, Result_void* ocvrs_return) {
		try {
			instance->setUsePostProcessing(_use_post_proc);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getUsePostProcessing() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:120
	void cv_ximgproc_EdgeAwareInterpolator_getUsePostProcessing(cv::ximgproc::EdgeAwareInterpolator* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUsePostProcessing();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setFGSLambda(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:124
	void cv_ximgproc_EdgeAwareInterpolator_setFGSLambda_float(cv::ximgproc::EdgeAwareInterpolator* instance, float _lambda, Result_void* ocvrs_return) {
		try {
			instance->setFGSLambda(_lambda);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getFGSLambda() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:126
	void cv_ximgproc_EdgeAwareInterpolator_getFGSLambda(cv::ximgproc::EdgeAwareInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getFGSLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setFGSSigma(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:129
	void cv_ximgproc_EdgeAwareInterpolator_setFGSSigma_float(cv::ximgproc::EdgeAwareInterpolator* instance, float _sigma, Result_void* ocvrs_return) {
		try {
			instance->setFGSSigma(_sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getFGSSigma() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:131
	void cv_ximgproc_EdgeAwareInterpolator_getFGSSigma(cv::ximgproc::EdgeAwareInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getFGSSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// getBoundingBoxes(cv::InputArray, cv::InputArray, std::vector<Rect> &, cv::OutputArray) /usr/include/opencv2/ximgproc/edgeboxes.hpp:79
	void cv_ximgproc_EdgeBoxes_getBoundingBoxes_const__InputArrayR_const__InputArrayR_vector_Rect_R_const__OutputArrayR(cv::ximgproc::EdgeBoxes* instance, const cv::_InputArray* edge_map, const cv::_InputArray* orientation_map, std::vector<cv::Rect>* boxes, const cv::_OutputArray* scores, Result_void* ocvrs_return) {
		try {
			instance->getBoundingBoxes(*edge_map, *orientation_map, *boxes, *scores);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getAlpha() /usr/include/opencv2/ximgproc/edgeboxes.hpp:83
	void cv_ximgproc_EdgeBoxes_getAlpha_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getAlpha();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setAlpha(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:86
	void cv_ximgproc_EdgeBoxes_setAlpha_float(cv::ximgproc::EdgeBoxes* instance, float value, Result_void* ocvrs_return) {
		try {
			instance->setAlpha(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getBeta() /usr/include/opencv2/ximgproc/edgeboxes.hpp:90
	void cv_ximgproc_EdgeBoxes_getBeta_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getBeta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setBeta(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:93
	void cv_ximgproc_EdgeBoxes_setBeta_float(cv::ximgproc::EdgeBoxes* instance, float value, Result_void* ocvrs_return) {
		try {
			instance->setBeta(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getEta() /usr/include/opencv2/ximgproc/edgeboxes.hpp:97
	void cv_ximgproc_EdgeBoxes_getEta_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getEta();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setEta(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:100
	void cv_ximgproc_EdgeBoxes_setEta_float(cv::ximgproc::EdgeBoxes* instance, float value, Result_void* ocvrs_return) {
		try {
			instance->setEta(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMinScore() /usr/include/opencv2/ximgproc/edgeboxes.hpp:104
	void cv_ximgproc_EdgeBoxes_getMinScore_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMinScore();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setMinScore(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:107
	void cv_ximgproc_EdgeBoxes_setMinScore_float(cv::ximgproc::EdgeBoxes* instance, float value, Result_void* ocvrs_return) {
		try {
			instance->setMinScore(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxBoxes() /usr/include/opencv2/ximgproc/edgeboxes.hpp:111
	void cv_ximgproc_EdgeBoxes_getMaxBoxes_const(const cv::ximgproc::EdgeBoxes* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMaxBoxes();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMaxBoxes(int) /usr/include/opencv2/ximgproc/edgeboxes.hpp:114
	void cv_ximgproc_EdgeBoxes_setMaxBoxes_int(cv::ximgproc::EdgeBoxes* instance, int value, Result_void* ocvrs_return) {
		try {
			instance->setMaxBoxes(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getEdgeMinMag() /usr/include/opencv2/ximgproc/edgeboxes.hpp:118
	void cv_ximgproc_EdgeBoxes_getEdgeMinMag_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getEdgeMinMag();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setEdgeMinMag(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:121
	void cv_ximgproc_EdgeBoxes_setEdgeMinMag_float(cv::ximgproc::EdgeBoxes* instance, float value, Result_void* ocvrs_return) {
		try {
			instance->setEdgeMinMag(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getEdgeMergeThr() /usr/include/opencv2/ximgproc/edgeboxes.hpp:125
	void cv_ximgproc_EdgeBoxes_getEdgeMergeThr_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getEdgeMergeThr();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setEdgeMergeThr(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:128
	void cv_ximgproc_EdgeBoxes_setEdgeMergeThr_float(cv::ximgproc::EdgeBoxes* instance, float value, Result_void* ocvrs_return) {
		try {
			instance->setEdgeMergeThr(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getClusterMinMag() /usr/include/opencv2/ximgproc/edgeboxes.hpp:132
	void cv_ximgproc_EdgeBoxes_getClusterMinMag_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getClusterMinMag();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setClusterMinMag(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:135
	void cv_ximgproc_EdgeBoxes_setClusterMinMag_float(cv::ximgproc::EdgeBoxes* instance, float value, Result_void* ocvrs_return) {
		try {
			instance->setClusterMinMag(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxAspectRatio() /usr/include/opencv2/ximgproc/edgeboxes.hpp:139
	void cv_ximgproc_EdgeBoxes_getMaxAspectRatio_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMaxAspectRatio();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setMaxAspectRatio(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:142
	void cv_ximgproc_EdgeBoxes_setMaxAspectRatio_float(cv::ximgproc::EdgeBoxes* instance, float value, Result_void* ocvrs_return) {
		try {
			instance->setMaxAspectRatio(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMinBoxArea() /usr/include/opencv2/ximgproc/edgeboxes.hpp:146
	void cv_ximgproc_EdgeBoxes_getMinBoxArea_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMinBoxArea();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setMinBoxArea(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:149
	void cv_ximgproc_EdgeBoxes_setMinBoxArea_float(cv::ximgproc::EdgeBoxes* instance, float value, Result_void* ocvrs_return) {
		try {
			instance->setMinBoxArea(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getGamma() /usr/include/opencv2/ximgproc/edgeboxes.hpp:153
	void cv_ximgproc_EdgeBoxes_getGamma_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getGamma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setGamma(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:156
	void cv_ximgproc_EdgeBoxes_setGamma_float(cv::ximgproc::EdgeBoxes* instance, float value, Result_void* ocvrs_return) {
		try {
			instance->setGamma(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getKappa() /usr/include/opencv2/ximgproc/edgeboxes.hpp:160
	void cv_ximgproc_EdgeBoxes_getKappa_const(const cv::ximgproc::EdgeBoxes* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getKappa();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setKappa(float) /usr/include/opencv2/ximgproc/edgeboxes.hpp:163
	void cv_ximgproc_EdgeBoxes_setKappa_float(cv::ximgproc::EdgeBoxes* instance, float value, Result_void* ocvrs_return) {
		try {
			instance->setKappa(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// params /usr/include/opencv2/ximgproc/edge_drawing.hpp:113
	void cv_ximgproc_EdgeDrawing_getPropParams_const(const cv::ximgproc::EdgeDrawing* instance, cv::ximgproc::EdgeDrawing::Params* ocvrs_return) {
			cv::ximgproc::EdgeDrawing::Params ret = instance->params;
			*ocvrs_return = ret;
	}
	
	// params /usr/include/opencv2/ximgproc/edge_drawing.hpp:113
	void cv_ximgproc_EdgeDrawing_setPropParams_Params(cv::ximgproc::EdgeDrawing* instance, cv::ximgproc::EdgeDrawing::Params* val) {
			instance->params = *val;
	}
	
	// detectEdges(cv::InputArray) /usr/include/opencv2/ximgproc/edge_drawing.hpp:77
	void cv_ximgproc_EdgeDrawing_detectEdges_const__InputArrayR(cv::ximgproc::EdgeDrawing* instance, const cv::_InputArray* src, Result_void* ocvrs_return) {
		try {
			instance->detectEdges(*src);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getEdgeImage(cv::OutputArray) /usr/include/opencv2/ximgproc/edge_drawing.hpp:83
	void cv_ximgproc_EdgeDrawing_getEdgeImage_const__OutputArrayR(cv::ximgproc::EdgeDrawing* instance, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			instance->getEdgeImage(*dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getGradientImage(cv::OutputArray) /usr/include/opencv2/ximgproc/edge_drawing.hpp:89
	void cv_ximgproc_EdgeDrawing_getGradientImage_const__OutputArrayR(cv::ximgproc::EdgeDrawing* instance, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			instance->getGradientImage(*dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSegments() /usr/include/opencv2/ximgproc/edge_drawing.hpp:93
	void cv_ximgproc_EdgeDrawing_getSegments(cv::ximgproc::EdgeDrawing* instance, Result<std::vector<std::vector<cv::Point>>*>* ocvrs_return) {
		try {
			std::vector<std::vector<cv::Point>> ret = instance->getSegments();
			Ok(new std::vector<std::vector<cv::Point>>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<std::vector<cv::Point>>*>))
	}
	
	// getSegmentIndicesOfLines() /usr/include/opencv2/ximgproc/edge_drawing.hpp:97
	void cv_ximgproc_EdgeDrawing_getSegmentIndicesOfLines_const(const cv::ximgproc::EdgeDrawing* instance, Result<std::vector<int>*>* ocvrs_return) {
		try {
			std::vector<int> ret = instance->getSegmentIndicesOfLines();
			Ok(new std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	// detectLines(cv::OutputArray) /usr/include/opencv2/ximgproc/edge_drawing.hpp:104
	void cv_ximgproc_EdgeDrawing_detectLines_const__OutputArrayR(cv::ximgproc::EdgeDrawing* instance, const cv::_OutputArray* lines, Result_void* ocvrs_return) {
		try {
			instance->detectLines(*lines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detectEllipses(cv::OutputArray) /usr/include/opencv2/ximgproc/edge_drawing.hpp:111
	void cv_ximgproc_EdgeDrawing_detectEllipses_const__OutputArrayR(cv::ximgproc::EdgeDrawing* instance, const cv::_OutputArray* ellipses, Result_void* ocvrs_return) {
		try {
			instance->detectEllipses(*ellipses);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setParams(const EdgeDrawing::Params &) /usr/include/opencv2/ximgproc/edge_drawing.hpp:120
	void cv_ximgproc_EdgeDrawing_setParams_const_ParamsR(cv::ximgproc::EdgeDrawing* instance, const cv::ximgproc::EdgeDrawing::Params* parameters, Result_void* ocvrs_return) {
		try {
			instance->setParams(*parameters);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// Params() /usr/include/opencv2/ximgproc/edge_drawing.hpp:35
	void cv_ximgproc_EdgeDrawing_Params_Params(Result<cv::ximgproc::EdgeDrawing::Params>* ocvrs_return) {
		try {
			cv::ximgproc::EdgeDrawing::Params ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::ximgproc::EdgeDrawing::Params>))
	}
	
	// read(const cv::FileNode &) /usr/include/opencv2/ximgproc/edge_drawing.hpp:69
	void cv_ximgproc_EdgeDrawing_Params_read_const_FileNodeR(cv::ximgproc::EdgeDrawing::Params instance, const cv::FileNode* fn, Result_void* ocvrs_return) {
		try {
			instance.read(*fn);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// write(cv::FileStorage &) /usr/include/opencv2/ximgproc/edge_drawing.hpp:70
	void cv_ximgproc_EdgeDrawing_Params_write_const_FileStorageR(const cv::ximgproc::EdgeDrawing::Params instance, cv::FileStorage* fs, Result_void* ocvrs_return) {
		try {
			instance.write(*fs);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// filter(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/edge_filter.hpp:395
	void cv_ximgproc_FastBilateralSolverFilter_filter_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::ximgproc::FastBilateralSolverFilter* instance, const cv::_InputArray* src, const cv::_InputArray* confidence, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			instance->filter(*src, *confidence, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// filter(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/edge_filter.hpp:466
	void cv_ximgproc_FastGlobalSmootherFilter_filter_const__InputArrayR_const__OutputArrayR(cv::ximgproc::FastGlobalSmootherFilter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			instance->filter(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detect(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/fast_line_detector.hpp:44
	void cv_ximgproc_FastLineDetector_detect_const__InputArrayR_const__OutputArrayR(cv::ximgproc::FastLineDetector* instance, const cv::_InputArray* image, const cv::_OutputArray* lines, Result_void* ocvrs_return) {
		try {
			instance->detect(*image, *lines);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// drawSegments(cv::InputOutputArray, cv::InputArray, bool, cv::Scalar, int) /usr/include/opencv2/ximgproc/fast_line_detector.hpp:54
	void cv_ximgproc_FastLineDetector_drawSegments_const__InputOutputArrayR_const__InputArrayR_bool_Scalar_int(cv::ximgproc::FastLineDetector* instance, const cv::_InputOutputArray* image, const cv::_InputArray* lines, bool draw_arrow, cv::Scalar* linecolor, int linethickness, Result_void* ocvrs_return) {
		try {
			instance->drawSegments(*image, *lines, draw_arrow, *linecolor, linethickness);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// filter(cv::InputArray, cv::OutputArray, int) /usr/include/opencv2/ximgproc/edge_filter.hpp:143
	void cv_ximgproc_GuidedFilter_filter_const__InputArrayR_const__OutputArrayR_int(cv::ximgproc::GuidedFilter* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, int dDepth, Result_void* ocvrs_return) {
		try {
			instance->filter(*src, *dst, dDepth);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getFeatures(const cv::Mat &, cv::Mat &, const int, const int, const int, const int, const int) /usr/include/opencv2/ximgproc/structured_edge_detection.hpp:83
	void cv_ximgproc_RFFeatureGetter_getFeatures_const_const_MatR_MatR_const_int_const_int_const_int_const_int_const_int(const cv::ximgproc::RFFeatureGetter* instance, const cv::Mat* src, cv::Mat* features, const int gnrmRad, const int gsmthRad, const int shrink, const int outNum, const int gradNum, Result_void* ocvrs_return) {
		try {
			instance->getFeatures(*src, *features, gnrmRad, gsmthRad, shrink, outNum, gradNum);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setK(int) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:153
	void cv_ximgproc_RICInterpolator_setK_int(cv::ximgproc::RICInterpolator* instance, int k, Result_void* ocvrs_return) {
		try {
			instance->setK(k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getK() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:157
	void cv_ximgproc_RICInterpolator_getK_const(const cv::ximgproc::RICInterpolator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getK();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setCostMap(const cv::Mat &) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:166
	void cv_ximgproc_RICInterpolator_setCostMap_const_MatR(cv::ximgproc::RICInterpolator* instance, const cv::Mat* costMap, Result_void* ocvrs_return) {
		try {
			instance->setCostMap(*costMap);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setSuperpixelSize(int) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:170
	void cv_ximgproc_RICInterpolator_setSuperpixelSize_int(cv::ximgproc::RICInterpolator* instance, int spSize, Result_void* ocvrs_return) {
		try {
			instance->setSuperpixelSize(spSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSuperpixelSize() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:174
	void cv_ximgproc_RICInterpolator_getSuperpixelSize_const(const cv::ximgproc::RICInterpolator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSuperpixelSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setSuperpixelNNCnt(int) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:178
	void cv_ximgproc_RICInterpolator_setSuperpixelNNCnt_int(cv::ximgproc::RICInterpolator* instance, int spNN, Result_void* ocvrs_return) {
		try {
			instance->setSuperpixelNNCnt(spNN);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSuperpixelNNCnt() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:182
	void cv_ximgproc_RICInterpolator_getSuperpixelNNCnt_const(const cv::ximgproc::RICInterpolator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSuperpixelNNCnt();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setSuperpixelRuler(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:186
	void cv_ximgproc_RICInterpolator_setSuperpixelRuler_float(cv::ximgproc::RICInterpolator* instance, float ruler, Result_void* ocvrs_return) {
		try {
			instance->setSuperpixelRuler(ruler);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSuperpixelRuler() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:190
	void cv_ximgproc_RICInterpolator_getSuperpixelRuler_const(const cv::ximgproc::RICInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getSuperpixelRuler();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setSuperpixelMode(int) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:197
	void cv_ximgproc_RICInterpolator_setSuperpixelMode_int(cv::ximgproc::RICInterpolator* instance, int mode, Result_void* ocvrs_return) {
		try {
			instance->setSuperpixelMode(mode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSuperpixelMode() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:201
	void cv_ximgproc_RICInterpolator_getSuperpixelMode_const(const cv::ximgproc::RICInterpolator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getSuperpixelMode();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setAlpha(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:204
	void cv_ximgproc_RICInterpolator_setAlpha_float(cv::ximgproc::RICInterpolator* instance, float alpha, Result_void* ocvrs_return) {
		try {
			instance->setAlpha(alpha);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getAlpha() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:208
	void cv_ximgproc_RICInterpolator_getAlpha_const(const cv::ximgproc::RICInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getAlpha();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setModelIter(int) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:211
	void cv_ximgproc_RICInterpolator_setModelIter_int(cv::ximgproc::RICInterpolator* instance, int modelIter, Result_void* ocvrs_return) {
		try {
			instance->setModelIter(modelIter);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getModelIter() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:215
	void cv_ximgproc_RICInterpolator_getModelIter_const(const cv::ximgproc::RICInterpolator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getModelIter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setRefineModels(bool) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:218
	void cv_ximgproc_RICInterpolator_setRefineModels_bool(cv::ximgproc::RICInterpolator* instance, bool refineModles, Result_void* ocvrs_return) {
		try {
			instance->setRefineModels(refineModles);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getRefineModels() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:222
	void cv_ximgproc_RICInterpolator_getRefineModels_const(const cv::ximgproc::RICInterpolator* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getRefineModels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setMaxFlow(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:226
	void cv_ximgproc_RICInterpolator_setMaxFlow_float(cv::ximgproc::RICInterpolator* instance, float maxFlow, Result_void* ocvrs_return) {
		try {
			instance->setMaxFlow(maxFlow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaxFlow() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:230
	void cv_ximgproc_RICInterpolator_getMaxFlow_const(const cv::ximgproc::RICInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getMaxFlow();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setUseVariationalRefinement(bool) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:233
	void cv_ximgproc_RICInterpolator_setUseVariationalRefinement_bool(cv::ximgproc::RICInterpolator* instance, bool use_variational_refinement, Result_void* ocvrs_return) {
		try {
			instance->setUseVariationalRefinement(use_variational_refinement);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getUseVariationalRefinement() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:237
	void cv_ximgproc_RICInterpolator_getUseVariationalRefinement_const(const cv::ximgproc::RICInterpolator* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseVariationalRefinement();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setUseGlobalSmootherFilter(bool) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:240
	void cv_ximgproc_RICInterpolator_setUseGlobalSmootherFilter_bool(cv::ximgproc::RICInterpolator* instance, bool use_FGS, Result_void* ocvrs_return) {
		try {
			instance->setUseGlobalSmootherFilter(use_FGS);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getUseGlobalSmootherFilter() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:244
	void cv_ximgproc_RICInterpolator_getUseGlobalSmootherFilter_const(const cv::ximgproc::RICInterpolator* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUseGlobalSmootherFilter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setFGSLambda(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:247
	void cv_ximgproc_RICInterpolator_setFGSLambda_float(cv::ximgproc::RICInterpolator* instance, float lambda, Result_void* ocvrs_return) {
		try {
			instance->setFGSLambda(lambda);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getFGSLambda() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:251
	void cv_ximgproc_RICInterpolator_getFGSLambda_const(const cv::ximgproc::RICInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getFGSLambda();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setFGSSigma(float) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:254
	void cv_ximgproc_RICInterpolator_setFGSSigma_float(cv::ximgproc::RICInterpolator* instance, float sigma, Result_void* ocvrs_return) {
		try {
			instance->setFGSSigma(sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getFGSSigma() /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:258
	void cv_ximgproc_RICInterpolator_getFGSSigma_const(const cv::ximgproc::RICInterpolator* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getFGSSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// create(int, int, int, int, int, double, double, int) /usr/include/opencv2/ximgproc/ridgefilter.hpp:42
	void cv_ximgproc_RidgeDetectionFilter_create_int_int_int_int_int_double_double_int(int ddepth, int dx, int dy, int ksize, int out_dtype, double scale, double delta, int borderType, Result<cv::Ptr<cv::ximgproc::RidgeDetectionFilter>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ximgproc::RidgeDetectionFilter> ret = cv::ximgproc::RidgeDetectionFilter::create(ddepth, dx, dy, ksize, out_dtype, scale, delta, borderType);
			Ok(new cv::Ptr<cv::ximgproc::RidgeDetectionFilter>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ximgproc::RidgeDetectionFilter>*>))
	}
	
	// getRidgeFilteredImage(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/ridgefilter.hpp:48
	void cv_ximgproc_RidgeDetectionFilter_getRidgeFilteredImage_const__InputArrayR_const__OutputArrayR(cv::ximgproc::RidgeDetectionFilter* instance, const cv::_InputArray* _img, const cv::_OutputArray* out, Result_void* ocvrs_return) {
		try {
			instance->getRidgeFilteredImage(*_img, *out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNumberOfSuperpixels() /usr/include/opencv2/ximgproc/scansegment.hpp:32
	void cv_ximgproc_ScanSegment_getNumberOfSuperpixels(cv::ximgproc::ScanSegment* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumberOfSuperpixels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// iterate(cv::InputArray) /usr/include/opencv2/ximgproc/scansegment.hpp:43
	void cv_ximgproc_ScanSegment_iterate_const__InputArrayR(cv::ximgproc::ScanSegment* instance, const cv::_InputArray* img, Result_void* ocvrs_return) {
		try {
			instance->iterate(*img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getLabels(cv::OutputArray) /usr/include/opencv2/ximgproc/scansegment.hpp:52
	void cv_ximgproc_ScanSegment_getLabels_const__OutputArrayR(cv::ximgproc::ScanSegment* instance, const cv::_OutputArray* labels_out, Result_void* ocvrs_return) {
		try {
			instance->getLabels(*labels_out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getLabelContourMask(cv::OutputArray, bool) /usr/include/opencv2/ximgproc/scansegment.hpp:61
	void cv_ximgproc_ScanSegment_getLabelContourMask_const__OutputArrayR_bool(cv::ximgproc::ScanSegment* instance, const cv::_OutputArray* image, bool thick_line, Result_void* ocvrs_return) {
		try {
			instance->getLabelContourMask(*image, thick_line);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// interpolate(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/sparse_match_interpolator.hpp:69
	void cv_ximgproc_SparseMatchInterpolator_interpolate_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::ximgproc::SparseMatchInterpolator* instance, const cv::_InputArray* from_image, const cv::_InputArray* from_points, const cv::_InputArray* to_image, const cv::_InputArray* to_points, const cv::_OutputArray* dense_flow, Result_void* ocvrs_return) {
		try {
			instance->interpolate(*from_image, *from_points, *to_image, *to_points, *dense_flow);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detectEdges(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/structured_edge_detection.hpp:109
	void cv_ximgproc_StructuredEdgeDetection_detectEdges_const_const__InputArrayR_const__OutputArrayR(const cv::ximgproc::StructuredEdgeDetection* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			instance->detectEdges(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// computeOrientation(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/structured_edge_detection.hpp:116
	void cv_ximgproc_StructuredEdgeDetection_computeOrientation_const_const__InputArrayR_const__OutputArrayR(const cv::ximgproc::StructuredEdgeDetection* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			instance->computeOrientation(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// edgesNms(cv::InputArray, cv::InputArray, cv::OutputArray, int, int, float, bool) /usr/include/opencv2/ximgproc/structured_edge_detection.hpp:129
	void cv_ximgproc_StructuredEdgeDetection_edgesNms_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR_int_int_float_bool(const cv::ximgproc::StructuredEdgeDetection* instance, const cv::_InputArray* edge_image, const cv::_InputArray* orientation_image, const cv::_OutputArray* dst, int r, int s, float m, bool isParallel, Result_void* ocvrs_return) {
		try {
			instance->edgesNms(*edge_image, *orientation_image, *dst, r, s, m, isParallel);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNumberOfSuperpixels() /usr/include/opencv2/ximgproc/lsc.hpp:78
	void cv_ximgproc_SuperpixelLSC_getNumberOfSuperpixels_const(const cv::ximgproc::SuperpixelLSC* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumberOfSuperpixels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// iterate(int) /usr/include/opencv2/ximgproc/lsc.hpp:94
	void cv_ximgproc_SuperpixelLSC_iterate_int(cv::ximgproc::SuperpixelLSC* instance, int num_iterations, Result_void* ocvrs_return) {
		try {
			instance->iterate(num_iterations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getLabels(cv::OutputArray) /usr/include/opencv2/ximgproc/lsc.hpp:106
	void cv_ximgproc_SuperpixelLSC_getLabels_const_const__OutputArrayR(const cv::ximgproc::SuperpixelLSC* instance, const cv::_OutputArray* labels_out, Result_void* ocvrs_return) {
		try {
			instance->getLabels(*labels_out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getLabelContourMask(cv::OutputArray, bool) /usr/include/opencv2/ximgproc/lsc.hpp:118
	void cv_ximgproc_SuperpixelLSC_getLabelContourMask_const_const__OutputArrayR_bool(const cv::ximgproc::SuperpixelLSC* instance, const cv::_OutputArray* image, bool thick_line, Result_void* ocvrs_return) {
		try {
			instance->getLabelContourMask(*image, thick_line);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// enforceLabelConnectivity(int) /usr/include/opencv2/ximgproc/lsc.hpp:129
	void cv_ximgproc_SuperpixelLSC_enforceLabelConnectivity_int(cv::ximgproc::SuperpixelLSC* instance, int min_element_size, Result_void* ocvrs_return) {
		try {
			instance->enforceLabelConnectivity(min_element_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNumberOfSuperpixels() /usr/include/opencv2/ximgproc/seeds.hpp:75
	void cv_ximgproc_SuperpixelSEEDS_getNumberOfSuperpixels(cv::ximgproc::SuperpixelSEEDS* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumberOfSuperpixels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// iterate(cv::InputArray, int) /usr/include/opencv2/ximgproc/seeds.hpp:99
	void cv_ximgproc_SuperpixelSEEDS_iterate_const__InputArrayR_int(cv::ximgproc::SuperpixelSEEDS* instance, const cv::_InputArray* img, int num_iterations, Result_void* ocvrs_return) {
		try {
			instance->iterate(*img, num_iterations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getLabels(cv::OutputArray) /usr/include/opencv2/ximgproc/seeds.hpp:111
	void cv_ximgproc_SuperpixelSEEDS_getLabels_const__OutputArrayR(cv::ximgproc::SuperpixelSEEDS* instance, const cv::_OutputArray* labels_out, Result_void* ocvrs_return) {
		try {
			instance->getLabels(*labels_out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getLabelContourMask(cv::OutputArray, bool) /usr/include/opencv2/ximgproc/seeds.hpp:139
	void cv_ximgproc_SuperpixelSEEDS_getLabelContourMask_const__OutputArrayR_bool(cv::ximgproc::SuperpixelSEEDS* instance, const cv::_OutputArray* image, bool thick_line, Result_void* ocvrs_return) {
		try {
			instance->getLabelContourMask(*image, thick_line);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNumberOfSuperpixels() /usr/include/opencv2/ximgproc/slic.hpp:85
	void cv_ximgproc_SuperpixelSLIC_getNumberOfSuperpixels_const(const cv::ximgproc::SuperpixelSLIC* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNumberOfSuperpixels();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// iterate(int) /usr/include/opencv2/ximgproc/slic.hpp:101
	void cv_ximgproc_SuperpixelSLIC_iterate_int(cv::ximgproc::SuperpixelSLIC* instance, int num_iterations, Result_void* ocvrs_return) {
		try {
			instance->iterate(num_iterations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getLabels(cv::OutputArray) /usr/include/opencv2/ximgproc/slic.hpp:113
	void cv_ximgproc_SuperpixelSLIC_getLabels_const_const__OutputArrayR(const cv::ximgproc::SuperpixelSLIC* instance, const cv::_OutputArray* labels_out, Result_void* ocvrs_return) {
		try {
			instance->getLabels(*labels_out);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getLabelContourMask(cv::OutputArray, bool) /usr/include/opencv2/ximgproc/slic.hpp:125
	void cv_ximgproc_SuperpixelSLIC_getLabelContourMask_const_const__OutputArrayR_bool(const cv::ximgproc::SuperpixelSLIC* instance, const cv::_OutputArray* image, bool thick_line, Result_void* ocvrs_return) {
		try {
			instance->getLabelContourMask(*image, thick_line);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// enforceLabelConnectivity(int) /usr/include/opencv2/ximgproc/slic.hpp:136
	void cv_ximgproc_SuperpixelSLIC_enforceLabelConnectivity_int(cv::ximgproc::SuperpixelSLIC* instance, int min_element_size, Result_void* ocvrs_return) {
		try {
			instance->enforceLabelConnectivity(min_element_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// processImage(cv::InputArray, cv::OutputArray) /usr/include/opencv2/ximgproc/segmentation.hpp:52
	void cv_ximgproc_segmentation_GraphSegmentation_processImage_const__InputArrayR_const__OutputArrayR(cv::ximgproc::segmentation::GraphSegmentation* instance, const cv::_InputArray* src, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			instance->processImage(*src, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setSigma(double) /usr/include/opencv2/ximgproc/segmentation.hpp:54
	void cv_ximgproc_segmentation_GraphSegmentation_setSigma_double(cv::ximgproc::segmentation::GraphSegmentation* instance, double sigma, Result_void* ocvrs_return) {
		try {
			instance->setSigma(sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSigma() /usr/include/opencv2/ximgproc/segmentation.hpp:55
	void cv_ximgproc_segmentation_GraphSegmentation_getSigma(cv::ximgproc::segmentation::GraphSegmentation* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setK(float) /usr/include/opencv2/ximgproc/segmentation.hpp:57
	void cv_ximgproc_segmentation_GraphSegmentation_setK_float(cv::ximgproc::segmentation::GraphSegmentation* instance, float k, Result_void* ocvrs_return) {
		try {
			instance->setK(k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getK() /usr/include/opencv2/ximgproc/segmentation.hpp:58
	void cv_ximgproc_segmentation_GraphSegmentation_getK(cv::ximgproc::segmentation::GraphSegmentation* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getK();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setMinSize(int) /usr/include/opencv2/ximgproc/segmentation.hpp:60
	void cv_ximgproc_segmentation_GraphSegmentation_setMinSize_int(cv::ximgproc::segmentation::GraphSegmentation* instance, int min_size, Result_void* ocvrs_return) {
		try {
			instance->setMinSize(min_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMinSize() /usr/include/opencv2/ximgproc/segmentation.hpp:61
	void cv_ximgproc_segmentation_GraphSegmentation_getMinSize(cv::ximgproc::segmentation::GraphSegmentation* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setBaseImage(cv::InputArray) /usr/include/opencv2/ximgproc/segmentation.hpp:187
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_setBaseImage_const__InputArrayR(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, const cv::_InputArray* img, Result_void* ocvrs_return) {
		try {
			instance->setBaseImage(*img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// switchToSingleStrategy(int, float) /usr/include/opencv2/ximgproc/segmentation.hpp:193
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSingleStrategy_int_float(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, int k, float sigma, Result_void* ocvrs_return) {
		try {
			instance->switchToSingleStrategy(k, sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// switchToSelectiveSearchFast(int, int, float) /usr/include/opencv2/ximgproc/segmentation.hpp:200
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSelectiveSearchFast_int_int_float(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, int base_k, int inc_k, float sigma, Result_void* ocvrs_return) {
		try {
			instance->switchToSelectiveSearchFast(base_k, inc_k, sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// switchToSelectiveSearchQuality(int, int, float) /usr/include/opencv2/ximgproc/segmentation.hpp:207
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_switchToSelectiveSearchQuality_int_int_float(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, int base_k, int inc_k, float sigma, Result_void* ocvrs_return) {
		try {
			instance->switchToSelectiveSearchQuality(base_k, inc_k, sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// addImage(cv::InputArray) /usr/include/opencv2/ximgproc/segmentation.hpp:212
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_addImage_const__InputArrayR(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, const cv::_InputArray* img, Result_void* ocvrs_return) {
		try {
			instance->addImage(*img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// clearImages() /usr/include/opencv2/ximgproc/segmentation.hpp:216
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearImages(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, Result_void* ocvrs_return) {
		try {
			instance->clearImages();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// addGraphSegmentation(Ptr<cv::ximgproc::segmentation::GraphSegmentation>) /usr/include/opencv2/ximgproc/segmentation.hpp:221
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_addGraphSegmentation_Ptr_GraphSegmentation_(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>* g, Result_void* ocvrs_return) {
		try {
			instance->addGraphSegmentation(*g);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// clearGraphSegmentations() /usr/include/opencv2/ximgproc/segmentation.hpp:225
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearGraphSegmentations(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, Result_void* ocvrs_return) {
		try {
			instance->clearGraphSegmentations();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// addStrategy(Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>) /usr/include/opencv2/ximgproc/segmentation.hpp:230
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_addStrategy_Ptr_SelectiveSearchSegmentationStrategy_(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* s, Result_void* ocvrs_return) {
		try {
			instance->addStrategy(*s);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// clearStrategies() /usr/include/opencv2/ximgproc/segmentation.hpp:234
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_clearStrategies(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, Result_void* ocvrs_return) {
		try {
			instance->clearStrategies();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// process(std::vector<Rect> &) /usr/include/opencv2/ximgproc/segmentation.hpp:239
	void cv_ximgproc_segmentation_SelectiveSearchSegmentation_process_vector_Rect_R(cv::ximgproc::segmentation::SelectiveSearchSegmentation* instance, std::vector<cv::Rect>* rects, Result_void* ocvrs_return) {
		try {
			instance->process(*rects);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setImage(cv::InputArray, cv::InputArray, cv::InputArray, int) /usr/include/opencv2/ximgproc/segmentation.hpp:82
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_setImage_const__InputArrayR_const__InputArrayR_const__InputArrayR_int(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance, const cv::_InputArray* img, const cv::_InputArray* regions, const cv::_InputArray* sizes, int image_id, Result_void* ocvrs_return) {
		try {
			instance->setImage(*img, *regions, *sizes, image_id);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// get(int, int) /usr/include/opencv2/ximgproc/segmentation.hpp:88
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_get_int_int(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance, int r1, int r2, Result<float>* ocvrs_return) {
		try {
			float ret = instance->get(r1, r2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// merge(int, int) /usr/include/opencv2/ximgproc/segmentation.hpp:94
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategy_merge_int_int(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy* instance, int r1, int r2, Result_void* ocvrs_return) {
		try {
			instance->merge(r1, r2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// addStrategy(Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>, float) /usr/include/opencv2/ximgproc/segmentation.hpp:142
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_addStrategy_Ptr_SelectiveSearchSegmentationStrategy__float(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple* instance, cv::Ptr<cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategy>* g, float weight, Result_void* ocvrs_return) {
		try {
			instance->addStrategy(*g, weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// clearStrategies() /usr/include/opencv2/ximgproc/segmentation.hpp:145
	void cv_ximgproc_segmentation_SelectiveSearchSegmentationStrategyMultiple_clearStrategies(cv::ximgproc::segmentation::SelectiveSearchSegmentationStrategyMultiple* instance, Result_void* ocvrs_return) {
		try {
			instance->clearStrategies();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
