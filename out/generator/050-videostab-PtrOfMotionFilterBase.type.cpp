extern "C" {
	void cv_PtrOfMotionFilterBase_delete(cv::Ptr<cv::videostab::MotionFilterBase>* instance) {
		delete instance;
	}

	const cv::videostab::MotionFilterBase* cv_PtrOfMotionFilterBase_get_inner_ptr(const cv::Ptr<cv::videostab::MotionFilterBase>* instance) {
		return instance->get();
	}

	cv::videostab::MotionFilterBase* cv_PtrOfMotionFilterBase_get_inner_ptr_mut(cv::Ptr<cv::videostab::MotionFilterBase>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IMotionStabilizer>* cv_PtrOfMotionFilterBase_to_PtrOfIMotionStabilizer(cv::Ptr<cv::videostab::MotionFilterBase>* instance) {
		return new cv::Ptr<cv::videostab::IMotionStabilizer>(instance->dynamicCast<cv::videostab::IMotionStabilizer>());
	}
}

