extern "C" {
	cv::Ptr<cv::videostab::NullFrameSource>* cv_PtrOfNullFrameSource_new(cv::videostab::NullFrameSource* val) {
		return new cv::Ptr<cv::videostab::NullFrameSource>(val);
	}
	
	void cv_PtrOfNullFrameSource_delete(cv::Ptr<cv::videostab::NullFrameSource>* instance) {
		delete instance;
	}

	const cv::videostab::NullFrameSource* cv_PtrOfNullFrameSource_get_inner_ptr(const cv::Ptr<cv::videostab::NullFrameSource>* instance) {
		return instance->get();
	}

	cv::videostab::NullFrameSource* cv_PtrOfNullFrameSource_get_inner_ptr_mut(cv::Ptr<cv::videostab::NullFrameSource>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IFrameSource>* cv_PtrOfNullFrameSource_to_PtrOfIFrameSource(cv::Ptr<cv::videostab::NullFrameSource>* instance) {
		return new cv::Ptr<cv::videostab::IFrameSource>(instance->dynamicCast<cv::videostab::IFrameSource>());
	}
}

