extern "C" {
	void cv_PtrOfIMotionStabilizer_delete(cv::Ptr<cv::videostab::IMotionStabilizer>* instance) {
		delete instance;
	}

	const cv::videostab::IMotionStabilizer* cv_PtrOfIMotionStabilizer_get_inner_ptr(const cv::Ptr<cv::videostab::IMotionStabilizer>* instance) {
		return instance->get();
	}

	cv::videostab::IMotionStabilizer* cv_PtrOfIMotionStabilizer_get_inner_ptr_mut(cv::Ptr<cv::videostab::IMotionStabilizer>* instance) {
		return instance->get();
	}
}

