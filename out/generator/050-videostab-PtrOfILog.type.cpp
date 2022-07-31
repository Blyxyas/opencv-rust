extern "C" {
	void cv_PtrOfILog_delete(cv::Ptr<cv::videostab::ILog>* instance) {
		delete instance;
	}

	const cv::videostab::ILog* cv_PtrOfILog_get_inner_ptr(const cv::Ptr<cv::videostab::ILog>* instance) {
		return instance->get();
	}

	cv::videostab::ILog* cv_PtrOfILog_get_inner_ptr_mut(cv::Ptr<cv::videostab::ILog>* instance) {
		return instance->get();
	}
}

