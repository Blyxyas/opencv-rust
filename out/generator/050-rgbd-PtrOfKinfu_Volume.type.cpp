extern "C" {
	void cv_PtrOfKinfu_Volume_delete(cv::Ptr<cv::kinfu::Volume>* instance) {
		delete instance;
	}

	const cv::kinfu::Volume* cv_PtrOfKinfu_Volume_get_inner_ptr(const cv::Ptr<cv::kinfu::Volume>* instance) {
		return instance->get();
	}

	cv::kinfu::Volume* cv_PtrOfKinfu_Volume_get_inner_ptr_mut(cv::Ptr<cv::kinfu::Volume>* instance) {
		return instance->get();
	}
}

