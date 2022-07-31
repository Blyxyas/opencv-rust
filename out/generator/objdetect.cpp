#include "ocvrs_common.hpp"
#include <opencv2/objdetect.hpp>
#include "objdetect_types.hpp"

extern "C" {
	// createFaceDetectionMaskGenerator() /usr/include/opencv2/objdetect.hpp:342
	void cv_createFaceDetectionMaskGenerator(Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator> ret = cv::createFaceDetectionMaskGenerator();
			Ok(new cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>))
	}
	
	// groupRectangles_meanshift(std::vector<Rect> &, std::vector<double> &, std::vector<double> &, double, cv::Size) /usr/include/opencv2/objdetect.hpp:166
	void cv_groupRectangles_meanshift_vector_Rect_R_vector_double_R_vector_double_R_double_Size(std::vector<cv::Rect>* rectList, std::vector<double>* foundWeights, std::vector<double>* foundScales, double detectThreshold, cv::Size* winDetSize, Result_void* ocvrs_return) {
		try {
			cv::groupRectangles_meanshift(*rectList, *foundWeights, *foundScales, detectThreshold, *winDetSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// groupRectangles(std::vector<Rect> &, int, double) /usr/include/opencv2/objdetect.hpp:155
	void cv_groupRectangles_vector_Rect_R_int_double(std::vector<cv::Rect>* rectList, int groupThreshold, double eps, Result_void* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, groupThreshold, eps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// groupRectangles(std::vector<Rect> &, int, double, std::vector<int> *, std::vector<double> *) /usr/include/opencv2/objdetect.hpp:160
	void cv_groupRectangles_vector_Rect_R_int_double_vector_int_X_vector_double_X(std::vector<cv::Rect>* rectList, int groupThreshold, double eps, std::vector<int>* weights, std::vector<double>* levelWeights, Result_void* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, groupThreshold, eps, weights, levelWeights);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// groupRectangles(std::vector<Rect> &, std::vector<int> &, int, double) /usr/include/opencv2/objdetect.hpp:157
	void cv_groupRectangles_vector_Rect_R_vector_int_R_int_double(std::vector<cv::Rect>* rectList, std::vector<int>* weights, int groupThreshold, double eps, Result_void* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, *weights, groupThreshold, eps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// groupRectangles(std::vector<Rect> &, std::vector<int> &, std::vector<double> &, int, double) /usr/include/opencv2/objdetect.hpp:163
	void cv_groupRectangles_vector_Rect_R_vector_int_R_vector_double_R_int_double(std::vector<cv::Rect>* rectList, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, int groupThreshold, double eps, Result_void* ocvrs_return) {
		try {
			cv::groupRectangles(*rectList, *rejectLevels, *levelWeights, groupThreshold, eps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// empty() /usr/include/opencv2/objdetect.hpp:186
	void cv_BaseCascadeClassifier_empty_const(const cv::BaseCascadeClassifier* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// load(const cv::String &) /usr/include/opencv2/objdetect.hpp:187
	void cv_BaseCascadeClassifier_load_const_StringR(cv::BaseCascadeClassifier* instance, const char* filename, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->load(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// detectMultiScale(cv::InputArray, std::vector<Rect> &, double, int, int, cv::Size, cv::Size) /usr/include/opencv2/objdetect.hpp:188
	void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_double_int_int_Size_Size(cv::BaseCascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, Result_void* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detectMultiScale(cv::InputArray, std::vector<Rect> &, std::vector<int> &, double, int, int, cv::Size, cv::Size) /usr/include/opencv2/objdetect.hpp:194
	void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_vector_int_R_double_int_int_Size_Size(cv::BaseCascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* numDetections, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, Result_void* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *numDetections, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detectMultiScale(cv::InputArray, std::vector<Rect> &, std::vector<int> &, std::vector<double> &, double, int, int, cv::Size, cv::Size, bool) /usr/include/opencv2/objdetect.hpp:201
	void cv_BaseCascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_vector_int_R_vector_double_R_double_int_int_Size_Size_bool(cv::BaseCascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, bool outputRejectLevels, Result_void* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *rejectLevels, *levelWeights, scaleFactor, minNeighbors, flags, *minSize, *maxSize, outputRejectLevels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// isOldFormatCascade() /usr/include/opencv2/objdetect.hpp:210
	void cv_BaseCascadeClassifier_isOldFormatCascade_const(const cv::BaseCascadeClassifier* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isOldFormatCascade();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// getOriginalWindowSize() /usr/include/opencv2/objdetect.hpp:211
	void cv_BaseCascadeClassifier_getOriginalWindowSize_const(const cv::BaseCascadeClassifier* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getOriginalWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// getFeatureType() /usr/include/opencv2/objdetect.hpp:212
	void cv_BaseCascadeClassifier_getFeatureType_const(const cv::BaseCascadeClassifier* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFeatureType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getOldCascade() /usr/include/opencv2/objdetect.hpp:213
	void cv_BaseCascadeClassifier_getOldCascade(cv::BaseCascadeClassifier* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->getOldCascade();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// setMaskGenerator(const Ptr<cv::BaseCascadeClassifier::MaskGenerator> &) /usr/include/opencv2/objdetect.hpp:222
	void cv_BaseCascadeClassifier_setMaskGenerator_const_Ptr_MaskGenerator_R(cv::BaseCascadeClassifier* instance, const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>* maskGenerator, Result_void* ocvrs_return) {
		try {
			instance->setMaskGenerator(*maskGenerator);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaskGenerator() /usr/include/opencv2/objdetect.hpp:223
	void cv_BaseCascadeClassifier_getMaskGenerator(cv::BaseCascadeClassifier* instance, Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator> ret = instance->getMaskGenerator();
			Ok(new cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>))
	}
	
	// generateMask(const cv::Mat &) /usr/include/opencv2/objdetect.hpp:219
	void cv_BaseCascadeClassifier_MaskGenerator_generateMask_const_MatR(cv::BaseCascadeClassifier::MaskGenerator* instance, const cv::Mat* src, Result<cv::Mat*>* ocvrs_return) {
		try {
			cv::Mat ret = instance->generateMask(*src);
			Ok(new cv::Mat(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Mat*>))
	}
	
	// initializeMask(const cv::Mat &) /usr/include/opencv2/objdetect.hpp:220
	void cv_BaseCascadeClassifier_MaskGenerator_initializeMask_const_MatR(cv::BaseCascadeClassifier::MaskGenerator* instance, const cv::Mat* unnamed, Result_void* ocvrs_return) {
		try {
			instance->initializeMask(*unnamed);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// cc /usr/include/opencv2/objdetect.hpp:339
	cv::Ptr<cv::BaseCascadeClassifier>* cv_CascadeClassifier_getPropCc(cv::CascadeClassifier* instance) {
			cv::Ptr<cv::BaseCascadeClassifier> ret = instance->cc;
			return new cv::Ptr<cv::BaseCascadeClassifier>(ret);
	}
	
	// cc /usr/include/opencv2/objdetect.hpp:339
	void cv_CascadeClassifier_setPropCc_Ptr_BaseCascadeClassifier_(cv::CascadeClassifier* instance, cv::Ptr<cv::BaseCascadeClassifier>* val) {
			instance->cc = *val;
	}
	
	void cv_CascadeClassifier_delete(cv::CascadeClassifier* instance) {
		delete instance;
	}
	// CascadeClassifier() /usr/include/opencv2/objdetect.hpp:235
	void cv_CascadeClassifier_CascadeClassifier(Result<cv::CascadeClassifier*>* ocvrs_return) {
		try {
			cv::CascadeClassifier* ret = new cv::CascadeClassifier();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::CascadeClassifier*>))
	}
	
	// CascadeClassifier(const cv::String &) /usr/include/opencv2/objdetect.hpp:240
	void cv_CascadeClassifier_CascadeClassifier_const_StringR(const char* filename, Result<cv::CascadeClassifier*>* ocvrs_return) {
		try {
			cv::CascadeClassifier* ret = new cv::CascadeClassifier(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::CascadeClassifier*>))
	}
	
	// empty() /usr/include/opencv2/objdetect.hpp:244
	void cv_CascadeClassifier_empty_const(const cv::CascadeClassifier* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->empty();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// load(const cv::String &) /usr/include/opencv2/objdetect.hpp:251
	void cv_CascadeClassifier_load_const_StringR(cv::CascadeClassifier* instance, const char* filename, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->load(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// read(const cv::FileNode &) /usr/include/opencv2/objdetect.hpp:256
	void cv_CascadeClassifier_read_const_FileNodeR(cv::CascadeClassifier* instance, const cv::FileNode* node, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->read(*node);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// detectMultiScale(cv::InputArray, std::vector<Rect> &, double, int, int, cv::Size, cv::Size) /usr/include/opencv2/objdetect.hpp:272
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_double_int_int_Size_Size(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, Result_void* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detectMultiScale(cv::InputArray, std::vector<Rect> &, std::vector<int> &, double, int, int, cv::Size, cv::Size) /usr/include/opencv2/objdetect.hpp:294
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_vector_int_R_double_int_int_Size_Size(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* numDetections, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, Result_void* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *numDetections, scaleFactor, minNeighbors, flags, *minSize, *maxSize);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detectMultiScale(cv::InputArray, std::vector<Rect> &, std::vector<int> &, std::vector<double> &, double, int, int, cv::Size, cv::Size, bool) /usr/include/opencv2/objdetect.hpp:319
	void cv_CascadeClassifier_detectMultiScale_const__InputArrayR_vector_Rect_R_vector_int_R_vector_double_R_double_int_int_Size_Size_bool(cv::CascadeClassifier* instance, const cv::_InputArray* image, std::vector<cv::Rect>* objects, std::vector<int>* rejectLevels, std::vector<double>* levelWeights, double scaleFactor, int minNeighbors, int flags, cv::Size* minSize, cv::Size* maxSize, bool outputRejectLevels, Result_void* ocvrs_return) {
		try {
			instance->detectMultiScale(*image, *objects, *rejectLevels, *levelWeights, scaleFactor, minNeighbors, flags, *minSize, *maxSize, outputRejectLevels);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// isOldFormatCascade() /usr/include/opencv2/objdetect.hpp:329
	void cv_CascadeClassifier_isOldFormatCascade_const(const cv::CascadeClassifier* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->isOldFormatCascade();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// getOriginalWindowSize() /usr/include/opencv2/objdetect.hpp:330
	void cv_CascadeClassifier_getOriginalWindowSize_const(const cv::CascadeClassifier* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getOriginalWindowSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// getFeatureType() /usr/include/opencv2/objdetect.hpp:331
	void cv_CascadeClassifier_getFeatureType_const(const cv::CascadeClassifier* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getFeatureType();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// getOldCascade() /usr/include/opencv2/objdetect.hpp:332
	void cv_CascadeClassifier_getOldCascade(cv::CascadeClassifier* instance, Result<void*>* ocvrs_return) {
		try {
			void* ret = instance->getOldCascade();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// convert(const cv::String &, const cv::String &) /usr/include/opencv2/objdetect.hpp:334
	void cv_CascadeClassifier_convert_const_StringR_const_StringR(const char* oldcascade, const char* newcascade, Result<bool>* ocvrs_return) {
		try {
			bool ret = cv::CascadeClassifier::convert(std::string(oldcascade), std::string(newcascade));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// setMaskGenerator(const Ptr<BaseCascadeClassifier::MaskGenerator> &) /usr/include/opencv2/objdetect.hpp:336
	void cv_CascadeClassifier_setMaskGenerator_const_Ptr_MaskGenerator_R(cv::CascadeClassifier* instance, const cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>* maskGenerator, Result_void* ocvrs_return) {
		try {
			instance->setMaskGenerator(*maskGenerator);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMaskGenerator() /usr/include/opencv2/objdetect.hpp:337
	void cv_CascadeClassifier_getMaskGenerator(cv::CascadeClassifier* instance, Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator> ret = instance->getMaskGenerator();
			Ok(new cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::BaseCascadeClassifier::MaskGenerator>*>))
	}
	
	void cv_DetectionBasedTracker_delete(cv::DetectionBasedTracker* instance) {
		delete instance;
	}
	// DetectionBasedTracker(cv::Ptr<IDetector>, cv::Ptr<IDetector>, const cv::DetectionBasedTracker::Parameters &) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:121
	void cv_DetectionBasedTracker_DetectionBasedTracker_Ptr_IDetector__Ptr_IDetector__const_ParametersR(cv::Ptr<cv::DetectionBasedTracker::IDetector>* mainDetector, cv::Ptr<cv::DetectionBasedTracker::IDetector>* trackingDetector, const cv::DetectionBasedTracker::Parameters* params, Result<cv::DetectionBasedTracker*>* ocvrs_return) {
		try {
			cv::DetectionBasedTracker* ret = new cv::DetectionBasedTracker(*mainDetector, *trackingDetector, *params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::DetectionBasedTracker*>))
	}
	
	// run() /usr/include/opencv2/objdetect/detection_based_tracker.hpp:124
	void cv_DetectionBasedTracker_run(cv::DetectionBasedTracker* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->run();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// stop() /usr/include/opencv2/objdetect/detection_based_tracker.hpp:125
	void cv_DetectionBasedTracker_stop(cv::DetectionBasedTracker* instance, Result_void* ocvrs_return) {
		try {
			instance->stop();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// resetTracking() /usr/include/opencv2/objdetect/detection_based_tracker.hpp:126
	void cv_DetectionBasedTracker_resetTracking(cv::DetectionBasedTracker* instance, Result_void* ocvrs_return) {
		try {
			instance->resetTracking();
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// process(const cv::Mat &) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:128
	void cv_DetectionBasedTracker_process_const_MatR(cv::DetectionBasedTracker* instance, const cv::Mat* imageGray, Result_void* ocvrs_return) {
		try {
			instance->process(*imageGray);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setParameters(const cv::DetectionBasedTracker::Parameters &) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:130
	void cv_DetectionBasedTracker_setParameters_const_ParametersR(cv::DetectionBasedTracker* instance, const cv::DetectionBasedTracker::Parameters* params, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->setParameters(*params);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// getParameters() /usr/include/opencv2/objdetect/detection_based_tracker.hpp:131
	void cv_DetectionBasedTracker_getParameters_const(const cv::DetectionBasedTracker* instance, Result<cv::DetectionBasedTracker::Parameters*>* ocvrs_return) {
		try {
			const cv::DetectionBasedTracker::Parameters ret = instance->getParameters();
			Ok(new const cv::DetectionBasedTracker::Parameters(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::DetectionBasedTracker::Parameters*>))
	}
	
	// getObjects(std::vector<cv::Rect> &) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:135
	void cv_DetectionBasedTracker_getObjects_const_vector_Rect_R(const cv::DetectionBasedTracker* instance, std::vector<cv::Rect>* result, Result_void* ocvrs_return) {
		try {
			instance->getObjects(*result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getObjects(std::vector<ExtObject> &) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:155
	void cv_DetectionBasedTracker_getObjects_const_vector_ExtObject_R(const cv::DetectionBasedTracker* instance, std::vector<cv::DetectionBasedTracker::ExtObject>* result, Result_void* ocvrs_return) {
		try {
			instance->getObjects(*result);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// addObject(const cv::Rect &) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:158
	void cv_DetectionBasedTracker_addObject_const_RectR(cv::DetectionBasedTracker* instance, const cv::Rect* location, Result<int>* ocvrs_return) {
		try {
			int ret = instance->addObject(*location);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// id /usr/include/opencv2/objdetect/detection_based_tracker.hpp:147
	int cv_DetectionBasedTracker_ExtObject_getPropId_const(const cv::DetectionBasedTracker::ExtObject* instance) {
			int ret = instance->id;
			return ret;
	}
	
	// id /usr/include/opencv2/objdetect/detection_based_tracker.hpp:147
	void cv_DetectionBasedTracker_ExtObject_setPropId_int(cv::DetectionBasedTracker::ExtObject* instance, int val) {
			instance->id = val;
	}
	
	// location /usr/include/opencv2/objdetect/detection_based_tracker.hpp:148
	void cv_DetectionBasedTracker_ExtObject_getPropLocation_const(const cv::DetectionBasedTracker::ExtObject* instance, cv::Rect* ocvrs_return) {
			cv::Rect ret = instance->location;
			*ocvrs_return = ret;
	}
	
	// location /usr/include/opencv2/objdetect/detection_based_tracker.hpp:148
	void cv_DetectionBasedTracker_ExtObject_setPropLocation_Rect(cv::DetectionBasedTracker::ExtObject* instance, cv::Rect* val) {
			instance->location = *val;
	}
	
	// status /usr/include/opencv2/objdetect/detection_based_tracker.hpp:149
	void cv_DetectionBasedTracker_ExtObject_getPropStatus_const(const cv::DetectionBasedTracker::ExtObject* instance, cv::DetectionBasedTracker::ObjectStatus* ocvrs_return) {
			cv::DetectionBasedTracker::ObjectStatus ret = instance->status;
			*ocvrs_return = ret;
	}
	
	// status /usr/include/opencv2/objdetect/detection_based_tracker.hpp:149
	void cv_DetectionBasedTracker_ExtObject_setPropStatus_ObjectStatus(cv::DetectionBasedTracker::ExtObject* instance, cv::DetectionBasedTracker::ObjectStatus val) {
			instance->status = val;
	}
	
	void cv_DetectionBasedTracker_ExtObject_delete(cv::DetectionBasedTracker::ExtObject* instance) {
		delete instance;
	}
	// ExtObject(int, cv::Rect, cv::DetectionBasedTracker::ObjectStatus) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:150
	void cv_DetectionBasedTracker_ExtObject_ExtObject_int_Rect_ObjectStatus(int _id, cv::Rect* _location, cv::DetectionBasedTracker::ObjectStatus _status, Result<cv::DetectionBasedTracker::ExtObject*>* ocvrs_return) {
		try {
			cv::DetectionBasedTracker::ExtObject* ret = new cv::DetectionBasedTracker::ExtObject(_id, *_location, _status);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::DetectionBasedTracker::ExtObject*>))
	}
	
	// detect(const cv::Mat &, std::vector<cv::Rect> &) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:78
	void cv_DetectionBasedTracker_IDetector_detect_const_MatR_vector_Rect_R(cv::DetectionBasedTracker::IDetector* instance, const cv::Mat* image, std::vector<cv::Rect>* objects, Result_void* ocvrs_return) {
		try {
			instance->detect(*image, *objects);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setMinObjectSize(const cv::Size &) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:80
	void cv_DetectionBasedTracker_IDetector_setMinObjectSize_const_SizeR(cv::DetectionBasedTracker::IDetector* instance, const cv::Size* min, Result_void* ocvrs_return) {
		try {
			instance->setMinObjectSize(*min);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setMaxObjectSize(const cv::Size &) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:84
	void cv_DetectionBasedTracker_IDetector_setMaxObjectSize_const_SizeR(cv::DetectionBasedTracker::IDetector* instance, const cv::Size* max, Result_void* ocvrs_return) {
		try {
			instance->setMaxObjectSize(*max);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMinObjectSize() /usr/include/opencv2/objdetect/detection_based_tracker.hpp:88
	void cv_DetectionBasedTracker_IDetector_getMinObjectSize_const(const cv::DetectionBasedTracker::IDetector* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getMinObjectSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// getMaxObjectSize() /usr/include/opencv2/objdetect/detection_based_tracker.hpp:92
	void cv_DetectionBasedTracker_IDetector_getMaxObjectSize_const(const cv::DetectionBasedTracker::IDetector* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getMaxObjectSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// getScaleFactor() /usr/include/opencv2/objdetect/detection_based_tracker.hpp:96
	void cv_DetectionBasedTracker_IDetector_getScaleFactor(cv::DetectionBasedTracker::IDetector* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScaleFactor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setScaleFactor(float) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:100
	void cv_DetectionBasedTracker_IDetector_setScaleFactor_float(cv::DetectionBasedTracker::IDetector* instance, float value, Result_void* ocvrs_return) {
		try {
			instance->setScaleFactor(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getMinNeighbours() /usr/include/opencv2/objdetect/detection_based_tracker.hpp:104
	void cv_DetectionBasedTracker_IDetector_getMinNeighbours(cv::DetectionBasedTracker::IDetector* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getMinNeighbours();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// setMinNeighbours(int) /usr/include/opencv2/objdetect/detection_based_tracker.hpp:108
	void cv_DetectionBasedTracker_IDetector_setMinNeighbours_int(cv::DetectionBasedTracker::IDetector* instance, int value, Result_void* ocvrs_return) {
		try {
			instance->setMinNeighbours(value);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// maxTrackLifetime /usr/include/opencv2/objdetect/detection_based_tracker.hpp:62
	int cv_DetectionBasedTracker_Parameters_getPropMaxTrackLifetime_const(const cv::DetectionBasedTracker::Parameters* instance) {
			int ret = instance->maxTrackLifetime;
			return ret;
	}
	
	// maxTrackLifetime /usr/include/opencv2/objdetect/detection_based_tracker.hpp:62
	void cv_DetectionBasedTracker_Parameters_setPropMaxTrackLifetime_int(cv::DetectionBasedTracker::Parameters* instance, int val) {
			instance->maxTrackLifetime = val;
	}
	
	// minDetectionPeriod /usr/include/opencv2/objdetect/detection_based_tracker.hpp:63
	int cv_DetectionBasedTracker_Parameters_getPropMinDetectionPeriod_const(const cv::DetectionBasedTracker::Parameters* instance) {
			int ret = instance->minDetectionPeriod;
			return ret;
	}
	
	// minDetectionPeriod /usr/include/opencv2/objdetect/detection_based_tracker.hpp:63
	void cv_DetectionBasedTracker_Parameters_setPropMinDetectionPeriod_int(cv::DetectionBasedTracker::Parameters* instance, int val) {
			instance->minDetectionPeriod = val;
	}
	
	void cv_DetectionBasedTracker_Parameters_delete(cv::DetectionBasedTracker::Parameters* instance) {
		delete instance;
	}
	// Parameters() /usr/include/opencv2/objdetect/detection_based_tracker.hpp:65
	void cv_DetectionBasedTracker_Parameters_Parameters(Result<cv::DetectionBasedTracker::Parameters*>* ocvrs_return) {
		try {
			cv::DetectionBasedTracker::Parameters* ret = new cv::DetectionBasedTracker::Parameters();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::DetectionBasedTracker::Parameters*>))
	}
	
	// scale /usr/include/opencv2/objdetect.hpp:353
	double cv_DetectionROI_getPropScale_const(const cv::DetectionROI* instance) {
			double ret = instance->scale;
			return ret;
	}
	
	// scale /usr/include/opencv2/objdetect.hpp:353
	void cv_DetectionROI_setPropScale_double(cv::DetectionROI* instance, double val) {
			instance->scale = val;
	}
	
	// locations /usr/include/opencv2/objdetect.hpp:355
	std::vector<cv::Point>* cv_DetectionROI_getPropLocations_const(const cv::DetectionROI* instance) {
			std::vector<cv::Point> ret = instance->locations;
			return new std::vector<cv::Point>(ret);
	}
	
	// locations /usr/include/opencv2/objdetect.hpp:355
	void cv_DetectionROI_setPropLocations_vector_Point_(cv::DetectionROI* instance, std::vector<cv::Point>* val) {
			instance->locations = *val;
	}
	
	// confidences /usr/include/opencv2/objdetect.hpp:357
	std::vector<double>* cv_DetectionROI_getPropConfidences_const(const cv::DetectionROI* instance) {
			std::vector<double> ret = instance->confidences;
			return new std::vector<double>(ret);
	}
	
	// confidences /usr/include/opencv2/objdetect.hpp:357
	void cv_DetectionROI_setPropConfidences_vector_double_(cv::DetectionROI* instance, std::vector<double>* val) {
			instance->confidences = *val;
	}
	
	void cv_DetectionROI_delete(cv::DetectionROI* instance) {
		delete instance;
	}
	// setInputSize(const cv::Size &) /usr/include/opencv2/objdetect/face.hpp:29
	void cv_FaceDetectorYN_setInputSize_const_SizeR(cv::FaceDetectorYN* instance, const cv::Size* input_size, Result_void* ocvrs_return) {
		try {
			instance->setInputSize(*input_size);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getInputSize() /usr/include/opencv2/objdetect/face.hpp:31
	void cv_FaceDetectorYN_getInputSize(cv::FaceDetectorYN* instance, Result<cv::Size>* ocvrs_return) {
		try {
			cv::Size ret = instance->getInputSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Size>))
	}
	
	// setScoreThreshold(float) /usr/include/opencv2/objdetect/face.hpp:37
	void cv_FaceDetectorYN_setScoreThreshold_float(cv::FaceDetectorYN* instance, float score_threshold, Result_void* ocvrs_return) {
		try {
			instance->setScoreThreshold(score_threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getScoreThreshold() /usr/include/opencv2/objdetect/face.hpp:39
	void cv_FaceDetectorYN_getScoreThreshold(cv::FaceDetectorYN* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getScoreThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setNMSThreshold(float) /usr/include/opencv2/objdetect/face.hpp:45
	void cv_FaceDetectorYN_setNMSThreshold_float(cv::FaceDetectorYN* instance, float nms_threshold, Result_void* ocvrs_return) {
		try {
			instance->setNMSThreshold(nms_threshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getNMSThreshold() /usr/include/opencv2/objdetect/face.hpp:47
	void cv_FaceDetectorYN_getNMSThreshold(cv::FaceDetectorYN* instance, Result<float>* ocvrs_return) {
		try {
			float ret = instance->getNMSThreshold();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<float>))
	}
	
	// setTopK(int) /usr/include/opencv2/objdetect/face.hpp:53
	void cv_FaceDetectorYN_setTopK_int(cv::FaceDetectorYN* instance, int top_k, Result_void* ocvrs_return) {
		try {
			instance->setTopK(top_k);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getTopK() /usr/include/opencv2/objdetect/face.hpp:55
	void cv_FaceDetectorYN_getTopK(cv::FaceDetectorYN* instance, Result<int>* ocvrs_return) {
		try {
			int ret = instance->getTopK();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// detect(cv::InputArray, cv::OutputArray) /usr/include/opencv2/objdetect/face.hpp:62
	void cv_FaceDetectorYN_detect_const__InputArrayR_const__OutputArrayR(cv::FaceDetectorYN* instance, const cv::_InputArray* image, const cv::_OutputArray* faces, Result<int>* ocvrs_return) {
		try {
			int ret = instance->detect(*image, *faces);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<int>))
	}
	
	// create(const cv::String &, const cv::String &, const cv::Size &, float, float, int, int, int) /usr/include/opencv2/objdetect/face.hpp:75
	void cv_FaceDetectorYN_create_const_StringR_const_StringR_const_SizeR_float_float_int_int_int(const char* model, const char* config, const cv::Size* input_size, float score_threshold, float nms_threshold, int top_k, int backend_id, int target_id, Result<cv::Ptr<cv::FaceDetectorYN>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FaceDetectorYN> ret = cv::FaceDetectorYN::create(std::string(model), std::string(config), *input_size, score_threshold, nms_threshold, top_k, backend_id, target_id);
			Ok(new cv::Ptr<cv::FaceDetectorYN>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::FaceDetectorYN>*>))
	}
	
	// alignCrop(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/objdetect/face.hpp:103
	void cv_FaceRecognizerSF_alignCrop_const_const__InputArrayR_const__InputArrayR_const__OutputArrayR(const cv::FaceRecognizerSF* instance, const cv::_InputArray* src_img, const cv::_InputArray* face_box, const cv::_OutputArray* aligned_img, Result_void* ocvrs_return) {
		try {
			instance->alignCrop(*src_img, *face_box, *aligned_img);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// feature(cv::InputArray, cv::OutputArray) /usr/include/opencv2/objdetect/face.hpp:109
	void cv_FaceRecognizerSF_feature_const__InputArrayR_const__OutputArrayR(cv::FaceRecognizerSF* instance, const cv::_InputArray* aligned_img, const cv::_OutputArray* face_feature, Result_void* ocvrs_return) {
		try {
			instance->feature(*aligned_img, *face_feature);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// match(cv::InputArray, cv::InputArray, int) /usr/include/opencv2/objdetect/face.hpp:116
	void cv_FaceRecognizerSF_match_const_const__InputArrayR_const__InputArrayR_int(const cv::FaceRecognizerSF* instance, const cv::_InputArray* face_feature1, const cv::_InputArray* face_feature2, int dis_type, Result<double>* ocvrs_return) {
		try {
			double ret = instance->match(*face_feature1, *face_feature2, dis_type);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// create(const cv::String &, const cv::String &, int, int) /usr/include/opencv2/objdetect/face.hpp:124
	void cv_FaceRecognizerSF_create_const_StringR_const_StringR_int_int(const char* model, const char* config, int backend_id, int target_id, Result<cv::Ptr<cv::FaceRecognizerSF>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::FaceRecognizerSF> ret = cv::FaceRecognizerSF::create(std::string(model), std::string(config), backend_id, target_id);
			Ok(new cv::Ptr<cv::FaceRecognizerSF>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::FaceRecognizerSF>*>))
	}
	
	// winSize /usr/include/opencv2/objdetect.hpp:596
	void cv_HOGDescriptor_getPropWinSize_const(const cv::HOGDescriptor* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->winSize;
			*ocvrs_return = ret;
	}
	
	// winSize /usr/include/opencv2/objdetect.hpp:596
	void cv_HOGDescriptor_setPropWinSize_Size(cv::HOGDescriptor* instance, cv::Size* val) {
			instance->winSize = *val;
	}
	
	// blockSize /usr/include/opencv2/objdetect.hpp:599
	void cv_HOGDescriptor_getPropBlockSize_const(const cv::HOGDescriptor* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->blockSize;
			*ocvrs_return = ret;
	}
	
	// blockSize /usr/include/opencv2/objdetect.hpp:599
	void cv_HOGDescriptor_setPropBlockSize_Size(cv::HOGDescriptor* instance, cv::Size* val) {
			instance->blockSize = *val;
	}
	
	// blockStride /usr/include/opencv2/objdetect.hpp:602
	void cv_HOGDescriptor_getPropBlockStride_const(const cv::HOGDescriptor* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->blockStride;
			*ocvrs_return = ret;
	}
	
	// blockStride /usr/include/opencv2/objdetect.hpp:602
	void cv_HOGDescriptor_setPropBlockStride_Size(cv::HOGDescriptor* instance, cv::Size* val) {
			instance->blockStride = *val;
	}
	
	// cellSize /usr/include/opencv2/objdetect.hpp:605
	void cv_HOGDescriptor_getPropCellSize_const(const cv::HOGDescriptor* instance, cv::Size* ocvrs_return) {
			cv::Size ret = instance->cellSize;
			*ocvrs_return = ret;
	}
	
	// cellSize /usr/include/opencv2/objdetect.hpp:605
	void cv_HOGDescriptor_setPropCellSize_Size(cv::HOGDescriptor* instance, cv::Size* val) {
			instance->cellSize = *val;
	}
	
	// nbins /usr/include/opencv2/objdetect.hpp:608
	int cv_HOGDescriptor_getPropNbins_const(const cv::HOGDescriptor* instance) {
			int ret = instance->nbins;
			return ret;
	}
	
	// nbins /usr/include/opencv2/objdetect.hpp:608
	void cv_HOGDescriptor_setPropNbins_int(cv::HOGDescriptor* instance, int val) {
			instance->nbins = val;
	}
	
	// derivAperture /usr/include/opencv2/objdetect.hpp:611
	int cv_HOGDescriptor_getPropDerivAperture_const(const cv::HOGDescriptor* instance) {
			int ret = instance->derivAperture;
			return ret;
	}
	
	// derivAperture /usr/include/opencv2/objdetect.hpp:611
	void cv_HOGDescriptor_setPropDerivAperture_int(cv::HOGDescriptor* instance, int val) {
			instance->derivAperture = val;
	}
	
	// winSigma /usr/include/opencv2/objdetect.hpp:614
	double cv_HOGDescriptor_getPropWinSigma_const(const cv::HOGDescriptor* instance) {
			double ret = instance->winSigma;
			return ret;
	}
	
	// winSigma /usr/include/opencv2/objdetect.hpp:614
	void cv_HOGDescriptor_setPropWinSigma_double(cv::HOGDescriptor* instance, double val) {
			instance->winSigma = val;
	}
	
	// histogramNormType /usr/include/opencv2/objdetect.hpp:617
	void cv_HOGDescriptor_getPropHistogramNormType_const(const cv::HOGDescriptor* instance, cv::HOGDescriptor::HistogramNormType* ocvrs_return) {
			cv::HOGDescriptor::HistogramNormType ret = instance->histogramNormType;
			*ocvrs_return = ret;
	}
	
	// histogramNormType /usr/include/opencv2/objdetect.hpp:617
	void cv_HOGDescriptor_setPropHistogramNormType_HistogramNormType(cv::HOGDescriptor* instance, cv::HOGDescriptor::HistogramNormType val) {
			instance->histogramNormType = val;
	}
	
	// L2HysThreshold /usr/include/opencv2/objdetect.hpp:620
	double cv_HOGDescriptor_getPropL2HysThreshold_const(const cv::HOGDescriptor* instance) {
			double ret = instance->L2HysThreshold;
			return ret;
	}
	
	// L2HysThreshold /usr/include/opencv2/objdetect.hpp:620
	void cv_HOGDescriptor_setPropL2HysThreshold_double(cv::HOGDescriptor* instance, double val) {
			instance->L2HysThreshold = val;
	}
	
	// gammaCorrection /usr/include/opencv2/objdetect.hpp:623
	bool cv_HOGDescriptor_getPropGammaCorrection_const(const cv::HOGDescriptor* instance) {
			bool ret = instance->gammaCorrection;
			return ret;
	}
	
	// gammaCorrection /usr/include/opencv2/objdetect.hpp:623
	void cv_HOGDescriptor_setPropGammaCorrection_bool(cv::HOGDescriptor* instance, bool val) {
			instance->gammaCorrection = val;
	}
	
	// svmDetector /usr/include/opencv2/objdetect.hpp:626
	std::vector<float>* cv_HOGDescriptor_getPropSvmDetector_const(const cv::HOGDescriptor* instance) {
			std::vector<float> ret = instance->svmDetector;
			return new std::vector<float>(ret);
	}
	
	// svmDetector /usr/include/opencv2/objdetect.hpp:626
	void cv_HOGDescriptor_setPropSvmDetector_vector_float_(cv::HOGDescriptor* instance, std::vector<float>* val) {
			instance->svmDetector = *val;
	}
	
	// oclSvmDetector /usr/include/opencv2/objdetect.hpp:629
	cv::UMat* cv_HOGDescriptor_getPropOclSvmDetector_const(const cv::HOGDescriptor* instance) {
			cv::UMat ret = instance->oclSvmDetector;
			return new cv::UMat(ret);
	}
	
	// oclSvmDetector /usr/include/opencv2/objdetect.hpp:629
	void cv_HOGDescriptor_setPropOclSvmDetector_UMat(cv::HOGDescriptor* instance, cv::UMat* val) {
			instance->oclSvmDetector = *val;
	}
	
	// free_coef /usr/include/opencv2/objdetect.hpp:632
	float cv_HOGDescriptor_getPropFree_coef_const(const cv::HOGDescriptor* instance) {
			float ret = instance->free_coef;
			return ret;
	}
	
	// free_coef /usr/include/opencv2/objdetect.hpp:632
	void cv_HOGDescriptor_setPropFree_coef_float(cv::HOGDescriptor* instance, float val) {
			instance->free_coef = val;
	}
	
	// nlevels /usr/include/opencv2/objdetect.hpp:635
	int cv_HOGDescriptor_getPropNlevels_const(const cv::HOGDescriptor* instance) {
			int ret = instance->nlevels;
			return ret;
	}
	
	// nlevels /usr/include/opencv2/objdetect.hpp:635
	void cv_HOGDescriptor_setPropNlevels_int(cv::HOGDescriptor* instance, int val) {
			instance->nlevels = val;
	}
	
	// signedGradient /usr/include/opencv2/objdetect.hpp:638
	bool cv_HOGDescriptor_getPropSignedGradient_const(const cv::HOGDescriptor* instance) {
			bool ret = instance->signedGradient;
			return ret;
	}
	
	// signedGradient /usr/include/opencv2/objdetect.hpp:638
	void cv_HOGDescriptor_setPropSignedGradient_bool(cv::HOGDescriptor* instance, bool val) {
			instance->signedGradient = val;
	}
	
	void cv_HOGDescriptor_delete(cv::HOGDescriptor* instance) {
		delete instance;
	}
	// HOGDescriptor() /usr/include/opencv2/objdetect.hpp:390
	void cv_HOGDescriptor_HOGDescriptor(Result<cv::HOGDescriptor*>* ocvrs_return) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::HOGDescriptor*>))
	}
	
	// HOGDescriptor(cv::Size, cv::Size, cv::Size, cv::Size, int, int, double, HOGDescriptor::HistogramNormType, double, bool, int, bool) /usr/include/opencv2/objdetect.hpp:410
	void cv_HOGDescriptor_HOGDescriptor_Size_Size_Size_Size_int_int_double_HistogramNormType_double_bool_int_bool(cv::Size* _winSize, cv::Size* _blockSize, cv::Size* _blockStride, cv::Size* _cellSize, int _nbins, int _derivAperture, double _winSigma, cv::HOGDescriptor::HistogramNormType _histogramNormType, double _L2HysThreshold, bool _gammaCorrection, int _nlevels, bool _signedGradient, Result<cv::HOGDescriptor*>* ocvrs_return) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(*_winSize, *_blockSize, *_blockStride, *_cellSize, _nbins, _derivAperture, _winSigma, _histogramNormType, _L2HysThreshold, _gammaCorrection, _nlevels, _signedGradient);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::HOGDescriptor*>))
	}
	
	// HOGDescriptor(const cv::String &) /usr/include/opencv2/objdetect.hpp:426
	void cv_HOGDescriptor_HOGDescriptor_const_StringR(const char* filename, Result<cv::HOGDescriptor*>* ocvrs_return) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(std::string(filename));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::HOGDescriptor*>))
	}
	
	// HOGDescriptor(const cv::HOGDescriptor &) /usr/include/opencv2/objdetect.hpp:434
	void cv_HOGDescriptor_HOGDescriptor_const_HOGDescriptorR(const cv::HOGDescriptor* d, Result<cv::HOGDescriptor*>* ocvrs_return) {
		try {
			cv::HOGDescriptor* ret = new cv::HOGDescriptor(*d);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::HOGDescriptor*>))
	}
	
	// getDescriptorSize() /usr/include/opencv2/objdetect.hpp:445
	void cv_HOGDescriptor_getDescriptorSize_const(const cv::HOGDescriptor* instance, Result<size_t>* ocvrs_return) {
		try {
			size_t ret = instance->getDescriptorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<size_t>))
	}
	
	// checkDetectorSize() /usr/include/opencv2/objdetect.hpp:449
	void cv_HOGDescriptor_checkDetectorSize_const(const cv::HOGDescriptor* instance, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->checkDetectorSize();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// getWinSigma() /usr/include/opencv2/objdetect.hpp:453
	void cv_HOGDescriptor_getWinSigma_const(const cv::HOGDescriptor* instance, Result<double>* ocvrs_return) {
		try {
			double ret = instance->getWinSigma();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<double>))
	}
	
	// setSVMDetector(cv::InputArray) /usr/include/opencv2/objdetect.hpp:460
	void cv_HOGDescriptor_setSVMDetector_const__InputArrayR(cv::HOGDescriptor* instance, const cv::_InputArray* svmdetector, Result_void* ocvrs_return) {
		try {
			instance->setSVMDetector(*svmdetector);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// read(cv::FileNode &) /usr/include/opencv2/objdetect.hpp:465
	void cv_HOGDescriptor_read_FileNodeR(cv::HOGDescriptor* instance, cv::FileNode* fn, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->read(*fn);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// write(cv::FileStorage &, const cv::String &) /usr/include/opencv2/objdetect.hpp:471
	void cv_HOGDescriptor_write_const_FileStorageR_const_StringR(const cv::HOGDescriptor* instance, cv::FileStorage* fs, const char* objname, Result_void* ocvrs_return) {
		try {
			instance->write(*fs, std::string(objname));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// load(const cv::String &, const cv::String &) /usr/include/opencv2/objdetect.hpp:477
	void cv_HOGDescriptor_load_const_StringR_const_StringR(cv::HOGDescriptor* instance, const char* filename, const char* objname, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->load(std::string(filename), std::string(objname));
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// save(const cv::String &, const cv::String &) /usr/include/opencv2/objdetect.hpp:483
	void cv_HOGDescriptor_save_const_const_StringR_const_StringR(const cv::HOGDescriptor* instance, const char* filename, const char* objname, Result_void* ocvrs_return) {
		try {
			instance->save(std::string(filename), std::string(objname));
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// copyTo(cv::HOGDescriptor &) /usr/include/opencv2/objdetect.hpp:488
	void cv_HOGDescriptor_copyTo_const_HOGDescriptorR(const cv::HOGDescriptor* instance, cv::HOGDescriptor* c, Result_void* ocvrs_return) {
		try {
			instance->copyTo(*c);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// compute(cv::InputArray, std::vector<float> &, cv::Size, cv::Size, const std::vector<Point> &) /usr/include/opencv2/objdetect.hpp:499
	void cv_HOGDescriptor_compute_const_const__InputArrayR_vector_float_R_Size_Size_const_vector_Point_R(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<float>* descriptors, cv::Size* winStride, cv::Size* padding, const std::vector<cv::Point>* locations, Result_void* ocvrs_return) {
		try {
			instance->compute(*img, *descriptors, *winStride, *padding, *locations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detect(cv::InputArray, std::vector<Point> &, std::vector<double> &, double, cv::Size, cv::Size, const std::vector<Point> &) /usr/include/opencv2/objdetect.hpp:515
	void cv_HOGDescriptor_detect_const_const__InputArrayR_vector_Point_R_vector_double_R_double_Size_Size_const_vector_Point_R(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Point>* foundLocations, std::vector<double>* weights, double hitThreshold, cv::Size* winStride, cv::Size* padding, const std::vector<cv::Point>* searchLocations, Result_void* ocvrs_return) {
		try {
			instance->detect(*img, *foundLocations, *weights, hitThreshold, *winStride, *padding, *searchLocations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detect(cv::InputArray, std::vector<Point> &, double, cv::Size, cv::Size, const std::vector<Point> &) /usr/include/opencv2/objdetect.hpp:531
	void cv_HOGDescriptor_detect_const_const__InputArrayR_vector_Point_R_double_Size_Size_const_vector_Point_R(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Point>* foundLocations, double hitThreshold, cv::Size* winStride, cv::Size* padding, const std::vector<cv::Point>* searchLocations, Result_void* ocvrs_return) {
		try {
			instance->detect(*img, *foundLocations, hitThreshold, *winStride, *padding, *searchLocations);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detectMultiScale(cv::InputArray, std::vector<Rect> &, std::vector<double> &, double, cv::Size, cv::Size, double, double, bool) /usr/include/opencv2/objdetect.hpp:551
	void cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vector_Rect_R_vector_double_R_double_Size_Size_double_double_bool(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, std::vector<double>* foundWeights, double hitThreshold, cv::Size* winStride, cv::Size* padding, double scale, double groupThreshold, bool useMeanshiftGrouping, Result_void* ocvrs_return) {
		try {
			instance->detectMultiScale(*img, *foundLocations, *foundWeights, hitThreshold, *winStride, *padding, scale, groupThreshold, useMeanshiftGrouping);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detectMultiScale(cv::InputArray, std::vector<Rect> &, double, cv::Size, cv::Size, double, double, bool) /usr/include/opencv2/objdetect.hpp:570
	void cv_HOGDescriptor_detectMultiScale_const_const__InputArrayR_vector_Rect_R_double_Size_Size_double_double_bool(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, double hitThreshold, cv::Size* winStride, cv::Size* padding, double scale, double groupThreshold, bool useMeanshiftGrouping, Result_void* ocvrs_return) {
		try {
			instance->detectMultiScale(*img, *foundLocations, hitThreshold, *winStride, *padding, scale, groupThreshold, useMeanshiftGrouping);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// computeGradient(cv::InputArray, cv::InputOutputArray, cv::InputOutputArray, cv::Size, cv::Size) /usr/include/opencv2/objdetect.hpp:582
	void cv_HOGDescriptor_computeGradient_const_const__InputArrayR_const__InputOutputArrayR_const__InputOutputArrayR_Size_Size(const cv::HOGDescriptor* instance, const cv::_InputArray* img, const cv::_InputOutputArray* grad, const cv::_InputOutputArray* angleOfs, cv::Size* paddingTL, cv::Size* paddingBR, Result_void* ocvrs_return) {
		try {
			instance->computeGradient(*img, *grad, *angleOfs, *paddingTL, *paddingBR);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// getDefaultPeopleDetector() /usr/include/opencv2/objdetect.hpp:587
	void cv_HOGDescriptor_getDefaultPeopleDetector(Result<std::vector<float>*>* ocvrs_return) {
		try {
			std::vector<float> ret = cv::HOGDescriptor::getDefaultPeopleDetector();
			Ok(new std::vector<float>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<float>*>))
	}
	
	// getDaimlerPeopleDetector() /usr/include/opencv2/objdetect.hpp:593
	void cv_HOGDescriptor_getDaimlerPeopleDetector(Result<std::vector<float>*>* ocvrs_return) {
		try {
			std::vector<float> ret = cv::HOGDescriptor::getDaimlerPeopleDetector();
			Ok(new std::vector<float>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<std::vector<float>*>))
	}
	
	// detectROI(cv::InputArray, const std::vector<cv::Point> &, std::vector<cv::Point> &, std::vector<double> &, double, cv::Size, cv::Size) /usr/include/opencv2/objdetect.hpp:651
	void cv_HOGDescriptor_detectROI_const_const__InputArrayR_const_vector_Point_R_vector_Point_R_vector_double_R_double_Size_Size(const cv::HOGDescriptor* instance, const cv::_InputArray* img, const std::vector<cv::Point>* locations, std::vector<cv::Point>* foundLocations, std::vector<double>* confidences, double hitThreshold, cv::Size* winStride, cv::Size* padding, Result_void* ocvrs_return) {
		try {
			instance->detectROI(*img, *locations, *foundLocations, *confidences, hitThreshold, *winStride, *padding);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detectMultiScaleROI(cv::InputArray, std::vector<cv::Rect> &, std::vector<DetectionROI> &, double, int) /usr/include/opencv2/objdetect.hpp:664
	void cv_HOGDescriptor_detectMultiScaleROI_const_const__InputArrayR_vector_Rect_R_vector_DetectionROI_R_double_int(const cv::HOGDescriptor* instance, const cv::_InputArray* img, std::vector<cv::Rect>* foundLocations, std::vector<cv::DetectionROI>* locations, double hitThreshold, int groupThreshold, Result_void* ocvrs_return) {
		try {
			instance->detectMultiScaleROI(*img, *foundLocations, *locations, hitThreshold, groupThreshold);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// groupRectangles(std::vector<cv::Rect> &, std::vector<double> &, int, double) /usr/include/opencv2/objdetect.hpp:676
	void cv_HOGDescriptor_groupRectangles_const_vector_Rect_R_vector_double_R_int_double(const cv::HOGDescriptor* instance, std::vector<cv::Rect>* rectList, std::vector<double>* weights, int groupThreshold, double eps, Result_void* ocvrs_return) {
		try {
			instance->groupRectangles(*rectList, *weights, groupThreshold, eps);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	void cv_QRCodeDetector_delete(cv::QRCodeDetector* instance) {
		delete instance;
	}
	// QRCodeDetector() /usr/include/opencv2/objdetect.hpp:749
	void cv_QRCodeDetector_QRCodeDetector(Result<cv::QRCodeDetector*>* ocvrs_return) {
		try {
			cv::QRCodeDetector* ret = new cv::QRCodeDetector();
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::QRCodeDetector*>))
	}
	
	// setEpsX(double) /usr/include/opencv2/objdetect.hpp:756
	void cv_QRCodeDetector_setEpsX_double(cv::QRCodeDetector* instance, double epsX, Result_void* ocvrs_return) {
		try {
			instance->setEpsX(epsX);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// setEpsY(double) /usr/include/opencv2/objdetect.hpp:761
	void cv_QRCodeDetector_setEpsY_double(cv::QRCodeDetector* instance, double epsY, Result_void* ocvrs_return) {
		try {
			instance->setEpsY(epsY);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// detect(cv::InputArray, cv::OutputArray) /usr/include/opencv2/objdetect.hpp:767
	void cv_QRCodeDetector_detect_const_const__InputArrayR_const__OutputArrayR(const cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->detect(*img, *points);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// decode(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/objdetect.hpp:776
	void cv_QRCodeDetector_decode_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, const cv::_OutputArray* straight_qrcode, Result<void*>* ocvrs_return) {
		try {
			std::string ret = instance->decode(*img, *points, *straight_qrcode);
			Ok(ocvrs_create_byte_string(ret.data(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// decodeCurved(cv::InputArray, cv::InputArray, cv::OutputArray) /usr/include/opencv2/objdetect.hpp:785
	void cv_QRCodeDetector_decodeCurved_const__InputArrayR_const__InputArrayR_const__OutputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, const cv::_OutputArray* straight_qrcode, Result<void*>* ocvrs_return) {
		try {
			cv::String ret = instance->decodeCurved(*img, *points, *straight_qrcode);
			Ok(ocvrs_create_byte_string(ret.data(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// detectAndDecode(cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/objdetect.hpp:793
	void cv_QRCodeDetector_detectAndDecode_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points, const cv::_OutputArray* straight_qrcode, Result<void*>* ocvrs_return) {
		try {
			std::string ret = instance->detectAndDecode(*img, *points, *straight_qrcode);
			Ok(ocvrs_create_byte_string(ret.data(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// detectAndDecodeCurved(cv::InputArray, cv::OutputArray, cv::OutputArray) /usr/include/opencv2/objdetect.hpp:802
	void cv_QRCodeDetector_detectAndDecodeCurved_const__InputArrayR_const__OutputArrayR_const__OutputArrayR(cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points, const cv::_OutputArray* straight_qrcode, Result<void*>* ocvrs_return) {
		try {
			std::string ret = instance->detectAndDecodeCurved(*img, *points, *straight_qrcode);
			Ok(ocvrs_create_byte_string(ret.data(), ret.size()), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<void*>))
	}
	
	// detectMulti(cv::InputArray, cv::OutputArray) /usr/include/opencv2/objdetect.hpp:810
	void cv_QRCodeDetector_detectMulti_const_const__InputArrayR_const__OutputArrayR(const cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_OutputArray* points, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->detectMulti(*img, *points);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// decodeMulti(cv::InputArray, cv::InputArray, std::vector<std::string> &, cv::OutputArrayOfArrays) /usr/include/opencv2/objdetect.hpp:819
	void cv_QRCodeDetector_decodeMulti_const_const__InputArrayR_const__InputArrayR_vector_string_R_const__OutputArrayR(const cv::QRCodeDetector* instance, const cv::_InputArray* img, const cv::_InputArray* points, std::vector<std::string>* decoded_info, const cv::_OutputArray* straight_qrcode, Result<bool>* ocvrs_return) {
		try {
			bool ret = instance->decodeMulti(*img, *points, *decoded_info, *straight_qrcode);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<bool>))
	}
	
	// create(const QRCodeEncoder::Params &) /usr/include/opencv2/objdetect.hpp:730
	void cv_QRCodeEncoder_create_const_ParamsR(const cv::QRCodeEncoder::Params* parameters, Result<cv::Ptr<cv::QRCodeEncoder>*>* ocvrs_return) {
		try {
			cv::Ptr<cv::QRCodeEncoder> ret = cv::QRCodeEncoder::create(*parameters);
			Ok(new cv::Ptr<cv::QRCodeEncoder>(ret), ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::Ptr<cv::QRCodeEncoder>*>))
	}
	
	// encode(const cv::String &, cv::OutputArray) /usr/include/opencv2/objdetect.hpp:736
	void cv_QRCodeEncoder_encode_const_StringR_const__OutputArrayR(cv::QRCodeEncoder* instance, const char* encoded_info, const cv::_OutputArray* qrcode, Result_void* ocvrs_return) {
		try {
			instance->encode(std::string(encoded_info), *qrcode);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// encodeStructuredAppend(const cv::String &, cv::OutputArrayOfArrays) /usr/include/opencv2/objdetect.hpp:742
	void cv_QRCodeEncoder_encodeStructuredAppend_const_StringR_const__OutputArrayR(cv::QRCodeEncoder* instance, const char* encoded_info, const cv::_OutputArray* qrcodes, Result_void* ocvrs_return) {
		try {
			instance->encodeStructuredAppend(std::string(encoded_info), *qrcodes);
			Ok(ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result_void))
	}
	
	// Params() /usr/include/opencv2/objdetect.hpp:719
	void cv_QRCodeEncoder_Params_Params(Result<cv::QRCodeEncoder::Params>* ocvrs_return) {
		try {
			cv::QRCodeEncoder::Params ret;
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::QRCodeEncoder::Params>))
	}
	
	// eps /usr/include/opencv2/objdetect.hpp:137
	double cv_SimilarRects_getPropEps_const(const cv::SimilarRects* instance) {
			double ret = instance->eps;
			return ret;
	}
	
	// eps /usr/include/opencv2/objdetect.hpp:137
	void cv_SimilarRects_setPropEps_double(cv::SimilarRects* instance, double val) {
			instance->eps = val;
	}
	
	void cv_SimilarRects_delete(cv::SimilarRects* instance) {
		delete instance;
	}
	// SimilarRects(double) /usr/include/opencv2/objdetect.hpp:128
	void cv_SimilarRects_SimilarRects_double(double _eps, Result<cv::SimilarRects*>* ocvrs_return) {
		try {
			cv::SimilarRects* ret = new cv::SimilarRects(_eps);
			Ok(ret, ocvrs_return);
		} OCVRS_CATCH(OCVRS_TYPE(Result<cv::SimilarRects*>))
	}
	
}
