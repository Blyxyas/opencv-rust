extern "C" {
	cv::Ptr<cv::videostab::LpMotionStabilizer>* cv_PtrOfLpMotionStabilizer_new(cv::videostab::LpMotionStabilizer* val) {
		return new cv::Ptr<cv::videostab::LpMotionStabilizer>(val);
	}
	
	void cv_PtrOfLpMotionStabilizer_delete(cv::Ptr<cv::videostab::LpMotionStabilizer>* instance) {
		delete instance;
	}

	const cv::videostab::LpMotionStabilizer* cv_PtrOfLpMotionStabilizer_get_inner_ptr(const cv::Ptr<cv::videostab::LpMotionStabilizer>* instance) {
		return instance->get();
	}

	cv::videostab::LpMotionStabilizer* cv_PtrOfLpMotionStabilizer_get_inner_ptr_mut(cv::Ptr<cv::videostab::LpMotionStabilizer>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IMotionStabilizer>* cv_PtrOfLpMotionStabilizer_to_PtrOfIMotionStabilizer(cv::Ptr<cv::videostab::LpMotionStabilizer>* instance) {
		return new cv::Ptr<cv::videostab::IMotionStabilizer>(instance->dynamicCast<cv::videostab::IMotionStabilizer>());
	}
}

