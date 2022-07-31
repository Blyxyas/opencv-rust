extern "C" {
	cv::Ptr<cv::videostab::MaskFrameSource>* cv_PtrOfMaskFrameSource_new(cv::videostab::MaskFrameSource* val) {
		return new cv::Ptr<cv::videostab::MaskFrameSource>(val);
	}
	
	void cv_PtrOfMaskFrameSource_delete(cv::Ptr<cv::videostab::MaskFrameSource>* instance) {
		delete instance;
	}

	const cv::videostab::MaskFrameSource* cv_PtrOfMaskFrameSource_get_inner_ptr(const cv::Ptr<cv::videostab::MaskFrameSource>* instance) {
		return instance->get();
	}

	cv::videostab::MaskFrameSource* cv_PtrOfMaskFrameSource_get_inner_ptr_mut(cv::Ptr<cv::videostab::MaskFrameSource>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrOfMaskFrameSource_to_PtrOfIFrameSource(cv::Ptr<cv::videostab::MaskFrameSource>* instance) {
		return new cv::Ptr<cv::videostab::IFrameSource>(instance->dynamicCast<cv::videostab::IFrameSource>());
	}
}

