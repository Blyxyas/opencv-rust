extern "C" {
	cv::Ptr<cv::videostab::FromFileMotionReader>* cv_PtrOfFromFileMotionReader_new(cv::videostab::FromFileMotionReader* val) {
		return new cv::Ptr<cv::videostab::FromFileMotionReader>(val);
	}
	
	void cv_PtrOfFromFileMotionReader_delete(cv::Ptr<cv::videostab::FromFileMotionReader>* instance) {
		delete instance;
	}

	const cv::videostab::FromFileMotionReader* cv_PtrOfFromFileMotionReader_get_inner_ptr(const cv::Ptr<cv::videostab::FromFileMotionReader>* instance) {
		return instance->get();
	}

	cv::videostab::FromFileMotionReader* cv_PtrOfFromFileMotionReader_get_inner_ptr_mut(cv::Ptr<cv::videostab::FromFileMotionReader>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* cv_PtrOfFromFileMotionReader_to_PtrOfImageMotionEstimatorBase(cv::Ptr<cv::videostab::FromFileMotionReader>* instance) {
		return new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(instance->dynamicCast<cv::videostab::ImageMotionEstimatorBase>());
	}
}

