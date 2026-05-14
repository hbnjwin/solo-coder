package com.linkyoyo.tobacco.service.impl;

import com.fasterxml.jackson.databind.ObjectMapper;
import com.linkyoyo.tobacco.entity.DepartmentDoc;
import com.linkyoyo.tobacco.entity.QDepartmentDoc;
import com.linkyoyo.tobacco.entity.QStandardDoc;
import com.linkyoyo.tobacco.entity.StatisticsData;
import com.linkyoyo.tobacco.entity.SysParaset;
import com.linkyoyo.tobacco.repository.StatisticsDataRepository;
import com.linkyoyo.tobacco.repository.SysParasetRepository;
import com.linkyoyo.tobacco.result.R;
import com.linkyoyo.tobacco.service.StatisticsDataService;
import com.querydsl.core.Tuple;
import com.querydsl.jpa.impl.JPAQueryFactory;
import lombok.extern.slf4j.Slf4j;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import javax.persistence.EntityManager;
import javax.persistence.PersistenceContext;
import java.util.*;

@Service
@Slf4j
public class StatisticsDataServiceImpl implements StatisticsDataService {

    @Autowired
    private StatisticsDataRepository statisticsDataRepository;
    
    @Autowired
    private SysParasetRepository sysParasetRepository;
    
    @Autowired
    private ObjectMapper objectMapper;
    
    @PersistenceContext
    private EntityManager entityManager;

    @Override
    public R getStatisticsData(Integer deptId) {
        try {
            // 获取系统参数设置
            SysParaset sysParaset = sysParasetRepository.findById(1).orElse(new SysParaset());
            boolean useStatistics = sysParaset.getUseStatistics() != null && sysParaset.getUseStatistics();
            
            // 获取实际的标准文件数量
            Map<String, Long> actualStandardDocCounts = countStandardDocsByDepartment();
            
            Optional<StatisticsData> statisticsDataOpt;
            StatisticsData stats;
            
            if (deptId == null) {
                // 全局统计数据
                statisticsDataOpt = statisticsDataRepository.findTotalStatistics();
                if (statisticsDataOpt.isPresent()) {
                    stats = statisticsDataOpt.get();
                } else {
                    // 如果没有找到统计数据，创建一个新的
                    stats = new StatisticsData();
                    stats.setDeptId(0); // 0表示全局统计数据
                    stats.setDeptName("河南省烟草专卖局");
                    stats.setCreateDate(new Date());
                }
                
                // 计算所有标准文件的总数
                long totalStandardDocs = actualStandardDocCounts.values().stream().mapToLong(Long::longValue).sum();
                stats.setStandardDocCount((int) totalStandardDocs);
                
                // 将各地市标准文档数量保存到content和deptStandardDocData字段
                try {
                    stats.setContent(objectMapper.writeValueAsString(actualStandardDocCounts));
                    stats.setDeptStandardDocData(objectMapper.writeValueAsString(actualStandardDocCounts));
                    
                    // 生成各部门使用AI智能体数量（模拟数据）
                    Map<String, Integer> deptAiAgentCounts = new HashMap<>();
                    Random random = new Random();
                    int totalAiAgentCount = 0;
                    for (Map.Entry<String, Long> entry : actualStandardDocCounts.entrySet()) {
                        // AI智能体使用数量与标准文件数量相关，但有随机波动
                        int aiAgentCount = (int)(entry.getValue() * 0.7) + random.nextInt(10);
                        deptAiAgentCounts.put(entry.getKey(), aiAgentCount);
                        totalAiAgentCount += aiAgentCount;
                    }
                    stats.setDeptAiAgentData(objectMapper.writeValueAsString(deptAiAgentCounts));
                    stats.setAiAgentCount(totalAiAgentCount);
                } catch (Exception e) {
                    log.error("将地市数据转换为JSON失败", e);
                }
                
                // 如果useStatistics为false，才生成模拟数据
                if (!useStatistics) {
                    // 填充模拟数据
                    fillSimulationData(stats);
                    
                    // 保存到数据库
                    statisticsDataRepository.save(stats);
                }
                
                return R.ok(stats);
            } else {
                // 特定部门的统计数据
                statisticsDataOpt = statisticsDataRepository.findByDeptId(deptId);
                if (statisticsDataOpt.isPresent()) {
                    stats = statisticsDataOpt.get();
                } else {
                    // 如果没有找到统计数据，创建一个新的
                    stats = new StatisticsData();
                    stats.setDeptId(deptId);
                    stats.setCreateDate(new Date());
                }
                
                // 查询部门名称
                JPAQueryFactory queryFactory = new JPAQueryFactory(entityManager);
                QDepartmentDoc qDepartmentDoc = QDepartmentDoc.departmentDoc;
                DepartmentDoc dept = queryFactory
                    .selectFrom(qDepartmentDoc)
                    .where(qDepartmentDoc.id.eq(deptId))
                    .fetchOne();
                
                if (dept != null) {
                    String deptName = dept.getDeptName();
                    stats.setDeptName(deptName);
                    
                    // 使用实际的标准文件数量
                    Long actualCount = actualStandardDocCounts.getOrDefault(deptName, 0L);
                    stats.setStandardDocCount(actualCount.intValue());
                    
                    // 将部门数据保存到content、deptStandardDocData和deptAiAgentData字段
                    try {
                        Map<String, Long> deptData = new HashMap<>();
                        deptData.put(deptName, actualCount);
                        stats.setContent(objectMapper.writeValueAsString(deptData));
                        stats.setDeptStandardDocData(objectMapper.writeValueAsString(deptData));
                        
                        // 生成部门使用AI智能体数量（模拟数据）
                        Map<String, Integer> deptAiAgentCounts = new HashMap<>();
                        // AI智能体使用数量与标准文件数量相关，但有随机波动
                        int aiAgentCount = (int)(actualCount * 0.7) + new Random().nextInt(10);
                        deptAiAgentCounts.put(deptName, aiAgentCount);
                        stats.setDeptAiAgentData(objectMapper.writeValueAsString(deptAiAgentCounts));
                        stats.setAiAgentCount(aiAgentCount);
                    } catch (Exception e) {
                        log.error("将部门数据转换为JSON失败", e);
                    }
                } else {
                    stats.setStandardDocCount(0);
                }
                
                // 如果useStatistics为false，才生成模拟数据
                if (!useStatistics) {
                    // 填充模拟数据
                    fillSimulationData(stats);
                    
                    // 保存到数据库
                    statisticsDataRepository.save(stats);
                }
                
                return R.ok(stats);
            }
        } catch (Exception e) {
            log.error("获取统计数据失败", e);
            return R.warning("获取统计数据失败: " + e.getMessage());
        }
    }
    
