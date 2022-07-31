extern "C" {
	void cv_PtrOfScanSegment_delete(cv::Ptr<cv::ximgproc::ScanSegment>* instance) {
		delete instance;
	}

	const cv::ximgproc::ScanSegment* cv_PtrOfScanSegment_get_inner_ptr(const cv::Ptr<cv::ximgproc::ScanSegment>* instance) {
		return instance->get();
	}

	cv::ximgproc::ScanSegment* cv_PtrOfScanSegment_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::ScanSegment>* instance) {
		return instance->get();
	}
}

