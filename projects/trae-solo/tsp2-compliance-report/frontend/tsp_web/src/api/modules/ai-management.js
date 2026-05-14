import appRequest from '../app-request.js';

export default {
	//标准智能查询
	getSearchList(params) {
		return appRequest.post('/api/standardDoc/advancedSearch', params);
	},
	getSearchDetail(id, params) {
		return appRequest.get(`/api/standardDoc/detail/${id}`, { params });
	},
	getKnowledgeGraph(id, params) {
		return appRequest.get(`/api/knowledge/standards/${id}/graph`, { params });
	},
};