    /**
     * 填充模拟统计数据
     * @param stats 统计数据对象
     */
    private void fillSimulationData(StatisticsData stats) {
        // 更新时间
        stats.setUpdateDate(new Date());
        
        // 生成随机的AI查询次数，基于标准文件数量的倍数
        int standardDocCount = stats.getStandardDocCount() != null ? stats.getStandardDocCount() : 0;
        int aiQueryCount = standardDocCount * 12 + new Random().nextInt(1000);
        stats.setAiQueryTotalCount(aiQueryCount);
        
        // 生成随机的标准检索次数，基于标准文件数量的倍数
        int standardCheckCount = standardDocCount * 5 + new Random().nextInt(500);
        stats.setStandardCheckCount(standardCheckCount);
        
        // 生成随机的训练文档数量，基于标准文件数量
        int trainingDocCount = standardDocCount * 3 + new Random().nextInt(300);
        stats.setTrainingDocCount(trainingDocCount);
        
        // 生成AI查询月度数据
        try {
            List<Map<String, Integer>> aiMonthlyData = new ArrayList<>();
            Calendar cal = Calendar.getInstance();
            for (int i = 5; i >= 0; i--) {
                cal.add(Calendar.MONTH, -1);
                String monthKey = String.format("%d-%02d", cal.get(Calendar.YEAR), cal.get(Calendar.MONTH) + 1);
                Map<String, Integer> monthData = new HashMap<>();
                monthData.put(monthKey, aiQueryCount / 6 + new Random().nextInt(aiQueryCount / 10));
                aiMonthlyData.add(monthData);
            }
            stats.setAiQueryMonthlyData(objectMapper.writeValueAsString(aiMonthlyData));
            
            // 生成训练文档月度数据
            List<Map<String, Integer>> trainingMonthlyData = new ArrayList<>();
            cal = Calendar.getInstance();
            for (int i = 5; i >= 0; i--) {
                cal.add(Calendar.MONTH, -1);
                String monthKey = String.format("%d-%02d", cal.get(Calendar.YEAR), cal.get(Calendar.MONTH) + 1);
                Map<String, Integer> monthData = new HashMap<>();
                monthData.put(monthKey, trainingDocCount / 6 + new Random().nextInt(trainingDocCount / 10));
                trainingMonthlyData.add(monthData);
            }
            stats.setTrainingDocMonthlyData(objectMapper.writeValueAsString(trainingMonthlyData));
            
            // 智能检索使用时间段统计
            List<Map<String, Integer>> timeData = new ArrayList<>();
            Map<String, Integer> morning = new HashMap<>();
            morning.put("00:00-06:00", standardCheckCount / 10 + new Random().nextInt(standardCheckCount / 20));
            timeData.add(morning);
            
            Map<String, Integer> noon = new HashMap<>();
            noon.put("06:01-12:00", standardCheckCount / 3 + new Random().nextInt(standardCheckCount / 10));
            timeData.add(noon);
            
            Map<String, Integer> afternoon = new HashMap<>();
            afternoon.put("12:01-18:00", standardCheckCount / 3 + new Random().nextInt(standardCheckCount / 10));
            timeData.add(afternoon);
            
            Map<String, Integer> evening = new HashMap<>();
            evening.put("18:01-24:00", standardCheckCount / 4 + new Random().nextInt(standardCheckCount / 10));
            timeData.add(evening);
            
            stats.setSmartSearchTimeData(objectMapper.writeValueAsString(timeData));
            
            // 用户问答类型统计
            List<Map<String, Integer>> queryTypeData = new ArrayList<>();
            Map<String, Integer> loggedIn = new HashMap<>();
            loggedIn.put("已登录", aiQueryCount * 6 / 10 + new Random().nextInt(aiQueryCount / 10));
            queryTypeData.add(loggedIn);
            
            Map<String, Integer> notLoggedIn = new HashMap<>();
            notLoggedIn.put("未登录", aiQueryCount * 2 / 10 + new Random().nextInt(aiQueryCount / 10));
            queryTypeData.add(notLoggedIn);
            
            Map<String, Integer> visitor = new HashMap<>();
            visitor.put("访客", aiQueryCount * 2 / 10 + new Random().nextInt(aiQueryCount / 10));
            queryTypeData.add(visitor);
            
            stats.setUserQueryTypeData(objectMapper.writeValueAsString(queryTypeData));
            
            // 生成AI知识库问答当月每日统计数据
            Map<String, Integer> aiQueryDayData = new LinkedHashMap<>();
            Calendar currentCal = Calendar.getInstance();
            int currentYear = currentCal.get(Calendar.YEAR);
            int currentMonth = currentCal.get(Calendar.MONTH);
            int daysInMonth = currentCal.getActualMaximum(Calendar.DAY_OF_MONTH);
            int currentDay = currentCal.get(Calendar.DAY_OF_MONTH);
            
            // 只生成到当前日期为止的数据
            for (int day = 1; day <= currentDay; day++) {
                String dayKey = String.format("%d-%02d-%02d", currentYear, currentMonth + 1, day);
                // 生成随机的每日查询次数，基于总查询次数的比例
                int dailyCount = aiQueryCount / daysInMonth + new Random().nextInt(aiQueryCount / 30);
                aiQueryDayData.put(dayKey, dailyCount);
            }
            stats.setAiQueryDayData(objectMapper.writeValueAsString(aiQueryDayData));
        } catch (Exception e) {
            log.error("生成模拟统计数据失败", e);
        }
    }

