extern "C" {
	cv::Ptr<cv::videostab::MotionInpainter>* cv_PtrOfMotionInpainter_new(cv::videostab::MotionInpainter* val) {
		return new cv::Ptr<cv::videostab::MotionInpainter>(val);
	}
	
	void cv_PtrOfMotionInpainter_delete(cv::Ptr<cv::videostab::MotionInpainter>* instance) {
		delete instance;
	}

	const cv::videostab::MotionInpainter* cv_PtrOfMotionInpainter_get_inner_ptr(const cv::Ptr<cv::videostab::MotionInpainter>* instance) {
		return instance->get();
	}

	cv::videostab::MotionInpainter* cv_PtrOfMotionInpainter_get_inner_ptr_mut(cv::Ptr<cv::videostab::MotionInpainter>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrOfMotionInpainter_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::MotionInpainter>* instance) {
		return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}
}

