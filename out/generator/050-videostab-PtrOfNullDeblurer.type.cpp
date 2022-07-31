extern "C" {
	cv::Ptr<cv::videostab::NullDeblurer>* cv_PtrOfNullDeblurer_new(cv::videostab::NullDeblurer* val) {
		return new cv::Ptr<cv::videostab::NullDeblurer>(val);
	}
	
	void cv_PtrOfNullDeblurer_delete(cv::Ptr<cv::videostab::NullDeblurer>* instance) {
		delete instance;
	}

	const cv::videostab::NullDeblurer* cv_PtrOfNullDeblurer_get_inner_ptr(const cv::Ptr<cv::videostab::NullDeblurer>* instance) {
		return instance->get();
	}

	cv::videostab::NullDeblurer* cv_PtrOfNullDeblurer_get_inner_ptr_mut(cv::Ptr<cv::videostab::NullDeblurer>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::videostab::DeblurerBase>* cv_PtrOfNullDeblurer_to_PtrOfDeblurerBase(cv::Ptr<cv::videostab::NullDeblurer>* instance) {
		return new cv::Ptr<cv::videostab::DeblurerBase>(instance->dynamicCast<cv::videostab::DeblurerBase>());
	}
}

