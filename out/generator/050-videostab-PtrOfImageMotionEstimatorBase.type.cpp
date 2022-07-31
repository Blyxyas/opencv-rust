extern "C" {
	void cv_PtrOfImageMotionEstimatorBase_delete(cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* instance) {
		delete instance;
	}

	const cv::videostab::ImageMotionEstimatorBase* cv_PtrOfImageMotionEstimatorBase_get_inner_ptr(const cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* instance) {
		return instance->get();
	}

	cv::videostab::ImageMotionEstimatorBase* cv_PtrOfImageMotionEstimatorBase_get_inner_ptr_mut(cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* instance) {
		return instance->get();
	}
}

