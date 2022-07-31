extern "C" {
	cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* cv_PtrOfMotionEstimatorRansacL2_new(cv::videostab::MotionEstimatorRansacL2* val) {
		return new cv::Ptr<cv::videostab::MotionEstimatorRansacL2>(val);
	}
	
	void cv_PtrOfMotionEstimatorRansacL2_delete(cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* instance) {
		delete instance;
	}

	const cv::videostab::MotionEstimatorRansacL2* cv_PtrOfMotionEstimatorRansacL2_get_inner_ptr(const cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* instance) {
		return instance->get();
	}

	cv::videostab::MotionEstimatorRansacL2* cv_PtrOfMotionEstimatorRansacL2_get_inner_ptr_mut(cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::MotionEstimatorBase>* cv_PtrOfMotionEstimatorRansacL2_to_PtrOfMotionEstimatorBase(cv::Ptr<cv::videostab::MotionEstimatorRansacL2>* instance) {
		return new cv::Ptr<cv::videostab::MotionEstimatorBase>(instance->dynamicCast<cv::videostab::MotionEstimatorBase>());
	}
}

