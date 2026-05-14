<template>
    <div class="common-wrapper">
		<div class="flex-start m-b-10">
			<el-button class="common-button-back" plain @click="handleBack">
				<img class="backImg" src="@/assets/image/result/back.png">
				<span>返回</span>
			</el-button>
			<div class="m-l-15 topsC-textSize"><span class="topText">标准管理 /</span>
				编 辑</div>
			<div class="flex-divider"></div>
			<el-button class="common-button-blue" type="primary" @click="handleSubmit">保 存</el-button>
		</div>
        <el-card style="height: calc(100vh - 185px); overflow-y: scroll;">
            <Title style="font-size:14px" :title="'基础信息'"></Title>
            <el-form ref="formRef" :model="form" :rules="rules" label-position="right" label-width="160px" class="m-t-20">
                <el-form-item prop="systemId" label="子体系：">
                    <el-select v-model="form.systemId" class="m-b-10" style="width: 300px;" clearable placeholder="请选择子体系">
                        <el-option v-for="item in systemNameOptions" :key="item.id" :label="item.systemName" :value="item.id" />
                    </el-select>
                </el-form-item>
                <el-form-item prop="standardDeparmentId" label="归口部门：">
                    <el-select v-model="form.standardDeparmentId" class="m-b-10" style="width:300px;" clearable placeholder="请选择归口部门">
                        <el-option v-for="item in standardDeparmentNameOptions" :key="item.id" :label="item.standardDepartmentName" :value="item.id" />
                    </el-select>
                </el-form-item>
                <el-form-item prop="standardNo" label="标准号：">
                    <el-input v-model="form.standardNo" placeholder="请输入标准号" style="width: 300px;"></el-input>
                </el-form-item>
                <el-form-item prop="standardName" label="标准名称：">
                    <el-input v-model="form.standardName" placeholder="请输入标准名称" style="width: 300px;"></el-input>
                </el-form-item>
                <el-form-item prop="standardLevelName" label="标准级别：">
                    <el-input v-model="form.standardLevelName" placeholder="请输入标准级别" style="width: 300px;"></el-input>
                </el-form-item>
                <el-form-item prop="publishDate" label="发布时间：">
                    <el-date-picker v-model="form.publishDate" type="date" value-format="YYYY-MM-DD HH:mm:ss" format="YYYY-MM-DD HH:mm:ss" placeholder="请输入创建时间" style="width: 300px;" />
                </el-form-item>
                <el-form-item label="标准文件：">
                    <el-upload
                        style="width: 500px;"
                        drag
                        action=""
                        multiple
                        accept=".pdf"
                        :auto-upload="false"
                        :show-file-list="false"
                        :on-change="uploadChange"
                    >
                        <el-icon class="el-icon--upload"><Plus /></el-icon>
                        <div class="m-b-10 flex-center">
                            <div>将文件拖拽到此处或点击上传</div>
                            <el-link style="color: #165dff" :underline="false">点击上传</el-link>
                        </div>
                        <div>支持PDF</div>
                    </el-upload>

                </el-form-item>
                <el-form-item>
                    <div class="file flex-start" v-if="form.fileUrl">
                        <img :src="pdf" alt="">
                        <div class="ellipsis p-l-15 p-r-15" :title="form.fileUrl">{{ form.fileUrl }}</div>
                    </div>
                </el-form-item>
            </el-form>
        </el-card>
    </div>
</template>

<script setup>
import { STANDARD_MANAGEMENT } from '@/api';
import pdf from '@/assets/image/fileIcon/pdf.png';
import { Back } from '@element-plus/icons-vue';
import Title from '@/components/title/index.vue';

const store = useStore();
const route = useRoute();
const router = useRouter();
const user = computed(() => store.state.authCenter.user);

const form = ref({
    systemId: null,
    standardDeparmentId: null,
    standardNo: null,
    standardName: null,
    standardLevelName: null,
    publishDate: null,
    fileUrl: null,
});
const rules = ({
    systemId: [{ required: true, message: '请选择子体系', trigger: ['blur', 'change'] }],
    standardDeparmentId: [{ required: true, message: '请选择归口部门', trigger: ['blur', 'change'] }],
    standardNo: [{ required: true, message: '请输入标准号', trigger: ['blur', 'change'] }],
    standardName: [{ required: true, message: '请输入标准名称', trigger: ['blur', 'change'] }],
    standardLevelName: [{ required: true, message: '请输入标准级别', trigger: ['blur', 'change'] }],
    publishDate: [{ required: true, message: '请选择发布时间', trigger: ['blur', 'change'] }],
})
const systemNameOptions = ref([]);
const standardDeparmentNameOptions = ref([]);

onMounted(() => {
    getTreeList();
});

const getTreeList = () => {
    const standardSystemTree = STANDARD_MANAGEMENT.getTreeExclude({ deptId: user.value.deptId, maxLevel: 1 });
    const standardDepartmentTree = STANDARD_MANAGEMENT.getTreeByCompany({ deptId: user.value.deptId })
    Promise.all([standardSystemTree, standardDepartmentTree])
        .then(([systemData, departmentData]) => {
            systemNameOptions.value = systemData.data.data;
            standardDeparmentNameOptions.value = departmentData.data.data;
            getListDetail();
        })
        .catch(err => console.log(err));
}

const getListDetail = () => {
    STANDARD_MANAGEMENT.getListDetail({ id: route.query.id })
        .then(({ data }) => {
            form.value = data.data;
            store.commit('query/getCompanyStore', data.data.deptId);
            store.commit('query/getsystemIdStore', data.data.systemId);
        })
        .catch(err => {
            console.log(err);
        });
};

const handleBack = () => {
    router.push({ name: 'StandardManagement.Query' });
};

const uploadChange = (file) => {
    const { standardDeparmentId, systemId } = form.value;
    var formdata = new FormData();
    formdata.append('id', route.query.id);
    formdata.append('userId', user.value.id);
    formdata.append('systemId', Array.isArray(systemId) ? systemId[systemId.length - 1] : systemId);
    formdata.append('standardDeparmentId', Array.isArray(standardDeparmentId) ? standardDeparmentId[standardDeparmentId.length - 1] : standardDeparmentId);
    formdata.append('file', file.raw);
    STANDARD_MANAGEMENT.postUploadFile(formdata)
        .then(({ data }) => {
            ElMessage({ message: '文件上传成功！', type: 'success' });
            const { systemId, standardDeparmentId, standardNo, standardName, standardLevelName, publishDate, fileUrl } = data.data;
            Object.assign(form.value, {
                systemId, standardDeparmentId, standardNo, standardName, standardLevelName, publishDate, fileUrl
            });
        })
        .catch(err => {console.log(err);});
};

const formRef = ref(null);
const handleSubmit = () => {
    formRef.value.validate((valid) => {
		if (valid) {
            if(form.fileUrl) return ElMessage({ type: 'warning', message: '请上传文件' });
            STANDARD_MANAGEMENT.getUpdate(form.value)
            .then(({ data }) => {
                ElMessage({ message: '保存成功！', type: 'success' });
                router.push({ name: 'StandardManagement.Query' });
            })
            .catch(err => {console.log(err);});
		}
	});
};

</script>

<style lang="less" scoped>
.file {
    height: 40px;
    line-height: 40px;
    width: 490px;
    border: 1px solid #cacbcc;
    border-radius: 4px;
    padding-left: 10px;
    margin-top: 15px;
    img {
        width: 20px;
    }
}
</style>