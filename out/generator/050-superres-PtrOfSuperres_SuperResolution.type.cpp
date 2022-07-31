extern "C" {
	void cv_PtrOfSuperres_SuperResolution_delete(cv::Ptr<cv::superres::SuperResolution>* instance) {
		delete instance;
	}

	const cv::superres::SuperResolution* cv_PtrOfSuperres_SuperResolution_get_inner_ptr(const cv::Ptr<cv::superres::SuperResolution>* instance) {
		return instance->get();
	}

	cv::superres::SuperResolution* cv_PtrOfSuperres_SuperResolution_get_inner_ptr_mut(cv::Ptr<cv::superres::SuperResolution>* instance) {
		return instance->get();
	}
}

