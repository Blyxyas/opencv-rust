extern "C" {
	cv::Ptr<cv::videostab::GaussianMotionFilter>* cv_PtrOfGaussianMotionFilter_new(cv::videostab::GaussianMotionFilter* val) {
		return new cv::Ptr<cv::videostab::GaussianMotionFilter>(val);
	}
	
	void cv_PtrOfGaussianMotionFilter_delete(cv::Ptr<cv::videostab::GaussianMotionFilter>* instance) {
		delete instance;
	}

	const cv::videostab::GaussianMotionFilter* cv_PtrOfGaussianMotionFilter_get_inner_ptr(const cv::Ptr<cv::videostab::GaussianMotionFilter>* instance) {
		return instance->get();
	}

	cv::videostab::GaussianMotionFilter* cv_PtrOfGaussianMotionFilter_get_inner_ptr_mut(cv::Ptr<cv::videostab::GaussianMotionFilter>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IMotionStabilizer>* cv_PtrOfGaussianMotionFilter_to_PtrOfIMotionStabilizer(cv::Ptr<cv::videostab::GaussianMotionFilter>* instance) {
		return new cv::Ptr<cv::videostab::IMotionStabilizer>(instance->dynamicCast<cv::videostab::IMotionStabilizer>());
	}
	
	cv::Ptr<cv::videostab::MotionFilterBase>* cv_PtrOfGaussianMotionFilter_to_PtrOfMotionFilterBase(cv::Ptr<cv::videostab::GaussianMotionFilter>* instance) {
		return new cv::Ptr<cv::videostab::MotionFilterBase>(instance->dynamicCast<cv::videostab::MotionFilterBase>());
	}
}

