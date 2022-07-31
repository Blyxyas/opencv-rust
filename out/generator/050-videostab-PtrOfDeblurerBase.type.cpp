extern "C" {
	void cv_PtrOfDeblurerBase_delete(cv::Ptr<cv::videostab::DeblurerBase>* instance) {
		delete instance;
	}

	const cv::videostab::DeblurerBase* cv_PtrOfDeblurerBase_get_inner_ptr(const cv::Ptr<cv::videostab::DeblurerBase>* instance) {
		return instance->get();
	}

	cv::videostab::DeblurerBase* cv_PtrOfDeblurerBase_get_inner_ptr_mut(cv::Ptr<cv::videostab::DeblurerBase>* instance) {
		return instance->get();
	}
}

