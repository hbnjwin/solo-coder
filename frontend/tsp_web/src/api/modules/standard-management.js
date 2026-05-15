import appRequest from '../app-request.js';

export default {
	getDepartmentDocList(params) {
		return appRequest.get('/api/departmentDoc/list', { params });
	},
	getStandardEvaluateTypeTree(params) {
		return appRequest.get('/api/standardEvaluateType/tree', { params });
	},
	getStandardSystemList(params) {
		return appRequest.get('/api/standardSystem/list', { params });
	},
	getStandardSystemTree(params) {
		return appRequest.get('/api/standardSystem/tree', { params });
	},
	getTreeExclude(params) {
		return appRequest.get('/api/standardSystem/treeExclude', { params });
	},
	getStandardDepartmentTree(params) {
		return appRequest.get('/api/standardDepartment/tree', { params });
	},
	getStandardEvaluateTypeList(params) {
		return appRequest.get('/api/standardEvaluateType/list', { params });
	},
	getStandardLevelList(params) {
		return appRequest.get('/api/standardLevel/list', { params });
	},
	getStandardDepartmentList(params) {
		return appRequest.get('/api/standardDepartment/list', { params });
	},
	getTreeByCompany(params) {
		return appRequest.get('/api/standardDepartment/treeByCompany', { params });
	},
	getList(params) {
		return appRequest.get('/api/standardDoc/list', { params });
	},
	getListDetail(params) {
		return appRequest.get('/api/standardDoc/listDetail', { params });
	},
	getListByEnterprise(params) {
		return appRequest.get('/api/standardDoc/listByEnterprise', { params });
	},
	postUploadFile(params) {
		return appRequest.post(`/api/standardDoc/uploadFile`, params);
	},
	postUploadZip(params) {
		return appRequest.post(`/api/standardDoc/uploadZip`, params);
	},
	getSetStatus(params) {
		return appRequest.get('/api/standardDoc/setStatus', { params });
	},
	getDelete(params) {
		return appRequest.get('/api/standardDoc/delete', { params });
	},
	getUpdate(params) {
		return appRequest.post('/api/standardDoc/update', params);
	},
	getDownload(id, params) {
		return appRequest.get(`/api/standardDoc/downFile/${id}`, { params, responseType: 'blob' });
	},
	getDownTotal(id, params) {
		return appRequest.get(`/api/standardDoc/downTotal/${id}`, { params, responseType: 'blob' });
	},
	getDownDetail(id, systemId, params) {
		return appRequest.get(`/api/standardDoc/downDetail/${id}/${systemId}`, { params, responseType: 'blob' });
	},
	getFile(id, params) {
		return appRequest.get(`/api/standardDoc/getFile/${id}`, { params, responseType: 'blob' });
	},
};