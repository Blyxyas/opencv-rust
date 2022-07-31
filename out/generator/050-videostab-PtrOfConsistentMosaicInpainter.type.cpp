extern "C" {
	cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* cv_PtrOfConsistentMosaicInpainter_new(cv::videostab::ConsistentMosaicInpainter* val) {
		return new cv::Ptr<cv::videostab::ConsistentMosaicInpainter>(val);
	}
	
	void cv_PtrOfConsistentMosaicInpainter_delete(cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* instance) {
		delete instance;
	}

	const cv::videostab::ConsistentMosaicInpainter* cv_PtrOfConsistentMosaicInpainter_get_inner_ptr(const cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* instance) {
		return instance->get();
	}

	cv::videostab::ConsistentMosaicInpainter* cv_PtrOfConsistentMosaicInpainter_get_inner_ptr_mut(cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::InpainterBase>* cv_PtrOfConsistentMosaicInpainter_to_PtrOfInpainterBase(cv::Ptr<cv::videostab::ConsistentMosaicInpainter>* instance) {
		return new cv::Ptr<cv::videostab::InpainterBase>(instance->dynamicCast<cv::videostab::InpainterBase>());
	}
}

