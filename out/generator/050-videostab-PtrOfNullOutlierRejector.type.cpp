extern "C" {
	cv::Ptr<cv::videostab::NullOutlierRejector>* cv_PtrOfNullOutlierRejector_new(cv::videostab::NullOutlierRejector* val) {
		return new cv::Ptr<cv::videostab::NullOutlierRejector>(val);
	}
	
	void cv_PtrOfNullOutlierRejector_delete(cv::Ptr<cv::videostab::NullOutlierRejector>* instance) {
		delete instance;
	}

	const cv::videostab::NullOutlierRejector* cv_PtrOfNullOutlierRejector_get_inner_ptr(const cv::Ptr<cv::videostab::NullOutlierRejector>* instance) {
		return instance->get();
	}

	cv::videostab::NullOutlierRejector* cv_PtrOfNullOutlierRejector_get_inner_ptr_mut(cv::Ptr<cv::videostab::NullOutlierRejector>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IOutlierRejector>* cv_PtrOfNullOutlierRejector_to_PtrOfIOutlierRejector(cv::Ptr<cv::videostab::NullOutlierRejector>* instance) {
		return new cv::Ptr<cv::videostab::IOutlierRejector>(instance->dynamicCast<cv::videostab::IOutlierRejector>());
	}
}

