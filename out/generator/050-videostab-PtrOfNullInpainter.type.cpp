extern "C" {
	cv::Ptr<cv::videostab::NullInpainter>* cv_PtrOfNullInpainter_new(cv::videostab::NullInpainter* val) {
		return new cv::Ptr<cv::videostab::NullInpainter>(val);
	}
	
	void cv_PtrOfNullInpainter_delete(cv::Ptr<cv::videostab::NullInpainter>* instance) {
		delete instance;
	}

	const cv::videostab::NullInpainter* cv_PtrOfNullInpainter_get_inner_ptr(const cv::Ptr<cv::videostab::NullInpainter>* instance) {
		return instance->get();
	}

	cv::videostab::NullInpainter* cv_PtrOfNullInpainter_get_inner_ptr_mut(cv::Ptr<cv::videostab::NullInpainter>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrOfNullInpainter_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::NullInpainter>* instance) {
		return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}
}

