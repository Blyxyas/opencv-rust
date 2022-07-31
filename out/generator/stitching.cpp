#include "ocvrs_common.hpp"
#include <opencv2/stitching.hpp>
#include "stitching_types.hpp"

extern "C" {
	// autoDetectWaveCorrectKind(const std::vector<Mat> &) /usr/include/opencv2/stitching/detail/motion_estimators.hpp:342
	void cv_detail_autoDetectWaveCorrectKind_const_vector_Mat_R(const std::vector<cv::Mat>* rmats, Result<cv::detail::WaveCorrectKind>* ocvrs_return) {
		try {
			cv::detail::WaveCorrectKind ret = cv::detail::autoDetectWaveCorrectKind(*rmats);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::WaveCorrectKind>))
	}
	
	// computeImageFeatures(const Ptr<cv::Feature2D> &, cv::InputArray, cv::detail::ImageFeatures &, cv::InputArray) /usr/include/opencv2/stitching/detail/matchers.hpp:86
	void cv_detail_computeImageFeatures_const_Ptr_Feature2D_R_const__InputArrayR_ImageFeaturesR_const__InputArrayR(const cv::Ptr<cv::Feature2D>* featuresFinder, const cv::_InputArray* image, cv::detail::ImageFeatures* features, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			cv::detail::computeImageFeatures(*featuresFinder, *image, *features, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// computeImageFeatures(const Ptr<cv::Feature2D> &, cv::InputArrayOfArrays, std::vector<ImageFeatures> &, cv::InputArrayOfArrays) /usr/include/opencv2/stitching/detail/matchers.hpp:73
	void cv_detail_computeImageFeatures_const_Ptr_Feature2D_R_const__InputArrayR_vector_ImageFeatures_R_const__InputArrayR(const cv::Ptr<cv::Feature2D>* featuresFinder, const cv::_InputArray* images, std::vector<cv::detail::ImageFeatures>* features, const cv::_InputArray* masks, Result_void* ocvrs_return) {
		try {
			cv::detail::computeImageFeatures(*featuresFinder, *images, *features, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// createLaplacePyrGpu(cv::InputArray, int, std::vector<UMat> &) /usr/include/opencv2/stitching/detail/blenders.hpp:173
	void cv_detail_createLaplacePyrGpu_const__InputArrayR_int_vector_UMat_R(const cv::_InputArray* img, int num_levels, std::vector<cv::UMat>* pyr, Result_void* ocvrs_return) {
		try {
			cv::detail::createLaplacePyrGpu(*img, num_levels, *pyr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// createLaplacePyr(cv::InputArray, int, std::vector<UMat> &) /usr/include/opencv2/stitching/detail/blenders.hpp:172
	void cv_detail_createLaplacePyr_const__InputArrayR_int_vector_UMat_R(const cv::_InputArray* img, int num_levels, std::vector<cv::UMat>* pyr, Result_void* ocvrs_return) {
		try {
			cv::detail::createLaplacePyr(*img, num_levels, *pyr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// createWeightMap(cv::InputArray, float, cv::InputOutputArray) /usr/include/opencv2/stitching/detail/blenders.hpp:170
	void cv_detail_createWeightMap_const__InputArrayR_float_const__InputOutputArrayR(const cv::_InputArray* mask, float sharpness, const cv::_InputOutputArray* weight, Result_void* ocvrs_return) {
		try {
			cv::detail::createWeightMap(*mask, sharpness, *weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// findMaxSpanningTree(int, const std::vector<MatchesInfo> &, cv::detail::Graph &, std::vector<int> &) /usr/include/opencv2/stitching/detail/motion_estimators.hpp:364
	void cv_detail_findMaxSpanningTree_int_const_vector_MatchesInfo_R_GraphR_vector_int_R(int num_images, const std::vector<cv::detail::MatchesInfo>* pairwise_matches, cv::detail::Graph* span_tree, std::vector<int>* centers, Result_void* ocvrs_return) {
		try {
			cv::detail::findMaxSpanningTree(num_images, *pairwise_matches, *span_tree, *centers);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// leaveBiggestComponent(std::vector<ImageFeatures> &, std::vector<MatchesInfo> &, float) /usr/include/opencv2/stitching/detail/motion_estimators.hpp:359
	void cv_detail_leaveBiggestComponent_vector_ImageFeatures_R_vector_MatchesInfo_R_float(std::vector<cv::detail::ImageFeatures>* features, std::vector<cv::detail::MatchesInfo>* pairwise_matches, float conf_threshold, Result<std::vector<int>*>* ocvrs_return) {
		try {
			std::vector<int> ret = cv::detail::leaveBiggestComponent(*features, *pairwise_matches, conf_threshold);
			Ok(new std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	// matchesGraphAsString(std::vector<String> &, std::vector<MatchesInfo> &, float) /usr/include/opencv2/stitching/detail/motion_estimators.hpp:356
	void cv_detail_matchesGraphAsString_vector_String_R_vector_MatchesInfo_R_float(std::vector<cv::String>* pathes, std::vector<cv::detail::MatchesInfo>* pairwise_matches, float conf_threshold, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = cv::detail::matchesGraphAsString(*pathes, *pairwise_matches, conf_threshold);
			Ok(ocvrs_create_string(ret.c_str()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// normalizeUsingWeightMap(cv::InputArray, cv::InputOutputArray) /usr/include/opencv2/stitching/detail/blenders.hpp:168
	void cv_detail_normalizeUsingWeightMap_const__InputArrayR_const__InputOutputArrayR(const cv::_InputArray* weight, const cv::_InputOutputArray* src, Result_void* ocvrs_return) {
		try {
			cv::detail::normalizeUsingWeightMap(*weight, *src);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// overlapRoi(cv::Point, cv::Point, cv::Size, cv::Size, cv::Rect &) /usr/include/opencv2/stitching/detail/util.hpp:103
	void cv_detail_overlapRoi_Point_Point_Size_Size_RectR(cv::Point* tl1, cv::Point* tl2, cv::Size* sz1, cv::Size* sz2, cv::Rect* roi, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::detail::overlapRoi(*tl1, *tl2, *sz1, *sz2, *roi);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// restoreImageFromLaplacePyrGpu(std::vector<UMat> &) /usr/include/opencv2/stitching/detail/blenders.hpp:177
	void cv_detail_restoreImageFromLaplacePyrGpu_vector_UMat_R(std::vector<cv::UMat>* pyr, Result_void* ocvrs_return) {
		try {
			cv::detail::restoreImageFromLaplacePyrGpu(*pyr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// restoreImageFromLaplacePyr(std::vector<UMat> &) /usr/include/opencv2/stitching/detail/blenders.hpp:176
	void cv_detail_restoreImageFromLaplacePyr_vector_UMat_R(std::vector<cv::UMat>* pyr, Result_void* ocvrs_return) {
		try {
			cv::detail::restoreImageFromLaplacePyr(*pyr);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// resultRoiIntersection(const std::vector<Point> &, const std::vector<Size> &) /usr/include/opencv2/stitching/detail/util.hpp:106
	void cv_detail_resultRoiIntersection_const_vector_Point_R_const_vector_Size_R(const std::vector<cv::Point>* corners, const std::vector<cv::Size>* sizes, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = cv::detail::resultRoiIntersection(*corners, *sizes);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// resultRoi(const std::vector<Point> &, const std::vector<Size> &) /usr/include/opencv2/stitching/detail/util.hpp:105
	void cv_detail_resultRoi_const_vector_Point_R_const_vector_Size_R(const std::vector<cv::Point>* corners, const std::vector<cv::Size>* sizes, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = cv::detail::resultRoi(*corners, *sizes);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// resultRoi(const std::vector<Point> &, const std::vector<UMat> &) /usr/include/opencv2/stitching/detail/util.hpp:104
	void cv_detail_resultRoi_const_vector_Point_R_const_vector_UMat_R(const std::vector<cv::Point>* corners, const std::vector<cv::UMat>* images, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = cv::detail::resultRoi(*corners, *images);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// resultTl(const std::vector<Point> &) /usr/include/opencv2/stitching/detail/util.hpp:107
	void cv_detail_resultTl_const_vector_Point_R(const std::vector<cv::Point>* corners, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = cv::detail::resultTl(*corners);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	// selectRandomSubset(int, int, std::vector<int> &) /usr/include/opencv2/stitching/detail/util.hpp:110
	void cv_detail_selectRandomSubset_int_int_vector_int_R(int count, int size, std::vector<int>* subset, Result_void* ocvrs_return) {
		try {
			cv::detail::selectRandomSubset(count, size, *subset);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// stitchingLogLevel() /usr/include/opencv2/stitching/detail/util.hpp:112
	void cv_detail_stitchingLogLevel(Result<int>* ocvrs_return) {
		try {
			int ret = cv::detail::stitchingLogLevel();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// waveCorrect(std::vector<Mat> &, cv::detail::WaveCorrectKind) /usr/include/opencv2/stitching/detail/motion_estimators.hpp:349
	void cv_detail_waveCorrect_vector_Mat_R_WaveCorrectKind(std::vector<cv::Mat>* rmats, cv::detail::WaveCorrectKind kind, Result_void* ocvrs_return) {
		try {
			cv::detail::waveCorrect(*rmats, kind);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_AffineWarper_delete(cv::AffineWarper* instance) {
		delete instance;
	}
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:162
	void cv_AffineWarper_create_const_float(const cv::AffineWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_CompressedRectilinearPortraitWarper_delete(cv::CompressedRectilinearPortraitWarper* instance) {
		delete instance;
	}
	// CompressedRectilinearPortraitWarper(float, float) /usr/include/opencv2/stitching/warpers.hpp:208
	void cv_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_float(float A, float B, Result<cv::CompressedRectilinearPortraitWarper*>* ocvrs_return) {
		try {
			cv::CompressedRectilinearPortraitWarper* ret = new cv::CompressedRectilinearPortraitWarper(A, B);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::CompressedRectilinearPortraitWarper*>))
	}
	
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:212
	void cv_CompressedRectilinearPortraitWarper_create_const_float(const cv::CompressedRectilinearPortraitWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_CompressedRectilinearWarper_delete(cv::CompressedRectilinearWarper* instance) {
		delete instance;
	}
	// CompressedRectilinearWarper(float, float) /usr/include/opencv2/stitching/warpers.hpp:197
	void cv_CompressedRectilinearWarper_CompressedRectilinearWarper_float_float(float A, float B, Result<cv::CompressedRectilinearWarper*>* ocvrs_return) {
		try {
			cv::CompressedRectilinearWarper* ret = new cv::CompressedRectilinearWarper(A, B);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::CompressedRectilinearWarper*>))
	}
	
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:201
	void cv_CompressedRectilinearWarper_create_const_float(const cv::CompressedRectilinearWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_CylindricalWarper_delete(cv::CylindricalWarper* instance) {
		delete instance;
	}
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:171
	void cv_CylindricalWarper_create_const_float(const cv::CylindricalWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_FisheyeWarper_delete(cv::FisheyeWarper* instance) {
		delete instance;
	}
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:184
	void cv_FisheyeWarper_create_const_float(const cv::FisheyeWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_MercatorWarper_delete(cv::MercatorWarper* instance) {
		delete instance;
	}
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:240
	void cv_MercatorWarper_create_const_float(const cv::MercatorWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_PaniniPortraitWarper_delete(cv::PaniniPortraitWarper* instance) {
		delete instance;
	}
	// PaniniPortraitWarper(float, float) /usr/include/opencv2/stitching/warpers.hpp:230
	void cv_PaniniPortraitWarper_PaniniPortraitWarper_float_float(float A, float B, Result<cv::PaniniPortraitWarper*>* ocvrs_return) {
		try {
			cv::PaniniPortraitWarper* ret = new cv::PaniniPortraitWarper(A, B);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::PaniniPortraitWarper*>))
	}
	
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:234
	void cv_PaniniPortraitWarper_create_const_float(const cv::PaniniPortraitWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_PaniniWarper_delete(cv::PaniniWarper* instance) {
		delete instance;
	}
	// PaniniWarper(float, float) /usr/include/opencv2/stitching/warpers.hpp:219
	void cv_PaniniWarper_PaniniWarper_float_float(float A, float B, Result<cv::PaniniWarper*>* ocvrs_return) {
		try {
			cv::PaniniWarper* ret = new cv::PaniniWarper(A, B);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::PaniniWarper*>))
	}
	
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:223
	void cv_PaniniWarper_create_const_float(const cv::PaniniWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_PlaneWarper_delete(cv::PlaneWarper* instance) {
		delete instance;
	}
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:153
	void cv_PlaneWarper_create_const_float(const cv::PlaneWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_PyRotationWarper_delete(cv::PyRotationWarper* instance) {
		delete instance;
	}
	// PyRotationWarper(cv::String, float) /usr/include/opencv2/stitching/warpers.hpp:55
	void cv_PyRotationWarper_PyRotationWarper_String_float(char* type, float scale, Result<cv::PyRotationWarper*>* ocvrs_return) {
		try {
			cv::PyRotationWarper* ret = new cv::PyRotationWarper(std::string(type), scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::PyRotationWarper*>))
	}
	
	// PyRotationWarper() /usr/include/opencv2/stitching/warpers.hpp:56
	void cv_PyRotationWarper_PyRotationWarper(Result<cv::PyRotationWarper*>* ocvrs_return) {
		try {
			cv::PyRotationWarper* ret = new cv::PyRotationWarper();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::PyRotationWarper*>))
	}
	
	// warpPoint(const cv::Point2f &, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/warpers.hpp:66
	void cv_PyRotationWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(cv::PyRotationWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->warpPoint(*pt, *K, *R);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	// warpPointBackward(const cv::Point2f &, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/warpers.hpp:76
	void cv_PyRotationWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR(cv::PyRotationWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->warpPointBackward(*pt, *K, *R);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/warpers.hpp:93
	void cv_PyRotationWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::PyRotationWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/warpers.hpp:105
	void cv_PyRotationWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::PyRotationWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	// warpBackward(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::Size, cv::OutputArray) /usr/include/opencv2/stitching/warpers.hpp:118
	void cv_PyRotationWarper_warpBackward_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_Size_const__OutputArrayR(cv::PyRotationWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, cv::Size* dst_size, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			instance->warpBackward(*src, *K, *R, interp_mode, border_mode, *dst_size, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// warpRoi(cv::Size, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/warpers.hpp:127
	void cv_PyRotationWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(cv::PyRotationWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->warpRoi(*src_size, *K, *R);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// getScale() /usr/include/opencv2/stitching/warpers.hpp:129
	void cv_PyRotationWarper_getScale_const(const cv::PyRotationWarper* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setScale(float) /usr/include/opencv2/stitching/warpers.hpp:130
	void cv_PyRotationWarper_setScale_float(cv::PyRotationWarper* instance, float unnamed, Result_void* ocvrs_return) {
		try {
			instance->setScale(unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_SphericalWarper_delete(cv::SphericalWarper* instance) {
		delete instance;
	}
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:178
	void cv_SphericalWarper_create_const_float(const cv::SphericalWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_StereographicWarper_delete(cv::StereographicWarper* instance) {
		delete instance;
	}
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:190
	void cv_StereographicWarper_create_const_float(const cv::StereographicWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_Stitcher_delete(cv::Stitcher* instance) {
		delete instance;
	}
	// create(cv::Stitcher::Mode) /usr/include/opencv2/stitching.hpp:184
	void cv_Stitcher_create_Mode(cv::Stitcher::Mode mode, Result<cv::Ptr<cv::Stitcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Stitcher> ret = cv::Stitcher::create(mode);
			Ok(new cv::Ptr<cv::Stitcher>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::Stitcher>*>))
	}
	
	// registrationResol() /usr/include/opencv2/stitching.hpp:186
	void cv_Stitcher_registrationResol_const(const cv::Stitcher* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->registrationResol();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setRegistrationResol(double) /usr/include/opencv2/stitching.hpp:187
	void cv_Stitcher_setRegistrationResol_double(cv::Stitcher* instance, double resol_mpx, Result_void* ocvrs_return) {
		try {
			instance->setRegistrationResol(resol_mpx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// seamEstimationResol() /usr/include/opencv2/stitching.hpp:189
	void cv_Stitcher_seamEstimationResol_const(const cv::Stitcher* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->seamEstimationResol();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setSeamEstimationResol(double) /usr/include/opencv2/stitching.hpp:190
	void cv_Stitcher_setSeamEstimationResol_double(cv::Stitcher* instance, double resol_mpx, Result_void* ocvrs_return) {
		try {
			instance->setSeamEstimationResol(resol_mpx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// compositingResol() /usr/include/opencv2/stitching.hpp:192
	void cv_Stitcher_compositingResol_const(const cv::Stitcher* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->compositingResol();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setCompositingResol(double) /usr/include/opencv2/stitching.hpp:193
	void cv_Stitcher_setCompositingResol_double(cv::Stitcher* instance, double resol_mpx, Result_void* ocvrs_return) {
		try {
			instance->setCompositingResol(resol_mpx);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// panoConfidenceThresh() /usr/include/opencv2/stitching.hpp:195
	void cv_Stitcher_panoConfidenceThresh_const(const cv::Stitcher* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->panoConfidenceThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setPanoConfidenceThresh(double) /usr/include/opencv2/stitching.hpp:196
	void cv_Stitcher_setPanoConfidenceThresh_double(cv::Stitcher* instance, double conf_thresh, Result_void* ocvrs_return) {
		try {
			instance->setPanoConfidenceThresh(conf_thresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// waveCorrection() /usr/include/opencv2/stitching.hpp:198
	void cv_Stitcher_waveCorrection_const(const cv::Stitcher* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->waveCorrection();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setWaveCorrection(bool) /usr/include/opencv2/stitching.hpp:199
	void cv_Stitcher_setWaveCorrection_bool(cv::Stitcher* instance, bool flag, Result_void* ocvrs_return) {
		try {
			instance->setWaveCorrection(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// interpolationFlags() /usr/include/opencv2/stitching.hpp:201
	void cv_Stitcher_interpolationFlags_const(const cv::Stitcher* instance, Result<cv::InterpolationFlags>* ocvrs_return) {
		try {
			cv::InterpolationFlags ret = instance->interpolationFlags();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::InterpolationFlags>))
	}
	
	// setInterpolationFlags(cv::InterpolationFlags) /usr/include/opencv2/stitching.hpp:202
	void cv_Stitcher_setInterpolationFlags_InterpolationFlags(cv::Stitcher* instance, cv::InterpolationFlags interp_flags, Result_void* ocvrs_return) {
		try {
			instance->setInterpolationFlags(interp_flags);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// waveCorrectKind() /usr/include/opencv2/stitching.hpp:204
	void cv_Stitcher_waveCorrectKind_const(const cv::Stitcher* instance, Result<cv::detail::WaveCorrectKind>* ocvrs_return) {
		try {
			cv::detail::WaveCorrectKind ret = instance->waveCorrectKind();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::WaveCorrectKind>))
	}
	
	// setWaveCorrectKind(detail::WaveCorrectKind) /usr/include/opencv2/stitching.hpp:205
	void cv_Stitcher_setWaveCorrectKind_WaveCorrectKind(cv::Stitcher* instance, cv::detail::WaveCorrectKind kind, Result_void* ocvrs_return) {
		try {
			instance->setWaveCorrectKind(kind);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// featuresFinder() /usr/include/opencv2/stitching.hpp:207
	void cv_Stitcher_featuresFinder(cv::Stitcher* instance, Result<cv::Ptr<cv::Feature2D>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Feature2D> ret = instance->featuresFinder();
			Ok(new cv::Ptr<cv::Feature2D>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::Feature2D>*>))
	}
	
	// featuresFinder() /usr/include/opencv2/stitching.hpp:208
	void cv_Stitcher_featuresFinder_const(const cv::Stitcher* instance, Result<cv::Ptr<cv::Feature2D>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::Feature2D> ret = instance->featuresFinder();
			Ok(new cv::Ptr<cv::Feature2D>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::Feature2D>*>))
	}
	
	// setFeaturesFinder(Ptr<cv::Feature2D>) /usr/include/opencv2/stitching.hpp:209
	void cv_Stitcher_setFeaturesFinder_Ptr_Feature2D_(cv::Stitcher* instance, cv::Ptr<cv::Feature2D>* features_finder, Result_void* ocvrs_return) {
		try {
			instance->setFeaturesFinder(*features_finder);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// featuresMatcher() /usr/include/opencv2/stitching.hpp:212
	void cv_Stitcher_featuresMatcher(cv::Stitcher* instance, Result<cv::Ptr<cv::detail::FeaturesMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::FeaturesMatcher> ret = instance->featuresMatcher();
			Ok(new cv::Ptr<cv::detail::FeaturesMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::FeaturesMatcher>*>))
	}
	
	// featuresMatcher() /usr/include/opencv2/stitching.hpp:213
	void cv_Stitcher_featuresMatcher_const(const cv::Stitcher* instance, Result<cv::Ptr<cv::detail::FeaturesMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::FeaturesMatcher> ret = instance->featuresMatcher();
			Ok(new cv::Ptr<cv::detail::FeaturesMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::FeaturesMatcher>*>))
	}
	
	// setFeaturesMatcher(Ptr<detail::FeaturesMatcher>) /usr/include/opencv2/stitching.hpp:214
	void cv_Stitcher_setFeaturesMatcher_Ptr_FeaturesMatcher_(cv::Stitcher* instance, cv::Ptr<cv::detail::FeaturesMatcher>* features_matcher, Result_void* ocvrs_return) {
		try {
			instance->setFeaturesMatcher(*features_matcher);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// matchingMask() /usr/include/opencv2/stitching.hpp:217
	void cv_Stitcher_matchingMask_const(const cv::Stitcher* instance, Result<cv::UMat*>* ocvrs_return) {
		try {
			const cv::UMat ret = instance->matchingMask();
			Ok(new const cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	// setMatchingMask(const cv::UMat &) /usr/include/opencv2/stitching.hpp:218
	void cv_Stitcher_setMatchingMask_const_UMatR(cv::Stitcher* instance, const cv::UMat* mask, Result_void* ocvrs_return) {
		try {
			instance->setMatchingMask(*mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// bundleAdjuster() /usr/include/opencv2/stitching.hpp:224
	void cv_Stitcher_bundleAdjuster(cv::Stitcher* instance, Result<cv::Ptr<cv::detail::BundleAdjusterBase>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::BundleAdjusterBase> ret = instance->bundleAdjuster();
			Ok(new cv::Ptr<cv::detail::BundleAdjusterBase>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::BundleAdjusterBase>*>))
	}
	
	// bundleAdjuster() /usr/include/opencv2/stitching.hpp:225
	void cv_Stitcher_bundleAdjuster_const(const cv::Stitcher* instance, Result<cv::Ptr<cv::detail::BundleAdjusterBase>*>* ocvrs_return) {
		try {
			const cv::Ptr<cv::detail::BundleAdjusterBase> ret = instance->bundleAdjuster();
			Ok(new const cv::Ptr<cv::detail::BundleAdjusterBase>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::BundleAdjusterBase>*>))
	}
	
	// setBundleAdjuster(Ptr<detail::BundleAdjusterBase>) /usr/include/opencv2/stitching.hpp:226
	void cv_Stitcher_setBundleAdjuster_Ptr_BundleAdjusterBase_(cv::Stitcher* instance, cv::Ptr<cv::detail::BundleAdjusterBase>* bundle_adjuster, Result_void* ocvrs_return) {
		try {
			instance->setBundleAdjuster(*bundle_adjuster);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// estimator() /usr/include/opencv2/stitching.hpp:229
	void cv_Stitcher_estimator(cv::Stitcher* instance, Result<cv::Ptr<cv::detail::Estimator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::Estimator> ret = instance->estimator();
			Ok(new cv::Ptr<cv::detail::Estimator>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::Estimator>*>))
	}
	
	// estimator() /usr/include/opencv2/stitching.hpp:230
	void cv_Stitcher_estimator_const(const cv::Stitcher* instance, Result<cv::Ptr<cv::detail::Estimator>*>* ocvrs_return) {
		try {
			const cv::Ptr<cv::detail::Estimator> ret = instance->estimator();
			Ok(new const cv::Ptr<cv::detail::Estimator>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::Estimator>*>))
	}
	
	// setEstimator(Ptr<detail::Estimator>) /usr/include/opencv2/stitching.hpp:231
	void cv_Stitcher_setEstimator_Ptr_Estimator_(cv::Stitcher* instance, cv::Ptr<cv::detail::Estimator>* estimator, Result_void* ocvrs_return) {
		try {
			instance->setEstimator(*estimator);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// warper() /usr/include/opencv2/stitching.hpp:234
	void cv_Stitcher_warper(cv::Stitcher* instance, Result<cv::Ptr<cv::WarperCreator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::WarperCreator> ret = instance->warper();
			Ok(new cv::Ptr<cv::WarperCreator>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::WarperCreator>*>))
	}
	
	// warper() /usr/include/opencv2/stitching.hpp:235
	void cv_Stitcher_warper_const(const cv::Stitcher* instance, Result<cv::Ptr<cv::WarperCreator>*>* ocvrs_return) {
		try {
			const cv::Ptr<cv::WarperCreator> ret = instance->warper();
			Ok(new const cv::Ptr<cv::WarperCreator>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::WarperCreator>*>))
	}
	
	// setWarper(Ptr<cv::WarperCreator>) /usr/include/opencv2/stitching.hpp:236
	void cv_Stitcher_setWarper_Ptr_WarperCreator_(cv::Stitcher* instance, cv::Ptr<cv::WarperCreator>* creator, Result_void* ocvrs_return) {
		try {
			instance->setWarper(*creator);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// exposureCompensator() /usr/include/opencv2/stitching.hpp:238
	void cv_Stitcher_exposureCompensator(cv::Stitcher* instance, Result<cv::Ptr<cv::detail::ExposureCompensator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::ExposureCompensator> ret = instance->exposureCompensator();
			Ok(new cv::Ptr<cv::detail::ExposureCompensator>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::ExposureCompensator>*>))
	}
	
	// exposureCompensator() /usr/include/opencv2/stitching.hpp:239
	void cv_Stitcher_exposureCompensator_const(const cv::Stitcher* instance, Result<cv::Ptr<cv::detail::ExposureCompensator>*>* ocvrs_return) {
		try {
			const cv::Ptr<cv::detail::ExposureCompensator> ret = instance->exposureCompensator();
			Ok(new const cv::Ptr<cv::detail::ExposureCompensator>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::ExposureCompensator>*>))
	}
	
	// setExposureCompensator(Ptr<detail::ExposureCompensator>) /usr/include/opencv2/stitching.hpp:240
	void cv_Stitcher_setExposureCompensator_Ptr_ExposureCompensator_(cv::Stitcher* instance, cv::Ptr<cv::detail::ExposureCompensator>* exposure_comp, Result_void* ocvrs_return) {
		try {
			instance->setExposureCompensator(*exposure_comp);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// seamFinder() /usr/include/opencv2/stitching.hpp:243
	void cv_Stitcher_seamFinder(cv::Stitcher* instance, Result<cv::Ptr<cv::detail::SeamFinder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::SeamFinder> ret = instance->seamFinder();
			Ok(new cv::Ptr<cv::detail::SeamFinder>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::SeamFinder>*>))
	}
	
	// seamFinder() /usr/include/opencv2/stitching.hpp:244
	void cv_Stitcher_seamFinder_const(const cv::Stitcher* instance, Result<cv::Ptr<cv::detail::SeamFinder>*>* ocvrs_return) {
		try {
			const cv::Ptr<cv::detail::SeamFinder> ret = instance->seamFinder();
			Ok(new const cv::Ptr<cv::detail::SeamFinder>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::SeamFinder>*>))
	}
	
	// setSeamFinder(Ptr<detail::SeamFinder>) /usr/include/opencv2/stitching.hpp:245
	void cv_Stitcher_setSeamFinder_Ptr_SeamFinder_(cv::Stitcher* instance, cv::Ptr<cv::detail::SeamFinder>* seam_finder, Result_void* ocvrs_return) {
		try {
			instance->setSeamFinder(*seam_finder);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// blender() /usr/include/opencv2/stitching.hpp:247
	void cv_Stitcher_blender(cv::Stitcher* instance, Result<cv::Ptr<cv::detail::Blender>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::Blender> ret = instance->blender();
			Ok(new cv::Ptr<cv::detail::Blender>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::Blender>*>))
	}
	
	// blender() /usr/include/opencv2/stitching.hpp:248
	void cv_Stitcher_blender_const(const cv::Stitcher* instance, Result<cv::Ptr<cv::detail::Blender>*>* ocvrs_return) {
		try {
			const cv::Ptr<cv::detail::Blender> ret = instance->blender();
			Ok(new const cv::Ptr<cv::detail::Blender>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::Blender>*>))
	}
	
	// setBlender(Ptr<detail::Blender>) /usr/include/opencv2/stitching.hpp:249
	void cv_Stitcher_setBlender_Ptr_Blender_(cv::Stitcher* instance, cv::Ptr<cv::detail::Blender>* b, Result_void* ocvrs_return) {
		try {
			instance->setBlender(*b);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// estimateTransform(cv::InputArrayOfArrays, cv::InputArrayOfArrays) /usr/include/opencv2/stitching.hpp:260
	void cv_Stitcher_estimateTransform_const__InputArrayR_const__InputArrayR(cv::Stitcher* instance, const cv::_InputArray* images, const cv::_InputArray* masks, Result<cv::Stitcher::Status>* ocvrs_return) {
		try {
			cv::Stitcher::Status ret = instance->estimateTransform(*images, *masks);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Stitcher::Status>))
	}
	
	// setTransform(cv::InputArrayOfArrays, const std::vector<detail::CameraParams> &, const std::vector<int> &) /usr/include/opencv2/stitching.hpp:270
	void cv_Stitcher_setTransform_const__InputArrayR_const_vector_CameraParams_R_const_vector_int_R(cv::Stitcher* instance, const cv::_InputArray* images, const std::vector<cv::detail::CameraParams>* cameras, const std::vector<int>* component, Result<cv::Stitcher::Status>* ocvrs_return) {
		try {
			cv::Stitcher::Status ret = instance->setTransform(*images, *cameras, *component);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Stitcher::Status>))
	}
	
	// setTransform(cv::InputArrayOfArrays, const std::vector<detail::CameraParams> &) /usr/include/opencv2/stitching.hpp:274
	void cv_Stitcher_setTransform_const__InputArrayR_const_vector_CameraParams_R(cv::Stitcher* instance, const cv::_InputArray* images, const std::vector<cv::detail::CameraParams>* cameras, Result<cv::Stitcher::Status>* ocvrs_return) {
		try {
			cv::Stitcher::Status ret = instance->setTransform(*images, *cameras);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Stitcher::Status>))
	}
	
	// composePanorama(cv::OutputArray) /usr/include/opencv2/stitching.hpp:277
	void cv_Stitcher_composePanorama_const__OutputArrayR(cv::Stitcher* instance, const cv::_OutputArray* pano, Result<cv::Stitcher::Status>* ocvrs_return) {
		try {
			cv::Stitcher::Status ret = instance->composePanorama(*pano);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Stitcher::Status>))
	}
	
	// composePanorama(cv::InputArrayOfArrays, cv::OutputArray) /usr/include/opencv2/stitching.hpp:289
	void cv_Stitcher_composePanorama_const__InputArrayR_const__OutputArrayR(cv::Stitcher* instance, const cv::_InputArray* images, const cv::_OutputArray* pano, Result<cv::Stitcher::Status>* ocvrs_return) {
		try {
			cv::Stitcher::Status ret = instance->composePanorama(*images, *pano);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Stitcher::Status>))
	}
	
	// stitch(cv::InputArrayOfArrays, cv::OutputArray) /usr/include/opencv2/stitching.hpp:292
	void cv_Stitcher_stitch_const__InputArrayR_const__OutputArrayR(cv::Stitcher* instance, const cv::_InputArray* images, const cv::_OutputArray* pano, Result<cv::Stitcher::Status>* ocvrs_return) {
		try {
			cv::Stitcher::Status ret = instance->stitch(*images, *pano);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Stitcher::Status>))
	}
	
	// stitch(cv::InputArrayOfArrays, cv::InputArrayOfArrays, cv::OutputArray) /usr/include/opencv2/stitching.hpp:300
	void cv_Stitcher_stitch_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::Stitcher* instance, const cv::_InputArray* images, const cv::_InputArray* masks, const cv::_OutputArray* pano, Result<cv::Stitcher::Status>* ocvrs_return) {
		try {
			cv::Stitcher::Status ret = instance->stitch(*images, *masks, *pano);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Stitcher::Status>))
	}
	
	// component() /usr/include/opencv2/stitching.hpp:302
	void cv_Stitcher_component_const(const cv::Stitcher* instance, Result<std::vector<int>*>* ocvrs_return) {
		try {
			std::vector<int> ret = instance->component();
			Ok(new std::vector<int>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<int>*>))
	}
	
	// cameras() /usr/include/opencv2/stitching.hpp:303
	void cv_Stitcher_cameras_const(const cv::Stitcher* instance, Result<std::vector<cv::detail::CameraParams>*>* ocvrs_return) {
		try {
			std::vector<cv::detail::CameraParams> ret = instance->cameras();
			Ok(new std::vector<cv::detail::CameraParams>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::detail::CameraParams>*>))
	}
	
	// workScale() /usr/include/opencv2/stitching.hpp:304
	void cv_Stitcher_workScale_const(const cv::Stitcher* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->workScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// resultMask() /usr/include/opencv2/stitching.hpp:305
	void cv_Stitcher_resultMask_const(const cv::Stitcher* instance, Result<cv::UMat*>* ocvrs_return) {
		try {
			cv::UMat ret = instance->resultMask();
			Ok(new cv::UMat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::UMat*>))
	}
	
	void cv_TransverseMercatorWarper_delete(cv::TransverseMercatorWarper* instance) {
		delete instance;
	}
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:246
	void cv_TransverseMercatorWarper_create_const_float(const cv::TransverseMercatorWarper* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	// create(float) /usr/include/opencv2/stitching/warpers.hpp:143
	void cv_WarperCreator_create_const_float(const cv::WarperCreator* instance, float scale, Result<cv::Ptr<cv::detail::RotationWarper>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::RotationWarper> ret = instance->create(scale);
			Ok(new cv::Ptr<cv::detail::RotationWarper>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::RotationWarper>*>))
	}
	
	void cv_Detail_AffineBasedEstimator_delete(cv::detail::AffineBasedEstimator* instance) {
		delete instance;
	}
	// AffineBasedEstimator() /usr/include/opencv2/stitching/detail/motion_estimators.hpp:124
	void cv_detail_AffineBasedEstimator_AffineBasedEstimator(Result<cv::detail::AffineBasedEstimator*>* ocvrs_return) {
		try {
			cv::detail::AffineBasedEstimator* ret = new cv::detail::AffineBasedEstimator();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::AffineBasedEstimator*>))
	}
	
	cv::detail::BestOf2NearestMatcher* cv_Detail_AffineBestOf2NearestMatcher_to_Detail_BestOf2NearestMatcher(cv::detail::AffineBestOf2NearestMatcher* instance) {
		return dynamic_cast<cv::detail::BestOf2NearestMatcher*>(instance);
	}
	
	void cv_Detail_AffineBestOf2NearestMatcher_delete(cv::detail::AffineBestOf2NearestMatcher* instance) {
		delete instance;
	}
	// AffineBestOf2NearestMatcher(bool, bool, float, int) /usr/include/opencv2/stitching/detail/matchers.hpp:237
	void cv_detail_AffineBestOf2NearestMatcher_AffineBestOf2NearestMatcher_bool_bool_float_int(bool full_affine, bool try_use_gpu, float match_conf, int num_matches_thresh1, Result<cv::detail::AffineBestOf2NearestMatcher*>* ocvrs_return) {
		try {
			cv::detail::AffineBestOf2NearestMatcher* ret = new cv::detail::AffineBestOf2NearestMatcher(full_affine, try_use_gpu, match_conf, num_matches_thresh1);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::AffineBestOf2NearestMatcher*>))
	}
	
	cv::detail::PlaneWarper* cv_Detail_AffineWarper_to_Detail_PlaneWarper(cv::detail::AffineWarper* instance) {
		return dynamic_cast<cv::detail::PlaneWarper*>(instance);
	}
	
	void cv_Detail_AffineWarper_delete(cv::detail::AffineWarper* instance) {
		delete instance;
	}
	// AffineWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:242
	void cv_detail_AffineWarper_AffineWarper_float(float scale, Result<cv::detail::AffineWarper*>* ocvrs_return) {
		try {
			cv::detail::AffineWarper* ret = new cv::detail::AffineWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::AffineWarper*>))
	}
	
	// warpPoint(const cv::Point2f &, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:251
	void cv_detail_AffineWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(cv::detail::AffineWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* H, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->warpPoint(*pt, *K, *H);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	// warpPointBackward(const cv::Point2f &, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:260
	void cv_detail_AffineWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR(cv::detail::AffineWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* H, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->warpPointBackward(*pt, *K, *H);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:271
	void cv_detail_AffineWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::AffineWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* H, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *H, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:283
	void cv_detail_AffineWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::AffineWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* H, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *H, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	// warpRoi(cv::Size, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:292
	void cv_detail_AffineWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(cv::detail::AffineWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* H, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->warpRoi(*src_size, *K, *H);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	cv::detail::AffineBestOf2NearestMatcher* cv_Detail_BestOf2NearestMatcher_to_Detail_AffineBestOf2NearestMatcher(cv::detail::BestOf2NearestMatcher* instance) {
		return dynamic_cast<cv::detail::AffineBestOf2NearestMatcher*>(instance);
	}
	
	cv::detail::BestOf2NearestRangeMatcher* cv_Detail_BestOf2NearestMatcher_to_Detail_BestOf2NearestRangeMatcher(cv::detail::BestOf2NearestMatcher* instance) {
		return dynamic_cast<cv::detail::BestOf2NearestRangeMatcher*>(instance);
	}
	
	void cv_Detail_BestOf2NearestMatcher_delete(cv::detail::BestOf2NearestMatcher* instance) {
		delete instance;
	}
	// BestOf2NearestMatcher(bool, float, int, int) /usr/include/opencv2/stitching/detail/matchers.hpp:184
	void cv_detail_BestOf2NearestMatcher_BestOf2NearestMatcher_bool_float_int_int(bool try_use_gpu, float match_conf, int num_matches_thresh1, int num_matches_thresh2, Result<cv::detail::BestOf2NearestMatcher*>* ocvrs_return) {
		try {
			cv::detail::BestOf2NearestMatcher* ret = new cv::detail::BestOf2NearestMatcher(try_use_gpu, match_conf, num_matches_thresh1, num_matches_thresh2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::BestOf2NearestMatcher*>))
	}
	
	// collectGarbage() /usr/include/opencv2/stitching/detail/matchers.hpp:187
	void cv_detail_BestOf2NearestMatcher_collectGarbage(cv::detail::BestOf2NearestMatcher* instance, Result_void* ocvrs_return) {
		try {
			instance->collectGarbage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// create(bool, float, int, int) /usr/include/opencv2/stitching/detail/matchers.hpp:188
	void cv_detail_BestOf2NearestMatcher_create_bool_float_int_int(bool try_use_gpu, float match_conf, int num_matches_thresh1, int num_matches_thresh2, Result<cv::Ptr<cv::detail::BestOf2NearestMatcher>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::BestOf2NearestMatcher> ret = cv::detail::BestOf2NearestMatcher::create(try_use_gpu, match_conf, num_matches_thresh1, num_matches_thresh2);
			Ok(new cv::Ptr<cv::detail::BestOf2NearestMatcher>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::BestOf2NearestMatcher>*>))
	}
	
	cv::detail::BestOf2NearestMatcher* cv_Detail_BestOf2NearestRangeMatcher_to_Detail_BestOf2NearestMatcher(cv::detail::BestOf2NearestRangeMatcher* instance) {
		return dynamic_cast<cv::detail::BestOf2NearestMatcher*>(instance);
	}
	
	void cv_Detail_BestOf2NearestRangeMatcher_delete(cv::detail::BestOf2NearestRangeMatcher* instance) {
		delete instance;
	}
	// BestOf2NearestRangeMatcher(int, bool, float, int, int) /usr/include/opencv2/stitching/detail/matchers.hpp:202
	void cv_detail_BestOf2NearestRangeMatcher_BestOf2NearestRangeMatcher_int_bool_float_int_int(int range_width, bool try_use_gpu, float match_conf, int num_matches_thresh1, int num_matches_thresh2, Result<cv::detail::BestOf2NearestRangeMatcher*>* ocvrs_return) {
		try {
			cv::detail::BestOf2NearestRangeMatcher* ret = new cv::detail::BestOf2NearestRangeMatcher(range_width, try_use_gpu, match_conf, num_matches_thresh1, num_matches_thresh2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::BestOf2NearestRangeMatcher*>))
	}
	
	cv::detail::FeatherBlender* cv_Detail_Blender_to_Detail_FeatherBlender(cv::detail::Blender* instance) {
		return dynamic_cast<cv::detail::FeatherBlender*>(instance);
	}
	
	cv::detail::MultiBandBlender* cv_Detail_Blender_to_Detail_MultiBandBlender(cv::detail::Blender* instance) {
		return dynamic_cast<cv::detail::MultiBandBlender*>(instance);
	}
	
	void cv_Detail_Blender_delete(cv::detail::Blender* instance) {
		delete instance;
	}
	// createDefault(int, bool) /usr/include/opencv2/stitching/detail/blenders.hpp:69
	void cv_detail_Blender_createDefault_int_bool(int type, bool try_gpu, Result<cv::Ptr<cv::detail::Blender>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::Blender> ret = cv::detail::Blender::createDefault(type, try_gpu);
			Ok(new cv::Ptr<cv::detail::Blender>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::Blender>*>))
	}
	
	// prepare(const std::vector<Point> &, const std::vector<Size> &) /usr/include/opencv2/stitching/detail/blenders.hpp:76
	void cv_detail_Blender_prepare_const_vector_Point_R_const_vector_Size_R(cv::detail::Blender* instance, const std::vector<cv::Point>* corners, const std::vector<cv::Size>* sizes, Result_void* ocvrs_return) {
		try {
			instance->prepare(*corners, *sizes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// prepare(cv::Rect) /usr/include/opencv2/stitching/detail/blenders.hpp:78
	void cv_detail_Blender_prepare_Rect(cv::detail::Blender* instance, cv::Rect* dst_roi, Result_void* ocvrs_return) {
		try {
			instance->prepare(*dst_roi);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// feed(cv::InputArray, cv::InputArray, cv::Point) /usr/include/opencv2/stitching/detail/blenders.hpp:85
	void cv_detail_Blender_feed_const__InputArrayR_const__InputArrayR_Point(cv::detail::Blender* instance, const cv::_InputArray* img, const cv::_InputArray* mask, cv::Point* tl, Result_void* ocvrs_return) {
		try {
			instance->feed(*img, *mask, *tl);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// blend(cv::InputOutputArray, cv::InputOutputArray) /usr/include/opencv2/stitching/detail/blenders.hpp:91
	void cv_detail_Blender_blend_const__InputOutputArrayR_const__InputOutputArrayR(cv::detail::Blender* instance, const cv::_InputOutputArray* dst, const cv::_InputOutputArray* dst_mask, Result_void* ocvrs_return) {
		try {
			instance->blend(*dst, *dst_mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_BlocksChannelsCompensator_delete(cv::detail::BlocksChannelsCompensator* instance) {
		delete instance;
	}
	// BlocksChannelsCompensator(int, int, int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:234
	void cv_detail_BlocksChannelsCompensator_BlocksChannelsCompensator_int_int_int(int bl_width, int bl_height, int nr_feeds, Result<cv::detail::BlocksChannelsCompensator*>* ocvrs_return) {
		try {
			cv::detail::BlocksChannelsCompensator* ret = new cv::detail::BlocksChannelsCompensator(bl_width, bl_height, nr_feeds);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::BlocksChannelsCompensator*>))
	}
	
	// apply(int, cv::Point, cv::InputOutputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:175
	void cv_detail_BlocksCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(cv::detail::BlocksCompensator* instance, int index, cv::Point* corner, const cv::_InputOutputArray* image, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			instance->apply(index, *corner, *image, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:176
	void cv_detail_BlocksCompensator_getMatGains_vector_Mat_R(cv::detail::BlocksCompensator* instance, std::vector<cv::Mat>* umv, Result_void* ocvrs_return) {
		try {
			instance->getMatGains(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:177
	void cv_detail_BlocksCompensator_setMatGains_vector_Mat_R(cv::detail::BlocksCompensator* instance, std::vector<cv::Mat>* umv, Result_void* ocvrs_return) {
		try {
			instance->setMatGains(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setNrFeeds(int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:178
	void cv_detail_BlocksCompensator_setNrFeeds_int(cv::detail::BlocksCompensator* instance, int nr_feeds, Result_void* ocvrs_return) {
		try {
			instance->setNrFeeds(nr_feeds);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNrFeeds() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:179
	void cv_detail_BlocksCompensator_getNrFeeds(cv::detail::BlocksCompensator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNrFeeds();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setSimilarityThreshold(double) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:180
	void cv_detail_BlocksCompensator_setSimilarityThreshold_double(cv::detail::BlocksCompensator* instance, double similarity_threshold, Result_void* ocvrs_return) {
		try {
			instance->setSimilarityThreshold(similarity_threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSimilarityThreshold() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:181
	void cv_detail_BlocksCompensator_getSimilarityThreshold_const(const cv::detail::BlocksCompensator* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSimilarityThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setBlockSize(int, int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:182
	void cv_detail_BlocksCompensator_setBlockSize_int_int(cv::detail::BlocksCompensator* instance, int width, int height, Result_void* ocvrs_return) {
		try {
			instance->setBlockSize(width, height);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setBlockSize(cv::Size) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:183
	void cv_detail_BlocksCompensator_setBlockSize_Size(cv::detail::BlocksCompensator* instance, cv::Size* size, Result_void* ocvrs_return) {
		try {
			instance->setBlockSize(*size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getBlockSize() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:184
	void cv_detail_BlocksCompensator_getBlockSize_const(const cv::detail::BlocksCompensator* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getBlockSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// setNrGainsFilteringIterations(int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:185
	void cv_detail_BlocksCompensator_setNrGainsFilteringIterations_int(cv::detail::BlocksCompensator* instance, int nr_iterations, Result_void* ocvrs_return) {
		try {
			instance->setNrGainsFilteringIterations(nr_iterations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNrGainsFilteringIterations() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:186
	void cv_detail_BlocksCompensator_getNrGainsFilteringIterations_const(const cv::detail::BlocksCompensator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNrGainsFilteringIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	void cv_Detail_BlocksGainCompensator_delete(cv::detail::BlocksGainCompensator* instance) {
		delete instance;
	}
	// BlocksGainCompensator(int, int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:211
	void cv_detail_BlocksGainCompensator_BlocksGainCompensator_int_int(int bl_width, int bl_height, Result<cv::detail::BlocksGainCompensator*>* ocvrs_return) {
		try {
			cv::detail::BlocksGainCompensator* ret = new cv::detail::BlocksGainCompensator(bl_width, bl_height);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::BlocksGainCompensator*>))
	}
	
	// BlocksGainCompensator(int, int, int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:213
	void cv_detail_BlocksGainCompensator_BlocksGainCompensator_int_int_int(int bl_width, int bl_height, int nr_feeds, Result<cv::detail::BlocksGainCompensator*>* ocvrs_return) {
		try {
			cv::detail::BlocksGainCompensator* ret = new cv::detail::BlocksGainCompensator(bl_width, bl_height, nr_feeds);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::BlocksGainCompensator*>))
	}
	
	// apply(int, cv::Point, cv::InputOutputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:220
	void cv_detail_BlocksGainCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(cv::detail::BlocksGainCompensator* instance, int index, cv::Point* corner, const cv::_InputOutputArray* image, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			instance->apply(index, *corner, *image, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:223
	void cv_detail_BlocksGainCompensator_getMatGains_vector_Mat_R(cv::detail::BlocksGainCompensator* instance, std::vector<cv::Mat>* umv, Result_void* ocvrs_return) {
		try {
			instance->getMatGains(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:225
	void cv_detail_BlocksGainCompensator_setMatGains_vector_Mat_R(cv::detail::BlocksGainCompensator* instance, std::vector<cv::Mat>* umv, Result_void* ocvrs_return) {
		try {
			instance->setMatGains(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_BundleAdjusterAffine_delete(cv::detail::BundleAdjusterAffine* instance) {
		delete instance;
	}
	// BundleAdjusterAffine() /usr/include/opencv2/stitching/detail/motion_estimators.hpp:292
	void cv_detail_BundleAdjusterAffine_BundleAdjusterAffine(Result<cv::detail::BundleAdjusterAffine*>* ocvrs_return) {
		try {
			cv::detail::BundleAdjusterAffine* ret = new cv::detail::BundleAdjusterAffine();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::BundleAdjusterAffine*>))
	}
	
	void cv_Detail_BundleAdjusterAffinePartial_delete(cv::detail::BundleAdjusterAffinePartial* instance) {
		delete instance;
	}
	// BundleAdjusterAffinePartial() /usr/include/opencv2/stitching/detail/motion_estimators.hpp:316
	void cv_detail_BundleAdjusterAffinePartial_BundleAdjusterAffinePartial(Result<cv::detail::BundleAdjusterAffinePartial*>* ocvrs_return) {
		try {
			cv::detail::BundleAdjusterAffinePartial* ret = new cv::detail::BundleAdjusterAffinePartial();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::BundleAdjusterAffinePartial*>))
	}
	
	// refinementMask() /usr/include/opencv2/stitching/detail/motion_estimators.hpp:136
	void cv_detail_BundleAdjusterBase_refinementMask_const(const cv::detail::BundleAdjusterBase* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->refinementMask();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// setRefinementMask(const cv::Mat &) /usr/include/opencv2/stitching/detail/motion_estimators.hpp:137
	void cv_detail_BundleAdjusterBase_setRefinementMask_const_MatR(cv::detail::BundleAdjusterBase* instance, const cv::Mat* mask, Result_void* ocvrs_return) {
		try {
			instance->setRefinementMask(*mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// confThresh() /usr/include/opencv2/stitching/detail/motion_estimators.hpp:143
	void cv_detail_BundleAdjusterBase_confThresh_const(const cv::detail::BundleAdjusterBase* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->confThresh();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setConfThresh(double) /usr/include/opencv2/stitching/detail/motion_estimators.hpp:144
	void cv_detail_BundleAdjusterBase_setConfThresh_double(cv::detail::BundleAdjusterBase* instance, double conf_thresh, Result_void* ocvrs_return) {
		try {
			instance->setConfThresh(conf_thresh);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// termCriteria() /usr/include/opencv2/stitching/detail/motion_estimators.hpp:146
	void cv_detail_BundleAdjusterBase_termCriteria(cv::detail::BundleAdjusterBase* instance, Result<cv::TermCriteria>* ocvrs_return) {
		try {
			cv::TermCriteria ret = instance->termCriteria();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::TermCriteria>))
	}
	
	// setTermCriteria(const cv::TermCriteria &) /usr/include/opencv2/stitching/detail/motion_estimators.hpp:147
	void cv_detail_BundleAdjusterBase_setTermCriteria_const_TermCriteriaR(cv::detail::BundleAdjusterBase* instance, const cv::TermCriteria* term_criteria, Result_void* ocvrs_return) {
		try {
			instance->setTermCriteria(*term_criteria);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_BundleAdjusterRay_delete(cv::detail::BundleAdjusterRay* instance) {
		delete instance;
	}
	// BundleAdjusterRay() /usr/include/opencv2/stitching/detail/motion_estimators.hpp:268
	void cv_detail_BundleAdjusterRay_BundleAdjusterRay(Result<cv::detail::BundleAdjusterRay*>* ocvrs_return) {
		try {
			cv::detail::BundleAdjusterRay* ret = new cv::detail::BundleAdjusterRay();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::BundleAdjusterRay*>))
	}
	
	void cv_Detail_BundleAdjusterReproj_delete(cv::detail::BundleAdjusterReproj* instance) {
		delete instance;
	}
	// BundleAdjusterReproj() /usr/include/opencv2/stitching/detail/motion_estimators.hpp:248
	void cv_detail_BundleAdjusterReproj_BundleAdjusterReproj(Result<cv::detail::BundleAdjusterReproj*>* ocvrs_return) {
		try {
			cv::detail::BundleAdjusterReproj* ret = new cv::detail::BundleAdjusterReproj();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::BundleAdjusterReproj*>))
	}
	
	// focal /usr/include/opencv2/stitching/detail/camera.hpp:65
	double cv_detail_CameraParams_getPropFocal_const(const cv::detail::CameraParams* instance) {
			double ret = instance->focal;
			return ret;
	}
	
	// focal /usr/include/opencv2/stitching/detail/camera.hpp:65
	void cv_detail_CameraParams_setPropFocal_double(cv::detail::CameraParams* instance, double val) {
			instance->focal = val;
	}
	
	// aspect /usr/include/opencv2/stitching/detail/camera.hpp:66
	double cv_detail_CameraParams_getPropAspect_const(const cv::detail::CameraParams* instance) {
			double ret = instance->aspect;
			return ret;
	}
	
	// aspect /usr/include/opencv2/stitching/detail/camera.hpp:66
	void cv_detail_CameraParams_setPropAspect_double(cv::detail::CameraParams* instance, double val) {
			instance->aspect = val;
	}
	
	// ppx /usr/include/opencv2/stitching/detail/camera.hpp:67
	double cv_detail_CameraParams_getPropPpx_const(const cv::detail::CameraParams* instance) {
			double ret = instance->ppx;
			return ret;
	}
	
	// ppx /usr/include/opencv2/stitching/detail/camera.hpp:67
	void cv_detail_CameraParams_setPropPpx_double(cv::detail::CameraParams* instance, double val) {
			instance->ppx = val;
	}
	
	// ppy /usr/include/opencv2/stitching/detail/camera.hpp:68
	double cv_detail_CameraParams_getPropPpy_const(const cv::detail::CameraParams* instance) {
			double ret = instance->ppy;
			return ret;
	}
	
	// ppy /usr/include/opencv2/stitching/detail/camera.hpp:68
	void cv_detail_CameraParams_setPropPpy_double(cv::detail::CameraParams* instance, double val) {
			instance->ppy = val;
	}
	
	// R /usr/include/opencv2/stitching/detail/camera.hpp:69
	cv::Mat* cv_detail_CameraParams_getPropR_const(const cv::detail::CameraParams* instance) {
			cv::Mat ret = instance->R;
			return new cv::Mat(ret);
	}
	
	// R /usr/include/opencv2/stitching/detail/camera.hpp:69
	void cv_detail_CameraParams_setPropR_Mat(cv::detail::CameraParams* instance, cv::Mat* val) {
			instance->R = *val;
	}
	
	// t /usr/include/opencv2/stitching/detail/camera.hpp:70
	cv::Mat* cv_detail_CameraParams_getPropT_const(const cv::detail::CameraParams* instance) {
			cv::Mat ret = instance->t;
			return new cv::Mat(ret);
	}
	
	// t /usr/include/opencv2/stitching/detail/camera.hpp:70
	void cv_detail_CameraParams_setPropT_Mat(cv::detail::CameraParams* instance, cv::Mat* val) {
			instance->t = *val;
	}
	
	void cv_Detail_CameraParams_delete(cv::detail::CameraParams* instance) {
		delete instance;
	}
	// CameraParams() /usr/include/opencv2/stitching/detail/camera.hpp:60
	void cv_detail_CameraParams_CameraParams(Result<cv::detail::CameraParams*>* ocvrs_return) {
		try {
			cv::detail::CameraParams* ret = new cv::detail::CameraParams();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::CameraParams*>))
	}
	
	// CameraParams(const cv::detail::CameraParams &) /usr/include/opencv2/stitching/detail/camera.hpp:61
	void cv_detail_CameraParams_CameraParams_const_CameraParamsR(const cv::detail::CameraParams* other, Result<cv::detail::CameraParams*>* ocvrs_return) {
		try {
			cv::detail::CameraParams* ret = new cv::detail::CameraParams(*other);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::CameraParams*>))
	}
	
	// K() /usr/include/opencv2/stitching/detail/camera.hpp:63
	void cv_detail_CameraParams_K_const(const cv::detail::CameraParams* instance, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->K();
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	void cv_Detail_ChannelsCompensator_delete(cv::detail::ChannelsCompensator* instance) {
		delete instance;
	}
	// ChannelsCompensator(int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:148
	void cv_detail_ChannelsCompensator_ChannelsCompensator_int(int nr_feeds, Result<cv::detail::ChannelsCompensator*>* ocvrs_return) {
		try {
			cv::detail::ChannelsCompensator* ret = new cv::detail::ChannelsCompensator(nr_feeds);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::ChannelsCompensator*>))
	}
	
	// apply(int, cv::Point, cv::InputOutputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:152
	void cv_detail_ChannelsCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(cv::detail::ChannelsCompensator* instance, int index, cv::Point* corner, const cv::_InputOutputArray* image, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			instance->apply(index, *corner, *image, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:153
	void cv_detail_ChannelsCompensator_getMatGains_vector_Mat_R(cv::detail::ChannelsCompensator* instance, std::vector<cv::Mat>* umv, Result_void* ocvrs_return) {
		try {
			instance->getMatGains(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:154
	void cv_detail_ChannelsCompensator_setMatGains_vector_Mat_R(cv::detail::ChannelsCompensator* instance, std::vector<cv::Mat>* umv, Result_void* ocvrs_return) {
		try {
			instance->setMatGains(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setNrFeeds(int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:155
	void cv_detail_ChannelsCompensator_setNrFeeds_int(cv::detail::ChannelsCompensator* instance, int nr_feeds, Result_void* ocvrs_return) {
		try {
			instance->setNrFeeds(nr_feeds);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNrFeeds() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:156
	void cv_detail_ChannelsCompensator_getNrFeeds(cv::detail::ChannelsCompensator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNrFeeds();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setSimilarityThreshold(double) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:157
	void cv_detail_ChannelsCompensator_setSimilarityThreshold_double(cv::detail::ChannelsCompensator* instance, double similarity_threshold, Result_void* ocvrs_return) {
		try {
			instance->setSimilarityThreshold(similarity_threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSimilarityThreshold() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:158
	void cv_detail_ChannelsCompensator_getSimilarityThreshold_const(const cv::detail::ChannelsCompensator* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSimilarityThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// gains() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:159
	void cv_detail_ChannelsCompensator_gains_const(const cv::detail::ChannelsCompensator* instance, Result<std::vector<cv::Scalar>*>* ocvrs_return) {
		try {
			std::vector<cv::Scalar> ret = instance->gains();
			Ok(new std::vector<cv::Scalar>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::Scalar>*>))
	}
	
	// a /usr/include/opencv2/stitching/detail/warpers.hpp:411
	float cv_detail_CompressedRectilinearPortraitProjector_getPropA_const(const cv::detail::CompressedRectilinearPortraitProjector* instance) {
			float ret = instance->a;
			return ret;
	}
	
	// a /usr/include/opencv2/stitching/detail/warpers.hpp:411
	void cv_detail_CompressedRectilinearPortraitProjector_setPropA_float(cv::detail::CompressedRectilinearPortraitProjector* instance, float val) {
			instance->a = val;
	}
	
	// b /usr/include/opencv2/stitching/detail/warpers.hpp:411
	float cv_detail_CompressedRectilinearPortraitProjector_getPropB_const(const cv::detail::CompressedRectilinearPortraitProjector* instance) {
			float ret = instance->b;
			return ret;
	}
	
	// b /usr/include/opencv2/stitching/detail/warpers.hpp:411
	void cv_detail_CompressedRectilinearPortraitProjector_setPropB_float(cv::detail::CompressedRectilinearPortraitProjector* instance, float val) {
			instance->b = val;
	}
	
	cv::detail::ProjectorBase* cv_Detail_CompressedRectilinearPortraitProjector_to_Detail_ProjectorBase(cv::detail::CompressedRectilinearPortraitProjector* instance) {
		return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}
	
	void cv_Detail_CompressedRectilinearPortraitProjector_delete(cv::detail::CompressedRectilinearPortraitProjector* instance) {
		delete instance;
	}
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:413
	void cv_detail_CompressedRectilinearPortraitProjector_mapForward_float_float_floatR_floatR(cv::detail::CompressedRectilinearPortraitProjector* instance, float x, float y, float* u, float* v, Result_void* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:414
	void cv_detail_CompressedRectilinearPortraitProjector_mapBackward_float_float_floatR_floatR(cv::detail::CompressedRectilinearPortraitProjector* instance, float u, float v, float* x, float* y, Result_void* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_CompressedRectilinearPortraitWarper_delete(cv::detail::CompressedRectilinearPortraitWarper* instance) {
		delete instance;
	}
	// CompressedRectilinearPortraitWarper(float, float, float) /usr/include/opencv2/stitching/detail/warpers.hpp:421
	void cv_detail_CompressedRectilinearPortraitWarper_CompressedRectilinearPortraitWarper_float_float_float(float scale, float A, float B, Result<cv::detail::CompressedRectilinearPortraitWarper*>* ocvrs_return) {
		try {
			cv::detail::CompressedRectilinearPortraitWarper* ret = new cv::detail::CompressedRectilinearPortraitWarper(scale, A, B);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::CompressedRectilinearPortraitWarper*>))
	}
	
	// a /usr/include/opencv2/stitching/detail/warpers.hpp:390
	float cv_detail_CompressedRectilinearProjector_getPropA_const(const cv::detail::CompressedRectilinearProjector* instance) {
			float ret = instance->a;
			return ret;
	}
	
	// a /usr/include/opencv2/stitching/detail/warpers.hpp:390
	void cv_detail_CompressedRectilinearProjector_setPropA_float(cv::detail::CompressedRectilinearProjector* instance, float val) {
			instance->a = val;
	}
	
	// b /usr/include/opencv2/stitching/detail/warpers.hpp:390
	float cv_detail_CompressedRectilinearProjector_getPropB_const(const cv::detail::CompressedRectilinearProjector* instance) {
			float ret = instance->b;
			return ret;
	}
	
	// b /usr/include/opencv2/stitching/detail/warpers.hpp:390
	void cv_detail_CompressedRectilinearProjector_setPropB_float(cv::detail::CompressedRectilinearProjector* instance, float val) {
			instance->b = val;
	}
	
	cv::detail::ProjectorBase* cv_Detail_CompressedRectilinearProjector_to_Detail_ProjectorBase(cv::detail::CompressedRectilinearProjector* instance) {
		return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}
	
	void cv_Detail_CompressedRectilinearProjector_delete(cv::detail::CompressedRectilinearProjector* instance) {
		delete instance;
	}
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:392
	void cv_detail_CompressedRectilinearProjector_mapForward_float_float_floatR_floatR(cv::detail::CompressedRectilinearProjector* instance, float x, float y, float* u, float* v, Result_void* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:393
	void cv_detail_CompressedRectilinearProjector_mapBackward_float_float_floatR_floatR(cv::detail::CompressedRectilinearProjector* instance, float u, float v, float* x, float* y, Result_void* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_CompressedRectilinearWarper_delete(cv::detail::CompressedRectilinearWarper* instance) {
		delete instance;
	}
	// CompressedRectilinearWarper(float, float, float) /usr/include/opencv2/stitching/detail/warpers.hpp:400
	void cv_detail_CompressedRectilinearWarper_CompressedRectilinearWarper_float_float_float(float scale, float A, float B, Result<cv::detail::CompressedRectilinearWarper*>* ocvrs_return) {
		try {
			cv::detail::CompressedRectilinearWarper* ret = new cv::detail::CompressedRectilinearWarper(scale, A, B);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::CompressedRectilinearWarper*>))
	}
	
	cv::detail::ProjectorBase* cv_Detail_CylindricalPortraitProjector_to_Detail_ProjectorBase(cv::detail::CylindricalPortraitProjector* instance) {
		return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}
	
	void cv_Detail_CylindricalPortraitProjector_delete(cv::detail::CylindricalPortraitProjector* instance) {
		delete instance;
	}
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:639
	void cv_detail_CylindricalPortraitProjector_mapForward_float_float_floatR_floatR(cv::detail::CylindricalPortraitProjector* instance, float x, float y, float* u, float* v, Result_void* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:640
	void cv_detail_CylindricalPortraitProjector_mapBackward_float_float_floatR_floatR(cv::detail::CylindricalPortraitProjector* instance, float u, float v, float* x, float* y, Result_void* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_CylindricalPortraitWarper_delete(cv::detail::CylindricalPortraitWarper* instance) {
		delete instance;
	}
	// CylindricalPortraitWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:647
	void cv_detail_CylindricalPortraitWarper_CylindricalPortraitWarper_float(float scale, Result<cv::detail::CylindricalPortraitWarper*>* ocvrs_return) {
		try {
			cv::detail::CylindricalPortraitWarper* ret = new cv::detail::CylindricalPortraitWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::CylindricalPortraitWarper*>))
	}
	
	cv::detail::ProjectorBase* cv_Detail_CylindricalProjector_to_Detail_ProjectorBase(cv::detail::CylindricalProjector* instance) {
		return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}
	
	void cv_Detail_CylindricalProjector_delete(cv::detail::CylindricalProjector* instance) {
		delete instance;
	}
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:334
	void cv_detail_CylindricalProjector_mapForward_float_float_floatR_floatR(cv::detail::CylindricalProjector* instance, float x, float y, float* u, float* v, Result_void* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:335
	void cv_detail_CylindricalProjector_mapBackward_float_float_floatR_floatR(cv::detail::CylindricalProjector* instance, float u, float v, float* x, float* y, Result_void* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::detail::CylindricalWarperGpu* cv_Detail_CylindricalWarper_to_Detail_CylindricalWarperGpu(cv::detail::CylindricalWarper* instance) {
		return dynamic_cast<cv::detail::CylindricalWarperGpu*>(instance);
	}
	
	void cv_Detail_CylindricalWarper_delete(cv::detail::CylindricalWarper* instance) {
		delete instance;
	}
	// CylindricalWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:348
	void cv_detail_CylindricalWarper_CylindricalWarper_float(float scale, Result<cv::detail::CylindricalWarper*>* ocvrs_return) {
		try {
			cv::detail::CylindricalWarper* ret = new cv::detail::CylindricalWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::CylindricalWarper*>))
	}
	
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:350
	void cv_detail_CylindricalWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::CylindricalWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:351
	void cv_detail_CylindricalWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::CylindricalWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	cv::detail::CylindricalWarper* cv_Detail_CylindricalWarperGpu_to_Detail_CylindricalWarper(cv::detail::CylindricalWarperGpu* instance) {
		return dynamic_cast<cv::detail::CylindricalWarper*>(instance);
	}
	
	void cv_Detail_CylindricalWarperGpu_delete(cv::detail::CylindricalWarperGpu* instance) {
		delete instance;
	}
	// CylindricalWarperGpu(float) /usr/include/opencv2/stitching/detail/warpers.hpp:590
	void cv_detail_CylindricalWarperGpu_CylindricalWarperGpu_float(float scale, Result<cv::detail::CylindricalWarperGpu*>* ocvrs_return) {
		try {
			cv::detail::CylindricalWarperGpu* ret = new cv::detail::CylindricalWarperGpu(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::CylindricalWarperGpu*>))
	}
	
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:592
	void cv_detail_CylindricalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::CylindricalWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:600
	void cv_detail_CylindricalWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::CylindricalWarperGpu* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cuda::GpuMat &, cuda::GpuMat &) /usr/include/opencv2/stitching/detail/warpers.hpp:609
	void cv_detail_CylindricalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(cv::detail::CylindricalWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, cv::cuda::GpuMat* xmap, cv::cuda::GpuMat* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// warp(const cuda::GpuMat &, cv::InputArray, cv::InputArray, int, int, cuda::GpuMat &) /usr/include/opencv2/stitching/detail/warpers.hpp:611
	void cv_detail_CylindricalWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(cv::detail::CylindricalWarperGpu* instance, const cv::cuda::GpuMat* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, cv::cuda::GpuMat* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	// parent /usr/include/opencv2/stitching/detail/util.hpp:64
	std::vector<int>* cv_detail_DisjointSets_getPropParent_const(const cv::detail::DisjointSets* instance) {
			std::vector<int> ret = instance->parent;
			return new std::vector<int>(ret);
	}
	
	// parent /usr/include/opencv2/stitching/detail/util.hpp:64
	void cv_detail_DisjointSets_setPropParent_vector_int_(cv::detail::DisjointSets* instance, std::vector<int>* val) {
			instance->parent = *val;
	}
	
	// size /usr/include/opencv2/stitching/detail/util.hpp:65
	std::vector<int>* cv_detail_DisjointSets_getPropSize_const(const cv::detail::DisjointSets* instance) {
			std::vector<int> ret = instance->size;
			return new std::vector<int>(ret);
	}
	
	// size /usr/include/opencv2/stitching/detail/util.hpp:65
	void cv_detail_DisjointSets_setPropSize_vector_int_(cv::detail::DisjointSets* instance, std::vector<int>* val) {
			instance->size = *val;
	}
	
	void cv_Detail_DisjointSets_delete(cv::detail::DisjointSets* instance) {
		delete instance;
	}
	// DisjointSets(int) /usr/include/opencv2/stitching/detail/util.hpp:58
	void cv_detail_DisjointSets_DisjointSets_int(int elem_count, Result<cv::detail::DisjointSets*>* ocvrs_return) {
		try {
			cv::detail::DisjointSets* ret = new cv::detail::DisjointSets(elem_count);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::DisjointSets*>))
	}
	
	// createOneElemSets(int) /usr/include/opencv2/stitching/detail/util.hpp:60
	void cv_detail_DisjointSets_createOneElemSets_int(cv::detail::DisjointSets* instance, int elem_count, Result_void* ocvrs_return) {
		try {
			instance->createOneElemSets(elem_count);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// findSetByElem(int) /usr/include/opencv2/stitching/detail/util.hpp:61
	void cv_detail_DisjointSets_findSetByElem_int(cv::detail::DisjointSets* instance, int elem, Result<int>* ocvrs_return) {
		try {
			int ret = instance->findSetByElem(elem);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// mergeSets(int, int) /usr/include/opencv2/stitching/detail/util.hpp:62
	void cv_detail_DisjointSets_mergeSets_int_int(cv::detail::DisjointSets* instance, int set1, int set2, Result<int>* ocvrs_return) {
		try {
			int ret = instance->mergeSets(set1, set2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	void cv_Detail_DpSeamFinder_delete(cv::detail::DpSeamFinder* instance) {
		delete instance;
	}
	// DpSeamFinder(cv::detail::DpSeamFinder::CostFunction) /usr/include/opencv2/stitching/detail/seam_finders.hpp:125
	void cv_detail_DpSeamFinder_DpSeamFinder_CostFunction(cv::detail::DpSeamFinder::CostFunction costFunc, Result<cv::detail::DpSeamFinder*>* ocvrs_return) {
		try {
			cv::detail::DpSeamFinder* ret = new cv::detail::DpSeamFinder(costFunc);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::DpSeamFinder*>))
	}
	
	// DpSeamFinder(cv::String) /usr/include/opencv2/stitching/detail/seam_finders.hpp:126
	void cv_detail_DpSeamFinder_DpSeamFinder_String(char* costFunc, Result<cv::detail::DpSeamFinder*>* ocvrs_return) {
		try {
			cv::detail::DpSeamFinder* ret = new cv::detail::DpSeamFinder(std::string(costFunc));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::DpSeamFinder*>))
	}
	
	// costFunction() /usr/include/opencv2/stitching/detail/seam_finders.hpp:128
	void cv_detail_DpSeamFinder_costFunction_const(const cv::detail::DpSeamFinder* instance, Result<cv::detail::DpSeamFinder::CostFunction>* ocvrs_return) {
		try {
			cv::detail::DpSeamFinder::CostFunction ret = instance->costFunction();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::DpSeamFinder::CostFunction>))
	}
	
	// setCostFunction(cv::detail::DpSeamFinder::CostFunction) /usr/include/opencv2/stitching/detail/seam_finders.hpp:129
	void cv_detail_DpSeamFinder_setCostFunction_CostFunction(cv::detail::DpSeamFinder* instance, cv::detail::DpSeamFinder::CostFunction val, Result_void* ocvrs_return) {
		try {
			instance->setCostFunction(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setCostFunction(cv::String) /usr/include/opencv2/stitching/detail/seam_finders.hpp:130
	void cv_detail_DpSeamFinder_setCostFunction_String(cv::detail::DpSeamFinder* instance, char* val, Result_void* ocvrs_return) {
		try {
			instance->setCostFunction(std::string(val));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &) /usr/include/opencv2/stitching/detail/seam_finders.hpp:132
	void cv_detail_DpSeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(cv::detail::DpSeamFinder* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks, Result_void* ocvrs_return) {
		try {
			instance->find(*src, *corners, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// createDefault(int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:67
	void cv_detail_ExposureCompensator_createDefault_int(int type, Result<cv::Ptr<cv::detail::ExposureCompensator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::ExposureCompensator> ret = cv::detail::ExposureCompensator::createDefault(type);
			Ok(new cv::Ptr<cv::detail::ExposureCompensator>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::ExposureCompensator>*>))
	}
	
	// feed(const std::vector<Point> &, const std::vector<UMat> &, const std::vector<UMat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:75
	void cv_detail_ExposureCompensator_feed_const_vector_Point_R_const_vector_UMat_R_const_vector_UMat_R(cv::detail::ExposureCompensator* instance, const std::vector<cv::Point>* corners, const std::vector<cv::UMat>* images, const std::vector<cv::UMat>* masks, Result_void* ocvrs_return) {
		try {
			instance->feed(*corners, *images, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// apply(int, cv::Point, cv::InputOutputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:87
	void cv_detail_ExposureCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(cv::detail::ExposureCompensator* instance, int index, cv::Point* corner, const cv::_InputOutputArray* image, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			instance->apply(index, *corner, *image, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:88
	void cv_detail_ExposureCompensator_getMatGains_vector_Mat_R(cv::detail::ExposureCompensator* instance, std::vector<cv::Mat>* unnamed, Result_void* ocvrs_return) {
		try {
			instance->getMatGains(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:89
	void cv_detail_ExposureCompensator_setMatGains_vector_Mat_R(cv::detail::ExposureCompensator* instance, std::vector<cv::Mat>* unnamed, Result_void* ocvrs_return) {
		try {
			instance->setMatGains(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setUpdateGain(bool) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:90
	void cv_detail_ExposureCompensator_setUpdateGain_bool(cv::detail::ExposureCompensator* instance, bool b, Result_void* ocvrs_return) {
		try {
			instance->setUpdateGain(b);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getUpdateGain() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:91
	void cv_detail_ExposureCompensator_getUpdateGain(cv::detail::ExposureCompensator* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getUpdateGain();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	cv::detail::Blender* cv_Detail_FeatherBlender_to_Detail_Blender(cv::detail::FeatherBlender* instance) {
		return dynamic_cast<cv::detail::Blender*>(instance);
	}
	
	void cv_Detail_FeatherBlender_delete(cv::detail::FeatherBlender* instance) {
		delete instance;
	}
	// FeatherBlender(float) /usr/include/opencv2/stitching/detail/blenders.hpp:103
	void cv_detail_FeatherBlender_FeatherBlender_float(float sharpness, Result<cv::detail::FeatherBlender*>* ocvrs_return) {
		try {
			cv::detail::FeatherBlender* ret = new cv::detail::FeatherBlender(sharpness);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::FeatherBlender*>))
	}
	
	// sharpness() /usr/include/opencv2/stitching/detail/blenders.hpp:105
	void cv_detail_FeatherBlender_sharpness_const(const cv::detail::FeatherBlender* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->sharpness();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setSharpness(float) /usr/include/opencv2/stitching/detail/blenders.hpp:106
	void cv_detail_FeatherBlender_setSharpness_float(cv::detail::FeatherBlender* instance, float val, Result_void* ocvrs_return) {
		try {
			instance->setSharpness(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// prepare(cv::Rect) /usr/include/opencv2/stitching/detail/blenders.hpp:108
	void cv_detail_FeatherBlender_prepare_Rect(cv::detail::FeatherBlender* instance, cv::Rect* dst_roi, Result_void* ocvrs_return) {
		try {
			instance->prepare(*dst_roi);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// feed(cv::InputArray, cv::InputArray, cv::Point) /usr/include/opencv2/stitching/detail/blenders.hpp:109
	void cv_detail_FeatherBlender_feed_const__InputArrayR_const__InputArrayR_Point(cv::detail::FeatherBlender* instance, const cv::_InputArray* img, const cv::_InputArray* mask, cv::Point* tl, Result_void* ocvrs_return) {
		try {
			instance->feed(*img, *mask, *tl);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// blend(cv::InputOutputArray, cv::InputOutputArray) /usr/include/opencv2/stitching/detail/blenders.hpp:110
	void cv_detail_FeatherBlender_blend_const__InputOutputArrayR_const__InputOutputArrayR(cv::detail::FeatherBlender* instance, const cv::_InputOutputArray* dst, const cv::_InputOutputArray* dst_mask, Result_void* ocvrs_return) {
		try {
			instance->blend(*dst, *dst_mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// createWeightMaps(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &) /usr/include/opencv2/stitching/detail/blenders.hpp:114
	void cv_detail_FeatherBlender_createWeightMaps_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(cv::detail::FeatherBlender* instance, const std::vector<cv::UMat>* masks, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* weight_maps, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->createWeightMaps(*masks, *corners, *weight_maps);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// isThreadSafe() /usr/include/opencv2/stitching/detail/matchers.hpp:145
	void cv_detail_FeaturesMatcher_isThreadSafe_const(const cv::detail::FeaturesMatcher* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isThreadSafe();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// collectGarbage() /usr/include/opencv2/stitching/detail/matchers.hpp:149
	void cv_detail_FeaturesMatcher_collectGarbage(cv::detail::FeaturesMatcher* instance, Result_void* ocvrs_return) {
		try {
			instance->collectGarbage();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::detail::ProjectorBase* cv_Detail_FisheyeProjector_to_Detail_ProjectorBase(cv::detail::FisheyeProjector* instance) {
		return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}
	
	void cv_Detail_FisheyeProjector_delete(cv::detail::FisheyeProjector* instance) {
		delete instance;
	}
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:362
	void cv_detail_FisheyeProjector_mapForward_float_float_floatR_floatR(cv::detail::FisheyeProjector* instance, float x, float y, float* u, float* v, Result_void* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:363
	void cv_detail_FisheyeProjector_mapBackward_float_float_floatR_floatR(cv::detail::FisheyeProjector* instance, float u, float v, float* x, float* y, Result_void* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_FisheyeWarper_delete(cv::detail::FisheyeWarper* instance) {
		delete instance;
	}
	// FisheyeWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:370
	void cv_detail_FisheyeWarper_FisheyeWarper_float(float scale, Result<cv::detail::FisheyeWarper*>* ocvrs_return) {
		try {
			cv::detail::FisheyeWarper* ret = new cv::detail::FisheyeWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::FisheyeWarper*>))
	}
	
	void cv_Detail_GainCompensator_delete(cv::detail::GainCompensator* instance) {
		delete instance;
	}
	// GainCompensator() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:115
	void cv_detail_GainCompensator_GainCompensator(Result<cv::detail::GainCompensator*>* ocvrs_return) {
		try {
			cv::detail::GainCompensator* ret = new cv::detail::GainCompensator();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::GainCompensator*>))
	}
	
	// GainCompensator(int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:117
	void cv_detail_GainCompensator_GainCompensator_int(int nr_feeds, Result<cv::detail::GainCompensator*>* ocvrs_return) {
		try {
			cv::detail::GainCompensator* ret = new cv::detail::GainCompensator(nr_feeds);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::GainCompensator*>))
	}
	
	// apply(int, cv::Point, cv::InputOutputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:123
	void cv_detail_GainCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(cv::detail::GainCompensator* instance, int index, cv::Point* corner, const cv::_InputOutputArray* image, const cv::_InputArray* mask, Result_void* ocvrs_return) {
		try {
			instance->apply(index, *corner, *image, *mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:124
	void cv_detail_GainCompensator_getMatGains_vector_Mat_R(cv::detail::GainCompensator* instance, std::vector<cv::Mat>* umv, Result_void* ocvrs_return) {
		try {
			instance->getMatGains(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:125
	void cv_detail_GainCompensator_setMatGains_vector_Mat_R(cv::detail::GainCompensator* instance, std::vector<cv::Mat>* umv, Result_void* ocvrs_return) {
		try {
			instance->setMatGains(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setNrFeeds(int) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:126
	void cv_detail_GainCompensator_setNrFeeds_int(cv::detail::GainCompensator* instance, int nr_feeds, Result_void* ocvrs_return) {
		try {
			instance->setNrFeeds(nr_feeds);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNrFeeds() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:127
	void cv_detail_GainCompensator_getNrFeeds(cv::detail::GainCompensator* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNrFeeds();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setSimilarityThreshold(double) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:128
	void cv_detail_GainCompensator_setSimilarityThreshold_double(cv::detail::GainCompensator* instance, double similarity_threshold, Result_void* ocvrs_return) {
		try {
			instance->setSimilarityThreshold(similarity_threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getSimilarityThreshold() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:129
	void cv_detail_GainCompensator_getSimilarityThreshold_const(const cv::detail::GainCompensator* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getSimilarityThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// prepareSimilarityMask(const std::vector<Point> &, const std::vector<UMat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:130
	void cv_detail_GainCompensator_prepareSimilarityMask_const_vector_Point_R_const_vector_UMat_R(cv::detail::GainCompensator* instance, const std::vector<cv::Point>* corners, const std::vector<cv::UMat>* images, Result_void* ocvrs_return) {
		try {
			instance->prepareSimilarityMask(*corners, *images);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// gains() /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:131
	void cv_detail_GainCompensator_gains_const(const cv::detail::GainCompensator* instance, Result<std::vector<double>*>* ocvrs_return) {
		try {
			std::vector<double> ret = instance->gains();
			Ok(new std::vector<double>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<double>*>))
	}
	
	void cv_Detail_Graph_delete(cv::detail::Graph* instance) {
		delete instance;
	}
	// Graph(int) /usr/include/opencv2/stitching/detail/util.hpp:88
	void cv_detail_Graph_Graph_int(int num_vertices, Result<cv::detail::Graph*>* ocvrs_return) {
		try {
			cv::detail::Graph* ret = new cv::detail::Graph(num_vertices);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::Graph*>))
	}
	
	// create(int) /usr/include/opencv2/stitching/detail/util.hpp:89
	void cv_detail_Graph_create_int(cv::detail::Graph* instance, int num_vertices, Result_void* ocvrs_return) {
		try {
			instance->create(num_vertices);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// numVertices() /usr/include/opencv2/stitching/detail/util.hpp:90
	void cv_detail_Graph_numVertices_const(const cv::detail::Graph* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->numVertices();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// addEdge(int, int, float) /usr/include/opencv2/stitching/detail/util.hpp:91
	void cv_detail_Graph_addEdge_int_int_float(cv::detail::Graph* instance, int from, int to, float weight, Result_void* ocvrs_return) {
		try {
			instance->addEdge(from, to, weight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::detail::GraphCutSeamFinderBase* cv_Detail_GraphCutSeamFinder_to_Detail_GraphCutSeamFinderBase(cv::detail::GraphCutSeamFinder* instance) {
		return dynamic_cast<cv::detail::GraphCutSeamFinderBase*>(instance);
	}
	
	void cv_Detail_GraphCutSeamFinder_delete(cv::detail::GraphCutSeamFinder* instance) {
		delete instance;
	}
	// GraphCutSeamFinder(int, float, float) /usr/include/opencv2/stitching/detail/seam_finders.hpp:243
	void cv_detail_GraphCutSeamFinder_GraphCutSeamFinder_int_float_float(int cost_type, float terminal_cost, float bad_region_penalty, Result<cv::detail::GraphCutSeamFinder*>* ocvrs_return) {
		try {
			cv::detail::GraphCutSeamFinder* ret = new cv::detail::GraphCutSeamFinder(cost_type, terminal_cost, bad_region_penalty);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::GraphCutSeamFinder*>))
	}
	
	// GraphCutSeamFinder(cv::String, float, float) /usr/include/opencv2/stitching/detail/seam_finders.hpp:245
	void cv_detail_GraphCutSeamFinder_GraphCutSeamFinder_String_float_float(char* cost_type, float terminal_cost, float bad_region_penalty, Result<cv::detail::GraphCutSeamFinder*>* ocvrs_return) {
		try {
			cv::detail::GraphCutSeamFinder* ret = new cv::detail::GraphCutSeamFinder(std::string(cost_type), terminal_cost, bad_region_penalty);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::GraphCutSeamFinder*>))
	}
	
	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &) /usr/include/opencv2/stitching/detail/seam_finders.hpp:250
	void cv_detail_GraphCutSeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(cv::detail::GraphCutSeamFinder* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks, Result_void* ocvrs_return) {
		try {
			instance->find(*src, *corners, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_GraphCutSeamFinderBase_delete(cv::detail::GraphCutSeamFinderBase* instance) {
		delete instance;
	}
	// from /usr/include/opencv2/stitching/detail/util.hpp:78
	int cv_detail_GraphEdge_getPropFrom_const(const cv::detail::GraphEdge* instance) {
			int ret = instance->from;
			return ret;
	}
	
	// from /usr/include/opencv2/stitching/detail/util.hpp:78
	void cv_detail_GraphEdge_setPropFrom_int(cv::detail::GraphEdge* instance, int val) {
			instance->from = val;
	}
	
	// to /usr/include/opencv2/stitching/detail/util.hpp:78
	int cv_detail_GraphEdge_getPropTo_const(const cv::detail::GraphEdge* instance) {
			int ret = instance->to;
			return ret;
	}
	
	// to /usr/include/opencv2/stitching/detail/util.hpp:78
	void cv_detail_GraphEdge_setPropTo_int(cv::detail::GraphEdge* instance, int val) {
			instance->to = val;
	}
	
	// weight /usr/include/opencv2/stitching/detail/util.hpp:79
	float cv_detail_GraphEdge_getPropWeight_const(const cv::detail::GraphEdge* instance) {
			float ret = instance->weight;
			return ret;
	}
	
	// weight /usr/include/opencv2/stitching/detail/util.hpp:79
	void cv_detail_GraphEdge_setPropWeight_float(cv::detail::GraphEdge* instance, float val) {
			instance->weight = val;
	}
	
	void cv_Detail_GraphEdge_delete(cv::detail::GraphEdge* instance) {
		delete instance;
	}
	// GraphEdge(int, int, float) /usr/include/opencv2/stitching/detail/util.hpp:74
	void cv_detail_GraphEdge_GraphEdge_int_int_float(int from, int to, float weight, Result<cv::detail::GraphEdge*>* ocvrs_return) {
		try {
			cv::detail::GraphEdge* ret = new cv::detail::GraphEdge(from, to, weight);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::GraphEdge*>))
	}
	
	void cv_Detail_HomographyBasedEstimator_delete(cv::detail::HomographyBasedEstimator* instance) {
		delete instance;
	}
	// HomographyBasedEstimator(bool) /usr/include/opencv2/stitching/detail/motion_estimators.hpp:103
	void cv_detail_HomographyBasedEstimator_HomographyBasedEstimator_bool(bool is_focals_estimated, Result<cv::detail::HomographyBasedEstimator*>* ocvrs_return) {
		try {
			cv::detail::HomographyBasedEstimator* ret = new cv::detail::HomographyBasedEstimator(is_focals_estimated);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::HomographyBasedEstimator*>))
	}
	
	// img_idx /usr/include/opencv2/stitching/detail/matchers.hpp:60
	int cv_detail_ImageFeatures_getPropImg_idx_const(const cv::detail::ImageFeatures* instance) {
			int ret = instance->img_idx;
			return ret;
	}
	
	// img_idx /usr/include/opencv2/stitching/detail/matchers.hpp:60
	void cv_detail_ImageFeatures_setPropImg_idx_int(cv::detail::ImageFeatures* instance, int val) {
			instance->img_idx = val;
	}
	
	// img_size /usr/include/opencv2/stitching/detail/matchers.hpp:61
	void cv_detail_ImageFeatures_getPropImg_size_const(const cv::detail::ImageFeatures* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->img_size;
			*ocvrs_return = ret;
	}
	
	// img_size /usr/include/opencv2/stitching/detail/matchers.hpp:61
	void cv_detail_ImageFeatures_setPropImg_size_Size(cv::detail::ImageFeatures* instance, cv::Size* val) {
			instance->img_size = *val;
	}
	
	// keypoints /usr/include/opencv2/stitching/detail/matchers.hpp:62
	std::vector<cv::KeyPoint>* cv_detail_ImageFeatures_getPropKeypoints_const(const cv::detail::ImageFeatures* instance) {
			std::vector<cv::KeyPoint> ret = instance->keypoints;
			return new std::vector<cv::KeyPoint>(ret);
	}
	
	// keypoints /usr/include/opencv2/stitching/detail/matchers.hpp:62
	void cv_detail_ImageFeatures_setPropKeypoints_vector_KeyPoint_(cv::detail::ImageFeatures* instance, std::vector<cv::KeyPoint>* val) {
			instance->keypoints = *val;
	}
	
	// descriptors /usr/include/opencv2/stitching/detail/matchers.hpp:63
	cv::UMat* cv_detail_ImageFeatures_getPropDescriptors_const(const cv::detail::ImageFeatures* instance) {
			cv::UMat ret = instance->descriptors;
			return new cv::UMat(ret);
	}
	
	// descriptors /usr/include/opencv2/stitching/detail/matchers.hpp:63
	void cv_detail_ImageFeatures_setPropDescriptors_UMat(cv::detail::ImageFeatures* instance, cv::UMat* val) {
			instance->descriptors = *val;
	}
	
	void cv_Detail_ImageFeatures_delete(cv::detail::ImageFeatures* instance) {
		delete instance;
	}
	// getKeypoints() /usr/include/opencv2/stitching/detail/matchers.hpp:64
	void cv_detail_ImageFeatures_getKeypoints(cv::detail::ImageFeatures* instance, Result<std::vector<cv::KeyPoint>*>* ocvrs_return) {
		try {
			std::vector<cv::KeyPoint> ret = instance->getKeypoints();
			Ok(new std::vector<cv::KeyPoint>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::KeyPoint>*>))
	}
	
	// src_img_idx /usr/include/opencv2/stitching/detail/matchers.hpp:105
	int cv_detail_MatchesInfo_getPropSrc_img_idx_const(const cv::detail::MatchesInfo* instance) {
			int ret = instance->src_img_idx;
			return ret;
	}
	
	// src_img_idx /usr/include/opencv2/stitching/detail/matchers.hpp:105
	void cv_detail_MatchesInfo_setPropSrc_img_idx_int(cv::detail::MatchesInfo* instance, int val) {
			instance->src_img_idx = val;
	}
	
	// dst_img_idx /usr/include/opencv2/stitching/detail/matchers.hpp:106
	int cv_detail_MatchesInfo_getPropDst_img_idx_const(const cv::detail::MatchesInfo* instance) {
			int ret = instance->dst_img_idx;
			return ret;
	}
	
	// dst_img_idx /usr/include/opencv2/stitching/detail/matchers.hpp:106
	void cv_detail_MatchesInfo_setPropDst_img_idx_int(cv::detail::MatchesInfo* instance, int val) {
			instance->dst_img_idx = val;
	}
	
	// matches /usr/include/opencv2/stitching/detail/matchers.hpp:107
	std::vector<cv::DMatch>* cv_detail_MatchesInfo_getPropMatches_const(const cv::detail::MatchesInfo* instance) {
			std::vector<cv::DMatch> ret = instance->matches;
			return new std::vector<cv::DMatch>(ret);
	}
	
	// matches /usr/include/opencv2/stitching/detail/matchers.hpp:107
	void cv_detail_MatchesInfo_setPropMatches_vector_DMatch_(cv::detail::MatchesInfo* instance, std::vector<cv::DMatch>* val) {
			instance->matches = *val;
	}
	
	// inliers_mask /usr/include/opencv2/stitching/detail/matchers.hpp:108
	std::vector<unsigned char>* cv_detail_MatchesInfo_getPropInliers_mask_const(const cv::detail::MatchesInfo* instance) {
			std::vector<unsigned char> ret = instance->inliers_mask;
			return new std::vector<unsigned char>(ret);
	}
	
	// inliers_mask /usr/include/opencv2/stitching/detail/matchers.hpp:108
	void cv_detail_MatchesInfo_setPropInliers_mask_vector_unsigned_char_(cv::detail::MatchesInfo* instance, std::vector<unsigned char>* val) {
			instance->inliers_mask = *val;
	}
	
	// num_inliers /usr/include/opencv2/stitching/detail/matchers.hpp:109
	int cv_detail_MatchesInfo_getPropNum_inliers_const(const cv::detail::MatchesInfo* instance) {
			int ret = instance->num_inliers;
			return ret;
	}
	
	// num_inliers /usr/include/opencv2/stitching/detail/matchers.hpp:109
	void cv_detail_MatchesInfo_setPropNum_inliers_int(cv::detail::MatchesInfo* instance, int val) {
			instance->num_inliers = val;
	}
	
	// H /usr/include/opencv2/stitching/detail/matchers.hpp:110
	cv::Mat* cv_detail_MatchesInfo_getPropH_const(const cv::detail::MatchesInfo* instance) {
			cv::Mat ret = instance->H;
			return new cv::Mat(ret);
	}
	
	// H /usr/include/opencv2/stitching/detail/matchers.hpp:110
	void cv_detail_MatchesInfo_setPropH_Mat(cv::detail::MatchesInfo* instance, cv::Mat* val) {
			instance->H = *val;
	}
	
	// confidence /usr/include/opencv2/stitching/detail/matchers.hpp:111
	double cv_detail_MatchesInfo_getPropConfidence_const(const cv::detail::MatchesInfo* instance) {
			double ret = instance->confidence;
			return ret;
	}
	
	// confidence /usr/include/opencv2/stitching/detail/matchers.hpp:111
	void cv_detail_MatchesInfo_setPropConfidence_double(cv::detail::MatchesInfo* instance, double val) {
			instance->confidence = val;
	}
	
	void cv_Detail_MatchesInfo_delete(cv::detail::MatchesInfo* instance) {
		delete instance;
	}
	// MatchesInfo() /usr/include/opencv2/stitching/detail/matchers.hpp:101
	void cv_detail_MatchesInfo_MatchesInfo(Result<cv::detail::MatchesInfo*>* ocvrs_return) {
		try {
			cv::detail::MatchesInfo* ret = new cv::detail::MatchesInfo();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::MatchesInfo*>))
	}
	
	// MatchesInfo(const cv::detail::MatchesInfo &) /usr/include/opencv2/stitching/detail/matchers.hpp:102
	void cv_detail_MatchesInfo_MatchesInfo_const_MatchesInfoR(const cv::detail::MatchesInfo* other, Result<cv::detail::MatchesInfo*>* ocvrs_return) {
		try {
			cv::detail::MatchesInfo* ret = new cv::detail::MatchesInfo(*other);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::MatchesInfo*>))
	}
	
	// getMatches() /usr/include/opencv2/stitching/detail/matchers.hpp:112
	void cv_detail_MatchesInfo_getMatches(cv::detail::MatchesInfo* instance, Result<std::vector<cv::DMatch>*>* ocvrs_return) {
		try {
			std::vector<cv::DMatch> ret = instance->getMatches();
			Ok(new std::vector<cv::DMatch>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<cv::DMatch>*>))
	}
	
	// getInliers() /usr/include/opencv2/stitching/detail/matchers.hpp:113
	void cv_detail_MatchesInfo_getInliers(cv::detail::MatchesInfo* instance, Result<std::vector<unsigned char>*>* ocvrs_return) {
		try {
			std::vector<unsigned char> ret = instance->getInliers();
			Ok(new std::vector<unsigned char>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<unsigned char>*>))
	}
	
	cv::detail::ProjectorBase* cv_Detail_MercatorProjector_to_Detail_ProjectorBase(cv::detail::MercatorProjector* instance) {
		return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}
	
	void cv_Detail_MercatorProjector_delete(cv::detail::MercatorProjector* instance) {
		delete instance;
	}
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:475
	void cv_detail_MercatorProjector_mapForward_float_float_floatR_floatR(cv::detail::MercatorProjector* instance, float x, float y, float* u, float* v, Result_void* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:476
	void cv_detail_MercatorProjector_mapBackward_float_float_floatR_floatR(cv::detail::MercatorProjector* instance, float u, float v, float* x, float* y, Result_void* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_MercatorWarper_delete(cv::detail::MercatorWarper* instance) {
		delete instance;
	}
	// MercatorWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:483
	void cv_detail_MercatorWarper_MercatorWarper_float(float scale, Result<cv::detail::MercatorWarper*>* ocvrs_return) {
		try {
			cv::detail::MercatorWarper* ret = new cv::detail::MercatorWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::MercatorWarper*>))
	}
	
	cv::detail::Blender* cv_Detail_MultiBandBlender_to_Detail_Blender(cv::detail::MultiBandBlender* instance) {
		return dynamic_cast<cv::detail::Blender*>(instance);
	}
	
	void cv_Detail_MultiBandBlender_delete(cv::detail::MultiBandBlender* instance) {
		delete instance;
	}
	// MultiBandBlender(int, int, int) /usr/include/opencv2/stitching/detail/blenders.hpp:130
	void cv_detail_MultiBandBlender_MultiBandBlender_int_int_int(int try_gpu, int num_bands, int weight_type, Result<cv::detail::MultiBandBlender*>* ocvrs_return) {
		try {
			cv::detail::MultiBandBlender* ret = new cv::detail::MultiBandBlender(try_gpu, num_bands, weight_type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::MultiBandBlender*>))
	}
	
	// numBands() /usr/include/opencv2/stitching/detail/blenders.hpp:132
	void cv_detail_MultiBandBlender_numBands_const(const cv::detail::MultiBandBlender* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->numBands();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setNumBands(int) /usr/include/opencv2/stitching/detail/blenders.hpp:133
	void cv_detail_MultiBandBlender_setNumBands_int(cv::detail::MultiBandBlender* instance, int val, Result_void* ocvrs_return) {
		try {
			instance->setNumBands(val);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// prepare(cv::Rect) /usr/include/opencv2/stitching/detail/blenders.hpp:135
	void cv_detail_MultiBandBlender_prepare_Rect(cv::detail::MultiBandBlender* instance, cv::Rect* dst_roi, Result_void* ocvrs_return) {
		try {
			instance->prepare(*dst_roi);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// feed(cv::InputArray, cv::InputArray, cv::Point) /usr/include/opencv2/stitching/detail/blenders.hpp:136
	void cv_detail_MultiBandBlender_feed_const__InputArrayR_const__InputArrayR_Point(cv::detail::MultiBandBlender* instance, const cv::_InputArray* img, const cv::_InputArray* mask, cv::Point* tl, Result_void* ocvrs_return) {
		try {
			instance->feed(*img, *mask, *tl);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// blend(cv::InputOutputArray, cv::InputOutputArray) /usr/include/opencv2/stitching/detail/blenders.hpp:137
	void cv_detail_MultiBandBlender_blend_const__InputOutputArrayR_const__InputOutputArrayR(cv::detail::MultiBandBlender* instance, const cv::_InputOutputArray* dst, const cv::_InputOutputArray* dst_mask, Result_void* ocvrs_return) {
		try {
			instance->blend(*dst, *dst_mask);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_NoBundleAdjuster_delete(cv::detail::NoBundleAdjuster* instance) {
		delete instance;
	}
	// NoBundleAdjuster() /usr/include/opencv2/stitching/detail/motion_estimators.hpp:224
	void cv_detail_NoBundleAdjuster_NoBundleAdjuster(Result<cv::detail::NoBundleAdjuster*>* ocvrs_return) {
		try {
			cv::detail::NoBundleAdjuster* ret = new cv::detail::NoBundleAdjuster();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::NoBundleAdjuster*>))
	}
	
	void cv_Detail_NoExposureCompensator_delete(cv::detail::NoExposureCompensator* instance) {
		delete instance;
	}
	// apply(int, cv::Point, cv::InputOutputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:103
	void cv_detail_NoExposureCompensator_apply_int_Point_const__InputOutputArrayR_const__InputArrayR(cv::detail::NoExposureCompensator* instance, int unnamed, cv::Point* unnamed_1, const cv::_InputOutputArray* unnamed_2, const cv::_InputArray* unnamed_3, Result_void* ocvrs_return) {
		try {
			instance->apply(unnamed, *unnamed_1, *unnamed_2, *unnamed_3);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:104
	void cv_detail_NoExposureCompensator_getMatGains_vector_Mat_R(cv::detail::NoExposureCompensator* instance, std::vector<cv::Mat>* umv, Result_void* ocvrs_return) {
		try {
			instance->getMatGains(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setMatGains(std::vector<Mat> &) /usr/include/opencv2/stitching/detail/exposure_compensate.hpp:105
	void cv_detail_NoExposureCompensator_setMatGains_vector_Mat_R(cv::detail::NoExposureCompensator* instance, std::vector<cv::Mat>* umv, Result_void* ocvrs_return) {
		try {
			instance->setMatGains(*umv);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_NoSeamFinder_delete(cv::detail::NoSeamFinder* instance) {
		delete instance;
	}
	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &) /usr/include/opencv2/stitching/detail/seam_finders.hpp:79
	void cv_detail_NoSeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(cv::detail::NoSeamFinder* instance, const std::vector<cv::UMat>* unnamed, const std::vector<cv::Point>* unnamed_1, std::vector<cv::UMat>* unnamed_2, Result_void* ocvrs_return) {
		try {
			instance->find(*unnamed, *unnamed_1, *unnamed_2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &) /usr/include/opencv2/stitching/detail/seam_finders.hpp:87
	void cv_detail_PairwiseSeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(cv::detail::PairwiseSeamFinder* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks, Result_void* ocvrs_return) {
		try {
			instance->find(*src, *corners, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// a /usr/include/opencv2/stitching/detail/warpers.hpp:453
	float cv_detail_PaniniPortraitProjector_getPropA_const(const cv::detail::PaniniPortraitProjector* instance) {
			float ret = instance->a;
			return ret;
	}
	
	// a /usr/include/opencv2/stitching/detail/warpers.hpp:453
	void cv_detail_PaniniPortraitProjector_setPropA_float(cv::detail::PaniniPortraitProjector* instance, float val) {
			instance->a = val;
	}
	
	// b /usr/include/opencv2/stitching/detail/warpers.hpp:453
	float cv_detail_PaniniPortraitProjector_getPropB_const(const cv::detail::PaniniPortraitProjector* instance) {
			float ret = instance->b;
			return ret;
	}
	
	// b /usr/include/opencv2/stitching/detail/warpers.hpp:453
	void cv_detail_PaniniPortraitProjector_setPropB_float(cv::detail::PaniniPortraitProjector* instance, float val) {
			instance->b = val;
	}
	
	cv::detail::ProjectorBase* cv_Detail_PaniniPortraitProjector_to_Detail_ProjectorBase(cv::detail::PaniniPortraitProjector* instance) {
		return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}
	
	void cv_Detail_PaniniPortraitProjector_delete(cv::detail::PaniniPortraitProjector* instance) {
		delete instance;
	}
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:455
	void cv_detail_PaniniPortraitProjector_mapForward_float_float_floatR_floatR(cv::detail::PaniniPortraitProjector* instance, float x, float y, float* u, float* v, Result_void* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:456
	void cv_detail_PaniniPortraitProjector_mapBackward_float_float_floatR_floatR(cv::detail::PaniniPortraitProjector* instance, float u, float v, float* x, float* y, Result_void* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_PaniniPortraitWarper_delete(cv::detail::PaniniPortraitWarper* instance) {
		delete instance;
	}
	// PaniniPortraitWarper(float, float, float) /usr/include/opencv2/stitching/detail/warpers.hpp:463
	void cv_detail_PaniniPortraitWarper_PaniniPortraitWarper_float_float_float(float scale, float A, float B, Result<cv::detail::PaniniPortraitWarper*>* ocvrs_return) {
		try {
			cv::detail::PaniniPortraitWarper* ret = new cv::detail::PaniniPortraitWarper(scale, A, B);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::PaniniPortraitWarper*>))
	}
	
	// a /usr/include/opencv2/stitching/detail/warpers.hpp:432
	float cv_detail_PaniniProjector_getPropA_const(const cv::detail::PaniniProjector* instance) {
			float ret = instance->a;
			return ret;
	}
	
	// a /usr/include/opencv2/stitching/detail/warpers.hpp:432
	void cv_detail_PaniniProjector_setPropA_float(cv::detail::PaniniProjector* instance, float val) {
			instance->a = val;
	}
	
	// b /usr/include/opencv2/stitching/detail/warpers.hpp:432
	float cv_detail_PaniniProjector_getPropB_const(const cv::detail::PaniniProjector* instance) {
			float ret = instance->b;
			return ret;
	}
	
	// b /usr/include/opencv2/stitching/detail/warpers.hpp:432
	void cv_detail_PaniniProjector_setPropB_float(cv::detail::PaniniProjector* instance, float val) {
			instance->b = val;
	}
	
	cv::detail::ProjectorBase* cv_Detail_PaniniProjector_to_Detail_ProjectorBase(cv::detail::PaniniProjector* instance) {
		return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}
	
	void cv_Detail_PaniniProjector_delete(cv::detail::PaniniProjector* instance) {
		delete instance;
	}
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:434
	void cv_detail_PaniniProjector_mapForward_float_float_floatR_floatR(cv::detail::PaniniProjector* instance, float x, float y, float* u, float* v, Result_void* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:435
	void cv_detail_PaniniProjector_mapBackward_float_float_floatR_floatR(cv::detail::PaniniProjector* instance, float u, float v, float* x, float* y, Result_void* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_PaniniWarper_delete(cv::detail::PaniniWarper* instance) {
		delete instance;
	}
	// PaniniWarper(float, float, float) /usr/include/opencv2/stitching/detail/warpers.hpp:442
	void cv_detail_PaniniWarper_PaniniWarper_float_float_float(float scale, float A, float B, Result<cv::detail::PaniniWarper*>* ocvrs_return) {
		try {
			cv::detail::PaniniWarper* ret = new cv::detail::PaniniWarper(scale, A, B);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::PaniniWarper*>))
	}
	
	cv::detail::ProjectorBase* cv_Detail_PlanePortraitProjector_to_Detail_ProjectorBase(cv::detail::PlanePortraitProjector* instance) {
		return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}
	
	void cv_Detail_PlanePortraitProjector_delete(cv::detail::PlanePortraitProjector* instance) {
		delete instance;
	}
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:658
	void cv_detail_PlanePortraitProjector_mapForward_float_float_floatR_floatR(cv::detail::PlanePortraitProjector* instance, float x, float y, float* u, float* v, Result_void* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:659
	void cv_detail_PlanePortraitProjector_mapBackward_float_float_floatR_floatR(cv::detail::PlanePortraitProjector* instance, float u, float v, float* x, float* y, Result_void* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_PlanePortraitWarper_delete(cv::detail::PlanePortraitWarper* instance) {
		delete instance;
	}
	// PlanePortraitWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:666
	void cv_detail_PlanePortraitWarper_PlanePortraitWarper_float(float scale, Result<cv::detail::PlanePortraitWarper*>* ocvrs_return) {
		try {
			cv::detail::PlanePortraitWarper* ret = new cv::detail::PlanePortraitWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::PlanePortraitWarper*>))
	}
	
	cv::detail::ProjectorBase* cv_Detail_PlaneProjector_to_Detail_ProjectorBase(cv::detail::PlaneProjector* instance) {
		return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}
	
	void cv_Detail_PlaneProjector_delete(cv::detail::PlaneProjector* instance) {
		delete instance;
	}
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:193
	void cv_detail_PlaneProjector_mapForward_float_float_floatR_floatR(cv::detail::PlaneProjector* instance, float x, float y, float* u, float* v, Result_void* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:194
	void cv_detail_PlaneProjector_mapBackward_float_float_floatR_floatR(cv::detail::PlaneProjector* instance, float u, float v, float* x, float* y, Result_void* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::detail::AffineWarper* cv_Detail_PlaneWarper_to_Detail_AffineWarper(cv::detail::PlaneWarper* instance) {
		return dynamic_cast<cv::detail::AffineWarper*>(instance);
	}
	
	cv::detail::PlaneWarperGpu* cv_Detail_PlaneWarper_to_Detail_PlaneWarperGpu(cv::detail::PlaneWarper* instance) {
		return dynamic_cast<cv::detail::PlaneWarperGpu*>(instance);
	}
	
	void cv_Detail_PlaneWarper_delete(cv::detail::PlaneWarper* instance) {
		delete instance;
	}
	// PlaneWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:206
	void cv_detail_PlaneWarper_PlaneWarper_float(float scale, Result<cv::detail::PlaneWarper*>* ocvrs_return) {
		try {
			cv::detail::PlaneWarper* ret = new cv::detail::PlaneWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::PlaneWarper*>))
	}
	
	// warpPoint(const cv::Point2f &, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:208
	void cv_detail_PlaneWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(cv::detail::PlaneWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->warpPoint(*pt, *K, *R);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	// warpPoint(const cv::Point2f &, cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:209
	void cv_detail_PlaneWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::detail::PlaneWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->warpPoint(*pt, *K, *R, *T);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	// warpPointBackward(const cv::Point2f &, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:211
	void cv_detail_PlaneWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR(cv::detail::PlaneWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->warpPointBackward(*pt, *K, *R);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	// warpPointBackward(const cv::Point2f &, cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:212
	void cv_detail_PlaneWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::detail::PlaneWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->warpPointBackward(*pt, *K, *R, *T);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:214
	void cv_detail_PlaneWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::PlaneWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *T, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:215
	void cv_detail_PlaneWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::PlaneWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:217
	void cv_detail_PlaneWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::PlaneWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:219
	void cv_detail_PlaneWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::PlaneWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, *T, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	// warpRoi(cv::Size, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:222
	void cv_detail_PlaneWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(cv::detail::PlaneWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->warpRoi(*src_size, *K, *R);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// warpRoi(cv::Size, cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:223
	void cv_detail_PlaneWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::detail::PlaneWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->warpRoi(*src_size, *K, *R, *T);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	cv::detail::PlaneWarper* cv_Detail_PlaneWarperGpu_to_Detail_PlaneWarper(cv::detail::PlaneWarperGpu* instance) {
		return dynamic_cast<cv::detail::PlaneWarper*>(instance);
	}
	
	void cv_Detail_PlaneWarperGpu_delete(cv::detail::PlaneWarperGpu* instance) {
		delete instance;
	}
	// PlaneWarperGpu(float) /usr/include/opencv2/stitching/detail/warpers.hpp:504
	void cv_detail_PlaneWarperGpu_PlaneWarperGpu_float(float scale, Result<cv::detail::PlaneWarperGpu*>* ocvrs_return) {
		try {
			cv::detail::PlaneWarperGpu* ret = new cv::detail::PlaneWarperGpu(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::PlaneWarperGpu*>))
	}
	
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:506
	void cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::PlaneWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:514
	void cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::PlaneWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *T, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:522
	void cv_detail_PlaneWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::PlaneWarperGpu* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:531
	void cv_detail_PlaneWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::PlaneWarperGpu* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, *T, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cuda::GpuMat &, cuda::GpuMat &) /usr/include/opencv2/stitching/detail/warpers.hpp:540
	void cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(cv::detail::PlaneWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, cv::cuda::GpuMat* xmap, cv::cuda::GpuMat* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::InputArray, cuda::GpuMat &, cuda::GpuMat &) /usr/include/opencv2/stitching/detail/warpers.hpp:542
	void cv_detail_PlaneWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(cv::detail::PlaneWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, cv::cuda::GpuMat* xmap, cv::cuda::GpuMat* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *T, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// warp(const cuda::GpuMat &, cv::InputArray, cv::InputArray, int, int, cuda::GpuMat &) /usr/include/opencv2/stitching/detail/warpers.hpp:544
	void cv_detail_PlaneWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(cv::detail::PlaneWarperGpu* instance, const cv::cuda::GpuMat* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, cv::cuda::GpuMat* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	// warp(const cuda::GpuMat &, cv::InputArray, cv::InputArray, cv::InputArray, int, int, cuda::GpuMat &) /usr/include/opencv2/stitching/detail/warpers.hpp:547
	void cv_detail_PlaneWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(cv::detail::PlaneWarperGpu* instance, const cv::cuda::GpuMat* src, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, int interp_mode, int border_mode, cv::cuda::GpuMat* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, *T, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	// scale /usr/include/opencv2/stitching/detail/warpers.hpp:147
	float cv_detail_ProjectorBase_getPropScale_const(const cv::detail::ProjectorBase* instance) {
			float ret = instance->scale;
			return ret;
	}
	
	// scale /usr/include/opencv2/stitching/detail/warpers.hpp:147
	void cv_detail_ProjectorBase_setPropScale_float(cv::detail::ProjectorBase* instance, float val) {
			instance->scale = val;
	}
	
	// k /usr/include/opencv2/stitching/detail/warpers.hpp:148
	float** cv_detail_ProjectorBase_getPropK(cv::detail::ProjectorBase* instance) {
			float(*ret)[9] = &instance->k;
			return (float**)ret;
	}
	
	// rinv /usr/include/opencv2/stitching/detail/warpers.hpp:149
	float** cv_detail_ProjectorBase_getPropRinv(cv::detail::ProjectorBase* instance) {
			float(*ret)[9] = &instance->rinv;
			return (float**)ret;
	}
	
	// r_kinv /usr/include/opencv2/stitching/detail/warpers.hpp:150
	float** cv_detail_ProjectorBase_getPropR_kinv(cv::detail::ProjectorBase* instance) {
			float(*ret)[9] = &instance->r_kinv;
			return (float**)ret;
	}
	
	// k_rinv /usr/include/opencv2/stitching/detail/warpers.hpp:151
	float** cv_detail_ProjectorBase_getPropK_rinv(cv::detail::ProjectorBase* instance) {
			float(*ret)[9] = &instance->k_rinv;
			return (float**)ret;
	}
	
	// t /usr/include/opencv2/stitching/detail/warpers.hpp:152
	float** cv_detail_ProjectorBase_getPropT(cv::detail::ProjectorBase* instance) {
			float(*ret)[3] = &instance->t;
			return (float**)ret;
	}
	
	void cv_Detail_ProjectorBase_delete(cv::detail::ProjectorBase* instance) {
		delete instance;
	}
	// setCameraParams(cv::InputArray, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:143
	void cv_detail_ProjectorBase_setCameraParams_const__InputArrayR_const__InputArrayR_const__InputArrayR(cv::detail::ProjectorBase* instance, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_InputArray* T, Result_void* ocvrs_return) {
		try {
			instance->setCameraParams(*K, *R, *T);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// warpPoint(const cv::Point2f &, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:71
	void cv_detail_RotationWarper_warpPoint_const_Point2fR_const__InputArrayR_const__InputArrayR(cv::detail::RotationWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->warpPoint(*pt, *K, *R);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	// warpPointBackward(const cv::Point2f &, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:81
	void cv_detail_RotationWarper_warpPointBackward_const_Point2fR_const__InputArrayR_const__InputArrayR(cv::detail::RotationWarper* instance, const cv::Point2f* pt, const cv::_InputArray* K, const cv::_InputArray* R, Result<cv::Point2f>* ocvrs_return) {
		try {
			cv::Point2f ret = instance->warpPointBackward(*pt, *K, *R);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point2f>))
	}
	
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:99
	void cv_detail_RotationWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::RotationWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:111
	void cv_detail_RotationWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::RotationWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	// warpBackward(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::Size, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:124
	void cv_detail_RotationWarper_warpBackward_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_Size_const__OutputArrayR(cv::detail::RotationWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, cv::Size* dst_size, const cv::_OutputArray* dst, Result_void* ocvrs_return) {
		try {
			instance->warpBackward(*src, *K, *R, interp_mode, border_mode, *dst_size, *dst);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// warpRoi(cv::Size, cv::InputArray, cv::InputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:133
	void cv_detail_RotationWarper_warpRoi_Size_const__InputArrayR_const__InputArrayR(cv::detail::RotationWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->warpRoi(*src_size, *K, *R);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// getScale() /usr/include/opencv2/stitching/detail/warpers.hpp:135
	void cv_detail_RotationWarper_getScale_const(const cv::detail::RotationWarper* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScale();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setScale(float) /usr/include/opencv2/stitching/detail/warpers.hpp:136
	void cv_detail_RotationWarper_setScale_float(cv::detail::RotationWarper* instance, float unnamed, Result_void* ocvrs_return) {
		try {
			instance->setScale(unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &) /usr/include/opencv2/stitching/detail/seam_finders.hpp:69
	void cv_detail_SeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(cv::detail::SeamFinder* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks, Result_void* ocvrs_return) {
		try {
			instance->find(*src, *corners, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// createDefault(int) /usr/include/opencv2/stitching/detail/seam_finders.hpp:71
	void cv_detail_SeamFinder_createDefault_int(int type, Result<cv::Ptr<cv::detail::SeamFinder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::detail::SeamFinder> ret = cv::detail::SeamFinder::createDefault(type);
			Ok(new cv::Ptr<cv::detail::SeamFinder>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::detail::SeamFinder>*>))
	}
	
	cv::detail::ProjectorBase* cv_Detail_SphericalPortraitProjector_to_Detail_ProjectorBase(cv::detail::SphericalPortraitProjector* instance) {
		return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}
	
	void cv_Detail_SphericalPortraitProjector_delete(cv::detail::SphericalPortraitProjector* instance) {
		delete instance;
	}
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:621
	void cv_detail_SphericalPortraitProjector_mapForward_float_float_floatR_floatR(cv::detail::SphericalPortraitProjector* instance, float x, float y, float* u, float* v, Result_void* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:622
	void cv_detail_SphericalPortraitProjector_mapBackward_float_float_floatR_floatR(cv::detail::SphericalPortraitProjector* instance, float u, float v, float* x, float* y, Result_void* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_SphericalPortraitWarper_delete(cv::detail::SphericalPortraitWarper* instance) {
		delete instance;
	}
	// SphericalPortraitWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:631
	void cv_detail_SphericalPortraitWarper_SphericalPortraitWarper_float(float scale, Result<cv::detail::SphericalPortraitWarper*>* ocvrs_return) {
		try {
			cv::detail::SphericalPortraitWarper* ret = new cv::detail::SphericalPortraitWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::SphericalPortraitWarper*>))
	}
	
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:304
	void cv_detail_SphericalProjector_mapForward_float_float_floatR_floatR(cv::detail::SphericalProjector instance, float x, float y, float* u, float* v, Result_void* ocvrs_return) {
		try {
			instance.mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:305
	void cv_detail_SphericalProjector_mapBackward_float_float_floatR_floatR(cv::detail::SphericalProjector instance, float u, float v, float* x, float* y, Result_void* ocvrs_return) {
		try {
			instance.mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	cv::detail::SphericalWarperGpu* cv_Detail_SphericalWarper_to_Detail_SphericalWarperGpu(cv::detail::SphericalWarper* instance) {
		return dynamic_cast<cv::detail::SphericalWarperGpu*>(instance);
	}
	
	void cv_Detail_SphericalWarper_delete(cv::detail::SphericalWarper* instance) {
		delete instance;
	}
	// SphericalWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:323
	void cv_detail_SphericalWarper_SphericalWarper_float(float scale, Result<cv::detail::SphericalWarper*>* ocvrs_return) {
		try {
			cv::detail::SphericalWarper* ret = new cv::detail::SphericalWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::SphericalWarper*>))
	}
	
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:325
	void cv_detail_SphericalWarper_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::SphericalWarper* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:326
	void cv_detail_SphericalWarper_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::SphericalWarper* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	cv::detail::SphericalWarper* cv_Detail_SphericalWarperGpu_to_Detail_SphericalWarper(cv::detail::SphericalWarperGpu* instance) {
		return dynamic_cast<cv::detail::SphericalWarper*>(instance);
	}
	
	void cv_Detail_SphericalWarperGpu_delete(cv::detail::SphericalWarperGpu* instance) {
		delete instance;
	}
	// SphericalWarperGpu(float) /usr/include/opencv2/stitching/detail/warpers.hpp:558
	void cv_detail_SphericalWarperGpu_SphericalWarperGpu_float(float scale, Result<cv::detail::SphericalWarperGpu*>* ocvrs_return) {
		try {
			cv::detail::SphericalWarperGpu* ret = new cv::detail::SphericalWarperGpu(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::SphericalWarperGpu*>))
	}
	
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:560
	void cv_detail_SphericalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::detail::SphericalWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, const cv::_OutputArray* xmap, const cv::_OutputArray* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// warp(cv::InputArray, cv::InputArray, cv::InputArray, int, int, cv::OutputArray) /usr/include/opencv2/stitching/detail/warpers.hpp:568
	void cv_detail_SphericalWarperGpu_warp_const__InputArrayR_const__InputArrayR_const__InputArrayR_int_int_const__OutputArrayR(cv::detail::SphericalWarperGpu* instance, const cv::_InputArray* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, const cv::_OutputArray* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	// buildMaps(cv::Size, cv::InputArray, cv::InputArray, cuda::GpuMat &, cuda::GpuMat &) /usr/include/opencv2/stitching/detail/warpers.hpp:577
	void cv_detail_SphericalWarperGpu_buildMaps_Size_const__InputArrayR_const__InputArrayR_GpuMatR_GpuMatR(cv::detail::SphericalWarperGpu* instance, cv::Size* src_size, const cv::_InputArray* K, const cv::_InputArray* R, cv::cuda::GpuMat* xmap, cv::cuda::GpuMat* ymap, Result<cv::Rect>* ocvrs_return) {
		try {
			cv::Rect ret = instance->buildMaps(*src_size, *K, *R, *xmap, *ymap);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Rect>))
	}
	
	// warp(const cuda::GpuMat &, cv::InputArray, cv::InputArray, int, int, cuda::GpuMat &) /usr/include/opencv2/stitching/detail/warpers.hpp:579
	void cv_detail_SphericalWarperGpu_warp_const_GpuMatR_const__InputArrayR_const__InputArrayR_int_int_GpuMatR(cv::detail::SphericalWarperGpu* instance, const cv::cuda::GpuMat* src, const cv::_InputArray* K, const cv::_InputArray* R, int interp_mode, int border_mode, cv::cuda::GpuMat* dst, Result<cv::Point>* ocvrs_return) {
		try {
			cv::Point ret = instance->warp(*src, *K, *R, interp_mode, border_mode, *dst);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Point>))
	}
	
	cv::detail::ProjectorBase* cv_Detail_StereographicProjector_to_Detail_ProjectorBase(cv::detail::StereographicProjector* instance) {
		return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}
	
	void cv_Detail_StereographicProjector_delete(cv::detail::StereographicProjector* instance) {
		delete instance;
	}
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:376
	void cv_detail_StereographicProjector_mapForward_float_float_floatR_floatR(cv::detail::StereographicProjector* instance, float x, float y, float* u, float* v, Result_void* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:377
	void cv_detail_StereographicProjector_mapBackward_float_float_floatR_floatR(cv::detail::StereographicProjector* instance, float u, float v, float* x, float* y, Result_void* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_StereographicWarper_delete(cv::detail::StereographicWarper* instance) {
		delete instance;
	}
	// StereographicWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:384
	void cv_detail_StereographicWarper_StereographicWarper_float(float scale, Result<cv::detail::StereographicWarper*>* ocvrs_return) {
		try {
			cv::detail::StereographicWarper* ret = new cv::detail::StereographicWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::StereographicWarper*>))
	}
	
	cv::detail::ProjectorBase* cv_Detail_TransverseMercatorProjector_to_Detail_ProjectorBase(cv::detail::TransverseMercatorProjector* instance) {
		return dynamic_cast<cv::detail::ProjectorBase*>(instance);
	}
	
	void cv_Detail_TransverseMercatorProjector_delete(cv::detail::TransverseMercatorProjector* instance) {
		delete instance;
	}
	// mapForward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:489
	void cv_detail_TransverseMercatorProjector_mapForward_float_float_floatR_floatR(cv::detail::TransverseMercatorProjector* instance, float x, float y, float* u, float* v, Result_void* ocvrs_return) {
		try {
			instance->mapForward(x, y, *u, *v);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// mapBackward(float, float, float &, float &) /usr/include/opencv2/stitching/detail/warpers.hpp:490
	void cv_detail_TransverseMercatorProjector_mapBackward_float_float_floatR_floatR(cv::detail::TransverseMercatorProjector* instance, float u, float v, float* x, float* y, Result_void* ocvrs_return) {
		try {
			instance->mapBackward(u, v, *x, *y);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_Detail_TransverseMercatorWarper_delete(cv::detail::TransverseMercatorWarper* instance) {
		delete instance;
	}
	// TransverseMercatorWarper(float) /usr/include/opencv2/stitching/detail/warpers.hpp:497
	void cv_detail_TransverseMercatorWarper_TransverseMercatorWarper_float(float scale, Result<cv::detail::TransverseMercatorWarper*>* ocvrs_return) {
		try {
			cv::detail::TransverseMercatorWarper* ret = new cv::detail::TransverseMercatorWarper(scale);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::detail::TransverseMercatorWarper*>))
	}
	
	void cv_Detail_VoronoiSeamFinder_delete(cv::detail::VoronoiSeamFinder* instance) {
		delete instance;
	}
	// find(const std::vector<UMat> &, const std::vector<Point> &, std::vector<UMat> &) /usr/include/opencv2/stitching/detail/seam_finders.hpp:111
	void cv_detail_VoronoiSeamFinder_find_const_vector_UMat_R_const_vector_Point_R_vector_UMat_R(cv::detail::VoronoiSeamFinder* instance, const std::vector<cv::UMat>* src, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks, Result_void* ocvrs_return) {
		try {
			instance->find(*src, *corners, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// find(const std::vector<Size> &, const std::vector<Point> &, std::vector<UMat> &) /usr/include/opencv2/stitching/detail/seam_finders.hpp:113
	void cv_detail_VoronoiSeamFinder_find_const_vector_Size_R_const_vector_Point_R_vector_UMat_R(cv::detail::VoronoiSeamFinder* instance, const std::vector<cv::Size>* size, const std::vector<cv::Point>* corners, std::vector<cv::UMat>* masks, Result_void* ocvrs_return) {
		try {
			instance->find(*size, *corners, *masks);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
}
