package com.linkyoyo.tobacco.service;

import com.linkyoyo.tobacco.entity.StatisticsData;
import com.linkyoyo.tobacco.result.R;

import java.util.Map;

/**
 * 统计数据服务接口
 */
public interface StatisticsDataService {
    
    /**
     * 获取统计分析数据
     * @param deptId 部门ID，如果为null则返回总体统计数据
     * @return 统计分析数据
     */
    R getStatisticsData(Integer deptId);
    
    /**
     * 初始化模拟数据
     * @return 操作结果
     */
    R initSimulationData();
    
    /**
     * 更新统计数据
     * @param statisticsData 统计数据
     * @return 更新后的统计数据
     */
    StatisticsData updateStatisticsData(StatisticsData statisticsData);
    
    /**
     * 使用QueryDSL统计各地市烟草公司的标准文件总数
     * @return 各地市烟草公司的标准文件数量统计，key为部门名称，value为文件数量
     */
    Map<String, Long> countStandardDocsByDepartment();
}
