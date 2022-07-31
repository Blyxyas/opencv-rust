extern "C" {
	cv::Ptr<cv::img_hash::ColorMomentHash>* cv_PtrOfColorMomentHash_new(cv::img_hash::ColorMomentHash* val) {
		return new cv::Ptr<cv::img_hash::ColorMomentHash>(val);
	}
	
	void cv_PtrOfColorMomentHash_delete(cv::Ptr<cv::img_hash::ColorMomentHash>* instance) {
		delete instance;
	}

	const cv::img_hash::ColorMomentHash* cv_PtrOfColorMomentHash_get_inner_ptr(const cv::Ptr<cv::img_hash::ColorMomentHash>* instance) {
		return instance->get();
	}

	cv::img_hash::ColorMomentHash* cv_PtrOfColorMomentHash_get_inner_ptr_mut(cv::Ptr<cv::img_hash::ColorMomentHash>* instance) {
		return instance->get();
	}
}