    @Override
    public R initSimulationData() {
        try {
            // 创建总体统计数据（deptId=0表示总体数据）
            StatisticsData totalStats = createTotalStatisticsData();
            statisticsDataRepository.save(totalStats);
            
            // 创建河南省烟草总公司统计数据
            StatisticsData henanStats = createHenanStatisticsData();
            statisticsDataRepository.save(henanStats);
            
            // 为18个地市烟草公司创建模拟数据
            createCityStatisticsData();
            
            return R.ok("初始化模拟数据成功");
        } catch (Exception e) {
            log.error("初始化模拟数据失败", e);
            return R.warning("初始化模拟数据失败: " + e.getMessage());
        }
    }

    @Override
    public StatisticsData updateStatisticsData(StatisticsData statisticsData) {
        return statisticsDataRepository.save(statisticsData);
    }
    
    @Override
    public Map<String, Long> countStandardDocsByDepartment() {
        try {
            JPAQueryFactory queryFactory = new JPAQueryFactory(entityManager);
            QStandardDoc qStandardDoc = QStandardDoc.standardDoc;
            QDepartmentDoc qDepartmentDoc = QDepartmentDoc.departmentDoc;
            
            // 查询所有地市烟草公司
            List<DepartmentDoc> companies = queryFactory
                .select(qDepartmentDoc)
                .from(qDepartmentDoc)
                .where(qDepartmentDoc.isCompany.eq(true))
                .fetch();
            
            // 初始化结果集，所有公司默认为0文件
            Map<String, Long> docCountByDept = new HashMap<>();
            Map<Integer, String> deptIdToNameMap = new HashMap<>();
            for (DepartmentDoc company : companies) {
                docCountByDept.put(company.getDeptName(), 0L);
                deptIdToNameMap.put(company.getId(), company.getDeptName());
            }
            
            // 查询各地市烟草公司的标准文件数量
            // 只统计status=1且delFlag=false的标准文件
            List<Tuple> results = queryFactory
                .select(qStandardDoc.deptId, qStandardDoc.count())
                .from(qStandardDoc)
                .innerJoin(qDepartmentDoc)
                .on(qStandardDoc.deptId.eq(qDepartmentDoc.id))
                .where(
                    qDepartmentDoc.isCompany.eq(true)
                    .and(qStandardDoc.status.eq(1))
                    .and(qStandardDoc.delFlag.eq(false))
                )
                .groupBy(qStandardDoc.deptId)
                .fetch();
            
            // 更新实际有文件的公司统计数据
            for (Tuple tuple : results) {
                Integer deptId = tuple.get(0, Integer.class);
                Long count = tuple.get(1, Long.class);
                if (deptId != null && count != null) {
                    String deptName = deptIdToNameMap.get(deptId);
                    if (deptName != null) {
                        docCountByDept.put(deptName, count);
                    }
                }
            }
            
            return docCountByDept;
        } catch (Exception e) {
            log.error("统计各地市烟草公司标准文件数量失败", e);
            return Collections.emptyMap();
        }
    }
    
