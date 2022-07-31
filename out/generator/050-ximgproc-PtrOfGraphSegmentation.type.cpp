extern "C" {
	void cv_PtrOfGraphSegmentation_delete(cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>* instance) {
		delete instance;
	}

	const cv::ximgproc::segmentation::GraphSegmentation* cv_PtrOfGraphSegmentation_get_inner_ptr(const cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>* instance) {
		return instance->get();
	}

	cv::ximgproc::segmentation::GraphSegmentation* cv_PtrOfGraphSegmentation_get_inner_ptr_mut(cv::Ptr<cv::ximgproc::segmentation::GraphSegmentation>* instance) {
		return instance->get();
	}
}

