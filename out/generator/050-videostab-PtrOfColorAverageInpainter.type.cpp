extern "C" {
	cv::Ptr<cv::videostab::ColorAverageInpainter>* cv_PtrOfColorAverageInpainter_new(cv::videostab::ColorAverageInpainter* val) {
		return new cv::Ptr<cv::videostab::ColorAverageInpainter>(val);
	}
	
	void cv_PtrOfColorAverageInpainter_delete(cv::Ptr<cv::videostab::ColorAverageInpainter>* instance) {
		delete instance;
	}

	const cv::videostab::ColorAverageInpainter* cv_PtrOfColorAverageInpainter_get_inner_ptr(const cv::Ptr<cv::videostab::ColorAverageInpainter>* instance) {
		return instance->get();
	}

	cv::videostab::ColorAverageInpainter* cv_PtrOfColorAverageInpainter_get_inner_ptr_mut(cv::Ptr<cv::videostab::ColorAverageInpainter>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrOfColorAverageInpainter_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::ColorAverageInpainter>* instance) {
		return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}
}