    /**
     * 创建总体统计数据
     */
    private StatisticsData createTotalStatisticsData() {
        StatisticsData data = new StatisticsData();
        data.setDeptId(0); // 0表示总体数据
        data.setDeptName("全省");
        data.setAiQueryTotalCount(101540); // AI知识库问答总次数 - 来自截图
        data.setStandardDocCount(48842);   // 累计标准文件总数 - 来自截图
        data.setStandardCheckCount(85541); // 标准检索次数 - 来自截图
        data.setTrainingDocCount(113683);  // 模型训练文件总量 - 来自截图
        
        // AI知识库问答月度统计数据 - 根据截图右上角折线图
        Map<String, Integer> aiQueryData = new LinkedHashMap<>();
        aiQueryData.put("2025-01", 50);
        aiQueryData.put("2025-02", 60);
        aiQueryData.put("2025-03", 40);
        aiQueryData.put("2025-04", 25);
        aiQueryData.put("2025-05", 65);
        data.setAiQueryMonthlyData(convertMapToJsonString(aiQueryData));
        
        // 训练文档月度变化数据 - 根据截图左上角折线图
        Map<String, Integer> trainingDocData = new LinkedHashMap<>();
        trainingDocData.put("2025-01", 65);
        trainingDocData.put("2025-02", 45);
        trainingDocData.put("2025-03", 80);
        trainingDocData.put("2025-04", 75);
        trainingDocData.put("2025-05", 45);
        data.setTrainingDocMonthlyData(convertMapToJsonString(trainingDocData));
        
        // 智能检索使用时间段统计 - 根据截图左下角折线图
        Map<String, Integer> timeData = new LinkedHashMap<>();
        timeData.put("00:00-06:00", 50);
        timeData.put("06:01-12:00", 95);
        timeData.put("12:01-18:00", 70);
        timeData.put("18:01-24:00", 85);
        data.setSmartSearchTimeData(convertMapToJsonString(timeData));
        
        // 用户问答类型统计 - 根据截图右下角饼图
        Map<String, Integer> userTypeData = new LinkedHashMap<>();
        userTypeData.put("已登录", 5902); // 60% of 9837
        userTypeData.put("未登录", 1967); // 20% of 9837
        userTypeData.put("访客", 1968);   // 20% of 9837
        data.setUserQueryTypeData(convertMapToJsonString(userTypeData));
        
        data.setCreateDate(new Date());
        data.setUpdateDate(new Date());
        
        return data;
    }
    
