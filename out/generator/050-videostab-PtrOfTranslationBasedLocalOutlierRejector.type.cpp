extern "C" {
	cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* cv_PtrOfTranslationBasedLocalOutlierRejector_new(cv::videostab::TranslationBasedLocalOutlierRejector* val) {
		return new cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>(val);
	}
	
	void cv_PtrOfTranslationBasedLocalOutlierRejector_delete(cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* instance) {
		delete instance;
	}

	const cv::videostab::TranslationBasedLocalOutlierRejector* cv_PtrOfTranslationBasedLocalOutlierRejector_get_inner_ptr(const cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* instance) {
		return instance->get();
	}

	cv::videostab::TranslationBasedLocalOutlierRejector* cv_PtrOfTranslationBasedLocalOutlierRejector_get_inner_ptr_mut(cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::IOutlierRejector>* cv_PtrOfTranslationBasedLocalOutlierRejector_to_PtrOfIOutlierRejector(cv::Ptr<cv::videostab::TranslationBasedLocalOutlierRejector>* instance) {
		return new cv::Ptr<cv::videostab::IOutlierRejector>(instance->dynamicCast<cv::videostab::IOutlierRejector>());
	}
}

