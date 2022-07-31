extern "C" {
	cv::Ptr<cv::videostab::MotionStabilizationPipeline>* cv_PtrOfMotionStabilizationPipeline_new(cv::videostab::MotionStabilizationPipeline* val) {
		return new cv::Ptr<cv::videostab::MotionStabilizationPipeline>(val);
	}
	
	void cv_PtrOfMotionStabilizationPipeline_delete(cv::Ptr<cv::videostab::MotionStabilizationPipeline>* instance) {
		delete instance;
	}

	const cv::videostab::MotionStabilizationPipeline* cv_PtrOfMotionStabilizationPipeline_get_inner_ptr(const cv::Ptr<cv::videostab::MotionStabilizationPipeline>* instance) {
		return instance->get();
	}

	cv::videostab::MotionStabilizationPipeline* cv_PtrOfMotionStabilizationPipeline_get_inner_ptr_mut(cv::Ptr<cv::videostab::MotionStabilizationPipeline>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IMotionStabilizer>* cv_PtrOfMotionStabilizationPipeline_to_PtrOfIMotionStabilizer(cv::Ptr<cv::videostab::MotionStabilizationPipeline>* instance) {
		return new cv::Ptr<cv::videostab::IMotionStabilizer>(instance->dynamicCast<cv::videostab::IMotionStabilizer>());
	}
}

