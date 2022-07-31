extern "C" {
	cv::Ptr<cv::videostab::ToFileMotionWriter>* cv_PtrOfToFileMotionWriter_new(cv::videostab::ToFileMotionWriter* val) {
		return new cv::Ptr<cv::videostab::ToFileMotionWriter>(val);
	}
	
	void cv_PtrOfToFileMotionWriter_delete(cv::Ptr<cv::videostab::ToFileMotionWriter>* instance) {
		delete instance;
	}

	const cv::videostab::ToFileMotionWriter* cv_PtrOfToFileMotionWriter_get_inner_ptr(const cv::Ptr<cv::videostab::ToFileMotionWriter>* instance) {
		return instance->get();
	}

	cv::videostab::ToFileMotionWriter* cv_PtrOfToFileMotionWriter_get_inner_ptr_mut(cv::Ptr<cv::videostab::ToFileMotionWriter>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::ImageMotionEstimatorBase>* cv_PtrOfToFileMotionWriter_to_PtrOfImageMotionEstimatorBase(cv::Ptr<cv::videostab::ToFileMotionWriter>* instance) {
		return new cv::Ptr<cv::videostab::ImageMotionEstimatorBase>(instance->dynamicCast<cv::videostab::ImageMotionEstimatorBase>());
	}
}

