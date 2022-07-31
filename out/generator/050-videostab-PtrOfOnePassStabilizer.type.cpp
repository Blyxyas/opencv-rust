extern "C" {
	cv::Ptr<cv::videostab::OnePassStabilizer>* cv_PtrOfOnePassStabilizer_new(cv::videostab::OnePassStabilizer* val) {
		return new cv::Ptr<cv::videostab::OnePassStabilizer>(val);
	}
	
	void cv_PtrOfOnePassStabilizer_delete(cv::Ptr<cv::videostab::OnePassStabilizer>* instance) {
		delete instance;
	}

	const cv::videostab::OnePassStabilizer* cv_PtrOfOnePassStabilizer_get_inner_ptr(const cv::Ptr<cv::videostab::OnePassStabilizer>* instance) {
		return instance->get();
	}

	cv::videostab::OnePassStabilizer* cv_PtrOfOnePassStabilizer_get_inner_ptr_mut(cv::Ptr<cv::videostab::OnePassStabilizer>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrOfOnePassStabilizer_to_PtrOfIFrameSource(cv::Ptr<cv::videostab::OnePassStabilizer>* instance) {
		return new cv::Ptr<cv::videostab::IFrameSource>(instance->dynamicCast<cv::videostab::IFrameSource>());
	}
}

