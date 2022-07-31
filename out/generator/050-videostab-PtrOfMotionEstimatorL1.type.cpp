extern "C" {
	cv::Ptr<cv::videostab::MotionEstimatorL1>* cv_PtrOfMotionEstimatorL1_new(cv::videostab::MotionEstimatorL1* val) {
		return new cv::Ptr<cv::videostab::MotionEstimatorL1>(val);
	}
	
	void cv_PtrOfMotionEstimatorL1_delete(cv::Ptr<cv::videostab::MotionEstimatorL1>* instance) {
		delete instance;
	}

	const cv::videostab::MotionEstimatorL1* cv_PtrOfMotionEstimatorL1_get_inner_ptr(const cv::Ptr<cv::videostab::MotionEstimatorL1>* instance) {
		return instance->get();
	}

	cv::videostab::MotionEstimatorL1* cv_PtrOfMotionEstimatorL1_get_inner_ptr_mut(cv::Ptr<cv::videostab::MotionEstimatorL1>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::MotionEstimatorBase>* cv_PtrOfMotionEstimatorL1_to_PtrOfMotionEstimatorBase(cv::Ptr<cv::videostab::MotionEstimatorL1>* instance) {
		return new cv::Ptr<cv::videostab::MotionEstimatorBase>(instance->dynamicCast<cv::videostab::MotionEstimatorBase>());
	}
}

