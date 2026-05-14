<template>
  <div class="base-layout">
    <div class="input-right">
      <el-button type="primary"
                 :icon="Refresh"
                 class="m-r-20"
                 @click="handleResetGraph">重 置</el-button>
      <el-input v-model="input2"
                style="width: 350px"
                placeholder="请输入实体名称"
                :prefix-icon="Search" />
    </div>
    <div v-if="!hasData"
         class="empty-state">
      <i class="iconfont icon-zhishitupu icon"
         style="font-size: 48px; color: #dcdfe6;"></i>
      <p style="margin-top: 12px; color: #909399;">{{ message || '暂无知识图谱数据' }}</p>
    </div>
    <div v-else
         ref="container"
         class="graph-container"></div>
  </div>
</template>

<script setup>
const router = useRouter();
import { ref, onMounted, computed } from 'vue';
import { Search, Refresh } from '@element-plus/icons-vue';
import * as d3 from 'd3';
const input2 = ref('');
const props = defineProps({
  info: {
    type: Object,
    required: () => { },
  },
});
const container = ref(null);
const message = ref('');
let svg, simulation, nodesG, linksG;
let graphDataOptions = reactive({
  nodes: [],
  links: [],
});
const renderOptions = {
  departments: '归口单位',
  draftUnits: '起草单位',
  drafters: '起草人',
  publishUnits: '发布单位',
  levels: '标准级别',
  proposeUnits: '提出部门',
  referencedStandards: '引用标准',
};

// 定义可触发的事件
const emit = defineEmits(['changeTabs']);

const typeStructure = ref({});
const level2Nodes = ref();
const level2ColorScale = ref();

const hasData = computed(() => {
  // 检查是否有有效的数据
  if (!props.info) return false;
  if (props.info.message) {
    message.value = props.info.message;
    return false;
  }
  if (props.info.nodes && props.info.nodes.length === 0) return false;
  if (props.info.properties) return true;
  return false;
});

onMounted(() => {
  if (hasData.value && container.value) {
    graphDataOptions.nodes.push({ id: 'node1', label: props.info.properties.standardName, group: '主体', level: 1 });
    Object.keys(renderOptions).forEach((key) => {
      graphDataOptions.nodes.push({ id: `node${graphDataOptions.nodes.length + 1}`, label: renderOptions[key], group: key, level: 2, parentId: 'node1', unique: key });
      graphDataOptions.links.push({ source: `node1`, target: `node${graphDataOptions.nodes.length}`, unique: key });
    });

    Object.keys(props.info.relationNodes).forEach((key) => {
      if (Object.keys(renderOptions).includes(key)) {
        typeStructure.value[key] = [];
        props.info.relationNodes[key].forEach((item) => {
          typeStructure.value[key].push(item.name);
          graphDataOptions.nodes.push({ id: `node${graphDataOptions.nodes.length + 1}`, label: key !== 'referencedStandards' ? item.name : item.standardName, group: `类别详情${key}`, level: 3, parentId: graphDataOptions.nodes.find((it) => it.unique === key).id, standardId: key !== 'referencedStandards' ? null : item.standardId });
          graphDataOptions.links.push({ source: graphDataOptions.links.find((it) => it.unique === key).target, target: `node${graphDataOptions.nodes.length}`, unique: key });
        });
      }
    });
    setTimeout(() => {
      // 1. 提取第二级节点，为其分配基准色
      level2Nodes.value = graphDataOptions.nodes.filter((d) => d.level === 2);
      // 使用 D3 颜色比例尺生成第二级节点的基准色（如 10 种分类色）
      level2ColorScale.value = d3
        .scaleOrdinal()
        .domain(level2Nodes.value.map((d) => d.id)) // 以第二级节点ID为键
        .range(d3.schemeCategory10); // 可选：d3.schemeSet3、自定义颜色数组
      renderGraph();
    }, 1000);
  }
});

