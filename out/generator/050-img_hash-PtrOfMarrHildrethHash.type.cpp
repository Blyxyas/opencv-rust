extern "C" {
	cv::Ptr<cv::img_hash::MarrHildrethHash>* cv_PtrOfMarrHildrethHash_new(cv::img_hash::MarrHildrethHash* val) {
		return new cv::Ptr<cv::img_hash::MarrHildrethHash>(val);
	}
	
	void cv_PtrOfMarrHildrethHash_delete(cv::Ptr<cv::img_hash::MarrHildrethHash>* instance) {
		delete instance;
	}

	const cv::img_hash::MarrHildrethHash* cv_PtrOfMarrHildrethHash_get_inner_ptr(const cv::Ptr<cv::img_hash::MarrHildrethHash>* instance) {
		return instance->get();
	}

	cv::img_hash::MarrHildrethHash* cv_PtrOfMarrHildrethHash_get_inner_ptr_mut(cv::Ptr<cv::img_hash::MarrHildrethHash>* instance) {
		return instance->get();
	}
}

