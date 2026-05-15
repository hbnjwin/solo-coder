<template>
  <div class="wrapper">
    <div class="top common">
      <div class="left">
        <p class="title">{{ standarDocBase.standardNo }} | {{ standarDocBase.standardName }}</p>
        <p class="content">{{ standarDocBase.standardEnName }}</p>
        <div class="button">
          <el-button type="primary"
                     round>{{ standarDocBase.standardLevelName }}</el-button>
          <el-button type="success"
                     round>{{ renderStatusText(standarDocBase.status) }}</el-button>
        </div>
      </div>
      <div class="right">
        <div class="button"
             @click="router.go(-1)"><i class="iconfont icon-fanhui icon cursor-pointer m-r-4"></i>返回</div>
        <div class="button"><i class="iconfont icon-shoucang icon cursor-pointer m-r-4"></i>收藏</div>
        <div class="button"><i class="iconfont icon-duibi1 icon cursor-pointer m-r-4"></i>对比</div>
        <div class="button"><i class="iconfont icon-yulan icon cursor-pointer m-r-4"></i>预览</div>
        <div class="button download"><i class="iconfont icon-xiazai icon cursor-pointer m-r-4"></i>下载</div>
      </div>
    </div>
    <div class="bottom common">
      <el-tabs v-model="tabActive"
               class="demo-tabs"
               v-if="Object.keys(standarDocBase).length > 0">
        <el-tab-pane name="base"><template #label>
            <span class="tab-label"><i class="iconfont icon-jibenxinxi icon cursor-pointer m-r-4"></i>基础信息</span> </template>
          <Base :info="standarDocBase" />
        </el-tab-pane>
        <el-tab-pane name="graph"><template #label>
            <span class="tab-label"><i class="iconfont icon-zhishitupu icon cursor-pointer m-r-4"></i>知识图谱</span> </template>
          <Graph :info="knowledgeGraphList"
                 v-if="tabActive === 'graph'"
                 @changeTabs="tabActive = 'base'" />
        </el-tab-pane>
        <el-tab-pane name="catalog"><template #label>
            <span class="tab-label"><i class="iconfont icon-a-zhangjiemulu1 icon cursor-pointer m-r-4"></i>章节目录</span> </template>
          <Catalog :info="tocListGraph"
                   :content="tocMdContent" />
        </el-tab-pane>
        <el-tab-pane name="xml"><template #label>
            <span class="tab-label"><i class="iconfont icon-jibenxinxi icon cursor-pointer m-r-4"></i>xml格式标准</span> </template>
          <XmlContent :content="xmlContent"
                      v-if="tabActive === 'xml'" />
        </el-tab-pane>
      </el-tabs>
    </div>
  </div>
</template>

<script setup>
import Base from './base.vue';
import Graph from './graph.vue';
import Catalog from './catalog.vue';
import XmlContent from './xml.vue';
const route = useRoute();
const router = useRouter();
import { STANDARD_MANAGEMENT, AI_MANAGEMENT } from '@/api';
const tabActive = ref('base');
const standarDocBase = ref({});
const tocListGraph = ref([]);
const tocMdContent = ref('');
const knowledgeGraphList = ref();
const xmlContent = ref('');
const statusOptions = [
  {
    label: '现行',
    value: 1,
  },
  {
    label: '未生效',
    value: 0,
  },
  {
    label: '废止',
    value: -1,
  },
];
onMounted(() => {
  const type = route.query?.type || 'base';
  tabActive.value = type;
  const id = route.query?.id || null;
  if (id) {
    getDetail(id);
    handleKnowledgeGraph(id);
  }
});

const getDetail = (id) => {
  AI_MANAGEMENT.getSearchDetail(id)
    .then(({ data }) => {
      console.log('=== 详情接口完整响应:', data);
      const responseData = data.data || data;  // 兼容两种可能的结构
      console.log('=== 解析后的数据:', responseData);
      const { standardDoc, tocList, mdContent, xmlContent: xml } = responseData;
      standarDocBase.value = standardDoc;
      tocListGraph.value = tocList;
      tocMdContent.value = mdContent;
      xmlContent.value = xml || '';
    })
    .catch((err) => {
      console.log('=== 详情接口报错:', err);
    });
};

const renderStatusText = (state) => {
  return statusOptions.find((item) => item.value === state)?.label || '';
};

const handleKnowledgeGraph = (id) => {
  AI_MANAGEMENT.getKnowledgeGraph(id)
    .then((res) => {
      knowledgeGraphList.value = res.data;
    })
    .catch((err) => {
      console.log(err);
    });
};
// 加载数据的函数
const fetchData = (id) => {
  // 根据新ID获取数据
  const type = route.query?.type || 'base';
  tabActive.value = type;
  getDetail(id);
  handleKnowledgeGraph(id);
};
// 监听路由参数变化
watch(
  () => route.query.id,
  (newId, oldId) => {
    if (newId !== oldId) {
      // 重新加载数据或执行其他逻辑
      fetchData(newId);
    }
  },
  { immediate: true }, // 初始渲染时也执行一次
);
</script>

<style lang="scss" scoped>
.wrapper {
	display: flex;
	align-items: center;
	flex-direction: column;
	justify-content: flex-start;
	.common {
		border-radius: 8px;
		background: #ffffff;
		box-shadow: 0px 3px 8px 0px rgba(0, 0, 0, 0.16);
		width: 70%;
	}
	.top {
		padding: 20px;
		display: flex;
		justify-content: space-between;
		margin: 30px 0;
		.left {
			display: flex;
			flex-direction: column;
			.title {
				font-size: 20px;
				font-weight: 500;
				color: #111827;
				margin-bottom: 8px;
			}
			.content {
				font-size: 14px;
				color: #6b7280;
				margin-bottom: 12px;
			}
			.button {
				display: flex;
			}
		}
		.right {
			display: flex;
			align-items: flex-start;
			.button {
				padding: 8px 15px;
				display: flex;
				justify-content: center;
				align-items: center;
				border-radius: 4px;
				background: #ffffff;
				border: 1px solid #dcdfe6;
				font-size: 14px;
				font-weight: 500;
				color: #606266;
				margin-left: 6px;
				cursor: pointer;
			}
			.download {
				background: #0078e9;
				border: 1px solid #0078e9;
				color: white;
			}
		}
	}
	.bottom {
		padding: 10px 20px;
		height: calc(100vh - 380px);
		overflow-y: auto;
		scrollbar-width: none;
		.tab-label {
			padding: 0 16px;
		}
	}
}
</style>