// 2. 定义颜色生成函数：根据节点层级和父级ID返回颜色
const getNodeColor = (node) => {
  if (node.level === 2) {
    // 第二级节点：直接使用基准色
    return level2ColorScale.value(node.id);
  } else if (node.level === 3) {
    // 第三级节点：找到父级（第二级）的基准色，衍生新颜色
    const parentNode = graphDataOptions.nodes.find((d) => d.id === node.parentId);
    const parentColor = level2ColorScale.value(parentNode.id);
    // 将父级颜色转为 HSL 格式，调整亮度（如降低 30% 亮度）
    return d3.hsl(parentColor).brighter(0.3).toString(); // 或 darker(0.3)
  } else {
    // 其他层级（如一级）：默认颜色
    return '#69b3a2';
  }
};

const renderGraph = () => {
  cleanup();
  // 初始化SVG
  const width = container.value.clientWidth;
  const height = container.value.clientHeight;

  svg = d3
    .select(container.value)
    .append('svg')
    .attr('width', width)
    .attr('height', height)
    .call(
      d3.zoom().on('zoom', (event) => {
        svg.attr('transform', event.transform);
      }),
    );

  // 创建背景矩形用于捕获鼠标事件
  svg.append('rect').attr('width', width).attr('height', height).style('fill', 'none').style('pointer-events', 'all');

  // 创建链接组和节点组
  linksG = svg.append('g').attr('class', 'links');
  nodesG = svg.append('g').attr('class', 'nodes');

  // 初始化力导向图
  simulation = d3
    .forceSimulation(graphDataOptions.nodes)
    .force(
      'link',
      d3
        .forceLink(graphDataOptions.links)
        .id((d) => d.id)
        .distance(150),
    )
    .force('charge', d3.forceManyBody().strength(-150))
    .force('center', d3.forceCenter(width / 2, height / 2));

  // 创建链接
  const links = linksG.selectAll('line').data(graphDataOptions.links).enter().append('line').attr('stroke', '#999').attr('stroke-width', 1.5);

  // 创建节点
  const nodes = nodesG.selectAll('g').data(graphDataOptions.nodes).enter().append('g').call(drag(simulation));

  nodes
    .append('circle')
    .attr('r', 15)
    .attr('fill', (d) => getNodeColor(d))
    .on('click', handleNodeClick); // 绑定点击事件

  // 添加节点文本
  nodes
    .append('text')
    .attr('dx', 20)
    .attr('font-size', 12)
    .attr('dy', '.35em')
    .text((d) => d.label);

  // 更新节点和链接位置
  simulation.on('tick', () => {
    links
      .attr('x1', (d) => d.source.x)
      .attr('y1', (d) => d.source.y)
      .attr('x2', (d) => d.target.x)
      .attr('y2', (d) => d.target.y);

    nodes.attr('transform', (d) => `translate(${d.x},${d.y})`);
  });
};
// 清理旧图表资源
const cleanup = () => {
  if (svg) {
    svg.remove();
    svg = null;
  }
  if (simulation) {
    simulation.stop();
    simulation = null;
  }
  linksG = null;
  nodesG = null;
};
// 拖拽功能
const drag = (simulation) => {
  return d3
    .drag()
    .on('start', (event, d) => {
      if (!event.active) simulation.alphaTarget(0.3).restart();
      d.fx = d.x;
      d.fy = d.y;
    })
    .on('drag', (event, d) => {
      d.fx = event.x;
      d.fy = event.y;
    })
    .on('end', (event, d) => {
      if (!event.active) simulation.alphaTarget(0);
      d.fx = null;
      d.fy = null;
    });
};

const handleResetGraph = () => {
  renderGraph();
};

//处理节点点击事件
const handleNodeClick = (event, d) => {
  event.stopPropagation();
  if (d.group !== '类别详情referencedStandards') return;
  console.log(d, '11');
  // 触发事件并传递参数
  router.push({
    name: 'AiManagement.Query.BaseMessage',
    query: {
      id: d.standardId,
      type: 'base',
    },
  });
  // emit('changeTabs');
};

onBeforeUnmount(() => {
  // 清理资源
  if (simulation) simulation.stop();
});
</script>

<style lang="less" scoped>
.base-layout {
	display: flex;
	flex-direction: column;
	.input-right {
		display: flex;
		justify-content: right;
		margin-bottom: 12px;
		margin-right: 12px;
	}
	.empty-state {
		width: 100%;
		height: 90%;
		display: flex;
		flex-direction: column;
		align-items: center;
		justify-content: center;
	}
	.graph-container {
		width: 100%;
		height: 90%;
	}
}
</style>