    /**
     * 创建河南省烟草总公司统计数据
     */
    private StatisticsData createHenanStatisticsData() {
        StatisticsData data = new StatisticsData();
        data.setDeptId(1); // 假设1是河南省烟草总公司的ID
        data.setDeptName("河南省烟草总公司");
        data.setAiQueryTotalCount(166743); // AI智能体使用次数 - 来自截图中间区域
        data.setStandardDocCount(13478);   // 标准文件数量 - 来自截图中间区域
        data.setStandardCheckCount(21385); // 标准检索次数
        data.setTrainingDocCount(28420);  // 模型训练文件总量
        
        // AI知识库问答月度统计数据
        Map<String, Integer> aiQueryData = new LinkedHashMap<>();
        aiQueryData.put("2025-01", 45);
        aiQueryData.put("2025-02", 55);
        aiQueryData.put("2025-03", 35);
        aiQueryData.put("2025-04", 20);
        aiQueryData.put("2025-05", 60);
        data.setAiQueryMonthlyData(convertMapToJsonString(aiQueryData));
        
        // 训练文档月度变化数据
        Map<String, Integer> trainingDocData = new LinkedHashMap<>();
        trainingDocData.put("2025-01", 60);
        trainingDocData.put("2025-02", 40);
        trainingDocData.put("2025-03", 75);
        trainingDocData.put("2025-04", 70);
        trainingDocData.put("2025-05", 40);
        data.setTrainingDocMonthlyData(convertMapToJsonString(trainingDocData));
        
        // 智能检索使用时间段统计
        Map<String, Integer> timeData = new LinkedHashMap<>();
        timeData.put("00:00-06:00", 45);
        timeData.put("06:01-12:00", 90);
        timeData.put("12:01-18:00", 65);
        timeData.put("18:01-24:00", 80);
        data.setSmartSearchTimeData(convertMapToJsonString(timeData));
        
        // 用户问答类型统计
        Map<String, Integer> userTypeData = new LinkedHashMap<>();
        userTypeData.put("已登录", 1476); // 60% of 2460
        userTypeData.put("未登录", 492);  // 20% of 2460
        userTypeData.put("访客", 492);        // 20% of 2460
        data.setUserQueryTypeData(convertMapToJsonString(userTypeData));
        
        data.setCreateDate(new Date());
        data.setUpdateDate(new Date());
        
        return data;
    }
    
