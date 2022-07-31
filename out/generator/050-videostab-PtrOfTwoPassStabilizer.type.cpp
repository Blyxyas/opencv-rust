extern "C" {
	cv::Ptr<cv::videostab::TwoPassStabilizer>* cv_PtrOfTwoPassStabilizer_new(cv::videostab::TwoPassStabilizer* val) {
		return new cv::Ptr<cv::videostab::TwoPassStabilizer>(val);
	}
	
	void cv_PtrOfTwoPassStabilizer_delete(cv::Ptr<cv::videostab::TwoPassStabilizer>* instance) {
		delete instance;
	}

	const cv::videostab::TwoPassStabilizer* cv_PtrOfTwoPassStabilizer_get_inner_ptr(const cv::Ptr<cv::videostab::TwoPassStabilizer>* instance) {
		return instance->get();
	}

	cv::videostab::TwoPassStabilizer* cv_PtrOfTwoPassStabilizer_get_inner_ptr_mut(cv::Ptr<cv::videostab::TwoPassStabilizer>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrOfTwoPassStabilizer_to_PtrOfIFrameSource(cv::Ptr<cv::videostab::TwoPassStabilizer>* instance) {
		return new cv::Ptr<cv::videostab::IFrameSource>(instance->dynamicCast<cv::videostab::IFrameSource>());
	}
}

