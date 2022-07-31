extern "C" {
	void cv_PtrOfInpainterBase_delete(cv::Ptr<cv::videostab::InpainterBase>* instance) {
		delete instance;
	}

	const cv::videostab::InpainterBase* cv_PtrOfInpainterBase_get_inner_ptr(const cv::Ptr<cv::videostab::InpainterBase>* instance) {
		return instance->get();
	}

	cv::videostab::InpainterBase* cv_PtrOfInpainterBase_get_inner_ptr_mut(cv::Ptr<cv::videostab::InpainterBase>* instance) {
		return instance->get();
	}
}

