extern "C" {
	cv::Ptr<cv::SimpleBlobDetector>* cv_PtrOfSimpleBlobDetector_new(cv::SimpleBlobDetector* val) {
		return new cv::Ptr<cv::SimpleBlobDetector>(val);
	}
	
	void cv_PtrOfSimpleBlobDetector_delete(cv::Ptr<cv::SimpleBlobDetector>* instance) {
		delete instance;
	}

	const cv::SimpleBlobDetector* cv_PtrOfSimpleBlobDetector_get_inner_ptr(const cv::Ptr<cv::SimpleBlobDetector>* instance) {
		return instance->get();
	}

	cv::SimpleBlobDetector* cv_PtrOfSimpleBlobDetector_get_inner_ptr_mut(cv::Ptr<cv::SimpleBlobDetector>* instance) {
		return instance->get();
	}
	
	cv::Ptr<cv::Feature2D>* cv_PtrOfSimpleBlobDetector_to_PtrOfFeature2D(cv::Ptr<cv::SimpleBlobDetector>* instance) {
		return new cv::Ptr<cv::Feature2D>(instance->dynamicCast<cv::Feature2D>());
	}
}

