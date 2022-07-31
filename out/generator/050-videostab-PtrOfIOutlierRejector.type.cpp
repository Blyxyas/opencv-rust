extern "C" {
	void cv_PtrOfIOutlierRejector_delete(cv::Ptr<cv::videostab::IOutlierRejector>* instance) {
		delete instance;
	}

	const cv::videostab::IOutlierRejector* cv_PtrOfIOutlierRejector_get_inner_ptr(const cv::Ptr<cv::videostab::IOutlierRejector>* instance) {
		return instance->get();
	}

	cv::videostab::IOutlierRejector* cv_PtrOfIOutlierRejector_get_inner_ptr_mut(cv::Ptr<cv::videostab::IOutlierRejector>* instance) {
		return instance->get();
	}
}

