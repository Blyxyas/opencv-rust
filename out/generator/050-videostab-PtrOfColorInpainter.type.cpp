extern "C" {
	cv::Ptr<cv::videostab::ColorInpainter>* cv_PtrOfColorInpainter_new(cv::videostab::ColorInpainter* val) {
		return new cv::Ptr<cv::videostab::ColorInpainter>(val);
	}
	
	void cv_PtrOfColorInpainter_delete(cv::Ptr<cv::videostab::ColorInpainter>* instance) {
		delete instance;
	}

	const cv::videostab::ColorInpainter* cv_PtrOfColorInpainter_get_inner_ptr(const cv::Ptr<cv::videostab::ColorInpainter>* instance) {
		return instance->get();
	}

	cv::videostab::ColorInpainter* cv_PtrOfColorInpainter_get_inner_ptr_mut(cv::Ptr<cv::videostab::ColorInpainter>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrOfColorInpainter_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::ColorInpainter>* instance) {
		return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}
}

