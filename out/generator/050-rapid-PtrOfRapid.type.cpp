extern "C" {
	void cv_PtrOfRapid_delete(cv::Ptr<cv::rapid::Rapid>* instance) {
		delete instance;
	}

	const cv::rapid::Rapid* cv_PtrOfRapid_get_inner_ptr(const cv::Ptr<cv::rapid::Rapid>* instance) {
		return instance->get();
	}

	cv::rapid::Rapid* cv_PtrOfRapid_get_inner_ptr_mut(cv::Ptr<cv::rapid::Rapid>* instance) {
		return instance->get();
	}
}

