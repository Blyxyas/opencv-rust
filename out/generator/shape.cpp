#include "ocvrs_common.hpp"
#include <opencv2/shape.hpp>
#include "shape_types.hpp"

extern "C" {
	// EMDL1(cv::InputArray, cv::InputArray) /usr/include/opencv2/shape/emdL1.hpp:66
	void cv_EMDL1_const__InputArrayR_const__InputArrayR(const cv::_InputArray* signature1, const cv::_InputArray* signature2, Result<float>* ocvrs_return) {
		try {
			float ret = cv::EMDL1(*signature1, *signature2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// createAffineTransformer(bool) /usr/include/opencv2/shape/shape_transformer.hpp:127
	void cv_createAffineTransformer_bool(bool fullAffine, Result<cv::Ptr<cv::AffineTransformer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::AffineTransformer> ret = cv::createAffineTransformer(fullAffine);
			Ok(new cv::Ptr<cv::AffineTransformer>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::AffineTransformer>*>))
	}
	
	// createChiHistogramCostExtractor(int, float) /usr/include/opencv2/shape/hist_cost.hpp:98
	void cv_createChiHistogramCostExtractor_int_float(int nDummies, float defaultCost, Result<cv::Ptr<cv::HistogramCostExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = cv::createChiHistogramCostExtractor(nDummies, defaultCost);
			Ok(new cv::Ptr<cv::HistogramCostExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::HistogramCostExtractor>*>))
	}
	
	// createEMDHistogramCostExtractor(int, int, float) /usr/include/opencv2/shape/hist_cost.hpp:91
	void cv_createEMDHistogramCostExtractor_int_int_float(int flag, int nDummies, float defaultCost, Result<cv::Ptr<cv::HistogramCostExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = cv::createEMDHistogramCostExtractor(flag, nDummies, defaultCost);
			Ok(new cv::Ptr<cv::HistogramCostExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::HistogramCostExtractor>*>))
	}
	
	// createEMDL1HistogramCostExtractor(int, float) /usr/include/opencv2/shape/hist_cost.hpp:106
	void cv_createEMDL1HistogramCostExtractor_int_float(int nDummies, float defaultCost, Result<cv::Ptr<cv::HistogramCostExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = cv::createEMDL1HistogramCostExtractor(nDummies, defaultCost);
			Ok(new cv::Ptr<cv::HistogramCostExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::HistogramCostExtractor>*>))
	}
	
	// createHausdorffDistanceExtractor(int, float) /usr/include/opencv2/shape/shape_distance.hpp:222
	void cv_createHausdorffDistanceExtractor_int_float(int distanceFlag, float rankProp, Result<cv::Ptr<cv::HausdorffDistanceExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::HausdorffDistanceExtractor> ret = cv::createHausdorffDistanceExtractor(distanceFlag, rankProp);
			Ok(new cv::Ptr<cv::HausdorffDistanceExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::HausdorffDistanceExtractor>*>))
	}
	
	// createNormHistogramCostExtractor(int, int, float) /usr/include/opencv2/shape/hist_cost.hpp:79
	void cv_createNormHistogramCostExtractor_int_int_float(int flag, int nDummies, float defaultCost, Result<cv::Ptr<cv::HistogramCostExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = cv::createNormHistogramCostExtractor(flag, nDummies, defaultCost);
			Ok(new cv::Ptr<cv::HistogramCostExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::HistogramCostExtractor>*>))
	}
	
	// createShapeContextDistanceExtractor(int, int, float, float, int, const Ptr<cv::HistogramCostExtractor> &, const Ptr<cv::ShapeTransformer> &) /usr/include/opencv2/shape/shape_distance.hpp:187
	void cv_createShapeContextDistanceExtractor_int_int_float_float_int_const_Ptr_HistogramCostExtractor_R_const_Ptr_ShapeTransformer_R(int nAngularBins, int nRadialBins, float innerRadius, float outerRadius, int iterations, const cv::Ptr<cv::HistogramCostExtractor>* comparer, const cv::Ptr<cv::ShapeTransformer>* transformer, Result<cv::Ptr<cv::ShapeContextDistanceExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ShapeContextDistanceExtractor> ret = cv::createShapeContextDistanceExtractor(nAngularBins, nRadialBins, innerRadius, outerRadius, iterations, *comparer, *transformer);
			Ok(new cv::Ptr<cv::ShapeContextDistanceExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ShapeContextDistanceExtractor>*>))
	}
	
	// createThinPlateSplineShapeTransformer(double) /usr/include/opencv2/shape/shape_transformer.hpp:112
	void cv_createThinPlateSplineShapeTransformer_double(double regularizationParameter, Result<cv::Ptr<cv::ThinPlateSplineShapeTransformer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ThinPlateSplineShapeTransformer> ret = cv::createThinPlateSplineShapeTransformer(regularizationParameter);
			Ok(new cv::Ptr<cv::ThinPlateSplineShapeTransformer>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ThinPlateSplineShapeTransformer>*>))
	}
	
	// setFullAffine(bool) /usr/include/opencv2/shape/shape_transformer.hpp:122
	void cv_AffineTransformer_setFullAffine_bool(cv::AffineTransformer* instance, bool fullAffine, Result_void* ocvrs_return) {
		try {
			instance->setFullAffine(fullAffine);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getFullAffine() /usr/include/opencv2/shape/shape_transformer.hpp:123
	void cv_AffineTransformer_getFullAffine_const(const cv::AffineTransformer* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getFullAffine();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setNormFlag(int) /usr/include/opencv2/shape/hist_cost.hpp:86
	void cv_EMDHistogramCostExtractor_setNormFlag_int(cv::EMDHistogramCostExtractor* instance, int flag, Result_void* ocvrs_return) {
		try {
			instance->setNormFlag(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNormFlag() /usr/include/opencv2/shape/hist_cost.hpp:87
	void cv_EMDHistogramCostExtractor_getNormFlag_const(const cv::EMDHistogramCostExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNormFlag();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setDistanceFlag(int) /usr/include/opencv2/shape/shape_distance.hpp:208
	void cv_HausdorffDistanceExtractor_setDistanceFlag_int(cv::HausdorffDistanceExtractor* instance, int distanceFlag, Result_void* ocvrs_return) {
		try {
			instance->setDistanceFlag(distanceFlag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDistanceFlag() /usr/include/opencv2/shape/shape_distance.hpp:209
	void cv_HausdorffDistanceExtractor_getDistanceFlag_const(const cv::HausdorffDistanceExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getDistanceFlag();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setRankProportion(float) /usr/include/opencv2/shape/shape_distance.hpp:217
	void cv_HausdorffDistanceExtractor_setRankProportion_float(cv::HausdorffDistanceExtractor* instance, float rankProportion, Result_void* ocvrs_return) {
		try {
			instance->setRankProportion(rankProportion);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getRankProportion() /usr/include/opencv2/shape/shape_distance.hpp:218
	void cv_HausdorffDistanceExtractor_getRankProportion_const(const cv::HausdorffDistanceExtractor* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getRankProportion();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// buildCostMatrix(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/shape/hist_cost.hpp:60
	void cv_HistogramCostExtractor_buildCostMatrix_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::HistogramCostExtractor* instance, const cv::_InputArray* descriptors1, const cv::_InputArray* descriptors2, const cv::_OutputArray* costMatrix, Result_void* ocvrs_return) {
		try {
			instance->buildCostMatrix(*descriptors1, *descriptors2, *costMatrix);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setNDummies(int) /usr/include/opencv2/shape/hist_cost.hpp:62
	void cv_HistogramCostExtractor_setNDummies_int(cv::HistogramCostExtractor* instance, int nDummies, Result_void* ocvrs_return) {
		try {
			instance->setNDummies(nDummies);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNDummies() /usr/include/opencv2/shape/hist_cost.hpp:63
	void cv_HistogramCostExtractor_getNDummies_const(const cv::HistogramCostExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNDummies();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setDefaultCost(float) /usr/include/opencv2/shape/hist_cost.hpp:65
	void cv_HistogramCostExtractor_setDefaultCost_float(cv::HistogramCostExtractor* instance, float defaultCost, Result_void* ocvrs_return) {
		try {
			instance->setDefaultCost(defaultCost);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDefaultCost() /usr/include/opencv2/shape/hist_cost.hpp:66
	void cv_HistogramCostExtractor_getDefaultCost_const(const cv::HistogramCostExtractor* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getDefaultCost();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setNormFlag(int) /usr/include/opencv2/shape/hist_cost.hpp:74
	void cv_NormHistogramCostExtractor_setNormFlag_int(cv::NormHistogramCostExtractor* instance, int flag, Result_void* ocvrs_return) {
		try {
			instance->setNormFlag(flag);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNormFlag() /usr/include/opencv2/shape/hist_cost.hpp:75
	void cv_NormHistogramCostExtractor_getNormFlag_const(const cv::NormHistogramCostExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getNormFlag();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setAngularBins(int) /usr/include/opencv2/shape/shape_distance.hpp:89
	void cv_ShapeContextDistanceExtractor_setAngularBins_int(cv::ShapeContextDistanceExtractor* instance, int nAngularBins, Result_void* ocvrs_return) {
		try {
			instance->setAngularBins(nAngularBins);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getAngularBins() /usr/include/opencv2/shape/shape_distance.hpp:90
	void cv_ShapeContextDistanceExtractor_getAngularBins_const(const cv::ShapeContextDistanceExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getAngularBins();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setRadialBins(int) /usr/include/opencv2/shape/shape_distance.hpp:97
	void cv_ShapeContextDistanceExtractor_setRadialBins_int(cv::ShapeContextDistanceExtractor* instance, int nRadialBins, Result_void* ocvrs_return) {
		try {
			instance->setRadialBins(nRadialBins);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getRadialBins() /usr/include/opencv2/shape/shape_distance.hpp:98
	void cv_ShapeContextDistanceExtractor_getRadialBins_const(const cv::ShapeContextDistanceExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getRadialBins();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setInnerRadius(float) /usr/include/opencv2/shape/shape_distance.hpp:104
	void cv_ShapeContextDistanceExtractor_setInnerRadius_float(cv::ShapeContextDistanceExtractor* instance, float innerRadius, Result_void* ocvrs_return) {
		try {
			instance->setInnerRadius(innerRadius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getInnerRadius() /usr/include/opencv2/shape/shape_distance.hpp:105
	void cv_ShapeContextDistanceExtractor_getInnerRadius_const(const cv::ShapeContextDistanceExtractor* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getInnerRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setOuterRadius(float) /usr/include/opencv2/shape/shape_distance.hpp:111
	void cv_ShapeContextDistanceExtractor_setOuterRadius_float(cv::ShapeContextDistanceExtractor* instance, float outerRadius, Result_void* ocvrs_return) {
		try {
			instance->setOuterRadius(outerRadius);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getOuterRadius() /usr/include/opencv2/shape/shape_distance.hpp:112
	void cv_ShapeContextDistanceExtractor_getOuterRadius_const(const cv::ShapeContextDistanceExtractor* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getOuterRadius();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setRotationInvariant(bool) /usr/include/opencv2/shape/shape_distance.hpp:114
	void cv_ShapeContextDistanceExtractor_setRotationInvariant_bool(cv::ShapeContextDistanceExtractor* instance, bool rotationInvariant, Result_void* ocvrs_return) {
		try {
			instance->setRotationInvariant(rotationInvariant);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getRotationInvariant() /usr/include/opencv2/shape/shape_distance.hpp:115
	void cv_ShapeContextDistanceExtractor_getRotationInvariant_const(const cv::ShapeContextDistanceExtractor* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->getRotationInvariant();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setShapeContextWeight(float) /usr/include/opencv2/shape/shape_distance.hpp:124
	void cv_ShapeContextDistanceExtractor_setShapeContextWeight_float(cv::ShapeContextDistanceExtractor* instance, float shapeContextWeight, Result_void* ocvrs_return) {
		try {
			instance->setShapeContextWeight(shapeContextWeight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getShapeContextWeight() /usr/include/opencv2/shape/shape_distance.hpp:125
	void cv_ShapeContextDistanceExtractor_getShapeContextWeight_const(const cv::ShapeContextDistanceExtractor* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getShapeContextWeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setImageAppearanceWeight(float) /usr/include/opencv2/shape/shape_distance.hpp:136
	void cv_ShapeContextDistanceExtractor_setImageAppearanceWeight_float(cv::ShapeContextDistanceExtractor* instance, float imageAppearanceWeight, Result_void* ocvrs_return) {
		try {
			instance->setImageAppearanceWeight(imageAppearanceWeight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getImageAppearanceWeight() /usr/include/opencv2/shape/shape_distance.hpp:137
	void cv_ShapeContextDistanceExtractor_getImageAppearanceWeight_const(const cv::ShapeContextDistanceExtractor* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getImageAppearanceWeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setBendingEnergyWeight(float) /usr/include/opencv2/shape/shape_distance.hpp:146
	void cv_ShapeContextDistanceExtractor_setBendingEnergyWeight_float(cv::ShapeContextDistanceExtractor* instance, float bendingEnergyWeight, Result_void* ocvrs_return) {
		try {
			instance->setBendingEnergyWeight(bendingEnergyWeight);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getBendingEnergyWeight() /usr/include/opencv2/shape/shape_distance.hpp:147
	void cv_ShapeContextDistanceExtractor_getBendingEnergyWeight_const(const cv::ShapeContextDistanceExtractor* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getBendingEnergyWeight();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setImages(cv::InputArray, cv::InputArray) /usr/include/opencv2/shape/shape_distance.hpp:155
	void cv_ShapeContextDistanceExtractor_setImages_const__InputArrayR_const__InputArrayR(cv::ShapeContextDistanceExtractor* instance, const cv::_InputArray* image1, const cv::_InputArray* image2, Result_void* ocvrs_return) {
		try {
			instance->setImages(*image1, *image2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getImages(cv::OutputArray, cv::OutputArray) /usr/include/opencv2/shape/shape_distance.hpp:156
	void cv_ShapeContextDistanceExtractor_getImages_const_const__OutputArrayR_const__OutputArrayR(const cv::ShapeContextDistanceExtractor* instance, const cv::_OutputArray* image1, const cv::_OutputArray* image2, Result_void* ocvrs_return) {
		try {
			instance->getImages(*image1, *image2);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setIterations(int) /usr/include/opencv2/shape/shape_distance.hpp:158
	void cv_ShapeContextDistanceExtractor_setIterations_int(cv::ShapeContextDistanceExtractor* instance, int iterations, Result_void* ocvrs_return) {
		try {
			instance->setIterations(iterations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getIterations() /usr/include/opencv2/shape/shape_distance.hpp:159
	void cv_ShapeContextDistanceExtractor_getIterations_const(const cv::ShapeContextDistanceExtractor* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getIterations();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setCostExtractor(Ptr<cv::HistogramCostExtractor>) /usr/include/opencv2/shape/shape_distance.hpp:166
	void cv_ShapeContextDistanceExtractor_setCostExtractor_Ptr_HistogramCostExtractor_(cv::ShapeContextDistanceExtractor* instance, cv::Ptr<cv::HistogramCostExtractor>* comparer, Result_void* ocvrs_return) {
		try {
			instance->setCostExtractor(*comparer);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getCostExtractor() /usr/include/opencv2/shape/shape_distance.hpp:167
	void cv_ShapeContextDistanceExtractor_getCostExtractor_const(const cv::ShapeContextDistanceExtractor* instance, Result<cv::Ptr<cv::HistogramCostExtractor>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::HistogramCostExtractor> ret = instance->getCostExtractor();
			Ok(new cv::Ptr<cv::HistogramCostExtractor>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::HistogramCostExtractor>*>))
	}
	
	// setStdDev(float) /usr/include/opencv2/shape/shape_distance.hpp:173
	void cv_ShapeContextDistanceExtractor_setStdDev_float(cv::ShapeContextDistanceExtractor* instance, float sigma, Result_void* ocvrs_return) {
		try {
			instance->setStdDev(sigma);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getStdDev() /usr/include/opencv2/shape/shape_distance.hpp:174
	void cv_ShapeContextDistanceExtractor_getStdDev_const(const cv::ShapeContextDistanceExtractor* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getStdDev();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setTransformAlgorithm(Ptr<cv::ShapeTransformer>) /usr/include/opencv2/shape/shape_distance.hpp:181
	void cv_ShapeContextDistanceExtractor_setTransformAlgorithm_Ptr_ShapeTransformer_(cv::ShapeContextDistanceExtractor* instance, cv::Ptr<cv::ShapeTransformer>* transformer, Result_void* ocvrs_return) {
		try {
			instance->setTransformAlgorithm(*transformer);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTransformAlgorithm() /usr/include/opencv2/shape/shape_distance.hpp:182
	void cv_ShapeContextDistanceExtractor_getTransformAlgorithm_const(const cv::ShapeContextDistanceExtractor* instance, Result<cv::Ptr<cv::ShapeTransformer>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::ShapeTransformer> ret = instance->getTransformAlgorithm();
			Ok(new cv::Ptr<cv::ShapeTransformer>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::ShapeTransformer>*>))
	}
	
	// computeDistance(cv::InputArray, cv::InputArray) /usr/include/opencv2/shape/shape_distance.hpp:69
	void cv_ShapeDistanceExtractor_computeDistance_const__InputArrayR_const__InputArrayR(cv::ShapeDistanceExtractor* instance, const cv::_InputArray* contour1, const cv::_InputArray* contour2, Result<float>* ocvrs_return) {
		try {
			float ret = instance->computeDistance(*contour1, *contour2);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// estimateTransformation(cv::InputArray, cv::InputArray, std::vector<DMatch> &) /usr/include/opencv2/shape/shape_transformer.hpp:67
	void cv_ShapeTransformer_estimateTransformation_const__InputArrayR_const__InputArrayR_vector_DMatch_R(cv::ShapeTransformer* instance, const cv::_InputArray* transformingShape, const cv::_InputArray* targetShape, std::vector<cv::DMatch>* matches, Result_void* ocvrs_return) {
		try {
			instance->estimateTransformation(*transformingShape, *targetShape, *matches);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// applyTransformation(cv::InputArray, cv::OutputArray) /usr/include/opencv2/shape/shape_transformer.hpp:75
	void cv_ShapeTransformer_applyTransformation_const__InputArrayR_const__OutputArrayR(cv::ShapeTransformer* instance, const cv::_InputArray* input, const cv::_OutputArray* output, Result<float>* ocvrs_return) {
		try {
			float ret = instance->applyTransformation(*input, *output);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// warpImage(cv::InputArray, cv::OutputArray, int, int, const cv::Scalar &) /usr/include/opencv2/shape/shape_transformer.hpp:85
	void cv_ShapeTransformer_warpImage_const_const__InputArrayR_const__OutputArrayR_int_int_const_ScalarR(const cv::ShapeTransformer* instance, const cv::_InputArray* transformingImage, const cv::_OutputArray* output, int flags, int borderMode, const cv::Scalar* borderValue, Result_void* ocvrs_return) {
		try {
			instance->warpImage(*transformingImage, *output, flags, borderMode, *borderValue);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setRegularizationParameter(double) /usr/include/opencv2/shape/shape_transformer.hpp:106
	void cv_ThinPlateSplineShapeTransformer_setRegularizationParameter_double(cv::ThinPlateSplineShapeTransformer* instance, double beta, Result_void* ocvrs_return) {
		try {
			instance->setRegularizationParameter(beta);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getRegularizationParameter() /usr/include/opencv2/shape/shape_transformer.hpp:107
	void cv_ThinPlateSplineShapeTransformer_getRegularizationParameter_const(const cv::ThinPlateSplineShapeTransformer* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getRegularizationParameter();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
}
