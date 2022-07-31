extern "C" {
	void cv_PtrOfIFrameSource_delete(cv::Ptr<cv::videostab::IFrameSource>* instance) {
		delete instance;
	}

	const cv::videostab::IFrameSource* cv_PtrOfIFrameSource_get_inner_ptr(const cv::Ptr<cv::videostab::IFrameSource>* instance) {
		return instance->get();
	}

	cv::videostab::IFrameSource* cv_PtrOfIFrameSource_get_inner_ptr_mut(cv::Ptr<cv::videostab::IFrameSource>* instance) {
		return instance->get();
	}
}