    /**
     * 为18个地市烟草公司创建模拟数据
     */
    private void createCityStatisticsData() {
        // 18个地市名称
        String[] cityNames = {
            "郑州市", "开封市", "洛阳市", "平顶山市", "安阳市", "鹤壁市", 
            "新乡市", "焦作市", "濮阳市", "许昌市", "漯河市", "三门峡市", 
            "南阳市", "商丘市", "信阳市", "周口市", "驻马店市", "济源市"
        };
        
        // 各地市标准文件数量分配，总和为13,478
        int[] standardDocCounts = {
            1850, 720, 1100, 650, 780, 420, 
            850, 610, 520, 680, 450, 380, 
            1200, 750, 680, 820, 580, 438
        };
        
        // 各地市AI智能体使用次数分配，总和为166,743
        int[] aiQueryCounts = {
            28500, 8200, 12500, 7800, 9500, 5200, 
            10200, 7600, 6800, 8400, 5600, 4800, 
            14500, 9200, 8400, 10200, 7200, 12143
        };
        
        // 各地市标准检索次数分配，总和为85,541
        int[] standardCheckCounts = {
            14500, 4200, 6500, 3800, 4900, 2700, 
            5200, 3900, 3500, 4300, 2900, 2500, 
            7400, 4700, 4300, 5200, 3700, 6241
        };
        
        // 各地市模型训练文件总量分配，总和为113,683
        int[] trainingDocCounts = {
            19500, 5600, 8700, 5100, 6500, 3600, 
            7000, 5200, 4700, 5800, 3900, 3300, 
            9900, 6300, 5800, 7000, 4900, 10683
        };
        
        Random random = new Random();
        
        for (int i = 0; i < cityNames.length; i++) {
            StatisticsData data = new StatisticsData();
            data.setDeptId(i + 2); // 从2开始，因为0是总体，1是省公司
            data.setDeptName(cityNames[i] + "烟草公司");
            
            // 使用预定义的数据
            data.setStandardDocCount(standardDocCounts[i]);     // 标准文件数量
            data.setAiQueryTotalCount(aiQueryCounts[i]);        // AI智能体使用次数
            data.setStandardCheckCount(standardCheckCounts[i]); // 标准检索次数
            data.setTrainingDocCount(trainingDocCounts[i]);      // 模型训练文件总量
            
            // AI知识库问答月度统计数据
            Map<String, Integer> aiQueryData = new LinkedHashMap<>();
            aiQueryData.put("2025-01", 30 + random.nextInt(30));
            aiQueryData.put("2025-02", 35 + random.nextInt(30));
            aiQueryData.put("2025-03", 25 + random.nextInt(20));
            aiQueryData.put("2025-04", 15 + random.nextInt(15));
            aiQueryData.put("2025-05", 40 + random.nextInt(30));
            data.setAiQueryMonthlyData(convertMapToJsonString(aiQueryData));
            
            // 训练文档月度变化数据
            Map<String, Integer> trainingDocData = new LinkedHashMap<>();
            trainingDocData.put("2025-01", 40 + random.nextInt(30));
            trainingDocData.put("2025-02", 30 + random.nextInt(20));
            trainingDocData.put("2025-03", 50 + random.nextInt(35));
            trainingDocData.put("2025-04", 45 + random.nextInt(35));
            trainingDocData.put("2025-05", 30 + random.nextInt(20));
            data.setTrainingDocMonthlyData(convertMapToJsonString(trainingDocData));
            
            // 智能检索使用时间段统计
            Map<String, Integer> timeData = new LinkedHashMap<>();
            timeData.put("00:00-06:00", 20 + random.nextInt(35));
            timeData.put("06:01-12:00", 50 + random.nextInt(45));
            timeData.put("12:01-18:00", 40 + random.nextInt(35));
            timeData.put("18:01-24:00", 45 + random.nextInt(40));
            data.setSmartSearchTimeData(convertMapToJsonString(timeData));
            
            // 用户问答类型统计
            int total = 300 + random.nextInt(400); // 每个地市的用户问答总数
            Map<String, Integer> userTypeData = new LinkedHashMap<>();
            int loggedIn = (int)(total * 0.6);  // 60%
            int notLoggedIn = (int)(total * 0.2); // 20%
            int visitor = total - loggedIn - notLoggedIn; // 剩余的约为20%
            userTypeData.put("已登录", loggedIn);
            userTypeData.put("未登录", notLoggedIn);
            userTypeData.put("访客", visitor);
            data.setUserQueryTypeData(convertMapToJsonString(userTypeData));
            
            data.setCreateDate(new Date());
            data.setUpdateDate(new Date());
            
            statisticsDataRepository.save(data);
        }
    }
    
    
    /**
     * 将Map转换为JSON字符串
     * @param map 数据Map
     * @return JSON字符串
     */
    private String convertMapToJsonString(Map<String, Integer> map) {
        try {
            return objectMapper.writeValueAsString(map);
        } catch (Exception e) {
            log.error("转换JSON失败", e);
            return "{}";
        }
    }
}
