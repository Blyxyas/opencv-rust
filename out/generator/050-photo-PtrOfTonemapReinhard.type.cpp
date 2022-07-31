extern "C" {
	void cv_PtrOfTonemapReinhard_delete(cv::Ptr<cv::TonemapReinhard>* instance) {
		delete instance;
	}

	const cv::TonemapReinhard* cv_PtrOfTonemapReinhard_get_inner_ptr(const cv::Ptr<cv::TonemapReinhard>* instance) {
		return instance->get();
	}

	cv::TonemapReinhard* cv_PtrOfTonemapReinhard_get_inner_ptr_mut(cv::Ptr<cv::TonemapReinhard>* instance) {
		return instance->get();
	}
}

