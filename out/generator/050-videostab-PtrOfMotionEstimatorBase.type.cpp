extern "C" {
	void cv_PtrOfMotionEstimatorBase_delete(cv::Ptr<cv::videostab::MotionEstimatorBase>* instance) {
		delete instance;
	}

	const cv::videostab::MotionEstimatorBase* cv_PtrOfMotionEstimatorBase_get_inner_ptr(const cv::Ptr<cv::videostab::MotionEstimatorBase>* instance) {
		return instance->get();
	}

	cv::videostab::MotionEstimatorBase* cv_PtrOfMotionEstimatorBase_get_inner_ptr_mut(cv::Ptr<cv::videostab::MotionEstimatorBase>* instance) {
		return instance->get();
	}
}

