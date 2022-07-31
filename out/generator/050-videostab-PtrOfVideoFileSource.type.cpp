extern "C" {
	cv::Ptr<cv::videostab::VideoFileSource>* cv_PtrOfVideoFileSource_new(cv::videostab::VideoFileSource* val) {
		return new cv::Ptr<cv::videostab::VideoFileSource>(val);
	}
	
	void cv_PtrOfVideoFileSource_delete(cv::Ptr<cv::videostab::VideoFileSource>* instance) {
		delete instance;
	}

	const cv::videostab::VideoFileSource* cv_PtrOfVideoFileSource_get_inner_ptr(const cv::Ptr<cv::videostab::VideoFileSource>* instance) {
		return instance->get();
	}

	cv::videostab::VideoFileSource* cv_PtrOfVideoFileSource_get_inner_ptr_mut(cv::Ptr<cv::videostab::VideoFileSource>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrOfVideoFileSource_to_PtrOfIFrameSource(cv::Ptr<cv::videostab::VideoFileSource>* instance) {
		return new cv::Ptr<cv::videostab::IFrameSource>(instance->dynamicCast<cv::videostab::IFrameSource>());
	}
}

