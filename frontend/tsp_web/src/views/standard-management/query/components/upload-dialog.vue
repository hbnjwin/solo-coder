<template>
    <el-dialog v-model="dialog.visible" :title="dialog.title" width="700px" @close="confirmClose">
        <el-form :model="dialog.form" label-position="left" label-width="120px" class="m-t-20" ref="formRef" :rules="dialog.rules">
            <el-form-item prop="systemId" label="子体系：">
                <el-select v-model="dialog.form.systemId" class="m-b-10" style="width: 100%;" clearable placeholder="请选择子体系">
                    <el-option v-for="item in props.systemNameOptions" :key="item.id" :label="item.systemName" :value="item.id" />
                </el-select>
            </el-form-item>
            <el-form-item prop="standardDeparmentId" label="归口部门：">
                <el-select v-model="dialog.form.standardDeparmentId" class="m-b-10" style="width: 100%;" clearable placeholder="请选择归口部门">
                    <el-option v-for="item in props.standardDeparmentNameOptions" :key="item.id" :label="item.standardDepartmentName" :value="item.id" />
                </el-select>
            </el-form-item>
            <el-form-item label="已上传文件：">
                <el-upload style="width: 100%;" drag action="" multiple accept=".pdf" :auto-upload="false" :show-file-list="false" :on-change="uploadChange">
                    <el-icon class="el-icon--upload"><Plus /></el-icon>
                    <div class="m-b-10 flex-center">
                        <div>将文件拖拽到此处或</div>
                        <el-link style="color: #165dff" :underline="false">点击上传</el-link>
                    </div>
                    <div>支持PDF</div>
                </el-upload>
                <div class="file flex-start" v-if="dialog.form.fileName">
                    <img :src="pdf" alt="">
                    <div class="m-r-10 m-l-10 ellipsis" :title="dialog.form.fileName">{{ dialog.form.fileName }}</div>
                    <div class="flex-divider"></div>
                    <el-icon @click="deleteFile"><Close /></el-icon>
                </div>
            </el-form-item>
        </el-form>
        <template #footer>
            <span class="dialog-footer">
                <el-button class="common-button-wite" @click="confirmClose">取 消</el-button>
                <el-button class="common-button-blue" type="primary" @click="handleSubmit" :loading="dialog.loading">确 定</el-button>
            </span>
        </template>
    </el-dialog>
</template>

<script setup>
import { STANDARD_MANAGEMENT } from '@/api';
import { Plus } from '@element-plus/icons-vue'
import pdf from '@/assets/image/fileIcon/pdf.png';

const props = defineProps({
	systemNameOptions: {
		type: Array,
		default: () => [],
	},
    standardDeparmentNameOptions: {
		type: Array,
		default: () => [],
	},
});

const store = useStore();

const dialog = reactive({
    title: '文件上传',
    top: '10vh',
    visible: false,
    loading: false,
    form: {
        systemId: null,
        standardDeparmentId: null,
        fileName: null,
    },
    raw: null,
    rules: {
        systemId: [{ required: true, message: '请选择子体系', trigger: ['blur', 'change'] }],
        standardDeparmentId: [{ required: true, message: '请选择归口部门', trigger: ['blur', 'change'] }],
    },
});
const user = computed(() => store.state.authCenter.user);

const show = (treeRow) => {
    dialog.visible = true;
    dialog.form.systemId = treeRow;
};

const uploadChange = (file) => {
    dialog.form.fileName = file.raw.name;
    dialog.raw = file.raw;
};
const deleteFile = () => {
    dialog.form.fileName = null;
    dialog.raw = null;
}

const formRef = ref(null);
const emits = defineEmits(['success']);
const handleSubmit = () => {
    formRef.value.validate((valid) => {
		if (valid) {
            dialog.loading = true;
            if(!dialog.form.fileName) return ElMessage({ type: 'warning', message: '请上传文件' });
            const { standardDeparmentId, systemId } = dialog.form;
            var formdata = new FormData();
            formdata.append('userId', user.value.id);
            formdata.append('systemId', systemId);
            formdata.append('standardDeparmentId', standardDeparmentId);
            formdata.append('file', dialog.raw);
            STANDARD_MANAGEMENT.postUploadFile(formdata)
            .then(({ data }) => {
                ElMessage({ message: '文件上传成功！', type: 'success' });
                emits('success', { id: data.data.id });
                dialog.loading = false;
                confirmClose();
            })
            .catch(err => {
                console.log(err);
                dialog.loading = false;
            });
		}
	});
};

const confirmClose = () => {
    dialog.form.systemId = null;
    dialog.form.standardDeparmentId = null;
    dialog.form.fileName = null;
	dialog.visible = false;
};

defineExpose({ show });
</script>

<style lang="scss">
.file {
    box-sizing: border-box;
    height: 40px;
    line-height: 40px;
    width: 100%;
    border: 1px solid #cacbcc;
    border-radius: 4px;
    padding: 0 10px;
    margin-top: 15px;
    img {
        width: 20px;
    }
}
</style>

