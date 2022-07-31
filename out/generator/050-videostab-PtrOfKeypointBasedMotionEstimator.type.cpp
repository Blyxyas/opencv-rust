extern "C" {
	cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* cv_PtrOfKeypointBasedMotionEstimator_new(cv::videostab::KeypointBasedMotionEstimator* val) {
		return new cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>(val);
	}
	
	void cv_PtrOfKeypointBasedMotionEstimator_delete(cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* instance) {
		delete instance;
	}

	const cv::videostab::KeypointBasedMotionEstimator* cv_PtrOfKeypointBasedMotionEstimator_get_inner_ptr(const cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* instance) {
		return instance->get();
	}

	cv::videostab::KeypointBasedMotionEstimator* cv_PtrOfKeypointBasedMotionEstimator_get_inner_ptr_mut(cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* cv_PtrOfKeypointBasedMotionEstimator_to_PtrOfImageMotionEstimatorBase(cv::Ptr<cv::videostab::KeypointBasedMotionEstimator>* instance) {
		return new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(instance->dynamicCast<cv::videostab::ImageMotionEstimatorBase>());
	}
}

