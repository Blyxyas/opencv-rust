extern "C" {
	void cv_PtrOfSuperres_FrameSource_delete(cv::Ptr<cv::superres::FrameSource>* instance) {
		delete instance;
	}

	const cv::superres::FrameSource* cv_PtrOfSuperres_FrameSource_get_inner_ptr(const cv::Ptr<cv::superres::FrameSource>* instance) {
		return instance->get();
	}

	cv::superres::FrameSource* cv_PtrOfSuperres_FrameSource_get_inner_ptr_mut(cv::Ptr<cv::superres::FrameSource>* instance) {
		return instance->get();
	}
}

