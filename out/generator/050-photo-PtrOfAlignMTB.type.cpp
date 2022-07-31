extern "C" {
	void cv_PtrOfAlignMTB_delete(cv::Ptr<cv::AlignMTB>* instance) {
		delete instance;
	}

	const cv::AlignMTB* cv_PtrOfAlignMTB_get_inner_ptr(const cv::Ptr<cv::AlignMTB>* instance) {
		return instance->get();
	}

	cv::AlignMTB* cv_PtrOfAlignMTB_get_inner_ptr_mut(cv::Ptr<cv::AlignMTB>* instance) {
		return instance->get();
	}
}

